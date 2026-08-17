# W1 Runtime Outcome Frozen Audit — P1 Remediation

## Scope and result

All 6 reopened P1 findings in `📋️w1-runtime-outcome-frozen-audit-v2.md` are remediated in the owned runtime scope. The repository-wide `MutationDiff::apply` P0 was deliberately not changed.

| Audit P1 | Remediation |
|---|---|
| Operations → Snapshot durable ledger identity | Snapshot merge now remaps a source ledger edit id to the unique established local edit with identical stable operation identities and payloads before preflight/merge. Ambiguity, unknown source edits, and colliding remaps fail explicitly. |
| Host outcome/conflict round trips | Backbone envelopes now carry the applied-edit cursor for both text and binary export. Text preserves durable messages and conflicts; the existing valid content-addressed host conflict fixture remains intact. |
| Conflict message ownership | `Conflict.messages[].op_index` is defined as the flattened operation index: quarantined envelopes in order, or degraded edit forwards in `edit_ids` order. Generated conflict messages are normalized to this ownership; persisted values are range-validated. |
| Low-level conflict tags | History conflict kind/status tags are rejected by both encoder and decoder unless they are declared ordinals. |
| Strict text cursor/replay | `.ops` now requires explicit inverse, metadata, message, conflict, and cursor records. The parser has no metadata/id/actor/timestamp synthesis and rejects missing, unknown, duplicate, and overlapping cursor references. |
| Runtime-only policy contract | The frozen conflict shape no longer persists `MergePolicy`; rejection policy is explicitly runtime-local authority state. |

## Evidence

- Production runtime files changed: 3 Rust (`🏪️store`, SPR `📜️history`, `🖥️host`) and 1 frozen contract document.
- Focused adversarial test functions added: 5 (1 history tag test; 4 store tests for ledger convergence, full text fidelity, strict text rejection, and conflict op-index ownership).
- The existing host round-trip test now covers both valid binary and text preservation plus invalid content-addressed conflict IDs on both paths.
- `rustfmt --check --edition 2024` passed for all 3 changed Rust files.
- Cargo was intentionally not run; shared integration scheduling remains with the coordinator.

## Serialized Compile Remediation

The coordinator's serialized stdio library check reported exactly 5 store-owned compiler errors and no stdio errors. They are remediated without changing public signatures: one diagnostic-free envelope equality assertion, two explicit quarantined-envelope vector types, and owned cursor-id validation that does not immutably borrow the edit list across metadata restoration. `rustfmt --check --edition 2024` passes again for `🏪️store` only. No Cargo command was run in this lane.

A second serialized check isolated one final identical `mutation_envelope_from_edit` collection. It now has the same explicit `Vec<MutationEnvelope>` type. A source audit found no remaining untyped collection of that same ambiguous shape; the other untyped `envelopes` binding decodes an already-typed payload and is not ambiguous.

## Excluded P0

The repository-wide fallible `MutationDiff::apply` migration remains open and untouched, exactly as required by the audit scope.

## V3 Remediation

All 6 findings from `📋️w1-runtime-outcome-frozen-audit-v3.md` are source-remediated in the runtime lane. The repository-wide `MutationDiff::apply` P0 remains excluded.

| V3 finding | Remediation |
|---|---|
| P1-1 multi-forward ledger convergence | Snapshot preflight resolves every source operation to exactly one established local owner. It partitions authored messages by source `op_index`, rewrites each local one-operation owner to index zero, preserves source order, and rejects duplicate full or per-operation ownership. |
| P1-2 complete host cursor | `BackboneDocument` now owns a required `ArtifactCursor`; binary payload, text export/import, and `OsWorkflowStore` preserve applied, redo, and checkpoint values without a synthesized lane. |
| P1-3 repeated conflict actors | Conflict actors are a lexically sorted, duplicate-free participant set in every generation path. Persisted conflict validation requires that exact canonical form and checks it against the kind's envelopes or edits. |
| P1-4 runtime-local policy plan | The master plan now uses `Quarantined { envelopes }` and states that the deciding merge policy is runtime-local, report/error-only, and never a persisted history field. |
| P1-5 strict text sequence fidelity | `.ops` `edit` records require and round-trip `sequence`; parsing uses that supplied value, and the authoritative validator rejects negative or duplicate sequences rather than deriving position. |
| P2-1 shared codec validation | Both `.ops` replay and `.spr` decode construct their envelope then invoke `ArtifactStore::validate_authoritative_history`; duplicated cursor-only prevalidation was removed. SPR also requires an explicit durable cursor. |

### V3 adversarial coverage

- Multi-forward Operations→Snapshot ledger partitioning and an ambiguous established owner rejection.
- Host binary, text, and workflow-store preservation of redo plus checkpoint cursor state.
- Repeated-actor quarantined conflict generation plus malformed repeated actor validation rejection.
- Non-contiguous text sequence round trip plus negative sequence rejection.
- Duplicate authoritative change rejection through both `.ops` and `.spr` codec boundaries.

### V3 static evidence

- `rustfmt` and `rustfmt --check` completed for `🏪️store/🦀️component.rs` and `🖥️host/🦀️component.rs`.
- `git diff --check` completed for those runtime files and the active ticket folder.
- Source audit confirms two codec-boundary calls to the shared authoritative validator and no `sequence_number: edits.len()` text-position synthesis.
- Cargo and Nx were not run; the parent coordinator owns serialized compilation.

### V3 Serialized Compile Repair

The first serialized no-run check found five store-bound errors caused by routing codec parsing through the serde-bounded `ArtifactStore` implementation. The complete structural validator is now the free generic `validate_durable_history`, shared by store construction, mutation paths, `.ops` replay, and `.spr` decoding. Codec parse helpers retain only their actual `Clone + ArtifactDsl` or `Clone + ArtifactPack` requirements; no public codec caller gained serde bounds. `rustfmt --check` and scoped diff validation pass after the repair; Cargo remains coordinator-owned.

### V3 Runtime Verification And Host-Full Closure

The coordinator granted the serialized Cargo lock. All six requested focused store cases passed individually: multi-forward ledger partitioning, ambiguous ledger-owner rejection, full text outcome/conflict/cursor round trip, strict text cursor rejection, non-contiguous sequence fidelity, and repeated conflict actor canonicalization. The first full kernel run exposed a deterministic SpaceHost HLT tie: an explicitly authored mutation timestamp was merged into the local clock and then replaced by wall-clock time. `replay_mutations` now preserves the authored durable timestamp and advances the local clock only for its next generated timestamp. The serial full-library gate then passed **952 passed, 0 failed**.

| Gate | Result | Ticket log |
|---|---:|---|
| `cargo test -p semio-framework-os-kernel --lib -- --test-threads=1` | 952 passed, 0 failed | `🧪️runtime-v3-kernel-lib-final.log` |
| `cargo check -p semio-framework-os-kernel --features sync` | passed | `🧪️runtime-v3-sync-check.log` |
| sync wire fixture exactness | 1 passed, 0 failed | `🧪️runtime-v3-wire-fixture.log` |
| `cargo test -p semio-framework-os --features os-host-full --no-run` | passed | `🧪️runtime-v3-host-full-no-run-final2.log` |
| host backbone/workflow cursor, outcome, conflict round trip | 1 passed, 0 failed | `🧪️runtime-v3-host-backbone-workflow-final.log` |

The first real `os-host-full` compile exposed 59 errors. All were remediated in the direct host/runtime surface: app test and registration records now carry explicit role and dialect; `OsAppRegistration` preserves both through reconstruction; `Result<Vec<Dialect>>` registry reads propagate an unavailable registry rather than silently selecting a fallback; `WorkflowFixture` has complete handcrafted text and pack codecs; space example paths are local and exact; and direct pack encoding uses its `Vec<u8>` contract. The stale host test referring to a nonexistent workflow-document asset was removed rather than synthesized. The final host-full no-run gate is green. `rustfmt --check` for all four changed runtime files and `git diff --check` are green.
