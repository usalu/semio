"""👁️ bench3-g watcher: waits for the landing triggers in coordination.md / result-gate-g.md, then runs one measurement stage at a time with the bench3-g harness.

Stages (state in state.json, override a trigger by touching `go-<stage>`; stop at the deadline or with `stop`):
  print-noop      print-rs:build hot no-op x3   (needs rustedit-g idle)
  course-quietfc  course hot no-op x3           after `GATE quietfc LANDED` / a GREEN gate row naming quietfc
  course-probes   course hot no-op x3 again     after toolsetup `probes` AND verify2-g landed (expect `[bazel] plan reused`)
  course-edit     course one-line edit          after graphlat-g landed (needs agents3-g idle)
  dash-noop       dashboard hot no-op x3        after dashlock-g landed
"""
import json, os, re, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
ROOT = "/Users/ueli/Documents/semio"
OUT = os.path.join(C, "🗑️generated", "bench3-g")
STATE = os.path.join(HERE, "state.json")
DEADLINE = os.environ.get("BENCH3_DEADLINE", "2300")
COORD = os.path.join(C, "coordination.md")
GATE = os.path.join(C, "result-gate-g.md")
RESULT = os.path.join(C, "result-bench3-g.md")
STAGES = [
    ("course-quietfc", ["quietfc"], "noop-course"),
    ("course-probes", ["toolsetup|probes", "verify2"], "noop-course"),
    ("course-edit", ["graphlat"], "edit-course"),
    ("dash-noop", ["dashlock"], "noop-dashboard"),
    ("print-noop", ["print"], "noop-print"),
]


def log(msg):
    print("[%s] %s" % (time.strftime("%H:%M:%S"), msg), flush=True)


def lines(path):
    try:
        with open(path, encoding="utf8", errors="replace") as handle:
            for line in handle:
                yield line[:1200]
    except OSError:
        return


def landed(pattern):
    """First coordination line `GATE <pattern> LANDED` (or a gate-g line saying landed/applied for it), else a gate-table row naming the batch with GREEN and no pending/RED; returns the line or None."""
    for line in lines(COORD):
        if re.search(r"^- \d\d:\d\d ((gate-g|coordinator) )?GATE [^|]*(%s)[^|]* LANDED" % pattern, line, re.I):
            return line.strip()
    for line in lines(GATE):
        if line.startswith("|") and re.search(pattern, line, re.I) and re.search(r"GREEN", line) and not re.search(r"NOT GREEN|pending|RED", line):
            return line.strip()
    return None


def ps_has(rx):
    out = subprocess.run(["ps", "-A", "-o", "pid=,command="], capture_output=True, text=True).stdout
    return any(re.search(rx, l) and "watch.py" not in l and "grep" not in l for l in out.splitlines())


def last_edit_state(lane):
    state = "OFF"
    for line in lines(COORD):
        m = re.search(r"%s EDIT (ON|OFF)" % lane, line)
        if m:
            state = m.group(1)
    return state


def idle(stage):
    if stage == "print-noop":
        return last_edit_state("rustedit-g") == "OFF" and not ps_has(r"rustedit-g-tools") and not ps_has(r"print-rs:build")
    if stage == "course-edit":
        return last_edit_state("agents3-g") == "OFF" and not ps_has(r"agents3-g-tools/agents3\.py") and not ps_has(r"architecture-course")
    return True


def load():
    try:
        return json.load(open(STATE, encoding="utf8"))
    except (OSError, ValueError):
        return {"done": {}}


def append(path, text):
    with open(path, "a", encoding="utf8") as handle:
        handle.write(text)


def announce(msg):
    append(COORD, "- %s bench3-g %s\n" % (time.strftime("%H:%M"), msg))


def finish(name, tag, window):
    d = os.path.join(OUT, tag, "edit" if window.startswith("edit") else "noop")
    summ = ["python3", "-I", os.path.join(HERE, "summarize.py"), d, name]
    table = subprocess.run(summ, capture_output=True, text=True).stdout
    row = subprocess.run(summ + ["--row"], capture_output=True, text=True).stdout.strip()
    report = os.path.join(OUT, tag, "report.json")
    cmp_text = ""
    try:
        subprocess.run(["bun", os.path.join(HERE, "to-report.ts"), d, report, os.path.join(OUT, tag, "report.md")], capture_output=True, text=True, timeout=300, cwd=ROOT)
        base = os.path.join(OUT, "baseline-report.json")
        cmp_text = subprocess.run(["bun", "repo", "bench", "compare", "--current=" + report, "--baseline=" + base], capture_output=True, text=True, timeout=600, cwd=ROOT).stdout[-6000:]
    except Exception as exc:
        cmp_text = "compare failed: %s" % exc
    append(RESULT, "\n### ROW %s (%s, %s)\n\n%s\n\n`repo bench compare` vs the bench2-g baseline:\n\n```\n%s\n```\n" % (name, tag, time.strftime("%H:%M"), table, cmp_text))
    announce("ROW: " + row)


def main():
    state = load()
    os.makedirs(OUT, exist_ok=True)
    log("watching; deadline %s" % DEADLINE)
    while time.strftime("%H%M") < DEADLINE and not os.path.exists(os.path.join(HERE, "stop")):
        for name, patterns, window in STAGES:
            if name in state["done"] or os.path.exists(os.path.join(HERE, "stop")):
                continue
            hits = [landed(p) for p in patterns] if name != "print-noop" else ["now"]
            forced = os.path.exists(os.path.join(HERE, "go-" + name))
            if not forced and not all(hits):
                continue
            if not idle(name):
                log("%s triggered but the lane is not idle" % name)
                continue
            tag = "%s-%s" % (name, time.strftime("%H%M"))
            log("RUN %s trigger=%s" % (name, [h[:160] if h else h for h in hits]))
            if name == "course-edit":
                announce("EDIT ON: one-line edit of the course source (byte backup, restored after the run); agents3-g please do not edit it meanwhile")
            code = subprocess.run(["sh", os.path.join(HERE, "window.sh"), tag, window], cwd=ROOT).returncode
            if name == "course-edit":
                announce("EDIT OFF: course source restored")
            finish(name, tag, window)
            state["done"][name] = {"tag": tag, "at": time.strftime("%H:%M"), "exit": code}
            json.dump(state, open(STATE, "w"), indent=1)
            break
        else:
            time.sleep(60)
            continue
    log("DONE %s" % list(state["done"]))


main()
