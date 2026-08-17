# Important — live blockers and rules (coordinator-owned)

## Forbidden territory
- `✏️s/🔌️plugins/🗄️stdio/**` and `📜️world.wit` — the `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS` ticket is open and owns them.
- `animate`, `layout`, `note` do not build to wasm (stdio-rooted / crate-local bugs, see the predecessor's `📓️w5-d-report.md`). They stay broken here and are reported as out of scope, never "skipped".

## Verification rules
- Never `cargo check --workspace` — peers keep it red.
- Hub: default features only (`cargo check -p semio-hub`, `cargo test -p semio-hub --lib`, `--bin os-hub`). Never `--all-features`, never `bun nx run os-hub:test*`.
- After any ABI/wire change, build the real wasm target: native `cargo check` never compiles `#[cfg(target_arch = "wasm32")]` code.
- `CHANNEL_VERSION` 12 invalidates every prebuilt guest module — rebuild the e2e plugin set after P3 lands.

## Deletion checklist (must be empty by the end of W1)
`PresencePoint` · `PresenceViewport` · `PresencePeer.cursor` · `PresencePeer.viewport` · `presence_hue_for_actor` · `presenceHueForActor` · `surface_fanout` · `surface_fanout_for` · `KNOWN_ARTIFACT_KINDS` · `known_artifact_kind` · the dead `host::presence_peers_json` helper.

## Known live-tree hazards
- Auto-commit + concurrent sessions: re-read every region right before editing; attribute red builds with `git log --date=iso -- <file>` (commit *messages* carry a frozen fake date).
- `PluginRuntime/🟦️component.tsx` and `🔌️plugin/🦀️component.rs` have been mid-rewrite by peer sessions twice in this ticket family — check `git status` before editing.
