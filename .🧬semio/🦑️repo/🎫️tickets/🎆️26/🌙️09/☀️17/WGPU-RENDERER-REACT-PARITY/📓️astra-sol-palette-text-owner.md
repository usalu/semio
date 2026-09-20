+# WGPU Palette Text Owner Lifecycle

## Runtime failure

Checkpoint 16b reproduced a WGPU-only terminal fault after the physical sequence Close → reopen → outside dismiss → reopen. The first terminal console entry was:

```text
wgpu renderer fault: worker-input-failed: text edit step failed: Protocol
```

The same hardened sequence completed in React. The WGPU trace showed the final palette chord reopening Search with `focused=Some("ui.search.input")`, followed by the protocol fault while the bounded text step advanced.

## Ownership defect

`InputState::reset_text_view` cleared `text_projection_pending` on focus or blur, although `TextEditAuthority` could still own a checked-out projection. The next queued replacement eventually reached `Published` and attempted `start_projection`; the authority correctly refused the second simultaneous projection as `TextEditFault::Protocol`.

Visible view reset and projection retirement are separate obligations. The repair leaves the pending flag live through focus/blur. The bounded driver finishes that existing projection before it advances another edit and starts the current projection. No fault is caught, translated, retried, or discarded.

## Contract

The neutral schema and fixture are:

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/⌨️text-owner-lifecycle/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧪️fixtures/⌨️text-owner-lifecycle/🔣️.json`

They fix the sequence to focus → advance to a real projection → blur → same-owner refocus → bounded drain, with the current owner and value preserved. The Rust law also directly starts two simultaneous projections and requires the genuine second start to remain exactly `Protocol`. React checkpoint 16b is the independent current-runtime reference for the close/reopen journey.

## Test receipts

Fail-first UI57 selected one test and failed exactly at the expected boundary:

```text
close/reopen must not manufacture a protocol fault: Protocol
0 passed; 1 failed; 645 filtered out
```

After the one-line ownership repair, UI58 ran the complete WGPU UI suite:

```text
646 tests run: 646 passed, 0 skipped
```

Receipts are `🗑️generated/astra-runtime/native-ui-57.log` and `🗑️generated/astra-runtime/native-ui-58.log`.

A fresh activated WGPU browser journey is still required. Checkpoint 16b used the pre-repair WASM cohort, so this report does not claim live runtime acceptance.

