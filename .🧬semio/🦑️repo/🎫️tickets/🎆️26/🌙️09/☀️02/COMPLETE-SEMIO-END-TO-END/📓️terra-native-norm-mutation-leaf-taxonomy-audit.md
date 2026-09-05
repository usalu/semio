# Norm Mutation-Leaf Taxonomy Frontier — Current Source Audit

**Scope:** the live `semio-s-plugin-norm` mutation-leaf fan-in only. This is a read-only source census performed 2026-09-04. No Norm Cargo or native D1 gate was run, so this report is source evidence, not a compile or runtime result.

## Verdict

**RED — two mechanical, complete-source taxonomy faults precede the native D1 law.** They have a small, ordered repair that does not need a compatibility layer:

1. Remove **250** imports which introduce a type name into the same mounted component module that immediately declares that type.
2. Add the actual `MutationLeaf` metadata to all **21** mounted ISO-16757 split payload leaves.

The old “265 self imports” wording was too broad. The current census finds 265 direct leaves that import *some* `mutations` path, but only 250 imports bind a locally declared payload name and are the deterministic duplicate-name failures. The other 15 are cross-leaf module/helper imports and must remain.

## Exact live taxonomy and evidence

`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/🦀️.rs` mounts two materially different layouts.

| Physical layout | Mounted shape | Current count | Required ownership rule |
| --- | --- | ---: | --- |
| Direct component leaf | `…/🧬️mutations/<leaf>/🦀️.rs`, mounted as `mod component; pub use component::*` | 371 semantic leaves; 250 local-name import collisions | The payload is defined in `component`; it must not import a same-named type from its public parent module. |
| Split triad leaf | `…/🧬️mutations/<leaf>/{🔺️diff,↩️inverse,🦠️mutation}/🦀️.rs`, mounted as sibling modules | 21 semantic leaves, all ISO-16757; all 21 lack leaf metadata | Only `🦠️mutation/🦀️.rs` owns the payload and must derive `dsl::MutationLeaf` with the protocol contract. |
| Aggregate | `…/🧬️mutations/🦀️.rs` | not a leaf | It only dispatches variants. Do not add a blanket implementation or relax its aggregate constraints. |

The direct representative is [`🗻change-annex/🦀️.rs`](../../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️🟠️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗻change-annex/🦀️.rs):7,15-20: it imports `change_annex::ChangeAnnex` and then declares `pub struct ChangeAnnex` in the same component scope. The parent wiring at [`🦀️.rs`](../../../../../../../../../✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/🦀️.rs):5031-5038 confirms the `component`/public-reexport shape. A bounded synthetic `rustc` probe of that exact module pattern yields `E0255` (“name defined multiple times”); an alias would merely conceal the accidental cyclic dependency and is not an acceptable repair.

The split representative is [`🛋️remove-selection-constraint/🦠️mutation/🦀️.rs`](../../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️remove-selection-constraint/🦠️mutation/🦀️.rs):6-23. It implements `protocol::MutationKind` but has neither `dsl::MutationLeaf` nor `#[mutation_leaf(contract = ::protocol)]`. Its three sibling modules are mounted at [`🦀️.rs`](../../../../../../../../../✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/🦀️.rs):139-147 and its aggregate dispatches all 21 payloads at [`🧬️mutations/🦀️.rs`](../../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs):26-68. The framework contract has real descriptor and provenance constants, not a marker-only bound: [`mutation/🦀️.rs`](../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs):745-749.

The 21 missing split payloads are exactly the ISO-16757 leaves: `add-selection-constraint`, `change-exchange-process`, `change-part-number-input`, `change-selection-class`, `change-selection-series`, `create-product`, `create-product-group`, `create-property-definition`, `create-subject`, `delete-product`, `delete-product-group`, `delete-property-definition`, `delete-subject`, `remove-part-number-input`, `remove-selection-constraint`, `rename-catalogue`, `rename-manufacturer`, `rename-product`, `rename-product-group`, `replace-part-number-rule`, and `update-script-limits`.

### Census method

The read-only Bun census traversed only `🦀️.rs` files below Norm's `🧬️mutations` directories and classified a semantic payload by an `impl protocol::MutationKind<…>`:

- direct candidate: exactly `<leaf>/🦀️.rs` below `mutations`; 371;
- collision: a `use …::mutations::…` statement binds the same identifier as a `pub struct` in that source; **250**;
- split candidate: `<leaf>/🦠️mutation/🦀️.rs`; 21;
- split payloads with `dsl::MutationLeaf`: 0; with the contract attribute: 0.

It deliberately excludes aggregate dispatch files, sibling diff/inverse files, and the 15 legitimate cross-leaf module/helper imports. This is why it corrects the older broad 265 count without downgrading the source RED.

## Smallest dependency-ordered implementation packet

### P0 — direct-leaf import hygiene

For every one of the 250 colliding direct components, delete only the `use` binding whose imported identifier equals the local `pub struct` name. Preserve imports of the aggregate mutation enum, snapshot/diff types, and other leaf modules/helpers. Do not use `as`, re-export aliases, wrapper types, or module-layout changes.

This removes the first deterministic Rust error without changing a semantic descriptor, reducer, inverse, wire form, or aggregate variant.

### P1 — make split payloads actual leaves

In the listed 21 ISO-16757 `🦠️mutation/🦀️.rs` payload files, extend the existing payload derive with `dsl::MutationLeaf` and add `#[mutation_leaf(contract = ::protocol)]` above the struct. Keep the present `MutationKind`, diff, inverse, enum variant, `KINDS`, and binary/text codec intact. Do not provide a blanket `MutationLeaf` implementation and do not weaken `dsl::Mutations`; either would erase the descriptor/provenance proof this packet is supposed to restore.

P0 comes first because it exposes a valid compile frontier. P1 is independent in data semantics but must be included before accepting the Norm mutation vocabulary. Static VDI catalog and viewer-assembly errors remain outside this packet and must be re-evaluated only after P0/P1 terminal evidence; this packet alone does not make native D1 green.

## Required non-vacuous gates

The current Norm Rust router at [`📜️script.ts`](../../../../../../../../../✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts):7-22 only offers package-wide `test` and `describe`; it has no exact-one taxonomy gate. Add one registered `mutation-leaf-taxonomy-check` command there rather than relying on a broad test pass.

1. **Bun source census:** that command must recompute the four counts above and fail unless collision count and split-metadata absence are zero. It must also fail if a newly added mounted semantic payload has no taxonomy row.
2. **Neutral oracle:** add `semio.norm.mutation-leaf-taxonomy/v1` JSON rows for every mounted payload: artifact/standard/subset, physical layout, module/type, aggregate variant, and semantic `{kind, verb, entity, record}`. An AJV/Node reader must validate the corpus without importing Rust, accept all committed positive rows, and reject a duplicate local binding, wrong layout, missing payload row, duplicate aggregate variant, and altered semantic descriptor. Existing ISO-16757 fixture/codec laws prove mutations but do not independently prove the missing descriptor metadata.
3. **Exact Rust law:** add one named law, for example `mutation_leaf_contract_tests::all_mounted_norm_leaves_match_taxonomy_v1`, which reads the real `MutationLeaf::DESCRIPTOR`/provenance and checks every row plus actual apply/inverse for the committed vectors. The router must first list tests, require exactly one fully-qualified match for the suffix, then run `cargo test -p semio-s-plugin-norm --lib <FQN> -- --exact --test-threads=1`. Do not count selection or compilation as execution.
4. **Current cross-fan-in:** after P0/P1 succeeds, run the existing uncached NX frontier:
   ```sh
   bun nx run @semio-tech/framework-renderer-wgpu:native-build --skip-nx-cache -- --scale
   ```
   Its current implementation invokes `cargo build -p semio-framework-renderer-wgpu --bin semio-wgpu-native --features native-bin` before the scale early return ([`📜️script.ts`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts):298-309).
5. **D1 escalation:** only after that terminal succeeds, run the registered hub gate:
   ```sh
   cd 🌎️hub/📦️packages/🦀️rust && bun ./📜️script.ts native-document-open-check
   ```
   It runs the neutral plan oracle, uncached WGPU NX build, and owned native admission laws in that order ([`📜️script.ts`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts):3037-3045). A P0/P1 source repair is not D1 runtime evidence.

## Handoff and nonclaims

Assign P0 and P1 to the Norm owner as one atomic taxonomy change with the new source/oracle/exact-one gate. The first acceptance point is the new Norm gate; the second is the existing WGPU fan-in. Native D1 remains **BLOCKED** until both complete and the registered D1 gate reaches its own assertions.

This report makes no claim about Draw, Terminal/stdio, VDI static catalogues, UI assembly, generated artifacts, or a working linked catalog. No repository source, tests, plan, matrix, or launch entry was edited by this audit.
