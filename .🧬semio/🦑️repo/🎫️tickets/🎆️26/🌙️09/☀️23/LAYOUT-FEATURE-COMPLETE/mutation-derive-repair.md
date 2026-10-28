# Mutation Derive Protocol Repair

The focused derive owner is repaired in place. Layout keeps its derived 58-variant enum; no handwritten dispatch, compatibility adapter, runtime dependency or build-system change was introduced.

## Contract And Tests First

Added the portable owner fixture `🧪️tests/🧫️fixtures/❄️cold-dispatch/🔣️.json` and mounted `🧪️tests/❄️cold-dispatch/🦀️.rs` before editing expansion. The cases cover the actual 58-leaf Layout roster with no explicit reference attributes, explicit same-snapshot reference, and a separate retained/native and wire snapshot pair. Native regressions expand with the production function, parse through third-party syn, and inspect the exact associated type, implementation count, each leaf arm and qualified hook, inverse admission, and generated fold-law generic arguments.

## Repair

`🔁️mutations-expansion/🦀️.rs` resolves one effective reference snapshot/diff pair: explicit attributes when supplied, otherwise the declared native snapshot/diff. It builds all cold leaf arms unconditionally and emits exactly one ColdMutation implementation for that pair. diff returns Self::ReferenceDiff; inverse retains the shared admission check; foreign_step_source routes through ColdMutationKind. The generated footprint law now passes all three required arguments (native snapshot, reference snapshot, aggregate) to the authoritative fold helper. Removed the unused alternate law template and duplicate native cold implementation.

## Executed Evidence

`cold-dispatch-check.mjs` reads the actual production quote template and arm templates, instantiates them against the portable cases in memory, and independently parses the result through existing web-tree-sitter Rust. All three cases pass: one implementation each, correct reference/diff, 174 Layout dispatch arms plus 6 arms in each explicit-reference case, qualified direct leaf calls, and one inverse admission check. This is quote-template instantiation and source checking, not execution of Rust proc-macro expansion. The production derive, native test and test mount each parse with zero Rust grammar errors.

The cached repo frontend remains stale (2026-10-13; no current cargo/run command), and repo list terminated with the persisted search-index 268435456-byte limit. Per repository constraints no raw cargo workaround was used. Native compilation, native syn regression execution and runtime behavior remain unverified. This report does not convert syntax/template checks into a native pass.

## Files

- Framework derive `🔁️mutations-expansion/🦀️.rs`.
- Framework derive tests root `🧪️tests/🦀️.rs`.
- New framework derive native regression `🧪️tests/❄️cold-dispatch/🦀️.rs`.
- New portable fixture `🧪️tests/🧫️fixtures/❄️cold-dispatch/🔣️.json`.
- Ticket input checker `cold-dispatch-check.mjs` and this report.
