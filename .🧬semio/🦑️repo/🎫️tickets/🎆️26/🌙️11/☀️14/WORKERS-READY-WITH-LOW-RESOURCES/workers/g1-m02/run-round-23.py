"""🧱 Runs readiness round 23 (memory pressure is recorded and never a stop, only a process of the run above 8 GB; `waiting for memory headroom` lines are copied; the stall rule applies only when no `graph:` line at all changes for 15 minutes; the fleet wait line with its `server last progressed S s ago` counts as progress; the `settled` plan lines and any refusal are copied too; as round 18; the `where: root=` lines of an access failure are copied with the EPERM lines: `native_rc=4` is recorded with the EPERM lines and the `service host` processes and ends the series only after three in a row; otherwise round 17: a fresh series of twelve tries; `vm_stat` and `memory_pressure` are added when the plan line says 0 MB per project; otherwise round 16) under the file names of round 14 (as round 14; memory pressure level 2 is recorded and no stop; a newer round never interrupts a running try, the runner ends after the try) on g1-m02: the native bootstrap up to twelve times, 3 minutes apart, with the P12 sampler beside every try, the hard stops and the stall rule of ROUND.md, and the key lines of the graph service log copied per try."""
import glob, json, os, re, signal, subprocess, sys, time, datetime
N = 23
home = os.path.expanduser("~")
out = os.path.dirname(os.path.abspath(__file__))
w = os.path.dirname(os.path.dirname(out))
worktree = f"{home}/Documents/semio"
service = "/var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501"
launcher = f"{home}/.local/share/semio/launcher/bin/repo"
samples = open(f"{out}/round-{N}.samples.txt", "a")
ansi = re.compile(r"\x1b\[[0-9;]*m")
def now(): return datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ")
def note(line): samples.write(f"{now()} {line}\n"); samples.flush(); print(now(), line.split("\n")[0][:300], flush=True)
def sh(command, timeout=None):
    try: return subprocess.run(command, shell=True, capture_output=True, text=True, errors="replace", timeout=timeout).stdout
    except subprocess.TimeoutExpired: return "(timed out)\n"
def newer():
    limit = 23
    for p in (f"{out}/next.md", f"{w}/ROUND.md"):
        try: m = re.search(r"round:\s*(\d+)", open(p).read())
        except OSError: m = None
        if m and int(m.group(1)) > limit: return int(m.group(1))
    return 0
def identity(label):
    text = sh(f'ls -li "{launcher}" "{home}/.local/bin/repo" 2>&1; echo "readlink launcher: $(readlink "{launcher}" 2>&1)"; echo "readlink ~/.local/bin/repo: $(readlink "{home}/.local/bin/repo" 2>&1)"; stat -f "launcher target inode %i mtime %Sm size %z" -t %Y-%m-%dT%H:%M:%S -L "{launcher}" 2>&1; codesign -dvvv "{launcher}" 2>&1 | grep -E "Identifier|CDHash"')
    note(f"LAUNCHER {label}\n{text}")
    return re.sub(r"^.*\n", "", text, count=0) and "\n".join(l for l in text.splitlines() if l.startswith(("launcher target", "CDHash", "Identifier", "readlink")))
def group_rss(pgid):
    rows = [line.split() for line in sh("ps -axo pgid,rss").splitlines()[1:]]
    return max([int(r[1]) for r in rows if r[0] == str(pgid)] or [0])
def state(rc, extra=None):
    body = {"origin": "g1-m02", "round": N, "native_rc": rc, "ready": False, "failed": [], "at": now()}
    if extra: body["note"] = extra
    open(f"{out}/state.json", "w").write(json.dumps(body) + "\n")
def progress(log):
    try: lines = [re.sub(r"\(\d+ s\)", "", ansi.sub("", l)) for l in open(log, errors="replace").read().splitlines() if l.strip() and not l.startswith("[script] {") and not re.search(r"still (working|running)", l)]
    except OSError: lines = []
    graph = sh(f"tail -n 400 $(ls -t {service}/host-*.log | head -1) | grep -a '^graph:' | tail -1")
    return (graph,)
def errors(plain): return sorted({re.sub(r"\d+", "N", l)[:200] for l in plain if l.startswith("error")})
previous_errors, previous_identity, consents = None, None, 0
for k in range(int(sys.argv[1]) if len(sys.argv) > 1 else 1, 13):
    if newer(): note(f"NEWER ROUND {newer()} before try {k}: runner ends"); break
    log = f"{home}/native-bootstrap-round-{N}.log" if k == 1 else f"{home}/native-bootstrap-round-{N}-try-{k}.log"
    before = identity(f"BEFORE TRY {k}")
    if previous_identity is not None: note(f"LAUNCHER since the end of the previous try: {'unchanged' if before == previous_identity else 'CHANGED'}")
    sizes = {p: os.path.getsize(p) for p in glob.glob(f"{service}/host-*.log")}
    p12 = f"{out}/round-{N}.p12-try-{k}.txt"
    sampler = subprocess.Popen(["zsh", f"{out}/p12-sampler.zsh", p12], cwd=worktree, start_new_session=True)
    child = subprocess.Popen(["bash", f"{w}/native-bootstrap.sh", log], start_new_session=True, preexec_fn=lambda: signal.signal(signal.SIGINT, signal.SIG_DFL))
    note(f"TRY {k} START group {child.pid} log {log}")
    peak, run_peak, stopped, twos, fours = 0, 0, None, 0, 0
    seen, since, store = None, time.time(), None
    while child.poll() is None:
        top = "\n".join(line.replace("\\012", " ")[:300] for line in sh("ps -axo rss,pid,ppid,args | sort -nr | head -3").splitlines()) + "\n"
        level = sh("sysctl -n kern.memorystatus_vm_pressure_level").strip()
        rss = int(top.split()[0]) if top.split() else 0
        mine = group_rss(child.pid)
        peak, run_peak = max(peak, rss), max(run_peak, mine)
        twos = twos + 1 if level == "2" else 0
        fours = fours + 1 if level == "4" else 0
        samples.write(f"{now()} pressure_level={level} run_group_max_rss_kib={mine}\n{top}"); samples.flush()
        reason = None
        if mine > 8 * 1024 * 1024: reason = f"process of the run above 8 GB resident ({mine} KiB)"
        if False: reason = f"memory pressure: level 4 in {fours}, level 2 in {twos} samples in a row"
        current = progress(log)
        if current != seen: seen, since = current, time.time()
        if not reason and time.time() - since >= 900: reason = f"no new progress line for {int(time.time() - since)} s"
        try: waits = [l for l in open(log, errors="replace").read().splitlines() if "holds the dependency store" in l]
        except OSError: waits = []
        if waits and store is None: store = time.time()
        if not reason and store and time.time() - store >= 300 and not stopped:
            reason = "dependency store wait of 5 minutes"
            note("STORE WAIT line: " + waits[-1][:1500] + "\nps of every bun/nx/repo process\n" + "\n".join(l[:300] for l in sh("ps -axo pid,ppid,etime,command").splitlines() if re.search(r"bun|nx|repo", l)))
        if reason and "no new progress" in reason and not stopped:
            newest = max(glob.glob(f"{service}/host-*.log"), key=os.path.getmtime)
            note("STALL last 60 lines of the graph service log " + newest + "\n" + "\n".join(l[:400] for l in sh(f"tail -60 '{newest}'").splitlines()))
            note("STALL ps of the service and the run\n" + "\n".join(l[:260] for l in sh("ps -axo pid,ppid,pgid,etime,rss,pcpu,command").splitlines() if re.search(r"service host|slice-worker|graph|native-bootstrap|bootstrap", l)))
        if reason and not stopped:
            stopped = reason
            note(f"STOP: {reason}; one SIGINT to group {child.pid}")
            os.killpg(child.pid, signal.SIGINT)
        time.sleep(30)
    text = open(log, errors="replace").read()
    plain = [ansi.sub("", l) for l in text.splitlines() if not l.startswith("[script] {")]
    rc = re.findall(r"native_rc=(\d+)", text)
    code = int(rc[-1]) if rc else -1
    reference = re.findall(r"unexpected failure \(reference ([0-9a-f]+)\)", text)
    refusal = code == 75 or bool(re.search(r"graph-wanted|for the fleet|the fleet was asked", text)) or bool(reference and sh(f"grep -h 'request-failed {reference[-1]} .*graph-wanted' {service}/host-*.log"))
    tasks = sum(1 for l in plain if l.startswith("> nx run"))
    found = errors(plain)
    moving = bool(re.search(r"source-differs|closure\.moving|retry|try again", "\n".join(l for l in plain if l.startswith("error")), re.I))
    with open(f"{out}/round-{N}.service-try-{k}.txt", "w") as part:
        for p in sorted(glob.glob(f"{service}/host-*.log"), key=os.path.getmtime):
            if os.path.getsize(p) > sizes.get(p, 0):
                with open(p, errors="replace") as source: source.seek(sizes.get(p, 0)); part.write(f"=== {p} from byte {sizes.get(p, 0)}\n" + source.read())
    access = [l[:600] for l in (text + open(f"{out}/round-{N}.service-try-{k}.txt", errors="replace").read()).splitlines() if re.search(r"fstatat failed|where: root=|caller's access|app's access|falls? back", l)]
    note(f"ACCESS LINES TRY {k}: {len(access)}\n" + "\n".join(dict.fromkeys(access)))
    open(p12 + ".stop", "w").close()
    try: sampler.wait(timeout=120)
    except subprocess.TimeoutExpired: sampler.terminate()
    part = open(f"{out}/round-{N}.service-try-{k}.txt", errors="replace").read().splitlines()
    key = [l[:1500] for l in part if re.search(r"graph: waiting for memory headroom|graph: plan stage|settled from|Graph: Plan-Stufe|graph-plan-too-heavy|graph: refreshing|graph: [0-9]+ stale projects|taken from the fleet|inferred here|too heavy|\[graph-phase\]|restored|closure-missing|request-failed|graph: generation .* published|nothing to wait", l)]
    note(f"KEY LINES TRY {k}: {len(key)}\n" + "\n".join(key))
    if any("about 0 MB per project" in l for l in part): note(f"ZERO MB TRY {k}\n" + sh("vm_stat") + sh("memory_pressure | tail -3", 60))
    for i, l in enumerate(part):
        if "EPERM" in l: note(f"EPERM TRY {k}, 10 lines before and the line\n" + "\n".join(x[:1500] for x in part[max(0, i - 10):i + 6]))
        if "graph-plan-too-heavy" in l: note(f"PLAN TOO HEAVY TRY {k}, 20 lines before and the line\n" + "\n".join(x[:1500] for x in part[max(0, i - 20):i + 1]))
        if "request-failed" in l: note(f"SERVICE FAILURE TRY {k}\n" + "\n".join(x[:1500] for x in part[max(0, i - 3):i + 12]))
    rows = [l for l in open(p12, errors="replace").read().splitlines() if " rss=" in l]
    top = max(rows, key=lambda l: int(re.search(r"rss=(\d+)MB", l).group(1)), default="no slice worker seen")
    note(f"P12 PEAK TRY {k}: {top[:400]}")
    missing = any("closure-missing" in l or "build-input-closures" in l for l in part)
    failures = sorted({re.sub(r"^\S+ request-failed \S+ ", "", l)[:200] for l in part if "request-failed" in l})
    after = identity(f"AFTER TRY {k}")
    note(f"LAUNCHER during try {k}: {'unchanged' if after == before else 'CHANGED'}")
    previous_identity = after
    note(f"TRY {k} END native_rc={code} peak_rss_kib={peak} run_peak_rss_kib={run_peak} stop={stopped} fleet_refusal={refusal} closure_missing={missing} nx_task_lines={tasks} error_lines={len(found)} moving={moving}")
    state(code, "consent" if code == 4 else None)
    if code == 0 or k == 12: break
    if newer(): note(f"NEWER ROUND {newer()} after try {k}: runner ends"); break
    consents = consents + 1 if code == 4 else 0
    if code == 4:
        note(f"CONSENT TRY {k} ({consents} in a row), ps of the service host\n" + sh('ps -axo pid,ppid,etime,command | grep "service host" | cut -c1-400'))
        if consents >= 3: note("three native_rc=4 in a row: round ends"); break
        note("native_rc=4: next try in 3 minutes"); time.sleep(180); continue
    signature = (found, failures)
    if (stopped and "dependency store" in stopped): note("dependency store wait: next try in 3 minutes"); time.sleep(180)
    elif stopped or code == 75 or refusal or missing or (tasks and moving): previous_errors = None; note("refusal, hard stop, stall, missing closure or moving sources: next try in 3 minutes"); time.sleep(180)
    elif previous_errors == signature: note("two tries in a row with the same other error: round ends"); break
    else: previous_errors = signature; note("another error: one more try in 3 minutes to see whether it repeats"); time.sleep(180)
note("ROUND RUNNER END")
