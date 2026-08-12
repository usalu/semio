# W1d — `semio-s-plugin-puzzle`: `.setup()` residue verification

Scope: `✏️s/🔌️plugins/🧩️puzzle/`. This pass did not author the code changes — the framework agent's
W1d pass (`📓️w1d-declaration-gaps-report.md`, Gap B) already made them, on the same session run
before this one started. This report verifies that state on-disk and runs the required build check.

## State found: `.setup()` narrowed, not eliminated — by design, documented

`🧩️puzzle/🦀️component.rs` `plugin()` (lines 36-48) has exactly **one** `.setup(setup)` call. The
`setup` free fn (lines 52-56) contains exactly two calls:

```rust
fn setup() {
    crate::artifacts::puzzle2d::standards::v1::engine::register_media_io();
    crate::artifacts::puzzle3d::standards::v1::engine::register_mesh_io();
    crate::artifacts::puzzle5d::standards::v1::engine::register_mesh_io();
}
```

(Three calls, two distinct fns — `register_mesh_io` is shared by 3d and 5d.)

### What was closed (B1 — app-schema, verified in place)

`register_app_schemas()` (the old third thing `.setup()` used to call) is gone. Grep confirms zero
remaining references to `register_app_schemas` or the old free-function `register_app_schema()` names
anywhere under `✏️s/🔌️plugins/🧩️puzzle/`. In its place:

- `Puzzle2dPlayApp::app_schema()` at `🎛️apps/◻2d/🦀️component.rs:856`
- `Puzzle3dPlayApp::app_schema()` at `🎛️apps/🧊️3d/🦀️component.rs:2232`
- `Puzzle5dPlayApp::app_schema()` at `🎛️apps/🖐️5d/🦀️component.rs:1594`

each `Some(…::app_schema_descriptor())`, backed by a converted (self-registering → returning)
`app_schema_descriptor()` fn in each app's `🎚️config/🧬️schema/🦀️component.rs`. This is wired through
`.register_document_app::<…>()`, already present in `plugin()` for all three apps — the exact `🗒️note`
pattern. **Confirmed closed**, category-1 app-scope schema, no residue.

### What survives (B2 — OS media-host bridges) and why

`register_media_io` (puzzle2d) / `register_mesh_io` (puzzle3d, puzzle5d) call into
`semio_framework_os`'s process-global media export/import handler registry
(`register_2d_export_handlers` / `register_dwg_import_handler` / `register_mesh_exporter` /
`register_mesh_importer` / `register_mesh_dwg_{export,import}_handler`) — a **separate registry** from
`io_registry`/`ComposerEntry`, which `.composers(...)` on each artifact's `declaration()` already
covers declaratively.

This is not a declaration-field gap that was missed; it's a genuinely different registry with no
corresponding `ArtifactDeclaration` field, for three concrete reasons (elaborated in the plugin-root
doc comment and in the framework agent's report):

1. Keyed by a legacy "OS media kind" string (`"2d.puzzle"`/`"3d.puzzle"`/`"5d.puzzle"`) that is **not**
   `ArtifactDeclaration.kind` (`"s.puzzle2d"` etc.) — no 1:1 mapping a declaration field could thread
   through the way `document_codec_bare` threads `schema`.
2. Format coverage only **partially** overlaps the composer tree (composer also serves
   PDF/JSON/DXF/LAS/PLY/GLTF the OS bridge doesn't touch) — not a clean duplicate safe to delete
   outright the way lowpoly's redundant `register_mesh_exporter` call was.
3. This exact registry family is independently flagged elsewhere in this ticket
   (`📓️status.md` finding #3) as non-deterministic under concurrent registrants (demonstrator racing
   an owning plugin for `3d.process`/`3d.procedural` via the same mechanism). Adding a declaration
   field for it would legitimize the mechanism this ticket exists to remove, not close the gap.

Verified the `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]` guards inside
`register_media_io` (`◻2d/…/⚙️engine/🦀️component.rs:54`) and `register_mesh_io`
(`🧊️3d/…/⚙️engine/🦀️component.rs:596`) are untouched — this pass changed nothing inside either
function body, only the caller wiring, so their native/non-wasm gating is unchanged and does not need
re-justifying.

**Conclusion: an honest residue, not a workaround.** `.setup()` is narrowed from 3 unrelated reasons to
exactly 1 (the OS media-bridge family), matches the task's own escape clause ("if some item genuinely
cannot be expressed by any declaration field, narrow `.setup()` to exactly that item and report
precisely what it is").

## Structural verification

- **`#[path]` resolution** (`📦️packages/🦀️rust/📦️glue.rs`): 506 non-`"."` `#[path = "..."]` attributes,
  all resolve to an existing file relative to `glue.rs`'s directory. 0 missing.
- **`include_str!`/`include_bytes!`**: scanned every `.rs` file under `✏️s/🔌️plugins/🧩️puzzle/`
  recursively — 205 occurrences, all resolve. 0 missing.
- **`cargo metadata --no-deps --manifest-path …/📦️glue`'s crate `Cargo.toml`**: exit 0, valid JSON,
  `semio-s-plugin-puzzle@0.1.0` listed as the workspace default member.

## `cargo check -p semio-s-plugin-puzzle --all-targets` — BLOCKED-CHURN, classified (c) upstream

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/🎯️target cargo check -p semio-s-plugin-puzzle --all-targets
→ exit 101
```

5 `error`-prefixed lines total: 4× `error[E0433]: cannot find `inferences` in `schema`` + 1 summary
(`could not compile `semio-s-plugin-stdio` (lib) due to 4 previous errors`). **All 4 errors are inside
`🗄️stdio`**, not puzzle:

| file (under `🗄️stdio/🗿️artifacts/…`) | line |
|---|---|
| `🎒️zip/🏅️standards/🔖️2.0/…/⚙️engine/🦀️component.rs` | 746 |
| `🧊️obj/🏅️standards/🔖️3.0/…/⚙️engine/🦀️component.rs` | 451 |
| `🟪️stl/🏅️standards/🔖️ascii/…/⚙️engine/🦀️component.rs` | 295 |
| `📼️avi/🏅️standards/🔖️1.0/…/🚪️io/🦀️component.rs` | 59 |

Grep-verified: zero error lines mention any `🧩️puzzle` path. `Compiling semio-s-plugin-stdio` and
`Compiling semio-s-plugin-puzzle` neither appear as a line in the log (cargo died building `🗄️stdio` as
a dependency before reaching `puzzle` itself, same "blocked-churn" pattern the framework agent's report
already hit — same error *class* (`E0433: cannot find inferences in schema`), but a **different set of
files** than that report's run (`zip`/`obj`/`stl`/`avi` here vs `mp3` there ×3) — consistent with an
in-progress edit sweeping across `🗄️stdio` file-by-file, not a stable break.

`stat -f '%Sm'` on all 4 failing files: `23:47:42`–`23:49:17` (this same evening), against a `date` of
`23:50:51` at check time — 1-3 minutes old, well inside an active edit window. `git log --oneline -3 --
🗄️stdio` shows unrelated older commits (`495`-`497`, not from today's session numbering), confirming
these are **uncommitted, in-flight** edits — not something `git log` alone would catch, matching this
ticket's own warning that `git status`/`git log` are not sufficient churn detectors for live sessions.
Per protocol (`🗄️stdio` is peer-held, off-limits, "retry-and-wait not patch") this was not touched.

**Classification: (c) upstream.** Zero errors originate in `🧩️puzzle`'s own paths; the failure is
entirely inside peer-held `🗄️stdio`, live-edited during this check. Full log:
`scratch-w1d-puzzle-check.txt` in this ticket folder. `cargo metadata` log:
`scratch-w1d-puzzle-cargo-metadata.txt`.

## Files touched this pass

None — verification-only. (All code changes for this plugin were made by the framework agent's prior
W1d pass; see its report for the diff list covering `🧩️puzzle/🦀️component.rs` and the six
`🎛️apps/*`/`🎚️config` files.)
