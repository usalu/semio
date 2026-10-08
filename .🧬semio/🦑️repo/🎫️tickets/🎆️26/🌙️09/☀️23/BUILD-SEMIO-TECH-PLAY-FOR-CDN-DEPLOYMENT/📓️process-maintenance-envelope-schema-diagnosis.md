# Process Maintenance Envelope Schema Diagnosis

The actual maintenance case `vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed` emits JSON in the canonical Process editor unit owner via `production_envelope_wire`. The live decoder reports `schema-json.missing-required-field` at byte offset 2181 before accepted maintenance publication. This is a distinct input/schema investigation from the publication agent’s Edit-array retirement demand repair. No input or schema changes have been made yet.

The fixture constructs top-level schema/id/vcs/editMessages/conflicts and one edit with id/actor/forwards/inverse/sequenceNumber/startedAt. Current authority/schema requirements are being compared directly.

## Confirmed required field

Canonical replication Edit JSON schema (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧬️schema/🔣️.json`) requires `line` alongside id/forwards/inverse/sequenceNumber/startedAt. `line` is string or null; explicit null means trunk or unknown provenance. `Edit::to_value` always emits line. The retained SPR edit cursor also requires it and accepts null. The authored Process fixture omits only this required field; decoder strictness is correct. The actual logged offset is 2181; no byte-position reconstruction has been claimed.

A three-row language-neutral corpus covers explicit trunk, Unicode authored branch, and absent provenance. The independent Ajv source gate validates it against the canonical Edit schema and compares the actual Rust fixture macro fields. Its actual Nx invocation is queued on graph construction; the existing maintenance runtime failure is the original semantic red.

## Source gate harness repair

The first source invocation exited 1 before semantic assertions because its fixture URL traversed one parent too many (ENOENT); this is not a valid missing-field regression receipt. Corrected both fixture and canonical schema paths relative to import.meta.url so Nx package cwd is irrelevant. The semantic red retry is active. All source/gate paths are registered in both launch files.

## Semantic red and canonical fixture repair

Actual uncached scoped Nxexec semantic red exited 1: zero pass / one fail / five assertions. Neutral explicit-null and Unicode branch cases passed Ajv; missing-line case was refused; the actual authored Rust maintenance edit failed the same required-field schema check. Added exactly `"line": null` to that edit, preserving trunk identity, full mutation payload, original publication/hostile authority assertions, work grants, polling and ACK/retirement lifecycle. No decoder or cleanup source changed. Green source and final maintenance runtime are still required.

Actual source green completed Nx exit 0: one passed, zero failed, six assertions, including independent Ajv validation of all three neutral branches and canonical authored fixture. Final original maintenance runtime still awaits shared physical retirement closure.

## Original Runtime Receipt

Publication owner reports original Process maintenance session28071 actual Nx exit0: one selected native case passed,379 cases outside selection, native158.720s, target10m58. It used the corrected strict edit fixture and current shared history-demand chain. This is a real lifecycle receipt for the original maintenance case; its previous900000ms budget was unchanged. The receipt predates the separate Process mutation-wrapper clamp repair, so final full380 current-source validation is still required. No assertion, actor, archive byte contract, release grant, or closure witness was relaxed.

Independent direct log inspection confirms PASS for `vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed`, native158.719s, Summary1passed379outside, and Nx Successfully ran target. Retained log: `🗑️generated/process3d-maintenance-valid-fixture-current.log`.
