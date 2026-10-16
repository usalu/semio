"""🔬 Patched full-resolution differential for the ui-runtime carve (analysis only, output to /dev/null)."""
import collections, json, os, re, shutil, subprocess, sys
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uirt"
REPO = "/Users/ueli/Documents/semio"
TOOLCHAIN = "+nightly-2026-07-20"
RT = "🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
EXTRA_FIXTURES = ["🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧪️tests/🧫️fixtures", "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement", "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🧪️tests/🧫️fixtures"]

def sandbox_from_repo(name):
    dst = f"{S}/{name}"
    shutil.rmtree(dst, ignore_errors=True)
    os.makedirs(dst)
    for rel in [RT] + EXTRA_FIXTURES:
        src = f"{REPO}/{rel}"
        out = f"{dst}/{rel}"
        os.makedirs(os.path.dirname(out), exist_ok=True)
        shutil.copytree(src, out, ignore=shutil.ignore_patterns("target"))
    return dst

def patch_tree(tree):
    mods = set(); files = []
    for dp, dn, fn in os.walk(tree):
        for name in fn:
            if name.endswith(".rs"):
                path = os.path.join(dp, name); files.append(path)
                text = open(path, encoding="utf-8", errors="ignore").read()
                for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)", text, re.M):
                    mods.add(m.group(1))
    keep = {"crate", "super", "self"} | mods
    glob = re.compile(r"^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$", re.M)
    ext = re.compile(r"^\s*extern crate [^;]+;[ \t]*$", re.M)
    for path in files:
        text = open(path, encoding="utf-8", errors="ignore").read()
        new = ext.sub("", text)
        new = glob.sub(lambda m: m.group(0) if m.group(2) in keep else m.group(1) + "const _: () = ();", new)
        if new != text:
            open(path, "w", encoding="utf-8").write(new)

def run(libroot, crate, extra, root):
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--emit=metadata", "-o", "/dev/null", "--error-format=json", *extra, libroot]
    proc = subprocess.run(cmd, cwd=os.path.dirname(libroot), capture_output=True, text=True)
    out = collections.Counter(); first = {}
    for line in proc.stderr.splitlines():
        try: d = json.loads(line)
        except Exception: continue
        if d.get("level") != "error": continue
        code = (d.get("code") or {}).get("code") or ""
        span = next((s for s in d.get("spans", []) if s.get("is_primary")), None)
        loc = ""; label = ""
        if span:
            fname = span["file_name"]
            loc = os.path.normpath(os.path.join(os.path.dirname(libroot), fname)) if not os.path.isabs(fname) else fname
            loc = loc.replace(root + "/", "")
            label = span.get("label") or ""
        key = (code, d["message"], loc, label)
        out[key] += 1
    return out, proc.stderr

CONFIGS = {"native": [], "test": ["--test"], "testkit": ["--cfg", 'feature="testkit"'], "testkit-test": ["--test", "--cfg", 'feature="testkit"']}
TARGETS = {"wasip2": ["--target", "wasm32-wasip2"], "wasmu": ["--target", "wasm32-unknown-unknown"]}
