# r8 execution report: z-baseline (HEAD compile fixes for the BIM graph)

`FL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow`, `FA` = `FL/🗿️artifacts/🌊️flow`, `W` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows`.

## Result
`cargo check --manifest-path ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml -p semio-s-artifact-bim-model --tests --message-format=short` is green native AND with `--target wasm32-wasip2` (both `Finished`, 0 errors).

## Cross-team fixes (owners: please review)
Migrated reference for every `RetainedCloneGrant` fix: `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs` (`ColdDictionaryBuilder::insert{,_controlled}`, `ValueRetirement::close_step`) and the API itself in `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs` + `🧬️retained-clone/🦀️.rs`.

| File | Error | Fix | Followed |
|---|---|---|---|
| `FA/🧵️retained/🦀️.rs` (`set_cursor`, `layout_cursor`, `node_cursor`) | `expected RetainedCloneGrant, found Grant` | `Retirement::advance(RetainedCloneGrant{maximum_items:1, maximum_copy_bytes:maximum_bytes, maximum_capacity_bytes:0, maximum_release_bytes:maximum_bytes, maximum_depth:values.next_depth_demand()})`; new `RetirementStep::ProcessedBytes(n)` counts as progress `n`; `Failure` records `fault` ("Flow ordered retirement refused"), reinstalls the cursor and reports blocked | neural `close_step` (line ~90) |
| same, `owner_release_demand` | match arms `Result<usize,ValueError>` vs `Result<usize,&str>` | ordered demands `.map_err(\|_\| "Flow ordered retirement demand refused")` (return type stays `&'static str`) | neural `next_close_byte_demand` (line ~70) |
| `FA/🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs` (`retire_ordered`, `insert_ordered`) | `Grant` vs `RetainedCloneGrant` | per-turn grants from cursor demands (`next_capacity_byte_demand`, `next_depth_demand`, `next_close_byte_demand`, `next_close_depth_demand`), `Failure` panics like neural cold paths | neural `ColdDictionaryBuilder::insert_controlled` |
| `FA/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🛬️decoding/🦀️.rs` (`retire_map`, `insert`, new `release_shared`) | `Grant`; `Arc` vs `SharedOwner<V>` (`take_removed` now returns `SharedOwner`) | same grants; removed value released via `SharedOwner::release_step(RetainedCloneGrant::one_release_turn(value.next_release_byte_demand(), 1))` replacing `Arc::into_inner` | ordered `release_cold` (`🗂️ordered/🦀️.rs` ~line 579) |
| `FL/🌿️vcs/🦀️.rs` (`close_operation_step`, `flow_vcs_step_mutation`) | `Grant` vs `RetainedCloneGrant` | import swapped; close uses `grant.bytes.max(next_close_byte_demand)` as release credit and maps `Failure` to `ClosePending`; advance is now `Result` and maps to `FlowVcsFault::Limit`/`Depth` | neural `insert_controlled` |
| `FA/🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs` (test only) | `{Factory,ChunkyFactory,StalledFactory}: FactoryRetirement` not satisfied | `#[derive(FactoryPayloadRetirement)]` on the two unit structs; hand `FactoryPayloadRetirement` impl for `Factory` (it has a counting `Drop`, so no destructuring derive) | `RootFactory` in the same file; `AtomicBool` impl in `🌱️value/♻️retirement/🏭️factory/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs` (`inverse`) | E0502 `after_order` borrowed while extended | collect `missing` ids first, then `extend` (same semantics: upserted ids are deduped) | n/a (plain borrowck) |
| `W/📐️section|🗺️plan|🧊️world/🎚️config/🧪️tests/🔬️unit/🦀️.rs` (BIM tests, after the window-config macro became a sparse diff by another agent) | `Config: MutationDiff<Config>` / `Diff == Config` | `assert_diff_algebra_between_law::<Config, ConfigDiff>`; plan test applies the snapshot diff to the default via `protocol::apply_diff` and compares the state | `bim_window_config!` sparse arm |

Already fixed by others before my last runs (not touched): `transient_root!` `diff:`/`fields:` form, `semio_framework::io` -> os-kernel `io`, `impl_whole_record_config` removal, `mutation-testing` feature.

## Verification
* BIM graph native + wasm: Finished (above).
* Non-BIM workspaces I touched: root `cargo check --manifest-path Cargo.toml -p semio-framework-os-flow -p semio-framework-artifact-flow-flow --tests` Finished; `cargo check --manifest-path ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/Cargo.toml --tests` Finished.
* NOT run: any `cargo test` (only checks). Behavioural risk: Flow retirement now reports a `Failure` as blocked + `fault`, previously impossible.

## Addendum: hub-bim graph (`cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-bim --tests`, native + wasm32-wasip2: Finished)
`RUN` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run`.

| File | Error | Fix | Followed |
|---|---|---|---|
| `RUN/🧬️schema/📸️snapshot/🦀️.rs` | `RunStatus`/`RunNodeStatus`/`RunTrigger: BorrowedDslField` not satisfied (DslRecord derive now needs the borrowed shape) | `BorrowedDslField` with `BorrowedShape::Enum(&[(label, ordinal)..])` for both enums (labels = the existing `*_variants()` rows); `RunTrigger` gets `BorrowedDslRecord` (kind + three optional text columns, same as `run_trigger_spec`) and `BorrowedDslField` as `Record(borrowed_record::<Self>)` | `MeshAttributeDomain` (`🏗️mesh-engine/🦀️.rs`), `ArtifactRef` (`🗿️artifact-reference/🪆️binding`) |
| `RUN/🧬️schema/🧬️mutations/**` | `Mutations requires an approved semantic verb` for `retract-run-log`/`retract-run-node` ("retract" is not in `APPROVED_VERBS`) | renamed both leaves to the approved verb `remove`: folders (schema + fixtures), `RemoveRunLog`/`RemoveRunNode`, kind/opcode `remove-run-*`, wire op `removeRun*`, record `RemovedRun*`, labels, schema `$id`/title/const, descriptor + aggregate JSON, `🦀️.rs` re-export, tests and `include_str!` paths; internal diff step `RunStep::LogRetract` kept (not a verb). Binary tag unchanged | `remove-element-property` etc. in bim (verb `remove`) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` + `📓️schema-catalog.md` | stale generated entries | handcrafted rename of the two keys/paths/exports and recomputed the two schema sha256 (same raw-file method as the neighbours) | catalog neighbours |
| `🌎️hub/🧩️compositions/🏙️bim/📦️packages/🦀️rust/Cargo.toml` | `plugin_exports!` expands to `semio_framework_async` (not a dependency) | added `semio-framework-async = { workspace = true }` | `🌍️gis` composition |
| `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs` (test only, run crate `--tests`) | `Cow<'static,str>` has no `capacity` (`ValueError.message` is a `Cow` now) | capacity = owned string capacity, 0 for a borrowed literal | `ValueError::literal` semantics |

Also verified: `cargo check --manifest-path Cargo.toml -p semio-framework-artifact-workflow-run --tests` Finished. No `cargo test` run.
