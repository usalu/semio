# Independent Typed-Command Full-Operation Audit

Date: 2026-08-26

## Verdict

**GREEN only for the narrow fail-closed admission boundary.** Every presently reachable typed-command call enters `require_complete_tool_operation_pipeline` before `refresh_cache`, root snapshots, operation allocation, or worker dispatch; that helper unconditionally returns `interactive-job.full-operation-pending` after validating the supplied contract. Therefore the legacy monolithic tail cannot be reached through this route.

**RED for a runnable full-operation foundation and production readiness.** The retained type is intentionally not live, has no reducer admission, has no source-visible freshness admission caller, and contains two unbounded fault-message copies in post-reducer branches. Its static scaffolding must not be used as evidence that an arbitrary generic command is now bounded.

No production source was edited. No Cargo, Nx, Wasm, browser, or Git-mutating command was run.

## Verified Narrow Green Boundary

| Check | Result | Evidence |
| --- | --- | --- |
| Fail closure precedes legacy root work | GREEN | `dispatch_typed_command_inner` calls `require_complete_tool_operation_pipeline(&admission)?` at line 19409; its first `refresh_cache().await` is line 19414. The helper always returns an error at lines 19388-19398. |
| Capacity saturation cannot reach legacy roots | GREEN | The fixed-registry admission at line 19411 is after the unconditional gate, so neither a full registry nor an empty registry proceeds to cache/snapshot work. |
| Generic reducer calls are absent from retained `InteractiveJob` implementation | GREEN | Focused census found `A::handle=0`, `A::ephemeral=0`, and `serde_json=0` between the job implementation and factory. Reducer instead faults at lines 15285-15288. |
| One-unit cancellation/fuel/deadline guard | GREEN | Each `step` checks cancellation, then `should_yield`, then zero fuel before its single fuel consumption at lines 15243-15252. `StepContext::should_yield` is the job-module deadline-or-fuel predicate (`🧰️framework/🔨️modules/🧵️job/🦀️component.rs:776-778`). |
| Schema fixture arithmetic | GREEN | The fixture declares 13 roots, 12 lanes, six stages, and four cases. A focused Bun oracle produced no mismatches. |
| Formatting | GREEN | `rustfmt --edition 2024 --config skip_children=true --check` on the plugin component exited 0. |

The retained job enumerates exactly thirteen preparation ownership checks (lines 15259-15271), uses explicit prepare/reducer/output/ephemeral/emit/expose stages, and its close path hands back strings one Unicode scalar at a time for `description` and `coalesce_key` (lines 15406-15423). The mounted shell takes a checked-out terminal outcome, releases it incrementally, then either resumes the session or starts close (lines 15081-15122); that is a real outcome handback/resume/close sequence, though it currently has no successful typed-command producer behind it.

## Adversarial Findings

### RED-1: Two hidden unbounded fault-message copies remain inside the retained job

At lines 15298 and 15378, `fault.message.as_bytes().to_vec()` clones the entire arbitrary `Fault` message before constructing the retained fault payload. No fixed message cap, byte cursor, or page owner guards that allocation. The current reducer faults before either branch becomes reachable through the route, so this does not pierce the narrow fail-closed gate; it does invalidate any stronger claim that the type itself contains no whole-output work.

Required remediation: preserve fault text as a bounded retained producer (or replace it with a fixed fault code/detail) and prove exact cap/handback behavior. Do not move either copy to a helper.

### RED-2: Post-reducer phases have no live freshness admission or executable producer

`admit_exposure_freshness` is defined at lines 15230-15238, but the focused source inspection found no caller. `Expose` therefore checkpoints indefinitely until external state is somehow injected, rather than receiving one retained revision/generation grant. `emit` and `ephemeral` are also initialized to `None` by the only construction site (lines 19437-19491), while `Reducer` unconditionally faults. This is accurate fail closure, not a resumable command path.

Required remediation: introduce an app-owned retained reducer interface, retained root-generation capture, and a one-grant freshness admission owned by the mounted operation. Then make `Expose` publish an acknowledged result page only after `validate_commit` accepts the exact current revision and generation.

### RED-3: Item cursors are declarations, not item transfer implementations

`output_item_cursor`, `ephemeral_item_cursor`, and `emit_item_cursor` are retained fields/checkpoint counters, but the implementation advances only lane cursors. Nonempty mutation, effect, event, child, task, presence, and transient lanes correctly fault rather than transferring their complete collections (lines 15299-15353). This is fail-safe today, but it is not proof of per-item pagination or publication.

Required remediation: add typed item/page codecs and one owned store/event/child/task transfer per grant; retain the exact item, byte, and retry cursors across cancellation and resumption.

### RED-4: The fixture does not test the stated hostile boundaries

`shared-typed-command-full-operation-v1.json` only covers empty and ASCII single/max/+1 output arithmetic. It has no multibyte UTF-8 case, parent-owner release-credit case, zero fuel, expired deadline, cancellation in every phase, stale revision/generation, missing root, registry saturation, exact close idempotence, fault-message cap, ACK/retry, or owner handback law. The Rust tests are source-present but were deliberately not compiled in this audit.

Required remediation: extend the language-neutral fixture and both owned/third-party oracle laws before live admission. Include a four-byte scalar and an insufficient-credit close probe, not merely ASCII byte totals.

### RED-5: Generic app calls still occur before the gate in surrounding route entry paths

The specific forbidden reducer calls are absent from the retained job, but the broader dispatch flow invokes app-controlled `command_from_action`, `command_from_intent`, and `command_id` before the central gate (for example lines 19335-19337, 19401-19402, and 20652-20657). Those calls are outside this partial's stated legacy-root boundary and are not asserted bounded by the retained job. They remain a production interactivity blocker until command decoding/identity is itself an exact preflight or retained operation.

## Focused Commands And Observations

```text
bun -e <fixture owned-oracle parity>
{"schema":"semio://framework.plugin/typed-command-full-operation/v1","roots":13,"lanes":12,"stages":6,"cases":4,"bad":[]}

bun -e <retained-job and route census>
{"job":{"stages":6,"genericHandle":0,"genericEphemeral":0,"serde":0,"loops":1,"wholeFaultCopy":2,"roots":13},"route":{"gate":783,"refresh":1348,"genericCommandId":186}}

rustfmt --edition 2024 --config skip_children=true --check <plugin component>
exit 0
```

The one textual `for` result is the Rust `impl ... for TypedCommandFullOperationJob<A>` header, not a loop. There is no `for`, `while`, or `loop` statement in the retained job body.

## Production Admission Blockers

1. Eliminate the two full `Fault` message copies with a fixed-cap retained fault mechanism.
2. Move generic command conversion and identity work behind exact preflight/retained authority.
3. Replace unconditional route rejection only after bounded root capture, app-owned reducer, typed output codec, per-store publication, freshness admission, and result ACK are all live.
4. Add multibyte, saturation, cancellation, deadline, stale, retry/ACK, and close/handback fixture-oracle laws; run native and Wasm after Rust ownership is quiescent.

Until all four are complete, the correct status is: typed commands deliberately reject before legacy root work; no typed command is production-resumable through this foundation.
