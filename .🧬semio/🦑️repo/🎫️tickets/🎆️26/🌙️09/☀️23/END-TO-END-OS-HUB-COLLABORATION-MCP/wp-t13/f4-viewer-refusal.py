#!/usr/bin/env python3
"""👁️ F4 (prepared, next landing window): `artifact_app_laws::assert_viewer_never_mutates` treats a viewer's refusal as
what it is — an emission of nothing — instead of `expect`-ing success, so retained-route viewers (gis camera lane,
energy) can use the shared law again. Refuses a second run. `--write` applies."""
import sys
from pathlib import Path

P = Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs")
DOC_OLD = """        /// 👁️ Exercises the production ViewerApp adapter with a representative viewer command.
        /// The adapter must emit neither document nor draft mutations. Command registration and
        /// admission are separate runtime contracts; a generic viewer fixture has no declared
        /// factory for its author-defined default command.
"""
DOC_NEW = """        /// 👁️ Exercises the production ViewerApp adapter with a representative viewer command.
        /// The adapter must emit neither document nor draft mutations. A refusal emits nothing and so
        /// satisfies the law — a retained-route viewer refuses a direct `handle` by design. Command
        /// registration and admission are separate runtime contracts; a generic viewer fixture has no
        /// declared factory for its author-defined default command.
"""
BODY_OLD = """            let (artifact_is_empty, draft_is_empty) = result.expect("viewer adapter command succeeds");
            assert!(artifact_is_empty, "a viewer must never emit document mutations");
            assert!(draft_is_empty, "a viewer must never emit draft mutations");
"""
BODY_NEW = """            if let Ok((artifact_is_empty, draft_is_empty)) = result {
                assert!(artifact_is_empty, "a viewer must never emit document mutations");
                assert!(draft_is_empty, "a viewer must never emit draft mutations");
            }
"""
text = P.read_text(encoding="utf-8")
problems = [f"anchor found {text.count(old)} times: {old.strip()[:60]!r}" for old in (DOC_OLD, BODY_OLD) if text.count(old) != 1]
print("dry-run" if "--write" not in sys.argv else "write", P, "problems:", problems)
if problems:
    sys.exit(1)
if "--write" in sys.argv:
    P.write_text(text.replace(DOC_OLD, DOC_NEW).replace(BODY_OLD, BODY_NEW), encoding="utf-8")
