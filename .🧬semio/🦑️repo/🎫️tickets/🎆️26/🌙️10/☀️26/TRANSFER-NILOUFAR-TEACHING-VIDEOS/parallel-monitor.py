"""⚡ Monitor retained render children and stage complete chapters without publishing."""

from concurrent.futures import ThreadPoolExecutor, as_completed
import ctypes
from ctypes import wintypes
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import threading

sys.dont_write_bytecode = True
TICKET = Path(__file__).resolve().parent
ROOT = Path.cwd() / "🎓️teaching"
sys.path.insert(0, str(ROOT))

from video_runtime import clip_state, load_manifest, manim_command, render_config

OUTPUT = TICKET / "🗑️generated/videos"
PRE = TICKET / "🗑️generated/pre"
SETTINGS = {"quality": "-ql", "resolution": None, "fps": None}
LOCK = threading.Lock()
PROCESSES = set()
STOP = threading.Event()
KERNEL = ctypes.WinDLL("kernel32", use_last_error=True)
KERNEL.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
KERNEL.OpenProcess.restype = wintypes.HANDLE
KERNEL.WaitForSingleObject.argtypes = [wintypes.HANDLE, wintypes.DWORD]
KERNEL.WaitForSingleObject.restype = wintypes.DWORD
KERNEL.GetExitCodeProcess.argtypes = [wintypes.HANDLE, ctypes.POINTER(wintypes.DWORD)]
KERNEL.GetExitCodeProcess.restype = wintypes.BOOL
KERNEL.CloseHandle.argtypes = [wintypes.HANDLE]


def wait_existing(pid):
    """🪟 Wait on a retained native process handle without sending any signals."""
    handle = KERNEL.OpenProcess(0x100000 | 0x1000, False, pid)
    if not handle:
        return
    try:
        while KERNEL.WaitForSingleObject(handle, 1000) == 258:
            if STOP.is_set():
                raise InterruptedError("Staging monitor cancelled")
        status = wintypes.DWORD()
        KERNEL.GetExitCodeProcess(handle, ctypes.byref(status))
        if status.value:
            raise RuntimeError(f"Retained Manim process {pid} exited with {status.value}")
    finally:
        KERNEL.CloseHandle(handle)


def render(job):
    """🎬 Render or adopt one complete staged chapter without publishing outputs."""
    number, index, video, source, name, pid = job
    if STOP.is_set():
        return
    media = PRE / str(number)
    config = render_config(media) if not pid else media / "render.cfg"
    common = [ROOT / "manim_fonts.py", ROOT / "manim_visuals.py"]
    source_folder = ROOT / "intro" if source.parent.name == "intro" else (ROOT / video["script"]).parent
    dependencies = [*common, *(path for path in source_folder.rglob("*") if path.suffix in (".py", ".json", ".pdf", ".svg", ".png", ".jpg"))]
    state = clip_state(SETTINGS, dependencies)
    clip = media / "v" / f"{name}.mp4"
    if pid:
        print(f"[DEBUG] Monitoring retained PID {pid}: {name}", flush=True)
        wait_existing(pid)
    elif not clip.is_file():
        environment = {**os.environ, "SEMIO_TEACHING_OUTPUT_ROOT": str(OUTPUT), "PYTHONUTF8": "1",
                       "PYTHONIOENCODING": "utf-8", "PYTHONDONTWRITEBYTECODE": "1"}
        environment.pop("PYTHONPYCACHEPREFIX", None)
        command = [*manim_command(), "--config_file", str(config), "-ql", "--media_dir", str(media),
                   "--disable_caching", str(source), name]
        print(f"[DEBUG] Starting staged {video['id']}: {name}", flush=True)
        with (media / "render.log").open("a", encoding="utf-8") as log:
            log.write(f"\n[DEBUG] Starting complete render attempt for {name}\n")
            log.flush()
            process = subprocess.Popen(command, cwd=ROOT, env=environment, stdout=log, stderr=subprocess.STDOUT)
            with LOCK:
                PROCESSES.add(process)
                mapping_path = PRE / "jobs.json"
                mapping = json.loads(mapping_path.read_text(encoding="utf-8"))
                for item in mapping:
                    if item["job"] == number:
                        item["proxy_pid"] = process.pid
                mapping_path.write_text(json.dumps(mapping, indent=2) + "\n", encoding="utf-8")
            code = process.wait()
            with LOCK:
                PROCESSES.discard(process)
        if code:
            raise RuntimeError(f"{name} failed with status {code}; see {media / 'render.log'}")
    if not clip.is_file() or clip.stat().st_size <= 1000:
        raise RuntimeError(f"Missing completed staged clip {clip}")
    if clip_state(SETTINGS, dependencies) != state:
        raise RuntimeError(f"Authored dependencies changed while rendering {name}")
    clip.with_suffix(".render.json").write_text(json.dumps(state, indent=2) + "\n", encoding="utf-8")
    print(f"[DEBUG] Staged complete {video['id']}: {name}", flush=True)


def main():
    """🚦 Adopt existing children and schedule remaining chapters across eight workers."""
    records = json.loads((PRE / "handoff.json").read_text(encoding="utf-8-sig"))
    retained = {}
    for record in records:
        match = re.search(r"\\pre\\(\d+)\\render.cfg", record["CommandLine"])
        if match and "cpython-" in record["CommandLine"]:
            retained[int(match.group(1))] = record["ProcessId"]
    videos = load_manifest()
    jobs = []
    for video_id, start in [("ventilation", -1), ("heating", 1), ("cooling", 3), ("energy_balance", 1)]:
        index, video = next((index, video) for index, video in enumerate(videos, 1) if video["id"] == video_id)
        source = ROOT / video["script"]
        entries = [(source, name) for name in video["scenes"][max(0, start):]]
        if start == -1:
            entries.insert(0, (ROOT / "intro/intro_scene.py", video["intro"]))
        for script, name in entries:
            number = len(jobs) + 1
            jobs.append((number, index, video, script, name, retained.get(number)))
    mapping = [{"job": number, "video_index": index, "video": video["id"], "source": str(source),
                "scene": name, "pid": pid, "media": str(PRE / str(number))}
               for number, index, video, source, name, pid in jobs]
    previous_mapping = json.loads((PRE / "jobs.json").read_text(encoding="utf-8")) if (PRE / "jobs.json").is_file() else []
    mapping.extend({**job, "pid": retained.get(job["job"], job.get("pid"))} for job in previous_mapping if job["job"] > len(jobs))
    (PRE / "jobs.json").write_text(json.dumps(mapping, indent=2) + "\n", encoding="utf-8")
    print(f"[DEBUG] Staging {len(jobs)} complete chapters across eleven workers; publication disabled", flush=True)
    pool = ThreadPoolExecutor(max_workers=11)
    futures = [pool.submit(render, job) for job in jobs]
    failures = []
    try:
        for future in as_completed(futures):
            try:
                future.result()
            except Exception as error:
                failures.append(str(error))
                print(f"[DEBUG] Staged job failed: {error}", flush=True)
                (PRE / "failures.json").write_text(json.dumps(failures, indent=2) + "\n", encoding="utf-8")
    except BaseException:
        STOP.set()
        with LOCK:
            processes = list(PROCESSES)
        for process in processes:
            process.terminate()
        pool.shutdown(wait=True, cancel_futures=True)
        raise
    pool.shutdown()
    if failures:
        raise RuntimeError(f"{len(failures)} staged jobs failed; completed clips remain intact")
    print("[DEBUG] All staged chapters completed; publish only after original jobs stop", flush=True)


if __name__ == "__main__":
    main()
