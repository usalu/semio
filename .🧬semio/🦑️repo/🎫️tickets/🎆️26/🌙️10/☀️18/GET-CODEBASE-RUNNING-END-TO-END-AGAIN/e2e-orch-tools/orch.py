#!/usr/bin/env python3
"""🧭 Orchestrated, detachable end-to-end driver for course, play and demo on Bazel (ticket input script, Python 3.9+, standard library only).

Usage: run.sh <course|play|demo|all> [--phase preflight|cold|warm|e2e|probe|report|all[,..]] [--go] [--run-id ID] [--parallel N] ...
       run.sh status|cancel [--run-id ID]
Plan: plan.json (schema plan.schema.json). Vectors for the pure functions: vectors.json. Selftest: selftest.py.
"""
from __future__ import annotations

import argparse
import concurrent.futures
import datetime
import json
import os
import platform
import re
import shutil
import signal
import socket
import subprocess
import sys
import threading
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
TICKET = HERE.parent
GENERATED = "\U0001F5D1️generated"
PHASE_ORDER = ["cold", "warm", "e2e", "probe"]
ANSI = re.compile(r"\x1b\[[0-9;?]*[ -/]*[@-~]")
SUMMARY = re.compile(r"(\d+) process(?:es)?:\s*(.+?)\.?\s*$")
SLOT = re.compile(r"\bslot ([sd]\d+)\b")
WAIT = re.compile(r"(?i)(slots? (?:are|is) in use|waiting for access|waiting for one|position \d+ of \d+|\[capacity\].*(?:waiting|queue)|host capacity.*(?:wait|queue))")
EVIDENCE = re.compile(r"^\s*(Compiling \S+ v\d|.*\bBundling\b)")
PLAYWRIGHT_RESULT = re.compile(r"^\s*(\d+) (passed|failed|flaky|skipped)\b")
STALE_RC = 75
STALE_TEXT = "runs an outdated implementation"
CACHED_RUNNERS = ("disk cache hit", "remote cache hit")
INTERNAL_RUNNER = "internal"


def now_iso() -> str:
    """🕒 Local time with offset, second precision."""
    return datetime.datetime.now().astimezone().isoformat(timespec="seconds")


def fmt_seconds(value) -> str:
    """⏱️ Compact duration such as 1h02m03s."""
    if value is None:
        return "-"
    value = int(round(value))
    hours, rest = divmod(value, 3600)
    minutes, seconds = divmod(rest, 60)
    if hours:
        return "%dh%02dm%02ds" % (hours, minutes, seconds)
    if minutes:
        return "%dm%02ds" % (minutes, seconds)
    return "%ds" % seconds


def strip_ansi(text: str) -> str:
    """🧽 Removes terminal escape sequences."""
    return ANSI.sub("", text)


def parse_summary_line(line: str):
    """📊 Parses one Bazel `N processes: a x, b y` line into a dict, or None."""
    match = SUMMARY.search(strip_ansi(line).strip())
    if not match:
        return None
    runners = {}
    for part in match.group(2).split(", "):
        piece = re.match(r"(\d+) (.+)$", part.strip())
        if piece:
            name = piece.group(2).strip()
            runners[name] = runners.get(name, 0) + int(piece.group(1))
    internal = runners.get(INTERNAL_RUNNER, 0)
    hits = sum(runners.get(name, 0) for name in CACHED_RUNNERS)
    spawns = sum(count for name, count in runners.items() if name != INTERNAL_RUNNER and name not in CACHED_RUNNERS)
    return {"processes": int(match.group(1)), "runners": runners, "internal": internal, "cacheHits": hits, "spawns": spawns}


def parse_log(text: str) -> dict:
    """🔎 Aggregates Bazel summaries, session slots, waits, work evidence and Playwright results of one log."""
    result = {"summaries": 0, "processes": 0, "spawns": 0, "cacheHits": 0, "internal": 0, "runners": {}, "slots": [], "waits": [], "evidence": [], "tests": {}}
    for raw in text.splitlines():
        line = strip_ansi(raw)
        parsed = parse_summary_line(line)
        if parsed:
            result["summaries"] += 1
            for key in ("processes", "spawns", "cacheHits", "internal"):
                result[key] += parsed[key]
            for name, count in parsed["runners"].items():
                result["runners"][name] = result["runners"].get(name, 0) + count
        slot = SLOT.search(line)
        if slot and slot.group(1) not in result["slots"]:
            result["slots"].append(slot.group(1))
        if WAIT.search(line) and len(result["waits"]) < 5:
            result["waits"].append(line.strip()[:200])
        if EVIDENCE.search(line) and len(result["evidence"]) < 5:
            result["evidence"].append(line.strip()[:200])
        tests = PLAYWRIGHT_RESULT.match(line)
        if tests:
            result["tests"][tests.group(2)] = result["tests"].get(tests.group(2), 0) + int(tests.group(1))
    return result


def warm_verdict(mode: str, allow: int, rc: int, spawns: int, summaries: int, seconds: float, max_seconds, slack: int = 0):
    """⚖️ Verdict of a warm step: (status, note). Zero spawns unless the plan allows named uncached probes."""
    if rc != 0:
        return "FAIL", "exit code %d" % rc
    if mode == "skip":
        return "PASS", "not asserted by plan"
    limit = 0 if mode == "zero" else allow + slack
    if summaries == 0:
        return "WARN", "no Bazel summary line in the log; cache hits not provable"
    if spawns > limit:
        return "FAIL", "%d spawns, at most %d allowed" % (spawns, limit)
    if max_seconds and seconds > max_seconds:
        return "WARN", "%d spawns (<= %d) but %s exceeds the expected %s" % (spawns, limit, fmt_seconds(seconds), fmt_seconds(max_seconds))
    return "PASS", "%d spawns (<= %d)" % (spawns, limit)


def plain_verdict(rc: int, parsed: dict):
    """⚖️ Verdict of a cold, e2e or probe step: exit code decides; Playwright failures also fail."""
    if rc != 0:
        return "FAIL", "exit code %d" % rc
    if parsed["tests"].get("failed", 0):
        return "FAIL", "%d test(s) failed" % parsed["tests"]["failed"]
    return "PASS", ""


NEUTRAL = re.compile(r"^(\s*$|#|\[repository:command\] running elapsedMs=)")


class QueueClock:
    """⏳ Seconds a step spent queued (capacity, lease or Bazel slot waits): a wait line opens the credit, any other output closes it, and a silent queue expires after `grace` seconds. Step timeouts count from admission: deadline = start + timeout + credit."""

    def __init__(self, grace: float = 60.0):
        self.grace, self.credit, self.tick, self.since, self.last = grace, 0.0, 0.0, None, None

    def feed(self, text: str, now: float) -> None:
        """📥 Reads new log text."""
        for raw in text.splitlines():
            line = strip_ansi(raw)
            if NEUTRAL.match(line):
                continue
            if WAIT.search(line):
                self.since = now if self.since is None else self.since
                self.last = now
            else:
                self.update(now)
                self.since = self.last = None

    def update(self, now: float) -> float:
        """🕰️ Accrues the waiting time since the previous call and returns the total credit."""
        if self.since is not None:
            self.credit += max(0.0, min(now, self.last + self.grace) - max(self.tick, self.since))
        self.tick = now
        return self.credit


def counters(text: str) -> dict:
    """🔢 Numeric leaves of a status text (JSON when it parses, else `label: N` lines)."""
    found = {}
    try:
        data = json.loads(text)
    except ValueError:
        data = None
    if data is not None:
        def walk(node, path):
            if isinstance(node, dict):
                for key, value in node.items():
                    walk(value, path + [str(key)])
            elif isinstance(node, list):
                for index, value in enumerate(node):
                    walk(value, path + [str(index)])
            elif isinstance(node, (int, float)) and not isinstance(node, bool):
                found[".".join(path)] = node
        walk(data, [])
        return found
    for raw in text.splitlines():
        match = re.match(r"^\s*([A-Za-z][^:=\n]{0,60}?)\s*[:=]\s*(-?\d+(?:\.\d+)?)\s*$", strip_ansi(raw))
        if match:
            found[match.group(1).strip()] = float(match.group(2)) if "." in match.group(2) else int(match.group(2))
    return found


def counter_delta(before: str, after: str) -> list:
    """➖ Rows (label, before, after, delta) of the counters that changed."""
    first, second = counters(before), counters(after)
    rows = []
    for key in sorted(set(first) | set(second)):
        a, b = first.get(key, 0), second.get(key, 0)
        if a != b:
            rows.append((key, a, b, b - a))
    return rows


def lane_slots(memory_mb: int, cores: int, resident_mb: int = 2560) -> int:
    """🧵 Build slots the adapter derives from the host budget (floor 20 percent clamped 4 to 8 GB, 30 percent share, at most cores/2)."""
    floor = min(8192, max(4096, memory_mb * 0.2))
    slots = int(max(0, memory_mb - floor) * 0.3 // resident_mb)
    return max(1, min(6, slots, max(1, cores // 2)))


def read_tail(path: Path, limit: int = 4096) -> str:
    """📜 Last bytes of a file as text."""
    try:
        with open(path, "rb") as handle:
            handle.seek(0, os.SEEK_END)
            size = handle.tell()
            handle.seek(max(0, size - limit))
            return handle.read().decode("utf-8", "replace")
    except OSError:
        return ""


def read_text(path: Path) -> str:
    """📖 Whole file as text, empty when unreadable."""
    try:
        return path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return ""


def write_atomic(path: Path, text: str) -> None:
    """💾 Atomic text write."""
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + ".tmp%d" % os.getpid())
    temporary.write_text(text, encoding="utf-8")
    os.replace(str(temporary), str(path))


def pid_alive(pid: int) -> bool:
    """🫀 Whether a process exists."""
    if pid <= 0:
        return False
    try:
        os.kill(pid, 0)
        return True
    except PermissionError:
        return True
    except OSError:
        return False


def capture(argv, timeout, cwd=None, env=None, machine=False):
    """📥 Runs a short command; returns (rc, output) with rc 124 on timeout and 127 when missing. A `machine` command (`--json`) keeps its result stream apart: stdout is the document, stderr the progress."""
    try:
        done = subprocess.run(argv, cwd=cwd, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE if machine else subprocess.STDOUT, stdin=subprocess.DEVNULL, timeout=timeout)
        return done.returncode, done.stdout.decode("utf-8", "replace") + (done.stderr.decode("utf-8", "replace") if machine and done.returncode != 0 else "")
    except subprocess.TimeoutExpired as error:
        return 124, (error.stdout or b"").decode("utf-8", "replace")
    except OSError as error:
        return 127, str(error)


def port_open(port: int) -> bool:
    """🔌 Whether something accepts connections on 127.0.0.1:port."""
    try:
        with socket.create_connection(("127.0.0.1", port), timeout=1):
            return True
    except OSError:
        return False


def memory_gb():
    """🧠 (total, available) memory in GB, None where the platform gives no answer."""
    system = platform.system()
    if system == "Darwin":
        rc, total = capture(["sysctl", "-n", "hw.memsize"], 10)
        rc2, vm = capture(["vm_stat"], 10)
        if rc != 0 or rc2 != 0:
            return None, None
        size = re.search(r"page size of (\d+) bytes", vm)
        pages = 0
        for label in ("Pages free", "Pages inactive", "Pages speculative"):
            hit = re.search(re.escape(label) + r":\s+(\d+)", vm)
            pages += int(hit.group(1)) if hit else 0
        return int(total.strip()) / 2**30, pages * (int(size.group(1)) if size else 16384) / 2**30
    if system == "Linux":
        info = read_text(Path("/proc/meminfo"))
        total = re.search(r"MemTotal:\s+(\d+) kB", info)
        free = re.search(r"MemAvailable:\s+(\d+) kB", info)
        return (int(total.group(1)) / 2**20 if total else None), (int(free.group(1)) / 2**20 if free else None)
    return None, None


def free_disk_gb(path: Path) -> float:
    """💽 Free space of the volume holding path."""
    usage = shutil.disk_usage(str(path))
    return usage.free / 2**30


def load_average():
    """📈 One-minute load average or None."""
    try:
        return os.getloadavg()[0]
    except (OSError, AttributeError):
        return None


def kill_group(process, grace: float) -> None:
    """🛑 Terminates a child and its process group, escalating to kill after the grace period."""
    if process.poll() is not None:
        return
    def send(signal_number):
        try:
            if os.name == "posix":
                os.killpg(process.pid, signal_number)
            elif signal_number == signal.SIGTERM:
                process.terminate()
            else:
                process.kill()
        except (OSError, ProcessLookupError):
            pass
    send(signal.SIGTERM)
    deadline = time.time() + grace
    while time.time() < deadline and process.poll() is None:
        time.sleep(0.5)
    if process.poll() is None:
        send(signal.SIGKILL if os.name == "posix" else signal.SIGTERM)
        try:
            process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            pass


class Orchestrator:
    """🎛️ One run: preflight, per-product pipelines in parallel, snapshots, monitor, report."""

    def __init__(self, args, plan):
        self.args = args
        self.plan = plan
        self.th = plan["thresholds"]
        self.ticket = Path(args.ticket).resolve()
        self.root = Path(args.root).resolve() if args.root else self.find_root()
        self.base = Path(args.out).resolve() if args.out else self.ticket / GENERATED / "e2e-orch"
        self.report_dir = Path(args.report_dir).resolve() if args.report_dir else self.ticket
        self.run_id = args.run_id or datetime.datetime.now().strftime("%Y%m%dT%H%M%S")
        self.dir = self.base / self.run_id
        self.report_path = self.report_dir / ("report-e2e-orch-%s.md" % self.run_id)
        self.cancel = threading.Event()
        self.cancel_reason = ""
        self.lock = threading.Lock()
        self.running = {}
        self.state = {"runId": self.run_id, "created": now_iso(), "argv": sys.argv[1:], "steps": [], "preflight": None, "snapshots": [], "samples": {"maxLoad": None, "minAvailableGb": None, "minFreeDiskGb": None}, "events": [], "cancelled": False, "finished": None, "pid": os.getpid()}
        self.snapshot_busy = threading.Lock()
        self.deadline = time.time() + self.th["deadlineHours"] * 3600
        self.parallel_hint = 1

    def find_root(self) -> Path:
        """🌳 Repository root via git, falling back to the nearest ancestor holding .git."""
        rc, out = capture(["git", "-C", str(self.ticket), "rev-parse", "--show-toplevel"], 30)
        if rc == 0 and out.strip():
            return Path(out.strip())
        for parent in [self.ticket] + list(self.ticket.parents):
            if (parent / ".git").exists():
                return parent
        raise SystemExit("repository root not found")

    def log(self, message: str) -> None:
        """📣 Timestamped line to stdout and the run log."""
        line = "[e2e-orch %s] %s" % (time.strftime("%H:%M:%S"), message)
        print(line, flush=True)
        with self.lock:
            self.state["events"].append(line)
            self.state["events"] = self.state["events"][-400:]

    def save(self) -> None:
        """💾 Persists the state."""
        with self.lock:
            text = json.dumps(self.state, ensure_ascii=False, indent=1, default=str)
        write_atomic(self.dir / "state.json", text)

    def load_state(self) -> bool:
        """📂 Loads an earlier state of the same run id."""
        path = self.dir / "state.json"
        if not path.exists():
            return False
        try:
            old = json.loads(read_text(path))
        except ValueError:
            return False
        old["argv"] = list(old.get("argv", [])) + ["||"] + sys.argv[1:]
        old["cancelled"] = False
        old["finished"] = None
        old["pid"] = os.getpid()
        old.setdefault("events", [])
        self.state.update(old)
        return True

    def child_env(self, product: str, extra=None) -> dict:
        """🌐 Environment of a child: English output, private cargo lane, stable stamps; Playwright path left to the tools."""
        env = dict(os.environ)
        env.pop("PLAYWRIGHT_BROWSERS_PATH", None)
        extra_path = [os.path.expanduser("~/.cargo/bin"), "/opt/homebrew/bin", "/usr/local/bin"]
        env["PATH"] = os.pathsep.join([entry for entry in extra_path if os.path.isdir(entry)] + [env.get("PATH", "")])
        env["SEMIO_LANGUAGE"] = "en"
        env["LANG"] = env.get("LANG") or "en_US.UTF-8"
        env["NO_COLOR"] = "1"
        env["SEMIO_TICKET_DIR"] = str(self.ticket)
        if product in self.plan["products"]:
            env["SEMIO_CARGO_LANE"] = env.get("E2E_ORCH_LANE_" + product.upper(), self.plan["products"][product]["cargoLane"])
            env["E2E_STAMP"] = "%s-%s" % (self.run_id, product)
        for key in ("PLAY_BZL_STAMP", "DEMO_BZL_STAMP"):
            env[key] = self.run_id
        env["E2E_SERVE_TIMEOUT"] = env.get("E2E_SERVE_TIMEOUT", "3600")
        env.update(extra or {})
        return env

    def substitute(self, text: str) -> str:
        """🔤 Fills {courseMode} and {courseTopology}."""
        return text.replace("{courseMode}", self.args.course_mode).replace("{courseTopology}", self.args.course_topology or "")

    def command_for(self, step: dict) -> list:
        """🧩 argv of a step."""
        if step["kind"] == "repo":
            return ["bun", "repo", "--tool=%s" % self.plan["tool"]] + [self.substitute(a) for a in step["args"]]
        if step["kind"] == "exec":
            return [self.substitute(a) for a in step["argv"]]
        script = self.ticket / step["script"]
        first = read_text(script).splitlines()[:1]
        interpreter = "sh" if first and first[0].strip().endswith("/sh") else "bash"
        extra = []
        declared = step["script"].endswith("run-e2e.sh") and step["args"][0] == "course"
        if declared and self.args.course_topology != "dev":
            extra.append("--release")
        arguments = [a for a in step["args"] if not (declared and "{courseTopology}" in a and not self.args.course_topology)]
        return [interpreter, str(script)] + [self.substitute(a) for a in arguments] + extra

    def steps_of(self, product: str, phase: str) -> list:
        """📋 Steps of a product phase (warm repeats cold)."""
        steps = self.plan["products"][product]["steps"][phase]
        if steps == "cold":
            steps = self.plan["products"][product]["steps"]["cold"]
        return steps

    def find_step_records(self, product, phase, step_id):
        """🔍 Recorded results of a step."""
        with self.lock:
            return [r for r in self.state["steps"] if r["product"] == product and r["phase"] == phase and r["id"] == step_id]

    def group_failed(self, product: str, group: str) -> bool:
        """❌ Whether any recorded step of the group failed, was skipped for failure or was cancelled."""
        with self.lock:
            return any(r["product"] == product and r["group"] == group and r["status"] in ("FAIL", "CANCELLED", "SKIP-FAILED") and r["rc"] != 0 and r["phase"] in ("cold", "warm") for r in self.state["steps"])

    def record(self, entry: dict) -> None:
        """🧾 Appends a step record."""
        with self.lock:
            self.state["steps"] = [r for r in self.state["steps"] if not (r["product"] == entry["product"] and r["phase"] == entry["phase"] and r["id"] == entry["id"])] + [entry]
        self.save()

    def execute(self, product, phase, step, index) -> dict:
        """▶️ Runs one step as a child process group with timeout, cancellation and a per-step log."""
        command = self.command_for(step)
        log_path = self.dir / product / ("%s-%d-%s.log" % (phase, index, step["id"]))
        log_path.parent.mkdir(parents=True, exist_ok=True)
        log_base = log_path.stat().st_size if log_path.exists() else 0
        entry = {"product": product, "phase": phase, "id": step["id"], "group": step["group"], "cmd": command, "start": now_iso(), "end": None, "seconds": None, "rc": None, "status": "RUNNING", "note": "", "log": self.rel(log_path), "bazel": {}, "slots": [], "waits": [], "evidence": [], "tests": {}}
        self.record(entry)
        env = self.child_env(product, step.get("env"))
        started = time.time()
        key = "%s:%s:%s" % (product, phase, step["id"])
        self.log("%s: start %s" % (key, " ".join(command)))
        rc, stopped = 0, ""
        with open(log_path, "ab", buffering=0) as handle:
            handle.write(("# %s\n# cwd %s\n" % (" ".join(command), self.root)).encode("utf-8"))
            try:
                kwargs = {"start_new_session": True} if os.name == "posix" else {"creationflags": getattr(subprocess, "CREATE_NEW_PROCESS_GROUP", 0)}
                process = subprocess.Popen(command, cwd=str(self.root), env=env, stdout=handle, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, **kwargs)
            except OSError as error:
                handle.write(("spawn failed: %s\n" % error).encode("utf-8"))
                process = None
                rc = 127
            if process is not None:
                with self.lock:
                    self.running[key] = {"started": started, "log": log_path, "pid": process.pid}
                queue, offset = QueueClock(self.th.get("queueGraceSeconds", 60)), log_base
                while True:
                    try:
                        rc = process.wait(timeout=1)
                        break
                    except subprocess.TimeoutExpired:
                        pass
                    with open(log_path, "rb") as reader:
                        reader.seek(offset)
                        fresh = reader.read()
                    offset += len(fresh)
                    clock = time.time()
                    queue.feed(fresh.decode("utf-8", "replace"), clock)
                    waited = queue.update(clock)
                    if self.cancel.is_set():
                        stopped = "cancelled: " + (self.cancel_reason or "requested")
                    elif clock > started + step["timeoutSeconds"] + waited:
                        stopped = "timeout after %s of work (+ %s queued)" % (fmt_seconds(step["timeoutSeconds"]), fmt_seconds(waited))
                    if stopped:
                        self.log("%s: stopping (%s)" % (key, stopped))
                        kill_group(process, self.th["terminateGraceSeconds"])
                        rc = process.returncode if process.returncode is not None else 1
                        break
                with self.lock:
                    self.running.pop(key, None)
        seconds = time.time() - started
        parsed = parse_log("\n".join(log_path.read_bytes()[log_base:].decode("utf-8", "replace").splitlines()[2:]))
        entry.update({"end": now_iso(), "seconds": round(seconds, 1), "rc": rc, "bazel": {k: parsed[k] for k in ("summaries", "processes", "spawns", "cacheHits", "internal", "runners")}, "slots": parsed["slots"], "waits": parsed["waits"], "evidence": parsed["evidence"], "tests": parsed["tests"]})
        if stopped:
            entry["status"] = "CANCELLED" if stopped.startswith("cancelled") else "FAIL"
            entry["note"] = stopped
        elif phase == "warm":
            warm = step.get("warm", {"mode": "zero"})
            entry["status"], entry["note"] = warm_verdict(warm["mode"], warm.get("allow", 0), rc, parsed["spawns"], parsed["summaries"], seconds, warm.get("maxSeconds"), self.args.warm_slack)
            entry["warm"] = {"mode": warm["mode"], "allow": warm.get("allow", 0) + (self.args.warm_slack if warm["mode"] == "allow" else 0), "maxSeconds": warm.get("maxSeconds")}
        else:
            entry["status"], entry["note"] = plain_verdict(rc, parsed)
        if step.get("ports") and phase == "e2e":
            time.sleep(3)
        ports = [p for p in step.get("ports", []) if port_open(p)]
        if phase == "e2e" and ports:
            entry["note"] = (entry["note"] + "; " if entry["note"] else "") + "ports still answering after the step: %s" % ports
            if entry["status"] == "PASS":
                entry["status"] = "WARN"
        self.record(entry)
        self.log("%s: %s rc=%s %s %s" % (key, entry["status"], rc, fmt_seconds(seconds), entry["note"]))
        return entry

    def execute_retrying(self, product, phase, step, index) -> dict:
        """🔁 Runs a step again (at most `staleRetries` times) while it exits 75 because a peer moved a declared format stamp under it (the process ran an outdated implementation: a retry, not a failure); a warm step that passed but did work is settled by `warmSettleRuns` further runs, because inputs edited between the cold and the warm run are rebuilt once and cached afterwards."""
        entry = self.execute(product, phase, step, index)
        for attempt in range(1, self.th.get("staleRetries", 2) + 1):
            if entry["rc"] != STALE_RC or self.cancel.is_set() or STALE_TEXT not in read_text(self.root / entry["log"]):
                break
            self.log("%s:%s:%s: outdated implementation (a peer moved a format stamp); retry %d" % (product, phase, step["id"], attempt))
            entry = self.execute(product, phase, step, index)
            entry["note"] = (entry["note"] + "; " if entry["note"] else "") + "retried %d time(s) after an outdated-implementation exit" % attempt
            self.record(entry)
        for settle in range(1, self.th.get("warmSettleRuns", 1) + 1):
            if phase != "warm" or entry["status"] != "FAIL" or entry["rc"] != 0 or self.cancel.is_set():
                break
            first = entry["note"]
            self.log("%s:%s:%s: warm run executed work (%s): the inputs moved after the cold run; settle run %d decides" % (product, phase, step["id"], first, settle))
            entry = self.execute(product, phase, step, index)
            entry["note"] = (entry["note"] + "; " if entry["note"] else "") + "settle run %d after a first warm run with work (%s)" % (settle, first)
            self.record(entry)
        return entry

    def forget_unfinished(self, product: str, phases: list) -> None:
        """♻️ Continuing a run id retries what did not pass: the records of failed, cancelled, skipped and interrupted steps of the selected phases are dropped so they neither skip their group nor show stale verdicts."""
        with self.lock:
            self.state["steps"] = [r for r in self.state["steps"] if not (r["product"] == product and r["phase"] in phases and r["status"] in ("FAIL", "CANCELLED", "SKIP-FAILED", "RUNNING"))]
        self.save()

    def pipeline(self, product: str, phases: list) -> None:
        """🚦 Runs the selected phases of one product sequentially; failed groups skip their dependants."""
        self.forget_unfinished(product, phases)
        delay = self.plan["products"][product]["order"] - 1
        if self.args.stagger and delay > 0:
            for _ in range(int(delay * self.th["staggerSeconds"])):
                if self.cancel.is_set():
                    return
                time.sleep(1)
        for phase in phases:
            for index, step in enumerate(self.steps_of(product, phase), start=1):
                if self.cancel.is_set():
                    return
                reason = None
                previous = self.find_step_records(product, phase, step["id"])
                if previous and previous[0]["status"] in ("PASS", "WARN") and not self.args.rerun:
                    self.log("%s:%s:%s: already %s in this run; skipped (use --rerun)" % (product, phase, step["id"], previous[0]["status"]))
                    continue
                if self.group_failed(product, step["group"]):
                    reason = "an earlier step of group '%s' failed" % step["group"]
                for needed in step.get("needs", []):
                    if self.group_failed(product, needed):
                        reason = "group '%s' it depends on failed" % needed
                if phase == "warm":
                    cold = self.find_step_records(product, "cold", step["id"])
                    if cold and cold[0]["status"] not in ("PASS", "WARN"):
                        reason = "the cold run of this step did not pass"
                if reason:
                    self.record({"product": product, "phase": phase, "id": step["id"], "group": step["group"], "cmd": self.command_for(step), "start": now_iso(), "end": now_iso(), "seconds": 0, "rc": None, "status": "SKIP-FAILED", "note": reason, "log": "", "bazel": {}, "slots": [], "waits": [], "evidence": [], "tests": {}})
                    self.log("%s:%s:%s: skipped, %s" % (product, phase, step["id"], reason))
                    continue
                self.execute_retrying(product, phase, step, index)

    def run_pipelines(self, products: list, phases: list) -> None:
        """🧵 Product pipelines in parallel, longest first, bounded by --parallel."""
        ordered = sorted(products, key=lambda p: self.plan["products"][p]["order"])
        workers = max(1, min(self.args.parallel, len(ordered)))
        self.log("pipelines: %s phases=%s parallel=%d" % (ordered, phases, workers))
        with concurrent.futures.ThreadPoolExecutor(max_workers=workers) as pool:
            futures = {pool.submit(self.pipeline, p, phases): p for p in ordered}
            for future in concurrent.futures.as_completed(futures):
                error = future.exception()
                if error is not None:
                    self.log("pipeline %s crashed: %r" % (futures[future], error))
                    self.record({"product": futures[future], "phase": "pipeline", "id": "crash", "group": "crash", "cmd": [], "start": now_iso(), "end": now_iso(), "seconds": 0, "rc": 1, "status": "FAIL", "note": repr(error), "log": "", "bazel": {}, "slots": [], "waits": [], "evidence": [], "tests": {}})

    def sample(self) -> None:
        """🌡️ Updates the resource extremes."""
        load = load_average()
        total, available = memory_gb()
        free = free_disk_gb(self.root)
        with self.lock:
            samples = self.state["samples"]
            if load is not None:
                samples["maxLoad"] = max(samples["maxLoad"] or 0, round(load, 1))
            if available is not None:
                samples["minAvailableGb"] = round(min(samples["minAvailableGb"] if samples["minAvailableGb"] is not None else available, available), 1)
            samples["minFreeDiskGb"] = round(min(samples["minFreeDiskGb"] if samples["minFreeDiskGb"] is not None else free, free), 1)
        if free < self.th["abortFreeDiskGb"] and not self.cancel.is_set():
            self.request_cancel("free disk %.1f GB below the abort threshold %s GB" % (free, self.th["abortFreeDiskGb"]))

    def request_cancel(self, reason: str) -> None:
        """🧯 Cancels every running step and stops scheduling."""
        if not self.cancel.is_set():
            self.cancel_reason = reason
            self.cancel.set()
            self.log("cancel requested: " + reason)

    def snapshot(self, label: str, wait: float = 0) -> None:
        """📸 Stores raw cache, capacity and lease status in a background thread (never blocks the pipelines); `wait` seconds to join."""
        if self.args.no_snapshots:
            return
        def work():
            if not self.snapshot_busy.acquire(wait > 0):
                return
            try:
                files = {}
                for kind, argv in (("capacity", ["bun", "repo", "capacity", "status"]), ("leases", ["bun", "repo", "leases", "status"]), ("cache", ["bun", "repo", "--tool=%s" % self.plan["tool"], "cache", "status", "--tasks"])):
                    rc, out = capture(argv, self.th["snapshotTimeoutSeconds"], cwd=str(self.root), env=self.child_env(""))
                    path = self.dir / ("snapshot-%s-%s.txt" % (label, kind))
                    write_atomic(path, "# rc %s\n%s" % (rc, out))
                    files[kind] = str(path.name)
                with self.lock:
                    self.state["snapshots"].append({"label": label, "time": now_iso(), "files": files})
                self.save()
            finally:
                self.snapshot_busy.release()
        thread = threading.Thread(target=work, daemon=True)
        thread.start()
        if wait:
            thread.join(wait)

    def monitor(self, stop: threading.Event) -> None:
        """💓 Heartbeat, resource samples, CANCEL file, periodic snapshots."""
        last_snapshot = time.time()
        counter = 0
        while not stop.wait(self.th["heartbeatSeconds"] if not self.args.fast else 1):
            if (self.dir / "CANCEL").exists():
                self.request_cancel("CANCEL file")
            if time.time() > self.deadline:
                self.request_cancel("run deadline of %s h reached" % self.th["deadlineHours"])
            self.sample()
            with self.lock:
                rows = [(key, info) for key, info in self.running.items()]
            parts = []
            for key, info in rows:
                tail = [l for l in strip_ansi(read_tail(info["log"])).splitlines() if l.strip()]
                parts.append("%s %s: %s" % (key, fmt_seconds(time.time() - info["started"]), (tail[-1].strip()[:110] if tail else "")))
            load = load_average()
            self.log("heartbeat load=%s running=%d %s" % ("%.1f" % load if load is not None else "?", len(rows), " | ".join(parts)))
            counter += 1
            if time.time() - last_snapshot > self.th["snapshotSeconds"]:
                last_snapshot = time.time()
                self.snapshot("t%d" % counter)
            self.save()
            self.write_report()

    # ---- preflight --------------------------------------------------------------------------

    def check(self, checks: list, status: str, name: str, detail: str) -> None:
        """✅ Appends one preflight row."""
        checks.append({"status": status, "name": name, "detail": detail})

    def tool_cache(self, name: str) -> Path:
        """🗄️ The repository tool cache directory of a tool (emoji path parts found by glob, not typed)."""
        found = sorted(self.root.glob(".*semio/*repo/*cache/tools/" + name))
        return found[0] if found else self.root / ".semio-missing" / name

    def browsers_check(self, checks: list) -> None:
        """🎭 Playwright browsers named by the installed playwright-core manifest, pinned to the plan."""
        cli = self.root / "node_modules" / "playwright" / "cli.js"
        if not cli.exists():
            return self.check(checks, "FAIL", "playwright", "node_modules/playwright/cli.js missing (dependency sync not green)")
        core = Path(os.path.realpath(str(self.root / "node_modules" / "playwright"))).parent / "playwright-core" / "browsers.json"
        try:
            manifest = json.loads(read_text(core))
        except ValueError:
            return self.check(checks, "FAIL", "playwright", "browsers.json unreadable at %s" % core)
        cache = self.tool_cache("ms-playwright")
        pins = {"chromium": self.plan["playwright"]["expectedChromium"], "chromium-headless-shell": self.plan["playwright"]["expectedChromium"], "ffmpeg": self.plan["playwright"]["expectedFfmpeg"]}
        for name, pin in pins.items():
            revision = next((b["revision"] for b in manifest["browsers"] if b["name"] == name), None)
            directory = cache / ("%s-%s" % (name.replace("-", "_"), revision))
            complete = (directory / "INSTALLATION_COMPLETE").exists()
            if revision != pin:
                self.check(checks, "FAIL", "playwright " + name, "manifest revision %s differs from the pinned %s: browsers must be downloaded again" % (revision, pin))
            elif not complete:
                self.check(checks, "FAIL", "playwright " + name, "%s not installed (download ~300 MB needs the user's go: bun repo run workspace:deps-browsers)" % directory.name)
            else:
                self.check(checks, "OK", "playwright " + name, "revision %s installed" % revision)

    def tectonic_check(self, checks: list) -> None:
        """📄 Pinned Tectonic binary and TeX bundle."""
        pin = self.plan["tectonic"]["version"]
        base = self.tool_cache("tectonic")
        found = sorted(self.root.glob("*framework/*products/*print/*modules/*tectonic-template-compilation/*toolchain/*.json"))
        toolchain = found[0] if found else self.root / "tectonic-toolchain-pin-not-found.json"
        try:
            pinned = json.loads(read_text(toolchain))
        except ValueError:
            return self.check(checks, "FAIL", "tectonic", "toolchain pin file unreadable: %s" % toolchain)
        if pinned.get("version") != pin:
            return self.check(checks, "FAIL", "tectonic", "plan pins %s, toolchain file pins %s" % (pin, pinned.get("version")))
        system = {"Darwin": "darwin", "Linux": "linux", "Windows": "win32"}.get(platform.system(), "")
        machine = platform.machine().lower()
        arch = "arm64" if machine in ("arm64", "aarch64") else "x64" if machine in ("x86_64", "amd64") else machine
        row = next((p for p in pinned["platforms"] if p["platform"] == system and p["architecture"] == arch), None)
        if row is None:
            return self.check(checks, "FAIL", "tectonic", "no pinned build for %s/%s" % (system, arch))
        directory = base / pin / row["target"]
        binary = directory / ("tectonic.exe" if system == "win32" else "tectonic")
        record = {}
        try:
            record = json.loads(read_text(directory / ".toolchain.json"))
        except ValueError:
            pass
        if not binary.exists():
            self.check(checks, "FAIL", "tectonic", "%s missing (provisioned by deps-tectonic)" % binary)
        elif record.get("archiveSha256") != row["sha256"]:
            self.check(checks, "FAIL", "tectonic", "installed archive hash differs from the pin")
        else:
            import hashlib
            digest = hashlib.sha256(binary.read_bytes()).hexdigest()
            expected = list(record.get("files", {}).values())
            self.check(checks, "OK" if digest in expected else "FAIL", "tectonic binary", "%s %s sha256 %s" % (row["target"], pin, "matches the recorded extraction" if digest in expected else "DIFFERS from the recorded extraction"))
        bundles = [d for d in (base / "bundles").glob("*") if d.is_dir() and any(d.iterdir())] if (base / "bundles").is_dir() else []
        self.check(checks, "OK" if bundles else "FAIL", "tex bundle", "%d populated bundle director%s" % (len(bundles), "y" if len(bundles) == 1 else "ies"))

    def markers_check(self, checks: list, product: str) -> None:
        """🧷 Informational: whether the staged sets the warm assertions rely on are in the tree."""
        for marker in self.plan["products"][product].get("appliedMarkers", []):
            if "exists" in marker:
                ok = (self.root / marker["exists"]).exists()
            else:
                try:
                    manifest = json.loads(read_text(self.root / marker["manifest"]))
                    ok = bool(manifest["targets"][marker["target"]].get("cache")) == marker["cache"]
                except (ValueError, KeyError):
                    ok = False
            self.check(checks, "OK" if ok else "WARN", "applied: " + marker["id"], "present" if ok else "NOT in the tree: warm assertions for %s will probably fail" % product)

    def verify_targets(self, checks: list, products: list) -> None:
        """🎯 Every project and target the plan runs must exist in the published graph: one `show targets --json` with SEMIO_BUILD_GRAPH_POLICY=current (the CLI caller parser refuses the flag form; the env form never waits for a rebuild; it queues at the capacity gate, 5 GB). A failed read is a WARN, a listed generation without a name is a FAIL."""
        wanted = {}
        for product in products:
            spec = self.plan["products"][product]
            names = list(spec.get("verifyTargets", []))
            for phase in ("cold", "e2e", "probe"):
                for step in spec["steps"][phase]:
                    if step["kind"] == "repo" and len(step["args"]) > 1 and step["args"][0] == "run" and ":" in step["args"][1]:
                        names.append(step["args"][1])
            for name in names:
                project, target = name.rsplit(":", 1)
                wanted.setdefault(project, set()).add(target)
        cli = ["bun", "repo", "--tool=%s" % self.plan["tool"], "show"]
        env = self.child_env("")
        env["SEMIO_BUILD_GRAPH_POLICY"] = "current"
        token = r"(?<![\w-])%s(?![\w-])"

        def candidates(text):
            try:
                data = json.loads(text)
            except ValueError:
                return text.splitlines()
            found = []
            def walk(node):
                if isinstance(node, str):
                    found.append(node)
                elif isinstance(node, dict):
                    for key, value in node.items():
                        found.append(str(key))
                        walk(value)
                elif isinstance(node, list):
                    for value in node:
                        walk(value)
            walk(data)
            return found

        def json_part(text):
            lines = [l for l in text.splitlines() if not l.startswith("[script]") and not l.startswith("[capacity]") and not l.startswith("$ ")]
            starts = [i for i, l in enumerate(lines) if l in ("[", "{") or l.startswith('["') or l.startswith('{"')]
            ends = [i for i, l in enumerate(lines) if l in ("]", "}")]
            if starts and ends and ends[-1] >= starts[0]:
                return "\n".join(lines[starts[0]:ends[-1] + 1])
            return "\n".join(lines)

        for _ in range(3):
            rc, out = capture(cli + ["targets", "--json"], self.th["verifyTargetSeconds"], cwd=str(self.root), env=env, machine=True)
            if self.cancel.is_set() or (rc == 0 and candidates(json_part(strip_ansi(out)))):
                break
        if rc != 0:
            tail = " ".join(l.strip() for l in strip_ansi(out).splitlines() if l.strip() and not l.startswith("[script]") and not l.startswith("[capacity]"))[-240:]
            return self.check(checks, "WARN", "graph targets", "could not read the published generation (exit %d: %s); target names not verified, the run itself will show a missing name" % (rc, tail))
        rows = candidates(json_part(strip_ansi(out)))
        if rows and not any(":" in r for r in rows):
            for _ in range(3):
                rc, out = capture(cli + ["projects", "--json"], self.th["verifyTargetSeconds"], cwd=str(self.root), env=env, machine=True)
                if self.cancel.is_set() or (rc == 0 and candidates(json_part(strip_ansi(out)))):
                    break
            if rc != 0:
                return self.check(checks, "WARN", "graph targets", "could not read the published projects (exit %d); target names not verified" % rc)
            projects, names = set(candidates(json_part(strip_ansi(out)))), set(rows)
            missing = sorted("%s:%s" % (project, target) for project, targets in wanted.items() for target in targets if project not in projects or target not in names)
            if missing:
                return self.check(checks, "FAIL", "graph targets", "%d of %d target(s) missing from the generation: %s" % (len(missing), sum(len(v) for v in wanted.values()), ", ".join(missing)))
            return self.check(checks, "OK", "graph targets", "%d project(s) and their %d target name(s) found in lean `show projects|targets --json` (SEMIO_BUILD_GRAPH_POLICY=current)" % (len(wanted), sum(len(v) for v in wanted.values())))
        missing = sorted("%s:%s" % (project, target) for project, targets in wanted.items() for target in targets if not any(re.search(token % re.escape(project), r) and re.search(token % re.escape(target), r) for r in rows))
        if missing:
            return self.check(checks, "FAIL", "graph targets", "%d of %d target(s) missing from the generation: %s" % (len(missing), sum(len(v) for v in wanted.values()), ", ".join(missing)))
        self.check(checks, "OK", "graph targets", "%d target(s) of %d project(s) found in one `show targets --json` (SEMIO_BUILD_GRAPH_POLICY=current)" % (sum(len(v) for v in wanted.values()), len(wanted)))

    def runtime_checks(self, checks: list) -> None:
        """📌 The pinned Bun (packageManager) is the one that runs: asked of the repo launcher (it owns the provisioning), then proved by a host Bun that starts in the checkout (the bunfig guard re-runs it as the pinned one)."""
        try:
            package = json.loads(read_text(self.root / "package.json"))
        except ValueError:
            package = {}
        wanted = str(package.get("packageManager", "")).split("@")[-1]
        launcher = shutil.which("repo")
        if not launcher:
            self.check(checks, "FAIL", "pinned bun", "the repo launcher is not on PATH; install it once (it provisions bun %s): bash 🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🐚️.sh" % (wanted or "-"))
            return
        rc, out = capture([launcher, "--version", "--lang=en"], 60, cwd=str(self.root), env=self.child_env(""))
        found = re.search(r"^runtime bun (\S+): (ready, (.+)|not provisioned yet)", out, re.M)
        if rc != 0 or not found:
            self.check(checks, "FAIL", "pinned bun", "`repo --version` exit %d: %s" % (rc, out.strip()[:160]))
            return
        pinned = found.group(1)
        if wanted and pinned != wanted:
            self.check(checks, "FAIL", "pinned bun", "launcher registry pins %s, packageManager %s" % (pinned, wanted))
        elif found.group(3):
            self.check(checks, "OK", "pinned bun", "%s provisioned: %s" % (pinned, found.group(3).strip()))
        else:
            self.check(checks, "WARN", "pinned bun", "%s is not provisioned yet; the first repo command downloads it (SHA-256 pinned)" % pinned)
        host = shutil.which("bun")
        if host:
            rc, out = capture([host, "-e", "console.log(process.versions.bun)"], 60, cwd=str(self.root), env=self.child_env(""))
            ran = out.strip().splitlines()[-1] if out.strip() else ""
            started = capture([host, "--version"], 20)[1].strip()
            self.check(checks, "OK" if rc == 0 and ran == pinned else "FAIL", "bun runs pinned", "bun on PATH is %s (%s) and runs the checkout as %s (pinned %s)" % (started, host, ran or "nothing: exit %d" % rc, pinned))

    def preflight(self, products: list, phases: list) -> dict:
        """🛫 Prerequisite checks: tools, pins, ports, disk, memory, load, capacity, slots, applied sets."""
        checks = []
        mutating = [p for p in phases if p in PHASE_ORDER]
        needs = set()
        for product in products:
            needs.update(self.plan["products"][product]["needs"])
        for tool in ("python3", "bun", "node", "git"):
            found = shutil.which(tool) or (sys.executable if tool == "python3" else None)
            self.check(checks, "OK" if found else ("FAIL" if tool in ("bun",) or (tool == "node" and "node" in needs) else "WARN"), "tool " + tool, found or "not on PATH")
        self.runtime_checks(checks)
        version_file = self.root / ".bazelversion"
        wanted_bazel = read_text(version_file).strip()
        binaries = sorted((self.root / ".cache" / "b" / "t" / "m" / "bazel" / "bin").glob("*/bazel*")) if (self.root / ".cache" / "b" / "t" / "m" / "bazel" / "bin").is_dir() else []
        if "bazel" in needs:
            if binaries:
                rc, out = capture([str(binaries[0]), "--version"], 60, cwd=str(self.root))
                self.check(checks, "OK" if wanted_bazel in out else "WARN", "bazel binary", "%s (pinned %s) %s" % (binaries[0].parent.name, wanted_bazel, out.strip()[:60]))
            else:
                self.check(checks, "WARN", "bazel binary", "not yet provisioned (bun repo tools setup bazel); first run will fetch it")
        if "playwright" in needs:
            self.browsers_check(checks)
        if "tectonic" in needs:
            self.tectonic_check(checks)
        minimum = self.th["minFreeDiskGbCold"] if ("cold" in mutating or not mutating) else self.th["minFreeDiskGbWarm"]
        free = free_disk_gb(self.root)
        self.check(checks, "OK" if free >= minimum else "FAIL", "disk", "%.0f GB free (need %s)" % (free, minimum))
        total, available = memory_gb()
        if available is None:
            self.check(checks, "WARN", "memory", "not measurable on %s; the host capacity gate decides" % platform.system())
        else:
            low = available < self.th["failAvailableMemoryGb"]
            self.check(checks, "FAIL" if low else "OK" if available >= self.th["minAvailableMemoryGb"] else "WARN", "memory", "%.1f GB available of %.1f GB (the live capacity gate queues heavy commands below its floor; FAIL only under %s GB)" % (available, total, self.th["failAvailableMemoryGb"]))
        cores = os.cpu_count() or 1
        load = load_average()
        if load is not None:
            self.check(checks, "INFO" if load > self.th["loadWarnPerCore"] * cores else "OK", "load", "%.1f on %d cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants)" % (load, cores))
        slots = lane_slots(int((total or 0) * 1024), cores) if total else 1
        override = os.environ.get("SEMIO_BAZEL_SERVERS")
        if override and override.isdigit():
            slots = int(override)
        self.parallel_hint = max(1, min(len(products), slots))
        self.check(checks, "OK" if slots >= len(products) else "WARN", "bazel build slots", "%d slot(s) from the host budget for %d product(s)%s" % (slots, len(products), "" if slots >= len(products) else "; products will queue fairly for a slot"))
        for product in products:
            used = [p for p in self.plan["products"][product]["ports"] if port_open(p)]
            self.check(checks, "FAIL" if used and "e2e" in mutating else "WARN" if used else "OK", "ports " + product, ("in use: %s (stop the other serve; lsof -nP -iTCP:<port> -sTCP:LISTEN)" % used) if used else "%s free" % self.plan["products"][product]["ports"])
            self.markers_check(checks, product)
        for variable, why in (("SEMIO_HOST_CAPACITY", "host capacity governor"), ("SEMIO_BAZEL_REMOTE_CACHE", "remote cache layer")):
            if os.environ.get(variable):
                self.check(checks, "WARN" if variable == "SEMIO_HOST_CAPACITY" else "INFO", "env " + variable, "%s=%s overrides the %s" % (variable, os.environ[variable], why))
        if Path("/tmp/semio-wasm-build.lock").exists():
            self.check(checks, "WARN", "fleet wasm mutex", "/tmp/semio-wasm-build.lock exists: a peer holds the legacy wasm mutex")
        if os.name == "posix":
            rc, out = capture(["ps", "-axo", "pid=,etime=,command="], 20)
            mine = str(os.getpid())
            rows = [l for l in out.splitlines() if re.search(r"\b(cargo|rustc|wasm-opt|jco|vite|tectonic)\b|bazel|playwright|chrom", l) and not re.search(r"\s(/bin/)?(zsh|bash|sh) -c|ps -axo|\.app/Contents|%s" % mine, l)]
            self.check(checks, "INFO", "foreign heavy processes", "%d running (shared fleet; the governor and the slots arbitrate)%s" % (len(rows), ": " + " || ".join(r.strip()[:90] for r in rows[:3]) if rows else ""))
        if shutil.which("bun") and not self.args.no_cli_probe:
            rc, out = capture(["bun", "repo", "--help"], self.th["cliProbeSeconds"], cwd=str(self.root), env=self.child_env(""))
            self.check(checks, "OK" if rc == 0 else "FAIL", "bun repo --help", "exit %d" % rc)
            rc, out = capture(["bun", "repo", "capacity", "status"], self.th["cliProbeSeconds"], cwd=str(self.root), env=self.child_env(""))
            first = [l for l in strip_ansi(out).splitlines() if l.startswith("[capacity]")][:1]
            pressure = re.search(r"pressure (\w+)", first[0]) if first else None
            hot = bool(pressure and pressure.group(1) in ("warn", "critical"))
            self.check(checks, "OK" if rc == 0 and not hot else "WARN", "bun repo capacity status", (first[0][:200] if first else "exit %d (governor not applied or queued)" % rc) + ("; steps will queue at the gate (progress lines are recorded as waits)" if hot else ""))
        if shutil.which("bun") and (self.args.verify_targets or (mutating and not self.args.skip_preflight and not self.args.no_verify_targets)) and not self.args.no_cli_probe:
            self.verify_targets(checks, products)
        verdict = "FAIL" if any(c["status"] == "FAIL" for c in checks) else "WARN" if any(c["status"] == "WARN" for c in checks) else "OK"
        return {"time": now_iso(), "verdict": verdict, "checks": checks}

    # ---- report -----------------------------------------------------------------------------

    def render_report(self) -> str:
        """📝 Markdown report of the state: matrix, per-product tables, warm assertions, cache stats, failures."""
        state = self.state
        rows = state["steps"]
        statuses = {}
        for row in rows:
            statuses[row["status"]] = statuses.get(row["status"], 0) + 1
        lines = ["# \U0001F9ED e2e-orch report %s" % state["runId"], ""]
        lines += ["Created %s, written %s, orchestrator pid %s. Argv: `%s`." % (state["created"], now_iso(), state.get("pid"), " ".join(state["argv"])), ""]
        lines += ["Host: %s %s, %s cores, parallel %s, Bazel tool `%s`. Counts: %s. Cancelled: %s%s." % (platform.system(), platform.release(), os.cpu_count(), self.args.parallel, self.plan["tool"], ", ".join("%s %d" % (k, v) for k, v in sorted(statuses.items())) or "no steps", state["cancelled"], (" (%s)" % self.cancel_reason) if self.cancel_reason else ""), ""]
        pre = state.get("preflight")
        lines += ["## Prerequisites", ""]
        if pre:
            lines += ["Verdict **%s** at %s." % (pre["verdict"], pre["time"]), "", "| status | check | detail |", "|---|---|---|"]
            lines += ["| %s | %s | %s |" % (c["status"], c["name"], c["detail"].replace("|", "/")) for c in pre["checks"]]
        else:
            lines += ["not run in this invocation."]
        lines += ["", "## Matrix", "", "| product | " + " | ".join(PHASE_ORDER) + " |", "|---|" + "---|" * len(PHASE_ORDER)]
        for product in sorted(self.plan["products"], key=lambda p: self.plan["products"][p]["order"]):
            cells = []
            for phase in PHASE_ORDER:
                mine = [r for r in rows if r["product"] == product and r["phase"] == phase]
                if not mine:
                    cells.append("-")
                    continue
                worst = "FAIL" if any(r["status"] in ("FAIL", "CANCELLED") for r in mine) else "SKIP" if all(r["status"] == "SKIP-FAILED" for r in mine) else "WARN" if any(r["status"] in ("WARN", "SKIP-FAILED") for r in mine) else "PASS" if all(r["status"] == "PASS" for r in mine) else "RUNNING"
                cells.append("%s %s" % (worst, fmt_seconds(sum(r["seconds"] or 0 for r in mine))))
            lines.append("| %s | %s |" % (product, " | ".join(cells)))
        for product in sorted(self.plan["products"], key=lambda p: self.plan["products"][p]["order"]):
            mine = [r for r in rows if r["product"] == product]
            if not mine:
                continue
            lines += ["", "## %s (`%s`)" % (self.plan["products"][product]["label"], self.plan["products"][product]["package"]), "", "| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |", "|---|---|---|---|---|---|---|---|---|---|---|---|"]
            for phase in PHASE_ORDER + ["pipeline"]:
                for row in [r for r in mine if r["phase"] == phase]:
                    b = row.get("bazel") or {}
                    lines.append("| %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %d | %s |" % (phase, row["id"], row["status"], row["rc"], fmt_seconds(row["seconds"]), b.get("processes", "-"), b.get("spawns", "-"), b.get("cacheHits", "-"), b.get("internal", "-"), ",".join(row.get("slots") or []) or "-", len(row.get("waits") or []), (row.get("note") or "").replace("|", "/")))
        warm = [r for r in rows if r["phase"] == "warm"]
        lines += ["", "## Warm assertions (Bazel cache hits)", ""]
        if warm:
            lines += ["| product | step | spawns | allowed | internal | cache hits | time | expected within | verdict |", "|---|---|---|---|---|---|---|---|---|"]
            for row in warm:
                b, w = row.get("bazel") or {}, row.get("warm") or {}
                lines.append("| %s | %s | %s | %s | %s | %s | %s | %s | %s %s |" % (row["product"], row["id"], b.get("spawns", "-"), w.get("allow", 0) if w.get("mode") != "zero" else 0, b.get("internal", "-"), b.get("cacheHits", "-"), fmt_seconds(row["seconds"]), fmt_seconds(w.get("maxSeconds")), row["status"], (row.get("note") or "").replace("|", "/")))
                for evidence in row.get("evidence") or []:
                    lines.append("|  |  |  |  |  |  |  |  | evidence: `%s` |" % evidence.replace("|", "/"))
        else:
            lines += ["no warm step recorded."]
        waits = [(r["product"], r["id"], w) for r in rows for w in (r.get("waits") or [])]
        lines += ["", "## Queueing (slots, capacity)", ""]
        lines += ["- %s %s: `%s`" % item for item in waits[:20]] or ["no waiting lines recorded."]
        lines += ["", "## Cache and capacity statistics", ""]
        snaps = state.get("snapshots") or []
        if len(snaps) >= 2:
            first, last = snaps[0], snaps[-1]
            for kind in ("cache", "capacity", "leases"):
                before = read_text(self.dir / first["files"].get(kind, "x")) if first["files"].get(kind) else ""
                after = read_text(self.dir / last["files"].get(kind, "x")) if last["files"].get(kind) else ""
                delta = counter_delta(before, after)
                lines.append("- %s: %d changed counter(s) between `%s` and `%s`" % (kind, len(delta), first["label"], last["label"]))
                lines += ["    - `%s`: %s -> %s (%+g)" % row for row in delta[:25]]
        else:
            lines += ["fewer than two snapshots (use the `start` and `end` snapshot files under the run directory when present)."]
        lines += ["", "Snapshots: " + (", ".join("%s@%s" % (s["label"], s["time"]) for s in snaps) or "none") + ". Raw files: `%s`." % self.rel(self.dir)]
        samples = state["samples"]
        lines += ["", "## Resources", "", "Max load %s, min available memory %s GB, min free disk %s GB." % (samples["maxLoad"], samples["minAvailableGb"], samples["minFreeDiskGb"])]
        failed = [r for r in rows if r["status"] in ("FAIL", "CANCELLED", "WARN")]
        lines += ["", "## Failures and warnings", ""]
        if not failed:
            lines += ["none."]
        lines.append("")
        for row in failed:
            lines += ["### %s %s %s: %s" % (row["product"], row["phase"], row["id"], row["status"]), "", "- command: `%s`" % " ".join(row["cmd"]), "- note: %s" % (row.get("note") or "-"), "- log: `%s`" % row["log"]]
            tail = [l for l in strip_ansi(read_tail(self.ticket / row["log"], 6000) if row["log"] else "").splitlines() if l.strip()][-25:]
            if tail:
                lines += ["", "```"] + [t[:240] for t in tail] + ["```"]
            lines.append("")
        lines += ["## Re-run", "", "Continue this run: `run.sh all --run-id %s --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md." % state["runId"]]
        return "\n".join(lines) + "\n"

    def rel(self, path: Path) -> str:
        """🧭 Path relative to the ticket when possible."""
        try:
            return str(Path(path).relative_to(self.ticket))
        except ValueError:
            return str(path)

    def write_report(self) -> None:
        """🖨️ Writes the report next to the ticket's other results."""
        write_atomic(self.report_path, self.render_report())


def acquire_lock(base: Path, run_id: str, force: bool) -> bool:
    """🔐 One mutating orchestrator at a time per out directory (ports, slots and lanes are shared)."""
    path = base / "ORCH.lock"
    try:
        held = json.loads(read_text(path)) if path.exists() else None
    except ValueError:
        held = None
    if held and pid_alive(int(held.get("pid", 0))) and int(held["pid"]) != os.getpid() and not force:
        print("another orchestrator is running (pid %s, run %s); use `status`/`cancel`, or --force-lock" % (held["pid"], held.get("runId")), file=sys.stderr)
        return False
    write_atomic(path, json.dumps({"pid": os.getpid(), "runId": run_id, "started": now_iso()}))
    return True


def release_lock(base: Path) -> None:
    """🔓 Releases the lock if it is ours."""
    path = base / "ORCH.lock"
    try:
        if int(json.loads(read_text(path)).get("pid", 0)) == os.getpid():
            path.unlink()
    except (ValueError, OSError):
        pass


def build_parser() -> argparse.ArgumentParser:
    """⌨️ Command line."""
    parser = argparse.ArgumentParser(prog="run.sh", description="Orchestrated end-to-end driver for course, play and demo on Bazel.")
    parser.add_argument("target", help="course | play | demo | all | status | cancel")
    parser.add_argument("--phase", default="all", help="comma list of preflight,cold,warm,e2e,probe,report or all (= preflight,cold,warm,e2e,report; probe edits sources and is opt-in)")
    parser.add_argument("--go", action="store_true", help="the coordinator's E2E GO; also E2E_GO=1. Required for cold, warm, e2e and probe")
    parser.add_argument("--run-id", default=None, help="run identifier; reuse it to continue a run phase by phase")
    parser.add_argument("--parallel", type=int, default=0, help="products in parallel (default: min(products, derived Bazel build slots))")
    parser.add_argument("--rerun", action="store_true", help="re-run steps that already passed in this run id")
    parser.add_argument("--dry-run", action="store_true", help="print the plan, run nothing")
    parser.add_argument("--skip-preflight", action="store_true")
    parser.add_argument("--no-cli-probe", action="store_true", help="preflight: skip the bun repo --help and capacity status probes")
    parser.add_argument("--verify-targets", action="store_true", help="preflight: check every project and target name in the published graph (show project --graph-policy=current); on by default before heavy phases")
    parser.add_argument("--no-verify-targets", action="store_true")
    parser.add_argument("--no-snapshots", action="store_true", help="skip cache/capacity/lease snapshots")
    parser.add_argument("--no-stagger", dest="stagger", action="store_false", help="start all products at once")
    parser.add_argument("--force-lock", action="store_true")
    parser.add_argument("--warm-slack", type=int, default=0, help="extra uncached probes tolerated per warm step that allows some")
    parser.add_argument("--course-mode", choices=["release", "dev"], default="release", help="configuration of the course and proctor build steps")
    parser.add_argument("--course-topology", choices=["dev", "rehearsal", "both"], default=None, help="narrow the course e2e gate to one topology; absent, the gate runs the topologies its target declares (both)")
    parser.add_argument("--plan", default=str(HERE / "plan.json"))
    parser.add_argument("--ticket", default=str(TICKET))
    parser.add_argument("--root", default=None)
    parser.add_argument("--out", default=None, help="base directory of run directories")
    parser.add_argument("--report-dir", default=None)
    parser.add_argument("--fast", action="store_true", help="selftest: 1 s heartbeat")
    return parser


def expand_phases(text: str) -> list:
    """🗺️ Phase list in execution order."""
    wanted = [p.strip() for p in text.split(",") if p.strip()]
    if "all" in wanted:
        wanted = ["preflight", "cold", "warm", "e2e", "report"] + [p for p in wanted if p == "probe"]
    unknown = [p for p in wanted if p not in ["preflight", "cold", "warm", "e2e", "probe", "report"]]
    if unknown:
        raise SystemExit("unknown phase(s): %s" % unknown)
    return [p for p in ["preflight", "cold", "warm", "e2e", "probe", "report"] if p in wanted]


def print_plan(orch: Orchestrator, products: list, phases: list) -> None:
    """🗒️ Dry run output."""
    print("run id %s; tool %s; phases %s; parallel %s" % (orch.run_id, orch.plan["tool"], phases, orch.args.parallel or "auto"))
    for product in sorted(products, key=lambda p: orch.plan["products"][p]["order"]):
        spec = orch.plan["products"][product]
        print("\n%s  lane=%s  ports=%s  needs=%s" % (product, spec["cargoLane"], spec["ports"], spec["needs"]))
        for phase in [p for p in PHASE_ORDER if p in phases]:
            for step in orch.steps_of(product, phase):
                warm = ""
                if phase == "warm":
                    w = step.get("warm", {"mode": "zero"})
                    warm = "  assert spawns<=%s within %s" % (0 if w["mode"] == "zero" else w.get("allow", 0) + orch.args.warm_slack, fmt_seconds(w.get("maxSeconds")))
                print("  %-5s %-16s group=%-8s timeout=%-6s%s\n        %s" % (phase, step["id"], step["group"], fmt_seconds(step["timeoutSeconds"]), warm, " ".join(orch.command_for(step))))


def status_command(args, plan) -> int:
    """📡 Prints the progress of a run from its state file."""
    base = Path(args.out).resolve() if args.out else Path(args.ticket).resolve() / GENERATED / "e2e-orch"
    run_id = args.run_id
    if not run_id:
        runs = sorted(p.name for p in base.glob("*") if (p / "state.json").exists())
        if not runs:
            print("no runs under %s" % base)
            return 1
        run_id = runs[-1]
    state = json.loads(read_text(base / run_id / "state.json"))
    alive = pid_alive(int(state.get("pid", 0)))
    print("run %s pid %s %s; started %s; finished %s; cancelled %s" % (run_id, state.get("pid"), "ALIVE" if alive else "not running", state["created"], state.get("finished"), state.get("cancelled")))
    for row in state["steps"]:
        b = row.get("bazel") or {}
        print("  %-7s %-6s %-14s %-11s rc=%-4s %-9s proc=%s spawns=%s %s" % (row["product"], row["phase"], row["id"], row["status"], row["rc"], fmt_seconds(row["seconds"]), b.get("processes", "-"), b.get("spawns", "-"), row.get("note", "")))
    for line in state.get("events", [])[-6:]:
        print("  " + line)
    return 0


def cancel_command(args) -> int:
    """🛑 Asks a running orchestrator to cancel: CANCEL file plus SIGTERM."""
    base = Path(args.out).resolve() if args.out else Path(args.ticket).resolve() / GENERATED / "e2e-orch"
    held = json.loads(read_text(base / "ORCH.lock") or "{}")
    run_id = args.run_id or held.get("runId")
    if not run_id:
        print("no running orchestrator found")
        return 1
    write_atomic(base / run_id / "CANCEL", now_iso())
    pid = int(held.get("pid", 0))
    if pid and pid_alive(pid):
        os.kill(pid, signal.SIGTERM)
        print("SIGTERM sent to %d (run %s)" % (pid, run_id))
    else:
        print("CANCEL file written for run %s; no live pid" % run_id)
    return 0


def main(argv=None) -> int:
    """🚀 Entry point."""
    args = build_parser().parse_args(argv)
    plan = json.loads(read_text(Path(args.plan)))
    if args.target == "status":
        return status_command(args, plan)
    if args.target == "cancel":
        return cancel_command(args)
    products = sorted(plan["products"], key=lambda p: plan["products"][p]["order"]) if args.target == "all" else [args.target]
    if any(p not in plan["products"] for p in products):
        print("unknown target %r; expected %s, all, status or cancel" % (args.target, sorted(plan["products"])), file=sys.stderr)
        return 2
    phases = expand_phases(args.phase)
    mutating = [p for p in phases if p in PHASE_ORDER]
    if phases == ["report"] and not args.run_id:
        latest = Path(args.out).resolve() / "latest" if args.out else Path(args.ticket).resolve() / GENERATED / "e2e-orch" / "latest"
        args.run_id = read_text(latest).strip() or None
    orch = Orchestrator(args, plan)
    go = args.go or os.environ.get("E2E_GO") == "1"
    if mutating and not go and not args.dry_run:
        print("refused: phases %s run heavy builds and need the coordinator's E2E GO (--go or E2E_GO=1). preflight, report and --dry-run need none." % mutating, file=sys.stderr)
        return 2
    if args.dry_run:
        print_plan(orch, products, phases)
        return 0
    if mutating and not acquire_lock(orch.base, orch.run_id, args.force_lock):
        return 2
    orch.dir.mkdir(parents=True, exist_ok=True)
    orch.load_state()
    orch.state["pid"] = os.getpid()
    write_atomic(orch.dir / "orch.pid", str(os.getpid()))
    write_atomic(orch.base / "latest", orch.run_id)
    if hasattr(signal, "SIGHUP"):
        signal.signal(signal.SIGHUP, signal.SIG_IGN)
    signal.signal(signal.SIGTERM, lambda *_: orch.request_cancel("SIGTERM"))
    signal.signal(signal.SIGINT, lambda *_: orch.request_cancel("SIGINT"))
    orch.parallel_hint = len(products)
    exit_code = 0
    stop = threading.Event()
    monitor = threading.Thread(target=orch.monitor, args=(stop,), daemon=True)
    try:
        orch.log("run %s: products %s phases %s root %s" % (orch.run_id, products, phases, orch.root))
        if "preflight" in phases or (mutating and not args.skip_preflight):
            orch.state["preflight"] = orch.preflight(products, phases)
            orch.save()
            for check in orch.state["preflight"]["checks"]:
                orch.log("preflight %-5s %-34s %s" % (check["status"], check["name"], check["detail"]))
            orch.log("preflight verdict %s" % orch.state["preflight"]["verdict"])
            if orch.state["preflight"]["verdict"] == "FAIL":
                exit_code = 3
        if not args.parallel:
            args.parallel = getattr(orch, "parallel_hint", 1)
        if mutating and exit_code == 0:
            monitor.start()
            orch.sample()
            orch.snapshot("start")
            orch.run_pipelines(products, mutating)
            orch.snapshot("end", wait=600)
            failures = [r for r in orch.state["steps"] if r["status"] in ("FAIL", "CANCELLED")]
            exit_code = 130 if orch.cancel.is_set() else 1 if failures else 0
        elif mutating:
            orch.log("preflight failed: nothing started (fix the FAIL rows, or --skip-preflight)")
    finally:
        stop.set()
        with orch.lock:
            orch.state["cancelled"] = orch.cancel.is_set()
            orch.state["finished"] = now_iso()
        orch.save()
        orch.write_report()
        if mutating:
            release_lock(orch.base)
        orch.log("report: %s" % orch.report_path)
    return exit_code


if __name__ == "__main__":
    sys.exit(main())
