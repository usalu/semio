import os, re, subprocess, sys, time
C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
name = sys.argv[1]
root = int(re.search(r"pid=(\d+)", open(f"{C}/🗑️generated/default-d/{name}.pid").read()).group(1))
def tree(p):
    out = subprocess.run(["ps", "-axo", "pid=,ppid="], capture_output=True, text=True).stdout.split("\n")
    kids = {}
    for l in out:
        t = l.split()
        if len(t) == 2: kids.setdefault(int(t[1]), []).append(int(t[0]))
    order = []
    def walk(x):
        for k in kids.get(x, []): walk(k)
        order.append(x)
    walk(p)
    return order
for sig in ("-TERM", "-KILL"):
    for p in tree(root):
        subprocess.run(["kill", sig, str(p)], stderr=subprocess.DEVNULL)
    time.sleep(3)
print("alive:", [p for p in tree(root) if subprocess.run(["kill", "-0", str(p)], stderr=subprocess.DEVNULL).returncode == 0])
