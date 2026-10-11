# 2026-10-10 Retained-Clone Protocol Migration Brief (v2, replaces "rename" brief)

The framework (value, job, store, plugin) moved from count-based close steps to a grant-based, five-currency retained-clone protocol (pushed from another machine; value crate completed locally and GREEN at 09:54). Callers must be MIGRATED, not renamed. Full analysis: `📓️2026-10-10-rename-rn-misc.md` (same folder).

## Protocol
- Close: `close_step(max_items, max_bytes) -> SnapshotRetirementStep` → `close_step(RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>` (`Progress(p)` / `Complete(p)`; each receipt must fit the grant).
- Factories: `retire_owned(&f, v)` → takes a grant, returns `Result<(Box<dyn ErasedSnapshotRetirement>, Progress), (ValueError, T)>`. `owned_retirement(v)` → `admit_owned_retirement(v, grant)` (same Result shape) + `owned_retirement_birth_bytes`.
- Job: `InteractiveJob::close_step(RetainedCloneGrant) -> InteractiveJobCloseStep` (`Pending{progress}`, `Blocked`, `Complete{progress}`, `Refused{kind, progress}`), plus four `next_close_*_demand` methods and `borrow_outcome`.
- One-item preparations: four demand methods + `begin_demand`; `begin(grant) -> (Box, Progress)`; fields become `ManuallyDrop<Option<..>>` with a `Drop` asserting the terminal state; payload types implement `RetireOwned`.
- `SnapshotRetirementStep` is gone (no alias).

## Template
Migrated exemplar: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (diff it against `git -c core.fsmonitor=false show HEAD~2:<path>` to see the exact before/after shape). Find other already-migrated callers with `rg -n 'admit_owned_retirement|RetainedCloneGrant' --glob '*.rs'`.

## Rules
- Migrate in the new direction only; no shims, no aliases, no unbounded fake grants unless the framework's own migrated callers do exactly that.
- Order: OS foundation first (`rn-os`: `💻️os` store, plugin crates, host, `🛢️db`), then plugins. Plugin verification waits until `🗑️generated/coord/os.status` starts with `GREEN os`.
- Wrap EVERY cargo command in the slot gate (max 4 concurrent fleet cargos, RAM is 32 GB):
  `bash "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" <agent> -- env CARGO_TARGET_DIR=… CARGO_BUILD_BUILD_DIR=… CARGO_BUILD_JOBS=3 cargo check …`
- Check both native and `--target wasm32-wasip2` for plugin crates (the release builds wasm components; wasm-gated code must compile). Use the features the plugin's component build uses.
