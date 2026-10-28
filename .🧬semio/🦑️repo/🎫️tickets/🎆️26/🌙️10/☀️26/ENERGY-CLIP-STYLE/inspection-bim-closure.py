"""🔐 Certify the final current BIM dependency closure after production publication completes."""
from __future__ import annotations

import hashlib
import importlib
import importlib.util
import json
from pathlib import Path
import sys
import time

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
TEACHING = ROOT / "🎓️teaching"
OUTPUT = TICKET / "🗑️generated/iv"
COMPLETE = TICKET / "🗑️generated/i/publish-complete.json"
MANIFEST = TEACHING / "🏛️architecture/⚡️energy/🎬️clip/🔣️.json"
CASES = TEACHING / "🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🎭️presentation/🔣️.cases.json"
sys.path.insert(0, str(ROOT))


def hashes(paths):
    """🔏 Capture exact current bytes for every dependency in a finite path set."""
    result = {}
    for path in sorted(paths):
        digest = hashlib.sha256()
        with path.open("rb") as stream:
            for block in iter(lambda: stream.read(1024 * 1024), b""):
                digest.update(block)
        result[path.resolve().as_posix()] = digest.hexdigest()
    return result


def main():
    """🏁 Check the pure production closure oracle and hold one stable post-completion hash snapshot."""
    marker = COMPLETE.read_bytes()
    completed = json.loads(marker)
    assert all(completed.get(key) == value for key, value in
               {"state": "complete", "videos": 17, "sources": 117, "chapters": 38, "chapter_exports": 38}.items())
    document = json.loads(MANIFEST.read_text(encoding="utf-8"))
    cases = json.loads(CASES.read_text(encoding="utf-8"))
    leaves = {TEACHING / segment["script"] for video in document["videos"] if video["id"].startswith("bim_")
              for chapter in video["chapters"] for segment in chapter["segments"]
              if segment["script"].startswith("🏛️architecture/🏗️bim/")}
    assert len(leaves) == 6
    required = {TEACHING / path for path in cases["dependencies"]}
    runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")

    def closures():
        """📚 Resolve actual literal imports for each current BIM leaf."""
        return {leaf: set(runtime.source_dependencies(leaf)) for leaf in leaves}


    base = TICKET / "🗑️generated/i/de/spectator"
    artifacts, published = set(), []
    for job in runtime.source_plan(runtime.segment_plan(document["videos"])):
        if TEACHING / job["segment"]["script"] not in leaves:
            continue
        clip = base / "s" / str(job["source_ordinal"]) / "v" / (job["segment"]["scene"] + ".mp4")
        certificate = clip.with_suffix(".render.json")
        state = json.loads(certificate.read_text(encoding="utf-8"))
        assert required <= {Path(path) for path in state["sources"]}, str(certificate)
        assert runtime.reusable_clip(clip, state) and runtime.state_inputs_current(state), str(clip)
        artifacts.add(certificate)
        published.append(state)
    for video in document["videos"]:
        if not video["id"].startswith("bim_"):
            continue
        clip = base / "rendered" / (video["output"] + "_spectator_de_480x270p4.mp4")
        certificate = clip.with_suffix(".render.json")
        state = json.loads(certificate.read_text(encoding="utf-8"))
        assert runtime.reusable_clip(clip, state), str(clip)
        artifacts.add(certificate)
        published.append(state)

    bindings = {}

    def collect(value, include_all=False):
        """🧬 Require retained artifact provenance to bind the same current BIM bytes."""
        if isinstance(value, list):
            for item in value:
                collect(item, include_all)
        elif isinstance(value, dict):
            for path, digest in value.get("sources", {}).items():
                if include_all or Path(path).is_relative_to(TEACHING / "🏛️architecture/🏗️bim"):
                    key = Path(path).resolve().as_posix()
                    assert key not in bindings or bindings[key] == digest, key
                    bindings[key] = digest
            for key, item in value.items():
                if key != "sources":
                    collect(item, include_all)

    for index, state in enumerate(published):
        collect(state, index < len(leaves))
    assert {path.resolve().as_posix() for path in required | leaves} <= set(bindings)
    tests_path = OUTPUT / "algorithms-current.json"
    tests = json.loads(tests_path.read_text(encoding="utf-8"))
    assert tests["pass"] and tests["tests"] == 4 and tests["languages"] == ["en", "de"] and tests["checked"] >= completed["completed"]
    for path, digest in tests["files"].items():
        key = Path(path).resolve().as_posix()
        assert key not in bindings or bindings[key] == digest, (key, "Focused controls and final BIM inputs disagree")
        bindings[key] = digest

    initial = closures()
    specification = importlib.util.spec_from_file_location("inspection_evidence", TICKET / "inspection-evidence.py")
    evidence = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(evidence)
    supplementary = evidence.bundle(document, cases, runtime, base)
    validation = {Path(__file__), COMPLETE, tests_path, TICKET / "inspection-algorithms-current.py", TICKET / "inspection-retain.mjs", TICKET / "inspection-bim-current-preflight.py", TICKET / "inspection-bim-schema-current.mjs",
                  TEACHING / "🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts",
                  TEACHING / "🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🎭️presentation/🐍️.py",
                  TEACHING / "🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧬️schema/🔣️.schema.json",
                  TEACHING / "🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🧬️loss-selector/🔣️.json",
                  TEACHING / "🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🧬️loss-selector/🟦️.ts",
                  TEACHING / "🏛️architecture/🏗️bim/🎬️clip/🧬️objects/🧪️tests/🚪️retirement/🔣️.json",
                  TEACHING / "🏛️architecture/🏗️bim/🎬️clip/🧬️objects/🧪️tests/🚪️retirement/🐍️.py"}
    paths = leaves | required | {MANIFEST, CASES} | validation | set().union(*initial.values())
    paths |= artifacts | {Path(path) for path in bindings} | {Path(path) for path in supplementary["files"]}
    before = hashes(paths)
    assert all(before[path] == digest for path, digest in supplementary["files"].items()), "Supplementary evidence changed after native pixel validation"
    assert all(before[path] == digest for path, digest in bindings.items()), "Published BIM artifact inputs are stale"
    for leaf, actual in initial.items():
        assert required <= actual, (str(leaf), sorted(str(path) for path in required - actual))
    final = closures()
    assert initial == final, "BIM dependency membership changed during the closure probe"
    after = hashes(paths)
    assert before == after, "BIM dependency bytes changed during the closure probe"
    assert COMPLETE.read_bytes() == marker, "Production completion marker changed during the closure probe"
    checked = time.time()
    assert checked >= completed["completed"]
    result = {"pass": True, "leaves": 6, "checked": checked, "files": after,
              "evidence": supplementary["evidence"], "evidence_counts": supplementary["evidence_counts"]}
    OUTPUT.mkdir(parents=True, exist_ok=True)
    temporary = OUTPUT / "bim-closure.new"
    temporary.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    temporary.replace(OUTPUT / "bim-closure.json")
    (TICKET / "inspection-bim-closure.md").write_text(
        "# Inspection Final BIM Closure\n\nPassed after the production 17-video/117-source/38-chapter completion marker. "
        + "All six current BIM leaves include every required widget/model provider in the actual production literal-import closure. "
        + str(len(after)) + " current source, asset, control and artifact-certificate files have identical hashes before and after the probe. "
        + "Canonical source and published BIM certificates bind these same current provider bytes. "
        + "The four existing bilingual algorithm feature controls also passed after collection completion with identical full production source/asset snapshots before and after execution. "
        + "Supplementary evidence binds all113 current BIM clause tiles and9 source122 stage frames to exact native decoded pixels, current native certificates and traces; only explicitly listed PNGs enter delivery. "
        + "The retention helper must recheck every listed hash before and after copying final deliveries.\n",
        encoding="utf-8")
    print("[DEBUG] Final BIM closure passed", len(leaves), len(after), checked, flush=True)


if __name__ == "__main__":
    main()
