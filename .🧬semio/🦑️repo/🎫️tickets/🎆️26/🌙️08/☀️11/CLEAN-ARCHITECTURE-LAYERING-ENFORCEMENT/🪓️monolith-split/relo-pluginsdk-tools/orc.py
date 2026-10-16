"""🔬 Full-resolution differential oracle for slice E (plugin SDK relocation).

Builds a patched sandbox of the plugin tree (from the live repo or the baseline snapshot), symlinking
everything else to the real repo, then runs analysis-only rustc (`--emit=metadata -o /dev/null`) and
prints the error multiset keyed by (code, message, label, primary snippet), file path ignored so moved
code still compares.

usage: orc.py build <src_root> <sbx_dir>
       orc.py run <sbx_dir> <crate_name> <libroot_rel> <config> <out.json>
       orc.py diff <base.json> <cur.json> [--non-extern]   (drops unresolved-external-crate keys)
"""
import json, os, re, shutil, subprocess, sys
from collections import Counter

REPO = "/Users/ueli/Documents/semio"
CHAIN = ["🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🔌️plugin"]
EXTS = (".rs", ".wit", ".json", ".toml", ".ts", ".feature", ".txt", ".md", ".graphql", ".proto", ".sql", ".wat")
KEEP_ROOTS = {"crate", "super", "self", "std", "core", "alloc"}
RX_GLOB = re.compile(r'^([ \t]*)((?:#\[[^\]\n]*\][ \t]*)*)(?:pub(?:\([^)\n]*\))?[ \t]+)?use[ \t]+(?:::)?(\w+)((?:::\w+)*)::\*;', re.M)
RX_EXT = re.compile(r'^(?:[ \t]*#\[[^\]\n]*\][ \t]*\n)*[ \t]*(?:#\[[^\]\n]*\][ \t]*)*(?:pub[ \t]+)?extern[ \t]+crate[ \t]+(\w+)[^;\n]*;', re.M)
RX_NEST = re.compile(r'^([ \t]*(?:#\[[^\]\n]*\][ \t]*)*(?:pub(?:\([^)\n]*\))?[ \t]+)?use[ \t]+(?:::)?)(\w+)((?:::\w+)*::\{)([^;]*)\};', re.M)
RX_DEF = re.compile(r'\b(?:mod|enum|struct|union|trait|type|fn|const|static)\s+(\w+)')


def build(src_root, sbx):
    shutil.rmtree(sbx, ignore_errors=True)
    os.makedirs(sbx)
    real, fake = REPO, sbx
    for i, comp in enumerate(CHAIN):
        for entry in os.listdir(real):
            if entry == comp:
                continue
            os.symlink(os.path.join(real, entry), os.path.join(fake, entry))
        real, fake = os.path.join(real, comp), os.path.join(fake, comp)
        if i < len(CHAIN) - 1:
            os.makedirs(fake)
    src = os.path.join(src_root, *CHAIN)
    names, files = set(), []
    for d, dirs, fs in os.walk(src):
        dirs[:] = [x for x in dirs if x not in ("node_modules", "target")]
        for f in fs:
            if f.endswith(EXTS):
                p = os.path.join(d, f)
                rel = os.path.relpath(p, src)
                files.append(rel)
                if f.endswith(".rs"):
                    names |= set(RX_DEF.findall(open(p, encoding="utf-8").read()))
    keep = KEEP_ROOTS | names
    stats = Counter()
    for rel in files:
        s, t = os.path.join(src, rel), os.path.join(fake, rel)
        os.makedirs(os.path.dirname(t), exist_ok=True)
        if not rel.endswith(".rs"):
            shutil.copyfile(s, t)
            continue
        text = open(s, encoding="utf-8").read()

        def g(m):
            if m.group(3) in keep:
                return m.group(0)
            stats["globs"] += 1
            return m.group(1) + m.group(2) + "const _: () = ();"

        def e(m):
            if m.group(1) in ("std", "core", "alloc", "proc_macro", "self"):
                return m.group(0)
            stats["externs"] += 1
            return ""

        def n(m):
            if m.group(2) in keep:
                return m.group(0)
            body = re.sub(r'(^|,)\s*\*\s*(?=,|$)', lambda q: q.group(1), m.group(4))
            if body != m.group(4):
                stats["nested"] += 1
            return m.group(1) + m.group(2) + m.group(3) + body + "};"

        text = RX_NEST.sub(n, RX_EXT.sub(e, RX_GLOB.sub(g, text)))
        open(t, "w", encoding="utf-8").write(text)
    print("built", sbx, "files", len(files), dict(stats))


def run(sbx, crate, libroot_rel, config, out):
    args = ["rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--emit=metadata", "-o", "/dev/null", "--error-format=json", "--cap-lints", "allow"]
    for part in config.split("+"):
        if part == "test":
            args.append("--test")
        elif part.startswith("wasm"):
            args += ["--target", part]
        elif part.startswith("feat:"):
            for f in part[5:].split(","):
                args += ["--cfg", 'feature="%s"' % f]
    args.append(os.path.join(sbx, libroot_rel))
    p = subprocess.run(args, capture_output=True, text=True, cwd=sbx)
    keys = []
    for line in p.stderr.splitlines():
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("level") != "error":
            continue
        code = (d.get("code") or {}).get("code", "")
        span = next((s for s in d.get("spans", []) if s.get("is_primary")), None)
        label = (span or {}).get("label") or ""
        snippet = ""
        if span and span.get("text"):
            t = span["text"][0]
            snippet = t["text"][t["highlight_start"] - 1:t["highlight_end"] - 1]
        fname = (span or {}).get("file_name", "")
        keys.append({"code": code, "msg": d.get("message", ""), "label": label, "snip": snippet, "file": fname, "line": (span or {}).get("line_start")})
    json.dump(keys, open(out, "w"), ensure_ascii=False, indent=0)
    print(config, "errors", len(keys))


def key(k):
    return (k["code"], k["msg"], k["label"], k["snip"])


def external(k):
    return k["code"] in ("E0432", "E0433") and "unresolved module or unlinked crate" in (k["label"] + k["msg"]) or k["msg"].startswith("aborting due to")


def diff(a, b, non_extern=False):
    A, B = json.load(open(a)), json.load(open(b))
    if non_extern:
        A, B = [x for x in A if not external(x)], [x for x in B if not external(x)]
    ca, cb = Counter(map(key, A)), Counter(map(key, B))
    print("base", len(A), "cur", len(B), "distinct", len(ca), len(cb))
    new, gone = cb - ca, ca - cb
    for label, c, src in (("NEW", new, B), ("GONE", gone, A)):
        for k, v in sorted(c.items()):
            where = [x for x in src if key(x) == k][:3]
            print(label, v, k, [("/".join(w["file"].split("/")[-3:-1]), w["line"]) for w in where])


if __name__ == "__main__":
    cmd = sys.argv[1]
    if cmd == "build":
        build(sys.argv[2], sys.argv[3])
    elif cmd == "run":
        run(*sys.argv[2:7])
    elif cmd == "diff":
        diff(sys.argv[2], sys.argv[3], "--non-extern" in sys.argv)
