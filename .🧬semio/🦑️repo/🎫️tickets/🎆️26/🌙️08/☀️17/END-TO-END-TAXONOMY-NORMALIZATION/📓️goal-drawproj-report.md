# Draw Editor-Command Projection — Gate Cleared

Performed the un-executed `artifact-editor-command-bundle-v1` projection (11 moves,
`rationaleRule: artifact-editor-command-projection-v1`) from `🔣️draw-plan.json`: moved
`canvas-pointer-down`'s fsm/macros tree from `🏅️standards/🔖️1/🪆️subsets/✳️any/...` to
`✏️editor/🪆️1-any/...`, kind-only leaves (`component.rs`→`.rs`, `glue.rs`→`library/.rs`), applied
all embedded `referenceEdits`.

Fixed the 5 stale consumer contracts (draw-package-cargo, draw-package-library,
draw-workspace-cargo, draw-dependency-registry, draw-workspace-script): plugin `Cargo.toml`,
plugin `glue.rs`, root `Cargo.toml` (workspace members), root `🔒️dependencies.json` (8 refs),
root `📜️script.ts`.

Also fixed relative-path arithmetic the plan's `referenceEdits` didn't cover (directory depth
dropped by 3): `../..` counts in fsm/macros `Cargo.toml` (dispatch-macros, async-macros deps),
`project.json` ($schema, cwd), `script.ts` (framework import) — all now resolve.

Verification: baseline commit `bb06c41f`; `clean taxonomy plan` for `🔢️number` →
`unresolved=0`; `clean taxonomy apply` → **`state=committed`** (first apply attempt timed out
mid-`verifying`, `--resume`d the journal to completion) — moves=2 edits=5 regenerations=1,
`.rs`/`Cargo.toml` path rewrite confirmed on disk.

`cargo check -p semio-s-plugin-draw`: 59 errors, all inside `semio-s-plugin-stdio`
(E0046/E0433/E0425/E0308/E0599), none touching `canvas-pointer-down`/`fsm`/`macros` —
pre-existing blocker, not introduced by this change.

Files touched: 11 moved artifact files (see plan for list) +
`✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml`, `📦️glue.rs`, root `Cargo.toml`,
`🔒️dependencies.json`, `📜️script.ts`, `.vscode/launch.json` (regenerated),
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/*` (regenerated),
`🧰️framework/🔨️modules/🔢️number/*` (the verification scope's own applied rename).
