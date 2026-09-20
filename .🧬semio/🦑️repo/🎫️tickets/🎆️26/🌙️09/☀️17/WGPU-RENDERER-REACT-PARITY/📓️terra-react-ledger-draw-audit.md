# Terra React Ledger and Draw Receipt Audit

Read-only audit on 2026-09-20. No production or probe code was changed, and no build or browser command was run by this audit.

## React ledger has no action-argument diagnostic

The React host accepts an action's `args?: unknown` at the ingress boundary ([🎯️input-ledger/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts:47)). It intentionally does not retain it:

* `InputLedgerEntryV1` contains only `provenance`, `controllerId`, and `action` ([🎯️input-ledger/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts:153)).
* `issue()` constructs precisely that reduced entry ([🎯️input-ledger/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts:231)).
* The public `InputLedgerRecordV1` is exactly `inputSeq, controllerId, action, origin, windowId, causedBy, outcome`; there is no `args`, preview, envelope, or descriptor field ([🎯️input-ledger/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts:167)). `outcome` can provide an applied/superseded/refused terminal result and a refusal's reason/detail, but cannot restore dispatch arguments.

The dev-only global is a census plus an already-materialized `recent` array, not a full dispatcher trace: [🏛️ShellHost/🟦️.tsx](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4428). It is available only in a Vite development build ([🏛️ShellHost/🟦️.tsx](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4385)), has the bounded ledger history, and does not gain arguments when `SEMIO_RUNTIME_DIAGNOSTICS` is armed.

That makes the Draw5/6 `args: null` observation expected. Canvas adapter serialization at [📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:250) computes `JSON.stringify(row.args ?? null)`; since `row.args` is absent, every React row becomes the literal string `"null"`. It is not evidence that the underlying action had null arguments.

The main parity probe already uses the correct React boundary: it reads identity, provenance, and outcome only at [🐍️parity-interact-probe.mjs](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🐍️parity-interact-probe.mjs:439). Its WGPU companion does expose `dumpChrome().actions[].args` ([🐍️parity-interact-probe.mjs](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🐍️parity-interact-probe.mjs:581)). The argument capabilities are therefore asymmetric by design; an argument assertion cannot be presented as a shared ledger receipt.

### Required Canvas-probe boundary

React action checks may use only sequence, controller, action, origin/window provenance, and settled outcome. They must not inspect `ActionRow.args`.

The current Layout probe already has the right React semantic witnesses:

* the physical source handle carries `data-drag-mime` and `data-drag-payload`, read at [📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:887);
* the real DOM drag receipt contains trustedness, addressed host, MIME types, and drop payload; the React branch checks it at [📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:927).

Leave those DOM witnesses as the React proof of Layout MIME and payload. WGPU can continue to decode its own retained action args. The one current false assertion is `selectLayoutDemo`: it asks `args.includes("demo")` for both renderers ([📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:812)). For React, require the physical exact Demo option action plus the known Demo projection already checked by the subsequent seed collection counts and mounted canvas. Retain the WGPU args check.

## Draw one-shot utility is intentional

Every nonempty shape/draft commit calls `commit_with_utility_reset`, which emits a `SetActiveUtility` effect to `DRAWING_DEFAULT_UTILITY` ([canvas-pointer-down/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs:205)). The default is `selectDirect` ([drawing/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:43)).

The shell applies that guest effect as an unconditional host-owned register write ([🏛️ShellHost/🟦️.tsx](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5777)). Its next utility widget press is guarded by the generation the widget rendered: descriptors are stamped with `windowId` and `expectedGeneration` at [🛠️ShellHelpers/🟦️.tsx](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:2089), and the shell's compare-and-set writes and bumps render state at [🏛️ShellHost/🟦️.tsx](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4444).

Thus the Draw6 image showing Direct Select after the first rectangle agrees with the producer and shell. It is not evidence that shapeRect should remain armed. A physical adapter must, before every independently expected shape:

1. re-resolve the Shape Rectangle control after any group collapse/repaint;
2. read its current `aria-pressed`;
3. click only when it is not true, then wait for it to become true; and
4. after the commit, require `selectDirect` to become pressed before starting the next case.

The current `utility()`/ `drawRectangle()` structure embodies this observable contract at [📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:669) and [📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:688). A React ledger row cannot prove the expected generation value; `aria-pressed` is the appropriate physical receipt.

## Draw modifier selection and engagement rename are semantically observable

The React Canvas host transmits Shift, Control, and Meta on both pointer begin and end ([Canvas2dHost/🟦️.tsx](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🟦️.tsx:436)). Draw maps them to framework-owned merge modes: Shift is additive, Control/Meta subtractive, both are invertive, otherwise replace ([canvas-pointer-down/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs:60)). The final set algebra deliberately happens in the framework interaction state, not inside the Draw document producer.

`engagementSubmit` is a valid physical consequence channel even though the ledger omits its value. The action renames exactly one interaction-selected layer and becomes a no-op for an empty value or any selection cardinality other than one ([engagement-submit/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️engagement-submit/🦀️.rs:16)). The existing adapter's row-text assertions therefore correctly prove the sequence:

* direct select plus a unique submitted name changes only the captured first layer row;
* Shift-selecting the second layer makes the next submit inert;
* Control/Meta-selecting that second layer back out restores one selected layer, and the following submitted name changes only the first row.

Those consequences are recorded through visible, scoped layer rows at [📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:720). They are stronger than attempting to infer merge or name text from absent React ledger args. The receipt must continue to capture both final row names and the multi-select no-mutation case; no new production diagnostic is needed.

## Finding

Confidence is high. React argument rows are structurally unavailable, and the planned Draw utility re-arm behavior matches the producer's one-shot contract. The only required correction is in probe evidence selection: use renderer-specific real DOM drag receipts for React arguments and retain action identity/outcome as the common ledger comparison.
