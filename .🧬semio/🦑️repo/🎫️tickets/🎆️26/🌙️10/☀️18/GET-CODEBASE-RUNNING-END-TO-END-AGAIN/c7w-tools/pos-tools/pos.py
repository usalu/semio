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
from poslib import *
import posunion as U
import postrait as TR
import poshir as HIR
from posstub import Stub, mask, DEFINED_RX

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
FILE_CACHE = {}
PATHRX = re.compile(r"\b(?:[a-z_][a-z0-9_]*::)+(?=[A-Za-z_{(<])")


def cat_of(code, msg):
    if code in PRIV:
        return "privacy"
    if code in RESOLVE or re.match(r"cannot find (macro|attribute|derive macro)", msg):
        return "resolve"
    if code in TYPEC:
        return "type"
    return "other"


PRIV_LINE = re.compile(r'"code":\{"code":"(?:E0603|E0616|E0624|E0451)"')


def priv_lines(err):
    return "\n".join(l for l in err.split("\n") if PRIV_LINE.search(l[:2000])) + "\n"


def is_trait_scope(d):
    k = d.get("kids", "")
    return d["code"] == "E0599" and d["msg"].startswith("no ") and ("implemented but not in scope" in k or "items from traits can only be used if the trait is in scope" in k)


def nrm(s):
    return re.sub(r"__m_[A-Za-z0-9_]+", "__m_X", re.sub(r"\bTr\d+\b", "Tr", PATHRX.sub("", s)))


def parse_json(stderr, cwd, stub_only=frozenset()):
    FILE_CACHE.clear()
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
        snippet = ""
        line_text, hs = None, None
        if prim and prim.get("text"):
            line_text = prim["text"][0].get("text")
            hs = prim["text"][0].get("highlight_start", 1) - 1
        if prim:
            for tx in prim.get("text", []):
                snippet += tx.get("text", "")[max(0, tx.get("highlight_start", 1) - 1):tx.get("highlight_end", 1) - 1] + " "
        toks = set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", msg + " " + label + " " + snippet))
        spanlabels = " ".join((s.get("label") or "") for s in d.get("spans", [])) + " " + " ".join((s.get("label") or "") for c_ in d.get("children", []) for s in c_.get("spans", []))
        child_snip = " ".join(tx.get("text", "") for c_ in d.get("children", []) for s_ in c_.get("spans", []) for tx in s_.get("text", [])) if code in ("E0204", "E0277", "E0599", "E0308") else ""
        ctoks = set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", child_snip))
        tscope_ = code == "E0599" and msg.startswith("no ") and ("implemented but not in scope" in kids or "items from traits can only be used if the trait is in scope" in kids)
        unit_noise = ("unit type `()`" in msg) or ("`()` is not" in msg) or ("`()` doesn't implement" in msg) or ("`__dsl_" in msg) or ("for type `!`" in msg) or ("`?` operator can only be applied" in msg) or ("no field `" in msg and "type `()`" in msg) or ("found `()`" in label and code == "E0308") or (code == "E0599" and ("for type parameter" in msg or re.search(r"for (?:mutable )?reference `&(?:mut )?[A-Z][A-Za-z0-9]?`", msg)) and not tscope_) or (code == "E0599" and "trait bounds were not satisfied" in msg) or (code == "E0599" and "for struct" in msg and msg.startswith("no method named") and not tscope_ and any(w in kids for w in ("the following trait defines", "trait bounds")))
        in_root_ = any(s_.get("file_name", "").endswith("__pos_root.rs") for s_ in d.get("spans", [])) or any(s_.get("file_name", "").endswith("__pos_root.rs") for c_ in d.get("children", []) for s_ in c_.get("spans", []))
        stub = (in_root_ or ("__ext" in (msg + " " + label + " " + kids + " " + spanlabels)) or bool((toks | ctoks) & stub_only) or unit_noise or code == "E0747") and cat_of(code, msg) != "privacy"
        ctx_trait = False
        if prim and code in TR.HEAD_CODES and file_ and prim.get("byte_start") is not None and not prim.get("expansion"):
            try:
                tx_ = FILE_CACHE.get(os.path.join(cwd, file_))
                if tx_ is None:
                    with open(os.path.join(cwd, file_), "rb") as fh_:
                        tx_ = fh_.read()
                    FILE_CACHE[os.path.join(cwd, file_)] = tx_
                pre_ = mask(tx_[:prim["byte_start"]].decode("utf-8", "replace"))
                post_ = tx_[prim["byte_end"]:prim["byte_end"] + 80].decode("utf-8", "replace")
                ctx_trait = TR.deep_trait_context(pre_, post_)
            except Exception:
                ctx_trait = False
        diags.append(dict(code=code, msg=msg, file=file_, line=ln, label=label, kids=kids, stub=stub, cat=cat_of(code, msg), line_text=line_text, hs=hs, ctx_trait=ctx_trait, in_macro=any(s_.get("expansion") for s_ in d.get("spans", [])), rendered=rendered[:1800]))
    canary = [d for d in diags if d["file"].endswith("__pos_root.rs") and d["code"] in ("E0616", "E0624", "E0308")]
    reached = {d["code"] for d in canary}
    diags = [d for d in diags if d not in canary]
    for d in diags:
        d["typeck"] = reached >= {"E0616", "E0624", "E0308"}
    return diags, reached


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
        cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", "pos_union", "--emit=metadata", "-o", out, "--cap-lints", "allow", "--error-format=json", "-Ztrim-diagnostic-paths=no", *features, *extra, "__pos_root.rs"]
        t0 = time.time()
        p = subprocess.run(cmd, capture_output=True, text=True, cwd=outdir)
        if os.path.exists(out):
            os.remove(out)
        return p.stderr, time.time() - t0


def build_side(rn, label, src, pkgs, mans, strip_attrs, stub_names_hint=None):
    crates = U.prepare(src, mans, pkgs, False)
    return crates


def quarantine(rn, d, s, cfgs, mans, stub_only, qmax, name):
    """rustc stops before body type-check for reasons I could not attribute generally (see RESULT section 4). Fallback: delta-debug which source files must be
    emptied for the canary to be reached, empty exactly those (and report them), and re-run. Coverage of the quarantined files (and the modules they mount) is lost, nothing else."""
    files = []
    for c in d["crates"]:
        for rel in sorted(c.code):
            files.append(os.path.join(d["dir"], c.mod, rel))
    orig = {}
    for p_ in files:
        if os.path.exists(p_):
            orig[p_] = open(p_, encoding="utf-8").read()
    runs = [0]

    def reached(S):
        for p_, tx in orig.items():
            rn.sb.write(p_, "" if p_ in S else tx)
        c0 = cfgs[0]
        en = U.feature_closure(d["crates"], "test" in c0, mans)
        for c in d["crates"]:
            c.features = en[c.pkg]
        err, sec = rn.run_rustc(d["dir"], CFGS[c0], U.cfg_args(d["crates"]), f"{s}_q")
        runs[0] += 1
        _, rc = parse_json(err, d["dir"], stub_only)
        return {"E0616", "E0624", "E0308"} <= rc

    allf = sorted(orig)
    S = list(allf)
    if not reached(set(S)):
        for p_, tx in orig.items():
            rn.sb.write(p_, tx)
        return None
    n = 2
    while len(S) >= 2 and runs[0] < qmax:
        chunk = max(1, len(S) // n)
        subs = [S[i:i + chunk] for i in range(0, len(S), chunk)]
        red = False
        for sub in subs:
            if runs[0] >= qmax:
                break
            if reached(set(sub)):
                S, n, red = sub, 2, True
                break
        if not red and runs[0] < qmax:
            for sub in subs:
                if runs[0] >= qmax:
                    break
                comp = [x for x in S if x not in sub]
                if comp and reached(set(comp)):
                    S, n, red = comp, max(n - 1, 2), True
                    break
        if not red:
            if n >= len(S):
                break
            n = min(len(S), n * 2)
        print(f"[{name}] quarantine {s}: candidate set {len(S)} files after {runs[0]} runs", flush=True)
    for p_, tx in orig.items():
        rn.sb.write(p_, "" if p_ in S else tx)
    return [os.path.relpath(x, d["dir"]) for x in S]


def run_family(spec, args):
    name = spec["name"]
    U.EXTRA_FEATURES = {k: set(v) for k, v in spec.get("features", {}).items()}
    label = args.get("label", "run")
    cfgs = args.get("cfgs") or ["lib", "test"]
    iters = int(args.get("iters", 8))
    mans = Manifests(Source(None))
    mans.build_index()
    rn = Runner(name)
    cur_src = Source(None)
    sides = {"cur": dict(src=cur_src, pkgs=spec["crates"])}
    if spec.get("base") and not args.get("no_base"):
        sides["base"] = dict(src=Source(spec["base"]["rev"]), pkgs=spec["base"]["crates"])
    for s, d in sides.items():
        t_prep = time.time()
        d["crates"] = U.prepare(d["src"], mans, d["pkgs"], False)
        print(f"[{name}] prepared {s}: {len(d['crates'])} crates in {round(time.time() - t_prep)}s", flush=True)
        d["dir"] = rn.base_dir + "/" + s
        missing = [c.pkg for c in d["crates"] if c.missing]
        d["missing_files"] = {c.pkg: c.missing[:5] for c in d["crates"] if c.missing}
        d["absent"] = [p for p in d["pkgs"] if p not in {c.pkg for c in d["crates"]}]
    strip_attrs = set()
    tfixes = {}
    brokenuse = {}
    tf_report = []
    result = dict(family=name, label=label, cfgs=cfgs, sides={})
    stub = Stub()
    for it in range(iters):
        masked = []
        globs = {}
        for s, d in sides.items():
            rn.wipe(d["dir"])
            os.makedirs(d["dir"], exist_ok=True)
            m_all, g, rfeats, notes = U.build(rn.sb, d["dir"], d["crates"], strip_attrs)
            TR.reset() if s == list(sides)[0] else None
            napplied = 0
            for fpath, fx in tfixes.get(s, {}).items():
                full = os.path.join(d["dir"], fpath)
                if os.path.exists(full):
                    old_t = open(full, encoding="utf-8").read()
                    new_t, k = TR.apply(old_t, fx)
                    napplied += k
                    if k:
                        rn.sb.write(full, new_t)
                        for i_, m_ in enumerate(m_all):
                            pass
            d["napplied"] = napplied
            ball = set()
            for fpath, lnmap in brokenuse.get(s, {}).items():
                full = os.path.join(d["dir"], fpath)
                if os.path.exists(full):
                    ball |= TR.broken_aliases(open(full, encoding="utf-8").read(), lnmap)
            if ball:
                rx_ = re.compile(r"\b(" + "|".join(re.escape(x) for x in sorted(ball)) + r")\b")
                for c_ in d["crates"]:
                    for rel_ in c_.code:
                        full = os.path.join(d["dir"], c_.mod, rel_)
                        if not os.path.exists(full):
                            continue
                        old_t = open(full, encoding="utf-8").read()
                        if not rx_.search(old_t):
                            continue
                        new_t, k = TR.neutralize_aliases(old_t, ball)
                        if k:
                            rn.sb.write(full, new_t)
                            d["napplied"] += k
            masked += m_all
            d["globs"], d["rfeats"], d["notes"] = g, rfeats, notes
        t_scan = time.time()
        stub = Stub()
        stub.scan(masked)
        stub.n_unresolved = TR.COUNTER[0]
        print(f"[{name}] stub scan {round(time.time() - t_scan)}s; build done", flush=True)
        if it == 0:
            defined = set()
            for s, d in sides.items():
                defined |= U.defined_names(d["crates"])
        fills = {}
        for fill_name, (modpath, nm, kind) in args.get("_fills", {}).items():
            if modpath == "mark_trait":
                stub.mark_trait(nm)
            else:
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
        def allnames(kids, acc):
            for k, n in kids.items():
                acc.add(k)
                allnames(n.kids, acc)
            return acc
        internal_idents = set(defined)
        stub_only = allnames(stub.root, set()) - internal_idents
        for s, d in sides.items():
            rn.sb.write(d["dir"] + "/__pos_root.rs", U.root_text(d["crates"], stub.render(), d["rfeats"]))
        results = {}
        for s, d in sides.items():
            for c_ in cfgs:
                test = "test" in c_
                en = U.feature_closure(d["crates"], test, mans)
                for c in d["crates"]:
                    c.features = en[c.pkg]
                feats = U.cfg_args(d["crates"])
                err, sec = rn.run_rustc(d["dir"], CFGS[c_], feats, f"{s}_{c_}")
                diags, reached = parse_json(err, d["dir"], stub_only)
                results[(s, c_)] = dict(diags=diags, canary=sorted(reached), seconds=round(sec), stderr_len=len(err), ice=HIR.ice_items(err))
                log_write(f"{GEN}/{PREFIX}-{name}-{label}-{s}-{c_}-it{it}.err", err[:12_000_000])
                log_write(f"{GEN}/{PREFIX}-{name}-{label}-{s}-{c_}-final.err", err[:30_000_000]); log_write(f"{GEN}/{PREFIX}-{name}-{label}-{s}-{c_}-priv.err", priv_lines(err))
                log_write(f"{GEN}/{PREFIX}-{name}-{label}-{s}-{c_}-it{it}-nonstub.txt", "\n".join(f"#{k} {x['code']} {x['file']}:{x['line']}\n{x['rendered']}" for k, x in enumerate(x for x in diags if not x["stub"])))
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
                m3 = re.match(r"expected trait, found \w+(?: \w+)* `([^`]+)`", dg["msg"])
                if m3:
                    nm3 = m3.group(1).split("::")[-1]
                    if nm3 not in defined and ("trait", nm3) not in fills:
                        fills[("trait", nm3)] = ("mark_trait", nm3, None)
                        new_fill = True
                m4 = re.match(r"expected a type, found a trait", dg["msg"])
                m2 = re.match(r"cannot find (type|trait|value|function|struct, variant or union type|tuple struct or tuple variant|unit struct, unit variant or constant|derive macro) `([^`]+)` in this scope|failed to resolve: use of undeclared type `([^`]+)`", dg["msg"])
                if not m2:
                    continue
                kindword = m2.group(1) or "type"
                nm = m2.group(2) or m2.group(3)
                if nm in defined or nm in ("Self", "self", "super", "crate"):
                    continue
                mod_, f_ = repo_rel(dg["file"], d["dir"])
                gl = d["globs"].get(os.path.join("__m_" + mod_, f_)) if mod_ else None
                if not gl and mod_:
                    _p = f_
                    while not gl:
                        _par = os.path.dirname(os.path.dirname(_p))
                        _cand = os.path.join(_par, "\U0001f980\ufe0f.rs") if _par else "\U0001f980\ufe0f.rs"
                        if _cand == _p:
                            break
                        gl = d["globs"].get(os.path.join("__m_" + mod_, _cand))
                        _p = _cand
                        if not _par:
                            break
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
        new_tfix = False
        for (s, c_), r in results.items():
            for dg in r["diags"]:
                fx = TR.fix_from_diag(dg) if not dg.get("in_macro") else None
                if fx and dg["file"]:
                    bucket = tfixes.setdefault(s, {}).setdefault(dg["file"], set())
                    if fx not in bucket:
                        bucket.add(fx)
                        new_tfix = True
                        tf_report.append(dict(side=s, file=dg["file"], line=dg["line"], path=fx[1], code=dg["code"], msg=dg["msg"][:120]))
        for (s, c_), r in results.items():
            for dg in r["diags"]:
                in_use = False
                if dg["code"] in ("E0432", "E0433", "E0412", "E0405") and dg["line_text"] and dg["file"]:
                    in_use = bool(TR.USE_LINE.match(dg["line_text"]))
                    if not in_use and dg["line"]:
                        fp_ = os.path.join(sides[s]["dir"], dg["file"])
                        if os.path.exists(fp_):
                            in_use = TR.use_statement_at(open(fp_, encoding="utf-8").read(), dg["line"]) is not None
                if in_use:
                    names = None
                    if dg["code"] == "E0432":
                        nm_ = set(x.split("::")[-1] for x in re.findall(r"`([^`]+)`", dg["msg"]))
                        names = nm_ or None
                    cur_ = brokenuse.setdefault(s, {}).setdefault(dg["file"], {})
                    if dg["line"] not in cur_:
                        cur_[dg["line"]] = names
                        new_tfix = True
                        tf_report.append(dict(side=s, file=dg["file"], line=dg["line"], path="(use statement; aliases neutralised in trait positions)", code=dg["code"], msg=dg["msg"][:120]))
        result["iterations"] = it + 1
        if newattr:
            strip_attrs |= newattr
            print(f"[{name}] stripping attributes {sorted(newattr)}", flush=True)
        if new_tfix:
            print(f"[{name}] neutralised {sum(len(v) for b in tfixes.values() for v in b.values())} unresolved trait paths; rebuilding", flush=True)
            continue
        if newattr:
            continue
        if not new_fill or it == iters - 1:
            break
        print(f"[{name}] grew stub globs by {len(fills)} names; re-running", flush=True)
    result["hir_cull"] = {}
    result["ice_bodies"] = {}
    hir_counter = [0]
    dirty = {}
    for s, d in sides.items():
        for c_ in cfgs:
            if len(results[(s, c_)]["canary"]) == 3:
                continue
            rows_all, ice_all = [], []
            hir_done = False
            if dirty.get(s):
                en = U.feature_closure(d["crates"], "test" in c_, mans)
                for c in d["crates"]:
                    c.features = en[c.pkg]
                err, sec = rn.run_rustc(d["dir"], CFGS[c_], U.cfg_args(d["crates"]), f"{s}_{c_}_h")
                diags, reached = parse_json(err, d["dir"], stub_only)
                results[(s, c_)] = dict(diags=diags, canary=sorted(reached), seconds=round(sec), stderr_len=len(err), ice=HIR.ice_items(err))
                log_write(f"{GEN}/{PREFIX}-{name}-{label}-{s}-{c_}-final.err", err[:30_000_000]); log_write(f"{GEN}/{PREFIX}-{name}-{label}-{s}-{c_}-priv.err", priv_lines(err))
                print(f"[{name}] refreshed {s}/{c_} after earlier rewrites: {len(diags)} errors, canary {sorted(reached)}", flush=True)
                if len(reached) == 3:
                    continue
            for rnd in range(int(args.get("hir_rounds", 60))):
                en = U.feature_closure(d["crates"], "test" in c_, mans)
                for c in d["crates"]:
                    c.features = en[c.pkg]
                t_h = time.time()
                if results[(s, c_)].get("ice"):
                    rows = HIR.neutralise_bodies(rn.sb, d["dir"], results[(s, c_)]["ice"], mask)
                    ice_all += rows
                    dirty[s] = True
                    print(f"[{name}] ice round {rnd} {s}/{c_}: {[(r['fn'], r['note']) for r in rows][:3]}", flush=True)
                    if not any(r["note"].startswith("body replaced") for r in rows):
                        break
                elif not hir_done:
                    spans = HIR.scan(d["dir"], [TOOLCHAIN], U.cfg_args(d["crates"]) + CFGS[c_])
                    rows = HIR.rewrite_files(rn.sb, d["dir"], spans, hir_counter, d["dir"] + "/__pos_root.rs")
                    rows_all += rows
                    dirty[s] = dirty.get(s) or bool(rows)
                    hir_done = True
                    print(f"[{name}] hir scan {s}/{c_} round {rnd}: {len(spans)} unresolved trait refs, {len(rows)} rewritten in {round(time.time() - t_h)}s", flush=True)
                    if not rows:
                        break
                else:
                    break
                err, sec = rn.run_rustc(d["dir"], CFGS[c_], U.cfg_args(d["crates"]), f"{s}_{c_}_h")
                diags, reached = parse_json(err, d["dir"], stub_only)
                results[(s, c_)] = dict(diags=diags, canary=sorted(reached), seconds=round(sec), stderr_len=len(err), ice=HIR.ice_items(err))
                log_write(f"{GEN}/{PREFIX}-{name}-{label}-{s}-{c_}-h{rnd}-nonstub.txt", "\n".join(f"#{k} {x['code']} {x['file']}:{x['line']}\n{x['rendered']}" for k, x in enumerate(x for x in diags if not x["stub"])))
                log_write(f"{GEN}/{PREFIX}-{name}-{label}-{s}-{c_}-final.err", err[:30_000_000]); log_write(f"{GEN}/{PREFIX}-{name}-{label}-{s}-{c_}-priv.err", priv_lines(err))
                print(f"[{name}] after hir cull {s}/{c_}: {len(diags)} errors, canary {sorted(reached)}", flush=True)
                if len(reached) == 3:
                    break
            if rows_all:
                result["hir_cull"][f"{s}/{c_}"] = rows_all
            if ice_all:
                result["ice_bodies"][f"{s}/{c_}"] = ice_all
    final = results
    result["quarantined"] = {}
    if args.get("quarantine"):
        for s, d in sides.items():
            blocked = any(not (r["canary"] and len(r["canary"]) == 3) for (ss, cc), r in results.items() if ss == s and cc == cfgs[0])
            if blocked:
                q = quarantine(rn, d, s, cfgs, mans, stub_only, int(args.get("qmax", 30)), name)
                result["quarantined"][s] = q
                if q:
                    for c_ in cfgs:
                        en = U.feature_closure(d["crates"], "test" in c_, mans)
                        for c in d["crates"]:
                            c.features = en[c.pkg]
                        err, sec = rn.run_rustc(d["dir"], CFGS[c_], U.cfg_args(d["crates"]), f"{s}_{c_}_q")
                        diags, reached = parse_json(err, d["dir"], stub_only)
                        results[(s, c_)] = dict(diags=diags, canary=sorted(reached), seconds=round(sec), stderr_len=len(err), ice=HIR.ice_items(err))
                        log_write(f"{GEN}/{PREFIX}-{name}-{label}-{s}-{c_}-q-nonstub.txt", "\n".join(f"#{k} {x['code']} {x['file']}:{x['line']}\n{x['rendered']}" for k, x in enumerate(x for x in diags if not x["stub"])))
                        print(f"[{name}] quarantined {s}/{c_}: {len(diags)} errors, canary {sorted(reached)}", flush=True)
        final = results
    result["strip_attrs"] = sorted(strip_attrs)
    result["trait_neutralised"] = tf_report
    result["stub_glob_fills"] = len(args.get("_fills", {}))
    result["fills"] = [[list(v[0]) if isinstance(v[0], list) else v[0], v[1], v[2]] for v in args.get("_fills", {}).values()]
    for s, d in sides.items():
        result["sides"][s] = dict(crates=[c.pkg for c in d["crates"]], absent=d["absent"], missing_files=d["missing_files"], dup_macros=d["notes"].get("dup_macros", {}), item_macros_removed=d["notes"].get("item_macros_removed", [])[:40], excluded_heads={k: sorted(v) for k, v in d["notes"].get("skipped_local", {}).items() if v})
    rows = {}
    for (s, c_), r in final.items():
        rows.setdefault(c_, {})[s] = r
    report = []
    summary = {}
    for c_ in cfgs:
        cur = rows[c_]["cur"]["diags"]
        base = rows[c_].get("base", {}).get("diags") if "base" in rows[c_] else None
        entry = dict(cur_total=len(cur), cur_noise=sum(1 for d in cur if d["stub"]), seconds=rows[c_]["cur"]["seconds"], canary_cur=rows[c_]["cur"]["canary"], canary_base=rows[c_].get("base", {}).get("canary"))
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
            base_names = {}
            for b_ in base:
                for x_ in re.findall(r"`([^`]+)`", b_["msg"]):
                    base_names.setdefault(b_["code"], set()).add(x_.split("::")[-1])
            for d_ in newlist:
                if d_["cat"] == "resolve" and not d_["stub"]:
                    nm_ = {x_.split("::")[-1] for x_ in re.findall(r"`([^`]+)`", d_["msg"])}
                    if nm_ and nm_ <= base_names.get(d_["code"], set()):
                        d_["inherited"] = True
            entry["new_list"] = newlist
            entry["gone_keys"] = [dict(code=k[0], msg=k[1], label=k[2], n=n) for k, n in sorted(gone.items())]
        else:
            entry["new_list"] = [d for d in cur if not d["stub"]]
        summary[c_] = entry
    result["summary"] = summary
    result["definition_sides"] = {s: d["dir"] for s, d in sides.items()}
    for c_n, e_ in summary.items():
        log_write(f"{GEN}/{PREFIX}-{name}-{label}-new-{c_n}.txt", "\n".join(f"#{k} {x['code']} {x['file']}:{x['line']} stub={x['stub']}\n{x.get('rendered', '')}" for k, x in enumerate(e_.get("new_list", [])[:400])))
        for d_ in e_.get("new_list", []):
            d_.pop("rendered", None)
    out_json = f"{GEN}/{PREFIX}-{name}-{label}-result.json"
    log_write(out_json, json.dumps(result, ensure_ascii=False, indent=1))
    txt = render_text(result)
    log_write(f"{GEN}/{PREFIX}-{name}-{label}-report.txt", txt)
    print(txt)
    return result


def short(f, ln, side_dir=None):
    pkg, rel = repo_rel(f, side_dir)
    return f"[{pkg}] {rel}:{ln}" if pkg else f"{f}:{ln}"


def render_text(result):
    out = [f"== family {result['family']} label {result['label']} iterations {result['iterations']} stub-glob-fills {result['stub_glob_fills']} stripped attrs {result['strip_attrs']}"]
    for s, d in result["sides"].items():
        out.append(f"   side {s}: {len(d['crates'])} crates; absent {d['absent']}; missing files { {k: v for k, v in d['missing_files'].items()} }; dup macros {d['dup_macros']}; excluded heads {d['excluded_heads']}")
    for sc_, rows_ in (result.get("ice_bodies") or {}).items():
        out.append(f"   ICE-BODIES {sc_}: {len(rows_)} fn bodies replaced by `loop {{}}` because rustc panicked (internal compiler error) while type-checking them: " + "; ".join(f"{r['file'][-60:]}:{r['line']} `{r['fn']}` ({r['note']})" for r in rows_[:12]))
    for sc_, rows_ in (result.get("hir_cull") or {}).items():
        out.append(f"   HIR-CULL {sc_}: {len(rows_)} Res::Err trait refs rewritten to stub traits (body type-check was blocked by them): " + "; ".join(f"{r['file'][-70:]}:{r['line']} `{r['path'][:40]}`" for r in rows_[:10]))
    if result.get("quarantined"):
        for s_, q_ in result["quarantined"].items():
            out.append(f"   QUARANTINE {s_}: " + ("canary never reached by emptying files" if q_ is None else f"{len(q_)} file(s) emptied so that body type-check runs: " + "; ".join(x[-110:] for x in q_[:12])))
    for c_, e in result["summary"].items():
        out.append(f"-- cfg {c_}: cur errors {e['cur_total']} (stub-involved {e['cur_noise']}), {e['seconds']}s" + (f"; base errors {e['base_total']}; NEW {e['new']}; GONE {e['gone']}" if "base_total" in e else "; absolute (no baseline)"))
        out.append(f"   typeck canary (E0616,E0624,E0308 expected): cur {e['canary_cur']} base {e['canary_base']}")
        out.append(f"   new errors flagged as stub-layer noise (hidden below): {sum(1 for d in e['new_list'] if d['stub'])}")
        out.append("   by code: " + ", ".join(f"{k}={v}" for k, v in e["by_code"].items()))
        grp = collections.defaultdict(list)
        e["new_stub_filtered"] = sum(1 for d in e["new_list"] if d["stub"])
        e["new_inherited"] = sum(1 for d in e["new_list"] if d.get("inherited") and not d["stub"])
        out.append(f"   new resolve rows whose every name is also unresolved in the baseline (inherited by name, hidden below): {e['new_inherited']}")
        for d in e["new_list"]:
            if not d["stub"] and not d.get("inherited"):
                if is_trait_scope(d):
                    grp["trait-scope"].append(d)
                else:
                    grp[d["cat"]].append(d)
        for cat in ("privacy", "trait-scope", "resolve", "type", "other"):
            lst = grp.get(cat, [])
            out.append(f"   {cat}: {len(lst)}" + (" (non-stub, new)" if "base_total" in e else " (non-stub)"))
            g2 = collections.OrderedDict()
            for d in lst:
                pkg, rel = repo_rel(d["file"], None)
                k = (d["code"], pkg, rel, nrm(d["msg"])[:170])
                g2.setdefault(k, []).append(d["line"])
            for (code, pkg, rel, msg), lines in list(g2.items())[:150]:
                rel_s = rel.split("/")[-4:] if rel else [""]
                out.append(f"      {code} [{pkg.replace('semio_framework_', '')}] .../{'/'.join(rel_s)} x{len(lines)} lines {sorted(set(lines))[:6]}: {msg}")
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
            elif a[i] == "--quarantine":
                args["quarantine"] = True
                i += 1
            elif a[i] == "--qmax":
                args["qmax"] = a[i + 1]
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
