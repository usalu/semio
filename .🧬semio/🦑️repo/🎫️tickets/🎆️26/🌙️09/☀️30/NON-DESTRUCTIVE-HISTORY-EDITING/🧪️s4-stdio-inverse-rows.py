#!/usr/bin/env python3
"""🧩️ Declares `x-semio-inverse-rows: {bounded: 128}` (= `SNAPSHOT_PATCH_MAX_INVERSE_PARTS`) on every stdio `patch-snapshot`
leaf payload schema (every leaf implemented by `snapshot_patch_leaf!`), right after the root `title`, the convention of the
other bounded leaves. Idempotent; `--check` reports pending files and exits 1 when any is pending; `--exclude=<artifact dir>,…`
skips artifacts another owner is sweeping.

@see ../../../../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🦀️.rs
"""
import json
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
BOUND = 128
LINE = f'  "x-semio-inverse-rows": {{ "bounded": {BOUND} }},\n'


def leaf_schemas() -> list[pathlib.Path]:
    listed = subprocess.run(["git", "ls-files", "-z", "--", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"], cwd=ROOT, capture_output=True, check=True).stdout.decode().split("\0")
    sources = [ROOT / name for name in listed if name.endswith("/🦀️.rs") and ("/🩹️patch-snapshot/" in name or "/📸️snapshot/🩹️patch/" in name)]
    excluded = [part for argument in sys.argv if argument.startswith("--exclude=") for part in argument.removeprefix("--exclude=").split(",")]
    return sorted(source.parent / "🧬️schema" / "🔣️.json" for source in sources if "snapshot_patch_leaf!" in source.read_text(encoding="utf-8") and not any(f"/{name}/" in str(source) for name in excluded))


def main() -> int:
    check = "--check" in sys.argv
    pending = []
    for schema in leaf_schemas():
        text = schema.read_text(encoding="utf-8")
        declared = json.loads(text).get("x-semio-inverse-rows")
        if declared == {"bounded": BOUND}:
            continue
        if declared is not None:
            text = re.sub(r'  "x-semio-inverse-rows": [^\n]*\n', "", text, count=1)
        title = re.search(r'\n  "title": [^\n]*\n', text)
        if title is None:
            raise SystemExit(f"no root title in {schema}")
        pending.append(schema)
        if not check:
            schema.write_text(text[: title.end()] + LINE + text[title.end() :], encoding="utf-8")
            assert json.loads(schema.read_text(encoding="utf-8"))["x-semio-inverse-rows"] == {"bounded": BOUND}
    print(f"{'pending' if check else 'declared'}: {len(pending)} of {len(leaf_schemas())} patch-snapshot leaf schemas")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
