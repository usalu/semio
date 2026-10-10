# r11-exec-store-pt6: plugin tests `extension-retirement` and `app-child-member-registry`

Files (plugin = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`):
- `🧪️tests/🔬️extension-retirement/🦀️.rs`
- `🧪️tests/🔬️app-child-member-registry/🦀️.rs`
- `🧫️fixtures/🧩️extension-retirement/🔣️.json` (removed the byte-pair keys `grant` and `handlerGrant`: byte ceilings are quoted demands now; the `.ts` oracle never read them)

## extension-retirement
- Test owners implement `close_step(RetainedCloneGrant) -> PluginLifecycleStep` and `retirement_demands(body)`; the fixture `Resource` drives a real `RetainedCloneClose` (`begin_granted`, `step_granted`, quoted per-axis demands).
- Helpers: `grant_of(items, bytes)` (all byte axes equal, used for zero/under-granted turns), `bundle_grant`/`registry_grant` (exact quote), `yielded`.
- Laws re-expressed: `Pending{0,0}` zero turn -> `Progress(default)` yield; `AwaitingInput` for a too small byte grant -> yield below the quoted axis; `Pending{1,0}` etc. -> exact receipts `Progress/Complete(RetainedCloneProgress{..})` that `fits(grant)`; metadata retirement asserts the exact quote (`release_bytes`, `capacity_bytes` for the expansion birth, depth 1) and receipts; registry terminal turn is `Complete{copied_items:1}`.
- Exported-poll test: the extension turn grant is `{items: fuel>0, copy: max_patch_bytes, capacity/release/depth: runtime_lifecycle_grant}`, so `max_patch_bytes` no longer gates the release of a large allocation (the old law "small patch budget cannot retire the 128 KiB string" is obsolete by design). Equivalent law: the quote sits on the release axis (> small, <= adequate, <= lifecycle release ceiling), a grant with release below it yields with unchanged quote, fuel 0 / cancel keep it, resume + quota event shrinks it, and the small patch budget alone retires everything.

## app-child-member-registry
- `OwnedDocumentMemberIngress(+Registry)` close loops use `retirement_demands(body)` + `close_step(grant)`; `CompositionPinsRetirement` uses its quote (new law: the text frontier costs the last scalar on the copy axis, a copy grant one below yields).
- `ArtifactStoreOneItemGrant` 5-field grants via `one_item(release, depth)`; `MemberOpenRequest` closed via `store::test_support::drive_retirement`.
- Deliberate law changes: an externally borrowed private root now yields `Progress(default)` (the old `Blocked` reason has no counterpart in `RetainedCloneStep`); the old "demand != 0" assertion (`next_close_byte_demand` used `.max(1)`) became "every metadata action converges with copied_items == 1 under a one-item grant of its quoted release" (the lib quote is 0 for empty fields); last turns that empty an owner are `Complete(progress)`.

## Results
- Compile: `cargo check/test -p semio-framework-plugin --lib --tests`: 0 errors in both files (only the pre-existing `unnecessary qualification` warnings).
- Tests (gate, `cargo test -p semio-framework-plugin --lib -- --test-threads=1 extension_retirement_tests child_member_registry_tests`): **20 passed, 0 failed** (14 in `child_member_registry_tests`, 6 in `extension_retirement_tests`). The `panicked at` lines in the log come from the two intentional `catch_unwind` / fail-closed Drop laws.
- Extension resource law found while running: a controlled retirement releases every frame it births (392 + 4096 + 32 bytes here) plus its 80-byte shell besides the payload, so the fixture owner now asserts `released == payload + all retained_capacity born` (previously `released == payload`).
- No lib bug found. Observation for r11-store: `ChildContentView::close_prepared_structure_step` has no way to say `Blocked`; an externally borrowed private root now surfaces as a zero-progress yield, so the caller ladder must treat a repeated empty receipt there as blocked.
- Not run: the two `.ts` oracles (unchanged apart from the removed unused fixture keys).
