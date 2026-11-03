"""🚚 Rebuild the 26 course previews one render at a time with the routes of ticket 2026/10/28 GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS, watching memory.

Usage: python driver.py [video-id ...]   (default: all videos not yet done in this run, cheap routes first)
State: T/🗑️generated/cp3/state.json, log: T/🗑️generated/cp3/progress.log
"""
import json
import os
import shutil
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
TICKET = HERE.parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
OLD = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/render"
GEN = TICKET / "🗑️generated/cp3"
GEN.mkdir(parents=True, exist_ok=True)
STATE = GEN / "state.json"
LOG = GEN / "progress.log"
PY = ROOT / ".🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
SCRIPT = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts"
SHIM = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/repo-shim.cmd"
CACHE = ROOT / ".🧬semio/🎓️teaching/⚡️cache/videos/en/speaker"
LIMIT = 4 * 1024**3
MANIFEST = json.loads((ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json").read_text(encoding="utf-8"))["videos"]
ENERGY = ["physical_fundamentals", "ventilation", "cooling", "energy_balance", "heating"]
TOPICS = {"grasshopper": "🦗️grasshopper", "excel": "📊️excel"}
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")


def log(message: str):
    """📝 Append a timestamped line to the progress log."""
    line = f"{time.strftime('%H:%M:%S')} {message}"
    with LOG.open("a", encoding="utf-8") as handle:
        handle.write(line + "\n")


def tree_memory(root_pid: int) -> int:
    """📏 Sum the working sets in bytes of a process and all its descendants."""
    command = "Get-CimInstance Win32_Process | Select-Object ProcessId,ParentProcessId,WorkingSetSize | ConvertTo-Csv -NoTypeInformation"
    out = subprocess.run(["powershell", "-NoProfile", "-Command", command], capture_output=True, text=True, stdin=subprocess.DEVNULL).stdout
    rows = {}
    for line in out.splitlines()[1:]:
        parts = line.replace('"', "").split(",")
        if len(parts) == 3 and parts[0].isdigit():
            rows[int(parts[0])] = (int(parts[1] or 0), int(parts[2] or 0))
    members, grew = {root_pid}, True
    while grew:
        grew = False
        for pid, (parent, _) in rows.items():
            if parent in members and pid not in members:
                members.add(pid)
                grew = True
    return sum(rows[pid][1] for pid in members if pid in rows)


def run(name: str, command: list, log_name: str, env_extra: dict) -> str:
    """🏃 Run one command detached from stdin; returns ok, fail or memory (killed above the limit)."""
    env = {**os.environ, "PYTHONUTF8": "1", "PYTHONIOENCODING": "utf-8", "TMP": str(GEN / "tmp"), "TEMP": str(GEN / "tmp"), **env_extra}
    (GEN / "tmp").mkdir(exist_ok=True)
    with (GEN / log_name).open("ab") as handle:
        process = subprocess.Popen(command, cwd=ROOT, env=env, stdin=subprocess.DEVNULL, stdout=handle, stderr=subprocess.STDOUT)
        peak, started = 0, time.time()
        while process.poll() is None:
            time.sleep(20)
            used = tree_memory(process.pid)
            peak = max(peak, used)
            if used > (14 * 1024**3 if name.startswith(("excel", "grasshopper")) else LIMIT):
                subprocess.run(["taskkill", "/F", "/T", "/PID", str(process.pid)], capture_output=True)
                process.wait()
                log(f"{name} KILLED memory {used / 1024**3:.1f} GB after {round(time.time() - started)} s")
                return "memory"
    log(f"{name} exit={process.returncode} peak={peak / 1024**3:.2f} GB {round(time.time() - started)} s")
    return "ok" if process.returncode == 0 else "fail"


def target(video: dict) -> Path:
    """🎯 The preview mp4 of a video."""
    return CACHE / "preview" / f"{video['output']}_speaker_en_480p15.mp4"


def route(video: dict) -> tuple:
    """🧭 Command and extra environment of the 10-31 route of a video."""
    segment = video["chapters"][0]["segments"][0]
    script = segment["script"]
    scene = segment["scene"]
    if video["id"] in ENERGY:
        command = ["bun", str(SCRIPT), "render", "--video", video["id"], "--language", "en", "--audience", "speaker", "--quality", "l",
                   "--fps", "15", "--no-play", "--log-output", "--jobs", "1"]
        return command, {"SEMIO_REPO_BINARY": str(SHIM)}
    series = video["id"].split("_")[0]
    if series in TOPICS:
        topic = script.split("/")[-2]
        return [str(PY), str(OLD / "preview.py"), TOPICS[series], topic, scene, video["output"], "en"], {}
    return [str(PY), str(OLD / "preview_any.py"), script, scene, video["output"], "en"], {}


def deliver_energy(video: dict):
    """📦 Copy the energy master to the preview folder."""
    master = CACHE / "rendered" / f"{video['output']}_speaker_en_480p15.mp4"
    if not master.is_file():
        raise FileNotFoundError(master)
    shutil.copyfile(master, target(video))
    log(f"{video['id']} copied master {master.stat().st_size} bytes")


GATES = {"optimization_sampling": "r2-op-report.md", "optimization_pareto": "r2-op-report.md",
         **{v: "r2-xl3-report.md" for v in ("excel_anatomy", "excel_references", "excel_program", "excel_energy")},
         **{v: ("r2-ap-report.md", "r2-fb-report.md", "r2-eb-report.md") for v in ENERGY}}
HELD = set()
EXPORT_ERROR = "Spread syntax requires"


def gate_open(video_id: str) -> bool:
    """🚧 Whether the coordinator reports that release a video exist."""
    gate = GATES.get(video_id, ())
    names = (gate,) if isinstance(gate, str) else tuple(gate)
    names += ("r2-eb-report.md",) if video_id in HELD else ()
    return all((TICKET / name).is_file() for name in names)


def run_bim(state: dict):
    """🏗️ Render the BIM previews in one process (one exact geometry build)."""
    bim = [v for v in MANIFEST if v["id"].startswith("bim_") and state.get(v["id"], {}).get("status") != "ok"]
    if bim:
        began = state.get("_began", time.time())
        state["_began"] = began
        listing = GEN / "bim.list"
        for attempt in range(1, 4):
            todo = [v for v in bim if not (target(v).is_file() and target(v).stat().st_mtime > began)]
            if not todo:
                break
            wanted_lines = [line for line in (OLD / "bim.list").read_text(encoding="utf-8").splitlines() if line.split()[-1] in {v["output"] for v in todo}]
            listing.write_text("\n".join(wanted_lines) + "\n", encoding="utf-8")
            log(f"bim start attempt {attempt}: {len(todo)} scenes")
            run(f"bim#{attempt}", [str(PY), str(OLD / "preview_many.py"), "bim_fast", "en", str(listing)], "bim.log", {})
        for video in bim:
            ok = target(video).is_file() and target(video).stat().st_mtime > began
            state[video["id"]] = {"status": "ok" if ok else "fail", "at": time.strftime("%Y-%m-%dT%H:%M:%S")}
        STATE.write_text(json.dumps(state, indent=2, ensure_ascii=False), encoding="utf-8")


def main():
    """🚦 Render every requested video once, one at a time."""
    state = json.loads(STATE.read_text(encoding="utf-8")) if STATE.is_file() else {}
    state.setdefault("_began", time.time())
    by_id = {v["id"]: v for v in MANIFEST}
    wanted = sys.argv[1:]
    cheap = [v["id"] for v in MANIFEST if v["id"].startswith("optimization")]
    mid = [v["id"] for v in MANIFEST if v["id"].split("_")[0] in ("excel", "grasshopper") and v["id"] != "grasshopper_capsule"]
    order = wanted or cheap + mid + ["bim"] + ENERGY + ["grasshopper_capsule"]
    pending = [v for v in order if state.get(v, {}).get("status") != "ok"]
    while pending:
        ready = [v for v in pending if gate_open(v)]
        if not ready:
            time.sleep(120)
            continue
        video_id = ready[0]
        pending.remove(video_id)
        if video_id == "bim":
            run_bim(state)
            continue
        video = by_id[video_id]
        command, env = route(video)
        result, memory_kills = "fail", 0
        for attempt in range(1, 4):
            log(f"{video_id} start attempt {attempt}")
            result = run(f"{video_id}#{attempt}", command, f"{video_id}.log", env)
            if result == "ok":
                if video_id in ENERGY:
                    deliver_energy(video)
                break
            if result == "fail" and EXPORT_ERROR in (GEN / f"{video_id}.log").read_bytes()[-40000:].decode("utf-8", "replace"):
                result = "export"
                break
            if result == "memory":
                memory_kills += 1
                if memory_kills > 1:
                    break
        if result == "export":
            HELD.add(video_id)
            pending.append(video_id)
            log(f"{video_id} cast export error, held for r2-eb-report.md")
            continue
        state[video_id] = {"status": result, "at": time.strftime("%Y-%m-%dT%H:%M:%S")}
        STATE.write_text(json.dumps(state, indent=2, ensure_ascii=False), encoding="utf-8")
    log("DRIVER DONE")


main()
