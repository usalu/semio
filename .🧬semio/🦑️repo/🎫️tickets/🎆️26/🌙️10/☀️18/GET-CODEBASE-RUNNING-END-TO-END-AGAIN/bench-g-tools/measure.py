"""📏 Interim product build benchmark harness (bench-g): one measured `repo` command and the baseline scenarios.

`measure.py run   --root R --id I -- <cmd...>`           one command: wall, per-phase times, tree RSS peak, actions, graph re-inference.
`measure.py noop  --product P --root R --tool T`          prime run, then N warm no-op runs.
`measure.py edit  --product P --root R --tool T`          N times: one-line edit of the product source, run, restore.
`measure.py peer  --product P --root R --tool T --peer X` N times: edit an unrelated file, run the product (0 re-executions wanted).

The phase rules, graph/action patterns and products live in `phases.json` / `products.json` (language-agnostic data); the
production catalogue replaces this script with `repo bench` (see result-bench-g.md). Edits happen only below a root that
is separate from the live checkout; the original bytes are restored whatever happens.
"""
import argparse, json, os, re, signal, subprocess, sys, threading, time
from datetime import datetime, timezone

HERE = os.path.dirname(os.path.abspath(__file__))
PHASES = json.load(open(os.path.join(HERE, "phases.json"), encoding="utf8"))
PRODUCTS = json.load(open(os.path.join(HERE, "products.json"), encoding="utf8"))
TEMPLATES = {
    ("ts", "private-body"): lambda n: "void %d;\n" % n,
    ("rs", "private-body"): lambda n: "const _: u32 = %d;\n" % n,
    ("ts", "comment"): lambda n: "// [incremental] %d\n" % n,
    ("rs", "comment"): lambda n: "// [incremental] %d\n" % n,
}


def row_key(line):
    """Turns a structured `[script] {json}` progress row into `script:<command>:<stage>:<resource>`; other lines stay as they are."""
    if line.startswith("[script] {"):
        try:
            row = json.loads(line[9:])
            return "script:%s:%s:%s" % (row.get("command", ""), row.get("stage", ""), (row.get("activity") or {}).get("resource", ""))
        except ValueError:
            return line
    return line


RULES = [(p["id"], [re.compile(x) for x in p["patterns"]]) for p in PHASES["phases"]]


def classify(key):
    for ident, patterns in RULES:
        if any(p.search(key) for p in patterns):
            return ident
    return None


class Stream:
    """Streaming analysis of the output lines: phase times, actions and graph re-inference without keeping a line."""

    def __init__(self):
        a, g = PHASES["actions"], PHASES["graph"]
        self.current, self.since, self.totals = PHASES["initial"], 0.0, {}
        self.proc, self.total_re = re.compile(a["bazelProcesses"]), re.compile(a["bazelTotal"])
        self.nx_task, self.nx_cached = re.compile(a["nxTask"]), re.compile(a["nxCached"])
        self.g_ref, self.g_inf, self.g_pub = re.compile(g["refreshing"]), re.compile(g["inferring"]), re.compile(g["published"])
        self.bazel, self.bazel_total, self.nx_total, self.nx_hit = None, None, 0, 0
        self.graph = {"refreshedProjects": 0, "inferredProjects": 0, "generation": None, "publishedSeconds": None}
        self.capacity_waits = 0
        self.lines = 0

    def feed(self, at, line):
        self.lines += 1
        key = row_key(line)
        phase = classify(key)
        if phase is not None and phase != self.current:
            self.totals[self.current] = self.totals.get(self.current, 0.0) + (at - self.since)
            self.current, self.since = phase, at
        if key.startswith("[capacity] waiting"):
            self.capacity_waits += 1
        if len(line) > 4096:
            return
        m = self.proc.search(line)
        if m:
            executed = cached = internal = 0
            for part in m.group(2).split(", "):
                pm = re.match(r"(\d+) (.+)", part.strip())
                if not pm:
                    continue
                n, kind = int(pm.group(1)), pm.group(2)
                if kind == "internal":
                    internal += n
                elif kind.endswith("cache hit"):
                    cached += n
                else:
                    executed += n
            self.bazel = {"processes": int(m.group(1)), "executed": executed, "cached": cached, "internal": internal, "line": line.strip()}
        m = self.total_re.search(line)
        if m:
            self.bazel_total = int(m.group(1))
        if self.nx_task.search(line):
            self.nx_total += 1
            if self.nx_cached.search(line):
                self.nx_hit += 1
        m = self.g_ref.search(line)
        if m:
            self.graph["refreshedProjects"] = max(self.graph["refreshedProjects"], int(m.group(1)))
        m = self.g_inf.search(line)
        if m:
            self.graph["inferredProjects"] = max(self.graph["inferredProjects"], int(m.group(1)))
        m = self.g_pub.search(line)
        if m:
            self.graph["generation"], self.graph["publishedSeconds"] = int(m.group(1)), int(m.group(3))

    def phases(self, wall_ms):
        totals = dict(self.totals)
        totals[self.current] = totals.get(self.current, 0.0) + (wall_ms - self.since)
        return {k: round(v, 1) for k, v in totals.items() if v > 0.5}

    def actions(self):
        bazel = dict(self.bazel) if self.bazel else None
        if bazel and self.bazel_total is not None:
            bazel["totalActions"] = self.bazel_total
        nx = {"tasks": self.nx_total, "cached": self.nx_hit, "executed": self.nx_total - self.nx_hit} if self.nx_total else None
        return {"bazel": bazel, "nx": nx}


def own_rss_bytes():
    import resource
    value = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    return value if sys.platform == "darwin" else value * 1024


LIMIT_BYTES = 500 * 1024 * 1024
LOG_LIMIT = 64 * 1024 * 1024


def process_table():
    if os.name == "nt":
        cmd = ["powershell", "-NoProfile", "-NonInteractive", "-Command", 'Get-CimInstance Win32_Process | ForEach-Object { "$($_.ProcessId),$($_.ParentProcessId),$($_.WorkingSetSize)" }']
    else:
        cmd = ["ps", "-A", "-o", "pid=,ppid=,rss="]
    try:
        text = subprocess.run(cmd, capture_output=True, text=True, timeout=20).stdout
    except Exception:
        return {}
    rows = {}
    for line in text.splitlines():
        cells = re.split(r"[\s,]+", line.strip().replace('"', ""))
        if len(cells) >= 3 and all(c.lstrip("-").isdigit() for c in cells[:3]):
            rows[int(cells[0])] = (int(cells[1]), int(cells[2]) * (1 if os.name == "nt" else 1024))
    return rows


def tree_rss(rows, root):
    if root not in rows:
        return None
    members, grown = {root}, True
    while grown:
        grown = False
        for pid, (parent, _) in rows.items():
            if pid not in members and parent in members:
                members.add(pid)
                grown = True
    return sum(rows[p][1] for p in members)


def swap_used_mb():
    try:
        text = subprocess.run(["sysctl", "-n", "vm.swapusage"], capture_output=True, text=True, timeout=5).stdout
        m = re.search(r"used = ([\d.]+)M", text)
        return float(m.group(1)) if m else None
    except Exception:
        return None


def load1():
    try:
        return os.getloadavg()[0]
    except Exception:
        return None


def environment(extra=None):
    kept = {k: v for k, v in os.environ.items() if k == "SEMIO_LANGUAGE" or not re.match(r"^(SEMIO_|NX_|CLAUDE|ANTHROPIC)", k)}
    kept.setdefault("SEMIO_LANGUAGE", "en")
    kept["BUN_RUNTIME_TRANSPILER_CACHE_PATH"] = "0"
    kept.update(extra or {})
    return kept


def run(cmd, cwd, ident, outdir, interval=0.25, extra_env=None):
    """Runs one command in its own session and returns its measurement record; the full log is kept in `outdir/<ident>.log`."""
    os.makedirs(outdir, exist_ok=True)
    log_path = os.path.join(outdir, ident + ".log")
    started_at = datetime.now(timezone.utc).isoformat()
    swap0, load0, t0 = swap_used_mb(), load1(), time.monotonic()
    child = subprocess.Popen(cmd, cwd=cwd, env=environment(extra_env), stdout=subprocess.PIPE, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, start_new_session=(os.name != "nt"), bufsize=0)
    stream, first_output, marker = Stream(), [None], [None]
    peak = [0, 0.0]
    stop = threading.Event()
    swap_max = [swap0 or 0.0]

    def reader():
        pending = b""
        with open(log_path, "wb") as log:
            while True:
                chunk = child.stdout.read1(65536) if hasattr(child.stdout, "read1") else child.stdout.read(65536)
                if not chunk:
                    break
                if log.tell() < LOG_LIMIT:
                    log.write(chunk)
                at = (time.monotonic() - t0) * 1000
                if first_output[0] is None:
                    first_output[0] = at
                pending += chunk
                if len(pending) > 1048576 and b"\n" not in pending:
                    pending = b""
                while b"\n" in pending:
                    raw, pending = pending.split(b"\n", 1)
                    line = raw.decode("utf8", "replace").rstrip("\r")
                    stream.feed(at, line)
                    if marker[0] is None and re.search(r"^(\[\d+ / \d+\]|> nx run )", line):
                        marker[0] = at

    def sampler():
        n = 0
        while not stop.is_set():
            value = tree_rss(process_table(), child.pid)
            if value is not None and value > peak[0]:
                peak[0], peak[1] = value, (time.monotonic() - t0) * 1000
            n += 1
            if own_rss_bytes() > LIMIT_BYTES:
                print("measure.py: own RSS above 500 MB, aborting", file=sys.stderr)
                os._exit(3)
            if n % 20 == 0:
                s = swap_used_mb()
                if s is not None:
                    swap_max[0] = max(swap_max[0], s)
            stop.wait(interval)

    threads = [threading.Thread(target=reader, daemon=True), threading.Thread(target=sampler, daemon=True)]
    for t in threads:
        t.start()
    try:
        code = child.wait()
    except KeyboardInterrupt:
        os.killpg(child.pid, signal.SIGTERM)
        code = child.wait()
    threads[0].join(10)
    stop.set()
    wall = (time.monotonic() - t0) * 1000
    swap1 = swap_used_mb()
    record = {
        "id": ident, "startedAt": started_at, "cmd": cmd, "cwd": cwd, "exit": code, "wallMs": round(wall, 1),
        "firstOutputMs": None if first_output[0] is None else round(first_output[0], 1),
        "executeStartMs": None if marker[0] is None else round(marker[0], 1),
        "peakRssBytes": peak[0] or None, "peakRssAtMs": round(peak[1], 1),
        "swapUsedMbStart": swap0, "swapUsedMbEnd": swap1, "swapUsedMbMax": swap_max[0],
        "load1Start": load0, "load1End": load1(),
        "phases": stream.phases(wall), "actions": stream.actions(), "graph": stream.graph,
        "capacityWaits": stream.capacity_waits,
        "log": log_path, "lines": stream.lines,
    }
    with open(os.path.join(outdir, ident + ".json"), "w", encoding="utf8") as handle:
        json.dump(record, handle, indent=1, ensure_ascii=False)
    return record


def refuse_live(root):
    """Edits only below a disposable root, except inside a coordinator-declared measurement window (BENCH_WINDOW=1)."""
    live = os.path.realpath(os.environ.get("BENCH_LIVE", "/Users/ueli/Documents/semio"))
    here = os.path.realpath(root)
    if (here == live or here.startswith(live + os.sep) or live.startswith(here + os.sep)) and os.environ.get("BENCH_WINDOW") != "1":
        raise SystemExit("refusing to edit inside the live checkout: " + root)
    until = os.environ.get("BENCH_WINDOW_UNTIL")
    if os.environ.get("BENCH_WINDOW") == "1" and until and time.strftime("%H%M") >= until:
        raise SystemExit("the measurement window ended at %s: no edit on the live checkout" % until)


def repo_command(root, tool, args):
    base = ["bun", "repo"] + (["--tool=" + tool] if tool else []) + args
    return (["nice", "-n", "10"] if os.name != "nt" else []) + base


def waiters(root):
    try:
        out = subprocess.run(["bun", "repo", "leases", "status", "--json"], cwd=root, capture_output=True, text=True, timeout=180, env=environment()).stdout
        return len(json.loads(out[out.index("{"):]).get("waiters", []))
    except Exception:
        return 0


def gate(root, limit=4, polls=90):
    for _ in range(polls):
        if waiters(root) < limit:
            return
        time.sleep(60)


def product_of(name):
    return next(p for p in PRODUCTS["products"] if p["id"] == name)


def scenario(args):
    outdir = args.out
    product = product_of(args.product)
    base = "%s.%s.%s" % (args.product, args.tool, args.kind)
    records = []

    def go(label, extra=None):
        if not args.nogate:
            gate(args.root)
        ident = "%s.%s" % (base, label)
        attempts = 0
        while True:
            attempts += 1
            record = run(repo_command(args.root, args.tool, ["run", product["target"]]), args.root, ident, outdir)
            churn = record["exit"] != 0 and record["wallMs"] < 180000 and "execute" not in record["phases"]
            if not churn or attempts >= args.retries:
                break
            print(json.dumps({"id": ident, "churn": True, "exit": record["exit"], "attempt": attempts, "wallMs": record["wallMs"]}), flush=True)
            time.sleep(150)
        record["attempts"] = attempts
        record.update({"scenario": args.kind, "product": args.product, "tool": args.tool, "label": label})
        record.update(extra or {})
        json.dump(record, open(os.path.join(outdir, ident + ".json"), "w", encoding="utf8"), indent=1, ensure_ascii=False)
        records.append(record)
        print(json.dumps({k: record[k] for k in ("id", "exit", "wallMs", "peakRssBytes", "phases", "actions", "graph", "attempts")}, ensure_ascii=False), flush=True)
        return record

    if args.kind in ("edit", "peer"):
        refuse_live(args.root)
    if args.prime:
        go("prime")
    for i in range(args.samples):
        if args.kind == "noop":
            go("s%d" % (i + 1))
            continue
        if args.kind == "edit":
            rel, language, op = product["editFile"], product["language"], "private-body"
        else:
            peer = next(p for p in PRODUCTS["peers"] if p["id"] == args.peer)
            rel, language, op = peer["file"], peer["language"], peer["op"]
        path = os.path.join(args.root, rel)
        original = open(path, "rb").read()
        text = original.decode("utf8")
        nonce = int(time.time()) % 1000000007 + i
        edited = (text if text.endswith("\n") or text == "" else text + "\n") + TEMPLATES[(language, op)](nonce)
        os.makedirs(os.path.join(outdir, "backup"), exist_ok=True)
        open(os.path.join(outdir, "backup", rel.replace(os.sep, "_")), "wb").write(original)
        signal.signal(signal.SIGTERM, lambda *_: (_ for _ in ()).throw(KeyboardInterrupt()))
        try:
            open(path, "wb").write(edited.encode("utf8"))
            go("s%d" % (i + 1), {"edit": rel})
        finally:
            open(path, "wb").write(original)
        if args.revert:
            go("r%d" % (i + 1), {"edit": rel, "reverted": True})
    json.dump(records, open(os.path.join(outdir, base + ".all.json"), "w", encoding="utf8"), indent=1, ensure_ascii=False)


def concurrent(args):
    """Runs the product builds at once (one per product, no edits) and records every tree plus the swap of the machine."""
    os.makedirs(args.out, exist_ok=True)
    records, threads = {}, []

    def one(name):
        product = product_of(name)
        records[name] = run(repo_command(args.root, args.tool, ["run", product["target"]]), args.root, "concurrent.%s.%s" % (args.tool, name), args.out)

    t0 = time.monotonic()
    swap0 = swap_used_mb()
    for name in args.products.split(","):
        thread = threading.Thread(target=one, args=(name,))
        thread.start()
        threads.append(thread)
    for thread in threads:
        thread.join()
    summary = {"makespanMs": round((time.monotonic() - t0) * 1000, 1), "swapStartMb": swap0, "swapEndMb": swap_used_mb(), "runs": {k: {x: v[x] for x in ("exit", "wallMs", "peakRssBytes", "phases", "actions", "swapUsedMbMax")} for k, v in records.items()}, "treeRssSumBytes": sum((v["peakRssBytes"] or 0) for v in records.values())}
    json.dump(summary, open(os.path.join(args.out, "concurrent.%s.json" % args.tool), "w"), indent=1)
    print(json.dumps(summary), flush=True)


def main():
    p = argparse.ArgumentParser()
    sub = p.add_subparsers(dest="mode", required=True)
    r = sub.add_parser("run")
    r.add_argument("--root", required=True)
    r.add_argument("--id", required=True)
    r.add_argument("--out", required=True)
    r.add_argument("cmd", nargs=argparse.REMAINDER)
    for kind in ("noop", "edit", "peer"):
        s = sub.add_parser(kind)
        s.add_argument("--product", required=True)
        s.add_argument("--root", required=True)
        s.add_argument("--tool", default="bazel")
        s.add_argument("--out", required=True)
        s.add_argument("--samples", type=int, default=3)
        s.add_argument("--prime", action="store_true")
        s.add_argument("--revert", action="store_true")
        s.add_argument("--nogate", action="store_true")
        s.add_argument("--retries", type=int, default=6)
        s.add_argument("--peer", default="unrelated-ts")
        s.set_defaults(kind=kind)
    c = sub.add_parser("concurrent")
    c.add_argument("--root", required=True)
    c.add_argument("--tool", default="bazel")
    c.add_argument("--out", required=True)
    c.add_argument("--products", default="dashboard,course,print")
    args = p.parse_args()
    if args.mode == "concurrent":
        concurrent(args)
    elif args.mode == "run":
        cmd = args.cmd[1:] if args.cmd and args.cmd[0] == "--" else args.cmd
        print(json.dumps(run(cmd, args.root, args.id, args.out), ensure_ascii=False, indent=1))
    else:
        scenario(args)


main()
