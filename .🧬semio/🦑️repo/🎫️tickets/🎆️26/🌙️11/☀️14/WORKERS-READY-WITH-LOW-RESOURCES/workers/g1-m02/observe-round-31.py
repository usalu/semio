"""👁️ Round 31: every 10 minutes writes `round-31.md` (ready state files of this machine, last 30 lines of the running stage log, memory pressure, five largest processes); never starts any setup; stops when `state.json` has `readyMs` or after 6 hours."""
import datetime, json, os, re, subprocess, time
home = os.path.expanduser("~")
ready = f"{home}/Documents/semio/.cache/f/ready"
out = os.path.dirname(os.path.abspath(__file__)) + "/round-31.md"
def sh(c): return subprocess.run(c, shell=True, capture_output=True, text=True, errors="replace").stdout
def rd(n):
    try: return open(f"{ready}/{n}", errors="replace").read()
    except OSError: return "(absent)"
end = time.time() + 6 * 3600
while time.time() < end:
    stamp = datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ")
    try: running = json.loads(rd("running.json"))
    except ValueError: running = {}
    stage = running.get("stage", "")
    log = "(no stage running)"
    if stage:
        lines = [l[:300] for l in rd(f"{stage}.log").splitlines() if not l.startswith("[script] {")]
        log = "\n".join(lines[-30:])
    top = sh("ps -axo pid,rss,etime,command | sort -k2 -nr | head -5 | cut -c1-260")
    text = f"# Round 31 Observation of g1-m02\n\nWritten {stamp}; every 10 minutes. No setup command is started by this session.\n\n## Ready State (`.cache/f/ready`)\n\n```\nstate.json: {rd('state.json')}\nrunning.json: {rd('running.json')}\nlease.json: {rd('lease.json')}\nready-timings.json: {rd('ready-timings.json')}\n```\n\n## Last 30 Lines of the `{stage or 'none'}` Log\n\n```\n{log}\n```\n\n## Memory\n\n`memory_pressure | tail -1`: {sh('memory_pressure | tail -1').strip()}\n\nFive largest processes by resident size (KiB):\n\n```\n{top}\n```\n\nPermission dialog on screen: not measured (this session cannot see the screen).\n"
    open(out, "w").write(text)
    try: done = json.loads(rd("state.json")).get("readyMs")
    except ValueError: done = None
    if done: print("READY", stamp, flush=True); break
    time.sleep(600)
