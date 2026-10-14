# Dashboard Current Graph Publication

The dashboard registry and inventory now read the same current neutral publication as the graph producer: `.cache/b/g/m/canonical.json`. They no longer read the historical `.🧬semio/🦑️repo/⚡️cache/build/graph.json`. The enforcing store's `.cache/b/g/m/graph.json` remains its charge receipt and is not a graph candidate.

## Shared Declaration

`📚️library/🏗️build/🧬️schema/📣️publication.json` declares the graph partition, canonical filename, authority filename and snapshot namespace. The common TypeScript graph API consumes its graph and authority filenames. The new standard-library Rust helper in `📚️library/🏗️build/🕸️graph/🦀️rust/🦀️.rs` combines the same declaration with the current neutral partition policy and native image mount declaration. It exposes an owned `&str` API and uses the dashboard's existing serde_json dependency. Registry and inventory call this helper. There is no historical fallback or migration.

The registry fixture, registry/inventory/command-tree Rust tests, TypeScript registry and coverage oracles, execution absence test and dashboard README use the current publication. The fixture includes a separate charge-receipt path so the independent oracle can distinguish the two files.

## Executed Evidence

- Before the correction, the retained `dashboard/📜️script.ts red` diagnostic compared the actual Rust consumer's constant with the common public `buildGraphPathV1` result and failed. Its output is in generated `dashboard-publication-red.txt`.
- The retained harness then compiled the actual new Rust helper through Cargo offline in a small synthetic crate, with all manifest, target and output data under ticket generated storage. It did not copy implementation sources. The newly compiled helper printed `.cache/b/g/m/canonical.json`, matching the TypeScript helper. Output: generated `dashboard-publication-rust.txt`.
- The permanent Bun test validates the neutral declaration using strict Ajv, compares both path implementations with the language-neutral fixture, places a charge receipt beside the actual graph, verifies the independent dashboard oracle still discovers the graph-only artifact without problems, and checks both Rust consumers use the shared helper. Focused run: **1 test, 9 assertions, passed**.
- The broader source-only registry selection passed **3 tests, 31 assertions, 892 ms**, including the new publication law, independent authored-catalog oracle and Ajv declaration acceptance/refusal law. Output: generated `dashboard-publication-source-suite.txt`.
- Scoped whitespace validation passed. A dashboard search leaves the historical path text only in a negative assertion, with no runtime reader.

## Limits

The full dashboard crate depends on the UI terminal stack and numerous repository domain crates. This correction did not run or claim a fresh full dashboard binary build. Previously installed binaries cannot qualify the new Rust source. The added Rust registry runtime fixture test is permanent but was not executed as a full crate test in this gate. The freshly compiled shared helper plus current source-only fixture/oracle checks provide the bounded evidence reported here. Final canonical publication and public exact-pin planner/Stdio acceptance remain owned by the coordinated final gate.

## File Inventory

The exact source and retained-input inventory is recorded in `📥️bazel-files.json`. All diagnostic generated output remains under this ticket's `🗑️generated`; it is not retained input.
