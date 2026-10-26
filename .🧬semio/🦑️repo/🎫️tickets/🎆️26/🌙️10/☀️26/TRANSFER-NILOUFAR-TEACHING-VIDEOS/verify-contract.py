"""📐 Validate the curriculum with the independent JSON Schema implementation."""
import json
import sys
from pathlib import Path
import jsonschema

root = Path(sys.argv[1])
schema = json.loads((root / "video_manifest.schema.json").read_text(encoding="utf-8"))
manifest = json.loads((root / "video_manifest.json").read_text(encoding="utf-8"))
jsonschema.Draft202012Validator.check_schema(schema)
jsonschema.Draft202012Validator(schema).validate(manifest)
assert len(manifest["videos"]) == 5
assert sum(len(video["chapters"]) for video in manifest["videos"]) == 26
assert sum(len(chapter["segments"]) for video in manifest["videos"] for chapter in video["chapters"]) == 112
print("Independent JSON Schema validation passed: five videos, 26 chapters, 112 authored beats")
