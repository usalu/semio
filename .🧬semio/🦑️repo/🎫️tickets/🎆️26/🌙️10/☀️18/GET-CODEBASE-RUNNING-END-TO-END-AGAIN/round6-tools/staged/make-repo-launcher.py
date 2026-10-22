#!/usr/bin/env python3
"""Copy of the repo-launcher set at the ticket root (`r6-repo-launcher-tools`) whose plan.json is rebuilt from the tree the chain has produced by then (make-plan.py: rewrites carry the hash they replace, and peers edited the entrypoint fixture and the Rust entrypoint since staging)."""
import os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/repo-launcher-tools", "r6-repo-launcher-tools", beside=True)
patch(T + "/make-plan.py", "'\\t\"os\"\\n \"path/filepath\"\\n\\t\"time\"\\n', '\\t\"os\"\\n\\t\"os/signal\"\\n\\t\"syscall\"\\n\\t\"time\"\\n')", "'\\t\"os\"\\n \"path/filepath\"\\n \"strings\"\\n\\t\"time\"\\n', '\\t\"os\"\\n\\t\"os/signal\"\\n \"strings\"\\n\\t\"syscall\"\\n\\t\"time\"\\n')")
for name in ("command-form-codemod.py", "make-plan.py", "manifest-neutral.py"):
    patch_if(T + "/" + name, "|repo\\b|--watch", "|--watch")

MP = T + "/make-plan.py"
mp = open(MP, encoding="utf8").read()
lines = mp.split("\n")
keep = [l for l in lines if not (l.startswith('edit("bootstrap", sh, "ensure_go || log') and l.endswith('"ensure_dotnet")'))]
mp = "\n".join(keep)
GOROW = r'ensure_go || log \"Go provisioning failed; install the Go version pinned in $PINS_FILE from https://go.dev/dl and rerun setup.\"\n'
if mp.count(GOROW) == 1:
    mp = mp.replace(GOROW, "")
open(MP, "w", encoding="utf8").write(mp)

mp = open(MP, encoding="utf8").read()
OLDH = '"installs Bun, Node.js, Go, uv and rustup from checksum-pinned archives", "installs Node.js, Go, uv and rustup from checksum-pinned archives and builds'
assert mp.count(OLDH) == 1
mp = mp.replace(OLDH, '"installs Bun, Node.js, uv and rustup from checksum-pinned archives", "installs Node.js, uv and rustup from checksum-pinned archives and builds')
open(MP, "w", encoding="utf8").write(mp)
patch(T + "/command-form-codemod.py", ' (re.compile(r"\\^bun(?=[ (\\\\])"), "^repo"),', ' (re.compile(r"\\^bun repo(?=[ (\\\\])"), "^repo"),\n (re.compile(r"\\^bun(?=[ (\\\\])"), "^repo"),')
run = subprocess.run([sys.executable, T + "/make-plan.py", "--root", repo_root()], capture_output=True, text=True, cwd=T)
if run.returncode:
    raise SystemExit("maker: repo-launcher make-plan.py failed: " + (run.stdout + run.stderr)[-800:])
print("repo-launcher copy:", T, run.stdout.strip()[-200:])
