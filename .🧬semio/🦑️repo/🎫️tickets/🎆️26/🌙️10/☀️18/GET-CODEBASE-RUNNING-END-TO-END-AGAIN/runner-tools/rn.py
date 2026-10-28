"""🏃 Lane runner driver: one official product run at a time through `gate-g-tools/product.py` or `prodmap-tools/group.py`.

`rn.py start P <label> <target>` | `rn.py start G <label> <projects> <targets>`: waits while `repo leases status` shows ≥ 4 WAITING rows, then starts the measured run detached (it survives a tool timeout) and waits like `wait`.
`rn.py wait [seconds]`: waits up to `seconds` (default 540) for the active run and prints its record or the log tail.
`rn.py table`: prints one Markdown row per `rn-*` record.
"""
import glob, json, os, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
ROOT = os.path.abspath(os.path.join(C, *([".."] * 7)))
GEN = os.path.join(C, "🗑️generated", "runner")
STATE = os.path.join(GEN, "active.json")
RECORDS = {"P": os.path.join(C, "gate-g-tools", "run"), "G": os.path.join(C, "🗑️generated", "prodmap-run")}
TOOLS = {"P": os.path.join(C, "gate-g-tools", "product.py"), "G": os.path.join(C, "prodmap-tools", "group.py")}
DROP = ("CLAUDE", "ANTHROPIC", "__CF", "XPC_", "TERM_SESSION_ID", "SEMIO_SCRIPT_STALL_MS")


def waiting():
    out = subprocess.run(["bun", "repo", "leases", "status"], cwd=ROOT, capture_output=True, text=True).stdout
    return sum(1 for line in out.splitlines() if "WAITING" in line)


def alive(pid):
    try:
        os.kill(pid, 0)
    except OSError:
        return False
    return subprocess.run(["ps", "-p", str(pid), "-o", "stat="], capture_output=True, text=True).stdout.strip()[:1] not in ("", "Z")


def start(kind, label, args):
    assert label.startswith("rn-"), "label must start with rn-"
    if os.path.exists(STATE) and alive(json.load(open(STATE))["pid"]):
        sys.exit("[runner] a run is still active: " + open(STATE).read())
    while (n := waiting()) >= 4:
        print("[runner] %s WAITING leases, holding" % n, flush=True)
        time.sleep(30)
    os.makedirs(GEN, exist_ok=True)
    env = {k: v for k, v in os.environ.items() if not k.startswith(DROP)}
    out = open(os.path.join(GEN, label + ".driver.log"), "ab", buffering=0)
    pid = subprocess.Popen([sys.executable, "-I", TOOLS[kind], label, *args], cwd=ROOT, env=env, stdout=out, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, start_new_session=True).pid
    json.dump({"pid": pid, "kind": kind, "label": label, "args": args, "started": time.strftime("%H:%M:%S")}, open(STATE, "w"))
    print("[runner] started", label, pid, flush=True)
    wait(int(os.environ.get("RN_WAIT", "540")))


def tail(path, n=25):
    if not os.path.exists(path):
        return ""
    with open(path, "rb") as stream:
        stream.seek(0, 2)
        stream.seek(max(0, stream.tell() - 6000))
        return "\n".join(stream.read().decode("utf8", "replace").splitlines()[-n:])


def wait(limit):
    state = json.load(open(STATE))
    base = os.path.join(RECORDS[state["kind"]], state["label"])
    t0 = time.monotonic()
    while alive(state["pid"]) and time.monotonic() - t0 < limit:
        time.sleep(10)
    if alive(state["pid"]):
        print("[runner] still running", state["label"], "since", state["started"], "\n" + tail(base + ".log", 6))
        return
    print(tail(base + ".log"))
    print(open(base + ".json").read() if os.path.exists(base + ".json") else "[runner] no record")


def table():
    rows = []
    for kind, folder in RECORDS.items():
        for path in glob.glob(os.path.join(folder, "rn-*.json")):
            r = json.load(open(path))
            rows.append((r["started"], "| %s | `%s` | %s | %s s | %s MB | %s%s |" % (r["label"], r["target"], r["started"], r["wall_s"], r["peak_tree_mb"], r["exit"], " killed " + r["killed"] if r.get("killed") else "")))
    print("\n".join(row for _, row in sorted(rows)))


if __name__ == "__main__":
    verb = sys.argv[1]
    if verb == "start":
        start(sys.argv[2], sys.argv[3], sys.argv[4:])
    elif verb == "wait":
        wait(int(sys.argv[2]) if len(sys.argv) > 2 else 540)
    else:
        table()
