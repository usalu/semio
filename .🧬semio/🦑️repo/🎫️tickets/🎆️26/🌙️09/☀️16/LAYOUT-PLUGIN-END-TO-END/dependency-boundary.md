# Drawing Dependency Source Boundary

2026-10-28. Independent read-only inventory; no compiler, Cargo graph, build or test launched. Used the canonical readRustSourceClosureV1 directly through repo tools exec --light bun against the model library entry.

## Measured Closure

semio-s-artifact-draw-drawing-model has 35 reachable inputs totaling 239514 bytes. The closure has 2579 active lines without testkit and 2581 with testkit, using macOS ARM64/unix cfg values. Both feature selections reach the same 35 files. These are source-plus-embedded inputs, not merely .rs files; schemas, inference TypeScript/GraphQL/proto/JSON and snapshot codec grammar/protocol/SQL count too.

The effective global sourceBoundary in caching/policy.json is 262144 bytes, 32 files, 4000 active lines. Thus current model exceeds files by 3, while bytes have 22630 headroom and lines have at least 1419 headroom. remainingFiles=0 is an actual file-cardinality refusal before allocation, not memory capacity or a missing source file. The 641-unit figure describes the full graph and does not affect this per-unit quota.

Model Cargo.toml declares role=library but no rust-source-boundary entry. Importantly, native source admission computes each quota as Math.min(global policy, unit boundary), so declaring maxSourceFiles=35 or 64 in this owner cannot raise the global 32 ceiling. Such a manifest-only change would not fix this run.

## Reachable Validation Split

The validation root mounts source.rs, index.rs and program.rs as three private children. Those three files exactly account for the three-file excess over 32. All four validation files are ordinary reachable source, not falsely included tests or stale assets. Read-only git status and diff for the Drawing schema are empty, so this is already present in the checkout baseline rather than an uncommitted layout change.

Other closure groups: root schema; identity; layer factories/tree; fill/rule/sampling; stroke; shape coordinates; affine; vector; font family; receive; snapshot plus lookup, owned/original/equality/original-text; inferences plus topology and four embedded inference resources; binary/text/SQLite snapshot codecs and their three embedded grammars.

## Recommended Owner Action

Do not add an ineffective larger owner declaration or weaken global admission merely to unblock this crate. If retaining the current architecture, consolidating the three private validation child files into their owning validation source would restore the measured 32-file closure without changing semantics or byte ownership; preserve namespace boundaries as inline modules and keep related tests at their existing taxonomy. Alternatively extract a coherent independent crate behind a domain interface, with all Cargo manifests/ownership updated. The latter is broader work. An owner declaration is useful only to tighten the actual effective bounds; it is not an exemption.

After the Rust agent changes the owner structure, rerun this lightweight canonical closure inventory first, then the already authorized layout native check. This report does not claim compile success. Exact count is scoped to current library cfg and with/without testkit; an actual test target may own a different closure and must be admitted independently.

## Consolidation Review

Canonical closure recheck after Rust agent consolidation: 32 files, 239448 bytes, 2582 active lines (2584 with testkit). Original helper bodies checked against HEAD remain verbatim inside private inline source/index/program modules. The three eliminated path attributes explain the 66-byte reduction; inline module wrappers explain the three-line increase. No quota was changed. Library closure now fits all global limits. Native compiler result remains separate.

## Layout Owner Inventory

Canonical library source scan with macOS ARM64/unix cfg, comparing no extra features and testkit:

| Package | Features | Inputs | Bytes | Active Lines | Result |
|---|---|---:|---:|---:|---|
| layout-model | none |31|223545|2553|Fits|
| layout-model |testkit|31|223545|2555|Fits|
| layout-editor |none|15|198981|2478|Fits|
| layout-editor |testkit|19|289256|3372|Bytes exceed by27112|
| layout-editor-commands |none/testkit|26|184821|2697|Fits|
| layout-engine |none/testkit|35|1334828|2200|Files exceed by3; bytes exceed by1072684|
| layout-engine-export |none|21|227666|2683|Fits|
| layout-engine-export |testkit|22|245992|3008|Fits|

Inspected model/editor/commands manifests have library role and no source-boundary override. Global effective limits remain32/262144/4000. The editor testkit mount exposes an entire61272-byte unit test source instead of only shared context, also embedding23304-byte interchange and4988-byte catalogue fixtures. Coherent repair is to separate shared context support from actual tests/fixtures, retaining execution in owned test targets; file consolidation cannot solve byte excess.

The engine closure contains19 Rust sources plus16 font binaries, including every Noto emoji subset and several Latin faces. Font assets dominate the1334828-byte input closure; source concatenation cannot solve that byte excess. Its existing package asset declarations and font installation/loader port provide the domain boundary for removing compile-time font bytes from the engine owner. Preserve native tests via test-owned independent font fixtures or a bounded asset implementation, and keep runtime fonts behind the established interface. Export engine itself needs no boundary rewrite based on these measurements.
