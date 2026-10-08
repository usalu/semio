#!/usr/bin/env python3
"""M-2 helper: resolve a list of dashboard commands with `semio run --dry-run` and print a compact summary.

Usage: python r2-m2-resolve.py <semio-binary> <cases.json> [out.jsonl]
A case is {"name": str, "id": str, "params": {k: v}, "env": {K: V}, "args": [str], "expectPort": int|null}.
"""
import json
import subprocess
import sys

ROOT = "C:/git/semio"


def resolve(binary, case):
    argv = [binary, "run", case["id"], "--dry-run", "--root", ROOT]
    for key, value in case.get("params", {}).items():
        argv += ["--param", f"{key}={value}"]
    for key, value in case.get("env", {}).items():
        argv += ["--env", f"{key}={value}"]
    if case.get("args"):
        argv += ["--", *case["args"]]
    done = subprocess.run(argv, capture_output=True, text=True, encoding="utf-8", cwd=ROOT)
    if done.returncode != 0:
        return {"ok": False, "code": done.returncode, "error": (done.stderr or done.stdout).strip()[:400]}
    launch = json.loads(done.stdout)
    return {"ok": True, "launch": launch}


def summarize(case, result):
    if not result["ok"]:
        return {"name": case["name"], "id": case["id"], "ok": False, "error": result["error"]}
    launch = result["launch"]
    last = launch["processes"][-1]
    readies = [p.get("ready") for p in launch["processes"] if p.get("ready")]
    port = readies[-1]["port"] if readies else None
    return {
        "name": case["name"],
        "id": case["id"],
        "ok": True,
        "cmd": " ".join([last["cmd"], *last["args"]])[:260],
        "env": {k: v for k, v in last["env"] if not k.startswith(("NX_", "VITE_"))},
        "ready": readies[-1] if readies else None,
        "requires": [r["commandId"] for r in launch.get("requires", [])],
        "processes": len(launch["processes"]),
        "portOk": None if case.get("expectPort") is None else port == case["expectPort"],
    }


def main():
    binary, cases_path = sys.argv[1], sys.argv[2]
    out = open(sys.argv[3], "w", encoding="utf-8") if len(sys.argv) > 3 else None
    cases = json.load(open(cases_path, encoding="utf-8"))
    for case in cases:
        summary = summarize(case, resolve(binary, case))
        line = json.dumps(summary, ensure_ascii=False)
        print(line)
        if out:
            out.write(line + "\n")


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8")
    main()
