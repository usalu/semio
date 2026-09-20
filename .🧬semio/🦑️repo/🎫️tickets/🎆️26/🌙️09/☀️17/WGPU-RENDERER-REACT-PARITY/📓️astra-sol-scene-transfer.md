# Retained Scene List Transfer and BlockList Sorting

## Contract

The packet uses one generation-owned transfer authority across retained scene surfaces and windows. A source is typed as Table row, BlockList step, BlockList block, or BlockList palette entry. It carries its source window, surface, retained-document generation, driver-owned physical origin, and exact MIME/payload where applicable. Release revalidates the source generation and identity, destination permission, MIME interoperability, and source/target type before emitting any action.

Handle drivers arm only a semantic handle and leave the label band inert. Surface drivers arm the row and omit the handle. A transfer promotes only after the shared drag slop; a promoted gesture suppresses its source click. Escape, release outside a retained scene, source-window closure, source subtree replacement, and a stale generation retire the authority without an action.

BlockList sorting follows the React host. Steps use closest-center across the step roster. Blocks use closest-center only within their source step and emit `moveBlock` with identical `fromStepId` and `toStepId`. Palette entries remain native transfer sources into a step. The alternative move-up/move-down button UI is removed; each step and block retains its remove button.

## Production API Boundary

- Non-bespoke retained scene leaves publish a semantic `HitKind::ComponentScene`; Shell routes that kind through `EventRouter`. Existing World3d and ScrollRegion bespoke routes remain unchanged.
- The canonical `UiDriverDrag` is passed explicitly from `FrameworkSceneHost` into `render_component_scene_step`, then into the BlockList layout, paint, hit, and source resolver.
- Scene pointer ingress receives the owning `window_id` and retained generation. The transfer authority stores both and refuses a release after the source window/subtree generation changes.
- Shell invokes explicit cancellation for Escape, pointer-up outside any retained scene, and source-window closure.
- BlockList plan targets carry typed source/drop roles and token-derived shared handle/control geometry. Paint and hit routing read the same plan.

The authority has one active owner at a time. `TableRow`, `BlockListStep`, `BlockListBlock`, and `BlockListPalette` are the complete source vocabulary. Starting another source replaces the prior owner; a driver change, source-generation change, source-window close, Escape, or completed release retires it. Table releases additionally require the destination's declared drop action and the exact source MIME. BlockList releases re-read the current source list before producing `moveStep`, `moveBlock`, or `addBlock`, so a source removed or reordered by a newer retained generation cannot act through stale geometry.

## Shared Fixture and React Oracle

`🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🔀️scene-list-transfer/🔣️.json` is validated by `SceneListTransferFixture` in the shared UI render schema. It covers Table A→Table B, Escape/outside/source-close cancellation, step reorder, within-step block reorder, palette insertion, Handle-label no-drag, Surface initiation, and stale-generation rejection.

The React oracle renders the actual `TableHost` and `BlockListHost` under Default/Handle and Compact/Surface drivers. It arms the real native Table drag handle, transfers the actual `dataTransfer` MIME/payload into a second Table host, and compares the destination action with the fixture. It checks the real BlockList handle census and Surface palette-row initiation, then calls dnd-kit's actual `closestCenter` for the step and source-step block target geometry.

Focused oracle result: **1 passed, 657 skipped**.

`NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache -- --run '../../../../🧪️tests/🔬️engine-contract/🟦️.ts' --silent=false --reporter=verbose --testNamePattern='shared scene-list transfer fixture'`

## Native Laws

The focused Table laws cover Handle versus Surface initiation, cross-window MIME/payload transfer, exact destination action shape, and stale source-generation rejection. The BlockList laws cover semantic handle census, inert Handle labels, Surface initiation, closest-center step and block reorder, palette insertion, Escape cancellation, source-generation cancellation, React-compatible action arguments, and four-sided card paint. They materialize the renderer dependency's bounded action queue through `take_action_step`; the crate-local test-only `drain_events` helper is deliberately unavailable here.

Shell's actual Display producer law now crosses the complete retained boundary: `build_display_windows_ui` → panel projection → retained document lease/reconcile/paint → published transfer hit → host down/move/up → empty-dock commit. It asserts the transfer MIME survives, the row publishes transfer rather than sort, a captured move promotes the dock drag, a release with `hit=None` still reaches retained capture, exactly one `main-2` window is committed, a duplicate release is inert, an outside release cancels, and both paths retire retained capture and dock transient state.

The production correction is shared by shell-painted and retained Tree hits. `begin_window_template_drag_from_hit` decodes and validates the window kind before the retained-owner early return. Hitless release falls back to the retained capture window and its published body rect after dock commit/cancel. A retained panel remains a pointer owner but is excluded from app-window activation, so `framework.display.windows` is not invented as an application window scope.

## Validation Status

- React/dnd-kit oracle: **1 passed, 657 skipped**.
- Shader syntax integration encountered during the packet: the literal raw-string `\\n` in canonical and target mesh WGSL was replaced by a real newline. The strengthened Naga law parses and validates all three runtime and all three canonical variants. Root-owned warm gate receipt: **135/135 passed, 0 skipped**.
- `rustfmt --emit stdout` parser checks pass for the changed Scene, Interpreter, UI engine/input, Shell, and focused Rust-law files.
- All three renderer `include_str!` paths resolve from the actual renderer crate manifest directory.
- The direct mounted-frame TypeScript source-law invocation did not complete: its pre-existing/concurrent `dormant-production-authority` hostile-mutation needle no longer exists in runtime source index 4. This is outside the transfer packet; the updated whole-scene call needle itself is current.
- Renderer10 compiled an earlier partial Table snapshot and stopped on 22 errors. The current source fixes its three fixture paths, two dependency-hidden `drain_events` calls, stale `RowTransfer` arm, `IconName`/`&str` call, and removed BlockList helper callers. No renderer10 assertion ran.
- Renderer11 compiled and executed 1,151 native renderer laws: 1,119 passed and 32 failed. The transfer-specific follow-up below repairs the assigned failures without weakening generation or retirement assertions; a fresh root-owned renderer gate remains required.

No Cargo, native, wasm, browser generator, or runtime job was started here.

## Renderer11 Failure Reconciliation

The assigned renderer11 failures split into production defects and test-contract drift:

- Table stale-generation rejection was a production defect. Invalidating generation 8 on source move 9 retired the transfer, but the destination's later `PointerUp` fell through to an ordinary row click even though that surface never received a down. A list release now requires that surface's own `pointer_was_down` after the transfer-release branch. Every cancellation/release path also clears the source surface's pointer ownership.
- VFS ctrl selection was a production defect. The helper returned the first selection without committing it to scene-local selection authority, so the next additive gesture started from empty. Plain, range and additive results now commit atomically, and additive output follows visible row order rather than `HashSet` iteration order.
- IconRender's integer/float failure was representation drift. The generated environment correctly carried azimuth 120 and elevation 25 as JSON numbers; its typed `f64` fields serialized as `120.0`/`25.0` while the neutral fixture used integer lexical forms. The law keeps full field checks and compares the three numeric sun scalars by numeric value.
- The World3d retirement panic was test teardown drift. The production `Drop` assertion remains unchanged. The law now drives the already-queued owner through bounded `advance_world3d_retirement_step` calls and requires terminal empty before teardown.
- The Display producer law now publishes under a unique test-owned surface identity. It still begins at the real `build_display_windows_ui` producer and asserts the exact Display transfer id/MIME/payload, avoiding collision with the process-global runtime panel id.

The previously missing host-level Table law is now present. It pack-encodes two actual `Component::Surface(Table)` records, publishes two retained document leases, paints both into one registry, proves the semantic `ComponentScene` target owns the physical source handle and destination row points, and drives Shell down/move/up. The result must equal the shared neutral fixture's merged destination action exactly once, with retained capture and scene transfer authority empty after release.

The UI retained-hit neutral fixture now includes a Table surface and the language-independent `componentScene` kind. The Rust live-registry law and TypeScript derivation share that case; the Rust kind match is exhaustive again.

## Mandatory Worker Submission Finding

Renderer11's two `Contended` panics were not cross-test interference: nextest ran each as a separate one-test process. Native `WorkerPool::submit` delegated to `try_submit`, and `try_submit` deliberately uses `try_lock`; the mandatory path therefore panicked whenever its own worker briefly held the selected lane queue mutex. The worker drops that mutex before invoking a job, maintenance callback, or any other blocking work, so mandatory admission can safely acquire the queue mutex once. The explicit `try_submit` path stays nonblocking and retains exact `Contended`/capacity refusal semantics. A deterministic native law holds the selected queue, verifies `try_submit` refuses with `Contended`, starts a mandatory submit on another thread, releases ownership, then requires admission and execution of the exact closure. It uses no retry or sleep.

The queue-lock safety boundary is structural. Native `select_and_pop` and `steal` move a `PoolWork` value out of the selected `Mutex<VecDeque<_>>`; their guards are local variables and are dropped when those functions return. `worker_loop` invokes `job.run` only after that return. The timer wheel likewise moves due callbacks into a local vector inside a scoped lock and invokes them after the lock scope closes. Mandatory `submit` holds only the selected lane queue long enough to check capacity and push the exact closure, drops it explicitly, then notifies idle workers. It never invokes a job, maintenance callback, timer callback, or wait while owning that queue.

The canonical focused Nx invocation for the new native law is:

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-async-rs:test-long --skip-nx-cache -- native_pool::tests::mandatory_submit_linearizes_after_queue_ownership_is_released
```

Root-owned focused receipt: **1 passed, 69 filtered**, recorded in `🗑️generated/astra-runtime/async-mandatory-submit.log`.

## Follow-up Validation Status

- `rustfmt --edition 2021 --emit stdout` parser checks pass for Scene production, IconRender law, both Shell law files, UI retained-hit law, async production, and the native async law.
- The retained-hit fixture parses as JSON.
- The first focused TypeScript attempt used the React target, which does not include the wgpu retained-hit file and correctly reported no test files. The second used the owning wgpu Nx target; its four declared generation prerequisites passed, then its combined Cargo/Vitest router rejected Vitest's `--run` flag before any compilation or test ran.
- Root-owned receipts remain required for the focused async law, UI retained-hit native law, five repaired renderer laws, the Display producer law, and `two_published_table_surfaces_transfer_through_the_shell_host`.

Runtime acceptance also remains pending a fresh browser artifact: checkpoint8 predates the retained Display repair and demonstrated the original empty-dock failure.

## Renderer13 Transfer Follow-up

Renderer13 executed 1,160 laws and exposed the two remaining packet-specific boundaries. Its Display producer law stopped before input at the missing transfer handle; that binary sampled the earlier projection that dropped `UiTreeItemNode.drag_data`. Current `PanelProjection::tree_item` carries the producer's payload through `TreeItemProps.drag_data`, so native14 must verify the complete gesture against the current source.

The two-Table law completed the destination action but left retained capture on the source. The release had been sent only to the current destination hit. Shell now snapshots the captured source, delivers the semantic `PointerUp` to the current destination first, and then sends a cleanup `PointerUp` to the captured source only when the two window identities differ. Destination-first ordering consumes the generation-owned transfer exactly once; the cleanup coordinates are outside the source surface, so they only clear source pressed/capture state and cannot emit a source click or a second drop. The existing law keeps the exact one-action assertion and requires both retained capture and scene transfer authority to be empty.

The release correction passes `rustfmt --edition 2021 --emit stdout` parsing. Native14 remains the root-owned verification receipt.
