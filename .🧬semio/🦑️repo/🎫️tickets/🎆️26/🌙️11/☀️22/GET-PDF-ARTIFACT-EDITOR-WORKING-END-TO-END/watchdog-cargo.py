import os, signal, subprocess, sys, time
limit = int(os.environ.get("WATCHDOG_TEST_SECONDS", "600"))
proc = subprocess.Popen(sys.argv[1:], start_new_session=True)
def tree(root):
    rows = subprocess.run(["ps", "-A", "-o", "pid=,ppid=,etime=,command="], capture_output=True, text=True).stdout.splitlines()
    table = {}
    for row in rows:
        parts = row.split(None, 3)
        if len(parts) == 4:
            table[int(parts[0])] = (int(parts[1]), parts[2], parts[3])
    found, frontier = {}, [root]
    while frontier:
        parent = frontier.pop()
        for pid, (ppid, etime, command) in table.items():
            if ppid == parent and pid not in found:
                found[pid] = (etime, command)
                frontier.append(pid)
    return found
def seconds(etime):
    days, _, clock = etime.partition("-") if "-" in etime else ("0", "", etime)
    fields = [int(x) for x in clock.split(":")]
    while len(fields) < 3:
        fields.insert(0, 0)
    return int(days) * 86400 + fields[0] * 3600 + fields[1] * 60 + fields[2]
while proc.poll() is None:
    time.sleep(15)
    for pid, (etime, command) in tree(proc.pid).items():
        if "/out/" in command.split(" ")[0] and seconds(etime) > limit:
            print(f"[watchdog] killing test binary {pid} after {etime}", flush=True)
            os.kill(pid, signal.SIGKILL)
sys.exit(proc.returncode)
