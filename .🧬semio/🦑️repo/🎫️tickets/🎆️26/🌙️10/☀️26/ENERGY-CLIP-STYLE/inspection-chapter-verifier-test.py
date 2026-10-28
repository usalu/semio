"""🧪 Exercise independent chapter acceptance with real finite PyAV fixtures and deliberate defects."""
import hashlib
import importlib.util
import json
from pathlib import Path
import time
import os
import threading
from unittest.mock import patch

TICKET = Path(__file__).resolve().parent


def load(name, filename):
    """📦 Load a retained owned input without modifying any production module."""
    specification = importlib.util.spec_from_file_location(name, TICKET / filename)
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


v = load("collection_acceptance", "inspection-chapter-verify.py")
media = load("independent_fixture_encoder", "inspection-verifier-test.py")
cases = json.loads((TICKET / "inspection-chapter-verifier-cases.json").read_text(encoding="utf-8"))
v.BASE = TICKET / "🗑️generated/iv/collection-selftest"
v.OUTPUT = v.BASE / "evidence"
v.BASE.mkdir(parents=True, exist_ok=True)
v.PUBLICATION = v.BASE / "publisher"
v.PUBLICATION.mkdir(parents=True, exist_ok=True)
status, pending = v.BASE / "reader-status.json", v.BASE / "reader-status.new"
status.write_text("old", encoding="utf-8")
pending.write_text(cases["status_retry"]["content"], encoding="utf-8")
if os.name == "nt":
    import ctypes
    from ctypes import wintypes
    kernel = ctypes.WinDLL("kernel32", use_last_error=True)
    kernel.CreateFileW.restype = wintypes.HANDLE
    kernel.CreateFileW.argtypes = [wintypes.LPCWSTR, wintypes.DWORD, wintypes.DWORD, ctypes.c_void_p, wintypes.DWORD, wintypes.DWORD, wintypes.HANDLE]
    kernel.CloseHandle.argtypes = [wintypes.HANDLE]
    handle = kernel.CreateFileW(str(status), 0x80000000, 1, None, 3, 0, None)
    assert handle != wintypes.HANDLE(-1).value
    try:
        pending.replace(status)
    except PermissionError:
        pass
    else:
        raise AssertionError("Native reader did not block deletion")
    def release_reader():
        """🔓 Release the real native reader after initial commit attempts."""
        time.sleep(cases["status_retry"]["hold_seconds"])
        kernel.CloseHandle(handle)
    reader = threading.Thread(target=release_reader)
    reader.start()
    v.commit_status(pending, status)
    reader.join()
else:
    replace = Path.replace
    failures = [0]
    def transient(path, target):
        """⏳ Reproduce transient sharing denial on other platforms."""
        failures[0] += 1
        if failures[0] < 3:
            raise PermissionError("reader")
        return replace(path, target)
    with patch.object(Path, "replace", transient):
        v.commit_status(pending, status)
assert status.read_text() == cases["status_retry"]["content"] and not pending.exists()
pending.write_text("blocked", encoding="utf-8")
with patch.object(Path, "replace", side_effect=PermissionError("permanent")) as blocked, patch.object(v.time, "sleep"):
    try:
        v.commit_status(pending, status)
    except PermissionError:
        pass
    else:
        raise AssertionError("Permanent status denial was swallowed")
    assert blocked.call_count == cases["status_retry"]["attempts"]
assert pending.exists() and status.read_text() == cases["status_retry"]["content"]
race_path = v.BASE / "race/chapters/Window.mp4"
race_path.parent.mkdir(parents=True, exist_ok=True)
for path in (race_path, race_path.with_suffix(".render.json"), race_path.with_suffix(".chapter.json")):
    path.write_bytes(b"fixture")
for kind in cases["publication_races"]:
    target = {"media": race_path, "render": race_path.with_suffix(".render.json"), "clock": race_path.with_suffix(".chapter.json")}[kind]
    def changed():
        """🔀 Simulate atomic publisher replacement during a failed decode."""
        target.write_bytes(target.read_bytes() + b"changed")
        raise AssertionError("fixture decode")
    assert v.verify_stable([race_path], changed) is None
try:
    v.verify_stable([race_path], lambda: (_ for _ in ()).throw(AssertionError("stable defect")))
except AssertionError as error:
    assert str(error) == "stable defect"
else:
    raise AssertionError("Stable verification defect was swallowed")
records, values, inputs = {}, {}, {}
for case in cases["chapters"]:
    scene, position = case["scene"], case["position"]
    source = v.BASE / "s" / str(position) / "v/Source.mp4"
    raw = v.BASE / str(position) / "v" / (scene + ".mp4")
    media.encode(source, cases["body_values"])
    media.encode(raw, cases["body_values"])
    authored = {"scene": scene, "settings": {}, "segments": []}
    raw.with_suffix(".render.json").write_text(json.dumps(authored), encoding="utf-8")
    entry = {"chapter": scene, "segment": {"script": "fixture.py", "scene": "Source"}, "source": source, "selected": source}
    clock = v.clock(scene, len(cases["body_values"]), case["index"])
    window = ([cases["boundary_value"]] * (clock["focus_frames"] - 1) + [cases["body_values"][0]]
              + cases["body_values"] + [cases["body_values"][-1]]
              + [cases["boundary_value"]] * (clock["restore_frames"] - 1))
    output = v.chapter_path(scene)
    media.encode(output, window, audio=True)
    output.with_suffix(".chapter.json").write_text(json.dumps(clock), encoding="utf-8")
    output.with_suffix(".render.json").write_text(json.dumps({"content": authored,
        "presentation": {"catalog": {"courses": [{"id": "architecture"}]}}}), encoding="utf-8")
    inputs[scene] = (case, authored, entry)
    values[scene] = window
    records[scene] = v.verify_chapter({"id": case["clip"]}, position, authored, [entry], case["index"])
assert len(v.joins(records)) == 1
names = [case["scene"] for case in cases["chapters"]]
lesson = {"id": "joined", "output": "Joined"}
final = v.BASE / "rendered/Joined_spectator_de_480x270p4.mp4"
combined = values[names[0]] + values[names[1]]
media.encode(final, combined, audio=True)
final.with_suffix(".render.json").write_text(json.dumps({"presentation": {"catalog": {"courses": [{"id": "architecture"}]}},
    "content": {"chapters": [{"scene": name} for name in names]}}), encoding="utf-8")
assert v.verify_lesson(lesson, [records[name] for name in names])["pass"]
rejected = []
first = cases["chapters"][0]
scene = first["scene"]
case, authored, entry = inputs[scene]
output = v.chapter_path(scene)
original_clock = v.read(output.with_suffix(".chapter.json"))
bad = list(values[scene])
bad[original_clock["focus_frames"] + 1] = 220
media.encode(output, bad, audio=True)
try:
    v.verify_chapter({"id": case["clip"]}, case["position"], authored, [entry], case["index"])
except AssertionError:
    rejected.append("body-overlay")
bad = list(values[scene])
sparse = media.np.full((270, 480, 3), cases["body_values"][1], dtype=media.np.uint8)
sparse[135, 240] = 220
bad[original_clock["focus_frames"] + 1] = sparse
media.encode(output, bad, audio=True)
try:
    v.verify_chapter({"id": case["clip"]}, case["position"], authored, [entry], case["index"])
except AssertionError:
    rejected.append("sparse-body-overlay")
assert "sparse-body-overlay" in rejected, "Verifier accepted a sparse body overlay"
media.encode(output, values[scene], audio=True)
clock = {**original_clock, "focus_frames": original_clock["focus_frames"] + 1}
output.with_suffix(".chapter.json").write_text(json.dumps(clock), encoding="utf-8")
try:
    v.verify_chapter({"id": case["clip"]}, case["position"], authored, [entry], case["index"])
except AssertionError:
    rejected.append("clock-mismatch")
output.with_suffix(".chapter.json").write_text(json.dumps(original_clock), encoding="utf-8")
loaded = time.time() - .5
publisher = {"state": "current", "loaded": loaded, "pid": os.getpid(),
             "sources": {str(TICKET / "inspection-publish.py"): v.STREAMS.sha(TICKET / "inspection-publish.py")}}
status = {case["clip"]: {"state": "running", "started": time.time(), "attempt": 1}}
def publication_state():
    """📜 Commit explicit publisher epoch/status witnesses in the isolated fixture."""
    (v.PUBLICATION / "publish-owner.json").write_text(json.dumps(publisher), encoding="utf-8")
    (v.PUBLICATION / "publish.json").write_text(json.dumps(status), encoding="utf-8")
def verify_window():
    """🎞️ Exercise the same strict real-media clock consumer used by the watcher."""
    return v.verify_chapter({"id": case["clip"]}, case["position"], authored, [entry], case["index"])
readiness = []
publication_state()
output.with_suffix(".chapter.json").write_text(json.dumps(clock), encoding="utf-8")
assert v.verify_stable([output], verify_window) is None
readiness.append("pending")
output.with_suffix(".chapter.json").write_text(json.dumps(original_clock), encoding="utf-8")
assert verify_window()["pass"]
readiness.append("committed")
for label in ("completed-wrong", "dead-writer", "old-epoch", "stale-owner"):
    publisher.update(state="current", loaded=loaded, pid=os.getpid())
    status[case["clip"]].update(state="running", started=time.time())
    if label == "completed-wrong":
        status[case["clip"]]["state"] = "completed"
    elif label == "dead-writer":
        publisher["pid"] = 2147483647
    elif label == "old-epoch":
        status[case["clip"]]["started"] = loaded - 1
    else:
        publisher["state"] = "restart_required"
    publication_state()
    output.with_suffix(".chapter.json").write_text(json.dumps(clock), encoding="utf-8")
    try:
        v.verify_stable([output], verify_window)
    except AssertionError:
        readiness.append(label)
    else:
        raise AssertionError("Invalid clock was accepted or deferred: " + label)
assert readiness == cases["clock_readiness_controls"]
output.with_suffix(".chapter.json").write_text(json.dumps(original_clock), encoding="utf-8")
media.encode(output, values[scene][:-1] + [64], audio=True)
changed = v.verify_chapter({"id": case["clip"]}, case["position"], authored, [entry], case["index"])
try:
    v.joins({scene: changed, names[1]: records[names[1]]})
except AssertionError:
    rejected.append("boundary-mismatch")
media.encode(output, values[scene], audio=True)
for label, sequence in (("chapter-order", values[names[1]] + values[names[0]]), ("missing-frame", combined[:-1])):
    media.encode(final, sequence, audio=True)
    try:
        v.verify_lesson(lesson, [records[name] for name in names])
    except AssertionError:
        rejected.append(label)
fixture = v.BASE / "source.py"
fixture.write_text("version one", encoding="utf-8")
certificate = v.read(output.with_suffix(".render.json"))
certificate["presentation"]["sources"] = {str(fixture): hashlib.sha256(fixture.read_bytes()).hexdigest()}
output.with_suffix(".render.json").write_text(json.dumps(certificate), encoding="utf-8")
assert v.current_certificate(output)
fixture.write_text("version two", encoding="utf-8")
assert not v.current_certificate(output)
rejected.append("stale-source")
cached_chapters = {scene: records[scene]}
cached_lessons = {case["clip"]: {"pass": True}, "unrelated": {"pass": True}}
assert v.chapter_identity(output, scene, case["clip"], cached_chapters, cached_lessons) is None
assert scene not in cached_chapters and case["clip"] not in cached_lessons and "unrelated" in cached_lessons
rejected.append("stale-record")
fixture.write_text("version one", encoding="utf-8")
marker = v.BASE / "publish-complete.json"
marker.write_text(json.dumps({"state": "complete", "completed": time.time()}), encoding="utf-8")
marker.with_name("publish-gate.json").write_text(json.dumps({"state": "passed"}), encoding="utf-8")
helper = TICKET / "inspection-publish.py"
owner = {"state": "current", "loaded": time.time() - 1, "pid": 1,
         "sources": {str(helper): v.STREAMS.sha(helper), str(fixture): v.STREAMS.sha(fixture)}}
owner_path = marker.with_name("publish-owner.json")
owner_path.write_text(json.dumps(owner), encoding="utf-8")
record = {**records[scene], "certificate_sha256": v.STREAMS.sha(output.with_suffix(".render.json"))}
assert v.verify_provenance([record], marker)["pass"]
owner["state"] = "restart_required"
owner_path.write_text(json.dumps(owner), encoding="utf-8")
try:
    v.verify_provenance([record], marker)
except AssertionError:
    rejected.append("stale-owner")
owner["state"] = "current"
owner["sources"][str(helper)] = "0" * 64
owner_path.write_text(json.dumps(owner), encoding="utf-8")
try:
    v.verify_provenance([record], marker)
except AssertionError:
    rejected.append("stale-loaded-helper")
assert rejected == cases["negative_controls"], rejected
(TICKET / "inspection-chapter-verifier-test.md").write_text(
    "# Independent Collection Verifier Controls\n\nTwo finite chapter windows and their exact ordered lesson passed complete decode, frame clocks, exact body pixels, silent AAC and adjacent LCA pixels. PyAV independently encoded every fixture. Ten deliberate regressions were rejected: full body overlay, sparse one-pixel body overlay, false clock, wrong boundary, reversed chapter order, missing frame, stale source binding, stale in-memory chapter/lesson retention, stale publisher owner and stale loaded helper. The sparse overlay was accepted by the previous mean-error rule (RED); exact decoded array equality rejects it (GREEN). A valid owner/source/contract snapshot also passed independent before/after provenance hashing. Three actual publication identity changes were retried; stable failures remained terminal. Native Windows reader sharing denial was bounded without accepting an incomplete status commit.\n\nSix clock readiness controls passed using real PyAV media and the actual native test process as the writer: a mismatched clock under a stable, current, live running publisher epoch was withheld; the matching committed clock passed strict decode immediately. Completed-wrong, dead-writer, old-epoch and stale-owner clocks all remained assertion failures. The previous consumer failed the pending case (RED). No clock tolerance or grace period was introduced. All test writes were confined to generated/iv/collection-selftest and this report. Production paths were not modified by this test.\n", encoding="utf-8")
print("[DEBUG] Two chapter fixtures, exact lesson, ten negative controls and six clock readiness controls passed", flush=True)
