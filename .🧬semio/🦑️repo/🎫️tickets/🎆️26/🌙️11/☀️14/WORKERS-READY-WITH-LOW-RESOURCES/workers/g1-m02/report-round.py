"""🧾 Collects the per-try facts of a round on g1-m02 from the bootstrap logs, the service log parts and the samples into round-<n>.tries.md."""
import glob, os, re, sys
n = sys.argv[1]
home = os.path.expanduser("~"); out = os.path.dirname(os.path.abspath(__file__))
ansi = re.compile(r"\x1b\[[0-9;]*m")
want = re.compile(r"waiting for the fleet|taken from the fleet|inferred here|refreshing \d+ projects|asking the host for|stale facts|nothing to wait|graph-wanted|request-failed|pressure|memory bound|exceeded|restored|adopted|Restarting|serving")
def collapse(line): return re.sub(r"\d+", "N", line)
def distinct(lines, limit):
    seen, kept = set(), []
    for line in lines:
        key = collapse(line)
        if key not in seen: seen.add(key); kept.append(line[:420])
    return kept[:limit] + ([f"… {len(kept) - limit} more distinct lines"] if len(kept) > limit else [])
samples = open(f"{out}/round-{n}.samples.txt", errors="replace").read()
doc = []
k = 0
while True:
    k += 1
    log = f"{home}/native-bootstrap-round-{n}.log" if k == 1 else f"{home}/native-bootstrap-round-{n}-try-{k}.log"
    if not os.path.exists(log): break
    lines = [ansi.sub("", l.rstrip("\n")) for l in open(log, errors="replace")]
    plain = [l for l in lines if not l.startswith("[script] {")]
    end = re.search(rf"TRY {k} END (.*)", samples)
    doc += [f"## Try {k}", "", f"- Log `{log}`: `{lines[0]}`, `{lines[-2]}`, `{lines[-1]}`", f"- Runner: `{end.group(1) if end else 'not recorded'}`"]
    tasks = [l for l in plain if l.startswith("> nx run")]
    doc += [f"- nx task lines: {len(tasks)}", "", "`[capacity]` lines of the bootstrap log:", "```"] + distinct([l for l in plain if l.startswith("[capacity]")], 14) + ["```", "", "Last distinct lines of the bootstrap log:", "```"] + distinct(plain[-400:], 400)[-12:] + ["```", ""]
    part = f"{out}/round-{n}.service-try-{k}.txt"
    if os.path.exists(part):
        service = [ansi.sub("", l.rstrip("\n")) for l in open(part, errors="replace")]
        arrived = [l for l in service if "projects arrived" in l]
        doc += [f"Service log, wanted lines ({len(arrived)} `arrived` lines; first `{arrived[0] if arrived else '-'}`, last `{arrived[-1] if arrived else '-'}`):", "```"] + distinct([l for l in service if want.search(l) and not l.startswith("[script]")], 30) + ["```", "", "Service log, last 30 lines of the try (cut at 300 characters):", "```"] + [l[:300] for l in service[-30:]] + ["```", ""]
        os.remove(part)
open(f"{out}/round-{n}.tries.md", "w").write("\n".join(doc) + "\n")
print(k - 1, "tries")
