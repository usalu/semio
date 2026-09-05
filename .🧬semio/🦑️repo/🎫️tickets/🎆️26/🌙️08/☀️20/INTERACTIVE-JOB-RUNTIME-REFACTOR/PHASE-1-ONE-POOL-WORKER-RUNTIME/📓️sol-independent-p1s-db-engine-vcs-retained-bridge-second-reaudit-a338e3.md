# Sol Independent P1s DB Engine VCS Retained Bridge Second Re-Audit — 2026-08-23

## Audit Admission

The coordinator requested an independent Sol High second re-audit of only the P1s item-credit
remediation. Terra admission remained scheduler-limited. This audit has no P1s implementation
authorship, made no production edits, and treated the prior rejection and implementation report as
claims to verify rather than acceptance evidence.

Reviewed evidence:

- `📓️sol-independent-p1s-db-engine-vcs-retained-bridge-reaudit-2026-08-23.md`;
- the updated `📓️p1s-db-engine-vcs-retained-bridge-2026-08-23.md`;
- current DB engine, CLI, facade, authored Hub, root interactivity verifier/self-tests, parent DB
  engine source, and the current remediation diff; and
- working, staged, and `HEAD` whitespace diffs.

## Verdict

**ACCEPT — source-only P1s second remediation.** The rejected checkpoint item ledger now counts
the operation, optional parent, every change-ID owner, every moved author-name String, and every
simultaneously retained cloned author-ID String with checked arithmetic before admission or
conversion. The discriminating 31/32-author boundary and the verifier mutation detect the exact
previous omission. The previously accepted byte/backing ownership repairs remain intact.

This verdict does not accept Phase 1. Phase 1 remains RED for the seven residual engine bridges,
the P1q indivisible filesystem/SQLite latency boundary, compilation/runtime evidence, and platform
timing/cancellation evidence.

## Exact Item Arithmetic

`checkpoint_credit` binds `derived_author_items = request.authors.len()` and computes:

```text
1 operation
+ optional parent
+ change_ids.len()
+ authors.len() moved name Strings
+ derived_author_items cloned ID Strings
```

Every addition is `checked_add`; overflow fails before admission. `vcs_credit` rejects zero or more
than the fixed 64 items, and `VcsOperationAdmission::try_claim` independently enforces the same
per-operation ceiling.

For the fixture's no-parent/no-change requests:

- 31 authors produce `1 + 31 + 31 = 63`, and admission succeeds;
- 32 authors produce `1 + 32 + 32 = 65`, exceeding 64 and rejecting; and
- the obsolete source-only formula would have computed 33 and falsely admitted the second case.

The fixture records the source author `Vec` allocation pointer and first nested String pointer for
both cases. The admitted borrowed preflight/admission leaves both pointers unchanged. Rejection at
65 items likewise returns the exact source `Vec`, first String allocation, and length 32 unchanged.
No mutation, conversion, derived allocation, or store acquisition precedes this rejection.

## Byte and Backing Ownership Recheck

The second remediation did not weaken the previously accepted byte ledger. Before construction or
clone, `checkpoint_credit` still checks and charges:

- source change-ID `Vec<String>` capacity backing;
- source author `Vec<ActorId>` capacity backing;
- derived `Vec<vcs::Author>` capacity backing;
- every change-ID String capacity;
- every source/moved author-name String capacity; and
- every cloned author-ID String capacity.

Conversion uses `Vec::with_capacity(source_authors.capacity())`, moves each source String into
`Author.name`, and clones it once into `Author.id`. The push count is the source length, which cannot
exceed the already credited source capacity, so the derived Vec has no capacity-growth escape. A
cloned String's required bytes cannot exceed the credited source String capacity. Record conversion
still moves `author.0`, reserves `size_of::<HashMutation>()`, and uses fixed `Vec::from([operation])`.
The 64 KiB per-operation and 4 MiB aggregate byte authorities are unchanged.

## Verifier and Mutation Evidence

The production verifier now requires both checked author-item additions inside the isolated
`checkpoint_credit` source slice. Its `uncredited-checkpoint-derived-id-item` mutation removes only:

```text
.and_then(|value| value.checked_add(derived_author_items))
```

It does not alter any byte term, Vec backing term, conversion, or fixture name. The authorized
`--self-test` run passed, which means this intended mutation produced a verifier failure while the
unmodified retained authority was accepted. The verifier also requires the exact 31/32 fixture.
The Rust fixtures were inspected but were not compiled or executed because builds were prohibited.

## Reachability and Census

| Surface | Independent result |
| --- | --- |
| Current production VCS region | Zero `block_on`, `submit_blocking`, `ask_blocking`, thread/pool construction, or polling loop. |
| Removed VCS bridges | Parent source contains the five former `ensure_store`, `record_change`, `checkpoint`, `merge_base`, and `head` `db_actor::block_on` sites; current production VCS contains none. |
| Remaining production DB engine | Exactly seven `db_actor::block_on` sites: WAL replay; three `open_with` capability/catalog/CAS sites; create-document catalog CAS; compaction; and hello. |
| DB CLI | Exactly 18 production `db::actor::block_on` sites plus one test-only `seed_document` site; 19 whole-file. |
| DB facade | Zero production forbidden bridges; its one `block_on` is under `#[cfg(test)]`. |
| Authored Hub | Zero `block_on`, `submit_blocking`, or `ask_blocking` sites. |

The initial engine census remains 12, with five VCS sites removed and seven residual sites live.

## Gates Run

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check --config skip_children=true` on DB engine | PASS |
| `bun ./📜️script.ts verify interactivity --self-test` | PASS; DENY clean, one existing test-only allowlist record |
| `bun ./📜️script.ts verify interactivity` | PASS; same DENY baseline |
| production VCS forbidden scan | PASS; zero matches |
| current/parent engine bridge census | PASS; five VCS sites removed, exactly seven production engine sites remain |
| DB CLI census | PASS; 18 production plus one test-only |
| DB facade/authored Hub production scans | PASS; zero forbidden sites |
| scoped and whole working/staged/`HEAD` whitespace checks | PASS |
| independent report `/dev/null` whitespace check | PASS; no diagnostic (exit 1 denotes the expected added file) |
| Cargo/Nx/Wasm/browser/network/runtime/timing | Not run; prohibited for this audit |

## Residual Status

P1s second remediation is source-accepted. Phase 1 remains RED for the seven engine bridge groups,
P1q's indivisible filesystem/SQLite syscall latency, compiler-generated future step duration,
runtime compilation, saturation/fairness timing, cancellation/interruption timing, and the full
native/Wasm/browser/platform matrix.
