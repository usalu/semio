"""🧮️ H13: runs the kernel-db TS source gates (kernel verbs + isolated `verify interactivity` db lanes) and records each gate's
exit code and failure set; `compare` reports failures present after a change that were absent before it.
usage: python3 h13-ts-gates.py run <out.json> | python3 h13-ts-gates.py compare <before.json> <after.json>"""
import json, os, re, subprocess, sys

R = "/Users/ueli/Documents/semio"
KERNEL = f"{R}/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
GATES = [("kernel " + v, KERNEL, ["bun", "./📜️script.ts", v]) for v in ("database-shutdown-check", "wal-committed-compaction-check", "document-mount-single-flight-check")]
GATES += [("interactivity " + v, R, ["bun", "./📜️script.ts", "verify", "interactivity", v]) for v in ("p1q-b1-b6", "p1w", "p1x", "p1y", "p1z")]


def failures(output: str) -> list[str]:
    lines = output.splitlines()
    found = [line for line in lines if line.startswith("error: [verify interactivity")]
    if found:
        return sorted({part.strip() for part in re.sub(r"^error: \[verify interactivity [^\]]+\] ", "", found[-1]).split("; ")})
    for index, line in enumerate(lines):
        if "AssertionError" in line or line.startswith("error:"):
            frame = [l for l in lines[:index] if re.match(r"^\d+ \|", l)]
            return [re.sub(r"^\d+ \|\s*", "", frame[-1]).strip() if frame else line.strip()]
    return []


if sys.argv[1] == "run":
    report = {}
    for name, cwd, command in GATES:
        done = subprocess.run(command, cwd=cwd, capture_output=True, text=True, env={**os.environ, "NX_DAEMON": "false"}, timeout=600)
        output = done.stdout + done.stderr
        report[name] = {"rc": done.returncode, "failures": failures(output) if done.returncode else [], "last": output.strip().splitlines()[-1][:300] if output.strip() else ""}
        print(f"{name}: rc={done.returncode} failures={len(report[name]['failures'])} | {report[name]['last'][:200]}")
    json.dump(report, open(sys.argv[2], "w"), ensure_ascii=False, indent=1)
else:
    before, after = json.load(open(sys.argv[2])), json.load(open(sys.argv[3]))
    new = []
    for name, row in after.items():
        prior = before.get(name, {"rc": 0, "failures": []})
        if row["rc"] and not prior["rc"]:
            new.append(f"{name}: newly red: {row['failures'] or row['last']}")
        new += [f"{name}: {failure}" for failure in row["failures"] if prior["rc"] and failure not in prior["failures"]]
    for line in new:
        print("NEW " + line)
    print(f"[h13 ts gates] new failures: {len(new)}")
    sys.exit(1 if new else 0)
