# Import Document, Export after refusal, Example switching (FEM 3D)

Probe: headless driver on port 6241 against http://127.0.0.1:6087/ (scripts `im-*.js` in this ticket, outputs `🗑️generated/im/`).

## 1. Import Document breaks the editor (FIXED, shared by 2D)

Root cause (measured with `[DEBUG]` prints, since removed):
- Import loads the archive into a NEW instance and adopts it through the surface switch, which first asks the retirement owner to persist the predecessor (`persistProgramDocument`).
- The predecessor document has an identity, so the persist branch tried to keep it in the local catalog and threw `natural-file.no-data-folder` (no `VITE_S_DATA_DIR`, the shell cannot keep anything).
- `admitProgramRetirementV1` seals the predecessor BEFORE persisting. The persist failure opened the "Unsaved changes ... The save was not acknowledged" modal and left fem#1 sealed, so every later edit hit `instance-sealed` (`addNode refused: instance-sealed fem#1`) while the import silently never published.

Fix: `programRetirementKeepsDocumentV1(localDataDir, parentDocumentId)` (pure, in `ShellHost/🚪️program-retirement/🟦️.ts`): a shell without a data folder cannot keep any document, so retirement does not demand a save it can never acknowledge. `persistProgramDocument` (`ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx`) uses it. Shared shell code, so 2D, CAD and every other program get the fix.

Verified in browser: export, import, then `addNode` dispatches to fem#2 with no refusal, notice `Opened “export.semio-archive” as a new document.`, caption `Case: dead`, no modal.

Law: `🧪️tests/🚪️program-retirement/🟦️.ts` (vitest, 14 passed, 4 new rows).

Side finding: after publish, the predecessor retirement logs `runtime close cleanup faulted for instance 1: zero-progress` (wasm close cleanup, non-blocking, successor is live). Not fixed.

## 2. Export after a refused command (NOT FIXED, root cause)

The actor's retained turn source (`🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🟦️.ts`, `holdNativeRefusal`) holds the first native refusal for the lifetime of the actor, and every later turn (pick, select, archive read for Export) throws `original-turn.source-not-ready` with that cause. A plugin command that returns `Err` (Cut `UnsupportedOwner`, `setSelectionMode: undeclared interaction domain`) arrives from `reactor.poll` as a failed call and is classified as a native refusal (`component-bridge-source`, `nativeRefusalError`), so a mere declined command poisons the actor. Export is not independent because it reads through the same turn source. A correct fix is in the plugin/runtime contract: a declined command must be a refused ingress outcome, not a failed poll. That needs a wasm rebuild and the clipboard agent's area; not changed here.

## 3. Example switching

Measured (wasm `prototype` profile, opt-level s with debug assertions):
- Concrete Forest: geometry replaced after 28 s, `Preparing analysis`, solved after 39 s. Its geometry does change (the earlier "unchanged" was the 28 s load lag at a 6 s wait).
- House: geometry replaced after 13 s, `Preparing analysis` for more than 370 s (still unsolved). The solve cannot be cancelled and blocks the actor, so the next switch (back to Demo) waits for it: Demo geometry came after 55 s, not solved after another 200 s.
- Native data: all three examples parse and solve in native laws (existing `🌲️concrete-forest`, `🏠️house` tests: house 4k-40k tets, 5 cases). New law `set_active_example_loads_a_distinct_document_per_example_3d` loads each example through the command and asserts non-empty, pairwise distinct documents.

Cause of slowness: House is a solid mesh (thousands of tets) solved in a debug-assertion wasm build, uncancellable, serialising the next load behind it. Not fixed: needs a solver/engine change (cancellable stepped solve, sparse factorisation) and a wasm rebuild.

## Files
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🚪️program-retirement/🟦️.ts`
- `.../🏛️ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx`
- `.../🧑‍🎨engine/🧪️tests/🚪️program-retirement/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/.../🎮️commands/📚️set-active-example/🧪️tests/🔬️unit/🦀️.rs`
- `🧊️3d/Cargo.lock`: removed 8 identical duplicate `dwg-snapshot-objects-*` package blocks that blocked cargo.

## Law status
- Vitest `program-retirement` (14 passed): PASS.
- Rust `set_active_example_loads_a_distinct_document_per_example_3d`: written, NOT yet run. Native cargo (`-p semio-s-artifact-fem-3d-editor --lib set_active_example`, detached launcher `/c/t/im-run.sh`, output `/c/t/im-run.out`, log `🗑️generated/im/law.log`) sat blocked on a shared `semio-framework` file lock held by other agents' builds. Rerun with `im-law.sh` when the lock frees; delete `C:\t\im*` afterwards.

## Coordinator follow-up
- Item 2: the selection agent's `plain_refusal()` (`🧵️job/⚠️failure`) plus the `🧊️reserved-routes` change now answer a declined reserved command as a typed refusal (`commit.sealed = Err(fault)`), so `reactor.poll` returns normally and `retained-turn` never reaches `holdNativeRefusal` for it. No TS change made: a genuine trap or unaccounted failure must keep poisoning the turn source (existing succession tests assert that). Needs the coordinator's wasm rebuild to confirm in the browser (export after a declined Cut/setSelectionMode).
- Zero-progress close cleanup of the old instance: already being fixed in `🎭️actor/🎟️retained-turn/🟦️.ts` by another agent (staged `closeJobs`: close registered jobs under paid steps before the context close, with a `retirement` succession fixture). Not duplicated.
- Law run: the detached native run (no `CARGO_BUILD_BUILD_DIR`, so it uses the shared repo build-dir) stayed blocked over an hour on a `semio-framework` file lock held by other builds, and was stopped. `set_active_example_loads_a_distinct_document_per_example_3d` is still UNRUN: `cargo test -p semio-s-artifact-fem-3d-editor --lib set_active_example`. `C:\t\im*` removed.
