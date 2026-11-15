import subprocess, time, datetime
out = open("/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/🧰️scripts/1113-measure/samples.csv", "w")
out.write("iso,t_s,daemon_pid,daemon_cpu,daemon_rss_kib,supervisor_pid,supervisor_cpu,supervisor_rss_kib\n")
t0 = time.time()
for i in range(31):
    p = subprocess.run(["ps", "-o", "pid=,%cpu=,rss=", "-p", "99888,99887"], capture_output=True, text=True)
    rows = {}
    for line in p.stdout.strip().splitlines():
        pid, cpu, rss = line.split()
        rows[int(pid)] = (cpu, rss)
    d = rows.get(99888, ("NA", "NA"))
    s = rows.get(99887, ("NA", "NA"))
    out.write(f"{datetime.datetime.now().isoformat(timespec='seconds')},{time.time()-t0:.1f},99888,{d[0]},{d[1]},99887,{s[0]},{s[1]}\n")
    out.flush()
    if i < 30:
        time.sleep(10)
out.close()
print("done", round(time.time()-t0, 1))
