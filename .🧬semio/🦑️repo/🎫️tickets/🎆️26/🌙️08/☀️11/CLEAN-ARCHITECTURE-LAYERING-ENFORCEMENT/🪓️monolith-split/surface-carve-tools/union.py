# 🔬️ Union differential: the producer crates are simulated as modules of one crate and compared with the original monolith.
# usage: union.py <spec.json> <cfg>...      spec: {"base": [{"root": abs-file, "label": ..}], "split": {"modules": [{"mod": name, "root": abs-file, "aliases": {"extern_name": "mod"}, "restrict": bool}]}}
import sys, os, re, json, subprocess, shutil, collections, tempfile
R = "/Users/ueli/Documents/semio"
SB = os.environ.get("SB", "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad")
CFG = {"native": [], "test": ["--test"], "wasm32": ["--target", "wasm32-unknown-unknown"], "wasip2": ["--target", "wasm32-wasip2"], "test-wasm32": ["--test", "--target", "wasm32-unknown-unknown"]}
MOD_RE = re.compile(r'((?:#\[[^\]\n]*\]\s*)*)(?:pub(?:\([^)\n]*\))?\s+)?mod\s+(\w+)\s*;')
PATH_RE = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
INC_RE = re.compile(r'include!\(\s*"([^"]+)"\s*\)')
def reachable(root):
    seen, order, stack = set(), [], [root]
    while stack:
        f = stack.pop()
        if f in seen or not os.path.isfile(f): continue
        seen.add(f); order.append(f)
        t = open(f, encoding="utf-8").read(); d = os.path.dirname(f)
        for m in MOD_RE.finditer(t):
            pm = PATH_RE.search(m.group(1))
            if pm: stack.append(os.path.normpath(os.path.join(d, pm.group(1))))
        for m in INC_RE.finditer(t): stack.append(os.path.normpath(os.path.join(d, m.group(1))))
    return order
def neutral(t):
    t = re.sub(r'^([ \t]*)((?:#\[[^\]\n]*\][ \t]*)*)(?:pub(?:\([^)\n]*\))?[ \t]+)?use[ \t]+(?:::)?(canvas|ui_styling|semio_\w+|serde|serde_json|pack|protocol)((?:::\w+)*)::\*;', lambda m: m.group(1) + m.group(2) + 'const _: () = ();', t, flags=re.M)
    t = re.sub(r'(?:pub[ \t]+)?use[ \t]+canvas::\{self,[ \t]*\*\};', 'const _: () = ();', t)
    t = re.sub(r'^(?:[ \t]*#\[[^\]\n]*\][ \t]*\n)*[ \t]*(?:pub[ \t]+)?extern[ \t]+crate[ \t]+(?!std\b|core\b|alloc\b)\w+[^;\n]*;', '', t, flags=re.M)
    return t
def build(label, mods, base_mode):
    out = f"{SB}/union-{label}"; shutil.rmtree(out, ignore_errors=True); os.makedirs(out)
    lines = []
    for spec in mods:
        mod, root = spec["mod"], spec["root"]
        files = reachable(root)
        for f in files:
            t = open(f, encoding="utf-8").read()
            t = neutral(t)
            if not base_mode and os.environ.get('MUTATE'):
                for pair in os.environ['MUTATE'].split(';;'):
                    a, b = pair.split('=>'); t = t.replace(a, b)
            if not base_mode and not spec.get("plain"):
                t = re.sub(r'(?<![\w$:])crate::', f'crate::{mod}::', t)
                for ext, tgt in spec.get("aliases", {}).items():
                    t = re.sub(rf'(?<![\w:])(?:::)?{ext}::', f'crate::{tgt}::', t)
                    t = re.sub(rf'(?<![\w:])use\s+{ext}\s+as\s+(\w+)\s*;', lambda m: f'use crate::{tgt} as {m.group(1)};', t)
                if spec.get("restrict"):
                    t = t.replace("pub(crate)", f"pub(in crate::{mod})")
            rel = os.path.relpath(f, spec.get("tree", R))
            dst = os.path.join(out, rel); os.makedirs(os.path.dirname(dst), exist_ok=True)
            open(dst, "w", encoding="utf-8").write(t)
            for im in re.finditer(r'include_(?:str|bytes)!\(\s*"([^"]+)"\s*\)', t):
                src = os.path.normpath(os.path.join(os.path.dirname(f), im.group(1)))
                d2 = os.path.normpath(os.path.join(os.path.dirname(dst), im.group(1)))
                if os.path.isfile(src) and not os.path.exists(d2):
                    os.makedirs(os.path.dirname(d2), exist_ok=True); os.symlink(src, d2)
        lines.append(f'#[path = "{os.path.relpath(root, spec.get("tree", R))}"]\npub mod {mod};')
    # unmodified neighbours that #[path] reaches outside the walked set are not needed: every reachable file was copied
    if base_mode:
        return os.path.join(out, os.path.relpath(mods[0]["root"], mods[0].get("tree", R)))
    open(out + "/union.rs", "w", encoding="utf-8").write("\n".join(lines) + "\n")
    return out + "/union.rs"
IGNORE = set(filter(None, os.environ.get('IGNORE', '').split(',')))
EXT = re.compile(r"cannot find module or crate|unresolved imports? |unlinked crate|aborting due|cannot find attribute|could not find `\w+` in|undeclared crate")
def run(root, cfg, extra=()):
    cmd = ["rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", "union_probe", "--emit=metadata", "-o", "/dev/null", "--error-format=json", "--cap-lints", "allow"] + CFG[cfg] + list(extra) + [root]
    p = subprocess.run(cmd, capture_output=True, text=True, cwd=os.path.dirname(root), env={**os.environ, 'CARGO_MANIFEST_DIR': '/nonexistent'})
    rows = collections.Counter()
    for line in p.stderr.splitlines():
        if not line.startswith("{"): continue
        d = json.loads(line)
        if not str(d.get("level", "")).startswith("error"): continue
        code = (d.get("code") or {}).get("code") or ""
        ps = [x for x in d.get("spans", []) if x.get("is_primary")]
        f, txt = "", ""
        if ps:
            x = ps[0]; f = os.path.normpath(re.sub(r"^.*?/union-(base|split)/", "", x["file_name"]))
            if x.get("text"): tt = x["text"][0]; txt = tt["text"][tt["highlight_start"] - 1:tt["highlight_end"] - 1]
        msg = re.sub(r"crate::\w+::", "crate::", d.get("message", ""))
        if txt in IGNORE or any(('`%s`' % n) in msg for n in IGNORE): continue
        rows[(code, msg[:140], txt[:60], f)] += 1
    return rows
if __name__ == "__main__":
    spec = json.load(open(sys.argv[1]))
    base = build("base", spec["base"], True); split = build("split", spec["split"], False)
    for cfg in sys.argv[2:]:
        extra = spec.get("extra", [])
        a, b = run(base, cfg, extra), run(split, cfg, extra)
        na, nb = sum(a.values()), sum(b.values())
        print(f"[{cfg}] base={na} split={nb}")
        more, less = b - a, a - b
        for k, n in sorted(more.items())[:int(os.environ.get("SHOW", "25"))]: print("  +SPLIT", n, k)
        for k, n in sorted(less.items())[:int(os.environ.get("SHOW", "25"))]: print("  -BASE ", n, k)
        print(f"  split-only={sum(more.values())} base-only={sum(less.values())}")
