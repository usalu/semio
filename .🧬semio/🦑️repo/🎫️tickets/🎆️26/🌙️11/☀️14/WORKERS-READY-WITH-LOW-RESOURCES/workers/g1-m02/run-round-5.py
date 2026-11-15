"""🛰️ Runs readiness round 5 on g1-m02: the native bootstrap up to ten times (again 2 minutes after a fleet refusal, 3 minutes after a hard stop), samples the three largest processes with parent pid every 30 s, traces the parents of a large git process, and sends one SIGINT on a hard-stop condition of ROUND.md."""
import glob, os, re, signal, subprocess, sys, time, datetime
home = os.path.expanduser("~")
out = os.path.dirname(os.path.abspath(__file__))
w = os.path.dirname(os.path.dirname(out))
service = "/var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501"
samples = open(f"{out}/round-5.samples.txt", "a")
def now(): return datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ")
def note(line): samples.write(f"{now()} {line}\n"); samples.flush(); print(now(), line, flush=True)
def sh(command): return subprocess.run(command, shell=True, capture_output=True, text=True, errors="replace").stdout
def group_rss(pgid):
    rows = [line.split() for line in sh("ps -axo pgid,rss").splitlines()[1:]]
    return max([int(r[1]) for r in rows if r[0] == str(pgid)] or [0])
def chain(pid):
    lines = []
    while pid and pid != "0" and len(lines) < 12:
        row = sh(f"ps -o pid=,ppid=,args= -p {pid}").strip().replace("\n", " ")[:300]
        if not row: break
        lines.append(row); pid = row.split()[1]
    return "\n".join(lines)
traced = set()
for k in range(int(sys.argv[1]) if len(sys.argv) > 1 else 1, 11):
    log = f"{home}/native-bootstrap-round-5.log" if k == 1 else f"{home}/native-bootstrap-round-5-try-{k}.log"
    child = subprocess.Popen(["bash", f"{w}/native-bootstrap.sh", log], start_new_session=True, preexec_fn=lambda: signal.signal(signal.SIGINT, signal.SIG_DFL))
    sizes = {p: os.path.getsize(p) for p in glob.glob(f"{service}/host-*.log")}
    note(f"TRY {k} START group {child.pid} log {log}")
    peak, run_peak, stopped, twos = 0, 0, None, 0
    while child.poll() is None:
        top = "\n".join(line.replace("\\012", " ")[:700] for line in sh("ps -axo rss,pid,ppid,args | sort -nr | head -3").splitlines()) + "\n"
        level = sh("sysctl -n kern.memorystatus_vm_pressure_level").strip()
        rss = int(top.split()[0]) if top.split() else 0
        mine = group_rss(child.pid)
        peak, run_peak = max(peak, rss), max(run_peak, mine)
        twos = twos + 1 if level == "2" else 0
        samples.write(f"{now()} pressure_level={level} run_group_max_rss_kib={mine}\n{top}"); samples.flush()
        for row in sh("ps -axo rss,pid,ppid,comm").splitlines()[1:]:
            f = row.split(None, 3)
            if len(f) == 4 and os.path.basename(f[3]).startswith("git") and int(f[0]) > 500 * 1024 and f[1] not in traced:
                traced.add(f[1]); note(f"LARGE GIT {f[0]} KiB pid {f[1]}; parent chain:\n{chain(f[1])}")
        reason = None
        if mine > 8 * 1024 * 1024: reason = f"process of the run above 8 GB resident ({mine} KiB)"
        if level == "4" or twos >= 5: reason = f"memory pressure level {level} ({twos} samples of 2 in a row)"
        if reason and not stopped:
            stopped = reason
            note(f"HARD STOP: {reason}; one SIGINT to group {child.pid}")
            os.killpg(child.pid, signal.SIGINT)
        time.sleep(30)
    text = open(log, errors="replace").read()
    rc = re.findall(r"native_rc=(\d+)", text)
    reference = re.findall(r"unexpected failure \(reference ([0-9a-f]+)\)", text)
    refusal = bool(rc and rc[-1] == "75") or bool(re.search(r"graph-wanted|for the fleet|the fleet was asked", text)) or bool(reference and sh(f"grep -h 'request-failed {reference[-1]} .*graph-wanted' {service}/host-*.log"))
    note(f"TRY {k} END native_rc={rc[-1] if rc else '?'} peak_rss_kib={peak} run_peak_rss_kib={run_peak} hard_stop={stopped} fleet_refusal={refusal}")
    with open(f"{out}/round-5.service-try-{k}.txt", "w") as part:
        for p in sorted(glob.glob(f"{service}/host-*.log"), key=os.path.getmtime):
            if os.path.getsize(p) > sizes.get(p, 0):
                with open(p, errors="replace") as source: source.seek(sizes.get(p, 0)); part.write(f"=== {p} from byte {sizes.get(p, 0)}\n" + source.read())
    tasks = "> nx run" in text
    if rc and rc[-1] == "0": break
    if k == 10: break
    if stopped: note("hard stop: waiting 3 minutes before the next try"); time.sleep(180)
    elif refusal: note("fleet refusal: waiting 2 minutes before the next try"); time.sleep(120)
    elif tasks: note("tasks ran and the run failed: round ends"); break
    else: note("ended without a task and without a refusal (code changed under it?): waiting 2 minutes before the next try"); time.sleep(120)
note("ROUND RUNNER END")
