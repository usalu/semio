# Schema Neutrality Portable Baseline and Production Contract

Observed 2026-10-02. This execution ledger supplements the historical read-only preparation report. Production Rust, Cargo manifests and native fixture bytes remain unchanged by this preparation. The live catalog/Draw epoch remains the production hold boundary.

## Executed baseline

Both the direct permanent owned script and registered Nx target reached the real 15-law test file and refused the current source: **4 passed, 11 failed, 100 assertions**. The independent portable model is positive; actual source ownership remains RED.

| Route | Command | Bun test duration | Result |
| --- | --- | --- | --- |
| Direct | bun schema/📦️packages/🦀️rust/📜️script.ts test neutrality | 2.16s | exit 1, 4/11/100 |
| Registered | bun nx run @semio-tech/framework-schema:test-neutrality --skip-nx-cache | 3.96s | exit 1, 4/11/100 |

The registered Nx task reported 1m40s. Its preceding multi-minute project-graph bootstrap is a separate interval. The authored driver retains its 15,000ms owned test budget. Nx inferred normal native preparation and entity generation; generation compared existing bytes without changing generated TS/Rust outputs. No Cargo build was launched by this test.

| Exact retained log | SHA-256 |
| --- | --- |
| .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality/current-schema-neutrality-owned-script-red.log | bd190c4e57c7e1f36d122bbb750acbe8d2d24103e62eac5c572a99c2c9040cb2 |
| .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality/current-schema-neutrality-source-baseline-red.log | 37b35834f96d7703ebc7714f28e3c397ba2733b901040f4888b4ab1f1fab626d |

Independent Node output is retained in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality/schema-neutrality-node-oracle.json` and the direct subdirectory. It uses existing AJV admission and fast-deep-equal with a Map-based transaction projection, independently of the owned array-based model. The baseline pins eight child projections, four states, eighteen full typed descriptor transactions, and all thirty original law names. Real source failures cover the product edge, missing composition/state packages, descriptor identities, catalog aliases/cycle, ignored mirror Result, missing free version functions and concrete-child law ownership.

## Canonical provider and API decisions

- Composition becomes one std-only package under the existing lower composition source; the OS compile mount is physically removed. Six composition identities retain one provider.
- State becomes one Value-backed lower package under schema/📶️state. Protocol consumes this identity rather than defining it.
- Existing std-only schema/📇️registry owns FacetLeaves and the artifact/inference/app descriptors plus transactions. Protocol depends on this lower registry, never main Schema, avoiding the actual Schema → Pack → Protocol cycle.
- Main Schema owns explicit free version functions over the lower descriptors. All actual callers must be rebound; no foreign inherent implementation or Kernel aliases/conversions remain.
- Single registration retains replacement behavior. Artifact mirror conflict is refused before either descriptor or export publication. Batch registration is strict and atomic under one owner lock: validate established and intra-batch conflicts, then publish the complete transaction. Failed batches publish no prefix, including their unique entries.
- Catalog families remain independent. User callbacks receive snapshots after the owner lock is released. Preflight is an observation, not a publication permit; batch commit validates current state again inside its one transaction.

## Atomic schedule refinement (native proof pending)

After the accepted baseline, the closed corpus was strengthened with two exact full-descriptor atomic schedules and a hostile schema substitution. The prior separate-preflight model explicitly shows two accepted stale observations followed by conflicting publication. Desired schedules permit exactly one winner and refuse the complete losing batch; loser-only keys must remain absent. This is a portable contract, not an executed Rust atomicity claim. Current refinement replay is pending.

The future native witness must invoke actual owned registry operations from two threads coordinated before admission, retain both complete conflicting batches, assert exactly one success and one conflict, and compare the complete final snapshot against the two independently authored schedule outcomes. A second witness invokes a reentrant registry read inside the user visit callback to verify lock release. No lock is held across callback execution. Native execution is delegated to the sole native queue after coordinated production release.

## Retained laws and registration

All thirty original component laws remain required. Only `artifact_composition_projection_real_child_alias_has_fixed_admission_bounds` moves to the actual higher OS child owner with its original 64/65, 257 traversal and 128 × ä UTF-8 assertions unchanged. No native law has yet moved. Launch seed 900.05790 owns Schema test-neutrality; the adjacent Oracle agent owns 900.05791. Generated launch files are untouched. Current earlier shared types/matcher receipts do not imply this new asset epoch is typechecked.

## Exact isolated authored asset roster

These eight paths are the only current preparation source changes. Hashes observe the post-atomic-schedule refinement and may differ from the executed baseline corpus.

| Path | SHA-256 |
| --- | --- |
| 🧰️framework/🔨️modules/🧬️schema/🧬️schema/🧱️neutrality/🔣️.json | b10f883d76f7b660c9fcfd4ca4daf44498226b9230c915ad22847177f4ee0049 |
| 🧰️framework/🔨️modules/🧬️schema/🧫️fixtures/🧱️neutrality/🔣️.json | df6c47e9c03020964ceea0e7947183c67767c1f4a9753beeb7220876718dc23b |
| 🧰️framework/🔨️modules/🧬️schema/🧪️tests/🧱️neutrality/🟦️.ts | e31631ee13f489181391eda835cd351bfce38a06b1919592bdce2767ee0d1aab |
| 🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts | 70d460e57f848062246f3a9097edb0a9f73b0010915f56871ba51c0c61b47fc3 |
| 🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📋️project.json | 42f8489cd1915945e4f801b9f3164b3d033126789b3c85d51a10ecdda1789998 |
| 🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/package.json | f89eb474862e0d11dda80a708183d7f88c5fd66d9fec37b79bda84a24a29a89a |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json | 7c256ebd4dfe5196da44441f62e7b3e5da8604836f0329e2f4a60c7adf2177ec |
| .vscode/🧩️launch.seed.jsonc | 5d6446e2c8c52d7c7721fdf57185245819b13ff899682ae8061ca6813922bb4c |

The first three paths are new colocated contract/schema/test assets; the last five extend existing canonical routing/registration. No new permanent script outside 📜️script.ts exists.

## Fresh complete version-API inventory

A fresh literal source query also found `AppSchemaDescriptor::config_schema_version` and `presence_schema_version` at component557/561, in addition to the four Artifact descriptor methods. Since App descriptor moves to the same lower registry, these two methods must become explicit main-Schema free functions over `&AppSchemaDescriptor`. No external method callers were found by the bounded Rust source query. Current isolated fixture still covers the initial four Artifact functions; six-function refinement is pending the active replay terminal. This finding does not authorize a foreign inherent implementation or Protocol → mainSchema cycle.

## Single canonical owner state

The existing lower registry also has separate clone/preflight/relock operations in `register_scope_schema_exports` and `register_scope_facet_leaves` (registry291–315). New descriptor transactions cannot be atomic with their mirrors if these old writers retain separate lock authority. The canonical registry will own one state containing all three descriptor families, named exports, fixed facets and referenced-document membership. All public transaction validation/publication uses one lock for that state. Reads clone the necessary snapshot and release the lock before invoking caller callbacks. Tests must preserve family independence, duplicate acceptance, single replacement where no mirror conflict exists, and exact strict failures.

## Atomic refinement direct execution

The same permanent owned script reached all fifteen current laws after the two schedule assertions and hostile substitution were added: **4 passed, 11 failed, 105 assertions; Bun 3.19s; exit 1**. Actual DEBUG confirms eight child projections, four states, eighteen typed transactions, both atomic schedule models and the old separate-lock defect projection matched the independent Node/AJV output. This is portable proof only; all existing native concurrency/publication obligations remain pending.

Log: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality/current-schema-neutrality-atomic-owned-script-red.log; SHA-256 `9034c94832d58b046dbceb383afb625f36fd8d419bdc64b229fb2741ea2d0c22`.
Independent oracle: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality/atomic-direct/schema-neutrality-node-oracle.json; SHA-256 `74e8772a2b4ee5bda9b88d4a54075c1addc6c060509ed3b918e2ff61ebe6e8e6`.

Exact independent schedule output:

```json
[
  {
    "id": "first-transaction-wins",
    "decisions": [
      {
        "actor": "first",
        "accepted": true
      },
      {
        "actor": "second",
        "accepted": false
      }
    ],
    "entries": [
      {
        "id": "first-only",
        "inference": {
          "rust": "pub struct First;",
          "typescript": "export type First = {};",
          "graphql": "type First { value: String }",
          "json_schema": "{\"type\":\"object\",\"title\":\"First\"}",
          "proto": "message First {}"
        }
      },
      {
        "id": "same",
        "inference": {
          "rust": "pub struct First;",
          "typescript": "export type First = {};",
          "graphql": "type First { value: String }",
          "json_schema": "{\"type\":\"object\",\"title\":\"First\"}",
          "proto": "message First {}"
        }
      }
    ]
  },
  {
    "id": "second-transaction-wins",
    "decisions": [
      {
        "actor": "second",
        "accepted": true
      },
      {
        "actor": "first",
        "accepted": false
      }
    ],
    "entries": [
      {
        "id": "same",
        "inference": {
          "rust": "pub struct Second;",
          "typescript": "export type Second = {};",
          "graphql": "type Second { value: String }",
          "json_schema": "{\"type\":\"object\",\"title\":\"Second\"}",
          "proto": "message Second {}"
        }
      },
      {
        "id": "second-only",
        "inference": {
          "rust": "pub struct Second;",
          "typescript": "export type Second = {};",
          "graphql": "type Second { value: String }",
          "json_schema": "{\"type\":\"object\",\"title\":\"Second\"}",
          "proto": "message Second {}"
        }
      }
    ]
  }
]
```

Current registered refinement is waiting on Nx graph construction in another process (session83855). It has not reached the target and is not a current registered pass or refusal yet. The earlier registered4/11/100 remains the historical real-source baseline.

The fresh single registration roster is retained at `📓️2026-10-02-schema-single-register-source-caller-roster.md`: 86 literal rows across40 physical files, including definitions/internal/tests, excluding generated outputs. This is a complete result for that bounded textual query, not semantic or mounted context completeness.

## Lower-state codec and lifecycle inventory

Current protocol wire294–323 owns StateClass and handwritten ordinary ToValue/FromValue methods. Its four wire spellings are pinned in the portable corpus. No controlled methods are present in that existing implementation. The extraction must retain the existing ordinary wire/error semantics and one provider; it cannot claim controlled encoding/decoding merely by moving the implementation. Controlled admission for this scalar, if required by actual callers, needs explicit owned implementations and actual bounded native vectors. No ordinary-then-charge bridge is justified.

## Planned native atomic publication witness

Use actual canonical public inference batch registration, not a test-only map implementation. Two owned threads rendezvous at a three-party std Barrier before invoking the two authored conflicting batches. Each batch has one unique id followed by the shared id, ensuring that failed commit must retract/refuse a potentially publishable prefix. Join both results; require exactly one success and one descriptor conflict; take the actual public snapshot and match it against exactly one of the two independent full-descriptor outcomes. No losing unique id may exist. Run both fixture orders with distinct isolated ids and retain family independence assertions. This contract accepts scheduling nondeterminism only between the two explicit serialized outcomes; success/failure/publication invariants are deterministic.

The callback witness invokes actual owned registration/read inside the public visit callback, communicates completion through a bounded channel, and fails on timeout. It must not require a private lock escape, reentrant mutex or a cloned fake state. Native High owns execution once production is released. Until then, only the language-neutral schedule contract is executed.

## Atomic refinement registered terminal

Session83855 is now terminal exit1. The registered Nx target reached real current source tests: **4 passed, 11 failed, 105 assertions**, Bun 5.41s; Nx reported task duration1m25s (preceding graph wait remains separate). The two atomic schedules and hostile substitution passed against independent Node/AJV. Exact log: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality/current-schema-neutrality-atomic-contract-red.log; SHA-256 `c22a7e4bc1963b2587897c82c73b10d86e3f98f0037315ae9dd4ad352a360eb5`. This closes the registered atomic portable refinement receipt and retains the same genuine source RED.

Immediately after the terminal, the six-version-function fixture/schema/test refinement was authored. This later API inventory epoch is not covered by the105 registered result yet. Rust/Cargo/native fixtures remain unchanged.

## Generated binding ownership inventory

The actual schema derive source is `schema/✨️derive/⚙️expansion/🦀️.rs` (not a generated file). Its emitted StateClass paths are at223/262; composition paths at235/238/249/270/271/275/278. These currently emit `::semio_framework_schema::...`. Production must emit direct lower `semio_framework_schema_state` and `semio_framework_schema_composition` identities, and every actual mounted deriving caller needs the corresponding direct Cargo providers. Main Schema cannot retain forwarding imports to hide missing caller ownership. This is a generated-binding source correction, not manual modification of generated Rust. The associated derive integration/compile fixtures must prove the emitted lower identities match the one actual type provider.

The complete public ArtifactSchemaRegistry, ArtifactInferenceRegistry and AppSchemaRegistry type families belong with their lower descriptors and owner state. Main Schema retains real schema operations such as version parsing, normative JSON catalog construction, GraphQL preamble composition and validation. Direct consumers of raw descriptor/registry/state/composition APIs bind lower owners; main-Schema schema operations consume those direct APIs without type conversions or forwarding facades. Actual caller/manifests must be inventoried at implementation time, preserving all physical mount contexts.

## Reachable public native setup boundaries

Portable transaction cases accept explicit initial snapshots; they do not imply every arbitrary initial snapshot is reachable through the eventual global public API. In particular `artifact-single-replace` starts with an unmirrored descriptor: it pins the retained descriptor-container replacement behavior when there is no mirror conflict, while the new unified global artifact writer always publishes descriptor and facets together. Its native law must use the actual retained public local descriptor container where appropriate, and cannot seed an invalid global state through a test-only bypass. Global mirror refusal cases can be built through actual public facet registration before descriptor registration. The actual concurrent witness deliberately uses inference batches, whose complete empty initial state and two competing transactions are publicly reachable without private state injection. No portable model is substituted for these native operations.

## Six-version current asset hashes

| Path | SHA-256 |
| --- | --- |
| 🧰️framework/🔨️modules/🧬️schema/🧬️schema/🧱️neutrality/🔣️.json | 30eeba8918f2227b8cf44b09a2a84d6b11dfbab023298d9a04a130e5065de369 |
| 🧰️framework/🔨️modules/🧬️schema/🧫️fixtures/🧱️neutrality/🔣️.json | cf8b32572982381efda8eef1acf44d19d6caa6f395deee4e16f7860812f0b678 |
| 🧰️framework/🔨️modules/🧬️schema/🧪️tests/🧱️neutrality/🟦️.ts | 96c8a4a54c4817c568e900e9ca5c3f049371632185e1e4358d98b433ddc6c97b |

These hashes supersede the initial four-version asset hashes only for this current refinement; execution receipts remain associated with their actual earlier source epochs.

## Final isolated six-version registered receipt

Session94929 reached the current six-version source-contract epoch and is terminal exit1: **4 passed, 11 failed, 105 assertions; fifteen laws; Bun 2.63s**. Nx reported 56.5s task duration. All positive portable contracts including two atomic schedules match actual independent Node/AJV. Eleven source ownership/identity/version/higher-law obligations remain genuine RED, as required before the production cut.

Log: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality/current-schema-neutrality-six-version-source-red.log; SHA-256 `17b5ebc86277cb76af14441d282631f36009165e0beb547ccb445c3a50246188`.
Oracle: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality/six-version/schema-neutrality-node-oracle.json; SHA-256 `74e8772a2b4ee5bda9b88d4a54075c1addc6c060509ed3b918e2ff61ebe6e8e6`.

Normal inferred entity generation ran compare-before-write; the actual generated outputs remain byte-identical to the baseline and their modification times remain unchanged:

| Generated canonical file | SHA-256 | Last write |
| --- | --- | --- |
| 🧰️framework/🔨️modules/🧬️schema/🤖️generated/🏷️entity-kinds/🟦️.ts | 2cddc1968109a1912b03b8dc567eb91c372fdedd582df0c3b52ec45beb21a397 | 2026-09-12T22:15:54.065Z |
| 🧰️framework/🔨️modules/🧬️schema/🤖️generated/🏷️entity-kinds/🦀️.rs | b08b820670b9c7d5b930d44a7ee9f1496ac81c78e72c9f892dcdf220045b1c54 | 2026-10-01T15:09:24.302Z |

The isolated contract/schema/test/script/project/package/taxonomy/seed frontier is now source-stable. No production Rust/Cargo/native fixture change has started. Current native catalog/Draw hold remains; no concurrency runtime promise or full products-deleted proof is claimed.

## Current Production Source Handoff

The registered current source contract is GREEN15laws/144assertions/2.05s Bun,1m29s Nx task duration (bootstrap graph wait remains separate), uncached. Log: 🗑️generated/goal-schema-neutrality/current-schema-production-portable-registered.log. Its independent Node/AJV JSON is retained under goal-schema-neutrality/current-registered. The two new assertions forbid mainSchema registry/state/composition forwarding facades. This is source-only evidence, not native lock/concurrency or whole product deletion proof.

Exact durable source/provider handoff: 📓️2026-10-02-schema-production-source-handoff.md.433 distinct direct Rust rebinding sources;109 distinct authored consumer manifests across104first-pass and6final-pass edits (one overlaps). Final physical inventory24,748source/manifest inputs/453participation observations/115required consumer manifests/oneunmounted candidate. Denied contexts remain unproved; the inventory records direct provider obligations without granting scope authority. Both generated JSON provider inventories and both durable exact source rosters remain retained.

New native surfaces are authored and unexecuted here: registry3public-operation integration laws (Barrier conflicting atomic batches, failed mirror/no prefix, unlocked callback reentrancy), State2Serde/controlled laws, Composition1eight-vector Option/Vec admission law. The original30component laws remain29inmainSchema plus1unchanged concrete-child body in OS/store. The actual higher IO type owner is crate::os_io; no nonexistent neutral IO provider was introduced. OS devdependency on mainSchema is acyclic after the removed mainSchema→OS edge. Native High alone receives the full runtime, public API and products-absent closure obligations.

Authored permanent lower package routes and project/package inputs are registered. Launch900.05793–900.05801 preserve Oracle900.05792 and all earlier JSONC entries. New schemaState semantic member is registered; closed neutrality fixtures/schema remain one parent Schema authority shared by the lower owner tests. No manual generated Rust, Git/worktree or AGENTS edits.

The actual running catalog4 belonged to a prior unadmitted live frontier. Its eventual receipt cannot attest this source epoch. This handoff makes no whole-goal completion claim.
