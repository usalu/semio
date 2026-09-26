#!/usr/bin/env python3
"""⌨️ F1 — lands the typing-coalescing patch set (post-publish window only): the SDK typing-run fixture + helper, writer's law,
trinity jack's coalesce fix + law, vcs's amend fix. `--dry-run` checks every anchor and target without writing."""
import os, sys, shutil

ROOT = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
DRY = "--dry-run" in sys.argv
PLUGIN = f"{ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
WRITER = f"{ROOT}/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📝️text-edit/🧪️tests/🔬️unit/🦀️.rs"
JACK = f"{ROOT}/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
VCS_EDIT = f"{ROOT}/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️edit/🦀️.rs"
problems, files = [], {}

def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()

def current(path):
    return files.get(path) if path in files else read(path)

def insert_before(path, anchor, text):
    source = current(path)
    if source.count(anchor) != 1:
        problems.append(f"{path}: anchor found {source.count(anchor)}× — {anchor[:70]!r}")
        return
    files[path] = source.replace(anchor, text + anchor)

def replace(path, old, new):
    source = current(path)
    if source.count(old) != 1:
        problems.append(f"{path}: old text found {source.count(old)}× — {old[:70]!r}")
        return
    files[path] = source.replace(old, new)

fixture_target = f"{PLUGIN}/🧫️fixtures/⌨️typing-run/🔣️.json"
if os.path.exists(fixture_target):
    problems.append(f"{fixture_target} already exists")
insert_before(f"{PLUGIN}/🦀️.rs", "        /// 🧪️ Replays `seed_genesis_children`'s roster lookup", read(f"{HERE}/sdk-typing-run-helper.rs"))
insert_before(WRITER, "#[semio_framework_async_macros::async_test]\nasync fn format_artifact_reformats_jack_query()", read(f"{HERE}/writer-law.rs").lstrip("\n") + "\n")
files[f"{JACK}/🎮️commands/✏️text-edit/🦀️.rs"] = read(f"{HERE}/jack-text-edit.rs")
insert_before(f"{JACK}/🧪️tests/🔬️unit/🦀️.rs", "#[semio_framework_async_macros::async_test]\nasync fn graph_scene_has_lod_json() {", read(f"{HERE}/jack-law.rs"))
replace(VCS_EDIT, "                Emit::mutations(operations)\n", "                Emit::amend(operations, VCS_TEXT_TYPING_COALESCE_KEY)\n")
VCS_EDITOR = f"{ROOT}/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
replace(f"{VCS_EDITOR}/🦀️.rs", "                return Ok(Some(if mutations.is_empty() { Emit::default() } else { Emit::mutations(mutations) }));\n", "                return Ok(Some(if mutations.is_empty() { Emit::default() } else { Emit::amend(mutations, crate::editor::vcs::commands::edit::VCS_TEXT_TYPING_COALESCE_KEY) }));\n")
insert_before(f"{VCS_EDITOR}/🧪️tests/🔬️unit/🦀️.rs", "//#endregion 🔖️CommandSurface\n", read(f"{HERE}/vcs-law.rs").lstrip("\n") + "\n")
insert_before(VCS_EDIT, "pub(crate) fn text_edit_operations(", "/// ⌨️ The coalesce key of a typing run in the vcs text editor: every keystroke amends the run's one edit, so typing is one\n/// undo step and never spends the store's fixed applied-edit ledger one keystroke at a time (ticket 26/09/23 F1).\npub(crate) const VCS_TEXT_TYPING_COALESCE_KEY: &str = \"vcs-text-typing\";\n\n")

if problems:
    print("\n".join(problems))
    sys.exit(1)
print(f"{'dry run' if DRY else 'applying'}: {len(files)} files + 1 new fixture")
for path in files:
    print("  ", path.replace(ROOT + "/", ""))
if not DRY:
    os.makedirs(os.path.dirname(fixture_target), exist_ok=True)
    shutil.copyfile(f"{HERE}/typing-run.fixture.json", fixture_target)
    for path, text in files.items():
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(text)
