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


MARK = re.compile(r"cargo-timing|^\[capacity\]|\[repo cargo\]|^graph: |^\[bazel\]|^INFO: |^Analyzing|^Loading|^\[\d+ / \d+\]|^> nx run|Waiting for|lease|admitted|Compiling semio-framework-print|Finished|Fresh|Running|^\[script\] \{.*\"stage\"")
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
        self.capacity_first = self.capacity_last = None
        self.lines = 0
        self.marks = []

    def feed(self, at, line):
        self.lines += 1
        if len(self.marks) < 3000 and MARK.search(line):
            self.marks.append([round(at), line.strip()[:240]])
        key = row_key(line)
        phase = classify(key)
        if phase is not None and phase != self.current:
            self.totals[self.current] = self.totals.get(self.current, 0.0) + (at - self.since)
            self.current, self.since = phase, at
        if key.startswith("[capacity]"):
            self.capacity_first = self.capacity_first or line.strip()[:300]
            self.capacity_last = line.strip()[:300]
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
        cmd = ["ps", "-A", "-o", "pid=,ppid=,rss=,comm="]
    try:
        text = subprocess.run(cmd, capture_output=True, text=True, timeout=20).stdout
    except Exception:
        return {}
    rows = {}
    for line in text.splitlines():
        cells = re.split(r"[\s,]+", line.strip().replace('"', ""))
        if len(cells) >= 3 and all(c.lstrip("-").isdigit() for c in cells[:3]):
            rows[int(cells[0])] = (int(cells[1]), int(cells[2]) * (1 if os.name == "nt" else 1024), " ".join(line.split()[3:]))
    return rows


def tree_rss(rows, root):
    if root not in rows:
        return None
    members, grown = {root}, True
    while grown:
        grown = False
        for pid, (parent, _, _) in rows.items():
            if pid not in members and parent in members:
                members.add(pid)
                grown = True
    return sum(rows[p][1] for p in members)


def bazel_servers_rss(rows):
    """RSS of the resident Bazel JVM servers (daemons outside any run tree; macOS shows them as `bazel(...)`)."""
    return sum(r[1] for r in rows.values() if r[2].startswith("bazel(") or r[2].rstrip().endswith("/bazel"))


UNITS = {"B": 1, "K": 1024, "M": 1048576, "G": 1073741824, "T": 1099511627776}


def top_bytes(text):
    m = re.match(r"^(\d+(?:\.\d+)?)([BKMGT])?[+-]?$", text)
    return int(float(m.group(1)) * UNITS[m.group(2) or "B"]) if m else 0


def footprint_table():
    """macOS `top` MEM and CMPRS per process (the physical footprint incl. compressed memory) in one call; empty elsewhere."""
    if sys.platform != "darwin":
        return {}
    try:
        text = subprocess.run(["top", "-l", "1", "-n", "5000", "-stats", "pid,ppid,mem,cmprs"], capture_output=True, text=True, timeout=30).stdout
    except Exception:
        return {}
    rows, seen = {}, False
    for line in text.splitlines():
        if line.startswith("PID"):
            seen = True
            continue
        cells = line.split()
        if seen and len(cells) >= 4 and cells[0].isdigit() and cells[1].isdigit():
            rows[int(cells[0])] = (int(cells[1]), top_bytes(cells[2]), top_bytes(cells[3]))
    return rows


def footprint_tree(rows, root):
    if root not in rows:
        return None
    members, grown = {root}, True
    while grown:
        grown = False
        for pid, (parent, _, _) in rows.items():
            if pid not in members and parent in members:
                members.add(pid)
                grown = True
    return sum(rows[p][1] for p in members), sum(rows[p][2] for p in members)


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
    if os.environ.get("RUSTEDIT_TIMING"):
        kept["SEMIO_CARGO_TIMING"] = "1"
    kept["BUN_RUNTIME_TRANSPILER_CACHE_PATH"] = "0"
    if os.environ.get("BENCH_TOTAL_MB"):
        kept["SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES"] = os.environ["BENCH_TOTAL_MB"]
    if os.environ.get("BENCH_CAPDIR"):
        kept["SEMIO_HOST_CAPACITY_DIRECTORY"] = os.environ["BENCH_CAPDIR"]
    kept.update(extra or {})
    return kept


def run(cmd, cwd, ident, outdir, interval=0.25, extra_env=None):
    """Runs one command in its own session and returns its measurement record; the full log is kept in `outdir/<ident>.log`."""
    os.makedirs(outdir, exist_ok=True)
    log_path = os.path.join(outdir, ident + ".log")
    started_at = datetime.now(timezone.utc).isoformat()
    swap0, load0, t0 = swap_used_mb(), load1(), time.monotonic()
    gate_start = gate_stamp()
    child = subprocess.Popen(cmd, cwd=cwd, env=environment(extra_env), stdout=subprocess.PIPE, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, start_new_session=(os.name != "nt"), bufsize=0)
    stream, first_output, marker = Stream(), [None], [None]
    peak = [0, 0.0]
    peak_servers = [0]
    peak_foot = [0, 0]
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
            rows = process_table()
            value = tree_rss(rows, child.pid)
            servers = bazel_servers_rss(rows)
            if servers > peak_servers[0]:
                peak_servers[0] = servers
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

    def footprints():
        while not stop.is_set():
            value = footprint_tree(footprint_table(), child.pid)
            if value is not None and value[0] > peak_foot[0]:
                peak_foot[0], peak_foot[1] = value
            stop.wait(2.0)

    threads = [threading.Thread(target=reader, daemon=True), threading.Thread(target=sampler, daemon=True), threading.Thread(target=footprints, daemon=True)]
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
        "peakRssBytes": peak[0] or None, "peakRssAtMs": round(peak[1], 1), "bazelServersPeakRssBytes": peak_servers[0] or None, "peakFootprintBytes": peak_foot[0] or None, "peakFootprintCompressedBytes": peak_foot[1] or None,
        "gate": gate_start, "swapUsedMbStart": swap0, "swapUsedMbEnd": swap1, "swapUsedMbMax": swap_max[0],
        "load1Start": load0, "load1End": load1(),
        "phases": stream.phases(wall), "actions": stream.actions(), "graph": stream.graph,
        "capacityWaits": stream.capacity_waits, "capacityFirst": stream.capacity_first, "capacityLast": stream.capacity_last, "totalMbProfile": os.environ.get("BENCH_TOTAL_MB"),
        "log": log_path, "lines": stream.lines, "marks": stream.marks,
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


def gate_stamp():
    """The last gate-g table row id and applied time at the start of a run (which batches the measured tree holds)."""
    try:
        path = os.path.join(HERE, "..", "result-gate-g.md")
        rows = [l for l in open(path, encoding="utf8") if re.match(r"^\| \d+[a-z]? \|", l)]
        cells = [c.strip() for c in rows[-1].split("|")[1:4]]
        return {"lastRow": cells[0], "batch": cells[1][:40], "appliedAt": cells[2], "rows": len(rows)}
    except Exception:
        return None


def gate_busy():
    """True while gate-g EXECUTES a product build or applies a batch. A gate build that still waits in the capacity queue (its log ends in a `[capacity] waiting` row, or has not grown for 3 minutes) is not building: this run queues behind it in FIFO order."""
    out = subprocess.run(["ps", "-A", "-o", "pid=,command="], capture_output=True, text=True).stdout
    base = os.path.join(HERE, "..", "🗑️generated", "gate-g")
    for line in out.splitlines():
        if "gate-g-tools/" not in line or "grep" in line:
            continue
        m = re.search(r"gate-g-tools/product\.py (\S+) ", line)
        if m is None:
            if re.search(r"gate-g-tools/noop\d*\.sh", line):
                continue
            return True
        log = os.path.join(base, m.group(1) + ".log")
        try:
            if time.time() - os.path.getmtime(log) > 180:
                continue
            with open(log, "rb") as handle:
                handle.seek(0, 2)
                handle.seek(max(0, handle.tell() - 4096))
                lines = [x for x in handle.read().decode("utf8", "replace").splitlines() if x.strip()]
            if lines and lines[-1].startswith("[capacity] waiting"):
                continue
        except OSError:
            pass
        return True
    return False


def gate(root, limit=4, polls=360):
    """Waits (bounded) until gate-g EXECUTES no product build and fewer than `limit` runs wait for a lease; two quiet polls in a row."""
    limit_waiters = limit
    if os.environ.get("BENCH_QUIET") == "1":
        return
    quiet = 0
    for _ in range(polls):
        if not gate_busy() and waiters(root) < limit_waiters:
            quiet += 1
            if quiet >= 2:
                return
            time.sleep(20)
            continue
        quiet = 0
        time.sleep(60)
    raise SystemExit("gate never quiet within %d polls" % polls)


def restore(path, original, edited):
    """Byte-exact revert that keeps a peer's concurrent edit: strips our appended suffix when the file still ends with it, else rewrites only when the file is unchanged since our edit."""
    suffix = edited[len(original):] if edited.startswith(original) else None
    current = open(path, "rb").read()
    if current == edited or suffix is None:
        open(path, "wb").write(original)
    elif suffix and current.endswith(suffix):
        open(path, "wb").write(current[: -len(suffix)])
    else:
        print(json.dumps({"restore": "peer changed the file meanwhile, left as is", "path": path}), flush=True)


def product_of(name):
    return next(p for p in PRODUCTS["products"] if p["id"] == name)


def scenario(args):
    outdir = args.out
    product = product_of(args.product)
    base = "%s.%s.%s" % (args.product, args.tool, args.kind if args.kind != "peer" else "peer-" + args.peer)
    records = []

    def go(label, extra=None):
        if not args.nogate:
            gate(args.root)
        ident = "%s.%s" % (base, label)
        attempts = 0
        while True:
            attempts += 1
            record = run(repo_command(args.root, args.tool, ["run", product["target"]]), args.root, ident, outdir)
            churn = record["exit"] != 0 and record["wallMs"] < 900000 and "execute" not in record["phases"]
            if not churn or attempts >= args.retries:
                break
            print(json.dumps({"id": ident, "churn": True, "exit": record["exit"], "attempt": attempts, "wallMs": record["wallMs"]}), flush=True)
            time.sleep(150)
        record["attempts"] = attempts
        record.update({"scenario": args.kind, "product": args.product, "tool": args.tool, "label": label, "peer": args.peer if args.kind == "peer" else None, "phase": getattr(args, "tag", None)})
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
            restore(path, original, edited.encode("utf8"))
        if args.revert:
            go("r%d" % (i + 1), {"edit": rel, "reverted": True})
    json.dump(records, open(os.path.join(outdir, base + ".all.json"), "w", encoding="utf8"), indent=1, ensure_ascii=False)


def concurrent(args):
    """Three agents at once: every product build in its own run (languages en, de, en), warm (`noop`) or each after its own one-line edit (`edit`); records every tree, the sum of the peaks, queue order and the swap of the machine."""
    os.makedirs(args.out, exist_ok=True)
    if args.mode == "edit":
        refuse_live(args.root)
    gate(args.root)
    names = args.products.split(",")
    records, threads, edits = {}, [], {}
    languages = ["en", "de", "en"]

    def one(index, name):
        product = product_of(name)
        records[name] = run(repo_command(args.root, args.tool, ["run", product["target"]]), args.root, "concurrent.%s.%s.%s" % (args.mode, args.tool, name), args.out, extra_env={"SEMIO_LANGUAGE": languages[index % 3]})

    if args.mode == "edit":
        for name in names:
            product = product_of(name)
            path = os.path.join(args.root, product["editFile"])
            original = open(path, "rb").read()
            text = original.decode("utf8")
            nonce = int(time.time()) % 1000000007
            edited = ((text if text.endswith("\n") or text == "" else text + "\n") + TEMPLATES[(product["language"], "private-body")](nonce)).encode("utf8")
            os.makedirs(os.path.join(args.out, "backup"), exist_ok=True)
            open(os.path.join(args.out, "backup", product["editFile"].replace(os.sep, "_")), "wb").write(original)
            edits[name] = (path, original, edited)
    t0 = time.monotonic()
    swap0 = swap_used_mb()
    try:
        for path, original, edited in edits.values():
            open(path, "wb").write(edited)
        for index, name in enumerate(names):
            thread = threading.Thread(target=one, args=(index, name))
            thread.start()
            threads.append(thread)
            time.sleep(args.stagger)
        for thread in threads:
            thread.join()
    finally:
        for path, original, edited in edits.values():
            restore(path, original, edited)
    summary = {"mode": args.mode, "makespanMs": round((time.monotonic() - t0) * 1000, 1), "swapStartMb": swap0, "swapEndMb": swap_used_mb(), "runs": {k: {x: v[x] for x in ("exit", "wallMs", "peakRssBytes", "phases", "actions", "graph", "capacityWaits", "swapUsedMbMax", "load1Start", "bazelServersPeakRssBytes")} for k, v in records.items()}, "treeRssSumBytes": sum((v["peakRssBytes"] or 0) for v in records.values())}
    json.dump(summary, open(os.path.join(args.out, "concurrent.%s.%s.json" % (args.mode, args.tool)), "w"), indent=1)
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
        s.add_argument("--tag", default=None)
        s.set_defaults(kind=kind)
    c = sub.add_parser("concurrent")
    c.add_argument("--root", required=True)
    c.add_argument("--tool", default="bazel")
    c.add_argument("--out", required=True)
    c.add_argument("--products", default="dashboard,course,print")
    c.add_argument("--mode", choices=("noop", "edit"), default="noop")
    c.add_argument("--stagger", type=float, default=2.0)
    args = p.parse_args()
    if args.mode == "concurrent":
        concurrent(args)
    elif args.mode == "run":
        cmd = args.cmd[1:] if args.cmd and args.cmd[0] == "--" else args.cmd
        print(json.dumps(run(cmd, args.root, args.id, args.out), ensure_ascii=False, indent=1))
    else:
        scenario(args)


main()
