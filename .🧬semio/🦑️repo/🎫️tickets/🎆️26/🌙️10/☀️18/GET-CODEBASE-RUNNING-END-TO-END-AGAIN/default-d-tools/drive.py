"""drive.py <name> <repo args...>: runs `nice -n 10 bun repo <args>` in its own session, logs to default-d/<name>.raw,
and when the Nx run summary ('Run duration:') has been printed and the loader does not exit within 20 s (fault N),
kills the whole tree (children first). Appends rc= (rc=nx-ok-loader-reaped when reaped after a green summary)."""
import os, subprocess, sys, time
C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
name, args = sys.argv[1], sys.argv[2:]
FLAKES = ("First failure: Sole exact committed recomputable image allocation required", "Funded Nx graph cache capacity exhausted", "Original private Nx dependency implementation differs", "Actual recovered committed child required", "native/image.json", "Exact original descendant creator identity required", "Original recovered creator identities required")

log = f"{C}/🗑️generated/default-d/{name}.raw"

def run_once():
    os.chdir("/Users/ueli/Documents/semio")
    open(log, "wb").close()
    fh = open(log, "ab", buffering=0)
    p = subprocess.Popen(["nice", "-n", "10", "bun", "repo"] + args, stdout=fh, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, start_new_session=True)
    open(f"{C}/🗑️generated/default-d/{name}.pid", "w").write(f"pid={p.pid} {time.ctime()}")
    def tree(root):
        out = subprocess.run(["ps", "-axo", "pid=,ppid="], capture_output=True, text=True).stdout.split("\n")
        kids = {}
        for l in out:
            t = l.split()
            if len(t) == 2: kids.setdefault(int(t[1]), []).append(int(t[0]))
        order = []
        def walk(x):
            for k in kids.get(x, []): walk(k)
            order.append(x)
        walk(root)
        return order
    def read(): return open(log, "rb").read().decode("utf-8", "replace")
    seen = None
    while p.poll() is None:
        time.sleep(2)
        txt = read()
        if seen is None and ("Run duration:" in txt or "Nx exit code" in txt or "Pass --verbose to see the stacktrace" in txt):
            seen = time.time()
        if seen and time.time() - seen > 20: break
    rc = p.poll()
    if rc is None:
        for sig in ("-TERM", "-KILL"):
            for q in tree(p.pid): subprocess.run(["kill", sig, str(q)], stderr=subprocess.DEVNULL)
            time.sleep(3)
        verdict = "nx-ok-loader-reaped" if "Successfully ran" in read() else "nx-failed-loader-reaped"
        fh.write(f"\nrc={verdict}\n".encode())
    else:
        fh.write(f"\nrc={rc}\n".encode())


for attempt in range(int(os.environ.get("DRIVE_RETRIES","12"))):
    run_once()
    txt = open(log, "rb").read().decode("utf-8", "replace")
    if any(f in txt for f in FLAKES) and "Successfully ran" not in txt and attempt < int(os.environ.get("DRIVE_RETRIES","12")) - 1:
        time.sleep(45)
        os.replace(log, f"{log}.flake")
        continue
    break

import shutil
RUNJSON = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts/run.json"
try: shutil.copy(RUNJSON, f"{log}.run.json")
except OSError: pass
