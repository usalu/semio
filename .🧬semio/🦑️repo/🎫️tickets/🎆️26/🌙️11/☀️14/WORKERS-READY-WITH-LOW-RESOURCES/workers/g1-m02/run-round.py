"""🛰️ Runs one readiness round on g1-m02: the native bootstrap once, samples resident sizes every 30 s and sends one SIGINT on a hard-stop condition of ROUND.md."""
import os, re, signal, subprocess, sys, time, datetime
n = sys.argv[1]
home = os.path.expanduser("~")
w = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
out = os.path.dirname(os.path.abspath(__file__))
log = f"{home}/native-bootstrap-round-{n}.log"
samples = open(f"{out}/round-{n}.samples.txt", "w")
child = subprocess.Popen(["bash", f"{w}/native-bootstrap.sh", log], start_new_session=True, preexec_fn=lambda: signal.signal(signal.SIGINT, signal.SIG_DFL))
peak, stopped = 0, None
def now(): return datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ")
while child.poll() is None:
    top = subprocess.run("ps -axo rss,command | sort -nr | head -3 | cut -c1-160", shell=True, capture_output=True, text=True, errors="replace").stdout
    pressure = subprocess.run("sysctl -n kern.memorystatus_vm_pressure_level; memory_pressure | tail -1", shell=True, capture_output=True, text=True).stdout.split("\n")
    rss = int(top.split()[0]) if top.split() else 0
    peak = max(peak, rss)
    samples.write(f"{now()} pressure_level={pressure[0]} {pressure[1] if len(pressure) > 1 else ''}\n{top}"); samples.flush()
    reason = None
    if rss > 8 * 1024 * 1024: reason = f"process above 8 GB resident ({rss} KiB)"
    if pressure[0].strip() not in ("1", ""): reason = f"memory pressure level {pressure[0]}"
    try:
        text = open(log, errors="replace").read()
        for m in re.finditer(r"graph: refreshing (\d+) projects|Inferring projects: .*? of (\d+) projects", text):
            if int(m.group(1) or m.group(2)) > 300: reason = f"hard stop line: {m.group(0)}"
    except FileNotFoundError: pass
    if reason and not stopped:
        stopped = reason
        samples.write(f"{now()} HARD STOP: {reason}; one SIGINT to group {child.pid}\n"); samples.flush()
        os.killpg(child.pid, signal.SIGINT)
    time.sleep(30)
samples.write(f"{now()} END child_rc={child.returncode} peak_rss_kib={peak} hard_stop={stopped}\n"); samples.close()
print(f"END child_rc={child.returncode} peak_rss_kib={peak} hard_stop={stopped}")
