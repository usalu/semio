"""🛰️ Runs readiness round 2 on g1-m02: the native bootstrap up to six times (again after a fleet refusal, 5 minutes apart), samples resident sizes every 30 s and sends one SIGINT on a hard-stop condition of ROUND.md."""
import os, re, signal, subprocess, sys, time, datetime
home = os.path.expanduser("~")
out = os.path.dirname(os.path.abspath(__file__))
w = os.path.dirname(os.path.dirname(out))
samples = open(f"{out}/round-2.samples.txt", "a")
def now(): return datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ")
def note(line): samples.write(f"{now()} {line}\n"); samples.flush(); print(now(), line, flush=True)
def sh(command): return subprocess.run(command, shell=True, capture_output=True, text=True, errors="replace").stdout
def group_rss(pgid):
    rows = [line.split(None, 2) for line in sh("ps -axo pgid,rss,pid").splitlines()[1:]]
    return max([int(r[1]) for r in rows if r[0] == str(pgid)] or [0])
service = "/var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501"
for k in range(int(sys.argv[1]) if len(sys.argv) > 1 else 1, 7):
    log = f"{home}/native-bootstrap-round-2.log" if k == 1 else f"{home}/native-bootstrap-round-2-try-{k}.log"
    child = subprocess.Popen(["bash", f"{w}/native-bootstrap.sh", log], start_new_session=True, preexec_fn=lambda: signal.signal(signal.SIGINT, signal.SIG_DFL))
    note(f"TRY {k} START group {child.pid} log {log}")
    peak, run_peak, stopped, twos = 0, 0, None, 0
    while child.poll() is None:
        top = sh("ps -axo rss,command | sort -nr | head -3 | cut -c1-160")
        level = sh("sysctl -n kern.memorystatus_vm_pressure_level").strip()
        rss = int(top.split()[0]) if top.split() else 0
        mine = group_rss(child.pid)
        peak, run_peak = max(peak, rss), max(run_peak, mine)
        twos = twos + 1 if level == "2" else 0
        samples.write(f"{now()} pressure_level={level} run_group_max_rss_kib={mine}\n{top}"); samples.flush()
        reason = None
        if mine > 8 * 1024 * 1024: reason = f"process of the run above 8 GB resident ({mine} KiB)"
        if level == "4" or twos >= 3: reason = f"memory pressure level {level} ({twos} samples of 2 in a row)"
        try:
            text = open(log, errors="replace").read()
            m = re.search(r"Inferring projects: .*? of (\d+) projects", text)
            if m and int(m.group(1)) > 300 and "fleet" not in text[:m.start()]: reason = f"hard stop line without a fleet line before it: {m.group(0)}"
        except FileNotFoundError: pass
        if reason and not stopped:
            stopped = reason
            note(f"HARD STOP: {reason}; one SIGINT to group {child.pid}")
            os.killpg(child.pid, signal.SIGINT)
        time.sleep(30)
    text = open(log, errors="replace").read()
    rc = re.findall(r"native_rc=(\d+)", text)
    reference = re.findall(r"unexpected failure \(reference ([0-9a-f]+)\)", text)
    refusal = bool(re.search(r"graph-wanted|wait(ed|ing) for the fleet", text)) or bool(reference and sh(f"grep -h 'request-failed {reference[-1]} .*graph-wanted' {service}/host-*.log"))
    note(f"TRY {k} END native_rc={rc[-1] if rc else '?'} peak_rss_kib={peak} run_peak_rss_kib={run_peak} hard_stop={stopped} fleet_refusal_text={refusal}")
    if stopped or not rc or rc[-1] == "0" or not refusal: break
    if k < 6: note("fleet refusal: waiting 5 minutes before the next try"); time.sleep(300)
note("ROUND RUNNER END")
