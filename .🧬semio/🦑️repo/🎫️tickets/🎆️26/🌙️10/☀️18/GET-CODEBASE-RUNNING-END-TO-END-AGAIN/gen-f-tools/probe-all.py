"""Joint edit proof for the course build (gen-f): python3 probe-all.py <tag> <cycles> [target|-] [baseline execlog for P1]
Cycle k: P (no edit, judged against the previous restore run: expects ZERO executions) -> E (one comment line appended to every probe file at once) -> R (every probe restored, expects ZERO executions).
Probes (probes.json): unrelated TEST files (must change no key) and in-closure files that only one action may read. The edit run is judged per executed action: its moved inputs, resolved to repository paths, must all be probe files allowed for that action, and every action named by a probe must execute.
Peer churn (executor closure, declared inputs edited by others) makes a pair noisy; it is recorded, never hidden, and the next cycle repeats. Files are restored in a finally block and on SIGTERM (only the probe line is removed if a peer edited meanwhile)."""
import glob, hashlib, json, os, re, signal, subprocess, sys, time
tag, cycles = sys.argv[1], int(sys.argv[2])
target = sys.argv[3] if len(sys.argv) > 3 and sys.argv[3] != "-" else "@teaching/architecture-course:build"
baseline = sys.argv[4] if len(sys.argv) > 4 else None
here = os.path.dirname(os.path.abspath(__file__)); root = "/Users/ueli/Documents/semio"; ticket = os.path.dirname(here)
gen = os.path.join(ticket, "🗑️generated", "gen-f", tag); os.makedirs(gen, exist_ok=True)
chain = open(os.path.join(gen, "chain.txt"), "a")
def say(text): chain.write(text + "\n"); chain.flush(); print(text, flush=True)
probes = json.load(open(os.path.join(here, "probes.json"), encoding="utf-8"))
sha = lambda data: hashlib.sha256(data).hexdigest()
marker = lambda: ("\n// gen-f-probe " + tag + "\n").encode()
originals = {}; edited = set()
def restore():
    for name in sorted(edited):
        path = os.path.join(root, probes[name]["file"]); current = open(path, "rb").read(); m = marker()
        if current == originals[name] + m: open(path, "wb").write(originals[name])
        elif m in current: open(path, "wb").write(current.replace(m, b"", 1)); say(f"PEER EDIT during the probe in {name}: only the probe line was removed")
        say(f"restored {name} sha256 {sha(open(path, 'rb').read())[:16]} (original {sha(originals[name])[:16]})")
    edited.clear()
signal.signal(signal.SIGTERM, lambda *_: (restore(), sys.exit(143)))
def identities():
    table = {}
    for f in glob.glob(root + "/.cache/b/p/m/bazel/workspaces/*/*/BUILD.bazel"):
        try: text = open(f, encoding="utf-8").read()
        except Exception: continue
        for name, identity in re.findall(r'name = "(t_[0-9a-f]{16})",.*?\n\s+identity = "([^"]+)"', text, re.S): table.setdefault(name, identity)
    return table
def aliases():
    table = {}
    for f in glob.glob(root + "/.cache/b/p/m/bazel/workspaces/*/*/sources.json"):
        try: data = json.load(open(f, encoding="utf-8"))
        except Exception: continue
        def walk(x):
            if isinstance(x, dict):
                for k, v in x.items():
                    if re.fullmatch(r"[0-9a-f]{10}", k) and isinstance(v, str): table[k] = v
                    else: walk(v)
            elif isinstance(x, list):
                for v in x: walk(v)
        walk(data)
    return table
def run(name):
    directory = os.path.join(gen, name); os.makedirs(os.path.join(directory, "ev"), exist_ok=True)
    env = {k: v for k, v in os.environ.items() if k != "SEMIO_SCRIPT_STALL_MS"}; env["SEMIO_BUILD_EVIDENCE_DIR"] = os.path.join(directory, "ev")
    for attempt in range(1, 6):
        started = time.time()
        with open(os.path.join(directory, "run.log"), "wb") as log:
            code = subprocess.call(["nice", "-n", "10", "bun", "repo", "--tool=bazel", "run", target], cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL)
        text = open(os.path.join(directory, "run.log"), "rb").read().decode("utf-8", "replace")
        say(f"run {name} rc {code} {round(time.time() - started)}s started {time.strftime('%H:%M:%S', time.localtime(started))} attempt {attempt}")
        if code == 0 or not any(t in text for t in ("The lock file of dependency scope", "lock file no longer matches", "Current shared Source-owner generation changed", "Stale loaded Source revision", "Exact original parent child allocation")): break
        say("infrastructure churn before the build: waiting 120 s"); time.sleep(120)
    return code, os.path.join(directory, "ev", "bazel.execlog.jsonl")
def judge(before, after, label):
    out = subprocess.run(["bun", os.path.join(ticket, "cwarm-d-tools", "verdicts.ts"), before, after], cwd=root, capture_output=True, text=True).stdout
    diff = subprocess.run([sys.executable, "-I", os.path.join(ticket, "cwarm-d-tools", "diff-execlogs.py"), before, after], cwd=root, capture_output=True, text=True).stdout
    open(os.path.join(gen, label.replace(" ", "-") + ".diff.txt"), "w").write(diff)
    ids, table = identities(), aliases(); moved = {}; current = None
    for line in diff.splitlines():
        m = re.match(r"== (//:\S+)", line)
        if m: current = m.group(1); moved[current] = []
        m = re.match(r"\s+changed a/([0-9a-f]{10})/(.*)", line)
        if m and current: moved[current].append(table.get(m.group(1), "?" + m.group(1)) + "/" + m.group(2).encode("latin1", "ignore").decode("utf-8", "ignore"))
        elif current and re.match(r"\s+changed (implementation|t_)", line): moved[current].append("(executor identity) " + line.split()[-1])
    executed = {}
    for line in out.splitlines():
        if line.startswith("//:") and " hit " not in line: executed[line.split()[0]] = ids.get(line.split()[0][3:], line.split()[0])
    say(f"## {label}"); say(out)
    for lab, identity in executed.items(): say(f"EXECUTED {identity} ({lab}) moved: " + "; ".join(sorted(set(moved.get(lab, []))))[:1800])
    return executed, moved
try:
    previous = None; clean = False
    for cycle in range(1, cycles + 1):
        if cycle == 1 and baseline: code, p = 0, baseline; say(f"P1 is the given baseline {baseline}")
        else:
            code, p = run(f"P{cycle}")
            if code != 0: say("run failed, stopping"); break
            if previous: judge(previous, p, f"R{cycle-1} to P{cycle}")
        for name, row in probes.items():
            path = os.path.join(root, row["file"]); data = open(path, "rb").read(); originals[name] = data
            open(path, "wb").write(data + marker()); edited.add(name); say(f"edited {name} {row['file']} sha256 {sha(data)[:16]}")
        codeE, e = run(f"E{cycle}"); restore()
        if codeE != 0: say("edit run failed"); break
        executed, moved = judge(p, e, f"P{cycle} to E{cycle} edit")
        want = {}
        for name, row in probes.items():
            for identity in row["actions"]: want.setdefault(identity, set()).add(row["file"])
        problems = []
        for lab, identity in executed.items():
            allowed = want.get(identity)
            if allowed is None: problems.append(f"{identity}: executed but no probe is allowed to move it")
            else:
                stray = [x for x in moved.get(lab, []) if not any(x == a or x.endswith("/" + a) or a in x for a in allowed)]
                if stray: problems.append(f"{identity}: moved by files other than its probes: {stray[:6]}")
        for identity in want:
            if identity not in executed.values(): problems.append(f"{identity}: probe edited but the action did not execute")
        codeR, r = run(f"R{cycle}")
        if codeR != 0: say("restore run failed"); break
        executedR, movedR = judge(e, r, f"E{cycle} to R{cycle} restore")
        if executedR: problems.append("restore run executed: " + ", ".join(sorted(executedR.values())))
        previous = r
        say(f"cycle {cycle}: problems={problems or 'none'}")
        if not problems: clean = True; say("CLEAN CYCLE: only the probed actions executed, unrelated test probes moved nothing, restore executed nothing"); break
        time.sleep(5)
finally:
    restore()
say("probe chain end clean=" + str(clean))
