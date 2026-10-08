# 🔍️ Framework Mutation Spine Audit

Read-only audit of the framework mutation spine against the laws L1-L5 and violation codes in `📋️design.md`
(re-read after the on-disk update that added the Rulings and `V5-ABSORB`). No source file was edited, no build ran, no git
write command was used. Scope: `🧰️framework` plus `✏️s` for counts. Plugins are only counted, not reviewed.

Method: `rg --no-ignore-vcs` with `-g '*.rs' -g '!**/target/**'`, targeted `sed`/`awk` reads, and one Python pass that
extracted `diff`/`inverse` bodies of every hand-written `impl ... Mutation<..> for ..`. The per-impl table is in
`🗑️generated/hand-impl-classification.txt` (generated output, deleted at ticket close). Heuristic regex counts are marked
as such; the sampled bodies were checked by eye.

## Summary

1. The spine has no capability gate. `MutationDiff::apply(&self, base: &P)` (`replication/🎮️mutation/🦀️.rs:104`) is public and
   callable from any leaf. `ApplyCapability` does not exist in any Rust file (0 hits).
2. The only `&mut P` in the spine is the inherent `MutationOutcome::apply_to(self, snapshot: &mut P)` (`:1316`). No trait
   method takes `&mut P`.
3. `#[derive(Mutations)]` generates no apply and no generic diff/inverse. It delegates each arm to the leaf's
   `MutationKind::diff`/`inverse`. The only generic fallbacks are `#[derive(CompositeMutation)]` arms that call
   `fold_plan_diff`/`fold_plan_inverse`.
4. 223 `impl MutationDiff<..>` lines (framework 50, artifacts 173). 57 of them have `Diff == Snapshot` (whole-state diff, the
   V1-GENERIC-DIFF pattern the new Minimality ruling bans).
5. Of 111 hand-written `impl Mutation<..> for ..` lines (100 files), 91 are production. 58 production impls have a
   whole-state diff, 10 clone base then mutate (V1-SNAPSHOT-DIFF), and 55 production inverses restore a whole snapshot
   (V2-RESTORE-INVERSE). 2 inverses are `Noop` for a state-changing diff (V2-EMPTY-INVERSE class).
6. Production leaf-level `.apply(` on a diff exists at 4 config leaves, 1 print leaf, 1 print `DiffAlgebra::inverse`, and
   about 20 store, plugin, tool and db fold sites that call `apply` directly (section 4, S-1..P-4). Each is a V3-LEAF-APPLY or a bypass of a central applier.
7. Central applier candidate: `store::fold_operation` (`store/🦀️.rs:25198`). It is already the keep-and-record step
   behind six `fold_operation` call sites in `store`. Proposal in section 4.
8. L3 helpers exist (`assert_mutation_inverse_law`, `assert_mutation_diff_absorb_law`, `assert_diff_algebra_*_law`), but none
   checks the inverse-sum identity `Σ == m.diff(base).diff().inverse(base)`, the per-step sparseness, or absorb coalescing.
9. The policy gate `verify mutation-outcome-law` has 7 rules, none checks diff shape, leaf `.apply(`, `&mut`, `between(`,
   or whole-state diffs. `policyMutationTriadCompletenessBreaches` is named in a doc comment but does not exist.

---

## 1. Core traits: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`

| Item | Line | Shape |
|---|---|---|
| `MutationApplyError` | 20 | `code`, `message`, `target`; structured, fallible |
| `trait MutationDiff<P>` | 103 | `apply(&self, base: &P) -> MutationApplyResult<P>` (104); `absorb(&mut self, other)` (119); `retire_cold(self)` (128); `retire_projection(P)` (141) |
| `trait DiffAlgebra<P>` | 155 | `inverse(&self, base: &P) -> Self` (157); `between(base, other) -> Self` (159); `is_empty` (161). Separate trait, not a `MutationDiff` method |
| `trait Mutation<P>` | 174 | `type Diff: MutationDiff<P>` (175); `DESCRIPTORS` (177); `descriptor`, `diff(&self, base: &P) -> MutationOutcome<Self::Diff>` (181); `inverse(&self, base: &P) -> Result<Vec<Self>, ValueError>` (182); `retire_cold` (186); metadata defaults 192-269; `foreign_steps(&self, base: &P)` (237) |
| `enum MutationInvertibility` | 283 | `SelfInvertible / ExplicitMutation / Plan / NonInvertible`. Metadata only |
| `enum MutationDiffParticipation` | 310 | `Detect / ApplyOnly / Plan / None`. Metadata only |
| `enum MutationOutcomeClass` | 338 | `Applied / NoOp / Empty / Disjoint / Rejected`. Metadata only |
| `MutationComposition`, `MutationLanguageSurface` | 369, 384 | Metadata only |
| `struct MutationLeafDescriptor` | 414 | 14-field static roster row. Metadata only |
| `trait MutationLeaf` | 971 | `DESCRIPTOR`, `PROVENANCE`, `PAYLOAD_SCHEMA`, `input_value`/`with_input_value`, `inverse_rows` (1012). No diff or apply |
| `struct MutationOutcome<D>` | 1238 | `{ diff: D, messages }`. `new` (1298), `diff()` (1302), `apply_to(self, &mut P)` (1316), `map` (1376) |
| Composite mutations | n/a here | Defined in `spr` (section 2), not in this file |

Can a mutation mutate state?

- Traits: `Mutation::diff/inverse` take `&P`. `MutationDiff::apply` takes `&P` and returns a new `P`. `DiffAlgebra` is pure.
  No trait signature has `&mut P`.
- Inherent: `MutationOutcome::apply_to(&mut P)` (`:1316`) mutates in place. The design marks it for removal.
- `MutationDiff::absorb(&mut self)` mutates the diff, not the snapshot.
- `retire_projection(P)` (`:141`) takes ownership and drops.

## 2. `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` and collection helpers

Traits:

| Item | Line | Notes |
|---|---|---|
| `trait MutationKind<P, Op>` | 219 | `diff(&self, base: &P) -> MutationOutcome<Op::Diff>` (225); `inverse(&self, base: &P) -> Result<Vec<Op>, _>` (229) |
| `trait SemanticMutation<P>` | 261 | Registration/introspection facade over `#[derive(Mutations)]` enums |
| `trait CompositeMutationKind<P, Op>` | 799 | `plan(&self, base, &mut Planner)` (800). Only `call`/`call_foreign` may advance the plan |
| `struct Planner<P, Op>` | 688 | `call` computes `op.diff(&self.base)` and advances `self.base = diff.apply(&self.base)?` (741). `&mut self` |
| `fold_plan_diff` | 833 | Folds local steps with `absorb` (`folded.absorb`) and `diff.apply(&current)` (853). Generic |
| `fold_plan_inverse` | 871 | Collects each local step's `op.inverse(&pre_state)`. Generic |
| `plan_foreign_steps` | 909 | Generic |

Diff-shape helpers in `spr`:

| Item | Line | Generic? | Mutates? |
|---|---|---|---|
| `NamedTripleDiff<K,V,Patch>` | 289 | Generic delta shape (removed / modified / added) | n/a |
| `named_apply(&mut Vec<V>, &diff)` | 310 | Validates, then applies atomically | `&mut Vec` |
| `IndexedTripleDiff<V,Patch>` | 362 | Generic index-keyed delta | n/a |
| `indexed_apply(&mut Vec<V>, &diff)` | 383 | Validates, then applies atomically | `&mut Vec` |

Collection helpers (re-exported at `spr` line 272, defined in `🌿️vcs/🦀️.rs`):

| Helper | `os_vcs` line | Generic? | Production use |
|---|---|---|---|
| `ItemPatch`, `CollectionDiff`, `Identified`, `Patchable` | 1758, 1766, 1781, 1790 | Generic | used by named/collection code |
| `enum CollectionMutation` | 1810 | Generic `Add/Remove/Move/Patch` | Design forbids exposing it as a verb |
| `apply_collection_mutation` | 1818 | Generic apply. `Add` clamps with `min(items.len())` (1825), against the "never clamp" rule | 0 non-test calls in `✏️s`; framework re-exports and tests only |
| `inverse_collection_mutation` | 1848 | Generic inverse from pre-state. `expect(..)` panics on a missing target | 0 non-test calls in `✏️s`; re-exports and tests only |
| `collection_diff_from_mutation` | 1878 | Generic projection. `Move` is encoded as remove+add | 0 non-test calls in `✏️s` |
| `apply_mutation` (replay step) | 1911 | Generic `operation.diff(snapshot)` then `diff.apply` (1916) | central-ish, see section 4 |

Answer to "generic mechanisms that leaves use instead of concrete ones": the three collection helpers are generic and
are not used by production leaves. The generic mechanisms that production does use are `fold_plan_diff`/`fold_plan_inverse`
(via `CompositeMutation`), `os_vcs::apply_mutation` (`store/🦀️.rs:27453`, retained config `:471`, `:503`), and
`named_apply` (a private copy in `🖋️dxf/…/🔺️diff/🦀️.rs:219`, plus `🔊️audio`).

Duplicate type: `🪐️space/🗿️artifacts/🗂️collection/🦀️.rs:291` `enum CollectionMutation` and `:389` `struct
CollectionDiff`, a second non-generic copy with the same name.

## 3. `#[derive(Mutations)]` and `#[derive(MutationLeaf)]`

Entry points: `🗣️dsl/✨️derive/📦️packages/🦀️rust/🦀️.rs` (`Mutations` at 43-45, `CompositeMutation` at 59). Owner
implementation: `🗣️dsl/✨️derive/🦀️.rs`.

- `expand_derive_mutations` (`:924`) generates `impl Mutation<S> for Agg` with `type Diff = #diff_ty` from the container
  attribute (`:1112`).
- `diff` (`:1126-1128`): a `match self` whose arms call `<Payload as MutationKind<S,Agg>>::diff(payload, base)` (arm built at
  `:1051`).
- `inverse` (`:1130-1132`): same, via `MutationKind::inverse` (arm at `:1052`).
- `retire_cold` is emitted only if the container attribute names a `retire_cold` function (`:956-958`).
- No `MutationDiff` impl, no `apply`, and no generic fallback is generated. Unknown kinds are refused in
  `from_payload_value` (`:1164`).
- `#[derive(MutationLeaf)]` (`expand_mutation_leaf`, `:572`) generates the descriptor constants and `inverse_rows`
  (`:634`). It generates no diff, inverse or apply.
- `#[derive(CompositeMutation)]` (`expand_derive_composite_mutation`, `:1306`) generates `MutationKind::diff =
  fold_plan_diff(self, base)` (`:1331-1332`) and `inverse = fold_plan_inverse(self, base)` (`:1334-1335`). This is the
  only generic fallback the derives emit.

## 4. Call sites, central applier, and counts

Counts (`rg`, line-level, regex on the line):

| Metric | Command shape | Result |
|---|---|---|
| `impl ... MutationDiff<` lines | `rg -n "^\s*impl\b.*\bMutationDiff<" 🧰️framework ✏️s -g '*.rs'` | 223 (framework 50, `✏️s` 173). 211 files. 3 framework lines are macro templates |
| of which `Diff == Snapshot` | same lines, python compare of type arg vs `for` type | 57 (whole-state diff) |
| diff-apply calls, non-test | `rg "diff\(\)\.apply\(\|\.diff\([^)]*\)\.apply\(\|diff\.apply\(\|outcome\.diff\(\)\.apply"`, minus `🧪️tests\|/tests/` | framework 27, `✏️s` 43 |
| `apply_to(` calls, non-test | `rg "apply_to\("` | framework 11, `✏️s` 74 |
| `os_vcs::apply_mutation(` calls | `rg "\bapply_mutation\("` | non-test framework: store 27453, retained config 471 and 503; the rest are test support |
| `.apply(` lines in `🧬️mutations/` dirs, non-test | `rg "\.apply\("` filtered to `🧬️mutations/` | 30 lines / 29 files (framework 5, drawing 2, lowpoly 1 non-diff `motion.apply`, raster 1 doc comment, 21 stdio, not classified individually) |
| `.apply(` lines in `🔺️diff/` dirs, non-test | same, `🔺️diff/` | 126 lines / 37 files (stdio composite sub-diff apply, `SemioDiff::Brep(d) => d.apply(b)`, legitimate composition, but it is `apply` outside a capability) |

Production leaf-level and bypass call sites (non-test):

| # | Site | Kind |
|---|---|---|
| L-a | `🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/🦀️.rs:42` (`ChangeChartValue::diff`) | V3-LEAF-APPLY: `diff.apply(base)` to validate |
| L-b | `📓️print/🧬️schema/🔀️diff/🦀️.rs:122` (`ChartDiff::inverse`) | V3-LEAF-APPLY + V2: `self.apply(base)` then `between(&next, base)`. Snapshot-based inverse |
| L-c | `🎚️config/🧬️schema/🧬️mutations/{🪪️sign-in:118, 🛡️change-merge-policy:71, 📥️admit-local-document:125, 📎️attach-local-folder:123}/🦀️.rs` | V3-LEAF-APPLY and `&mut` snapshot: `*snapshot = mutation.diff(snapshot).diff().apply(snapshot)?` |
| L-d | `🎚️config/🧬️schema/🦀️.rs:163` and `:210` (`apply_opening_config_mutation`, `apply_ui_preferences_config_mutation`) | Bridge that takes `&mut` and applies. A second applier outside the central one |
| L-e | `🖍️drawing/🧬️schema/🧬️mutations/🦀️.rs:143`, `:174` | V3-LEAF-APPLY (leaf-side fold) |
| L-f | `💠️lowpoly/🧬️schema/🧬️mutations/🦀️.rs:109` (`lowpoly_selection_motion_diff`) | V1-SNAPSHOT-DIFF and V3: decodes base mesh, mutates it with `motion.apply(&mut mesh, ..)`, re-encodes the whole mesh into the patch |
| S-1 | `store/🦀️.rs:5961` presence `apply`, `:6128` presence single-item, `:6279` transient `apply`, `:6476` transient single-item, `:6650` hover `apply` | Store lane appliers that each call `mutation.diff(..).diff().apply(..)` directly, not through `fold_operation` |
| S-2 | `store/🦀️.rs:23429` (`replay_mutations`) | Inline `diff.apply(&*snapshot)` in replay |
| S-3 | `store/🦀️.rs:25203` (`fold_operation`, fn at 25198) | The existing keep-and-record fold step |
| S-4 | `store/🦀️.rs:25819` (`ReplayMode::Merge` arm) | Inline copy of `fold_operation`'s logic |
| S-5 | `store/🦀️.rs:27453` (`resolve_backbone`) | `os_vcs::apply_mutation` |
| P-1 | `🔌️plugin/🦀️.rs:29798`, `:33495`, `:36432` | Transaction member folds: `outcome.diff().apply(&running)` |
| P-2 | `🔌️plugin/🦀️.rs:41903` (`absorb(self, diff)` wrapper) | `diff.apply(&self.snapshot)` |
| P-3 | `🛠️tool-machine/🦀️.rs:60` (`fold_leaf`), `⏯️tool-run/🦀️.rs:540` (`fold_one`) | Fold seams that apply diffs |
| P-4 | `🛢️db/🗿️artifact/🦀️.rs:234` | `diff.diff().apply(base)`, and the envelope stores whole `post` and whole `base` (snapshot-based) |
| SP-1 | `📡️spr/🎮️command/🦀️.rs:741` (`Planner::call`), `:853` (`fold_plan_diff`) | Generic composite folds |

Proposed central applier (`ApplyCapability`, single mint point):

- Mint point: `store::fold_operation` (`store/🦀️.rs:25198`). It already gives one keep-and-record step with
  `retire_cold`. Route S-2, S-4, S-5 (via `os_vcs::apply_mutation`), P-1..P-4 and SP-1 through it, or through an
  `os_vcs::apply_mutation` that calls it. Presence/transient/hover lanes (S-1) also route through the same function.
- Signature to add in `protocol` (`replication/🎮️mutation/🦀️.rs`): `MutationDiff::apply(&self, base: &P, cap:
  &ApplyCapability)`, plus `pub fn apply_diff<P, D: MutationDiff<P>>(base: &P, diff: &D, cap: &ApplyCapability) ->
  MutationApplyResult<P>` as the one callable entry.
- Limit to state in the design: `ApplyCapability` is a token with a private field. Rust `pub(crate)` only works inside one
  crate. `protocol` is crate `semio-framework-replication` (`replication/📦️packages/🦀️rust/Cargo.toml`), while the folds are
  in `semio-framework-os-kernel` (`os/📦️packages/🦀️rust/Cargo.toml`), and leaves depend on both. A token that os-kernel can
  mint is also mintable by a leaf crate, so L5 cannot be enforced by visibility alone. Options: (a) keep the mint `pub` but
  `#[doc(hidden)]` and enforce its callers by policy gate; (b) move the mint into a crate that leaves cannot depend on. This
  is my analysis, not compiled or checked.

## 5. Existing law helpers and what is missing for L3

Existing (`📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs`):

| Helper | Line | Checks |
|---|---|---|
| `assert_mutation_diff_absorb_law` (+ `_cold`) | 517, 535 | `absorb(d1,d2).apply(base) == d2.apply(d1.apply(base))` |
| `assert_mutation_inverse_law` (+ `_cold`) | 572, 605 | Applies `inverse(base)` reversed, one op at a time, and checks the final state `== base` |
| `assert_diff_algebra_between_law` | 637 | `between(a,b).apply(a) == b`; `between(a,a).is_empty()` |
| `assert_diff_algebra_inverse_law` | 649 | `d.inverse(base).apply(d.apply(base)) == base` |
| `assert_operation_round_trip`, `assert_store_roundtrip` | `store/🦀️.rs:29658`, `:29675` (in `pub mod test_support` at `:29539`) | Store round-trip using `apply_mutation` |
| `inverse_restores_inactive_and_active_alternatives` | `store/🧬️schema/🧬️mutations/🧹️remove-space-alternative/🧪️tests/🔬️unit/🦀️.rs:9` | Example per-leaf inverse test |
| `policyDiffAlgebraBreaches` | `📜️script.ts:23238` | Checks only that a stdio diff has an `impl DiffAlgebra<..>` block. Allowlist `POLICY_DIFF_ALGEBRA_ALLOWLIST` is an empty set (`:23236`) |

Usage: any L3 helper (inverse, absorb, DiffAlgebra) appears in 41 `✏️s` files; the absorb law alone in 37 `✏️s` files;
the combined pattern on 68 framework lines. These are file counts, not per-leaf coverage. There is no gate that checks every leaf has one (V4 untested is
not enforced).

Missing for L3 (`Σ = d₁ ⊕ … ⊕ dₙ`, `Σ.apply(after) == base`, `Σ == m.diff(base).diff().inverse(base)`):

1. No helper folds the inverse steps with `absorb` and compares Σ to the `DiffAlgebra::inverse` of the forward diff.
   The inverse law checks only the sequential result.
2. No helper checks that each `d_k = inv_k.diff(s_{k-1})` is sparse (L1), so whole-state diffs pass.
3. No helper checks absorb coalescing (`V5-ABSORB`: patch∘patch, create∘delete, delete∘create).
4. `assert_diff_algebra_inverse_law` covers one diff, not a chain.
5. No check that a state-changing mutation's inverse is non-empty and actually restores (V2-EMPTY-INVERSE). The two
   `Noop` inverses in section 7 pass the current law only if `base` and the diff result are equal, which they are not.
6. No per-leaf coverage gate (V4).

## 6. Policy gate `verify mutation-outcome-law` (`📜️script.ts`)

Route: `📜️script.ts:7223` → `runMutationOutcomeLaw` (`:7946-7952`, filters to `priority === "high"` at `:7947`). Named
target in `📋️project.json:2000-2006`. Shared bundle `policyMutationOutcomeMergePolicyBreaches` at `:21453`. Its
other callers: `:8503` (runGate) and `:25603` (policy).

Region `//#region 🔧️PolicyRuleMutationOutcomeMergePolicy` (`:20940-21462`). The shared inventory is
`policyMutationLawInventory` (`:20982`).

| Rule | Function (line) | What it checks |
|---|---|---|
| 1 | `policyMutationOutcomeBreaches` (`:21032`) | Every `🧬️mutations/<slug>/🔺️diff/🦀️.rs` returns `protocol::MutationOutcome<` and mentions one of the frozen codes |
| 2 | `policyMutationMessageCodeBreaches` (`:21130`) → `policyOutcomeCodeFileBreaches` (`:21141`) | Outcome code and level spellings against the frozen vocabulary; no `warn` spelling |
| 3 | `policyNoCrdtVocabularyBreaches` (`:21248`) | CRDT tokens (`merge_strategy`, `ConflictRule`, …) absent |
| 4 | `policyNoValidateOverrideBreaches` (`:21287`) | No `fn validate(&self, ..)` |
| 5 | `policySeverityInfoBreaches` (`:21344`) | No `Severity::Hint` |
| 6 | `policyMergePolicyParityBreaches` (`:21411`) | Three `MergePolicy` variants present in four surfaces |
| 7 | `policyDeriveGlueMountBreaches` (`:21436`) | Derive glue mounts the owner `🦀️.rs` |

No rule checks diff shape (whole-state), leaf `.apply(`, leaf `&mut`, leaf `between(`, a derived inverse, or an empty
inverse. Rule 1 checks only the return type.

Stale reference: the comment at `:21029` names `policyMutationTriadCompletenessBreaches`, which does not exist.

Where to add rules: inside the same region, as new `export function policy…Breaches(repoRoot)` next to rule 7, then
spread into the bundle at `:21453-21461`. Reuse `policyMutationLawInventory(repoRoot).mutationsDirs` and
`policyReadFileSafe`. Proposed rules: (a) `fn (diff|inverse)\([^)]*&mut` in leaf dirs; (b) `\.apply\(` in
`🧬️mutations/**` excluding `🔺️diff` composition; (c) `::between\(` in `🧬️mutations/**` and `↩️inverse`; (d) `type Diff =`
equal to the impl's self type (whole-state, V1-GENERIC-DIFF); (e) `fn inverse` body with `self.diff(`; (f) `Noop` in an
inverse of a non-empty diff.

## 7. Hand-written `impl Mutation<..> for ..` (not derive)

Strict grep: `rg -n "^\s*impl\b[^{]*\bMutation<[^{]*> for " 🧰️framework ✏️s -g '*.rs' -g '!**/target/**'`. Clean count
(no `$` or `#impl` macro templates): 111 lines, 100 files. Framework 30 (19 in test paths, 11 production). Artifacts 81 (1
test, 80 production).

Production classification (heuristic regex on the bodies, spot-checked):

| Diff class | Inverse class | Count | Examples |
|---|---|---|---|
| Whole-state (`MutationOutcome::new(config.clone())` / `presence.clone()`, `Diff = snapshot`) | Whole-restore (`Snapshot { config: base.clone() }`) | 47 | `🌿️vcs/…/🎚️config/🦀️.rs:143`, `📐️cad/…/👥️presence/🦀️.rs:102`, `🌀️procedural/…/🎚️config/🦀️.rs:158`, `🧩️puzzle/…/🪟️window/🦀️.rs:63` |
| Whole-state | Concrete per-field (heuristic) | 9 | `🏛️architect/…/📋️register/🎚️config/🦀️.rs:27` (inverse is `SetActiveRegister` with base value, so only the forward diff is V1) |
| Whole-state | `Noop` (no restore) | 2 | `🌿️vcs/…/👥️presence/🦀️.rs:68`, `🪐️space/🏠️home/…/👥️presence/🦀️.rs:65`. Diff resets to default, inverse does `[Noop]`, so V2-EMPTY-INVERSE |
| Clone-then-mutate (`let mut next = base.clone()` then `clone_from`, or whole arm) | Whole-restore | 8 | `📐️cad/…/🎚️config/🦀️.rs:224`, `💠️lowpoly/…/🎚️config/🦀️.rs:252`, `🧱️block/…/🧊️3d/…/🎚️config/🦀️.rs:214` |
| Clone-then-mutate | Concrete | 2 | `🌀️generation3d/…/👁️preview/🫧️transient/🦀️.rs:75`, `🏭️process3d/…/🎚️config/🦀️.rs:187` (V1-SNAPSHOT-DIFF) |
| Delegate (projection bridge to typed `Mutation<Snapshot>`) | Concrete | 6 | `🧩️puzzle/…/🧬️schema/🧬️mutations/🦀️.rs:566, :747, :542, :714, :853, :1034`. The `Mutation<Value>` bridges decode with `unwrap_or_default()` (silent default on decode failure, a contract issue) |
| Sparse-or-literal | Concrete | 6 artifacts + 9 framework production | Concrete examples: `🪐️space/🗿️artifacts/🪐️space/🦀️.rs:567` (`SpaceDiff` sparse fields, compliant), `🪐️space/…/🗂️collection/🦀️.rs:711` (sparse, compliant). Uninhabited stubs (`match *self {}`): sequence, wires, procedure, flow, dag, `NoConfig/NoPresence/NoTransient` |
| Apply-in-leaf | Derived-from-diff | 1 | `📓️print/🧬️schema/🧬️mutations/🦀️.rs:34` (`ChangeChartValue`) |

Totals, production: V1-GENERIC-DIFF (whole-state) 58; V1-SNAPSHOT-DIFF (clone-then-mutate) 10; V2-RESTORE-INVERSE 55;
V2-EMPTY-INVERSE 2; V3-LEAF-APPLY 1 (print leaf, plus the leaves in section 4); V3-HAND-MUTATION (bypasses the derive): all 80 production artifact impls are hand-written, not derived. None of them calls `apply` in its diff or inverse body (regex), so the bypass is structural, not a direct apply. Each is still a V3-HAND-MUTATION candidate under the design definition and should move to the derive path.

Whole-state `MutationDiff` impls (`Diff == Snapshot`, 57 of 223), e.g. `🧩️puzzle/…/👥️presence/🦀️.rs:24` (`apply` returns
`Ok(self.clone())`, `absorb` is `*self = other`), `🧰️framework/…/🎚️config/🧬️schema/🦀️.rs:131`
(`OpeningPreferences`), and the `$ty`/`$state` macro sites at `store/🦀️.rs:12887` and `plugin/🪟️window/…/🫧️transient/🦀️.rs:453`.

Not audited: `absorb` soundness (V5-ABSORB) beyond two samples. `ChartDiff::absorb` concatenates `edits`, which is
sequential but not coalesced. The 409 files with `fn absorb` were not reviewed.

## 8. Design-level implications (for the implementer)

- The 57 whole-state `MutationDiff` impls and 58 whole-state leaf diffs are the largest V1 class. The Minimality ruling
  covers them, and the per-artifact fix is one concrete diff per leaf in its `🧬️mutations/<leaf>/🔺️diff` directory.
- Generic seams named for deletion in the Rulings still exist. Token file counts (rough, `rg -l`): `diff_from_model` 295,
  `graph_edit_diff` 77, `commit_value_tree_edit` 2, `_selection_inverse` 21, `apply_in_place` 34, `SetSnapshot` 606,
  `Restore` 204, `apply_to(` 86.
- The central applier is the only proposed mint point. Start with `store::fold_operation` and route S-1..S-5 and P-1..P-4
  through it, then add the `ApplyCapability` parameter. The cross-crate limit in section 4 needs a decision first.

## Notes on method and limits

- Regex counts are line-level. Multi-line `impl` headers are missed. `.apply(` counts include non-diff applies.
- The classification table is heuristic. The 15+ bodies quoted above were read directly.
- `🗑️generated/hand-impl-classification.txt` (in this ticket) holds the full per-impl list. It is generated output, to be
  deleted at ticket close.
- Scratch intermediates are under `/private/tmp/claude-501/-Users-ueli-Documents-semio/04146190-065c-41c6-927c-62acd5070cc1/scratchpad/`.
