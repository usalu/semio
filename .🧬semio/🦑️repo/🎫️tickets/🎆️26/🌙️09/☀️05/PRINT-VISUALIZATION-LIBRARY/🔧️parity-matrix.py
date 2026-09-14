"""📊️ Renders the case × scenario matrix of the print owner from the platform's own evidence, so the
report is the run and not a retyping of it. Execution status comes from
`⚡️cache/tests/results/*/📤️results.jsonl`; a scenario is marked `differs` when the runner left a
`⚡️cache/tests/diffs/*.diff.txt` newer than that case's result record, which is exactly the
condition under which the run printed `parity failed` for it."""
import io, json, os, re, sys, collections

ROOT = sys.argv[1] if len(sys.argv) > 1 else "C:/git/semio"
CACHE = os.path.join(ROOT, ".\U0001f9ecsemio", "\U0001f991\ufe0frepo", "\u26a1\ufe0fcache", "tests")
RESULTS = os.path.join(CACHE, "results")
DIFFS = os.path.join(CACHE, "diffs")

diffs = {}
if os.path.isdir(DIFFS):
    for name in os.listdir(DIFFS):
        diffs[name] = os.path.getmtime(os.path.join(DIFFS, name))


def diff_key(case, scenario):
    slug = re.sub(r"[^A-Za-z0-9]+", "_", case + "::" + scenario + "::typescript::subject")
    return slug


rows = collections.defaultdict(dict)
stamps = {}
for name in os.listdir(RESULTS):
    path = os.path.join(RESULTS, name, "\U0001f4e4\ufe0fresults.jsonl")
    if not os.path.isfile(path):
        continue
    for line in io.open(path, encoding="utf-8"):
        line = line.strip()
        if not line:
            continue
        record = json.loads(line)
        if "\U0001f4d3\ufe0fprint" not in record.get("owner", ""):
            continue
        rows[record["case"]][(record["scenario"], record["role"])] = record["status"]
        stamps[record["case"]] = max(stamps.get(record["case"], 0), os.path.getmtime(path))

print("| case | scenarios | subject × scenario |")
print("|---|---|---|")
total = failing = 0
for case in sorted(rows):
    scenarios = sorted({key[0] for key in rows[case]})
    cells = []
    for scenario in scenarios:
        total += 1
        subject = rows[case].get((scenario, "subject"), "-")
        oracle = rows[case].get((scenario, "oracle"), "-")
        key = diff_key(case, scenario)
        differs = any(name.endswith(key + ".diff.txt") and stamp >= stamps.get(case, 0) - 5 for name, stamp in diffs.items())
        if subject != "passed" or oracle not in ("passed", "-"):
            mark = "**" + subject + "**"
            failing += 1
        elif differs:
            mark = "**differs**"
            failing += 1
        else:
            mark = "ok"
        cells.append(scenario + " " + mark)
    print("| `" + case + "` | " + str(len(scenarios)) + " | " + ", ".join(cells) + " |")
print()
print("cases " + str(len(rows)) + ", scenarios " + str(total) + ", not green " + str(failing))
