"""🧪 peer-g (harness3): peer-edit scenarios on an official product build; reuses bench3-g `measure.run` (phases, tree RSS, Bazel actions, log signals).

`scenario.py run <out> <stage>...`   stage = `control:<product>` | `peer:<id>:<product>[:<when>]` with ids and triggers from `scenarios.json`
`scenario.py preflight`              edit targets exist, are tracked, carry no residue of an earlier crash (read-only)
`scenario.py restore <out>`          puts the files back from the byte backups of a crashed run
`scenario.py render <out>`           markdown verdict table of the stage records (same as harness3-tools/render.py peer)

A peer stage runs the build, waits for the trigger row (`when`: graph | bazel | execute) in its live log, appends ONE comment line to
every file of the scenario (the comment syntax follows the file suffix), holds it for `holdSeconds`, restores the bytes exactly
(sha256 checked, forced again in `finally` and on SIGTERM/SIGINT) and lets the build finish. The next `control` stage is the
follow-up hot run. Exit 75 and host faults (ENFILE/EMFILE) are retried like an agent would; every attempt is recorded.
Dry runs: `PEER_ROOT=<fake checkout> PEER_FAKE_CMD="python3 harness3-tools/fakebuild.py <log>" scenario.py run ...` (no `bun`, announcements go to the output directory).
"""
import hashlib, json, os, re, signal, subprocess, sys, threading, time

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.abspath(os.path.join(HERE, ".."))
ROOT = os.environ.get("PEER_ROOT") or os.path.abspath(os.path.join(HERE, *([".."] * 8)))
BENCH = os.path.join(HERE, "..", "bench3-g-tools", "measure.py")
COORD = os.path.join(HERE, "..", "coordination.md")
SPEC = json.load(open(os.path.join(HERE, "scenarios.json"), encoding="utf8"))
TARGETS = SPEC["products"]
sys.path.insert(0, os.path.join(HERE, "..", "harness3-tools"))
import hostgate  # noqa: E402
NAMESPACE = {"__name__": "measure", "__file__": BENCH}
exec(compile(open(BENCH, encoding="utf8").read(), BENCH, "exec"), NAMESPACE)
RUN = NAMESPACE["run"]
WATCH = ["🧰️framework", "🎓️teaching", "✏️s"]
NOISE = ("/node_modules/", "/.cache/", "/🗑️generated/", "/target/", "/dist/", ".🧾️store")
OUT = [None]
TERM = [False]
STOP = threading.Event()


def say(text):
    line = "- %s peer-g(harness3) %s\n" % (time.strftime("%H:%M"), text)
    path = os.path.join(OUT[0], "announce.log") if os.environ.get("PEER_ROOT") and OUT[0] else COORD
    with open(path, "a", encoding="utf8") as handle:
        handle.write(line)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def comment_line(path, nonce):
    suffix = os.path.splitext(path)[1]
    template = SPEC["comments"].get(suffix)
    if template is None:
        raise SystemExit("no comment syntax for %s in scenarios.json" % suffix)
    return (template % nonce).encode("utf8")


def peers_since(marker, skip):
    """Files below the watched roots modified after the marker (other lanes' edits during the run), minus our own edits and the build's own stores."""
    command = ["find"] + [w for w in WATCH if os.path.isdir(os.path.join(ROOT, w))] + ["-type", "f", "-newer", marker]
    try:
        out = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=300).stdout.splitlines()
    except Exception:
        return None
    return sorted(x for x in out if x not in skip and not any(n in "/" + x for n in NOISE))[:60]


def tracked(path):
    return subprocess.run(["git", "ls-files", "--error-unmatch", path], cwd=ROOT, capture_output=True, text=True).returncode == 0


def preflight():
    bad = 0
    for ident, scenario in SPEC["scenarios"].items():
        for rel in scenario["files"]:
            path = os.path.join(ROOT, rel)
            if not os.path.isfile(path):
                print("MISSING %s %s" % (ident, rel))
                bad += 1
                continue
            data = open(path, "rb").read()
            residue = b"[peer-g]" in data or b"[incremental]" in data
            comment_line(path, 0)
            note = "tracked" if tracked(rel) or os.environ.get("PEER_ROOT") else "UNTRACKED"
            print("%s %-4s %-5s %7d B sha256 %s %s%s" % ("RESIDUE" if residue else "ok     ", ident, note, len(data), sha(data)[:12], rel, "  <- an earlier crash left an edit: run `restore`" if residue else ""))
            bad += int(residue or note == "UNTRACKED")
    for name, rx in SPEC["triggers"].items():
        re.compile(rx)
    print("triggers:", ", ".join(SPEC["triggers"]))
    raise SystemExit(1 if bad else 0)


def restore_dir(out):
    manifest = os.path.join(out, "backup", "manifest.jsonl")
    done = 0
    for line in open(manifest, encoding="utf8"):
        row = json.loads(line)
        path = os.path.join(ROOT, row["file"])
        original = open(os.path.join(out, "backup", row["backup"]), "rb").read()
        current = open(path, "rb").read() if os.path.exists(path) else None
        if current != original:
            open(path, "wb").write(original)
            done += 1
        print("%s %s sha256 %s" % ("restored" if current != original else "intact  ", row["file"], sha(original)[:12]))
    print("restored %d file(s)" % done)


def hold_seconds(scenario):
    return float(os.environ.get("PEER_HOLD") or scenario["holdSeconds"])


def restore_file(path, original, edited):
    """Byte-exact revert that keeps another writer's change: `exact` (file back to the original), `merged` (our line stripped, a peer's change kept), `conflict` (left as is)."""
    current = open(path, "rb").read() if os.path.exists(path) else None
    if current == original:
        return "exact"
    suffix = edited[len(original):] if edited is not None and edited.startswith(original) else b""
    if current is not None and edited is not None and current == edited:
        open(path, "wb").write(original)
        return "exact"
    at = current.rfind(suffix) if current is not None and suffix else -1
    if at >= 0:
        open(path, "wb").write(current[:at] + current[at + len(suffix):])
        return "merged"
    return "conflict"


def hold_edit(files, originals, edited, backups, scenario, state, ident, product, label, nonce):
    try:
        for f in files:
            originals[f] = open(f, "rb").read()
            open(backups[f], "wb").write(originals[f])
            data = originals[f]
            edited[f] = data + (b"\n" if data and not data.endswith(b"\n") else b"") + comment_line(f, nonce)
            open(f, "wb").write(edited[f])
            state["edits"].append(os.path.relpath(f, ROOT))
            say("EDIT ON %s (%s %s, build %s)" % (os.path.relpath(f, ROOT), ident, product, label))
        state["edited"] = True
        STOP.wait(hold_seconds(scenario))
    finally:
        states = {}
        for f in files:
            if f in edited:
                states[os.path.relpath(f, ROOT)] = restore_file(f, originals[f], edited[f])
                say("EDIT OFF %s (restore: %s)" % (os.path.relpath(f, ROOT), states[os.path.relpath(f, ROOT)]))
        state["restored"] = True if all(v == "exact" for v in states.values()) else states


def stage(out, spec, index):
    parts = spec.split(":")
    kind = parts[0]
    ident, product, when = (parts + [None, None, None])[1:4] if kind == "peer" else (None, parts[1], None)
    scenario = SPEC["scenarios"][ident] if ident else None
    when = when or (scenario or {}).get("when", "graph")
    label = "%02d-%s-%s%s" % (index, product, ident or "control", "-" + when if ident else "")
    hostgate.wait_quiet(ROOT, say=lambda text: print("[%s] %s" % (time.strftime("%H:%M:%S"), text), flush=True))
    marker = os.path.join(out, label + ".marker")
    open(marker, "w").close()
    state = {"edited": False, "restored": None, "edits": [], "triggerAtMs": None, "when": when if ident else None}
    files = [os.path.join(ROOT, f) for f in (scenario["files"] if scenario else [])]
    originals = {f: open(f, "rb").read() for f in files}
    edited, backups = {}, {}
    os.makedirs(os.path.join(out, "backup"), exist_ok=True)
    with open(os.path.join(out, "backup", "manifest.jsonl"), "a", encoding="utf8") as manifest:
        for f, data in originals.items():
            name = label + "." + hashlib.sha1(f.encode()).hexdigest()[:8]
            backups[f] = os.path.join(out, "backup", name)
            open(backups[f], "wb").write(data)
            manifest.write(json.dumps({"label": label, "file": os.path.relpath(f, ROOT), "backup": name, "sha256": sha(data)}, ensure_ascii=False) + "\n")
    log = os.path.join(out, label + ".log")
    t0 = time.monotonic()
    finished = threading.Event()

    def editor():
        pattern = re.compile(SPEC["triggers"][when])
        offset, ready = 0, False
        while not ready and time.monotonic() - t0 < 3600 and not finished.is_set():
            time.sleep(0.2)
            if not os.path.exists(log):
                continue
            with open(log, "rb") as handle:
                handle.seek(offset)
                chunk = handle.read()
            if b"\n" in chunk:
                chunk, offset = chunk[: chunk.rindex(b"\n") + 1], offset + chunk.rindex(b"\n") + 1
                ready = any(pattern.search(line) for line in chunk.decode("utf8", "replace").splitlines())
        if not ready:
            return
        state["triggerAtMs"] = round((time.monotonic() - t0) * 1000)
        hold_edit(files, originals, edited, backups, scenario, state, ident, product, label, int(time.time()))

    thread = None
    if scenario:
        thread = threading.Thread(target=editor, daemon=True)
        thread.start()
    fake = os.environ.get("PEER_FAKE_CMD")
    command = fake.split() if fake else ["nice", "-n", "10", "bun", "repo", "--tool=bazel", "run", TARGETS[product]]
    started = time.strftime("%H:%M:%S")
    attempts, record = [], None
    try:
        for attempt in range(4):
            record = RUN(command, ROOT, label + ("" if attempt == 0 else ".retry%d" % attempt), out)
            tail = open(record["log"], "rb").read()[-6000:].decode("utf8", "replace")
            host_fault = record["exit"] != 0 and "execute" not in record["phases"] and ("ENFILE" in tail or "EMFILE" in tail) and not state["edited"]
            bazel = record["actions"].get("bazel") or {}
            attempts.append({"exit": record["exit"], "wallMs": record["wallMs"], "executed": bazel.get("executed"), "hostFault": host_fault, "yielded75": record["exit"] == 75, "restarts": record["signals"]["restarts"]["count"]})
            if TERM[0] or not (host_fault or record["exit"] == 75):
                break
            time.sleep(10 if record["exit"] == 75 else 60)
    except BaseException:
        STOP.set()
        raise
    finally:
        finished.set()
        try:
            if thread:
                thread.join((hold_seconds(scenario) if scenario else 0) + 120)
        except KeyboardInterrupt:
            STOP.set()
            thread.join(30)
        for f in edited:
            if restore_file(f, originals[f], edited[f]) != "exact" and state["restored"] is True:
                state["restored"] = "forced after run"
    first_wall = attempts[0]["wallMs"] if attempts else None
    state["editedBeforeExitMs"] = None if state["triggerAtMs"] is None or first_wall is None else round(first_wall - state["triggerAtMs"])
    state["editDuringRun"] = state["editedBeforeExitMs"] is not None and state["editedBeforeExitMs"] > 0
    record.update({"stage": spec, "label": label, "startedLocal": started, "peerState": state, "peerFilesDuringRun": peers_since(marker, set(state["edits"])), "attempts": attempts, "retried75": any(a["yielded75"] for a in attempts)})
    json.dump(record, open(os.path.join(out, label + ".json"), "w", encoding="utf8"), indent=1, ensure_ascii=False)
    os.remove(marker)
    actions = record["actions"].get("bazel") or {}
    print(json.dumps({"label": label, "exit": record["exit"], "wallMs": record["wallMs"], "executed": actions.get("executed"), "line": actions.get("line"), "graph": record["signals"]["graph"]["outcome"], "plan": record["signals"]["plan"]["outcome"], "restarts": record["signals"]["restarts"]["count"], "attempts": len(attempts), "editDuringRun": state["editDuringRun"], "restored": state["restored"], "peers": len(record["peerFilesDuringRun"] or [])}, ensure_ascii=False), flush=True)
    return record


def render(out):
    subprocess.run([sys.executable, "-I", os.path.join(HERE, "..", "harness3-tools", "render.py"), "peer", out, "--write"], check=False)


def run_chain(out, stages):
    OUT[0] = out
    os.makedirs(out, exist_ok=True)
    def terminate(*_):
        TERM[0] = True
        STOP.set()
        raise KeyboardInterrupt()

    signal.signal(signal.SIGTERM, terminate)
    for index, spec in enumerate(stages):
        if TERM[0]:
            raise SystemExit(143)
        stage(out, spec, index)
        if TERM[0]:
            raise SystemExit(143)
    print("[%s] CHAIN-DONE" % time.strftime("%H:%M:%S"), flush=True)
    render(out)


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else ""
    if mode == "preflight":
        preflight()
    elif mode == "restore" and len(sys.argv) == 3:
        restore_dir(sys.argv[2])
    elif mode == "render" and len(sys.argv) == 3:
        render(sys.argv[2])
    elif mode == "run" and len(sys.argv) >= 4:
        run_chain(sys.argv[2], sys.argv[3:])
    else:
        raise SystemExit(__doc__)


main()
