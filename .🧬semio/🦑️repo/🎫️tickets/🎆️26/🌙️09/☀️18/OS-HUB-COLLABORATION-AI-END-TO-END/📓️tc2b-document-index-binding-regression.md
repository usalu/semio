# TC2b — document-index binding regression repair

Slice TC2b (session 6, 2026-09-20). Urgent repair on top of TC2 §8/§8b.

## 0. Status

| field | value |
|---|---|
| suite before TC2's document-index change | `321 — 315 / 6` (21:03) |
| suite after | `321 — 297 / 24` (21:22, 21:37 — identical) |
| failure (A) | ×15 `publish dedicated creation genesis: Conflict("artifact checkpoint requires its descriptor-bound index")` at `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:957` (the 5 "singles" at `:470` are the same panic rethrown by the socket-thread `join()`, not a second fault) |
| failure (B) | ×6 `Conflict("document index descriptor binding differs")` at `🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs:242` (×5) and `:1090` (memory fold) |
| failure (C) | ×1 `artifact_authority::creation::tests::directory_document_index_is_ordered_idempotent_and_backend_descriptor_bound` at `🌱️creation/🧪️tests/🔬️unit/🦀️.rs:156`: `"two-space-dialect" left: 0 right: 1` — **client** rows, not backend |
| accounted for | 15 (A) + 6 (B) + 1 (C) = **22**; the remaining 2 of the 24 are the pre-existing HT8 inference laws (`💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:590` hash-domain, `:316` + `🗄️durable-group/🦀️.rs:1521` abandoned-approval drop). The `🗄️durable-group:1521` panic the coordinator listed is the **second** panic of that one inference test, not a store-slice red of its own. |
| `cargo check -p semio-hub --all-targets` | **Finished, 0 errors, 316 warnings** (`🗑️generated/tc2b-hub-check.txt`) |
| `cargo check -p semio-framework-os-kernel --all-targets` | **Finished, 0 errors, 18 warnings** (`🗑️generated/tc2b-kernel-check.txt`) |
| **needs hub rerun** | **YES** |

## 1. Inherited state

No `🗑️generated/tc2b-*` captures and no TC2b report existed; this slice had no predecessor. TC2's
edits were all uncommitted in the working tree and were read through `git diff HEAD`, not reverted.
No process was started or killed, the live 7611 hub and its data root were never touched, and no
`🗑️generated` file I did not create was modified.

## 2. Root of (A) — an EIGHTH equality site TC2's census missed

`🌎️hub/📇️directory/🦀️.rs` `validate_checkpoint_index_v1` (before this slice):

```rust
if index.is_some_and(|row| &row.descriptor == descriptor
    && row.descriptor_digest_v1 == checkpoint.descriptor_digest_v1
    && row.entry.dialect.artifact_kind == descriptor.artifact_kind) { … }
```

That last conjunct is **exactly** the `parent_dialect.artifact_kind == kind_id` cross-space equality
TC2 removed from seven sites, in an eighth place its census did not reach — the census grepped
`parent_dialect` / `parentDialect`, and this site spells it `row.entry.dialect`. It is called by all
four backends (memory `:1922`, sqlite `:1051`, postgres `:1017`, neo4j `:755`) on **every checkpoint
publication**, genesis included.

It was invisible until TC2 §8 product-shaped the hub's own synthetic descriptor: once
`document_descriptor_for_test` became a two-space document (`artifact_kind: "test.artifact"`,
`TEST_ARTIFACT_PARENT_DIALECT_KIND = "s.test.artifact"`, `🔬️bin-unit/🦀️.rs:788-800`), every genesis
publication in the bin suite hit `"test.artifact" != "s.test.artifact"` and was refused. **The 15
reds are the correct answer to a wrong law**: TC2's fixture change is what made the hub's own tests
stop being gis-shaped, and this equality is the gate that only a gis-shaped document could pass.

Product-first: a real hub creating a real gis map through `POST /spaces/{s}/artifact-creations`
survived only because `gis` collapses the two spaces. A `note` document (`2d.note` /
`s.note.note`) would have been refused at its own genesis checkpoint — after the descriptor was
announced and the index row written. The dialect equality adds nothing the law needs: the row is
already bound to the checkpoint by the **whole descriptor** and by the descriptor digest the
checkpoint itself announces.

**Fixed** by dropping the conjunct and recording the two spaces in a `🪢` docstring
(`🌎️hub/📇️directory/🦀️.rs`, `validate_checkpoint_index_v1`).

## 3. Root of (B) — the rule is right, the producers were gis-shaped

TC2 §8b's canonical-grammar predicate is legitimate. I re-measured the population it governs: a
scan of every JSON in the repo for an index-entry-shaped `{name, dialect:{artifactKind}}` returns
**three literals only** — `s.gis.map`, `s.gis.gismap` (product), `foreign.kind` (the deliberate
negative) — and the Space plugin's own artifact index carries `s.draw.draw`, `s.puzzle.5d`,
`s.energy.model`. Canonical `s.<plugin>[.<artifact>]` is what the product really produces.

What the six reds hit is the hub's **own genesis fixture family**, which was collapsing the two
spaces twice over:

1. `🌎️hub/📇️directory/🧫️fixtures/📸️artifact-checkpoint-projection/🔣️.json` `indexEntry.dialect.artifactKind`
   was `"s.gis:gismap"` — copied from the fixture's deliberately neutral descriptor kind. Neutral
   punctuation is the right test for a descriptor (opaque text, inside `descriptor_digest_v1`); it is
   the wrong value for a **dialect**, which is a coordinate.
2. `🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs:210` `publish_fixture_genesis` set
   `intent.parent_dialect.artifact_kind = descriptor.artifact_kind.clone()` — the gis collapse,
   hand-written, in the helper five of the six failing tests share.

**Fixed at the producers**: the fixture entry becomes `s.gis.gismap` (the entry is outside
`descriptor_digest_v1`'s domain, so **no golden recomputation** — the descriptor keeps
`s.gis:gismap` and its digest), and the helper takes a named
`FIXTURE_PARENT_DIALECT_KIND = "s.gis.gismap"` with a `🪢` docstring. Both fixtures now exercise a
**two-space** document, like `🔬️bin-unit` already does after TC2 §8.

## 4. Root of (C) — a NINTH and TENTH site, on the client side

`two-space-dialect` asserts `clientRows: 1`. It got 0, and no backend was involved:
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs:185` joined the indexed event to its
announced descriptor with

```rust
space.documents.iter().find(|d| d.document_id == scope.document_id && d.artifact_kind == entry.dialect.artifact_kind)
```

— the same equality again, as a **silent drop** rather than a refusal: on a real hub every non-gis
document would simply never appear in any client's space listing. Its TS twin (`🟦️.ts:205`) said the
same. A tenth copy sits in the hub script's independent Node oracle
(`📜️script.ts`, `proveSpaceArtifactCreationContractV1`), which re-derives `backendAccepted`.

## 5. The repair — ONE predicate, in the schema that owns the entry

Rather than keep TC2's `canonical_document_index_dialect_kind` in the hub and add a second copy to
the client fold and a third to the oracle, the grammar moved **into the type it describes**:
`DocumentIndexEntryV1::validate` (`📇️document-index-v1/🦀️.rs`), which both sides already call — the
client fold directly, and the hub's `document_index_projection_v1` through
`validate_directory_event_page_event`. The module already mirrors `SubsetId::ANY` for exactly this
reason (it must not depend on the io crate), so the grammar is mirrored beside it and nowhere else.

| # | file | change |
|---|---|---|
| 1 | `🧰️framework/…/📇️directory/🧬️schema/📇️document-index-v1/🦀️.rs` | `validate` bounds `dialect.artifact_kind` by new `canonical_dialect_artifact_kind` (`s.` + 1–2 lowercase-kebab segments, ≤256 bytes) instead of the loose `identity`; `🪢` + `🌿` docstrings record the two id spaces and why the plugin segment is compared to nothing (`demonstrator` ships 8 foreign-plugin dialects among the 127 shipped app dialects) |
| 2 | `…/🧬️schema/🟦️.ts` | `validDocumentIndexEntryV1` gains the same predicate, same docstring |
| 3 | `🧰️framework/…/📇️directory/🦀️.rs` | client fold joins on `document_id` alone; `🪢` docstring on `fold` |
| 4 | `…/📇️directory/🟦️.ts` | the TS twin, same change and docstring |
| 5 | `🌎️hub/📇️directory/🦀️.rs` | `canonical_document_index_dialect_kind` **deleted**; `document_index_projection_v1` keeps scope + digest and inherits the grammar through `validate_directory_event_page_event`; docstring rewritten to point at the one owner |
| 6 | `🌎️hub/📇️directory/🦀️.rs` | `validate_checkpoint_index_v1` drops the cross-space equality (§2) with a `🪢` docstring |
| 7 | `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | the independent Node oracle drops `descriptor.artifactKind === body.entry.dialect.artifactKind` |
| 8 | `🌎️hub/📇️directory/🧫️fixtures/📸️artifact-checkpoint-projection/🔣️.json` | `indexEntry.dialect.artifactKind` `s.gis:gismap` → `s.gis.gismap` |
| 9 | `🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs` | `FIXTURE_PARENT_DIALECT_KIND` replaces the descriptor-derived dialect in `publish_fixture_genesis` |

**Everything the slice was told to keep, is kept.**

- KD1's two-id-space admission: `two-space-dialect` (index), `ready-two-space-dialect` and
  `two-space-dialect` (creation catalog) all pass — see §6. Nothing compares `kind_id` to a dialect
  anywhere on the index or creation path any more.
- The `foreign-kind` refusal: `foreign.kind` is still not a dialect, so `validate()` refuses it. It is
  now refused **one gate earlier and on both sides** — the client drops the row (`clientRows: 0`) and
  the backend refuses the event page — where before only the hub projection refused it.
- TC2's honest hole is untouched and unwidened: the entry is still outside `descriptor_digest_v1`'s
  domain, and no wire field was added.

**Two pre-existing reds found on the way and repaired** (§6): the same independent Node oracle also
carried TC2 sites 1 and 2 (`value.ready.kindId === value.ready.parentDialect.artifactKind` and
`kind.kindId === kind.dialect.artifactKind`), whose Rust twins TC2 had already changed to per-field
identity bounds. They made `ready-two-space-dialect` throw before any of my work — TC2's two new
positive catalog laws had never been run. Both now mirror the Rust: `creationIdentity(…)` per field,
ordering check untouched.

## 6. Verification (measured, with the captures)

- `cargo check -p semio-hub --all-targets` — **Finished, 0 errors, 316 warnings**
  (`🗑️generated/tc2b-hub-check.txt`). Warnings present, so expansion really ran (68 s cold).
- `cargo check -p semio-framework-os-kernel --all-targets` — **Finished, 0 errors, 18 warnings**
  (`🗑️generated/tc2b-kernel-check.txt`). This is the crate that owns edits 1 and 3; `--all-targets`
  so its own test targets are type-checked, which the hub check would not have covered.
- `bun ./📜️script.ts space-artifact-creation-check source` — the two blocks in this lane **pass**
  (`🗑️generated/tc2b-creation-contract-source.txt`):
  ```
  [DEBUG] space artifact creation contract: cases=53 … ready-only-coordinate=1 ordinary-bridge=3
  [DEBUG] document index fixture: ordered-client=11 AJV=1 independent-node-SHA256=11
  ```
  `cases=53` includes TC2's `ready-two-space-dialect` and `two-space-dialect`; `ordered-client=11`
  and `independent-node-SHA256=11` are all eleven index rows, client fold and independent backend
  re-derivation, including `two-space-dialect` accepted and `foreign-kind` refused on both sides.
  Before this slice the first of those two blocks threw at `ready-two-space-dialect`.
- **The Rust laws are compile-verified only.** They live in `cargo test -p semio-hub`, which preamble
  rule 26 reserves for the coordinator. **That rerun is requested.**

## 7. Honest gaps

1. **No runtime observation.** No hub was booted, no document created. Every claim about the 22 reds
   is a read of the product code that raises each message plus `cargo check`; the proof is the
   coordinator's rerun.
2. **A third block of `space-artifact-creation-check source` still fails, and it is not this slice's
   and not TC2's**: `required checkpoint presence differs: present`
   (`📜️script.ts` checkpoint-presence loop). Measured, not guessed: the loop, its fixture rows and
   `validPlan` are byte-identical at `HEAD`, so a working-tree change causes it; `🐍️tc2b-open-plan-presence-diagnose.ts`
   shows `parseDocumentOpenPlanV1` and `parseDocumentExecutionTargetLeaseFieldsV1` both **accept**
   the neutral plan, so the refusal is in one of the four script-local predicates
   (`documentOpenNeutralStructure`, `executionTargetLeaseFieldsAdmissible`, or the two AJV schema
   exports). TC2's and my edits only ever **removed** a throw there, and a `present` row must be
   accepted, so the cause is a tightening from another slice — the in-flight `blake3` blob work in
   `🧬️schema/🟦️.ts` (`:478`, `:1535`, `:1570`) is the visible candidate. It is not in the
   coordinator's `cargo nextest -p semio-hub` suite. **Left for its owner; not touched.**
3. **The grammar is now enforced at event-page admission**, one gate earlier than TC2 put it. That is
   a tightening of a persisted event body: a directory log that already contains a `DocumentIndexed`
   event with a non-canonical dialect would now fail page validation. The repo-wide scan in §3 found
   no such producer, and the live 7611 hub only holds gis documents (`s.gis.gismap`), but this was
   not verified against that data root — rule 26/the hand-off forbid touching it.
4. **The TS twins are verified only by the hub script's oracle**, which exercises
   `validDocumentIndexEntryV1` and the client fold over all eleven index rows. No vitest project was
   run for `📇️directory/🟦️.ts`.
5. **TC2 §7.3's two surviving gis-only equalities are still there** (`💡️inference/📇️catalog/🦀️.rs:90`,
   `💡️inference/🧬️schema/🦀️.rs:216`) and are still out of scope: they gate the gis inference service
   only.

## 8. Files changed

Product code: `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📇️document-index-v1/🦀️.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`,
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🟦️.ts`,
`🌎️hub/📇️directory/🦀️.rs`.

Laws, fixtures and oracles: `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (three independent re-derivations),
`🌎️hub/📇️directory/🧫️fixtures/📸️artifact-checkpoint-projection/🔣️.json`,
`🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs`.

Ticket-owned: this report, `🐍️tc2b-open-plan-presence-diagnose.ts`, and
`🗑️generated/tc2b-hub-check.txt`, `tc2b-kernel-check.txt`, `tc2b-creation-contract-source.txt`.
