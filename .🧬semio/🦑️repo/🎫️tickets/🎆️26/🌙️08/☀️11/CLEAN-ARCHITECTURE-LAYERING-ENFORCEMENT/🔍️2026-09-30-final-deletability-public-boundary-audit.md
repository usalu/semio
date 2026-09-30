# Final Deletability and Public Boundary Audit

Read-only audit of current source on 2026-09-30. Only this report was written; no Git operations or builds/tests were executed. Read the spatial/CAD ownership, TypeScript removability, framework ownership, and source-direction integration reports and applicable owner instructions.

## Assessment

The TypeScript CAD package entry now exports only general core (`✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🟦️.ts`), supporting the stated TS scope. Dependency-direction independent inventory rejects linked roots and linked sources. The script rootSources selector now includes symbolic links (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts:45`), closing the root-link omission reported by the previous integration audit.

The concurrently added Cargo `toOwnerSegments` matcher correctly unions exact physical target segments with target roles, while preserving from-role classification (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️cargo/🟦️.ts:123`). Its validation rejects empty/duplicate segments, separators, and dot segments. The taxonomy plugin-to-artifact rule includes the physical artifact segment. Thus declaring an artifact-folder target as another allowed role no longer bypasses that rule. This is inspected code, not a claim that the new fixtures ran.

## Remaining Concrete Boundary Violations

Rust CAD remains physically dependent on artifact owners: `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml` declares six artifact dependencies under normal dependencies, starting at line 58. Its `🦀️.rs:22` imports CAD artifact and lines 27–35 publicly reexport its editor/viewer interfaces. Deleting artifacts cannot preserve this Rust plugin crate or its public interface. This is an already documented scoped limit of the TypeScript refactor, not a newly introduced regression. The strengthened physical Cargo rule should report it rather than role metadata hiding it; no exception should be added to make the gate green.

Framework ownership report still records production GIS-specific public types and task-source coupling. The fixture relocation proves literal fixture independence, not complete framework deletion independence. The final completion claim must preserve that distinction until those concrete interfaces and integration commands are relocated.

## Scoped Limits

Physical Cargo enforcement remains conditioned on the source role. An incorrectly declared source plugin role is validated only as a known role; this matcher alone does not prove source role agrees with owner location. This audit did not demonstrate a new live source-role misclassification; it is a limit of metadata policy, not a current regression finding.

Rust scanner nested module and environment ownership work is assigned separately. No final verdict on that work is made here. No expensive shared build or real graph scan was repeated. Current code review therefore supports the narrow enforcement improvement, not a monorepo-wide passing assertion.

## Settled Cargo Selector and Routing Follow-Up

Reviewed settled matcher, portable fixture/schema/tests, script dispatcher, Nx project, package scripts, and launch registration. No implementation mutation or test execution.

Selector schema/runtime agree for the introduced selector: arrays must be nonempty and unique; members must be nonempty strings without either path separator and cannot equal `.` or `..`. Runtime uses exact owner path segments. No selector behavioral regression identified. The test file currently validates valid fixture/policy objects but does not mutate invalid selector cases; malformed selector rejection is code-inspected rather than independently tested. Other policy validation differences (e.g. role-list duplicates accepted by runtime but rejected by schema) predate this selector change.

Portable artifact cases explicitly cover normal renamed dependency, conditional dev dependency, conditional optional normal dependency, two physically artifact-owned targets with non-artifact roles, and a segment-name decoy. All use `artifact_alias`; the resulting violations preserve alias/kind/optional/platform. The independent oracle builds these declarations, parses TOML with @iarna/toml, runs Cargo metadata, and compares the same first-party report. Inventory traverses all three declaration kinds and target tables without filtering disabled features or current platform, so optional and conditional declarations remain subject to enforcement. Root reports 4 passing Cargo tests/204 assertions and 8 passing TS tests/514 assertions; these were not rerun by this audit.

One concrete routing omission remains: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json` contains TS test and Rust source test/lint aliases but lacks `test-cargo-dependency-direction` and `lint-cargo-dependency-direction` aliases. Nx targets and launch commands for both exist and invoke the correct existing 📜️script.ts routes. Add the two analogous `nx run @semio-tech/repo-lib:<target>` package script aliases to complete the stated package routing.

`lint-cargo-dependency-direction` is cache-enabled with explicit Cargo manifest, taxonomy, scanner, fixture/schema, and script inputs. This differs from uncached live TS/Rust scans; root's current direct lint run is authoritative only for that run. No stale-cache failure is demonstrated here. The aggregate dispatcher runs portable contracts followed by live TS, Rust source, and Cargo scans, so a portable passing result alone cannot satisfy the aggregate.

Root live physical Cargo lint failed with 291 strict violations and 0 metadata problems across 3165 local declarations/280 packages. Exact ANSI-free rows and grouped source-owner counts are preserved in [Live Cargo Physical Direction Violation Inventory](🔍️2026-09-30-live-cargo-physical-violations.md). No aggregate architecture passing claim is justified.

## Routing Repair Confirmed

Final package-file read confirms `test-cargo-dependency-direction`, `lint-cargo-dependency-direction`, and `lint-dependency-direction` aliases now invoke their matching repo-lib Nx targets, alongside the three existing test/source aliases. The earlier Cargo package routing omission is repaired and retained above only as audit history. Root reports structural verification passed all six dependency targets with matching Nx-to-📜️script, package-to-Nx, and exactly one launch entry each. This follow-up independently confirms the package-file state; it does not rerun structural checks or tests. No new audit scope was added.

## Final Shared Rust Scanner Review

One concrete static false-green path remains: alternative conditional mounts on an inline module lose every path after the first. In shared discovery `rustPathAttributes` retains all authored direct/cfg_attr alternatives, but `rustPathAttribute` takes `[0]`, and `inspectRustModuleGraphFacts` assigns that single value to its module fact. The compile-reference scanner inline-module branch validates the paths then recurses without emitting their path references. Therefore an inline module with a safe first configuration and forbidden second configuration gives nested leaf path references only the first graph base.

```rust
#[cfg_attr(unix, path = "safe")]
#[cfg_attr(windows, path = "../specific")]
mod mounted {
    #[path = "leaf.rs"] mod child;
}
```

For `general/source.rs`, the second configuration physically mounts `specific/leaf.rs`, but the graph represents `general/safe/leaf.rs` only. `rustSourceDirectionEdges` derives nested #[path] bases exclusively from those contexts, so no forbidden second target is checked. This is a code-proven omitted ownership alternative; it was not runtime-tested in this read-only review. Preserve all conditional mount contexts or reject ambiguous inline mount inputs explicitly to close this path. Exact affected file: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`, functions `inspectRustCompileReferences`, `rustPathAttribute`, `inspectRustModuleGraphFacts`, `inspectRustModuleGraph`.

Other reviewed previously reported paths now have explicit handling: nested module contexts are supplied to edge resolution, direct/cfg_attr path extraction excludes unrelated serde metadata, manifest env/concat references require provenance, generated OUT_DIR paths reject parent traversal, macro dynamic expressions fail closed when unresolved, and rule pathNot exclusions are honored. Root reports 2182 files/4009 references, zero forbidden edges, and one explicitly unsupported proc-macro quote input; this failed-closed unsupported result is not itself a regression or a passing live-lint result.

## Final Framework Fixture and Slider Integration Read

Read current framework ownership report, source tests, native slider parity test, WidgetDescriptor, copied Hub client-contract imports/manifests, and source launch route. No additional actual regression found in this bounded inspection. WidgetDescriptor InputSlider now owns a required `label: String` with no serde/value default; descriptor construction preserves that authored label. Native parity assertions cover label propagation through widget, DAG name, chrome, and descriptor reconstruction, and reject missing labels for Widget and WidgetDescriptor. Source Ajv checks validate the shared schema and reject missing labels. This is inspected assertion coverage, not independently executed native success.

The source-specific launch command invokes framework-rs:test-fixture-ownership with `--args=source`; the dispatcher accepts source and returns before native invocations. Hub's copied integration test uses directory::os_directory through the explicit directory alias (normal and dev dependency resolve the same kernel package), with async/schema/MCP imports matched by authored dependencies. The Hub test Cargo target and script name both select inference_client_contract. Shared corpora remain framework-owned, while GIS-specific integration tests remain Hub-owned. Root reports latest source-only verification 5 tests/29 assertions; native compilation remains pending and no passing native claim is made here.

## Conditional Inline Mount Repair Confirmed

Final targeted code/fixture read confirms the reported false-green path is repaired. `inspectRustCompileReferences` now rejects path-bearing cfg_attr inline modules with `Unsupported Rust compile conditional inline module mount`; fixture unsupported cases cover alternate-owner and conditional/default alternatives. Direct inline mounts emit typed directory path references, including empty current-directory mounts. Edge matching checks both normalized directory target and its trailing-slash form, so ownership rules anchored to directory prefixes also cover exact target-owner roots. The earlier finding above is retained as audit history and is no longer outstanding.

Root reports final portable result 4 tests/91 assertions with 19 rustc compilations, and live 2182 files/4029 references with zero forbidden edges and one unsupported DSL derive quote source. This audit independently confirms repaired code/fixture state only; it did not rerun tests. Unsupported live source still prevents a passing live-lint claim.

## Neutral Catalog Descriptor Correction and Final Native Routes

Read settled schema-owned `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/📦️catalog-package/🔣️.json` and shared MCP source-builders `catalog_contract_descriptor` helper. Witness contains a complete neutral plugin manifest (plugin ID, label, version, apps/examples), execution protocol, contribution, and hashes; it does not read an installed domain descriptor. Helper decodes PackageDescriptor through first-party JSON and independently compares serde_json, then canonical Pack-encodes it. Both changed catalog consumers now use this helper while retaining the lease corpus for lease fields and rebinding package/descriptor digests. No specific-owner include was introduced. No additional concrete regression found in this correction. Full runtime catalog acceptance still depends on the in-progress native run; parsing parity alone is not claimed as full manifest-validation success.

Final permanent native routes now use `runExactCargoLaws` with explicit 28 framework and 6 Hub law suffixes, target-owned build/list stages, exact executable fingerprints, and exact listed native law execution. This supersedes the earlier intermediate budgeted-runner observation. Settled scripts install SIGINT/SIGTERM flag handlers, pass the cancellation callback, report progress, and remove handlers in finally. Exact process helper polls cancellation every 100ms and kills the process tree; timeout/output limits also terminate the tree. Thus the earlier shared-helper cancellation caveat does not describe the final native execution routes. Source-only Bun route continues using its existing bounded runner. No tests/builds were executed in this independent review.
