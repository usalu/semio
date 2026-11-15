"""🧾 Builds `round-<n>.tries.md` for rounds run with the P12 sampler (13 and later): per try the runner's end line, the key lines of the graph service log, refusals, service failures, stalls, the sampler extract with its peak, the `sample` head, the task lines with the first error and 15 lines before it, and the last 40 lines of the bootstrap log; then removes the service log parts."""
import glob, os, re, sys
n = sys.argv[1]
d = os.path.dirname(os.path.abspath(__file__))
home = os.path.expanduser("~")
notes = open(f"{d}/round-{n}.samples.txt", errors="replace").read()
ansi = re.compile(r"\x1b\[[0-9;]*m")
stamp = r"\n20\d\d-\d\d-\d\dT\d\d:\d\d:\d\dZ (P12|ACCESS|LAUNCHER|TRY|SERVICE|PLAN|KEY|STALL|STOP|STORE|NEWER|ROUND|pressure|refusal|another|two tries|dependency|CONSENT)"
def blocks(name, limit=80, width=1500):
    found, start = [], 0
    while True:
        i = notes.find(name, start)
        if i < 0: return found
        m = re.search(stamp, notes[i + len(name):])
        end = i + len(name) + (m.start() if m else 8000)
        found.append("\n".join(l[:width] for l in notes[i:end].splitlines()[1:limit + 1]))
        start = end
def fence(text): return "```\n" + (text or "(none)") + "\n```\n"
def rss(line): return int(re.search(r"rss=(\d+)MB", line).group(1))
def footprint(line):
    m = re.search(r"Footprint: (\d+) MB", line)
    return int(m.group(1)) if m else 0
out = []
k = 0
while True:
    k += 1
    log = f"{home}/native-bootstrap-round-{n}.log" if k == 1 else f"{home}/native-bootstrap-round-{n}-try-{k}.log"
    if not os.path.exists(log) or f"TRY {k} START" not in notes: break
    plain = [ansi.sub("", l) for l in open(log, errors="replace").read().splitlines() if not l.startswith("[script] {")]
    end = re.search(rf"TRY {k} END (.*)", notes)
    out.append(f"## Try {k}\n")
    out.append(f"- Log `{log}`: " + ", ".join(f"`{l}`" for l in plain if re.match(r"(start|end) 20|native_rc=", l)))
    out.append(f"- Runner: `{end.group(1) if end else 'no end line'}`")
    peak = blocks(f"P12 PEAK TRY {k}:", 0)
    line = re.search(rf"P12 PEAK TRY {k}: (.*)", notes)
    out.append(f"- P12 sampler peak: `{line.group(1)[:420] if line else 'not measured'}`\n")
    out.append("Key lines of the graph service log, verbatim, in order:\n")
    out.append(fence("\n".join(blocks(f"KEY LINES TRY {k}:", 120))))
    for b in blocks(f"PLAN TOO HEAVY TRY {k},"): out.append("`graph-plan-too-heavy` with the 20 lines before it:\n"); out.append(fence(b))
    for b in blocks(f"SERVICE FAILURE TRY {k}", 16): out.append("Service failure (3 lines before, the line, the stack):\n"); out.append(fence(b))
    p12 = f"{d}/round-{n}.p12-try-{k}.txt"
    if os.path.exists(p12):
        rows = open(p12, errors="replace").read().splitlines()
        first = next((i for i, l in enumerate(rows) if footprint(l) > 6000), None)
        shown = rows[:3] + (["…"] + rows[max(3, first - 4):] if first is not None else ["… (no footprint above 6000 MB)"] + rows[-2:])
        out.append(f"P12 sampler `round-{n}.p12-try-{k}.txt` ({sum(1 for l in rows if ' rss=' in l)} samples with a slice worker):\n")
        out.append(fence("\n".join(l[:420] for l in shown)))
        for s in sorted(glob.glob(p12 + ".sample-*.txt")):
            lines = open(s, errors="replace").read().splitlines()
            at = next((i for i, l in enumerate(lines) if l.startswith("Sort by top of stack")), len(lines))
            out.append(f"`sample` of the worker `{os.path.basename(s)}` (head and `Sort by top of stack`):\n")
            out.append(fence("\n".join(l[:300] for l in lines[:12] + ["…"] + lines[at:at + 45])))
    tasks = [i for i, l in enumerate(plain) if l.startswith("> nx run")]
    out.append(f"nx task lines: {len(tasks)}\n")
    if tasks:
        out.append(fence("\n".join(dict.fromkeys(plain[i][:300] for i in tasks))))
        errors = [i for i, l in enumerate(plain) if l.startswith("error")]
        seen = set()
        for i in errors:
            owner = max([t for t in tasks if t < i] or [0])
            if owner in seen: continue
            seen.add(owner)
            out.append(f"First error after `{plain[owner][:200]}` with 15 lines before it:\n")
            out.append(fence("\n".join(l[:600] for l in plain[max(0, i - 15):i + 1])))
        summary = [l[:400] for l in plain if re.search(r"Successfully ran|Failed tasks|failed with exit code|tasks? (succeeded|failed)|Ran target|could not be run|not run", l)]
        out.append("Summary lines of the run:\n"); out.append(fence("\n".join(dict.fromkeys(summary))))
    for b in blocks(f"STORE WAIT line:", 60, 300): out.append("Dependency store wait (line and `ps`):\n"); out.append(fence(b))
    out.append("Last 40 lines of the bootstrap log (without `[script] {` rows):\n")
    out.append(fence("\n".join(l[:900] for l in plain[-40:])))
stalls = blocks("STALL last 60 lines", 61, 400)
for i, b in enumerate(stalls): out.append(f"## Stall {i + 1}: Last 60 Lines of the Service Log\n"); out.append(fence(b))
for i, b in enumerate(blocks("STALL ps of the service", 30, 260)): out.append(f"## Stall {i + 1}: `ps` of the Service and the Run\n"); out.append(fence(b))
open(f"{d}/round-{n}.tries.md", "w").write("\n".join(out))
for p in glob.glob(f"{d}/round-{n}.service-try-*.txt"): os.remove(p)
print(k - 1, "tries")
