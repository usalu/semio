#!/usr/bin/env python3
"""✅️ Laws of a staged batch on a root (live or mirror), without a test framework: python3 -I verify_batch.py <batch dir> <root>.
Applied or base state is checked by stage.py; this checks the files as they are on the root: parse (C/gate-g-tools/scan.ts), docstring openers + emoji reuse + [DEBUG] (cp-ast.ts rows), and for every file named in the manifest that the batch touched."""
import json, os, subprocess, sys, tempfile
batch, root = sys.argv[1], sys.argv[2]
here = os.path.dirname(os.path.abspath(__file__))
c = os.path.dirname(here)
manifest = json.load(open(os.path.join(batch, "manifest.json"), encoding="utf8"))
rels = [r for r, row in manifest.items() if row["new"] is not None]
with tempfile.TemporaryDirectory() as scratch:
    listing = os.path.join(scratch, "l.txt")
    open(listing, "w", encoding="utf8").write("\n".join(rels) + "\n")
    parse = [r for r in rels if r.endswith((".ts", ".mts", ".mjs")) and not r.endswith(".d.mts")]
    open(os.path.join(scratch, "p.txt"), "w", encoding="utf8").write("\n".join(os.path.join(root, r) for r in parse) + "\n")
    run = subprocess.run(["bun", os.path.join(c, "gate-g-tools", "scan.ts"), os.path.join(scratch, "p.txt"), os.path.join(scratch, "s")], capture_output=True, text=True, cwd=c)
    print(run.stdout.strip().splitlines()[-1] if run.stdout.strip() else run.stderr[-300:])
    if "bad 0" not in run.stdout: print(run.stdout[-1500:]); sys.exit(1)
    js = [r for r in rels if r.endswith((".ts", ".mts", ".mjs"))]
    open(os.path.join(scratch, "j.txt"), "w", encoding="utf8").write("\n".join(js) + "\n")
    out = os.path.join(scratch, "ast.jsonl")
    run = subprocess.run(["bun", os.path.join(here, "cp-ast.ts"), os.path.join(scratch, "j.txt"), out], capture_output=True, text=True, env=dict(os.environ, CP_ROOT=root), cwd=c)
    if run.returncode != 0: print(run.stderr[-500:]); sys.exit(1)
    bad = 0
    for line in open(out, encoding="utf8"):
        row = json.loads(line)
        for x in row.get("rows", []):
            if x["rule"] in ("dup-emoji", "doc-not-emoji", "debug-tag"):
                bad += 1; print("FAIL", x["rule"], row["file"][-70:], x["line"], x["detail"][:60])
    run = subprocess.run(["bun", os.path.join(here, "cp-docs-law.ts"), root, listing], capture_output=True, text=True, cwd=c)
    print(run.stdout.strip().splitlines()[-1] if run.stdout.strip() else run.stderr[-300:])
    if run.returncode != 0: bad += 1; print(run.stdout[-1200:])
    if any(r.endswith("🔒️allowlist/🔣️.json") for r in rels):
        law = os.path.join(scratch, "lawfull.json")
        run = subprocess.run(["bun", os.path.join(here, "cp-lawfull.ts"), law], capture_output=True, text=True, cwd=c, env=dict(os.environ, CP_ROOT=root))
        if run.returncode != 0: print(run.stderr[-400:]); sys.exit(1)
        own = [f for f in json.load(open(law, encoding="utf8"))["findings"] if f["path"] in set(rels)]
        for f in own: bad += 1; print("FAIL language-law", f["rule"], f["detail"], f["path"][-70:])
        print("language law: %d findings on the files of this batch" % len(own))
    py = [r for r in rels if r.endswith(".py")]
    if py:
        open(os.path.join(scratch, "py.txt"), "w", encoding="utf8").write("\n".join(py) + "\n")
        out2 = os.path.join(scratch, "other.jsonl")
        env = dict(os.environ, CP_ROOT=root)
        subprocess.run([sys.executable, "-I", os.path.join(here, "cp-other.py"), os.path.join(scratch, "py.txt"), out2], env=env, check=True)
        for line in open(out2, encoding="utf8"):
            row = json.loads(line)
            for x in row.get("rows", []):
                if x["rule"] in ("dup-emoji", "doc-not-emoji", "comment-in-body", "debug-tag"):
                    bad += 1; print("FAIL", x["rule"], row["file"][-70:], x["line"], x["detail"][:60])
        for r in py:
            compile(open(os.path.join(root, r), encoding="utf8").read(), r, "exec")
    print("verify_batch: %d files, %d findings" % (len(rels), bad))
    sys.exit(1 if bad else 0)
