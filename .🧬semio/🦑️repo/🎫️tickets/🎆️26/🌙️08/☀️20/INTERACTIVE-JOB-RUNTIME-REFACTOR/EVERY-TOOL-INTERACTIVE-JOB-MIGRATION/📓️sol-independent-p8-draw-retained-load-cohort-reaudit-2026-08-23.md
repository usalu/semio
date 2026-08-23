# Sol Independent P8 Draw Retained-Load Cohort Re-Audit — 2026-08-23

## Verdict

**REJECT — Draw retained-load source cohort.** The former live whole-snapshot/diff/serde/whole-operation-encode seams are gone from `DrawStoreInitializationAuthority`, and the replacement has useful retained ownership structure. The replacement is not yet an exact schema-first authority, however: its mutation digest omits semantic fields, and its candidate/container authorities allocate simultaneously retained derived owners without a mutation-aware aggregate item/byte preflight. The current fixtures and verifier mutations do not discriminate either defect.

This was an independent Sol High source re-audit. Terra admission was scheduler-blocked, so this report does not claim or imply a Terra verdict. I did not edit production source.

Phase 8 remains **RED at 0/884 admitted commands and 18 failure classes**. Cargo/native, Wasm, browser, runtime timing, hostile-payload timing, and the complete 884-command migration remain **RED/unverified**.

## Blocking Findings

### 1. The retained mutation digest is bounded but not schema-complete

`DrawMutationDigestAuthority` does not whole-materialize an encoded operation, but it also does not observe every semantic field that the replaced operation encoding distinguished (`owned/component.rs:1752-1927`):

- `ReplaceLayerFill` observes only gradient stops (`1835-1850`). It omits the `None`/`Some` and Solid/Linear/Radial discriminants, a Solid color, and Linear/Radial coordinates. `None`, any Solid fill, and an empty-stop gradient can therefore produce the same edit digest for the same target layer.
- `ReplaceLayerStroke` observes cap, join, and dash entries only (`1851-1869`). It omits the optional-value discriminant, color, and width. Distinct stroke mutations can therefore produce the same edit digest.
- `CreateLayer` delegates to `DrawLayerCloneAuthority` (`1871-1889`), but that authority installs layer variants and scalar geometry into a skeleton without observing a variant tag or those scalars (`847-885`). Fill/stroke scalar state is likewise copied while observing an empty slice (`954-979`). Shape/Path/Text/Image/Group/Boolean/Trace owners with equal observed strings and collection contents can collide despite different semantic values.

Those digests feed the applied/redo revision records, so this is not merely a diagnostic hash difference. The retained digest cannot truthfully replace the complete operation encoding until every variant discriminator, optional discriminator, scalar, string, and nested item is advanced through the digest exactly once.

The all-fourteen-variant fixture (`3190-3224`) exercises candidate application and retirement only. It neither drives `DrawMutationDigestAuthority` nor asserts distinct/equivalent digests against the existing operation codec. The permanent verifier requires the digest authority's name but has no mutation that removes a fill/stroke/created-layer field observation. Consequently all 182 source self-tests remain green while this semantic collision path is live.

### 2. Mutation-derived owners have no exact aggregate preflight

`DrawSnapshotBoundsAuthority` preflights the source snapshot before `DrawSnapshotCloneAuthority` copies it, but `DrawMutationCandidateAuthority` has no mutation-aware bounds phase. After the source clone completes, mutation application can allocate additional derived ownership without proving the resulting or simultaneously retained item/byte total:

- `CreateLayer` and `DuplicateLayer` construct `DrawLayerCloneAuthority` directly (`2157-2208`). Its skeleton immediately allocates nested `Vec::with_capacity` buffers for polygon points, path segments, Group children, and Boolean children (`873-879`) without first running a retained bounds cursor over the mutation layer or the duplicated subtree.
- Fill/stroke authorities cap an individual stop/dash vector at 4,096 (`1591-1605`, `1677-1686`), but do not combine that derived owner with the already retained source, candidate, mutation, and displaced style bytes/items.
- `DrawContainerRebuildAuthority::new` reserves both a reverse buffer and an output buffer (`1464-1490`). Its output calculation checks arithmetic only. A 4,096-item source plus one pending insertion requests a 4,097-item output, and no comparison with `DRAW_MAXIMUM_NESTED_ITEMS` or an aggregate process credit precedes either reservation.
- Scalar replacement strings are checked only against the 4,096-byte per-field limit. A candidate already at the aggregate snapshot byte cap can grow beyond it through rename, blend-mode, Boolean-operation, create, or duplicate mutation.

This violates the required fixed item/byte preflight before allocation/copy and leaves the advertised caps descriptive of the input snapshot, not exact for every simultaneously retained candidate/rebuild owner. It also means an allocation failure returns through local error/close behavior rather than an already-proven cap boundary with deterministic exact handback.

The named saturation fixture (`3248-3269`) admits a two-item container successfully and then tests zero-grant close; it does not saturate admission. There is no snapshot item cap/+1, aggregate byte cap/+1, 4,096/+1 structural insertion, or derived-owner credit fixture. The verifier checks only that the fixture name and selected preflight strings exist, so it does not reject removal of a mutation aggregate ledger or the current absent ledger.

### 3. History validation still contains multi-item work and the evidence matrix is incomplete

`ValidateEditId` now reaches every edit, including a single and final edit, which closes the former indexing hole. Within one grant, however, it scans the complete `mutation_meta` collection with `iter().any(...)` (`2749-2764`). This is not one retained field/item opportunity. Later, `CommitApplied` clones both edit ID and actor, while `ArtifactStoreInitializationRuntime::push_applied` clones the already cloned ID again, in one grant (`2927-2942` and shared store `10948-10954`). `CommitRedo` has the same derived-ID shape (`3009-3022` and shared store `10957-10963`). These simultaneously retained strings are field-bounded but are not item/byte credited as derived owners.

The populated live route is meaningful for one `RenameLayer`, success, generation publication, first/duplicate acknowledgement, partial-ingress cancellation, and single/final ID overflow (`editor/component.rs:667-797`). The owned fixtures meaningfully exercise all fourteen candidate variants, depth +1, hostile target-field rejection, false terminal, and interrupted close. They do not exercise digest parity/collisions, aggregate item/byte +1, actual admission saturation, stale-generation rejection during candidate/digest work, cancellation in those phases, or an allocation rejection that returns the exact recursive mutation owner. The corresponding permanent mutations append forbidden literal strings or remove required names; they do not distinguish these live semantic gaps.

## Accepted Source Evidence

The following remediation is source-valid and should be preserved:

| Requirement | Re-audit result | Evidence |
| --- | --- | --- |
| Initializer forbidden seams | PASS for the live owned initializer | Zero `snapshot.clone()`, `operation.diff`, `diff.apply`, `operation.encode_op`, or serde reconstruction/serialization in the owned initializer source. |
| Retained candidate route | PASS structurally, subject to the aggregate-admission blocker | `DrawMutationCandidateAuthority` covers all fourteen mutation variants; typed fill/stroke/layer cursors and fixed-depth locators advance incrementally. |
| Structural order and atomic publication | PASS structurally | `DrawContainerRebuildAuthority` moves FIFO layer owners through retained reverse/output buffers; the initializer replaces `runtime.current` only after candidate terminal completion and retains the displaced snapshot in `active` retirement (`2847-2908`). |
| Exact terminal shells | PASS structurally on inspected authorities | Candidate, rebuild, clone, digest, and initializer use retained owners, one-owner close paths, terminal-empty witnesses, and Drop assertions. |
| Edit-ID indexing | PASS | `ValidateEditId { edit }` visits every entry before pair validation, and the live fixture covers single/final over-cap IDs. |
| Whole-materialization removal | PASS narrowly | The live initializer no longer calls the old diff/apply/serde/whole-encode route. The new digest's bounded storage does not cure its missing semantic fields. |

## Exact Census

The repository source census remains exactly **14** `reject_whole_buffer_artifact_envelope_ingress` occurrences: **one** shared fail-closed definition and **13** live callers. The Draw subtree has **zero** occurrences. The remaining callers are outside this narrow cohort and remain RED.

The owned initializer file has zero direct occurrences of the five prohibited replay/materialization patterns. Other Draw product/config/diff/I/O paths still contain clone/serde/operation-codec behavior, but the inspected live initializer does not call those paths after this remediation; this verdict does not classify those unrelated routes as accepted.

## Executed Source Gates

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check --config skip_children=true` on Draw owned/editor/glue | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | PASS: **182** self-tests clean |
| Draw retained-route predicate through the full tool-job verifier | PASS mechanically: no Draw-named failure |
| `bun ./📜️script.ts verify interactivity --self-test --format json` | PASS: DENY clean; one recorded test-only blocking bridge and two predeclared future entries |
| `bun ./📜️script.ts verify interactivity --format json` | PASS: same DENY result |
| Full tool-job verifier | Expected global RED: exit 1, 50 macro hosts, 50 invocations, 775 rows, 773 unique rows, **0/884**, **18** failures, 182 self-tests |
| Deterministic Draw ledgers | PASS: current and repeat ledgers byte-identical, 312,305 bytes each, SHA-256 `21873b21e009a9b82d3a5a6497f8acbaf9e699187a77cd88890811392dcb7ba9` |
| Direct placeholder census | PASS for Draw: one definition + 13 live callers repository-wide; Draw zero |
| Scoped and whole working/staged/HEAD `git diff --check` | PASS at the pre-report boundary |
| Cargo, Nx, native, Wasm, browser, network, root lint, runtime timing | Not run by instruction; **RED/unverified** |

## Required Repair Before Another Re-Audit

1. Make `DrawMutationDigestAuthority` schema-complete for all fourteen variants and every nested layer/style field, with explicit variant/optional/length boundaries; add parity and collision-discrimination fixtures against the normative codec without whole-materializing in production.
2. Add a retained mutation/candidate preflight that checked-adds the source, candidate delta, temporary clone, rebuild reverse/output, pending, and displaced owners before any derived allocation. Reject exact item/byte/depth +1 with the original mutation/envelope owners still retrievable.
3. Split `ValidateEditId` metadata scanning and applied/redo history string derivation into one-field/item retained phases with exact derived-owner credits.
4. Add executable aggregate item/+1 and byte/+1 fixtures, a real saturation rejection, digest field mutations, stale/cancel at digest/candidate/rebuild phases, and exact owner handback/terminal-empty assertions. Extend the verifier with mutations that fail for the intended semantic reason rather than authority-name presence alone.

Until those paths close, the Draw retained-load source cohort remains **REJECTED**. Full Phase 8 remains **RED: 0/884, 18 failure classes, runtime unverified**.
