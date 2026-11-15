"""🛰️ Runs readiness round 8 on g1-m02: the native bootstrap up to ten times with the waits of ROUND.md (consent, fleet refusal, hard stop, moving sources), records the launcher identity around every try, samples memory every 30 s and ends early when a newer round is published before any task ran."""
import glob, json, os, re, signal, subprocess, sys, time, datetime
N = 8
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
    for p in (f"{out}/next.md", f"{w}/ROUND.md"):
        try: m = re.search(r"round:\s*(\d+)", open(p).read())
        except OSError: m = None
        if m and int(m.group(1)) > N: return int(m.group(1))
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
def errors(plain): return sorted({re.sub(r"\d+", "N", l)[:200] for l in plain if l.startswith("error")})
previous_errors, previous_identity = None, None
for k in range(int(sys.argv[1]) if len(sys.argv) > 1 else 1, 11):
    if newer(): note(f"NEWER ROUND {newer()} before try {k}: runner ends"); break
    log = f"{home}/native-bootstrap-round-{N}.log" if k == 1 else f"{home}/native-bootstrap-round-{N}-try-{k}.log"
    before = identity(f"BEFORE TRY {k}")
    if previous_identity is not None: note(f"LAUNCHER since the end of the previous try: {'unchanged' if before == previous_identity else 'CHANGED'}")
    sizes = {p: os.path.getsize(p) for p in glob.glob(f"{service}/host-*.log")}
    child = subprocess.Popen(["bash", f"{w}/native-bootstrap.sh", log], start_new_session=True, preexec_fn=lambda: signal.signal(signal.SIGINT, signal.SIG_DFL))
    note(f"TRY {k} START group {child.pid} log {log}")
    peak, run_peak, stopped, twos, fours = 0, 0, None, 0, 0
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
        if fours >= 2 or twos >= 6: reason = f"memory pressure: level 4 in {fours}, level 2 in {twos} samples in a row"
        if not reason and newer():
            try: started = "> nx run" in open(log, errors="replace").read()
            except OSError: started = False
            if not started: reason = f"round {newer()} was published and no task has started"
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
    after = identity(f"AFTER TRY {k}")
    note(f"LAUNCHER during try {k}: {'unchanged' if after == before else 'CHANGED'}")
    previous_identity = after
    note(f"TRY {k} END native_rc={code} peak_rss_kib={peak} run_peak_rss_kib={run_peak} stop={stopped} fleet_refusal={refusal} nx_task_lines={tasks} error_lines={len(found)} moving={moving}")
    state(code, "consent" if code == 4 else None)
    if code == 0 or k == 10: break
    if stopped and "published" in stopped: break
    if code == 4:
        note("CONSENT: waiting for the owner's click, preflight every 60 s, at most 30 minutes")
        end = time.time() + 1800
        while time.time() < end and not newer():
            access = [l for l in ansi.sub("", sh(f'cd "{worktree}" && PATH="{home}/.local/bin:$PATH" SEMIO_WORKSPACE_ROOT="{worktree}" repo preflight --lang en 2>&1', 300)).splitlines() if re.search(r"\baccess\b", l, re.I)]
            note("preflight access: " + (" | ".join(a[:200] for a in access) or "no access line"))
            if access and all(re.search(r"\bok\b", a, re.I) for a in access): break
            time.sleep(60)
    elif stopped: note("hard stop: waiting 3 minutes"); time.sleep(180)
    elif refusal: note("fleet refusal: waiting 2 minutes"); time.sleep(120)
    elif tasks and moving: note("task failures with moving sources or retry wording: waiting 2 minutes"); previous_errors = found; time.sleep(120)
    elif tasks:
        if previous_errors == found: note("two tries in a row failed with the same task errors: round ends"); break
        previous_errors = found; note("task failures of another kind: one more try in 2 minutes to see whether they repeat"); time.sleep(120)
    else: note("ended without a task, a refusal or a consent text: waiting 2 minutes"); time.sleep(120)
note("ROUND RUNNER END")
