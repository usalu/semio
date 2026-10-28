"""🔎 Check the finite curriculum contract and import every declared render source."""
from __future__ import annotations

import ast
import importlib
import json
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
OUTPUT = TICKET / "🗑️generated/iv"
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(OUTPUT / "cache")
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(OUTPUT / "deps"))
sys.dont_write_bytecode = True


def main():
    """📚 Validate the schema independently before checking production source imports."""
    import jsonschema
    from manim import Scene, tempconfig

    teaching = ROOT / "🎓️teaching"
    manifest_path = teaching / "🏛️architecture/⚡️energy/🎬️clip/🔣️.json"
    contract = json.loads(manifest_path.read_text(encoding="utf-8"))
    schema = json.loads((manifest_path.parent / contract["$schema"]).read_text(encoding="utf-8"))
    jsonschema.Draft202012Validator(schema).validate(contract)
    videos = contract["videos"]
    references = {}
    for video in videos:
        references.setdefault(video["script"], set())
        for chapter in video["chapters"]:
            for segment in chapter["segments"]:
                references.setdefault(segment["script"], set()).add(segment["scene"])
    failures = []
    with tempconfig({"media_dir": str(OUTPUT / "media"), "text_dir": str(OUTPUT / "text")}):
        for relative, scenes in references.items():
            try:
                path = teaching / relative
                ast.parse(path.read_text(encoding="utf-8"), filename=str(path))
                module = importlib.import_module("🎓️teaching." + relative.removesuffix(".py").replace("/", "."))
                for name in scenes:
                    value = getattr(module, name)
                    assert isinstance(value, type) and issubclass(value, Scene), name
            except Exception as error:
                failures.append((relative, str(error)))
    runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
    assert runtime.load_manifest() == videos
    chapters = sum(len(video["chapters"]) for video in videos)
    segments = [segment for video in videos for chapter in video["chapters"] for segment in chapter["segments"]]
    clauses = [segment for segment in segments if "clauses" in segment]
    unique = {(segment["script"], segment["scene"]) for segment in segments}
    lines = ["# Inspection Preflight", "", f"Status: {'FAILED' if failures else 'passed'}.", "", f"Independent jsonschema Draft202012Validator accepted the canonical schema contract. Production load_manifest also accepted the complete catalog, including its shape, paths, identifiers and semantic clause ranges. Imported and AST-parsed {len(references)} wrapper and source modules; checked every declared scene is a Manim Scene subclass. Catalog: {len(videos)} lessons, {chapters} chapters, {len(segments)} ordered source occurrences, {len(unique)} unique canonical scenes and {len(clauses)} clause excerpts. The test-only JSON Schema validator was installed into disposable generated/iv/deps; the teaching environment and runtime dependency graph were unchanged.", "", "| Lesson | Chapters | Occurrences | Clause Cuts |", "| --- | ---: | ---: | ---: |"]
    for video in videos:
        entries = [segment for chapter in video["chapters"] for segment in chapter["segments"]]
        lines.append(f"| {video['id']} | {len(video['chapters'])} | {len(entries)} | {sum('clauses' in segment for segment in entries)} |")
    lines += ["", "Import failures:", "", *(f"- {path}: {error}" for path, error in failures)] if failures else ["", "No import, schema, source existence, declared scene or semantic clause-selection blockers were found. This bounded preflight does not execute animations; final decoded media validation covers render-time behavior."]
    (TICKET / "inspection-preflight.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("[DEBUG]", json.dumps({"lessons": len(videos), "modules": len(references), "chapters": chapters, "segments": len(segments), "unique": len(unique), "clause_excerpts": len(clauses), "failures": failures}, ensure_ascii=False), flush=True)
    raise SystemExit(bool(failures))


if __name__ == "__main__":
    main()
