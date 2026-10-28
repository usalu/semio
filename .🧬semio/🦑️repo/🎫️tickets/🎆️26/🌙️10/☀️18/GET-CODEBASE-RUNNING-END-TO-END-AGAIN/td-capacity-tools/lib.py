"""🧰 [td-capacity] staging helper: anchored edits of the CURRENT live files into a flat batch (manifest.json + files/ + base/ + stage.py from xplat5-tools/stage.template.py + verify.sh). usage in an edit module: from lib import *; build('<batch>', {rel: [(old, new), ...]}, create={rel: text}, verify='<verify.sh text>')."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
ROOT = C
while not os.path.isdir(os.path.join(ROOT, "🧰️framework")) and os.path.dirname(ROOT) != ROOT: ROOT = os.path.dirname(ROOT)
P = "🧰️framework/🔨️modules/🏃️process/"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def sha(b): return hashlib.sha256(b).hexdigest()
def edit(text, pairs, rel):
    for old, new in pairs:
        n = text.count(old)
        if n != 1: sys.exit(f"anchor x{n} in {rel[-70:]}: {old[:90]!r}")
        text = text.replace(old, new)
    return text
def build(batch, edits, create=None, verify="", root=None):
    root = root or ROOT; out = os.path.join(HERE, "staged", batch)
    shutil.rmtree(out, ignore_errors=True)
    for d in ("files", "base"): os.makedirs(os.path.join(out, d))
    manifest = {}
    for i, (rel, pairs) in enumerate(sorted(edits.items())):
        base = open(os.path.join(root, rel), "rb").read(); new = edit(base.decode("utf8"), pairs, rel).encode("utf8")
        name = f"{i:02d}.ts" if rel.endswith(".ts") else f"{i:02d}.dat"
        open(os.path.join(out, "files", name), "wb").write(new); open(os.path.join(out, "base", name), "wb").write(base)
        manifest[rel] = {"file": name, "base": sha(base), "new": sha(new)}
    for j, (rel, text) in enumerate(sorted((create or {}).items())):
        name = f"n{j:02d}.ts"; new = text.encode("utf8")
        open(os.path.join(out, "files", name), "wb").write(new)
        manifest[rel] = {"file": name, "base": None, "new": sha(new)}
    json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    template = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8").read()
    template = template.replace("Staged batch `universe` (lane rustedit)", f"Staged batch `{batch}` (lane td-capacity)", 1)
    open(os.path.join(out, "stage.py"), "w", encoding="utf8").write(template)
    open(os.path.join(out, "verify.sh"), "w", encoding="utf8").write(verify)
    os.chmod(os.path.join(out, "verify.sh"), 0o755)
    print("staged", batch, len(manifest), "files ->", out)
