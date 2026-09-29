#!/usr/bin/env python3
"""🩹️ L1 pre-chain test-only hotfix (rule 22) for C12's re-landed splice set: the writer concurrent-typing law imports
`settle_framework_reserved_admission` from `artifact_app_laws`, but the SDK exports it from `app` (as puzzle 3d's laws use it).
usage: l1-c12-test-import.py [--write]   (default dry run; idempotent)"""
import sys
from pathlib import Path

PATH = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/✂️concurrent-typing/🦀️.rs")
OLD = "use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, paired_registered_apps_with_members, settle_framework_reserved_admission, settle_registered_typed_operation};\n"
NEW = "use semio_framework_plugin::app::settle_framework_reserved_admission;\nuse semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, paired_registered_apps_with_members, settle_registered_typed_operation};\n"
text = PATH.read_text(encoding="utf-8")
if NEW in text:
    print("applied")
elif text.count(OLD) != 1:
    print(f"PROBLEM anchor found {text.count(OLD)}x")
    sys.exit(1)
elif "--write" in sys.argv:
    PATH.write_text(text.replace(OLD, NEW), encoding="utf-8")
    print("written")
else:
    print("dry-run: 1 hunk, 0 problems")
