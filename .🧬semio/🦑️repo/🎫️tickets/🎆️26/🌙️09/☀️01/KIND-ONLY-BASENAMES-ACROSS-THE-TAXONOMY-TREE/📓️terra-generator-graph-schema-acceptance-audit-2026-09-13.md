# Generator Graph and Schema Extraction Acceptance Audit

Date: 2026-09-13  
Status: **accepted.**

## Current Ownership

The graph package router at `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📜️script.ts` is a thin registration layer importing only the execution owner. The prior generator monolith has been replaced by five anonymous leaves with one-way composition:

- `🛂️manifest/📥️admission/🟦️.ts` discovers and parses admitted manifest inputs;
- `🛂️manifest/📇️catalog/🟦️.ts` reads and validates the output catalog;
- `🛂️manifest/📽️projection/🟦️.ts` derives the render plan;
- `🛂️manifest/📤️publication/🟦️.ts` inventories and materializes declared outputs; and
- `🛂️manifest/🏃️execution/🟦️.ts` composes generate, preview, check, test, and lint commands.

The direct import graph is admission/catalog → projection → publication/execution, with execution also owning the command-runtime edge. No router imports a projection or publication implementation directly. The authored Rust bridge remains separate at `🛂️manifest/🔄️value-conversion/🦀️.rs`, included by `🛂️manifest/🦀️.rs:15`; it is not a generated leaf.

Schema likewise now has four domain owners below `🏷️entity-kinds`: `📥️source/🟦️.ts`, `📽️projection/🟦️.ts`, `📋️plan/🟦️.ts`, and `🏃️execution/🟦️.ts`. Source validates and fingerprints the catalog, projection emits TS/Rust/Go, plan assigns the three outputs, and execution owns preview/generate/check. The package router imports execution only. There is no remaining live import of either former `🏭️generator/🟦️.ts` monolith.

## Source Data and Consumers

The catalog source remains `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/🔣️.json`; generated TS and Rust remain sibling leaves under `🧬️schema/🤖️generated/🏷️entity-kinds/`.

The Rust component maintains three distinct edges:

- `⚛️component/🦀️.rs:69` compiles generated Rust through `include!`;
- `⚛️component/🦀️.rs:864` treats that generated Rust as a schema-export leaf through `include_str!`; and
- `⚛️component/🦀️.rs:874` reads the JSON catalog as source data.

The component unit control validates catalog rows and compares every row with the Rust projection. The TypeScript control imports generated TS, compares Ajv and owned-parser decisions, preserves all 58 ordered entries and the two FIRST-WINS shadowed emoji cases, and verifies shared provenance. The Go projection is a further runtime consumer; root’s separate package-route prerequisite passed over 58 source rows, covering ordered identifiers/emojis, normalized emoji lookup, and FIRST-WINS only.

Graph’s source-data authority is `🛂️manifest/📇️outputs.json`: nine manifest identities plus shared registry/index/type leaves. Its portable suite covers catalog shape and hostile paths, no-output failure paths, stale pruning, output symlink refusal, and malformed/nonmanifest/identity controls. The bridge test at `🛂️manifest/🧪️tests/🔬️unit/🦀️.rs:88` iterates all 21 generated enum families, asserting every declared wire’s ToValue string, FromValue inverse, uniqueness, and rejection of unknown and non-string values. It is a behavior control, not a source-body hash.

## Repaired Findings and Independent Evidence

The initial current-source audit found a graph source-admission failure: a linked plugin ancestor was followed and a chmod 000 directory was reduced to an empty discovery result. This is repaired in `📥️admission/🟦️.ts`: every root/area component uses `lstat`, symbolic and wrong-kind components reject, `ENOENT` is the only intentional absence, and inspection/read failures retain their source context before rendering can plan or write.

I independently ran the current focused portable controls:

- `bun test ./🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️suite/🟦️.ts --test-name-pattern 'manifest admission refuses linked inputs|unreadable admitted directory'`: 2 passing tests, 6 assertions, 1.71 s.
- `bun test ./🧰️framework/🔨️modules/🧬️schema/🧪️tests/🏷️entity-kinds/🟦️.ts`: 7 passing tests, 60 assertions, 0.733 s. This exercised Ajv and owned parsing, invalid cases, 58 entries, FIRST-WINS, provenance, and generator input closure.

I also created and removed a private native fixture as uid 501. A `plugins` symlink failed with `admitted graph manifest ancestor is a symbolic link: plugins`; a chmod 000 `plugins/blocked` directory failed with `cannot read admitted graph manifest directory plugins/blocked`. In both calls to `renderGraphArtifacts`, the output directory was absent. Permissions were restored before deletion.

The original cache-input finding is repaired. Graph’s default named input now includes every five owner, the suite, fixture, catalog schema, manifest data, Rust bridge through the graph Rust input, and direct external command-runtime/cache/discovery/taxonomy owners. Schema includes all four owners, catalog data, parser, and direct command-runtime/cache owners. The graph portable route asserts the exact input closure.

## Registered Evidence

The executor ran the real isolated registered routes with `--skip-nx-cache`:

- `@semio-tech/framework-graph:check-generated` exited 0; Nx reported 25.1 s with cache skipped. Its target ran the complete portable suite at 7 passing tests and 98 assertions, then confirmed all nine generated manifest outputs were fresh.
- `@semio-tech/framework-schema:check` exited 0; Nx reported 3.4 s with cache skipped and confirmed the 58-entry catalog at source SHA prefix `f237` was fresh.

The reported wall times include separate slow Nx graph/bootstrap latency; the target outcomes above are the relevant execution evidence. Root’s independent native Go selected consumer also passed against the moved catalog JSON over 58 rows, with the limited ordered-ID/emoji, normalized lookup, and FIRST-WINS scope stated above.

## Bridge-Native Evidence

The executor then ran the exact bridge law in a ticket-private `CARGO_TARGET_DIR` with two jobs and existing toolchain caches:

`cargo test -p semio-framework-graph generated_manifest_enum_value_mappings_are_exact_and_total -- --nocapture`

It exited 0. The private-target build finished in 1m38s; the selected law passed once with zero failures and 184 filtered tests in 0.01s. Cargo metadata confirmed the private target coordinate; the repository cache wrapper reported its content-addressed final binary through the standard cache. The run executed all 21 structured manifest family mappings. Its only emitted warning was an unrelated existing `AtomicU64` deprecation warning.

This closes the previously unresolved bridge gate. The older shared-lock cancellations remain non-evidence and are not used for acceptance.

## Acceptance

**Accepted.** The graph and schema generator extraction has semantic anonymous owners, registration-only package routers, closed source-data/producer/consumer inputs, current no-follow failure behavior, portable parser/projection controls, isolated registered freshness results, a bounded native Go consumer prerequisite, and the exact native Rust bridge law. No source-body hash is treated as behavior evidence.
