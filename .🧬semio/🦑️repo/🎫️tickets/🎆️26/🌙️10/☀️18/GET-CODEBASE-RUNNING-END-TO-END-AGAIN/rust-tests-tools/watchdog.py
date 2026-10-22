import os, re, subprocess, sys, time
limit = int(sys.argv[1]) if len(sys.argv) > 1 else 420
log = sys.argv[2] if len(sys.argv) > 2 else "/dev/stderr"
def table():
    out = subprocess.run(["ps", "-axo", "pid=,ppid=,etime=,command="], capture_output=True, text=True).stdout
    rows = {}
    for line in out.splitlines():
        m = re.match(r"\s*(\d+)\s+(\d+)\s+(\S+)\s+(.*)", line)
        if m: rows[int(m[1])] = (int(m[2]), m[3], m[4])
    return rows
def seconds(e):
    d = 0
    if "-" in e: d, e = e.split("-"); d = int(d)
    p = [int(x) for x in e.split(":")]
    while len(p) < 3: p.insert(0, 0)
    return d * 86400 + p[0] * 3600 + p[1] * 60 + p[2]
def mine(pid, rows):
    while pid in rows:
        ppid, _, cmd = rows[pid]
        if "rust-tests-tools/" in cmd and ("batch.sh" in cmd or "step.sh" in cmd): return True
        pid = ppid
    return False
while True:
    rows = table()
    if not any("rust-tests-tools/" in c and ("batch.sh" in c or "step.sh" in c) for _, _, c in rows.values()): break
    for pid, (ppid, et, cmd) in rows.items():
        if "cache/rust/fixed/" in cmd and "/out/" in cmd and seconds(et) > limit and mine(pid, rows):
            os.kill(pid, 9)
            with open(log, "a") as f: f.write("%s killed pid %d after %ds: %s\n" % (time.strftime("%H:%M:%S"), pid, seconds(et), cmd.split("/out/")[-1][:120]))
    time.sleep(20)
