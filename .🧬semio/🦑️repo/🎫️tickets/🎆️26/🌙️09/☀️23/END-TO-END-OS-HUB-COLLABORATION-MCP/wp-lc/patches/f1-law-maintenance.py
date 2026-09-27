#!/usr/bin/env python3
"""🌡️ LC — prepared for the next landing window (no guest-linked edits during the B3 rebuild): the F1 typing-run laws drive
the displaced-owner maintenance the plugin runtime spends on a pressured instance. Measured 27 10:41 (`s13-lc-laws/writer.txt`):
writer's law failed after N keystrokes with `artifact store displaced-owner fixed retirement authority is saturated` — every
amend displaces the edit's envelope, snapshot and dag, and the live runtime drains a pressured queue in one-item bursts in
the same turn (`plugin_runtime`, `maintenance_under_pressure`), which the fixture settle does not model.
usage: python3 f1-law-maintenance.py [--dry-run]"""
import sys

ROOT = "/Users/ueli/Documents/semio/"
DRY = "--dry-run" in sys.argv
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
WRITER = "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📝️text-edit/🧪️tests/🔬️unit/🦀️.rs"
JACK = "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
VCS = "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
HELPER = '''        /// 🌡️ Spends the maintenance the plugin runtime spends on a pressured instance: while a displaced-owner queue of the
        /// document or a config-lane store sits at or above `store::ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`, the runtime
        /// drains it in one-item steps of `plugin_runtime::RUNTIME_CLOSE_BYTES_PER_STEP` within the same turn. A law that
        /// commits faster than the fair rotation retires (a typing run: every keystroke amends one edit and displaces its
        /// envelope, snapshot and dag) calls this after each settled command, as the live host does.
        pub fn drain_maintenance_pressure<P: PluginApp>(app: &mut P) {
            let mut idle_stages = 0;
            while app.maintenance_under_pressure() {
                match PluginApp::maintenance_step(app, 1, crate::plugin_runtime::RUNTIME_CLOSE_BYTES_PER_STEP).expect("bounded maintenance") {
                    crate::app::PluginCloseStep::Pending { released_items, released_bytes } if released_items > 0 || released_bytes > 0 => idle_stages = 0,
                    _ => idle_stages += 1,
                }
                assert!(idle_stages <= usize::from(super::MAINTENANCE_STAGES), "a pressured displaced-owner queue made no maintenance progress over a whole stage rotation");
            }
        }

'''
EDITS = [
    (SDK, "    const RUNTIME_CLOSE_BYTES_PER_STEP: usize = 32 * 1_024;\n", "    pub(crate) const RUNTIME_CLOSE_BYTES_PER_STEP: usize = 32 * 1_024;\n"),
    (SDK, "        /// 🧪️ Replays `seed_genesis_children`'s roster lookup", HELPER + "        /// 🧪️ Replays `seed_genesis_children`'s roster lookup"),
    (WRITER, "        dispatch(&mut app, WriterCommand::TextEdit(super::TextEdit { text: text.clone() })).await;\n",
     "        dispatch(&mut app, WriterCommand::TextEdit(super::TextEdit { text: text.clone() })).await;\n        semio_framework_plugin::artifact_app_laws::drain_maintenance_pressure(&mut *app);\n"),
    (JACK, "        drive_query_ownership_operations(&mut app).await.unwrap_or_else(|error| panic!(\"keystroke {index} did not publish: {error}\"));\n",
     "        drive_query_ownership_operations(&mut app).await.unwrap_or_else(|error| panic!(\"keystroke {index} did not publish: {error}\"));\n        artifact_app_laws::drain_maintenance_pressure(&mut app.app);\n"),
    (VCS, "        assert!(typed.edited_document(), \"keystroke {index} was not saved\");\n",
     "        assert!(typed.edited_document(), \"keystroke {index} was not saved\");\n        semio_framework_plugin::artifact_app_laws::drain_maintenance_pressure(&mut *instance);\n"),
]
texts, problems = {}, []
for path, old, new in EDITS:
    text = texts.get(path) or open(ROOT + path, encoding="utf-8").read()
    if text.count(old) != 1:
        problems.append(f"{path}: anchor found {text.count(old)}x — {old[:80]!r}")
        continue
    texts[path] = text.replace(old, new)
if problems:
    print("\n".join(problems))
    sys.exit(1)
print(f"{'dry run clean' if DRY else 'applied'}: {len(EDITS)} hunks in {len(texts)} files")
if not DRY:
    for path, text in texts.items():
        open(ROOT + path, "w", encoding="utf-8").write(text)
