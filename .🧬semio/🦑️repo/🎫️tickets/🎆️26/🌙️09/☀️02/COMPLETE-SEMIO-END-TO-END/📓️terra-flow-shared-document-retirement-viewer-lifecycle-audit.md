# Flow Shared Document Retirement and Viewer Lifecycle Audit

Date: 2026-09-04  
Scope: current source only; no Cargo/Nx command was run because the provider owns the cold native target.

## Current source verdict

The Flow artifact extraction is source-accepted for document scene/snapshot/mutation ownership. The viewer now has the newly required owner/disposer declarations, but there is not yet a native `VcsArtifactApp<ViewerApp<FlowViewer>>` lifecycle law. The existing Flow exact gate proves source fixtures plus five other native laws; it does not construct and close the viewer aggregate. Treat viewer lifecycle as **source-complete, runtime-unproven** until the focused law below runs.

## Shared artifact retirement

`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/♻️retirement/🦀️.rs:13-18` owns the one reusable `MemberStoreOwners<FlowSnapshot, FlowMutation>` catalog. Both the root and owned snapshot route use `SnapshotRetirementFactory`; mutations use `MutationRetirementFactory`; the store cursor has one `ArtifactStoreCursorDisposer`.

- `RootRetirement` transfers `Arc<T>` to `Option<T>` only with `Arc::into_inner` and then transfers that `T` to its domain-specific `FlowRetirement` (`:46-73`). If another reader still owns the `Arc`, `into_inner` returns `None`, the captured root reference is released, and the other reader remains the owner. It neither force-drops nor clones the payload.
- The two handoff turns report one item and zero bytes (`:64-65`); only `FlowRetirement::close_step(1, bytes)` releases payload bytes (`:66`). This avoids double byte charging.
- `retire_scene` retains the three exact scene payloads (widgets, synapses, layout, `:21-27`). `retire_mutation` covers the current ten `FlowMutation` variants (`:29-43`), and the former editor-local retirement delegates `Owner::Scene` and `Owner::Mutation` back to these shared functions at `✏️.../✏️editor/🧵️retained/🦀️.rs:108,178`. There is no remaining duplicate editor mutation-arm map.

## Snapshot child cache and accounting

`♻️retirement/📸️snapshot/🦀️.rs:27-57` separates the root/snapshot, cached child scene, and every wire string.

- Phase 0 calls `take_local_owner::<FlowWorkingScene>()` (`:35-38`). A foreign erased owner yields an error before removal; an exact owner transfers only when this is its final `Arc`. A shared cached scene stays owned by the other reader. This is fail-closed, not a silent drop.
- Phases 1–6 retire the document schema and every `ArtifactChild` coordinate string (`:40-49`). The `camera` is not omitted from a heap-owner path: `flow::CameraJson` is exactly three `f64` values at `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifact/🦀️.rs:177-182`.
- `FlowSnapshot` has no other durable heap field outside `schema` and `content`; the local scene is explicitly non-wire cache authority in `✏️.../🗿️artifacts/🌊️flow/🦀️.rs:197-218`.

No ownership loss, stale editor import, or source-visible byte-accounting duplication was found in this extracted path.

## Viewer hooks now wired, but not executed as an aggregate

Current `FlowViewer` explicitly supplies:

- shared document catalog and document disposer (`👁️viewer/🦀️.rs:53-63`);
- bounded `NoConfig` catalog/disposer (`:57-67`);
- local and peer `NoPresenceRetirementFactory` plus the exact `PresenceStoreOwnedDisposer` (`:69-79`);
- exact `NoTransientStoreDisposer` (`:81-83`).

`ArtifactViewer` declares these fail-closed hooks and `ViewerApp` forwards them without adapter synthesis at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26391-26419,26914-26949`. `ViewerApp` owns the only possible `NoDraft` lane and supplies its bounded owners/disposer itself (`:26943-26949`), so Flow must not duplicate a viewer draft hook.

The new presence disposer has a valid exact-terminal fence: it preserves the terminal `Arc`, stores a weak pointer plus generation only after `PresenceStore::begin_retirement`, and on subsequent turns requires the same local pointer, generation, local terminal predicate, and empty peers (`👥️presence/♻️retirement/🦀️.rs:35-86`). `begin_retirement` atomically replaces local/peers only after both relevant retirement factories exist (`🏪️store/👥️presence/♻️retirement/🦀️.rs:196-214`).

The transient disposer is intentionally restricted to the two zero-sized no-state types and asserts the concrete store has only its current root/generation footprint (`🫧️transient/♻️retirement/🦀️.rs:7-40`). Its tuple-size assertion is a cross-platform source guard against ordinary extra state; semantics are also fenced by a weak root and generation, so it is not relying on byte-size equality for ownership identity.

## Missing acceptance law

The viewer has only definition/dialect tests (`👁️viewer/🦀️.rs:118+`). The registered Flow `child-identity-check` runs five exact native laws (`📦️packages/🦀️rust/📜️script.ts:27-41`), none constructs `VcsArtifactApp<ViewerApp<FlowViewer>>` and drains its six close stages. The neutral viewer fixture itself requires `complete` and `terminalEmpty` but is presently source-oracle only (`👁️viewer/🧪️fixtures/🧹️owners/🔣️.json`).

Smallest truthful native law (place it in the Flow viewer test module or the existing Flow artifact lifecycle test module):

1. Load the neutral viewer owner fixture and use its sole positive `grant`/`maximumSteps` values.
2. Assert through the public `ArtifactApp` adapter that `ViewerApp<FlowViewer>` exposes `Some` for document/config owners and document/config/presence/transient disposers; assert the viewer is read-only by running existing `testkit::assert_viewer_never_mutates::<FlowViewer>()`.
3. Construct the real aggregate through `semio_framework_plugin::testkit::new_viewer::<FlowViewer>().await` (`🧰️.../🔌️plugin/🦀️.rs:7026-7031`), not manually-built stores.
4. Import `PluginApp`; repeatedly call the real `app.close_step(1, 4096)`. Every `Pending` result must remain within that grant; `Blocked` is failure for this unshared genesis fixture; completion must occur within fixture `maximumSteps`.
5. Require `app.close_terminal_is_empty()` immediately after `Complete`. This exercises `VcsArtifactApp`'s actual document, config, framework-owned draft, presence, transient, and interaction close stages (`🧰️.../🔌️plugin/🦀️.rs:23691-23715`).
6. Register that exact FQN as a sixth `child-identity-check` law only after the source-first version is observed RED and then green. Keep the existing independent Bun/AJV owner fixture in the same target; it must not be substituted for this runtime law.

This test needs no test-only access to private `VcsArtifactApp` fields and no generic lifecycle helper. It proves the exported constructor plus the production `PluginApp` closure path directly.

## Acceptance boundary

Source1254's source-green status supports the shared extraction and the now-visible viewer declarations. It does not prove cold native compilation or the full viewer lifecycle; the provider-owned build remains the only active native evidence and was not disturbed.
