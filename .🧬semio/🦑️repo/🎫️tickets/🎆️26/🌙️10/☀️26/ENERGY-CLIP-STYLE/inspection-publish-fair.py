"""🎞️ Assemble and publish the current catalog through bounded production APIs."""
from __future__ import annotations
import contextlib
import contextvars
import traceback
import weakref
from concurrent.futures import ThreadPoolExecutor, as_completed, wait, FIRST_COMPLETED
from collections import deque
import importlib
import hashlib
import json
import os
from pathlib import Path
import sys
import threading
import time

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
OUTPUT = TICKET / "🗑️generated" / "i"
CONTROL = TICKET / "inspection-publish-control.json"
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
os.environ.update(PYTHONUTF8="1", PYTHONIOENCODING="utf-8", SEMIO_TEACHING_OUTPUT_ROOT=str(OUTPUT))
_LOCK = threading.RLock()
_STATUS = {}

class RestartRequired(RuntimeError):
    """🛑 Require a fresh interpreter when an imported render owner no longer matches disk."""

_OWNER_ROOT = ROOT / Path(*PACKAGE.split("."))
_OWNER_EXTRA = (Path(__file__).resolve(),)
_OWNER_STATE = None
_OWNER_REPORTED = False
_OWNED_SESSIONS = weakref.WeakSet()
_SESSION_LOCK = threading.Lock()

def owner_state(root: Path, extra=()) -> dict:
    """🧬 Fingerprint imported Python owners and cached authoring metadata without body leaves or tests."""
    paths = set(map(Path, extra))
    for directory, children, files in os.walk(root, followlinks=False):
        children[:] = [name for name in children if not name.startswith("🧪") and name not in ("__pycache__", "node_modules", "🗑️generated")
                       and not (Path(directory) / name).is_symlink()]
        paths.update(Path(directory) / name for name in files if Path(name).suffix in (".py", ".json")
                     and not (Path(directory) / name).is_symlink())
    result = {}
    for path in sorted(paths):
        try:
            result[str(path.resolve())] = hashlib.sha256(path.read_bytes()).hexdigest()
        except OSError as error:
            result[str(path.resolve())] = "unavailable:" + str(error.errno)
    return result

def require_owner_state(expected: dict, root: Path, extra=()) -> None:
    """🚦 Refuse membership or content drift in an already imported rendering owner."""
    actual = owner_state(root, extra)
    if expected != actual:
        paths = sorted(path for path in expected.keys() | actual.keys() if expected.get(path) != actual.get(path))
        raise RestartRequired("restart required: imported render owner differs from disk: " + ", ".join(paths[:8]))

def ensure_owner_current() -> None:
    """📣 Publish a fatal restart checkpoint before stale in-memory functions can certify new sources."""
    global _OWNER_REPORTED
    if _OWNER_STATE is None:
        return
    try:
        require_owner_state(_OWNER_STATE, _OWNER_ROOT, _OWNER_EXTRA)
    except RestartRequired as error:
        with _LOCK:
            first = not _OWNER_REPORTED
            if first:
                _OWNER_REPORTED = True
                data = dict(state="restart_required", reason=str(error), checked=time.time(), pid=os.getpid())
                (OUTPUT / "publish-owner.json").write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")
                print("[DEBUG] " + str(error), flush=True)
        if first:
            with _SESSION_LOCK:
                sessions = list(_OWNED_SESSIONS)
            if sessions:
                with ThreadPoolExecutor(max_workers=min(8, len(sessions))) as pool:
                    for future in as_completed([pool.submit(session.cancel) for session in sessions]):
                        try:
                            future.result()
                        except Exception as failure:
                            print("[DEBUG] Owned render cancellation: " + str(failure), flush=True)
        raise

def install_owner_guard(runtime) -> None:
    """🔌 Use existing cancellation ports and reject stale Python before issuing presentation provenance."""
    original_session = runtime.ToolSession
    class OwnedSession(original_session):
        """🚦 Retain only this interpreter's owned tool sessions for cooperative cancellation."""
        def __init__(self):
            super().__init__()
            with _SESSION_LOCK:
                _OWNED_SESSIONS.add(self)
    runtime.ToolSession = OwnedSession
    original_state = runtime.presentation_state
    def coherent_state(*args, **kwargs):
        ensure_owner_current()
        result = original_state(*args, **kwargs)
        ensure_owner_current()
        return result
    runtime.presentation_state = coherent_state

class Tee:
    """📝 Persist progress while preserving the caller's console."""
    def __init__(self, original, log):
        self.original, self.log = original, log
    def write(self, value):
        with _LOCK:
            self.original.write(value)
            self.log.write(value)
            self.log.flush()
        return len(value)
    def flush(self):
        with _LOCK:
            self.original.flush()
            self.log.flush()

def record(identity, **values):
    """🧾 Publish independent per-video progress atomically."""
    with _LOCK:
        _STATUS.setdefault(identity, {}).update(values)
        temporary = OUTPUT / "publish.new"
        temporary.write_text(json.dumps(_STATUS, ensure_ascii=False, indent=2), encoding="utf-8")
        temporary.replace(OUTPUT / "publish.json")

def chapter_export_current(runtime, collection, video_id, raw, directory, authored, settings, presentation):
    """📺 Check an individually published chapter against its exact current raw body and navigation clock."""
    try:
        hierarchy = importlib.import_module(PACKAGE + ".🎬️clip.🪟️presentation.🥞️hierarchy.🐍️")
        if hierarchy.chapter(authored["scene"])["clip_id"] != video_id:
            return False
        output = collection.chapter_output(directory, authored["scene"], settings)
        expected = dict(content=authored, cursor=runtime._cursor_hash(raw), presentation=presentation)
        if (not runtime.reusable_clip(raw, authored) or not runtime.state_inputs_current(authored)
                or not runtime.reusable_clip(output, expected) or not runtime.state_inputs_current(expected)):
            return False
        runtime.cursor_intervals(raw)
        activity = runtime.cursor_intervals(output)
        clock = json.loads(output.with_suffix(".chapter.json").read_text(encoding="utf-8"))
        frames = clock["body_frames"]
        if type(frames) is not int or frames <= 0 or clock != collection.chapter_clock(authored["scene"], frames, settings):
            return False
        duration = clock["frames"] / clock["fps"]
        return bool(activity) and all(0 <= part["start"] < part["end"] <= duration + 0.001 for part in activity)
    except (OSError, ValueError, KeyError, RuntimeError):
        return False

def gate(runtime, videos, base, settings, excluded=(), *, chapters=True):
    """🔒 Validate source, excerpt, cursor and optional chapter provenance through production guards."""
    ensure_owner_current()
    collection = importlib.import_module(PACKAGE + ".🎬️clip.🪟️presentation.📚️collection.🐍️")
    plan = runtime.segment_plan(videos)
    indexes = {index for index, video in enumerate(videos, 1) if video["id"] not in excluded}
    sources = runtime.source_plan([job for job in plan if job["video_index"] in indexes])
    groups = [(index, chapter_index, chapter) for index, video in enumerate(videos, 1)
              if index in indexes for chapter_index, chapter in enumerate(video["chapters"])]
    expected = (len(sources), len(groups)) if excluded else (117, 38)
    if (len(sources), len(groups)) != expected:
        raise RuntimeError(f"Unexpected scope: {len(sources)} sources, {len(groups)} chapters")
    partial = {job["source_ordinal"] for job in plan if "clauses" in job["segment"]}
    def source(job):
        clip = runtime.render_segment(job["segment"], base / "s" / str(job["source_ordinal"]),
            settings, concat_only=True, output_root=base,
            require_trace=job["source_ordinal"] in partial)
        runtime.cursor_intervals(clip)
    def chapter(entry):
        index, chapter_index, authored = entry
        jobs = [job for job in plan if job["video_index"] == index and job["chapter_index"] == chapter_index]
        paths = [base / "s" / str(job["source_ordinal"]) / "v" / f"{job['segment']['scene']}.mp4" for job in jobs]
        for job, clip in zip(jobs, paths, strict=True):
            selected = runtime.excerpt_segment(clip, job["segment"], settings, concat_only=True)
            runtime.cursor_intervals(selected)
        clip = base / str(index) / "v" / f"{authored['scene']}.mp4"
        state = runtime._logical_state(authored["scene"], authored["segments"], settings, paths)
        if not runtime.reusable_clip(clip, state) or not runtime.state_inputs_current(state):
            raise RuntimeError(f"Missing or stale chapter {authored['scene']}")
        runtime.cursor_intervals(clip)
        lesson = base / "rendered" / f"{videos[index - 1]['output']}_spectator_de_480x270p4.mp4"
        presentation = json.loads(lesson.with_suffix(".render.json").read_text(encoding="utf-8"))["presentation"]
        if not chapter_export_current(runtime, collection, videos[index - 1]["id"], clip,
                base / "rendered", state, settings, presentation):
            raise RuntimeError(f"Missing or stale chapter window {authored['scene']}")
    with ThreadPoolExecutor(max_workers=8) as pool:
        for future in as_completed([pool.submit(contextvars.copy_context().run, source, job) for job in sources]):
            future.result()
        if chapters:
            for future in as_completed([pool.submit(contextvars.copy_context().run, chapter, entry) for entry in groups]):
                future.result()
    ensure_owner_current()
    result = dict(canonical_sources=len(sources), chapters=len(groups) if chapters else 0,
                  source_only=not chapters, chapter_exports=len(groups) if chapters else 0, excluded=sorted(excluded), checked=time.time(), state="passed")
    path = OUTPUT / ("publish-source-gate.json" if not chapters else "publish-gate.json")
    path.write_text(json.dumps(result, indent=2), encoding="utf-8")
    print(f"[DEBUG] Provenance gate passed: {result}", flush=True)

def stale_owners(runtime, videos, base, settings):
    """🔎 Select ordinary composition for owners whose source certificates are no longer current."""
    plan = runtime.segment_plan(videos)
    partial = {job["source_ordinal"] for job in plan if "clauses" in job["segment"]}
    stale = []
    def inspect(job):
        try:
            clip = runtime.render_segment(job["segment"], base / "s" / str(job["source_ordinal"]), settings,
                concat_only=True, output_root=base, require_trace=job["source_ordinal"] in partial)
            runtime.cursor_intervals(clip)
        except (OSError, ValueError, RuntimeError) as error:
            return dict(ordinal=job["source_ordinal"], scene=job["segment"]["scene"], error=str(error))
    with ThreadPoolExecutor(max_workers=8) as pool:
        for result in pool.map(inspect, runtime.source_plan(plan)):
            if result:
                stale.append(result)
    ordinals = {job["ordinal"] for job in stale}
    owners = {videos[job["video_index"] - 1]["id"] for job in plan if job["source_ordinal"] in ordinals}
    (OUTPUT / "publish-stale.json").write_text(json.dumps(dict(owners=sorted(owners), sources=stale), ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"[DEBUG] Current stale inventory: {len(stale)} sources; owners {sorted(owners)}", flush=True)
    return owners


def prepare_excerpts(runtime, videos, base, settings):
    """✂️ Prepare every shared clause selection serially before any video composer runs."""
    selections = {}
    for job in runtime.segment_plan(videos):
        if "clauses" in job["segment"]:
            selections.setdefault(json.dumps(job["segment"], sort_keys=True), job)
    if len(selections) != 8:
        raise RuntimeError(f"Unexpected clause-selection count: {len(selections)}")
    for job in selections.values():
        source = runtime.render_segment(job["segment"], base / "s" / str(job["source_ordinal"]), settings,
            concat_only=False, output_root=base, require_trace=True, log_output=True)
        clip = runtime.excerpt_segment(source, job["segment"], settings, force=False, concat_only=False)
        runtime.cursor_intervals(clip)
    print("[DEBUG] All eight clause excerpts and cursor sidecars are current", flush=True)

def retry_token(identity):
    """🎛️ Read an explicit retry counter without changing any source or certificate."""
    if not CONTROL.is_file():
        return 0
    return json.loads(CONTROL.read_text(encoding="utf-8")).get("retry", {}).get(identity, 0)

def cancel_owned_sessions():
    """🛑 Stop this publisher's active tool trees before joining worker threads."""
    with _SESSION_LOCK:
        sessions = list(_OWNED_SESSIONS)
    with ThreadPoolExecutor(max_workers=max(1, min(8, len(sessions)))) as pool:
        for future in as_completed([pool.submit(session.cancel) for session in sessions]):
            future.result()


def run_fair(selected, attempt, ready, checkpoint, cancel, *, workers=4, poll_seconds=0.25, ready_seconds=2.0):
    """⚖️ Yield retries and input waits to a FIFO coordinator without holding worker slots."""
    queue = deque(selected)
    identities = [video["id"] for video in queue]
    if len(set(identities)) != len(identities):
        raise ValueError("Duplicate video identities")
    waiting, active, results = {}, {}, {}
    pool = ThreadPoolExecutor(max_workers=workers)
    try:
        while queue or waiting or active:
            checkpoint()
            now = time.monotonic()
            for identity, (video, outcome, due) in list(waiting.items()):
                if now >= due:
                    if ready(video, outcome):
                        queue.append(video)
                        del waiting[identity]
                    else:
                        waiting[identity] = (video, outcome, now + ready_seconds)
            while queue and len(active) < workers:
                video = queue.popleft()
                future = pool.submit(contextvars.copy_context().run, attempt, video)
                active[future] = video
            if not active:
                threading.Event().wait(poll_seconds)
                continue
            completed, _ = wait(active, timeout=poll_seconds, return_when=FIRST_COMPLETED)
            for future in completed:
                video = active.pop(future)
                outcome = future.result()
                identity = video["id"]
                if outcome["state"] == "completed":
                    results[identity] = outcome.get("output")
                elif outcome["state"] in ("retry", "waiting"):
                    waiting[identity] = (video, outcome, time.monotonic())
                else:
                    raise ValueError("Invalid publication attempt outcome")
    except BaseException:
        try:
            cancel()
        finally:
            for future in active:
                future.cancel()
        raise
    finally:
        pool.shutdown(wait=True, cancel_futures=True)
    return results


def publish(runtime, hierarchy):
    """🪟 Publish distinct videos fairly while sharing canonical raster assets."""
    if sys.flags.utf8_mode != 1:
        raise RuntimeError("Launch this entry with Python -X utf8")
    base = OUTPUT / "de" / "spectator"
    settings = dict(language="de", quality="-ql", resolution="480,270", fps=4.0, audience="spectator")
    videos = runtime.load_manifest(source_ids=[])
    if len(videos) != 17:
        raise RuntimeError("The captured inspection catalog must contain seventeen videos")
    collection = importlib.import_module(PACKAGE + ".🎬️clip.🪟️presentation.📚️collection.🐍️")
    catalog = hierarchy.hierarchy_contract()
    optimization = {video["id"] for topic in catalog["topics"] if topic["id"] == "optimization"
                    for video in topic["videos"]}
    if len(optimization) != 6 or not optimization.issubset({video["id"] for video in videos}):
        raise RuntimeError("The captured catalog must contain six declared optimization videos")
    (OUTPUT / "publish-catalog.json").write_text(json.dumps(catalog, ensure_ascii=False, indent=2), encoding="utf-8")
    ensure_owner_current()
    stale_owners(runtime, videos, base, settings)
    native = {video["id"] for video in videos}
    ensure_owner_current()
    prepare_excerpts(runtime, videos, base, settings)
    ensure_owner_current()
    if "--check-only" in sys.argv:
        return 0
    plan = runtime.segment_plan(videos)
    positions = {video["id"]: index for index, video in enumerate(videos, 1)}
    def output_path(video):
        return base / "rendered" / f"{video['output']}_spectator_de_480x270p4.mp4"
    def current(video):
        ensure_owner_current()
        try:
            output = output_path(video)
            state = json.loads(output.with_suffix(".render.json").read_text(encoding="utf-8"))
            content = base / str(positions[video["id"]]) / "content.mp4"
            presentation = runtime.presentation_state(video["id"], settings, "de")
            chapters = state["content"]["chapters"]
            if [chapter["scene"] for chapter in chapters] != [chapter["scene"] for chapter in video["chapters"]] or any(not chapter_export_current(runtime, collection, video["id"],
                    base / str(positions[video["id"]]) / "v" / (authored["scene"] + ".mp4"), base / "rendered",
                    authored, settings, presentation) for authored in chapters):
                return False
            valid = (runtime.reusable_clip(output, state) and runtime.state_inputs_current(state)
                    and state["content"]["settings"] == settings
                    and runtime.reusable_clip(content, state["content"])
                    and runtime._cursor_hash(content) == state["cursor"]
                    and presentation == state["presentation"])
            ensure_owner_current()
            return valid
        except RestartRequired:
            raise
        except (OSError, ValueError, KeyError, RuntimeError):
            return False
    def versions(video):
        jobs = runtime.source_plan([job for job in plan if job["video_index"] == positions[video["id"]]])
        return [runtime.segment_state({key: job["segment"][key] for key in ("script", "scene")}, settings)
                for job in jobs]
    attempts = {}
    def compose(video):
        identity = video["id"]
        ensure_owner_current()
        attempt = attempts.get(identity, 0) + 1
        attempts[identity] = attempt
        before = versions(video) if identity in native else None
        record(identity, state="running", attempt=attempt, started=time.time())
        try:
            result = runtime.compose_videos([identity], language="de", audience="spectator", quality_flag="-ql",
                output_root=OUTPUT, resolution="480,270", fps=4.0, jobs=1, log_output=True,
                concat_only=identity not in native, play=False)
            ensure_owner_current()
            record(identity, state="completed", completed=time.time(), output=str(result[identity]))
            print(f"[DEBUG] Published {identity}: {result[identity]}", flush=True)
            return dict(state="completed", output=result[identity])
        except RestartRequired:
            raise
        except Exception as error:
            ensure_owner_current()
            changed = "changed" in str(error).lower()
            after = versions(video) if identity in native else None
            if changed or identity in native and before != after:
                record(identity, state="retrying", error=str(error), updated=time.time())
                print(f"[DEBUG] Queued changed inputs for {identity}: {error}", flush=True)
                return dict(state="retry", after=after, not_before=time.monotonic() + min(5, attempt))
            if identity not in native:
                record(identity, state="failed", error=str(error), completed=time.time())
                raise
            token = retry_token(identity)
            record(identity, state="waiting_for_input_change", error=str(error), updated=time.time())
            print(f"[DEBUG] {identity} yielded until an input change or retry control: {error}", flush=True)
            return dict(state="waiting", after=after, token=token)
    def ready(video, outcome):
        ensure_owner_current()
        if outcome["state"] == "retry":
            return time.monotonic() >= outcome["not_before"]
        return versions(video) != outcome["after"] or retry_token(video["id"]) != outcome["token"]
    def parallel(selected):
        return run_fair(selected, compose, ready, ensure_owner_current, cancel_owned_sessions)
    parallel(sorted(videos, key=lambda video: video["id"] != "physical_fundamentals"))
    while True:
        invalid = [video for video in videos if not current(video)]
        if invalid:
            parallel(invalid)
            continue
        try:
            gate(runtime, videos, base, settings)
        except (OSError, ValueError, RuntimeError):
            invalid = [video for video in videos if not current(video)]
            if not invalid:
                raise
            parallel(invalid)
            continue
        if all(current(video) for video in videos):
            break
    ensure_owner_current()
    (OUTPUT / "publish-complete.json").write_text(json.dumps(
        dict(videos=17, sources=117, chapters=38, chapter_exports=38, state="complete", completed=time.time()), indent=2), encoding="utf-8")
    print("[DEBUG] All seventeen clips and thirty-eight chapter windows passed the full source/chapter/output gate", flush=True)
    return 0

def main():
    """📸 Capture one catalog across validation, physical seeding, retries and final publication."""
    global _OWNER_STATE, _OWNER_EXTRA
    _OWNER_STATE = owner_state(_OWNER_ROOT, _OWNER_EXTRA)
    runtime = importlib.import_module(PACKAGE + ".🎬️clip.⚙️runtime.🐍️")
    hierarchy = importlib.import_module(PACKAGE + ".🎬️clip.🪟️presentation.🥞️hierarchy.🐍️")
    browser = importlib.import_module(PACKAGE + ".🎬️clip.🪟️presentation.🌐️browser.🐍️")
    ensure_owner_current()
    extra = tuple(dict.fromkeys((*_OWNER_EXTRA, *hierarchy.source_files())))
    candidates = owner_state(_OWNER_ROOT, extra)
    ensure_owner_current()
    for path in extra:
        key = str(Path(path).resolve())
        if key not in _OWNER_STATE:
            if Path(path).resolve().is_relative_to(_OWNER_ROOT.resolve()):
                raise RestartRequired("restart required: a render owner appeared during imports: " + key)
            _OWNER_STATE[key] = candidates[key]
    _OWNER_EXTRA = extra
    ensure_owner_current()
    install_owner_guard(runtime)
    (OUTPUT / "publish-owner.json").write_text(json.dumps(dict(state="current", loaded=time.time(), pid=os.getpid(), sources=_OWNER_STATE), ensure_ascii=False, indent=2), encoding="utf-8")
    original_sources = browser.source_files
    calls = 0
    def measured_sources():
        nonlocal calls
        calls += 1
        number, started = calls, time.monotonic()
        print(f"[DEBUG] UI dependency discovery {number} started", flush=True)
        result = original_sources()
        print(f"[DEBUG] UI dependency discovery {number}: {len(result)} files, {time.monotonic()-started:.3f}s", flush=True)
        return result
    browser.source_files = measured_sources
    main_thread = threading.get_ident()
    def observe():
        with (OUTPUT / "publish-python-stacks.log").open("a", encoding="utf-8", buffering=1) as log:
            while True:
                time.sleep(60)
                frame = sys._current_frames().get(main_thread)
                if frame is None:
                    return
                log.write(f"[DEBUG] {time.time()} UI source count {len(browser._SOURCES)}\n")
                log.write("".join(traceback.format_stack(frame)))
                del frame
    threading.Thread(target=observe, daemon=True).start()
    with hierarchy.capture_catalog():
        ensure_owner_current()
        return publish(runtime, hierarchy)

if __name__ == "__main__":
    OUTPUT.mkdir(parents=True, exist_ok=True)
    with (OUTPUT / "publish.log").open("a", encoding="utf-8", buffering=1) as log:
        with contextlib.redirect_stdout(Tee(sys.stdout, log)), contextlib.redirect_stderr(Tee(sys.stderr, log)):
            raise SystemExit(main())
