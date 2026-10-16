"""🔬 POS privacy/type oracle (reusable). Builds a single-crate sandbox union of a crate family (stub layer for everything outside it), runs
`rustc +nightly-2026-07-20 --emit=metadata` for lib and `--test` (and optional wasm targets) and reports the type-level errors (E0603, E0616, E0624, E0599, E0277, E0308 ...).
With a baseline (git revision + pre-carve crate names) it diffs the keyed error multisets; without one it classifies absolutely.
Writes only inside the sandbox (scratchpad/pos-work) and pos-* files in 🗑️generated; the repo is only read (working tree or `git cat-file`).
usage:
  pos.py index [--force]
  pos.py run <family.json> [--cfgs lib,test,wasm,p2] [--label L] [--iters N] [--no-base] [--mutate "<crate-pkg>::<repo-rel-file>@@old@@new"]...
"""
import collections, json, os, re, shutil, subprocess, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ftclib import *
import ftcunion as U
from ftcstub import Stub, mask, DEFINED_RX

CFGS = {
    "lib": [],
    "test": ["--test"],
    "wasm": ["--target", "wasm32-unknown-unknown"],
    "p2": ["--target", "wasm32-wasip2"],
    "wasmtest": ["--test", "--target", "wasm32-unknown-unknown"],
    "p2test": ["--test", "--target", "wasm32-wasip2"],
}
PRIV = {"E0603", "E0616", "E0624", "E0451", "E0364", "E0365", "E0445", "E0446", "E0742", "E0704"}
RESOLVE = {"E0412", "E0425", "E0422", "E0423", "E0432", "E0433", "E0405", "E0407", "E0531", "E0532", "E0574", "E0583", "E0426", "E0408", "E0424", "E0434", "E0435"}
TYPEC = {"E0599", "E0277", "E0308", "E0282", "E0283", "E0061", "E0063", "E0559", "E0560", "E0609", "E0610", "E0614", "E0615", "E0618", "E0369", "E0053", "E0046", "E0117", "E0116", "E0119", "E0120", "E0195", "E0200", "E0201", "E0204", "E0207", "E0220", "E0229", "E0191", "E0107", "E0106", "E0038", "E0040", "E0004", "E0023", "E0026", "E0027", "E0029", "E0030", "E0034", "E0054", "E0057", "E0069", "E0070", "E0072", "E0080", "E0081", "E0084", "E0133", "E0184", "E0185", "E0186", "E0199", "E0261", "E0263", "E0271", "E0275", "E0276", "E0373", "E0381", "E0382", "E0383", "E0384", "E0499", "E0502", "E0503", "E0505", "E0506", "E0507", "E0515", "E0597", "E0599", "E0600", "E0604", "E0605", "E0606", "E0607", "E0608", "E0620"}
PATHRX = re.compile(r"\b(?:[a-z_][a-z0-9_]*::)+(?=[A-Za-z_{(<])")


def cat_of(code, msg):
    if code in PRIV:
        return "privacy"
    if code in RESOLVE or re.match(r"cannot find (macro|attribute|derive macro)", msg):
        return "resolve"
    if code in TYPEC:
        return "type"
    return "other"


def nrm(s):
    return PATHRX.sub("", s)


def parse_json(stderr, cwd):
    diags = []
    for line in stderr.split("\n"):
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("level") not in ("error", "error: internal compiler error"):
            continue
        msg = d.get("message", "")
        if msg.startswith("aborting due to") or msg.startswith("could not compile") or "previous error" in msg:
            continue
        code = (d.get("code") or {}).get("code") or ""
        prim = None
        for s in d.get("spans", []):
            if s.get("is_primary"):
                prim = s
                break
        if prim is None and d.get("spans"):
            prim = d["spans"][0]
        file_, ln, label = "", 0, ""
        if prim:
            file_ = prim.get("file_name", "")
            ln = prim.get("line_start", 0)
            label = prim.get("label") or ""
            for ex in prim.get("expansion") and [prim["expansion"]] or []:
                pass
        kids = " | ".join(c.get("message", "") for c in d.get("children", []))
        rendered = d.get("rendered", "") or ""
        diags.append(dict(code=code, msg=msg, file=file_, line=ln, label=label, kids=kids, stub=("__ext" in rendered), cat=cat_of(code, msg)))
    return diags


def repo_rel(file_, side_dir):
    m = re.match(r"^(__m_[A-Za-z0-9_]+)/(.*)$", file_)
    if m:
        return m.group(1)[4:], m.group(2)
    return "", file_


def key_of(d):
    return (d["code"], nrm(d["msg"]), nrm(d["label"]))


class Runner:
    def __init__(self, name):
        self.name = name
        self.base_dir = SANDBOX_BASE + "/" + name
        self.sb = Sandbox(SANDBOX_BASE)
        os.makedirs(self.base_dir, exist_ok=True)

    def wipe(self, d):
        rp = os.path.realpath(d)
        assert rp.startswith(os.path.realpath(SANDBOX_BASE) + os.sep), rp
        if os.path.exists(rp):
            shutil.rmtree(rp)

    def run_rustc(self, outdir, extra, features, tag):
        out = os.path.join(outdir, f"__out_{tag}.rmeta")
        self.sb.check(out)
        cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", "ftc_union", "--emit=metadata", "-o", out, "--cap-lints", "allow", "--error-format=json", "-Ztrim-diagnostic-paths=no", *features, *extra, os.path.join(outdir, "__ftc_root.rs")]
        t0 = time.time()
        p = subprocess.run(cmd, capture_output=True, text=True, cwd=outdir)
        if os.path.exists(out):
            os.remove(out)
        return p.stderr, time.time() - t0


def build_side(rn, label, src, pkgs, mans, strip_attrs, stub_names_hint=None):
    crates = U.prepare(src, mans, pkgs, False)
    return crates


def run_family(spec, args):
    name = spec["name"]
    label = args.get("label", "run")
    cfgs = args.get("cfgs") or ["lib", "test"]
    iters = int(args.get("iters", 4))
    mans = Manifests(Source(None))
    mans.build_index()
    rn = Runner(name)
    cur_src = Source(None)
    sides = {"cur": dict(src=cur_src, pkgs=spec["crates"])}
    if spec.get("base") and not args.get("no_base"):
        sides["base"] = dict(src=Source(spec["base"]["rev"]), pkgs=spec["base"]["crates"])
    for s, d in sides.items():
        d["crates"] = U.prepare(d["src"], mans, d["pkgs"], False)
        d["dir"] = rn.base_dir + "/" + s
        missing = [c.pkg for c in d["crates"] if c.missing]
        d["missing_files"] = {c.pkg: c.missing[:5] for c in d["crates"] if c.missing}
        d["absent"] = [p for p in d["pkgs"] if p not in {c.pkg for c in d["crates"]}]
    strip_attrs = set()
    result = dict(family=name, label=label, cfgs=cfgs, sides={})
    stub = Stub()
    for it in range(iters):
        masked = []
        globs = {}
        for s, d in sides.items():
            rn.wipe(d["dir"])
            os.makedirs(d["dir"], exist_ok=True)
            m_all, g, rfeats, notes = U.build(rn.sb, d["dir"], d["crates"], strip_attrs, spec.get("libNoTest", []) if s == "cur" else [])
            masked += m_all
            d["globs"], d["rfeats"], d["notes"] = g, rfeats, notes
        stub = Stub()
        stub.scan(masked)
        if it == 0:
            defined = set()
            for s, d in sides.items():
                defined |= U.defined_names(d["crates"])
        fills = {}
        for fill_name, (modpath, nm, kind) in args.get("_fills", {}).items():
            stub.add_name(modpath, nm, kind)
        for s, d in sides.items():
            for ap in args.get("_mutations", []):
                pass
        for s, d in sides.items():
            for mut in args.get("mutate", []):
                pkg_rel, old, new = mut.split("@@")
                pkg, rel = pkg_rel.split("::", 1)
                cr = [c for c in d["crates"] if c.pkg == pkg]
                if not cr or s != "cur":
                    continue
                fp = os.path.join(d["dir"], cr[0].mod, rel)
                rn.sb.check(fp)
                tx = open(fp, encoding="utf-8").read()
                assert old in tx, f"mutation anchor missing in {rel}: {old!r}"
                rn.sb.write(fp, tx.replace(old, new, 1))
        for s, d in sides.items():
            rn.sb.write(d["dir"] + "/__ftc_root.rs", U.root_text(d["crates"], stub.render(), d["rfeats"]))
        results = {}
        for s, d in sides.items():
            for c_ in cfgs:
                test = "test" in c_
                en = U.feature_closure(d["crates"], test, mans)
                for c in d["crates"]:
                    c.features = en[c.pkg]
                feats = U.cfg_args(d["crates"])
                err, sec = rn.run_rustc(d["dir"], CFGS[c_], feats, f"{s}_{c_}")
                diags = parse_json(err, d["dir"])
                results[(s, c_)] = dict(diags=diags, seconds=round(sec), stderr_len=len(err))
                log_write(f"{GEN}/ftc-pos-{name}-{label}-{s}-{c_}-it{it}.err", err[:4_000_000])
                print(f"[{name}] it{it} {s}/{c_}: {len(diags)} errors in {round(sec)}s", flush=True)
        new_fill = False
        newattr = set()
        fills = args.setdefault("_fills", {})
        for (s, c_), r in results.items():
            d = sides[s]
            for dg in r["diags"]:
                m = re.match(r"cannot find attribute `([^`]+)` in this scope", dg["msg"])
                if m and m.group(1) not in strip_attrs and m.group(1) not in ("test",):
                    newattr.add(m.group(1))
                m2 = re.match(r"cannot find (type|trait|value|function|struct, variant or union type|tuple struct or tuple variant|unit struct, unit variant or constant|derive macro) `([^`]+)` in this scope|failed to resolve: use of undeclared type `([^`]+)`", dg["msg"])
                if not m2:
                    continue
                kindword = m2.group(1) or "type"
                nm = m2.group(2) or m2.group(3)
                if nm in defined or nm in ("Self", "self", "super", "crate"):
                    continue
                mod_, f_ = repo_rel(dg["file"], d["dir"])
                gl = d["globs"].get(os.path.join("__m_" + mod_, f_)) if mod_ else None
                if not gl:
                    continue
                modpath = gl[0]
                kind = "trait" if kindword == "trait" else ("struct" if kindword in ("type", "struct, variant or union type", "tuple struct or tuple variant", "unit struct, unit variant or constant") else "fn")
                if kindword in ("value", "function") and re.match(r"^[A-Z]", nm):
                    kind = "struct"
                if kindword == "derive macro":
                    continue
                kk = ("/".join(modpath), nm)
                if kk in fills or stub.has(modpath, nm):
                    continue
                fills[kk] = (modpath, nm, kind)
                new_fill = True
        result["iterations"] = it + 1
        if newattr:
            strip_attrs |= newattr
            print(f"[{name}] stripping attributes {sorted(newattr)} and rebuilding", flush=True)
            continue
        if not new_fill or it == iters - 1:
            break
        print(f"[{name}] grew stub globs by {len(fills)} names; re-running", flush=True)
    final = results
    result["strip_attrs"] = sorted(strip_attrs)
    result["stub_glob_fills"] = len(args.get("_fills", {}))
    for s, d in sides.items():
        result["sides"][s] = dict(crates=[c.pkg for c in d["crates"]], absent=d["absent"], missing_files=d["missing_files"], dup_macros=d["notes"].get("dup_macros", {}), excluded_heads={k: sorted(v) for k, v in d["notes"].get("excluded", {}).items() if v})
    rows = {}
    for (s, c_), r in final.items():
        rows.setdefault(c_, {})[s] = r
    report = []
    summary = {}
    for c_ in cfgs:
        cur = rows[c_]["cur"]["diags"]
        base = rows[c_].get("base", {}).get("diags") if "base" in rows[c_] else None
        entry = dict(cur_total=len(cur), cur_noise=sum(1 for d in cur if d["stub"]), seconds=rows[c_]["cur"]["seconds"])
        by = collections.Counter((d["code"], "stub" if d["stub"] else "internal") for d in cur)
        entry["by_code"] = {f"{k[0]}/{k[1]}": v for k, v in sorted(by.items())}
        if base is not None:
            kb = collections.Counter(key_of(d) for d in base)
            kc = collections.Counter(key_of(d) for d in cur)
            newc = kc - kb
            gone = kb - kc
            entry.update(base_total=len(base), new=sum(newc.values()), gone=sum(gone.values()))
            seen = collections.Counter()
            newlist = []
            for d in cur:
                k = key_of(d)
                seen[k] += 1
                if seen[k] > kb.get(k, 0):
                    newlist.append(d)
            entry["new_list"] = newlist
            entry["gone_keys"] = [dict(code=k[0], msg=k[1], label=k[2], n=n) for k, n in sorted(gone.items())]
        else:
            entry["new_list"] = [d for d in cur if not d["stub"]]
        summary[c_] = entry
    result["summary"] = summary
    result["definition_sides"] = {s: d["dir"] for s, d in sides.items()}
    out_json = f"{GEN}/ftc-pos-{name}-{label}-result.json"
    log_write(out_json, json.dumps(result, ensure_ascii=False, indent=1))
    txt = render_text(result)
    log_write(f"{GEN}/ftc-pos-{name}-{label}-report.txt", txt)
    print(txt)
    return result


def short(f, ln, side_dir=None):
    pkg, rel = repo_rel(f, side_dir)
    return f"[{pkg}] {rel}:{ln}" if pkg else f"{f}:{ln}"


def render_text(result):
    out = [f"== family {result['family']} label {result['label']} iterations {result['iterations']} stub-glob-fills {result['stub_glob_fills']} stripped attrs {result['strip_attrs']}"]
    for s, d in result["sides"].items():
        out.append(f"   side {s}: {len(d['crates'])} crates; absent {d['absent']}; missing files { {k: v for k, v in d['missing_files'].items()} }; dup macros {d['dup_macros']}; excluded heads {d['excluded_heads']}")
    for c_, e in result["summary"].items():
        out.append(f"-- cfg {c_}: cur errors {e['cur_total']} (stub-involved {e['cur_noise']}), {e['seconds']}s" + (f"; base errors {e['base_total']}; NEW {e['new']}; GONE {e['gone']}" if "base_total" in e else "; absolute (no baseline)"))
        out.append("   by code: " + ", ".join(f"{k}={v}" for k, v in e["by_code"].items()))
        grp = collections.defaultdict(list)
        for d in e["new_list"]:
            grp[d["cat"]].append(d)
        for cat in ("privacy", "resolve", "type", "other"):
            lst = grp.get(cat, [])
            if cat == "privacy" or cat == "resolve":
                pass
            out.append(f"   {cat}: {len(lst)}" + (" (non-stub, new)" if "base_total" in e else " (non-stub)"))
            for d in lst[:200]:
                out.append(f"      {d['code']} {short(d['file'], d['line'])}: {d['msg'][:200]} | {d['label'][:100]}")
    return "\n".join(out) + "\n"


def main():
    a = sys.argv[1:]
    if not a:
        print(__doc__)
        return
    if a[0] == "index":
        m = Manifests(Source(None))
        m.build_index("--force" in a)
        print(len(m.by_name), "packages")
        return
    if a[0] == "run":
        spec = json.load(open(a[1], encoding="utf-8"))
        args = {"mutate": []}
        i = 2
        while i < len(a):
            if a[i] == "--cfgs":
                args["cfgs"] = a[i + 1].split(",")
                i += 2
            elif a[i] == "--label":
                args["label"] = a[i + 1]
                i += 2
            elif a[i] == "--iters":
                args["iters"] = a[i + 1]
                i += 2
            elif a[i] == "--no-base":
                args["no_base"] = True
                i += 1
            elif a[i] == "--mutate":
                args["mutate"].append(a[i + 1])
                i += 2
            else:
                raise SystemExit("bad arg " + a[i])
        run_family(spec, args)


if __name__ == "__main__":
    main()
