# Terra Main16 Palette Text Owner Reconciliation

## Scope

Read-only reconciliation of `checkpoint-16-full`'s terminal WGPU error with the current retained-text and palette source. No Cargo, Nx, browser, activation, or production change was run.

## Result

The recorded WGPU crash was an actual historical production failure, but its specific retained-text owner cause is repaired in the current source. The failing WGPU console artifact ended at 2026-09-20 15:00:37; the current input implementation and its regression were modified afterwards at 16:21. This establishes source inclusion after the observed failure, not browser acceptance of the newly built WGPU target.

The current source has no source-level counterexample to the original close/reopen protocol fault:

- Palette open, accessibility focus/value, and close all route through `InputState::focus_input_owned` or `blur_input`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13458-13480,15108-15131,16020-16031`.
- Current `reset_text_view` clears only the visible projection and cursor; it does not clear `text_projection_pending`: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:482-521`.
- `drive_text_step` drains an existing projection before advancing another edit. It starts a replacement projection only after the prior pending projection has completed: the same file `:554-576`.
- This ordering matters because `TextEditAuthority::start_projection` correctly returns `Protocol` for a simultaneous second projection: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🪢️text-edit/🦀️.rs:442-456`.
- The browser worker continues to preserve a bounded TextEditStart → TextEditChunk → TextEditCommit protocol: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:507-574`. The renderer turns a text-step fault into the reported terminal string: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:15629-15642`.

The historical trace supports the ownership sequence: after `⌘P`, the shell records `focused=Some("ui.search.input")` before the terminal `Protocol` error. See `🗑️generated/astra-runtime/checkpoint-16-full/wgpu/console.txt:20701-20706,20763`. The current repository's owner report identifies the old defect as clearing `text_projection_pending` during focus/blur, which made a later edit attempt a concurrent projection: `📓️astra-sol-palette-text-owner.md:8-17`.

## Existing Fail-First Law

The new schema fixture fixes the real palette owner and sequence:

```json
{
  "owner": "ui.search.input",
  "sequence": ["focus", "advance-to-projection", "blur", "refocus", "drain"],
  "expected": { "fault": null, "duplicateProjectionFault": "Protocol" }
}
```

It lives at `🧰️framework/🔨️modules/🖱️ui/🧪️fixtures/⌨️text-owner-lifecycle/🔣️.json`, governed by `🧰️framework/🔨️modules/🖱️ui/🧬️schema/⌨️text-owner-lifecycle/🔣️.json`. The production law advances until a projection is truly pending, blurs and refocuses that owner, then drains it under a fixed bound without a fault; it also retains the direct second-projection `Protocol` refusal: `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-input-unit/🦀️.rs:4-34`.

That law is the minimal regression for this defect. Replacing `text_projection_pending` reset into `reset_text_view` would reproduce the original `Protocol` fault; retaining it makes the queued current value project only after the old projection drains. The existing owner receipt records its fail-first boundary and the following complete UI-suite pass in `📓️astra-sol-palette-text-owner.md:20-35`; this audit did not rerun either.

## Separate Probe State Defect

The later React error is not evidence that the retained WGPU protocol remains broken:

- `checkpoint-16-full/parity.md:54-55` records the WGPU crash in `command-palette-activate`, then an unmeasured WGPU keyboard step. In the same run React's keyboard step rejects because its focused input contains `"Design festlegen"`.
- The same browser helper invokes `command-palette-activate` and then `command-palette-keyboard` in one persistent renderer session: `🐍️parity-interact-probe.mjs:767-768`. It demands a focused *empty* input at every invocation: `:866-889`.
- React intentionally retains a query on an unactivated dismissal and clears it only in `handleSelect`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔎️ShellSearch/🟦️.tsx:41-75`. The prior activation step timed out, so its query did not establish the selected-and-cleared precondition. The second step's empty assertion therefore conflicts with React's current ownership semantics.

The browser law must be conditional: a keyboard palette case that requires an empty input must begin in a new renderer session, or first establish a successful selection that clears the query. If it follows an unactivated/dismissed palette, it must require focus and the retained expected query instead. The helper already has an explicit `reopenQuery` assertion for the preserved-query case at `🐍️parity-interact-probe.mjs:955-995`.

## Confidence and Limit

High confidence that the source repair covers the historical `Protocol` cause: current owner state, authority refusal, schema fixture, and regression all agree, and their file timestamps follow the failing artifact. Medium confidence in live acceptance: no fresh WGPU activation/browser journey was run here, so a different current runtime input fault has not been ruled out.

