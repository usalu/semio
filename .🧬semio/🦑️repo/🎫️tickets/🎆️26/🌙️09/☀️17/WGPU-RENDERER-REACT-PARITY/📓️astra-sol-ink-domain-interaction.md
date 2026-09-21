# Ink Canvas Domain Interaction

## Scope

The shared `🖋️ink-canvas-domain-interaction` law now drives both renderer hosts. Ink selection and hover publish through the framework interaction domain, document items supply scoped topology identities, and scene selection/hover remain raw paint identities.

## Diagnosis

The focused React oracle initially produced **3 failed / 2 passed**. `InkCanvasHost` still dispatched the removed app-private `setHover` and `setSelection` verbs with raw block ids. The WGPU interaction job contained the same private writers. The shared scene schema had no Ink domain envelope, while Note's canvas projection omitted each block's `note-play-block:*` interaction identity and its request-context renderer discarded current interaction presence.

## Implementation

- `InkCanvasScene` now carries one optional `interactionDomain: { id, granularityId }` record across TypeScript, Rust serde, `ToValue`/`FromValue`, and the retained typed-scene catalog. The record is structurally whole; serde and `FromValue` reject missing, empty, unknown, or duplicate members; and `InkCanvasScene::base` leaves it absent for genuine non-domain surfaces. React and WGPU also refuse manually constructed empty records.
- The public Ink action catalog no longer exposes `setHover` or `setSelection`.
- React publishes `interactionHover` and `interactionSelect` with JSON-text topology targets, exact merge/method fields, and empty clears. A picked item without `interactionId` is refused; a rectangle omits unavailable targets. Incoming `selectionJson` and `hoveredId` still paint raw ids.
- WGPU uses the same canonical action envelopes from the bounded `InkInteractionJob`. Hit and marquee scans map raw document blocks to their scoped `interactionId`; raw ids remain the move/edit/paint identity.
- Local editing and movement remain available on a genuine non-domain canvas, matching React. Such a canvas emits no selection/hover action. A domain canvas whose item lacks `interactionId` likewise refuses publication without disabling its local editor or substituting the raw id.
- Note projects `interactionId: note-play-block:{rawId}` on every block recursively, removes that renderer-only field on the inverse wire conversion, and renders the composite scene through `render_with_request_context`. Current `blocks` selection/hover are converted from scoped topology ids back to raw paint ids.
- The Ink story reducer and every Rust `InkCanvasScene` literal were updated with the new contract.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️scenes-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/📇️catalog.json`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️fixtures/🖋️ink-canvas-domain-interaction/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/🖋️ink-canvas-domain-interaction/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖋️ink-canvas-domain-interaction/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost/📖️stories/🧪️.story.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-ink-canvas/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🦀️.rs`

## Verification

Fail-first command:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test-long '../../../../🧪️tests/🖋️ink-canvas-domain-interaction/🟦️.tsx' --silent=false --reporter=verbose --skip-nx-cache
```

Initial receipt: **1 file, 3 failed / 2 passed**.

Corrected strict-envelope receipt from the same command: **1 file, 5 passed / 0 failed**, duration **10.43s**; Nx target completed successfully with cache skipped. The schema law also mutates the record through both missing-member forms and both empty-member forms and requires all four to be refused.

Static receipts:

- `git diff --check` over the Ink/shared-scene/Note file set: clean.
- retained typed-scene catalog and fixture/schema JSON parsed: pass.
- all repository `InkCanvasScene { ... }` Rust literals were scanned and carry the new fields.

Root owns the Cargo/native lane. Exact registered native filters supplied for Native60:

```text
ink_canvas_domain_hover_uses_scoped_topology_ids
ink_canvas_domain_picks_use_scoped_topology_ids
ink_canvas_domain_marquee_uses_scoped_topology_ids
```

Native61 executed the new domain laws successfully but exposed one regression in the pre-existing non-domain Ink editing lifecycle: the first domain implementation coupled local editor activation to successful canonical publication, so the fixture with no interaction capability could not focus its text editor. The corrected WGPU path treats local editor/drag state and framework interaction publication as separate outcomes, exactly as React does. The existing native lifecycle filter is `ink_canvas_text_and_table_editing_matches_the_react_host_lifecycle`.

The strict UI-scene filter is `ink_canvas_document_payload_is_document_json_on_both_encoders`; it now includes real negative `FromValue` cases for half, empty, unknown, and duplicate domain records in addition to serde refusal.

Root executed that exact UI-scene filter after Native62: **1 selected test passed**, 140 tests remained outside the filter, duration **15.1s**. An earlier invocation supplied `--nocapture` to the Nx wrapper rather than the Rust harness and selected no test; it is not counted as execution evidence. Native62 also executed the repaired pre-existing Ink lifecycle law successfully. The full Native62 remainder was exactly the two locale-fixture failures documented in `📓️astra-sol-locale-retained-refresh.md`.
