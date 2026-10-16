"""🔬 Patched full-resolution differential helpers for the replication carve (analysis only, rustc output goes to /dev/null)."""
import collections, json, os, re, subprocess
from rlib import SP, R

TOOLCHAIN = "+nightly-2026-07-20"

def patch_tree(tree, keep_extra=()):
    mods = set(); files = []
    for dp, dn, fn in os.walk(tree):
        for name in fn:
            if name.endswith(".rs"):
                path = os.path.join(dp, name); files.append(path)
                text = open(path, encoding="utf-8", errors="ignore").read()
                for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)", text, re.M):
                    mods.add(m.group(1))
    keep = {"crate", "super", "self", "protocol"} | mods | set(keep_extra)
    glob = re.compile(r"^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$", re.M)
    ext = re.compile(r"^\s*extern crate (?!self )[^;]+;[ \t]*$", re.M)
    for path in files:
        text = open(path, encoding="utf-8", errors="ignore").read()
        new = ext.sub("", text)
        new = glob.sub(lambda m: m.group(0) if m.group(2) in keep else m.group(1) + "const _: () = ();", new)
        if new != text:
            open(path, "w", encoding="utf-8").write(new)

def run(libroot, name, extra, cwd=None):
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", name, "--emit=metadata", "-o", "/dev/null", "--error-format=json", *extra, libroot]
    proc = subprocess.run(cmd, cwd=cwd or os.path.dirname(libroot), capture_output=True, text=True)
    out = collections.Counter()
    for line in proc.stderr.splitlines():
        try: d = json.loads(line)
        except Exception: continue
        if d.get("level") != "error": continue
        code = (d.get("code") or {}).get("code") or ""
        span = next((s for s in d.get("spans", []) if s.get("is_primary")), None)
        loc = ""
        if span:
            fname = span["file_name"]
            loc = os.path.normpath(os.path.join(os.path.dirname(libroot), fname)) if not os.path.isabs(fname) else fname
            loc = loc.split("/" + R)[-1] if "/" + R in loc else loc
        out[(code, d["message"], loc)] += 1
    return out, proc.stderr

CFGS = {
    "native": ["--cfg", 'feature="deflate"'],
    "native-nodeflate": [],
    "test": ["--test", "--cfg", 'feature="deflate"'],
    "wasm": ["--target", "wasm32-unknown-unknown", "--cfg", 'feature="deflate"'],
    "wasm-test": ["--test", "--target", "wasm32-unknown-unknown", "--cfg", 'feature="deflate"'],
    "wasip2": ["--target", "wasm32-wasip2", "--cfg", 'feature="deflate"'],
}
