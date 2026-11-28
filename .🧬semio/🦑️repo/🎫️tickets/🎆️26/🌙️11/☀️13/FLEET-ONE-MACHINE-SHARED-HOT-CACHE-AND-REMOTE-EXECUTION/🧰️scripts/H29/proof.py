# -*- coding: utf-8 -*-
import json, os, subprocess, sys, time

S = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
file = sys.argv[1]
modes = sys.argv[2:] or ["never", "always"]
rows = {}
for mode in modes:
    env = {**os.environ, "SEMIO_TEST_FILE_OFFLOAD": mode, "SEMIO_LANGUAGE": "en"}
    started = time.time()
    run = subprocess.run(["nice", "-n", "10", os.path.expanduser("~/.local/bin/repo"), "test-file", "--json", "--", file], cwd=ROOT, env=env, capture_output=True, text=True)
    seconds = round(time.time() - started, 1)
    open("%s/proof-%s.err" % (S, mode), "w", encoding="utf-8").write(run.stderr)
    try:
        document = json.loads(run.stdout)
    except Exception:
        document = {"unparsable": run.stdout[:300]}
    rows[mode] = {"exit": run.returncode, "wall": seconds, "document": document}
    summary = [(row.get("file", "")[-40:], row.get("suite"), row.get("status"), row.get("pass"), row.get("fail"), row.get("ranOn")) for row in document.get("files", [])]
    print(mode, "exit", run.returncode, "wall", seconds, "status", document.get("status"), "counts", document.get("counts"), summary)
    notes = [line for line in run.stderr.splitlines() if "[test-file]" in line]
    print("  notices:", notes[:6])
json.dump(rows, open(S + "/proof.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
if "never" in rows and "always" in rows:
    local = [(r["suite"], r["status"], r.get("pass"), r.get("fail")) for r in rows["never"]["document"].get("files", [])]
    remote = [(r["suite"], r["status"], r.get("pass"), r.get("fail")) for r in rows["always"]["document"].get("files", [])]
    print("same rows:", local == remote, "same exit:", rows["never"]["exit"] == rows["always"]["exit"])
