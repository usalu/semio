# Catalog Shell Generation Edges

## Existing Domain-Neutral Ownership

The taxonomy already declares `generatorContracts.shell-typegen`: owned project `@semio-tech/framework-os-shell-rs:typegen`, exact ignored output root `🧰️framework/🔨️modules/🐚️shell/🤖️generated`, and its existing preview producer. Owner-discovery `readBuildGeneratedCompileOutputV1` (line 141) resolves physical imported descendants against this contract and authenticates the producer project manifest. Its selected compile-demand traversal records per-target generated dependencies (lines 216–227), includes type imports only for compile-demand roles, and refuses a generator consuming its own output. Project assembly line 431 and `applyBuildGeneratedTaskDependenciesV1` add deduplicated producer dependencies and the producer default Source inputs. Script-inputs also projects generators discovered through explicitly declared TypeScript program closures. Existing source-demand neutral tests cover cold/warm generated imports and demanded package types.

Consequently, missing authored `dependsOn` alone does not prove a missing effective edge. The earlier prerequisites report described authored edges, not a verified inferred graph deficiency. No extra hard-coded dependency should be added until the effective selected graph is observed.

## Consumer Placement

`@semio-tech/framework:typecheck` invokes actual admitted TypeScript compilation of its package tsconfig. `@semio-tech/s-dev-composition:typecheck` compiles the actual `✏️s/tsconfig.json` through the repository typecheck owner. These compile closures consume the shell type projection and are the correct targets for inferred shell-typegen edges. If inference genuinely omits the edge, repair the existing generated compile-demand mechanism or its declared compiler closure, rather than adding shell generation to all catalog tasks.

`test-catalog-exports` is a runtime/freshness test. Its declared entries and values do not need shell type-only imports at runtime. The separate ad-hoc semantic audit used the entire SpecificCLI script as a root, so its missing shell types do not establish that this runtime export leaf needs an extra Rust prerequisite. Catalog `generate` owns catalog declarations, not shell types. Do not attach shell generation to it.

`@semio-tech/framework-os-shell:test` routes Vitest, without a separate authored typecheck target; its erased type-only imports do not alone require the projection for runtime execution. Conversely, Rust `schema-check` explicitly reads the rendered mirror, so making that actual cross-language check depend on typegen is justified if zero-touch standalone schema-check is intended. It must not make typegen depend on schema-check, because the exporter already validates its registry internally.

## Producer Prerequisite Found

Actual shell Cargo.toml contains no `[features]` declaration and no `typegen` feature, while the existing typegen and schema-check launch helpers pass `--features typegen`. That is a concrete potential first producer failure; wait for Root owned 73042 actual result rather than launching a duplicate. Schema registry exporter tests are cfg(test), not gated by that feature in the inspected files. Do not invent a feature or stub types to conceal this mismatch.

## Supported Observation Limit

No guaranteed cheap graph-free public query returning these targets effective generatedDependencies was established in Source. An Nx graph/show or repo run query can enter admission, start a daemon or regenerate the graph and therefore does not meet this audit no-launch boundary. A read-only existing authenticated selected graph receipt, if Root already has one, can be decoded with registered Python to inspect the two target dependsOn rows. Without such a receipt, report the graph inference mechanism and pending runtime evidence; do not claim a missing effective edge or suggest a raw private SQLite query.

## Browser Dependency Row Registration Snapshot

The new caching suite row is `browser-dependencies`, 15000ms, typecheck none, all darwin/linux/win32, explicit empty capabilities. Its test source reads authored Source and exercises the provisioner using injected mock ports; Ajv and the owned schema validator independently validate neutral results. No real package install or external process is used by this leaf. At review time the caching project test aggregate has 186 unique dependencies, but no generated target command contains `browser-dependencies` and there is no `test-browser-dependencies` target. This is an unsynchronized registration boundary unless the canonical config writer is still in progress. A published canonical route should be verified before claiming aggregate coverage. No Source, tests, Native, process or lease operations were changed or launched by this audit.

## Exact Canonical Aggregate Writer

The supported command is `SEMIO_LANGUAGE=en repo ./📜️script.ts generate config`, followed by `SEMIO_LANGUAGE=en repo ./📜️script.ts generate config --check`. `GenerateScript.generateConfig` calls the derived-config owner `writeDerivedConfig`, which replaces only the canonical caching aggregate target using `deriveCanonicalTestAggregateV1`. It also owns nextest configuration; check mode checks nextest, Dependabot and aggregate freshness. No writer was launched by this audit.

This writer deliberately does not synthesize independent test leaves. `canonicalTestRoutesV1` derives the browser route from its closed suite row; `resolveCanonicalTestTargetsV1` matches literal executable entry, cwd and exact arguments against an existing authored leaf and refuses zero or duplicate matches. With the browser leaf absent, generation must fail closed with route-targets rather than invent a target. The independent leaf is authored project configuration, while its membership in aggregate dependsOn is generated. Preserve that distinction; add the correctly owned leaf through its existing project declaration mechanism, then use the canonical writer rather than editing aggregate dependencies by hand. Existing aggregate route tests cover missing, duplicate, undeclared, contracts and self-reference refusal.
