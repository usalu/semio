# Selection Owner Checkpoint

This supersedes the renderer-local coverage computation described in earlier gesture notes. The current native host captures intrinsic rectangle, ellipse, lasso and wand intent, prepares its path in bounded frames, and submits `selectPixels`. The config command owns coverage computation; hosts do not republish large RLE through typed action nodes.

## Implemented Source

- Surface `paint/🎯️selection/🦀️.rs`: intrinsic geometry and captured image/target/transform revision identity. Locked layers remain selectable; paint admission remains separate.
- Native `EngineCanvas/…/paint2d/🦀️.rs`: rectangle, ellipse, lasso up to 4096 points and wand release; exact retained-string cancellation handles; bounded path encoding; source invalidation; previous published coverage remains authoritative until completion.
- Native scene rendering advances preparation once per production frame, paints progress and a cancellation hint, and keeps requesting frames. The mailbox exposes pending selection work for browser scheduling. Parent updated browser worker scheduling.
- Native Shell Escape resolves the exact focused live Paint2d host. Preparation cancellation targets that host's actual retained input handle. Admitted cancellation reads a typed scene authority with actual actor, app instance, window, operation and generation; mismatched ownership or retired scenes are ignored. Dispatch uses the existing `cancelTypedOperation` handler. Tasks Cancel retains its original capability check.
- Generic operation progress now carries originating verb/actor/instance/window. Raster composite publishes a cancellation descriptor only for one unambiguous active `selectPixels` operation from its window. Progress dirties the composite body as well as Layers.
- Paint2d scene Rust/TypeScript contracts include optional typed `selectionCancellation`. Serde and FromValue validate nonempty actor/window, strict 16 lowercase hexadecimal operation/generation and exact fields. Absence means no offered admitted cancellation.
- `commands/🎯️select-pixels`: schema-first small intent, dedicated retained factory/admission/proof, explicit 1 MiB path plus bounded header, bounded parsing/shape/flood/restoration/combine/RLE stages, explicit source/temporary storage retirement, exact input-root fencing, and config-only completion. Named all/invert/none methods are included. Raw path parsing yields across whitespace pages too.
- Config `📬️preparation`: replaced the cold config preparation factory with existing retained base/mutation cloning and field exchanges. Selection spans validate in pages; invalid late rows preserve partial receipts. Paint target changes move previous coverage into the ordered two-row inverse. Framework ConfigApply hooks remain domain neutral, with fixed default validation and one inverse for existing users.

## Authored Verification

These are source assertions, not execution results:

- Shared pixel flood fixtures run the new resumable flood against existing neutral outputs, with tiny grants and cancellation.
- Surface geometry tests cover intrinsic coordinates and image-key invalidation.
- Native selection tests cover small intent publication and cancellation after pointer release; production `frame_into_step` tests advance an unchanged document across frames, verify pending-work becomes false, and scope admitted cancellation against foreign actor/instance/window and retired host.
- Retained-input neutral cancellation tests ensure cancelling an exact old handle cannot cancel a newer source.
- SelectPixels neutral fixtures cover rectangle, ellipse, lasso, wand, all, invert, none and merge modes, with image crate oracles where applicable. Additional tests cover incomplete result withholding, mid-shape/wand/combine cancellation, input-root conflicts, stale image keys, malformed JSON, 4096/4097 vertices and large whitespace pages.
- Config preparation fixtures cover 64 validation pages, malformed late rows with exact receipts, move-only inverse coverage custody, and cancellation closure.
- Scene carrier round trips compare native value encoding with serde JSON; invalid schema parity tests are being added by the React agent.

## Actual Tool Evidence

One MCP native surface test target was requested. It reached a terminal 300-second timeout without test diagnostics; this is not a passing result. Parent's repo build investigations found sandbox FD3/FD0 pipe writes fail with EPERM after admission. No Rust compile or native test pass has been obtained. Read-only source inspection through supported direct `repo` script ingress succeeds. No additional approval calls or duplicate test launches were made.

## Open Acceptance Work

### Later Source Checkpoint

The flat RLE publication limitation below is being removed: `RasterPixelSelection.spans` now owns a firstparty `PersistentList`, and SelectPixels streams runs into `SelectionSpanWriter` using actual shared-row/page admission receipts. It keeps one pending scalar run, no complete flat RLE copy. Config clone/publication aliases the immutable root, while the original producer pays page allocation. Old public `setPixelSelection` command/action/schema/proofs were removed by parent decision; current census is 33 commands, 30 general retained commands plus dedicated selection, mask and export factories. Internal config `SetPixelSelection` remains the authoritative owner output.

Shape rasterization now uses one canonical incremental row initialization, edge scan, heap sort and fill state machine. Native work uses bounded primitive/write grants and checks its supplied deadline/cancellation callback; row-granted reference calls use the same implementation. The SelectPixels contract/proof/neutral execution fixture explicitly declare 1024 CPU units per step. Actual shape work and scratch rewrites are charged separately from retained capacity.

Selection geometry now uses the shared granted layer lookup and rejects hidden targets while allowing locked targets. Shape work stops geometry preparation before any image pixels are reserved. Prior coverage restoration caches one span across pixel writes and quotes ranked depth instead of repeatedly traversing the page tree for every pixel.

Fixture migration uses actual original NativeReceive and funded retirement from `OriginalWorkFixture`; cold persistent-list construction is intentionally unsupported. Nonempty config Pack reload is still an acceptance seam: RasterConfig currently declares its reference codec and must expose the native receiving/source route. Scene JSON generation and parsing also remain monolithic despite paged transport. The earlier flat-vector limitation notes below describe the superseded checkpoint, not an acceptable final implementation.

- Large selection publication still reaches the existing 1 MiB one-item retained envelope. The new config preparation honestly reports payload cost and refuses oversized flat RLE; it does not claim full-capacity support. `PersistentList` is a candidate for shared paged coverage but currently lacks the selection wrapper's DSL/value/canonical/receiving integration. Silently reporting baseline-only cost or widening global budgets is not acceptable.
- Generic action bridging still folds/copies the up-to-1-MiB path before typed decoding. It needs original owned path handoff to eliminate the remaining whole-string bridge copy.
- Shape scratch rewrites need actual per-step byte receipts; shared pixel agent is adding the shape counterpart of flood `last_write_bytes`.
- Shape target validation currently calls synchronous visible-layer lookup. Shared source agent now exposes granted `advance_geometry`/`geometry_ready`; selection should use that geometry path without preparing source pixels for shapes.
- Config validation hooks execute through the Store retained interface, which has no StepContext parameter. The enclosing Store scheduler's work/fuel receipt must be verified independently from retained byte accounting.
- Rust source remains uncompiled. No broad native or end-to-end completion is claimed.
## Packed Receiving Continuation

The packed builder and writer now share one page constructor. Native array receiving retains the original ranked source binding and one span receiver, then pushes rows into that constructor. The DSL owned receiver moves one original list cell at a time through the existing owned receiving slot and uses the same packed constructor. Both retain all partial ownership through explicit funded close paths. These sources are authored and mounted; no compiler or test execution verdict is available yet.

New neutral receiving laws cover flat row order across page boundaries and cancelled partial DSL pages, using the actual original caller and decoder. Serde provides the independent flat tuple sequence oracle. Config validation now reads one packed page slice and one predecessor per quantum, requests the actual rank depth, and caps each quantum at 256 spans. Its CPU work still requires transport through the generic retained edit interface; coordination with the owner of original work transport is in progress.

SelectPixels publication now reserves its one mutation row under a separate admitted turn, retains that capacity during cancellation, and moves the prepared row vector into Emit. It no longer allocates a completed mutation vector at the terminal publication expression.

The parent is checking active repo runners before admitting another native compile. Earlier timeout observations remain neither compile failures nor test passes.

The new `selection-audit.ts` source audit passed with 15 Rust sources, 17 resolved source/fixture includes and four parsed JSON fixtures, including all local session Cargo dependency paths. Command: `SEMIO_LANGUAGE=en repo .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/selection-audit.ts`. This verifies source paths and fixture syntax only, not Rust compilation.

Config genesis/catalog sources and a genuine original Pack receiving/cancellation law are now authored under `🎚️config/🌱️initialization`. The catalog installs original mutation receiving and the shared retained ConfigApply preparation. Config and every nested required record explicitly derive only the owned DSL construction contract. The SDK audit found no Raster `OriginalArtifactAppBirth` declaration; the parent owns that actual app/document registration. Therefore runtime reload remains unproved.

The source audit was broadened to the SelectPixels command: current result is 18 Rust sources, 21 resolved includes and seven parsed JSON fixtures; all local session Cargo paths resolve. Selection raw wire admission is now explicitly `2 * 1 MiB + 3 * 4096 * 6 + 65536 = 2236416` bytes, covering escaped JSON path whitespace and bounded metadata without changing decoded path or point limits. Neutral tests author LF/tab escape and maximally escaped metadata vectors against Serde, and reject vertical-tab/form-feed as non-JSON whitespace. The path loop now charges bounded whitespace/token units and checks the original deadline/cancellation callback; prior coverage restoration charges the actual canonical ranked read work.

Live shape construction now uses `PixelSelectionJob::begin`: only fixed shape metadata is checked at construction, while polygon coordinate finiteness is validated in the same metered, stoppable advance machine before any coverage initialization. The explicit cold `new` constructor keeps its reference validation. A neutral budget fixture and late-NaN test require validation of all 4096 points through bounded work turns with zero initialized coverage. Config validation exposes a bounded next-work quote before its actual work, and ConfigApply captures both successful and failed validation work for the shared preparation transport. These laws remain authored, not executed.

Remaining boundaries include actual app genesis/reload registration (parent owned), generic preparation-owner work-method census (native-controls owned), native compiler/test execution, production scene JSON preparation/transport for large coverage, and a measured packed-coverage peak allocation audit. The current SelectPixels output envelope remains the prior conservative 64 bytes per pixel; it has not been narrowed from measured packed page costs.
