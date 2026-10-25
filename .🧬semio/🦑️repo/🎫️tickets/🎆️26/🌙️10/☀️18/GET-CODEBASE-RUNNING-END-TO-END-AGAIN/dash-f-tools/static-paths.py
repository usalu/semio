"""Static resolver: every Cargo [lib]/[[bin]]/[[test]] path, #[path], include_str!/include_bytes!/include! literal of tracked+untracked Rust crates under the repo products tree must resolve. usage: static-paths.py <root>..."""
import os, re, subprocess, sys
root = os.getcwd()
files = subprocess.run(["rg", "--files", "-g", "Cargo.toml", "-g", "*.rs", *sys.argv[1:]], capture_output=True, text=True).stdout.split("\n")
bad = 0
for f in files:
    if not f: continue
    d = os.path.dirname(f)
    text = open(f, encoding="utf-8", errors="replace").read()
    if f.endswith("Cargo.toml"):
        for m in re.finditer(r'^\s*path\s*=\s*"([^"]+)"', text, re.M):
            p = os.path.normpath(os.path.join(d, m.group(1)))
            if not os.path.exists(p): print("MISSING", f, "->", m.group(1)); bad += 1
    else:
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]|include_(?:str|bytes)?!\(\s*(?:concat!\(env!\("CARGO_MANIFEST_DIR"\),\s*)?"([^"]+)"', text):
            lit = m.group(1) or m.group(2)
            base = d
            if m.group(2) and "CARGO_MANIFEST_DIR" in m.group(0):
                continue
            p = os.path.normpath(os.path.join(base, lit))
            if not os.path.exists(p): print("MISSING", f, "->", lit); bad += 1
print("bad", bad, "files", len(files))
