"""⚖️ One group verdict as a recipe of measured official product runs: `verdict.py <label> <recipe> [--plan]`.

`<recipe>` is a built-in name (`python3 -I verdict.py - list`) or a JSON file `{"stages":[{"name","parallel"?,"steps":[...]}]}`; a step is `{"suffix","target"}` (`bun repo --tool=bazel run <target> [extra]`), `{"suffix","command":[...]}` (any `bun repo --tool=bazel` argument list, e.g. a `run-many … --dry-run` warm-up), `{"suffix","edit":{"file","target","prefix"?}}` (one-line probe: `<suffix>-edit`, byte-exact restore, `<suffix>-revert`; always alone and sequential) and may carry `"unless":"served:<suffix>"` (skipped when that earlier step printed `served from the memo`).
Every step writes `run/<label>-<suffix>.{log,json}` with the product.py fields (wall, peak tree RSS, exit, queued, graph rows) plus the exact exit time (a waiting thread per child, no polling grain), RSS sampled every 0.5 s, the macOS physical footprint (`top -l 1 -stats pid,ppid,mem`, buildmem's product-fp.py method, every 5 s in its own thread; ps elsewhere) and the phase split of `phases.py`. `run/<label>.verdict.json` sums it up: per stage wall, GREEN only when every run exited 0, and whether the warm-up left the first real runs without a graph refresh.
Limits per run as product.py (20 GB tree RSS, 45 min of unqueued running time) plus 90 % of the physical memory for the sum of a parallel stage (then only the largest run of the stage is stopped, so the verdict says which one to rerun alone); only its own process groups are ever signalled. `--plan` prints the expanded steps and runs nothing."""
import importlib.util, json, os, re, signal, subprocess, sys, threading, time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.environ.get("VERDICT_ROOT") or os.path.abspath(os.path.join(HERE, *([".."] * 8)))
OUT = os.environ.get("VERDICT_OUT") or os.path.join(HERE, "run")
COMMAND = json.loads(os.environ["VERDICT_COMMAND"]) if os.environ.get("VERDICT_COMMAND") else ["nice", "-n", "10", "bun", "repo", "--tool=bazel"]
LIMIT_KB, LIMIT_S, TICK, FOOTPRINT_S = 20 * 1024 * 1024, 45 * 60, 0.5, 5
STAGE_KB = int(os.sysconf("SC_PAGE_SIZE") * os.sysconf("SC_PHYS_PAGES") * 0.9 / 1024) if hasattr(os, "sysconf") else 28 * 1024 * 1024
DROP = ("CLAUDE", "ANTHROPIC", "__CF", "XPC_", "TERM_SESSION_ID", "SEMIO_SCRIPT_STALL_MS")
UNITS = {"B": 1 / 1024, "K": 1, "M": 1024, "G": 1024 * 1024}
SERVED = re.compile(r"served from the memo")
REFRESH = re.compile(r"^graph: refreshing (\d+) projects", re.M)
COURSE, CTEST, PRINT, DASH = "@teaching/architecture-course:build", "@teaching/architecture-course:test", "@semio-tech/print-rs:test", "@semio-tech/repo-dashboard-rs:build"
COURSE_FILE, PRINT_FILE = "🧰️framework/🛍️products/📚️course/🟦️.ts", "🧰️framework/🛍️products/📓️print/🦀️.rs"
WARM = {"suffix": "warm", "command": ["run-many", "--targets=build,test", "--projects=@teaching/architecture-course,@semio-tech/print-rs,@semio-tech/repo-dashboard-rs", "--dry-run"]}
RECIPES = {
    "group": {"stages": [
        {"name": "warm", "steps": [WARM]},
        {"name": "first", "parallel": True, "steps": [{"suffix": "course", "target": COURSE}, {"suffix": "print1", "target": PRINT}, {"suffix": "dash1", "target": DASH}]},
        {"name": "second", "steps": [{"suffix": "ctest", "target": CTEST}, {"suffix": "hot1", "target": COURSE}, {"suffix": "hot2", "target": COURSE}, {"suffix": "hot3", "target": COURSE, "unless": "served:hot2"}, {"suffix": "print2", "target": PRINT}, {"suffix": "print3", "target": PRINT, "unless": "served:print2"}, {"suffix": "dash2", "target": DASH}]},
        {"name": "edits", "steps": [{"suffix": "c", "edit": {"file": COURSE_FILE, "target": COURSE}}, {"suffix": "p", "edit": {"file": PRINT_FILE, "target": PRINT}}]},
    ]},
    "group-seq": {"stages": [
        {"name": "sequential", "steps": [{"suffix": "course", "target": COURSE}, {"suffix": "ctest", "target": CTEST}, {"suffix": "hot1", "target": COURSE}, {"suffix": "hot2", "target": COURSE}, {"suffix": "hot3", "target": COURSE}, {"suffix": "print1", "target": PRINT}, {"suffix": "print2", "target": PRINT}, {"suffix": "print3", "target": PRINT}, {"suffix": "dash1", "target": DASH}, {"suffix": "dash2", "target": DASH}]},
        {"name": "edits", "steps": [{"suffix": "c", "edit": {"file": COURSE_FILE, "target": COURSE}}, {"suffix": "p", "edit": {"file": PRINT_FILE, "target": PRINT}}]},
    ]},
    "course": {"stages": [
        {"name": "first", "steps": [{"suffix": "course", "target": COURSE}, {"suffix": "ctest", "target": CTEST}, {"suffix": "hot1", "target": COURSE}, {"suffix": "hot2", "target": COURSE}, {"suffix": "hot3", "target": COURSE, "unless": "served:hot2"}]},
        {"name": "edits", "steps": [{"suffix": "c", "edit": {"file": COURSE_FILE, "target": COURSE}}]},
    ]},
    "print": {"stages": [
        {"name": "first", "steps": [{"suffix": "print1", "target": PRINT}, {"suffix": "print2", "target": PRINT}, {"suffix": "print3", "target": PRINT, "unless": "served:print2"}]},
        {"name": "edits", "steps": [{"suffix": "p", "edit": {"file": PRINT_FILE, "target": PRINT}}]},
    ]},
    "dash": {"stages": [{"name": "first", "steps": [{"suffix": "dash1", "target": DASH}, {"suffix": "dash2", "target": DASH}]}]},
}


def load_phases():
    spec = importlib.util.spec_from_file_location("gate_phases", os.path.join(HERE, "phases.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


PHASES = load_phases()


def recipe_of(name):
    if name in RECIPES:
        return RECIPES[name]
    with open(name, encoding="utf8") as stream:
        return json.load(stream)


def expand(recipe):
    stages = []
    for stage in recipe["stages"]:
        steps = []
        for step in stage["steps"]:
            if "edit" in step:
                if stage.get("parallel"):
                    raise SystemExit("an edit probe never runs in a parallel stage: " + step["suffix"])
                edit = step["edit"]
                steps.append({"suffix": step["suffix"] + "-edit", "args": ["run", edit["target"]], "probe": edit, "unless": step.get("unless")})
                steps.append({"suffix": step["suffix"] + "-revert", "args": ["run", edit["target"]], "unless": step.get("unless")})
            else:
                args = list(step["command"]) if "command" in step else ["run", step["target"], *step.get("extra", [])]
                steps.append({"suffix": step["suffix"], "args": args, "unless": step.get("unless")})
        stages.append({"name": stage["name"], "parallel": bool(stage.get("parallel")), "steps": steps})
    return stages


def table():
    rows = {}
    for line in subprocess.run(["ps", "-A", "-o", "pid=,ppid=,rss="], capture_output=True, text=True).stdout.splitlines():
        cells = line.split()
        if len(cells) == 3 and all(c.isdigit() for c in cells):
            rows[int(cells[0])] = (int(cells[1]), int(cells[2]))
    return rows


def descendants(root, parents):
    kids = {}
    for pid, (ppid, _) in parents.items():
        kids.setdefault(ppid, []).append(pid)
    seen, stack = [], [root]
    while stack:
        pid = stack.pop()
        seen.append(pid)
        stack.extend(kids.get(pid, []))
    return seen


def footprints():
    if sys.platform != "darwin":
        return {}
    rows = {}
    for line in subprocess.run(["top", "-l", "1", "-stats", "pid,ppid,mem"], capture_output=True, text=True).stdout.splitlines():
        cells = line.split()
        if len(cells) == 3 and cells[0].isdigit() and cells[1].isdigit():
            value = cells[2].rstrip("+-")
            if value and value[-1] in UNITS:
                try:
                    rows[int(cells[0])] = (int(cells[1]), float(value[:-1]) * UNITS[value[-1]])
                except ValueError:
                    pass
    return rows


def queued(log):
    try:
        with open(log, "rb") as stream:
            stream.seek(0, 2)
            stream.seek(max(0, stream.tell() - 600))
            tail = stream.read().decode("utf8", "replace").strip().splitlines()[-1:] or [""]
    except OSError:
        return False
    return "[capacity] waiting" in tail[0] or "a peer builds the graph" in tail[0]


def command_of(pid):
    text = subprocess.run(["ps", "-o", "command=", "-p", str(pid)], capture_output=True, text=True).stdout.strip()
    for marker in ("--label", "build:graph-plan", "repository:command", "bazel", "cargo", "rustc"):
        if marker in text:
            return marker + " | " + text[-120:]
    return text[:160]


class Run:
    """🏃 One official product run in its own process group, with an exact exit time from a waiting thread."""

    def __init__(self, label, step, environment):
        self.label, self.step, self.suffix = label, step, step["suffix"]
        self.log = os.path.join(OUT, f"{label}-{self.suffix}.log")
        self.started_at, self.t0 = time.strftime("%H:%M:%S"), time.monotonic()
        self.handle = open(self.log, "wb", buffering=0)
        self.child = subprocess.Popen([*COMMAND, *step["args"]], cwd=ROOT, env=environment, stdout=self.handle, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, start_new_session=True)
        self.ended, self.peak, self.peak_n, self.peak_fp, self.parked, self.tick, self.killed, self.procs = None, 0, 0, 0.0, 0.0, time.monotonic(), None, {}
        threading.Thread(target=self.wait, daemon=True).start()

    def wait(self):
        self.child.wait()
        self.ended = time.monotonic()

    def sample(self, rows):
        now = time.monotonic()
        self.parked += (now - self.tick) if queued(self.log) else 0.0
        self.tick = now
        pids = [pid for pid in descendants(self.child.pid, rows) if pid in rows]
        kb = sum(rows[pid][1] for pid in pids)
        self.peak, self.peak_n = max(self.peak, kb), max(self.peak_n, len(pids))
        return kb

    def sample_footprint(self, fps, rows):
        pids = descendants(self.child.pid, fps)
        self.peak_fp = max(self.peak_fp, sum(fps[pid][1] for pid in pids if pid in fps))
        for pid in pids:
            if pid in fps:
                row = self.procs.setdefault(pid, {"pid": pid, "rss_mb": 0, "fp_mb": 0, "cmd": None})
                row["rss_mb"], row["fp_mb"] = max(row["rss_mb"], rows.get(pid, (0, 0))[1] // 1024), max(row["fp_mb"], int(fps[pid][1] // 1024))
                if row["cmd"] is None and row["fp_mb"] > 300:
                    row["cmd"] = command_of(pid)

    def overdue(self):
        return time.monotonic() - self.t0 - self.parked > LIMIT_S

    def kill(self, why):
        if self.ended is not None or self.killed:
            return
        self.killed = why
        try:
            os.killpg(self.child.pid, signal.SIGTERM)
        except ProcessLookupError:
            return
        deadline = time.monotonic() + 10
        while self.ended is None and time.monotonic() < deadline:
            time.sleep(0.2)
        if self.ended is None:
            try:
                os.killpg(self.child.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass

    def record(self, stage):
        while self.ended is None:
            time.sleep(0.05)
        self.handle.close()
        found = {}
        with open(self.log, encoding="utf8", errors="replace") as stream:
            for line in stream:
                for key, pattern in PHASES.FACTS.items():
                    if key in found:
                        continue
                    match = pattern.search(line)
                    if match:
                        found[key] = list(match.groups())
        text = open(self.log, encoding="utf8", errors="replace").read()
        record = {"label": f"{self.label}-{self.suffix}", "target": " ".join(self.step["args"]), "stage": stage, "started": self.started_at, "wall_s": round(self.ended - self.t0, 2), "peak_tree_mb": self.peak // 1024, "peak_tree_footprint_mb": int(self.peak_fp // 1024), "largest": sorted(self.procs.values(), key=lambda r: -r["fp_mb"])[:5], "peak_processes": self.peak_n, "exit": self.child.returncode, "killed": self.killed, "queued_s": round(self.parked, 1), "graph": found, "served": bool(SERVED.search(text)), "refreshed": int(REFRESH.search(text).group(1)) if REFRESH.search(text) else 0}
        record["phases"] = PHASES.split(self.log)["spans"]
        with open(os.path.join(OUT, f"{self.label}-{self.suffix}.json"), "w", encoding="utf8") as stream:
            json.dump(record, stream, ensure_ascii=False, indent=1)
        print(json.dumps({k: record[k] for k in ("label", "stage", "wall_s", "exit", "killed", "peak_tree_mb", "peak_tree_footprint_mb", "refreshed", "served", "phases")}, ensure_ascii=False), flush=True)
        return record


def run_stage(label, stage, environment, done):
    groups = [[step for step in stage["steps"] if not skipped(step, done)]] if stage["parallel"] else [[step] for step in stage["steps"]]
    records = []
    for group in groups:
        group = [step for step in group if not skipped(step, done)]
        if not group:
            continue
        restore = None
        if group[0].get("probe"):
            probe = group[0]["probe"]
            path = os.path.join(ROOT, probe["file"])
            original = open(path, "rb").read()
            restore = (path, original)
            open(path, "ab").write((("\n" if not original.endswith(b"\n") else "") + probe.get("prefix", "//") + " gate probe " + label + "\n").encode())
        try:
            runs = [Run(label, step, environment) for step in group]
            stop = threading.Event()

            def sample_fp():
                while not stop.is_set():
                    fps = footprints()
                    if fps:
                        rows = table()
                        for run in runs:
                            if run.ended is None:
                                run.sample_footprint(fps, rows)
                    stop.wait(FOOTPRINT_S)

            sampler = threading.Thread(target=sample_fp, daemon=True)
            sampler.start()
            while any(run.ended is None for run in runs):
                rows = table()
                sizes = {}
                for run in runs:
                    if run.ended is None:
                        sizes[run] = run.sample(rows)
                        if sizes[run] > LIMIT_KB:
                            run.kill("rss")
                        elif run.overdue():
                            run.kill("time")
                if len(sizes) > 1 and sum(sizes.values()) > STAGE_KB:
                    max(sizes, key=sizes.get).kill("rss-stage")
                time.sleep(TICK)
            stop.set()
        finally:
            if restore:
                open(restore[0], "wb").write(restore[1])
        for run in runs:
            record = run.record(stage["name"])
            done[run.suffix] = record
            records.append(record)
    return records


def skipped(step, done):
    unless = step.get("unless")
    if not unless:
        return False
    kind, _, suffix = unless.partition(":")
    return kind == "served" and bool(done.get(suffix, {}).get("served"))


def main(arguments):
    plan_only = "--plan" in arguments
    arguments = [a for a in arguments if a != "--plan"]
    if len(arguments) == 2 and arguments[1] == "list":
        for name, recipe in RECIPES.items():
            print(name, " | ".join(f"{s['name']}{'(parallel)' if s.get('parallel') else ''}: " + ",".join(step["suffix"] for step in s["steps"]) for s in recipe["stages"]))
        return 0
    label, recipe = arguments
    stages = expand(recipe_of(recipe))
    if plan_only:
        for stage in stages:
            print(stage["name"], "parallel" if stage["parallel"] else "sequential")
            for step in stage["steps"]:
                print("  ", f"{label}-{step['suffix']}", "bun repo --tool=bazel " + " ".join(step["args"]), "(probe " + step["probe"]["file"] + ")" if step.get("probe") else "", ("unless " + step["unless"]) if step.get("unless") else "")
        return 0
    os.makedirs(OUT, exist_ok=True)
    environment = {k: v for k, v in os.environ.items() if not k.startswith(DROP)}
    environment.setdefault("SEMIO_LANGUAGE", "en")
    t0, done, summary = time.monotonic(), {}, {"label": label, "recipe": recipe, "started": time.strftime("%H:%M:%S"), "stages": []}
    for stage in stages:
        s0 = time.monotonic()
        records = run_stage(label, stage, environment, done)
        summary["stages"].append({"name": stage["name"], "parallel": stage["parallel"], "wall_s": round(time.monotonic() - s0, 1), "runs": [{k: r[k] for k in ("label", "wall_s", "exit", "killed", "refreshed", "served")} for r in records]})
        if any(r["exit"] != 0 for r in records):
            summary["aborted"] = stage["name"]
            break
    runs = [run for stage in summary["stages"] for run in stage["runs"]]
    warm = next((s for s in summary["stages"] if s["name"] == "warm"), None)
    after = next((s for s in summary["stages"] if s["name"] == "first"), None)
    summary.update({"wall_s": round(time.monotonic() - t0, 1), "verdict": "GREEN" if runs and "aborted" not in summary and all(r["exit"] == 0 for r in runs) else "RED", "warm_effective": None if not (warm and after) else all(r["refreshed"] == 0 for r in after["runs"])})
    with open(os.path.join(OUT, label + ".verdict.json"), "w", encoding="utf8") as stream:
        json.dump(summary, stream, ensure_ascii=False, indent=1)
    print(json.dumps({k: summary[k] for k in ("label", "recipe", "wall_s", "verdict", "warm_effective")}, ensure_ascii=False))
    return 0 if summary["verdict"] == "GREEN" else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
