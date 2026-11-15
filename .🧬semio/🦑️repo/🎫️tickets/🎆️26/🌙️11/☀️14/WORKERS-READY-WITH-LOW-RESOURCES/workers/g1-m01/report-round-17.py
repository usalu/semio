"""📝 Assembles round-17.md, round-17.tail.txt and round-17.bootstrap-tails.txt for g1-m01 from the bootstrap logs, the service logs and the sampler files of round 17. Read-only outside this folder. Usage: python3 report-round-17.py"""
import os, re, glob, json, subprocess, datetime as dt
here = os.path.dirname(os.path.abspath(__file__)); home = os.path.expanduser("~")
hostlogs = os.path.join(os.environ.get("TMPDIR", "/tmp").rstrip("/"), "semio-repo", str(os.getuid()))
ansi = re.compile(r"\x1b\[[0-9;]*[A-Za-z]")
def clean(path, first=1, last=None):
    with open(path, errors="replace") as f:
        return [l for l in (ansi.sub("", x.rstrip("\n")) for x in f.readlines()[first - 1:last]) if not l.startswith("[script] {")]
def utc(day, hms): return dt.datetime.strptime(day + " " + hms, "%Y-%m-%d %H:%M:%SZ").replace(tzinfo=dt.timezone.utc).timestamp()
def cap(lines, n=6, width=700):
    seen, out = set(), []
    for l in lines:
        if l not in seen: seen.add(l); out.append(l[:width])
    return out if len(out) <= 2 * n else out[:n] + [f"… {len(out) - 2 * n} more distinct lines left out …"] + out[-n:]
def block(title, lines):
    return [f"- {title}: none"] if not lines else [f"- {title}:", "", "```", *lines, "```", ""]
samples = open(os.path.join(here, "round-17.samples.txt"), errors="replace").read().split("\n")
tries = {}
for l in samples:
    m = re.match(r"(\d\d:\d\d:\d\dZ) TRY (\d+) start, log (\S+); service log (\S+) from line (\d+)", l)
    if m: tries[int(m[2])] = {"start": m[1], "log": m[3], "service": m[4], "mark": int(m[5])}
    m = re.match(r"(\d\d:\d\d:\d\dZ) TRY (\d+) end: (.*?); hard stop: (.*?); failed tasks: (.*?); reference: (.*)", l)
    if m: tries[int(m[2])].update(end=m[1], result=m[3], stop=m[4], failed=m[5], reference=m[6])
day = "2026-11-15"; out = []; tails = []
after = None
for l in open(os.path.join(here, "round-27.samples.txt"), errors="replace"):
    m = re.match(r"\S+ TRY 1 start, log \S+; service log (\S+) from line (\d+)", l)
    if m: after = {"service": m[1], "mark": int(m[2])}; break
pressure = {}
current = None
for l in samples:
    m = re.match(r"\d\d:\d\d:\d\dZ TRY (\d+) (start|end)", l)
    if m: current = int(m[1]) if m[2] == "start" else None
    m = re.match(r"\d\d:\d\d:\d\dZ pressure=(\d)", l)
    if m and current: pressure.setdefault(current, {}).setdefault(m[1], 0); pressure[current][m[1]] += 1
    m = re.match(r"  run peak rss=(\d+) KiB", l)
    if m and current: tries[current]["runpeak"] = max(tries[current].get("runpeak", 0), int(m[1]))
pick = lambda lines, pattern: [l for l in lines if re.search(pattern, l)]
for k in sorted(tries):
    t = tries[k]; s, e = utc(day, t["start"]), utc(day, t["end"]) + 45
    boot = clean(t["log"]); service = []
    names = []
    for path in sorted(glob.glob(os.path.join(hostlogs, "host-*.log")), key=os.path.getmtime):
        base = os.path.basename(path); mt = os.path.getmtime(path)
        if base == t["service"]:
            nxt = tries.get(k + 1)
            lines = clean(path, t["mark"], nxt["mark"] - 1 if nxt and nxt["service"] == base else None)
            service += lines; names.append(f"{base} from line {t['mark']}")
        elif s <= os.stat(path).st_birthtime <= e:
            nxt = tries.get(k + 1) or after
            lines = clean(path, 1, nxt["mark"] - 1 if nxt and nxt["service"] == base else None)
            service += lines; names.append(base)
    p12 = open(os.path.join(here, f"round-17.p12-try-{k}.txt"), errors="replace").read().split("\n")
    peak, peakline = 0, ""
    for l in p12:
        m = re.search(r"rss=(\d+)MB", l)
        if m and int(m[1]) > peak: peak, peakline = int(m[1]), l
    final = [l for l in boot if not l.startswith(("end ", "native_rc"))][-1] if boot else ""
    wall = int(utc(day, t["end"]) - utc(day, t["start"]))
    out += [f"### Try {k}: {t['result']}", "",
            f"- Start {day}T{t['start']}, end {day}T{t['end']}, wall time {wall} s. Hard stop: {t['stop']}. Failed setup tasks: {t['failed']} (the setup tasks were not reached). Reference: {t['reference']}.",
            f"- Bootstrap log `{t['log'].replace(home, '~')}` ({len(boot)} lines cleaned); service log(s): {', '.join(names) or 'none found'}.",
            f"- Pressure samples by level (30 s): {json.dumps(pressure.get(k, {}), sort_keys=True)}. Largest process of the run in the 30 s samples: {t.get('runpeak', 0) // 1024} MB resident.",
            f"- Slice worker sampler (5 s): peak {peak} MB resident" + (f" at this line:\n\n```\n{peakline[:600]}\n```\n" if peak else " (no slice worker seen)."),
            *[f"- {l}" for l in p12 if l.startswith("closure entries")],
            "- Last message of the bootstrap:", "", "```", final[:1600], "```", ""]
    out += block("Restore line", cap(pick(service, r"restored .* from a state|restored generation|graph: restored"), 3))
    out += block("First plan line (`per project`)", cap(pick(service + boot, r"per project"), 2))
    out += block("`taken from the fleet` / `inferred here` lines", cap(pick(service, r"taken from the fleet|inferred here"), 4))
    out += block("`too heavy` lines", cap(pick(service, r"too heavy"), 3, 900))
    out += block("Plan stage lines", cap(pick(service, r"plan stage"), 12))
    out += block("`waiting for the fleet` lines", cap(pick(service, r"waiting for the fleet"), 3))
    out += block("`waiting for memory headroom` lines", cap(pick(service, r"waiting for memory headroom"), 3))
    out += block("`access-proof`, `where:`, EPERM and refusal lines", cap(pick(service, r"access-proof|where: |EPERM|request-failed|launchd route"), 5))
    out += block("`closure-missing` lines", cap(pick(service, r"closure-missing"), 4))
    out += block("`[capacity]` lines of the bootstrap log", cap(pick(boot, r"^\[capacity\]"), 3))
    if t["result"] in ("native_rc=1", "native_rc=4"):
        tails += [f"##### try {k} {t['result']}: last 40 lines of the bootstrap log", *[l[:900] for l in boot[-40:]], ""]
    if t["reference"] != "none":
        hits = [i for i, l in enumerate(service) if t["reference"] in l]
        if hits: out += block(f"Service log around reference {t['reference']} (20 lines before, 10 after)", [l[:700] for l in service[max(0, hits[0] - 20): hits[0] + 10]])
pre = open(os.path.join(here, "round-17.pre.md")).read().rstrip("\n")
tools = open(os.path.join(here, "round-17.tools.txt")).read().rstrip("\n")
notes = open(os.path.join(here, "round-17.notes.md")).read().rstrip("\n")
table = ["| Try | Start | End | Wall s | Result | Where it ended |", "|---|---|---|---|---|---|"]
where = json.load(open(os.path.join(here, "round-17.where.json")))
for k in sorted(tries):
    t = tries[k]; table.append(f"| {k} | {t['start']} | {t['end']} | {int(utc(day, t['end']) - utc(day, t['start']))} | {t['result']} | {where[str(k)]} |")
open(os.path.join(here, "round-17.md"), "w").write("\n".join([pre, "", notes, "", "## Tries", "", *table, "", "## Step 0: `repo tools setup all`", "", "```", tools, "```", "", "## Per Try", "", *out]) + "\n")
open(os.path.join(here, "round-17.bootstrap-tails.txt"), "w").write("\n".join(tails) + "\n")
last = sorted(tries)[-1]
open(os.path.join(here, "round-17.tail.txt"), "w").write("\n".join(l[:1200] for l in clean(tries[last]["log"])[-200:]) + "\n")
