"""🧪 Run the existing four native algorithm controls under a complete stable-input snapshot."""
import hashlib
import importlib
import importlib.util
import json
from pathlib import Path
import sys
import time
import unittest

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
TEACHING = ROOT / "🎓️teaching"
sys.path.insert(0, str(ROOT))


def sha(path):
    """🔏 Hash finite current test inputs with bounded reads."""
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(block)
    return value.hexdigest()


def main():
    """🏁 Keep current native feature results separate from historical moving-input runs."""
    marker = TICKET / "🗑️generated/i/publish-complete.json"
    completed = marker.read_bytes()
    assert all(json.loads(completed).get(key) == value for key, value in
               {"state": "complete", "videos": 17, "sources": 117, "chapters": 38, "chapter_exports": 38}.items())
    output = TICKET / "🗑️generated/iv/algorithms-current.json"
    temporary = output.with_suffix(".new")
    temporary.write_text(json.dumps({"pass": False, "state": "running", "started": time.time()}) + "\n", encoding="utf-8")
    temporary.replace(output)
    manifest = TEACHING / "🏛️architecture/⚡️energy/🎬️clip/🔣️.json"
    video = next(item for item in json.loads(manifest.read_text(encoding="utf-8"))["videos"] if item["id"] == "optimization_algorithms")
    segment = video["chapters"][0]["segments"][0]
    source = TEACHING / segment["script"]
    controls = {Path(__file__), manifest, marker}
    drivers = []
    for name in ("🧩️widgets", "📈️comparison"):
        owner = source.parent / "🧪️tests" / name
        drivers.append(owner / "🐍️.py")
        controls.update({owner / "🐍️.py", owner / "🔣️.json"})
    runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")

    def states():
        """🌐 Capture both explicit language contexts and every actual production asset binding."""
        return {language: runtime.segment_state(segment, {"width": 480, "height": 270, "fps": 4,
                "audience": "spectator", "language": language}) for language in ("en", "de")}

    initial = states()
    before = {path.resolve().as_posix(): sha(path) for path in controls}
    suite = unittest.TestSuite()
    for index, path in enumerate(drivers):
        specification = importlib.util.spec_from_file_location("final_algorithm_control_" + str(index), path)
        module = importlib.util.module_from_spec(specification)
        sys.modules[specification.name] = module
        specification.loader.exec_module(module)
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(module))
    started = time.time()
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    assert result.wasSuccessful() and result.testsRun == 4
    assert initial == states(), "Algorithm dependency membership or bytes changed during the native controls"
    assert before == {path.resolve().as_posix(): sha(path) for path in controls}, "Algorithm controls changed while executing"
    bindings = dict(before)
    pending = list(initial.values())
    while pending:
        value = pending.pop()
        if isinstance(value, dict):
            for key, item in value.items():
                if key in ("sources", "assets") and isinstance(item, dict):
                    for path, digest in item.items():
                        name = Path(path).resolve().as_posix()
                        assert name not in bindings or bindings[name] == digest, name
                        bindings[name] = digest
                pending.append(item)
        elif isinstance(value, list):
            pending.extend(value)
    assert all(sha(Path(path)) == digest for path, digest in bindings.items())
    assert marker.read_bytes() == completed
    checked = time.time()
    assert checked >= json.loads(completed)["completed"]
    payload = {"pass": True, "tests": result.testsRun, "languages": ["en", "de"], "checked": checked,
               "seconds": checked - started, "files": bindings}
    temporary.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    temporary.replace(output)
    with (TICKET / "inspection-algorithms-lifecycle.md").open("a", encoding="utf-8") as report:
        report.write("\n## Final Stable Current Feature Controls\n\nAll4 existing native controls passed in both explicit locales after real collection completion. "
                     + str(len(bindings)) + " source, asset, timing and control bindings matched before/after the run and a final full rehash. "
                     + "No production file was written. Checked " + str(checked) + ", elapsed " + str(payload["seconds"]) + " seconds.\n")
    print("[DEBUG] Final current algorithm controls passed", result.testsRun, len(bindings), checked, flush=True)


if __name__ == "__main__":
    main()
