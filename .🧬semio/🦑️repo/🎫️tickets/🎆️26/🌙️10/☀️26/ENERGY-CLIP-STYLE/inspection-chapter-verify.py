"""🔎 Independently verify chapter windows and exact ordered lesson concatenations."""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import itertools
import json
import math
import os
import time
from pathlib import Path

import numpy as np
from PIL import Image

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
PRESENTATION = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation"
CASES = json.loads((PRESENTATION / "🥞️hierarchy/🧪️tests/🧩️chapters/🔣️.cases.json").read_text(encoding="utf-8"))
BASE = TICKET / "🗑️generated/i/de/spectator"
PUBLICATION = BASE.parents[1]
OUTPUT = TICKET / "🗑️generated/iv/collection"
SETTINGS = {"width": 480, "height": 270, "fps": 4, "language": "de", "audience": "spectator"}
SPEC = importlib.util.spec_from_file_location("inspection_streams", TICKET / "inspection-verify.py")
STREAMS = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(STREAMS)
HASHES = {}


class PublicationPending(Exception):
    """🚧 Withhold a clock while its independently witnessed writer is committing it."""


def writer_active(pid, loaded):
    """🔎 Observe publisher liveness without signaling a Windows process."""
    if os.name != "nt":
        try:
            os.kill(pid, 0)
            return True
        except (OSError, OverflowError):
            return False
    import ctypes
    from ctypes import wintypes
    kernel = ctypes.WinDLL("kernel32", use_last_error=True)
    kernel.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
    kernel.OpenProcess.restype = wintypes.HANDLE
    kernel.CloseHandle.argtypes = [wintypes.HANDLE]
    kernel.GetExitCodeProcess.argtypes = [wintypes.HANDLE, ctypes.POINTER(wintypes.DWORD)]
    kernel.GetProcessTimes.argtypes = [wintypes.HANDLE, *[ctypes.POINTER(wintypes.FILETIME)] * 4]
    handle = kernel.OpenProcess(0x1000, False, pid)
    if not handle:
        return False
    try:
        code = wintypes.DWORD()
        clocks = [wintypes.FILETIME() for _ in range(4)]
        if not kernel.GetExitCodeProcess(handle, ctypes.byref(code)) or code.value != 259:
            return False
        if not kernel.GetProcessTimes(handle, *[ctypes.byref(value) for value in clocks]):
            return False
        created = ((clocks[0].dwHighDateTime << 32) | clocks[0].dwLowDateTime) / 10000000 - 11644473600
        return created <= loaded
    finally:
        kernel.CloseHandle(handle)


def clock_pending(clip):
    """🏗️ Prove a current, live publisher epoch owns this still-running chapter commit."""
    paths = [PUBLICATION / "publish-owner.json", PUBLICATION / "publish.json"]
    try:
        before = [path.read_bytes() for path in paths]
        owner, status = [json.loads(value) for value in before]
        loaded, pid = owner["loaded"], owner["pid"]
        active = status[clip]
        started = active["started"]
        if (owner["state"] != "current" or active["state"] != "running"
            or not isinstance(pid, int) or pid <= 0
            or not all(isinstance(value, (int, float)) and math.isfinite(value) for value in (loaded, started))
            or not 0 < loaded <= started <= time.time()):
            return False
        sources = owner["sources"]
        if not sources or (TICKET / "inspection-publish.py").resolve() not in {Path(name).resolve() for name in sources}:
            return False
        if any(STREAMS.sha(Path(name)) != digest for name, digest in sources.items()):
            return False
        return writer_active(pid, loaded) and before == [path.read_bytes() for path in paths]
    except (OSError, ValueError, KeyError, TypeError, OverflowError):
        return False


def read(path):
    """📜 Read one finite owned JSON record."""
    return json.loads(path.read_text(encoding="utf-8"))


def signature(path):
    """🔒 Track both complete media and its publication certificate identity."""
    paths = [path, path.with_suffix(".render.json")]
    if path.parent.name == "chapters":
        paths.append(path.with_suffix(".chapter.json"))
    return [(str(item), item.stat().st_size, item.stat().st_mtime_ns) for item in paths]


def chapter_path(scene):
    """📁 Resolve the explicitly authored individual chapter output convention."""
    return BASE / "rendered/chapters" / (scene + "_spectator_de_480x270p4.mp4")


def verify_stable(paths, operation):
    """🔄 Retry only failed verification whose actual publication identity changed."""
    before = [signature(path) for path in paths]
    try:
        return operation()
    except PublicationPending:
        print("[DEBUG] Active publisher has not committed the matching chapter clock; pending", paths[0], flush=True)
        return None
    except Exception:
        try:
            after = [signature(path) for path in paths]
        except OSError:
            after = None
        if after != before:
            print("[DEBUG] Publication changed during verification; retrying", paths[0], flush=True)
            return None
        raise


def current_certificate(path):
    """🧬 Ignore historical media until every recorded source binding matches the live publication epoch."""
    state = read(path.with_suffix(".render.json"))
    if not state["presentation"]["catalog"].get("courses"):
        return False
    pending = [state]
    while pending:
        value = pending.pop()
        if isinstance(value, dict):
            for key, item in value.items():
                if key in ("sources", "assets") and isinstance(item, dict):
                    for name, digest in item.items():
                        if not isinstance(digest, str) or len(digest) != 64:
                            continue
                        source = Path(name)
                        measured = source.stat()
                        identity = (measured.st_size, measured.st_mtime_ns)
                        if name not in HASHES or HASHES[name][0] != identity:
                            HASHES[name] = (identity, STREAMS.sha(source))
                        if HASHES[name][1] != digest:
                            return False
                pending.append(item)
        elif isinstance(value, list):
            pending.extend(value)
    return True


def chapter_identity(path, scene, clip, chapters, lessons):
    """🧹 Remove measured records when their publication or any certified source is stale."""
    try:
        identity = signature(path)
        if current_certificate(path):
            return identity
    except OSError:
        pass
    chapters.pop(scene, None)
    lessons.pop(clip, None)
    return None


def clock(scene, frames, index):
    """⏲️ Derive navigation frame counts independently from portable ancestry and authored timing."""
    catalog = read(PRESENTATION / "🔣️.json")
    before, after = CASES["boundaries"][index:index + 2]
    counts = []
    for phase, boundary in (("focus", before), ("restore", after)):
        timing = catalog["timing" if phase == "focus" else "restoreTiming"]
        seconds = (4 - len(boundary["path"])) * sum(timing[key] for key in
            ("topicTravel", "topicReveal", "topicPress", "topicHold"))
        seconds += timing["focusHold"] + catalog["contentCamera"]["duration"]
        counts.append(max(3, round(seconds * SETTINGS["fps"])))
    return {"chapter": scene, "path": [CASES["course"], *next(
        [clip["topic"], clip["id"], scene] for clip in CASES["clips"]
        if any(chapter["scene"] == scene for chapter in clip["chapters"]))],
        "focus_frames": counts[0], "body_frames": frames, "restore_frames": counts[1],
        "frames": frames + sum(counts), "fps": SETTINGS["fps"],
        "before": {key: before[key] for key in ("path", "level")},
        "after": {key: after[key] for key in ("path", "level")}}


def verify_chapter(video, position, authored, entries, index):
    """🧪 Decode complete body coverage, chrome-free preservation, clocks, endpoints and audio."""
    scene = authored["scene"]
    path = chapter_path(scene)
    identity = signature(path)
    raw = BASE / str(position) / "v" / (scene + ".mp4")
    raw_state = read(raw.with_suffix(".render.json"))
    state = read(path.with_suffix(".render.json"))
    assert state["content"] == raw_state and raw_state["scene"] == scene
    assert state["presentation"]["catalog"]["courses"][0]["id"] == CASES["course"]
    source_infos = [STREAMS.info(entry["source"]) for entry in entries]
    selected_infos = [STREAMS.info(entry["selected"]) for entry in entries]
    cuts = [STREAMS.cut_info(entry) for entry in entries]
    raw_info, final = STREAMS.info(raw), STREAMS.info(path)
    expected = clock(scene, sum(item["frames"] for item in selected_infos), index)
    assert raw_info["frames"] == expected["body_frames"]
    assert final["frames"] == expected["frames"], (scene, final["frames"], expected)
    committed = read(path.with_suffix(".chapter.json"))
    if committed != expected and clock_pending(video["id"]):
        raise PublicationPending(scene)
    assert committed == expected, (scene, "committed chapter clock", committed, expected)
    start, stop = expected["focus_frames"], expected["focus_frames"] + expected["body_frames"]
    body = STREAMS.frames(raw)
    canonical = itertools.chain.from_iterable(STREAMS.frames(entry["selected"]) for entry in entries)
    selected_error, body_error, edge_error = 0.0, 0.0, 0.0
    captures = {}
    boundaries = list(itertools.accumulate([0, *[item["frames"] for item in selected_infos]]))
    samples = {0, start - 1, start, stop - 1, stop, final["frames"] - 1}
    samples.update(start + (left + right - 1) // 2 for left, right in zip(boundaries, boundaries[1:]))
    for ordinal, frame in enumerate(STREAMS.frames(path)):
        if ordinal in samples:
            captures[ordinal] = frame.copy()
        if start <= ordinal < stop:
            raw_frame, source = next(body), next(canonical)
            delta = STREAMS.error(raw_frame, source)
            selected_error = max(selected_error, delta[0], delta[1])
            assert delta[0] < 0.01 and delta[1] < 0.01, (scene, ordinal, "raw source order", delta)
            delta = STREAMS.error(frame, raw_frame)
            body_error, edge_error = max(body_error, delta[0]), max(edge_error, delta[1])
            assert np.array_equal(frame, raw_frame), (scene, ordinal, "body preservation", delta)
    assert next(body, None) is None and next(canonical, None) is None
    directory = OUTPUT / scene
    directory.mkdir(parents=True, exist_ok=True)
    tiles = [(0, "Open " + expected["before"]["level"]), (start - 1, "Focus body")]
    tiles += [(start + (left + right - 1) // 2, entry["segment"]["scene"])
              for entry, left, right in zip(entries, boundaries, boundaries[1:])]
    tiles += [(stop, "Restore body"), (final["frames"] - 1, "Close " + expected["after"]["level"])]
    STREAMS.sheet(directory / "coverage.png", captures, tiles)
    Image.fromarray(captures[0]).save(directory / "opening.png")
    Image.fromarray(captures[final["frames"] - 1]).save(directory / "closing.png")
    assert STREAMS.error(captures[start - 1], captures[start])[0] < 2
    assert STREAMS.error(captures[stop], captures[stop - 1])[0] < 2
    audio = STREAMS.audio_info(path)
    assert abs(audio["seconds"] - final["duration"]) < 0.1
    if all(item["audio_streams"] == 0 for item in source_infos):
        assert audio["rms"] < 1e-7
    assert signature(path) == identity, (scene, "publication changed during verification")
    return {"id": scene, "scene": scene, "clip_id": video["id"], "path": str(path), "pass": True, **final,
        "source_frames": sum(item["frames"] for item in source_infos), "selected_source_frames": expected["body_frames"],
        "clock": expected, "body_max_mean_error": body_error, "body_max_edge_error": edge_error,
        "source_max_error": selected_error, "cuts": [cut for cut in cuts if cut], "audio": audio,
        "opening_rgb_sha256": hashlib.sha256(captures[0].tobytes()).hexdigest(),
        "closing_rgb_sha256": hashlib.sha256(captures[final["frames"] - 1].tobytes()).hexdigest(),
        "signature": identity, "certificate_sha256": STREAMS.sha(path.with_suffix(".render.json"))}


def verify_lesson(video, chapters):
    """🎞️ Compare every decoded lesson frame with its ordered independently published chapter."""
    path = BASE / "rendered" / (video["output"] + "_spectator_de_480x270p4.mp4")
    identity = signature(path)
    state = read(path.with_suffix(".render.json"))
    assert state["presentation"]["catalog"]["courses"][0]["id"] == CASES["course"]
    assert [chapter["scene"] for chapter in state["content"]["chapters"]] == [chapter["id"] for chapter in chapters]
    final = STREAMS.info(path)
    assert final["frames"] == sum(chapter["frames"] for chapter in chapters)
    authored = itertools.chain.from_iterable(STREAMS.frames(Path(chapter["path"])) for chapter in chapters)
    for index, (actual, expected) in enumerate(zip(STREAMS.frames(path), authored, strict=True)):
        assert np.array_equal(actual, expected), (video["id"], index, "ordered chapter pixel equality")
    audio = STREAMS.audio_info(path)
    assert abs(audio["seconds"] - final["duration"]) < 0.1
    if all(chapter["audio"]["rms"] < 1e-7 for chapter in chapters):
        assert audio["rms"] < 1e-7
    assert signature(path) == identity
    return {"id": video["id"], "path": str(path), "pass": True, **final, "audio": audio,
        "chapters": [chapter["id"] for chapter in chapters], "chapter_sha256": {chapter["id"]: chapter["sha256"] for chapter in chapters}, "signature": identity,
        "certificate_sha256": STREAMS.sha(path.with_suffix(".render.json"))}


def joins(records):
    """🔗 Check every available adjacent boundary and the common outer course overview."""
    result = []
    for case in CASES["boundaries"]:
        left, right = case["left"], case["right"]
        if left is None or right is None:
            continue
        if left in records and right in records:
            assert records[left]["closing_rgb_sha256"] == records[right]["opening_rgb_sha256"], (left, right, case)
            result.append({**case, "pass": True})
    ordered = [chapter["scene"] for clip in CASES["clips"] for chapter in clip["chapters"]]
    if ordered[0] in records and ordered[-1] in records:
        assert records[ordered[0]]["opening_rgb_sha256"] == records[ordered[-1]]["closing_rgb_sha256"]
        result.append({"left": None, "right": None, "path": [], "level": "courses", "pass": True})
    return result


def commit_status(pending, target):
    """🔏 Bound retries for Windows readers denying atomic status replacement."""
    for attempt in range(50):
        try:
            pending.replace(target)
            return
        except PermissionError:
            if attempt == 49:
                raise
            time.sleep(.1)


def publish(chapters, lessons, comparisons, complete=False):
    """📝 Atomically retain independent measured coverage without promoting old wrapper previews."""
    payload = {"profile": SETTINGS, "expected": 38, "verified": len(chapters), "lessons": len(lessons),
        "joins_verified": len(comparisons), "joins_pass": len(comparisons) == 38, "order_pass": len(lessons) == 17,
        "pass": complete, "provenance_pass": complete,
        "chapters": list(chapters.values()), "videos": list(lessons.values()), "joins": comparisons, "checked": time.time()}
    OUTPUT.mkdir(parents=True, exist_ok=True)
    pending = OUTPUT.parent / "chapter-measurements.new"
    pending.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    commit_status(pending, OUTPUT.parent / "chapter-measurements.json")
    lines = ["# Individual Chapter And Concatenated Lesson Verification", "", f"Measured {len(chapters)}/38 chapter windows and {len(lessons)}/17 exact lesson concatenations; {len(comparisons)}/38 pixel joins.", "",
        "The five earlier whole-lesson previews remain separate evidence and do not satisfy this collection gate.", ""]
    lines += [f"- {record['id']}: {record['frames']} frames / {record['duration']:.2f}s; body mean/edge {record['body_max_mean_error']:.4f}/{record['body_max_edge_error']:.4f}." for record in chapters.values()]
    (TICKET / "inspection-chapter-verification.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


def verify_provenance(records, marker):
    """🔐 Rehash every unique nested source binding and publication record before final acceptance."""
    bindings = {}

    def bind(path, digest):
        """🧬 Reject a mixed-version dependency shared by two accepted publications."""
        path = str(Path(path).resolve())
        assert path not in bindings or bindings[path] == digest, (path, "conflicting current source bindings")
        bindings[path] = digest

    def visit(value):
        """🌳 Read finite certificate trees without trusting a rendering module's currentness helper."""
        if isinstance(value, dict):
            for key, item in value.items():
                if key in ("sources", "assets") and isinstance(item, dict):
                    for name, digest in item.items():
                        if isinstance(digest, str) and len(digest) == 64:
                            bind(name, digest)
                visit(item)
        elif isinstance(value, list):
            for item in value:
                visit(item)

    owned = [Path(__file__), Path(STREAMS.__file__), STREAMS.MANIFEST,
             TICKET / "inspection-chapter-verifier-test.py", TICKET / "inspection-chapter-verifier-cases.json",
             ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧬️algorithms/🧪️tests/📈️comparison/🔣️.json",
             ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧬️algorithms/🧪️tests/📈️comparison/🐍️.py",
             ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧬️algorithms/🧪️tests/🧩️widgets/🔣️.json",
             ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧬️algorithms/🧪️tests/🧩️widgets/🐍️.py",
             ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts",
             PRESENTATION / "🔣️.json", PRESENTATION / "📚️collection/🔣️.json",
             PRESENTATION / "🥞️hierarchy/🧪️tests/🧩️chapters/🔣️.cases.json",
             marker, marker.with_name("publish-gate.json")]
    owner_path = marker.with_name("publish-owner.json")
    owner = read(owner_path)
    assert owner["state"] == "current" and owner["sources"], "The publisher's loaded source guard is not current"
    assert 0 < owner["loaded"] <= read(marker)["completed"]
    visit(owner)
    owned.extend([owner_path, TICKET / "inspection-publish.py"])
    for record in records:
        media = Path(record["path"])
        certificate = media.with_suffix(".render.json")
        assert STREAMS.sha(certificate) == record["certificate_sha256"]
        visit(read(certificate))
        owned.append(certificate)
        if media.parent.name == "chapters":
            owned.append(media.with_suffix(".chapter.json"))
    for path in owned:
        bind(path, STREAMS.sha(path))
    before = {path: STREAMS.sha(Path(path)) for path in bindings}
    assert before == bindings, "An accepted source or publication record is no longer current"
    after = {path: STREAMS.sha(Path(path)) for path in bindings}
    assert after == before, "Publication inputs changed during the final provenance snapshot"
    result = {"pass": True, "checked": time.time(), "files": bindings}
    (OUTPUT.parent / "chapter-provenance.json").write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    return result


def main():
    """👁️ Verify arriving finite publications and wait for the all-source all-chapter completion gate."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--watch", action="store_true")
    parser.add_argument("--chapter")
    args = parser.parse_args()
    videos = read(STREAMS.MANIFEST)["videos"]
    planned = STREAMS.plan(videos, BASE)
    chapters, lessons = {}, {}
    while True:
        index = 0
        for video in videos:
            position, entries = planned[video["id"]]
            for authored in video["chapters"]:
                scene = authored["scene"]
                ordinal, index = index, index + 1
                if args.chapter and args.chapter != scene:
                    continue
                path = chapter_path(scene)
                current = chapter_identity(path, scene, video["id"], chapters, lessons)
                if current is None:
                    continue
                if scene in chapters and chapters[scene]["signature"] == current:
                    continue
                print("[DEBUG] Verifying chapter", scene, flush=True)
                measured = verify_stable([path], lambda: verify_chapter(video, position, authored,
                    [entry for entry in entries if entry["chapter"] == scene], ordinal))
                if measured is None:
                    chapters.pop(scene, None)
                    lessons.pop(video["id"], None)
                    continue
                chapters[scene] = measured
                comparisons = joins(chapters)
                publish(chapters, lessons, comparisons)
            if args.chapter:
                continue
            names = [chapter["scene"] for chapter in video["chapters"]]
            if not all(name in chapters for name in names):
                continue
            path = BASE / "rendered" / (video["output"] + "_spectator_de_480x270p4.mp4")
            try:
                current = signature(path)
            except OSError:
                lessons.pop(video["id"], None)
                continue
            try:
                published = read(path.with_suffix(".render.json"))
                if not current_certificate(path):
                    lessons.pop(video["id"], None)
                    continue
                if path.stat().st_mtime_ns < max(Path(chapters[name]["path"]).stat().st_mtime_ns for name in names):
                    lessons.pop(video["id"], None)
                    continue
            except (OSError, ValueError, KeyError):
                lessons.pop(video["id"], None)
                continue
            if video["id"] in lessons and lessons[video["id"]]["signature"] == current and lessons[video["id"]]["chapter_sha256"] == {name: chapters[name]["sha256"] for name in names}:
                continue
            print("[DEBUG] Verifying exact lesson", video["id"], flush=True)
            measured = verify_stable([path, *[Path(chapters[name]["path"]) for name in names]],
                lambda: verify_lesson(video, [chapters[name] for name in names]))
            if measured is None:
                lessons.pop(video["id"], None)
                continue
            lessons[video["id"]] = measured
            publish(chapters, lessons, joins(chapters))
        comparisons = joins(chapters)
        marker = BASE.parents[1] / "publish-complete.json"
        complete = len(chapters) == 38 and len(lessons) == 17 and len(comparisons) == 38
        if complete and marker.is_file():
            completion = read(marker)
            complete = completion.get("state") == "complete" and completion.get("chapter_exports") == 38 and completion.get("videos") == 17 and completion.get("sources") == 117 and completion.get("chapters") == 38
            gate = read(BASE.parents[1] / "publish-gate.json")
            complete = complete and gate.get("state") == "passed" and gate.get("canonical_sources") == 117 and gate.get("chapters") == 38 and gate.get("chapter_exports") == 38 and gate.get("excluded") == []
            records = [*chapters.values(), *lessons.values()]
            complete = complete and completion["completed"] >= max(Path(record["path"]).stat().st_mtime for record in records)
            complete = complete and all(signature(Path(record["path"])) == record["signature"] for record in records)
        else:
            complete = False
        if complete:
            verify_provenance([*chapters.values(), *lessons.values()], marker)
            complete = all(signature(Path(record["path"])) == record["signature"] for record in [*chapters.values(), *lessons.values()])
        publish(chapters, lessons, comparisons, complete)
        if complete or not args.watch:
            break
        time.sleep(5)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
