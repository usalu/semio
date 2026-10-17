"""🔭 c4p29: which crate lib roots (the three stdio artifacts) mount each target file, and each crate's lib root path. Read only."""
import os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *
from c4psplit import lib_files

out = subprocess.run(["git", "-c", "core.fsmonitor=false", "ls-files", "--", f"{ART}/📖️pdf", f"{ART}/📜️docx"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
mans = [p for p in out if p.endswith("/Cargo.toml")]
targets = {ROOT + "/" + k: v for k, v in KEYS.items()}
for m in mans:
    t = read(ROOT + "/" + m)
    lm = re.search(r"^\[lib\][^\[]*?^path\s*=\s*\"([^\"]+)\"", t, re.M | re.S)
    nm = re.search(r"^\[package\][^\[]*?^name\s*=\s*\"([^\"]+)\"", t, re.M | re.S)
    if not lm:
        continue
    lib = os.path.normpath(os.path.join(os.path.dirname(ROOT + "/" + m), lm.group(1)))
    if not os.path.isfile(lib):
        print("NOLIB", m[-80:], lib[-60:])
        continue
    cl = lib_files(os.path.relpath(lib, ROOT))
    hit = [targets[os.path.normpath(p)] for p in cl if os.path.normpath(p) in targets]
    print(f"{nm.group(1) if nm else '?':50} files={len(cl):4} lib={os.path.relpath(lib, ROOT)[-70:]} targets={hit}")
