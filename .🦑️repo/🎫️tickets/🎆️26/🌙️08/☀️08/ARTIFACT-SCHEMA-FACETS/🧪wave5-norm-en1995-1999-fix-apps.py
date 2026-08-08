#!/usr/bin/env python3
from __future__ import annotations

import subprocess
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
APPS = REPO / "✏️s/🔌️plugins/📕️norm/🎛️apps"

KEYS = ["en1995", "en1996", "en1997", "en1998", "en1999"]


def git_show(path: str) -> str:
    r = subprocess.run(["git", "-C", str(REPO), "show", f"HEAD:{path}"], capture_output=True, text=True)
    return r.stdout if r.returncode == 0 else ""


def xform(text: str, key: str, prefix: str) -> str:
    t = text.replace("use crate::artifacts::" + key + ".Document", f"use crate::artifacts::{key}.{prefix}Snapshot")
    t = t.replace("Document", f"{prefix}Snapshot")
    t = t.replace("type Projection", "type Snapshot")
    t = t.replace("initial_projection", "initial_snapshot")
    t = t.replace("doc.projection", "doc.snapshot")
    t = t.replace("cfg.projection", "cfg.snapshot")
    t = t.replace("set_document", "set_snapshot")
    t = t.replace("SetDocument", "SetSnapshot")
    t = t.replace('"setDocument"', '"setSnapshot"')
    t = t.replace('"set-document"', '"set-snapshot"')
    t = t.replace("setDocument", "setSnapshot")
    t = t.replace("pub snapshot: En", "pub snapshot: En")  # noop
    t = t.replace(f"pub {prefix}Snapshot: {prefix}Snapshot", f"pub snapshot: {prefix}Snapshot")
    t = t.replace("commit_document(payload.snapshot.clone()", "commit_document(payload.snapshot.clone()")
    t = t.replace(
        f"use crate::document::{prefix}SnapshotMutation",
        f"use crate::artifacts::{key}::op::{prefix}Mutation",
    )
    t = t.replace(
        f"use crate::document::{prefix}Mutation",
        f"use crate::artifacts::{key}::op::{prefix}Mutation",
    )
    t = t.replace("dsl(keyword = \"set-document\")", "dsl(keyword = \"set-snapshot\")")
    t = t.replace("&SetSnapshot { snapshot:", "&SetSnapshot { snapshot:")
    return t


def main() -> None:
    for key in KEYS:
        prefix = "En" + key[2:]
        base = f"✏️s/🔌️plugins/📕️norm/🎛️apps/📘️{key}"
        files = [
            ("🦀️component.rs", f"{base}/🦀️component.rs"),
            ("🎮️commands/📤️set-document/🦀️component.rs", f"{base}/🎮️commands/📤️set-snapshot/🦀️component.rs"),
            ("🎮️commands/🧮️evaluate/🦀️component.rs", f"{base}/🎮️commands/🧮️evaluate/🦀️component.rs"),
            ("🎮️commands/☑️selected-check/🦀️component.rs", f"{base}/🎮️commands/☑️selected-check/🦀️component.rs"),
        ]
        for git_rel, out_rel in files:
            src = git_show(git_rel)
            if not src:
                continue
            out = APPS / f"📘️{key}" / out_rel.split(f"📘️{key}/", 1)[1]
            out.parent.mkdir(parents=True, exist_ok=True)
            out.write_text(xform(src, key, prefix))
        print(key)


if __name__ == "__main__":
    main()
