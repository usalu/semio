# W3 — `semio-s-plugin-forms` — APA closure report

apa-status: complete

**Note:** this report replaces an earlier stale version of itself titled "STOPPED at clearance
gate" (apa-status: partial), written by a prior pass that read absence-from-RELEASED as HELD. That
was the exact misreading the ticket brief's clearance note warns about — corrected below.

## Clearance

`📋️forms` is absent from all four sections of SMO's `📓️plugin-release-status.md` live predicate
(RELEASED / HELD-in-flight / HELD-between-waves / NOT-SMO'S-TO-RELEASE). That file was updated
mid-session to add an explicit clarification: "ABSENCE FROM THIS FILE MEANS FREE, NOT HELD" —
naming `📋️forms` directly as one of five plugins ("`📐️cad`, `🏗️fem`, `🖍️draw`, `🌀️procedural` and
`📋️forms`") that earlier agents wrongly skipped on the stricter reading. Per that corrected rule,
`📋️forms` was FREE to proceed without further confirmation.

## What changed

Deleted three doc-only, unmounted facet directories at the plugin root:

- `✏️s/🔌️plugins/📋️forms/🛂️manifest/🦀️component.rs` — single-line docstring only, no code.
- `✏️s/🔌️plugins/📋️forms/🎟️capabilities/🦀️component.rs` — single-line docstring only, no code.
- `✏️s/🔌️plugins/📋️forms/🔧️setup/🦀️component.rs` — single-line docstring only, no code.

Confirmed unmounted before deletion:
`grep -n "🛂️manifest\|🎟️capabilities\|🔧️setup" "✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/📦️glue.rs"` → no matches (exit 1).

No plugin-root `.DS_Store` or `node_modules` present (nothing to delete there). No `AGENTS.md` /
`README.md` existed at the plugin root — none added, per instructions.

Step 2 (close plugin root) required no further action: after the three deletions the root already
holds only `🦀️component.rs`, `🎛️apps`, `🗿️artifacts`, `📦️packages` — the target shape.

Step 3 (escape-hatch call sites): none found.
`grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_2d_export_handlers\|register_app_io\|register_os_media_" "✏️s/🔌️plugins/📋️forms/"` → no matches.

Step 4 (dependency purge): no action needed/taken. `grep -rln "semio_framework_os::"` in the
plugin → no matches, and `📦️packages/🦀️rust/Cargo.toml` never depended on bare
`semio-framework-os` in the first place — only `semio-framework-os-kernel` (aliased `dsl` /
`protocol` / `store`) and `flow` (package `semio-framework-os-flow`), both still referenced
throughout the crate. Nothing to remove.

## Files created / updated / removed

- Removed: `✏️s/🔌️plugins/📋️forms/🛂️manifest/🦀️component.rs` (dir removed)
- Removed: `✏️s/🔌️plugins/📋️forms/🎟️capabilities/🦀️component.rs` (dir removed)
- Removed: `✏️s/🔌️plugins/📋️forms/🔧️setup/🦀️component.rs` (dir removed)
- Updated: this report (replaced the stale "stopped at clearance gate" version).

No other files touched. `🧬️mutations/**` and any draft-lane facet were not touched.

## Step 6 — structural verification (no cargo; SDK is red)

1. `ls -a "✏️s/🔌️plugins/📋️forms/"` after deletion:
   ```
   .
   ..
   🎛️apps
   📦️packages
   🗿️artifacts
   🦀️component.rs
   ```
   Matches the required closed shape exactly.

2. Every `#[path = "..."]` mount in `📦️packages/🦀️rust/📦️glue.rs` resolves on disk. Verified
   exhaustively with a script that parsed all `#[path = "..."]` attributes, skipped the `"."`
   self-referential grouping-module ones, resolved each remaining path relative to `glue.rs`'s own
   directory, and checked `os.path.isfile`:
   ```
   checked (non-'.' path mounts): 79
   missing: 0
   ```
   All 79 non-trivial mounts (artifacts standards/subsets/schema/snapshot/inferences/diff/mutations
   triads, io import/export deserializers/serializers, apps config/presence/terminology/commands/
   modes/windows/panels, examples, the plugin-root `🦀️component.rs` mount itself) resolve.

3. `grep -rn` repo-wide for any reference to the removed module paths or their Rust module names —
   none found:
   ```
   grep -rn "forms/🛂️manifest\|forms/🎟️capabilities\|forms/🔧️setup\|forms::manifest\|forms::capabilities\|forms::setup" --include="*.rs" .
   → no matches
   grep -rn "crate::manifest\|crate::capabilities\|crate::setup\b" "✏️s/🔌️plugins/📋️forms/"
   → no matches
   ```

4. No files were moved in this packet (nothing beyond the three doc-only facet dirs needed
   relocating), so "moved file exists at its own new path" is vacuously satisfied — there is
   nothing to check.

## Step 5 — inventory only (nothing changed)

- `thread_local!`: none in the plugin.
- `RefCell<...>` / `Cell<...>` / `Mutex<...>` / `RwLock<...>` / `static mut` / `OnceCell` /
  `Lazy<...>`: none as live code. The only hit is a docstring in
  `🎛️apps/📋️forms/🎚️config/🦀️component.rs` describing a *pre-migration* history: `FormsConfig` /
  `FormsConfigMutation` already absorbed every field that used to live on
  `forms_ui::FormsPlayApp`'s `RefCell<FormsPlayRuntime>` (blueprint selection, Try-wizard step
  index, in-progress answer JSON) plus `locale` and `contributions_json`, all now event-sourced
  through `FormsConfigMutation` variants (`SetSelection`, `SetStepIndex`, `SetTryValues`,
  `SetLocale`, `SetContributions`) with a real `Snapshot`-based `inverse()`/`backwards()`. This is
  app-level view state (not document state) but it is already migrated off interior mutability —
  there is no draft-lane facet to inventory here, and none was authored.
- `std::fs` / `std::env` / `std::process` / `Command::new` / network (`reqwest`, `TcpStream`,
  `std::net`) outside `#[cfg(test)]`: none.
- `fn seed(`: none.

Conclusion: `📋️forms` has no outstanding draft-lane / interior-mutable app state to migrate and no
filesystem/env/process/network side effects to relocate. Nothing proposed for `Draft` fields or
verb-slugs because there is nothing left that needs them.

## sharedFileRequests

None.

## Concurrent-churn observations

`📓️plugin-release-status.md` (SMO's live predicate) was updated mid-session — outside this
plugin's boundary, read-only — to add the "absence means free" clarification, explicitly naming
`📋️forms` among the plugins a prior pass wrongly skipped. Not reverted; used as the basis for
proceeding, per the system reminder accompanying that change.

## Notes

Per this packet's clearance note, forms had no extra plugin-root dirs beyond the three deleted
facets, so the "MOVE real code into the owning artifact's engine" branch of Step 1 never
triggered — all three facet files were doc-only comments with nothing to relocate.

cargo verification intentionally deferred: `semio-framework-plugin` is red from a peer session's
in-flight work (E0499/E0560/E0609) and eight agents share one build lock. All verification above
is structural (path-mount resolution, textual grep for dangling references), matching the
ticket's explicit no-cargo directive.
