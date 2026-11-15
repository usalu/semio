"""🧰 Runs step 0 once more in round 19: `repo tools setup all` from the worktree root into ~/tools-setup-round-20.log, with one SIGINT when the dependency store wait line stands for more than 2 minutes; writes exit code, wall time and the last 20 lines to round-20.tools-setup.md."""
import os, re, signal, subprocess, time, datetime
home = os.path.expanduser("~")
out = os.path.dirname(os.path.abspath(__file__))
log = f"{home}/tools-setup-round-20.log"
def now(): return datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ")
begin, stamp = time.time(), now()
child = subprocess.Popen(["zsh", "-c", f'repo tools setup all > "{log}" 2>&1'], cwd=f"{home}/Documents/semio", start_new_session=True)
first, record, level = None, "", []
while child.poll() is None:
    time.sleep(10)
    level.append(subprocess.run("sysctl -n kern.memorystatus_vm_pressure_level", shell=True, capture_output=True, text=True).stdout.strip())
    try: waits = [l for l in open(log, errors="replace").read().splitlines() if "holds the dependency store" in l]
    except OSError: waits = []
    if waits and first is None: first = time.time()
    if first and time.time() - first > 120 and not record:
        ps = subprocess.run('ps -axo pid,ppid,etime,command | grep -E "bun|repo"', shell=True, capture_output=True, text=True, errors="replace").stdout
        record = f"Dependency store wait for more than 2 minutes; one SIGINT at {now()}. The line:\n\n```\n{waits[-1][:1500]}\n```\n\n`ps`:\n\n```\n" + "\n".join(l[:300] for l in ps.splitlines()) + "\n```\n"
        os.killpg(child.pid, signal.SIGINT)
lines = [l for l in open(log, errors="replace").read().splitlines() if not l.startswith("[script]") and "still running" not in l]
tail = "\n".join(l[:600] for l in lines[-20:])
counts = ", ".join(f"level {v} in {level.count(v)}" for v in sorted(set(level)))
open(f"{out}/round-20.tools-setup.md", "w").write(f"## Step 0 Again in Round 20: `repo tools setup all`\n\n- Start {stamp}, end {now()}, wall time {int(time.time() - begin)} s, exit code {child.returncode}\n- Log `~/tools-setup-round-20.log` ({len(lines)} lines without `[script]` rows and `still running` lines)\n- Memory pressure, samples 10 s apart: {counts}\n\n{record}Last 20 lines (without `[script]` rows and `still running` lines):\n\n```\n{tail}\n```\n")
print("TOOLS SETUP END rc", child.returncode, "wall", int(time.time() - begin))
