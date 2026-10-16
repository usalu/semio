"""Parse proof (read-only): rustc -Zunpretty=normal loads every out-of-line module of P and the companions; rustfmt --check counts parse errors / diff hunks.
usage: parse.py <crate-id>"""
import os, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
import union

cid = sys.argv[1]
c = union.cfg_of(cid)
TOOL = "+nightly-2026-07-20"
rows = [("P-live", f"{union.ROOT}/{c['p_lib']}", union.ROOT), ("P-snap", f"{union.W}/{cid}/snap/{c['p_lib']}", f"{union.W}/{cid}/snap")]
for i, comp in enumerate(c["comps"]):
    rows.append((f"C{i}-live", f"{union.ROOT}/{comp['lib']}", union.ROOT))
for label, lib, cwd in rows:
    if not os.path.exists(lib):
        print(label, "missing", lib)
        continue
    p = subprocess.run(["rustc", TOOL, "-Zunpretty=normal", "--edition", "2021", "--crate-type", "lib", lib], capture_output=True, text=True, cwd=cwd)
    errs = [l for l in p.stderr.split("\n") if l.startswith("error")]
    p2 = subprocess.run(["rustfmt", "--edition", "2021", "--check", lib], capture_output=True, text=True, cwd=cwd)
    fe = [l for l in (p2.stderr + p2.stdout).split("\n") if l.startswith("error")]
    print(f"[{cid}/{label}] unpretty exit={p.returncode} errors={len(errs)} {errs[:3]} | rustfmt exit={p2.returncode} parse-errors={len(fe)} diff-hunks={p2.stdout.count('Diff in')} {fe[:2]}")
