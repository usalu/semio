# W0 Barrier — GREEN

Run by the coordinator, serially, after lanes 0-A and 0-B reported. Raw output:
`🧪️w0-barrier-check.txt`, `🧪️w0-barrier-test.txt`.

## Gate results (actually run)

| Gate | Command | Result |
|---|---|---|
| Kernel + derive compile | `cargo check -p semio-framework-os-kernel -p semio-framework-os-kernel-dsl-derive` | **PASS** — 0 errors; 10 warnings in `semio-framework-os-kernel` (all pre-existing dead-code warnings, e.g. `PendingEdit.line_no`, `set_envelope`), derive crate clean. Finished in 2.85 s (warm). |
| Command spine tests | `cargo test -p semio-framework-os-kernel --lib -- os_spr::command` | **PASS — 37 passed; 0 failed; 0 ignored; 843 filtered out.** Includes `operation_descriptor_fingerprint_is_golden_pinned` (re-baked) and `derive_mutations_wires_mutation_and_semantic_mutation` (0-B's derive output). |
| Full kernel lib suite (lane 0-A, informational) | `cargo test -p semio-framework-os-kernel --lib` | 879/880 — 1 failure in `CompositionCoordinator` territory, which is **lane 1-E's lease**. Handed to 1-E. |

## Structural verification (coordinator, independent of lane claims)

| Check | Result |
|---|---|
| Derive mirror byte-identity | `diff 🗣️dsl/✨️derive/🦀️component.rs 🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs` ⇒ **empty (IDENTICAL)** |
| `📡️spr/🔀️crdt/` deleted | **yes** — directory does not exist |
| `📡️spr/⚔️conflict/🦀️component.rs` exists | **yes** |
| `fn validate` in `📡️spr/🎮️command` | **0 occurrences** |
| Channel tags still free as frozen | AppCommand next free **30**, AppFrame next free **23**, `CHANNEL_VERSION` still **10**, `ArtifactCommand` next free **15** — all confirmed by scout 1 |

## Landed API (authoritative for every downstream lane — code against THIS)

`📡️spr/🎮️command/🦀️component.rs`, region `🔖️Message` (lines 98–266):

- `MutationMessage::{info,warn,error,fatal}(code, message)` — associated ctors, then `.at(target)` / `.at_op(i)`.
- `MutationOutcome<D>`: associated `new(diff)`, `empty()`, `error(code, msg, target)`, `fatal(code, msg, target)`.
- Chainable builders on a value: `.info(code, msg)`, `.warn(code, msg)`, `.absorb_messages(..)`, `.stamp_op_index(i)`, `.map(f)`.
- Readers: `.diff()`, `.messages()`, `.into_parts()`, `.worst_level()`, `.is_applicable(policy)`.
- Free fn `worst_level(&[MutationMessage]) -> Option<Severity>`.

**Accepted contract clarification (coordinator ruling).** C2 listed both an associated `::error` and a
chainable `.error(..)`; Rust forbids an associated and an instance method sharing a name on the same
type. Landed shape: `MutationOutcome::error(..)` is the associated ctor (empty diff, per the C2 law);
to attach an Error/Fatal message to an outcome that already has a diff, use
`.absorb_messages([MutationMessage::error(code, msg).at(target)])`. `.info`/`.warn` remain chainable.
This is the only deviation from the frozen text and it is now part of the freeze.

`📡️spr/⚔️conflict/🦀️component.rs`: `ConflictId::new(kind, artifact_id, mutation_ids, hlc)`,
`ConflictKind`, `ConflictStatus`, `ConflictResolution`, `Conflict`, `EditMessages`,
`DispatchReport`, `MergeReport` — the full C5 set, as frozen.

## Debt carried out of W0 (allocated, not forgotten)

1. **`reconcile_with_last` / `SpaceConflict` were stubbed, not deleted** by 0-A (Rust compiles the
   crate as one unit, so it kept them compiling rather than reaching into 1-A/1-E's regions).
   C6/C10 require deletion → assigned to **lane 1-A** (`SpaceConflict` and friends,
   `reconcile_with_last`, `materialize_document_snapshot_with_conflicts`, `snapshot_with_conflicts`).
2. **A second, independent `MergeStrategyKind`** lives in `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs:668`
   with `ArtifactMergeKind::merge_strategy()` and a re-export at
   `🧰️framework/📦️packages/🦀️rust/📦️glue.rs:103`. It is unrelated to the deleted `protocol_crdt` one
   and has **no remaining callers**. Dead CRDT vocabulary → assigned to **lane 2-E**.
3. **`🛢️db` is red as expected** — `📄️artifact`, `⚔️conflict`, `⌨️cli`, `👁️preview` still reference
   `protocol::MergeStrategyKind` / `protocol::ConflictRule` / `ResolutionPlan`, which 0-A deleted.
   That is precisely lane **2-E**'s C9 work.
4. **CRDT vocabulary residue in comments/docstrings** inside `📡️spr/🧾️wire`, `📡️spr/🎮️command`,
   `🌿️vcs`, `🏪️store`, `📡️spr/🧪️testkit/benches/protocol.rs` — these are prose noting the removal,
   but `policyNoCrdtVocabularyBreaches` counts raw occurrences, so they must be reworded.
   → **W4 coordinator sweep**, tracked here.
5. **`Severity::Hint` survives in 5 places**: `🔌️plugin/🦀️component.rs:18760` (→ lane **2-A**) and
   4 sites in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/…/🧬️schema/🦀️component.rs` (→ lane **3-E**).

## Scout results feeding W1/W2/W3

- `📓️scout-1-channel.md` — tags/regions confirmed; TS `AppChannelCodec` 1616–2319, `AppChannelClient`
  2321–2584; **no Rust↔TS drift**; `Invocation`/`Error` are safe to extend with trailing fields;
  `ApplyOutcome::Rejected { reason: String }` at `📡️wire:76`; version pin in
  `🧫️fixtures/📡️channel/channel-version.json`, asserted by a TS test.
- `📓️scout-2-stdio.md` — `semio-s-plugin-stdio` **currently compiles** (0 errors). FULL-STDIO is
  editing only the glTF subtree (~28 of 314 leaves). Collision risk for 3-E: **contained today**.
- `📓️scout-3-shell.md` — settings surface is `ChromePanels` (Select-row pattern, lines 372–424);
  frame decode seam at `💻️os/🟦️component.ts` 2115–2244; config triad reference is `set-default-app`;
  i18n bundles at `⚛️react/📦️index.tsx` 2230–3482.
- `📓️scout-4-fanout-census.md` — **1543 `🔺️diff` leaves**, 718 hand-written `impl Mutation`/
  `MutationKind` blocks, **62 `fn validate` overrides** (stdio 55, energy 3, procedural 3, flow 1).
  Per-lane actual counts run ~5–25 % above the plan's estimates; 3-E is +56. Lane briefs carry the
  corrected numbers.

**Verdict: W0 barrier is GREEN. W1 and W3 are released.**
