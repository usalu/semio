# Media Export Owner Context Regression

The existing media request context accepted only the port, document, and transient projection. Consequently the framework's VCS export path could not pass its retained instance operation owner to an editor or viewer. The parent changes the existing method uniformly to accept that owner first and forwards it through the real wrappers and VCS path. This regression owns no SDK signature edits and adds no S/domain dependency.

## Authored Neutral Contract

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🎞️media-owner-context.json` is authored before its new test implementation. Its independent output schema requires an owner string and integer document count, with no extra properties. Two owners (`alpha` and `beta`) over the same document count have exact different outputs. Four invalid payloads exercise missing owner/count, empty owner, and incorrectly typed count. The named refusal roster is foreign owner, closed owner, and unknown port; the default port remains `artifact:out`.

The existing framework `SurfaceEditorFixture` and `SurfaceViewerFixture` build a private test-owned media operation owner. The fixture's context export reads that exact supplied owner for `retained:out`; other ports delegate to the existing default exporter. No global owner is introduced. The private child test module uses the real `EditorApp`/`ViewerApp` adapters and actual `VcsArtifactApp::export_media` path. It proves both caller-supplied identities, closed-owner refusal, foreign-type refusal, exact unchanged document pack/base64/schema/media type, and unknown-port `NotImplemented`. The live VCS test changes the two private instance owners to distinct labels before export and checks those precise labels in the actual outputs. Both document projections remain unchanged. Owners and fixture apps are explicitly closed.

The owner close participant respects zero/insufficient grants, explicitly releases the retained string allocation, and becomes terminal only after close. It adds no test-only public product API. Direct assertions run against the existing wrapper types; the VCS path uses the instance owner retained by the real application wrapper rather than another helper-created owner.

## Native And Independent Proof

The initial uncached Nx plugin `test -- --no-run` failed in 31.3 seconds with exactly two `E0050` errors: each new owner-aware editor/viewer override had four parameters while the existing trait required three. The native compiler failure is recorded at `🗑️generated/media-owner-red.log:6923` and `:6934`. The parent waited for this actual RED before landing the SDK owner argument and forwarding changes.

An uncached `@semio-tech/framework-plugin:canonical-architecture -- --oracle-only` then passed in 3.9 seconds with the new AJV oracle's six independent admissions/rejections, preserving the established required-bridge 38, extension-retirement 13, and source-freshness 17 assertions/cases. Its log is `🗑️generated/media-owner-oracle.log`.

The first native compile after the SDK change found one fixture-only error at `media-owner-final/exact-cargo-laws-AGFc2y/00`: `Fault` has no `Display` implementation. The new fixture now maps its existing `error.message` into the media error. The clean native rerun uses `🗑️generated/media-owner-final-rerun.log`; no production code was changed for this correction.

The clean uncached canonical Nx run passed in **11 minutes 50 seconds**, including a wait for the shared Cargo artifact-directory lock and a fresh private-root compile. It used both `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` set to `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/native-canonical-final`, with `SEMIO_TEST_ARTIFACT_DIR` set to the absolute ticket `🗑️generated/media-owner-final`, `NX_DAEMON=false`, and `NX_CACHE_PROJECT_GRAPH=false`.

The exact native receipt group is `🗑️generated/media-owner-final/exact-cargo-laws-nXzeKq/00`. Build and native list exited zero, without signals. All **13** individual exact-law executions exited zero, without signals, and each output reports **one passed, zero failed, zero ignored**. The new regression is `component::app::mutation_fixture::surface::media_owner_context::media_export_request_context_preserves_exact_supplied_owner`; its `law-0.stdout` names the test once and reports success in 0.04 seconds. The compiled executable SHA-256 is `da0073f148f18424dd41d310e9146d533324cc7f3c0ffed694bfd00bc3b7451d`. The same final target records the six new AJV checks and all established 38/13/17 oracle assertions/cases.

Scoped `git diff --check` passed for both updated files; the three new implementation files have no trailing whitespace. No temporary debug logging was added. This proof is the generic 13-law SDK group, not a claim about the concrete app 275-law group or the full original 355-law composition aggregate.

The canonical target adds one native law, `media_export_request_context_preserves_exact_supplied_owner`, and preserves the previous 12 native laws. The new AJV oracle is also invoked by the ordinary plugin test route. No separate executable command/target/launcher is added.

## Exact Owned Manifest

Created:

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🎞️media-owner-context.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🎞️media-owner-context/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🎞️media-owner-context/🟦️.ts`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🎞️2026-09-30-media-export-owner-context.md`

Updated:

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts`

Deleted: none. No `AGENTS.md`, Git/worktree state, dependencies, project configuration, or launch command was edited by this increment. SDK/media product signatures and concrete generation3d code belong to the parent and IO implementation owner. Temporary logs/receipts remain ticket-only for parent cleanup.

## Settled Implementation Digests

Paths are relative to the framework plugin root listed above.

| Path | SHA-256 |
| --- | --- |
| `🧫️fixtures/🎞️media-owner-context.json` | `68ca657f8a1207b4e8980d262740600db6363cab4fa31155368e04f2b40fe498` |
| `🧪️tests/🎞️media-owner-context/🦀️.rs` | `44baf8874821869fdb40aa8ece6bb9cef4589069d63c97c06807b0bb2aa04b6e` |
| `🧪️tests/🎞️media-owner-context/🟦️.ts` | `f00c7113186a435404fb727f739f94fe3cfedb1ab34c62ed8d66ebec62307c4e` |
| `🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs` | `c561736b3606eb12f2f2bb6ecf6ac6d2bea63534d9d5696d5e7d413e228236fe` |
| `📦️packages/🦀️rust/📜️script.ts` | `45512534a1e96b431147303964d23fc9fa3bb4f1cffcd73336ca982ce7cc4b5e` |
