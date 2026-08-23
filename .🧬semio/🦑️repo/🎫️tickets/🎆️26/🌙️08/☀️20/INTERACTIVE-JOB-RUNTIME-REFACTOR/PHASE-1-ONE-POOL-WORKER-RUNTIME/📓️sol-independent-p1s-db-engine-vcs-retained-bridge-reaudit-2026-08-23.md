# Sol Independent P1s DB Engine VCS Retained Bridge Re-Audit — 2026-08-23

## Audit Admission

The coordinator requested an independent Sol High re-audit of the P1s remediation. Terra admission
was scheduler-blocked. This audit has no P1s implementation authorship, changed no production
source, and did not use the prior implementation report as acceptance evidence.

Reviewed evidence:

- `📓️sol-independent-p1s-db-engine-vcs-retained-bridge-audit-2026-08-23.md`;
- the updated `📓️p1s-db-engine-vcs-retained-bridge-2026-08-23.md`;
- current and parent DB-engine source, the remediation diff, DB CLI/facade, authored Hub, and the
  root interactivity verifier/self-tests; and
- current working, staged, and `HEAD` diffs.

## Verdict

**REJECT — source packet.** The rejected String/Vec **byte** authority is repaired, the record author
is now moved rather than duplicated, fixed/derived Vec backing is credited before construction,
the 64 KiB operation and 4 MiB aggregate byte fixtures are meaningful, and the census is corrected
to 18 production CLI waits plus one test-only wait. However, checkpoint admission still counts each
source author only once in its item credit while conversion creates two simultaneously retained ID
String owners (`Author.name` receives the source String and `Author.id` receives its clone). The
required derived-ID **item** credit is absent. The verifier and fixture advertised for that boundary
also accept the omission.

P1s therefore remains source-RED. Phase 1 remains RED independently of this verdict.

## Remediation Evidence

### Record conversion: repaired

`record_credit` includes the document, optional parent, source author, message, and one fixed
`size_of::<HashMutation>()` backing claim before `VcsOperationAdmission::try_claim`. After the
retained store acquire, `ChangeRecord` is destructured and `author.0` moves directly into
`protocol::ActorId`; `change.author.0.clone()` is absent. `Vec::from([operation])` has one fixed
mutation element and no growable push path. The source author and a derived author String are no
longer simultaneously retained by this conversion.

The record exact-cap fixture sets:

```text
16 KiB base page + source/moved author capacity + size_of::<HashMutation>() = 64 KiB
```

and uses a borrowed preflight plus pointer/length witness for the byte-cap-plus-one rejection. The
64-owner aggregate fixture admits 64 × 64 KiB = 4 MiB, rejects the next owner, and leaves its source
pointer unchanged. These fixtures were inspected but not executed because builds were prohibited.

### Checkpoint byte and capacity backing: repaired

`checkpoint_credit` executes before admission, store acquisition, derived allocation, or clone. It
uses checked multiplication/addition for:

- the source `Vec<String>` change-ID backing;
- the source `Vec<ActorId>` backing;
- the derived `Vec<vcs::Author>` backing at the same source capacity;
- every source change-ID String capacity;
- every source author-ID String capacity; and
- a second author-ID String capacity for every derived `Author.id` clone.

Conversion allocates `Vec::with_capacity(source_authors.capacity())`, then moves the source String
into `Author.name` and clones it into `Author.id`. The fixed capacity is established before any
push, so this conversion has no Vec growth beyond the credited capacity. A String clone has no
greater logical capacity requirement than the source capacity charged for its derived twin. The
byte-cap/+1 and 64-operation aggregate fixtures preserve the rejected source pointer and the exact
4 MiB admission total. These source fixtures were not compiled or run.

## Blocking Item-Credit Gap

`checkpoint_credit` currently computes:

```text
1 operation + optional parent + change_ids.len() + authors.len()
```

It never adds the second `authors.len()` required for the derived `Author.id` String owners. At the
end of conversion, every source author identifier has become `Author.name` while a cloned
identifier is simultaneously owned by `Author.id`; both are live inside the output author Vec.

A concrete counterexample is a checkpoint with no parent/change IDs and 32 empty author IDs. The
current item formula is 33 and admits it. The converted command owns one operation plus 32 name
Strings plus 32 derived ID Strings: 65 items, exceeding the fixed 64-item operation authority. The
byte total remains small, so byte admission does not mask the item-cap breach.

The existing `item_rejected` fixture uses 64 source authors. The current incomplete formula already
computes 65 and rejects it, so the fixture cannot distinguish correct source-plus-derived item
credit from the defective source-only count. There is no 31/32 boundary fixture, and the verifier
does not require a second checked `authors.len()` term or mutate it away. Its derived-author
mutations cover only byte-term names/backing and therefore accept the live omission; both verifier
runs are green despite this source defect.

Required repair: count both source/moved name and derived ID String items with checked arithmetic
before admission/allocation, add a boundary fixture that the source-only formula would falsely
accept, and add a verifier mutation that removes the derived item term. Rejection must continue to
return the exact untouched input owner.

## Census and Reachability

| Surface | Independent result |
| --- | --- |
| Current production VCS region | Zero `block_on`, `submit_blocking`, `ask_blocking`, thread/pool/runtime construction, or polling loop. |
| Removed VCS bridges | Parent source shows the five former `ensure_store`, `record_change`, `checkpoint`, `merge_base`, and `head` `db_actor::block_on` bridges; all five are absent from current production VCS. |
| Remaining production DB engine | Exactly seven `db_actor::block_on` calls: WAL replay; three `open_with` capability/catalog/CAS calls; create-document catalog CAS; compaction; and hello. |
| DB CLI | Exactly 18 calls before the `#[cfg(test)]` module and one call in test-only `seed_document`; 19 whole-file. |
| DB facade | Zero production `block_on`, `submit_blocking`, or `ask_blocking`; its one call is under `#[cfg(test)]`. |
| Authored Hub | Zero `block_on`, `submit_blocking`, or `ask_blocking`. |

The corrected report census is accurate: five of the initial 12 engine bridges are removed and the
seven listed engine groups remain. The earlier 19-production CLI claim is no longer present.

## Fixtures and Mutations

Meaningful source fixtures cover operation-slot cap/+1, byte cap/+1, record/checkpoint exact byte
owners, 64 simultaneous maximum-byte owners, FIFO one-shot wake, cancellation, admission ABA, and
the live VCS forbidden-source scan. The remediation mutations meaningfully reject a record-author
clone, dynamic one-mutation Vec, missing record backing bytes, missing derived checkpoint ID bytes,
missing derived author Vec bytes, and an output Vec not preallocated to credited capacity.

The derived-ID item boundary is not meaningfully covered: the 64-author fixture rejects under both
correct and incorrect formulas, and no verifier mutation removes a second item term because no
such term is required by the verifier's accepted source model.

## Gates Run

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check --config skip_children=true` on DB engine | PASS |
| `bun ./📜️script.ts verify interactivity --self-test` | PASS; DENY clean, one existing test-only allowlist record |
| `bun ./📜️script.ts verify interactivity` | PASS; same DENY baseline |
| production VCS forbidden scan | PASS; zero matches |
| current engine production bridge scan | PASS; exactly seven |
| DB CLI scan | PASS; 18 production plus one test-only |
| DB facade/authored Hub production forbidden scans | PASS; zero |
| scoped and whole working/staged/`HEAD` diff checks | PASS |
| builds/runtime/timing | Not run |

No Cargo, Nx, compilation, native/Wasm/browser execution, network, root lint, or runtime timing was
run or inferred.

## Residual Status

P1s is RED pending exact checkpoint derived-ID item admission and a verifier/fixture that can detect
its removal. Phase 1 also remains RED for the seven engine bridge groups, P1q's indivisible
filesystem/SQLite syscall latency, compiler-generated future step duration, runtime compilation,
saturation/fairness timing, cancellation/interruption timing, and the full platform/thread matrix.
