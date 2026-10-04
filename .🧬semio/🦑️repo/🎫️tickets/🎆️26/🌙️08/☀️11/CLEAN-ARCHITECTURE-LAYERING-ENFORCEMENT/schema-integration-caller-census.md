# Current Schema Integration And Caller Census

Readonly actual-source audit. No production edits, native commands, tests, or runtime claims. Full exact source inputs retained temporarily under generated/schema-integration-census. Paths below are repository-relative raw source coordinates; prepend workspace root when linking.

## Actual TypeScript And Task Entry Points

- Neutral Schema `🧰️framework/🔨️modules/🧬️schema/🟦️.ts` exports descriptors, state vocabulary, named export types, registries, and entity catalog validation. It has no general document source/reference closure service. Registry `register` overwrites by ID. Do not conflate facet-leaf descriptor registration with strict resource indexing.
- Validator `🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts` exports unknown shape guards and `validateJsonSchemaSubset(schema,value,root)`. Private `jsonSchemaSubsetResolve` only handles local `#`, drops empty pointer components, ordinary property accesses, and decodeURIComponent. It cannot discover explicit resources, nested IDs, anchors or external dependency closure; schema-aware source capture belongs at the new Schema source owner, not inside the existing instance validator.
- Validator TS task owner `🧬️schema/✅️validator/📦️packages/🟦️typescript/📜️script.ts` uses BundleScript, ScriptRouter, runScriptMain, and `runOwnedCommand(process.execPath,["test",...],...,"schema-validator:shape",15000)`. Current command is `test shape`; package script `test-shape` invokes nx target; project target invokes `bun ./📜️script.ts test shape`, cache:false, inputs validator subtree + sharedGlobals. New neutral source tests should follow this actual owned routing rather than adding any extra script file.
- Schema task owner `🧬️schema/📦️packages/🦀️rust/📜️script.ts` already routes pure TS laws via `test subset-contract`, `test mutation-leaf-registration`, `test neutrality`, and entity ownership beside native tests. `test neutrality` and registration require caller-owned SEMIO_TEST_ARTIFACT_DIR, use runOwnedCommand with 15000ms; subset-contract directly imports oracle helper. A TS-only source-closure command can extend the contract map without invoking Cargo. Its project and package entries must follow existing target/package->nx->script flow.
- `.vscode/launch.json:10483` current shape command is `bun nx run @semio-tech/schema-validator-ts:test-shape --skip-nx-cache`, group `9_gates`, order `900.05778`, caller-owned ticket generated dir in env. Schema neutrality is nearby at line 10730. Register new source-closure launch entry in corresponding existing grouping/order.

## Actual OS Derive Source/Index And Consumers

Owner: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs`.

| Source / Line | Actual Behavior | Canonical Replacement Obligation |
| --- | --- | --- |
| 575 | calls referenced-document walker and translates returned paths to include paths | consume captured, admitted closure physical identities; preserve include dependency bytes |
| 616 | emits leaf PAYLOAD_SCHEMA_DOCUMENTS via include_str! | unique referenced physical documents, excludes root; no silent missing reference |
| 707 mutation_leaf_referenced_documents | pending external IDs, seen initialized by root $id, unknown index entries continue silently, paths sorted | explicit unknown refusal; retain every origin edge and deduplicate physical identities rather than reference IDs |
| 727 mutation_schema_search_root | derives broad plugin or framework module directory from filesystem ancestors, fallback schema owner/parent | source capture stage explicitly owns resource set; neutral string resolver must not infer filesystem containment/existence |
| 735 mutation_schema_document_index | thread-local cache keyed only by root PathBuf; cached once per compiler process | eliminate ambient stale root cache; explicit exact source input receipt per operation |
| 740–750 walk | unreadable dirs/files and malformed JSON skipped; schemas under broad walk; unsorted read_dir; root `$id` only; index.entry(id).or_insert(path) first wins | deterministic discovery/capture outside neutral resolver; malformed/duplicate resource refusal; no traversal-order authority |
| 765 mutation_schema_document_references | serde_json parse; recursively walks every object/array including const/default/enum/examples; strips fragments; skips local refs; relative text looked up verbatim | owned strict member authority before parse; schema-aware children only; nested URI scope, anchors/pointers, local validation, explicit external identities |
| 2161,2249 | collects each leaf list into Mutation::INPUT_SCHEMA_DOCUMENTS | retain per-leaf closure relationship and exact source provenance |

Actual contract defaults: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:247` INPUT_SCHEMA_DOCUMENTS and line 980 PAYLOAD_SCHEMA_DOCUMENTS are static raw source arrays. Puzzle 2d/3d/5d mutation bridge impls forward the same Mutation constant; no new discovery algorithm there.

Actual runtime registration consumers: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24753` async AppController with_registry_on_bus loops lists and registers before creating snapshots/store. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs:878` async member store visit registers before history owner work. Both call `semio_framework_schema_registry::register_referenced_schema_documents`.

Canonical registry implementation `🧬️schema/📇️registry/🦀️.rs:625` deduplicates by raw body string equality and preserves publication order, under global catalog lock; it does not establish unique IDs/source provenance. `registered_referenced_schema_documents` returns cloned raw list. New TS closure should not claim this runtime registration already consumes exact owned resource authority.

General Schema integration `🧬️schema/⚛️component/🦀️.rs:290` structural_validator_in resolves selected JSON export and indexes all sibling JSON leaves by root `$id`. Malformed/unresolvable siblings are silently skipped; identical-ID same-body allowed, conflicting bodies error; sibling bodies sorted by ID then compile_with_documents. This is actual separate runtime consumer needing eventual source-resource policy parity. Current TS-only work can expose the explicit closure contract and tests without claiming these Rust consumer cuts completed.

## Current Pack JSON Member Provider Availability

At audit read time, actual `🧰️framework/🔨️modules/🎒️pack/🔤️json` had four files: owned Rust source, unit test, fixture JSON, owned-controls fixture. No TypeScript members owner, members schema, or independent Cargo package files were present yet in this concurrently edited tree. `JsonMemberPolicy`/`DuplicateMemberPolicy`/`MemberPolicy` exact source search found no current declaration. This is a timestamped observation; re-read before integrating High agent output.

Actual existing Rust parse_object currently inserts duplicate names with replacement, and controlled parser explicitly replaces duplicate member values. Existing UI OwnedUiSceneJsonCursor is scene-source coupled and not a neutral strict TS parser; Repo regex scanner and OS catalog recursive scanner remain product-owned. Therefore no usable already-mounted strict neutral TS member parsing interface was found; Root should directly import the new canonical `pack/🔤️json/🧩️members/🟦️.ts` once present, mandatory production Reject, without copying or forwarding old scanner names. Source-closure schema admission can remain private; do not expose numeric-data transport to justify platform parse.

## Exact Inputs

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `🧰️framework/🔨️modules/🧬️schema/🟦️.ts` | 15150 | `a7d7c20d59b42a1734d33d36a49c790043a63fc3384a0451b66bb1a4498d5548` |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts` | 10467 | `2617c03029c26a0cb97b3cf7fab1fa159b03c3f821d163ba55a55902c76170f3` |
| `🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs` | 18467 | `4b2d01c82f1b0b4e8c179951c7dcb15743f17d7a468389e07a5cec99a7e5ad03` |
| `🧰️framework/🔨️modules/🧬️schema/📇️registry/🦀️.rs` | 31642 | `a6623d265ff7c3922ba58fd39f85f04dc2717a4c8bf557399e2341d28e35614e` |
| `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts` | 4629 | `70d460e57f848062246f3a9097edb0a9f73b0010915f56871ba51c0c61b47fc3` |
| `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📋️project.json` | 4882 | `42f8489cd1915945e4f801b9f3164b3d033126789b3c85d51a10ecdda1789998` |
| `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/package.json` | 834 | `f89eb474862e0d11dda80a708183d7f88c5fd66d9fec37b79bda84a24a29a89a` |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/📜️script.ts` | 885 | `32e361caee0e8dde0d7b2026771ca10851418d9930a6669326dc301d222475cc` |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/📋️project.json` | 570 | `baa98e73f1dbc4218340e8997919d730e4a409857159c5e15c3a667f0caf9f65` |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/package.json` | 218 | `53c2804f9f8829812c469e867a8eb1637235b20752b70953764415a7c4f3de99` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs` | 166451 | `d9f28be7e7f71db1cc63dec03fa5a8d2bfb419b3de5def1ebfad94204e732005` |
| `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs` | 100813 | `39de4fad6a5473030d39687d74dfc7c2fcebcb8c2a74508ac648c3b875479ac2` |
| `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs` | 76901 | `65c771273827c85ab9dedfc9f007d8c5caa0538a140ac2c5f952d86f8810da47` |
