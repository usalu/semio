# Cargo Capture And Final Inventory Audit

Read-only production-source audit2026-11-04. No compiler/build queue or source edit.

## Whole-Capture Contract

readCargoObjectCaptureV1 parses the complete input exactly once with system JSON.parse and returns only a non-null, non-array object. Leading/trailing JSON whitespace and pretty objects are valid; junk, JSONL/concatenation, truncation, arrays and scalar roots refuse. Failure returns undefined without including captured bytes or exception detail.

Reviewed all three consumers: actualCargoCompilerPlanningPortV1.capture, withRepositoryCargoConfigurationV1 effective configuration capture, and proveCargoCompilerConfigurationAdmissionV1 runtime law. Each refuses undefined with a generic preexisting domain error and continues existing semantic profile/configuration/source/admission checks afterward. No secret stdout is added to errors, no external runtime dependency or package/lock edit is introduced, and no guard is bypassed. Existing bounded capture byte/line limits precede parsing.

Tests independently use strict jsonc-parser and Ajv fixture-schema validation. The separate Ajv JTD oracle explicitly records its pretty-empty-object limitation; production/native JSON test does not skip the case. No additional concrete contract or callsite defect found in inspected code. Actual native runtime remains pending.

## Diagnostic Source Closure Refresh

Exact same cfg=["test"] sample rerun: graph2d3209lines/183397bytes/18files; grid2d2227/152236/13; graph3d3758/223406/21; grid3d3988/243326/25. Non-authoritative and not an upper bound. Detailed generated diagnostic file refreshed; no source refactor based solely on it.

## Worked Inventory And Snapshot

Union inventory now230paths, preserving prior worked paths plus current WFC changes and eight global capture/parser/schema/test/oracle paths. Fresh snapshot generated/provenance-r2-after-capture.json captured22:04:58UTC:209input hashes,71JSON parsed,maxabsolute194UTF16. Descriptor semio is now375765byte binary with NUL atoffset2; NUL textscan flags it by design, not evidence of corruption.

A sequencing error during inventory-script modification ran the previous script after its patch failed, overwriting the21:10 generated/provenance-r2.json baseline. The preserved21:09:35 initial snapshot remains intact and was restored to that canonical filename; new22:04:14 output was preserved separately as provenance-r2-220414.json. The exact21:10 extra clean-lock metadata is not claimed recovered. Parent was notified immediately. Future comparisons must use the actual capturedAt field, not assume the canonical filename is21:10.

The input script now supports an explicit output filename and includes global fix paths. Parent should retain the initial and after-capture snapshots until comparison, then remove generated outputs at ticket completion.

## Provenance Correction And Overwrite Guard

Use the preserved explicit 🗑️generated/provenance-r2-initial.json path for initial comparisons: captured21:09:35UTC with182inputs. The canonical provenance-r2.json now copies that earlier snapshot; it is NOT the lost21:10/189input snapshot or digest. Its seven additional lock records were not recovered. The separate after-capture snapshot has209inputs. Actual native SourceAdmission authority is unaffected by this optional audit evidence loss.

wfc-provenance.cjs now requires an explicit new basename ending.json, refuses existing output before status/inventory writes and writes the hash manifest with exclusive flag wx. A real invocation targeting existing provenance-r2-after-capture.json refused before mutation with exit1. The script parses successfully. No source/descriptor changed.

## Streaming And Deadline Adapter Final Review

Reviewed current machine planning adapter and existing runner. Planning keeps declared128MiB/65536line envelope; runOwnedCommand enforces finite output limits, cancels child tree on refusal, waits for child settlement, propagates nonzero/signal/cancel/timeout, and invokes synchronous byte consumer. Original issued control, its exact signal and resource process environment remain in place. Remaining timeout is positively checked then floored; sub1ms refuses, null remains0 with Original signal authority. No admission ceiling changed.

readCargoObjectStreamV1 consumes stdout only through fatal UTF8 TextDecoder with stream:true and final decode flush, then joins once and parses one object. Stderr never enters the JSON value. Config retains separated4MiB capture and terminal-success checking. General captureOwnedCommandV1 human-tail implementation was not changed by these callsite switches.

Concrete documentation mismatch: report says no stderr/configuration values copied to errors, while current CargoPlanningFailureV1 appends the bounded stderr diagnostic ring to its failure message. No raw stdout is appended. Existing stderr forwarding remains, but documentation must distinguish bounded stderr error context from stdout confidentiality. This finding does not request modifying an active runtime.

Deadline fixture search found no consumer import exercising numeric floor cases in the inspected helper tests. The current report accurately calls them portable fixtures validated by Ajv and separately cites actual integration6887 red/77440 passed boundary; it does not claim a six-case executed adapter regression. Actual compilation progress is integration evidence of passing that boundary, not a substitute for all fixture assertions.

No additional concrete adapter/control/UTF8 defect established. Native77440 was not edited, restarted or otherwise disturbed.
