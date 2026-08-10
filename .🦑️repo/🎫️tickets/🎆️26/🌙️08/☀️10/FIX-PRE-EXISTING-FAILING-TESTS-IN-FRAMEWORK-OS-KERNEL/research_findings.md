# 🔬 Test Failure Diagnosis and Subsystem Analysis for `semio-framework-os-kernel`

## Overview
Out of 737 total unit tests in `semio-framework-os-kernel`, 695 pass and 42 fail across 4 main subsystems:
1. `os_dsl` (10 derive tests, 4 family grammar tests, 6 fixture sweep grammar tests, 4 production coverage tests, 1 protocol conformance test, 2 notation/recognizer tests = 27 tests)
2. `os_store` (13 component tests)
3. `os_spr` (2 channel golden-hex corpus tests)

---

## Root Cause Analysis by Subsystem

### 1. `os_dsl::component::tests` (10 failing tests)
- **Error**: `valid envelope_id: InvalidPreamble("envelope id must be plugin.artifact, got derivedoc")`
- **Root Cause**: `DerivedDocument` in `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs` is configured with `#[dsl(extension = "derivedoc")]` without an explicit dotted `id`. `DslDocument` macro sets `__DSL_ENVELOPE_ID` to `"derivedoc"`. `SemioEnvelope::from_envelope_id` requires a dotted `plugin.artifact` identifier (e.g. `"derived.doc"`).
- **Fix**: Update `DerivedDocument`'s derive attribute to `#[dsl(id = "derived.doc", extension = "derivedoc")]`.

### 2. `os_store::component::tests` (13 failing tests)
- **Error**: `Schema("invalid semio preamble: envelope id must be plugin.artifact, got demo")`
- **Root Cause**: `DemoSnapshot` / `DemoDocument` in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` defines `envelope_id` as `"demo"` (undotted). `SemioEnvelope::from_envelope_id` fails when parsing non-dotted envelope IDs.
- **Fix**: Update `DemoSnapshot`'s `envelope_id` implementation to return `"demo.doc"`.

### 3. `os_dsl::family::{geo,graph,scene,sheet}::tests` (4 failing tests)
- **Error**: `family-geo.grammar must parse: TextError { message: "expected a symbol, found LParen", span: TextSpan { line: 3, column: 23, length: 1 }, expected: None }`
- **Root Cause**: The `.grammar.semio` files for `family-geo`, `family-graph`, `family-scene`, and `family-sheet` use parentheses `(A | B)` for grouping inline alternatives. In `dsl_grammar`, parentheses `( )` are reserved exclusively for macro argument lists, while grouping uses braces `{A | B}`.
- **Fix**: Replace `( ... | ... )` grouping syntax in the 4 `.grammar.semio` files with `{ ... | ... }`.

### 4. `os_spr::channel::tests` (2 failing tests)
- **Error**: `assertion \`left == right\` failed: Hello's encoding drifted from its committed golden hex`
- **Root Cause**: `CHANNEL_VERSION` was updated to `5`, so `encode_app_command` emits `0005...` whereas `channel_command_fixture_hex` / `channel_frame_fixture_hex` still pins `0004...`.
- **Fix**: Update `channel_command_fixture_hex` and `channel_frame_fixture_hex` golden hex strings to match `CHANNEL_VERSION = 5` encoding output.

### 5. `os_dsl::fixture_sweep` (11 failing tests)
- **Root Cause**: Handcrafted grammars and production fixtures need alignment with updated grammar parsing rules and schema conventions.
- **Fix**: Update grammar recognizers and fixtures to conform with `dsl_grammar` syntax.
