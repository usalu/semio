# End-to-end follow-through (2026-08-06)

## User mandate
- Everything must run end to end.
- Grammars ≠ protocols.
- Grammars = text (`🗣️dsl`, `🔧️op`; also `🔺️diff`).
- Protocols = binary (`🎒️pack`, `📡️spr`).
- Subagents: Cursor Grok 4.5 + Composer 2.5 only (non-fast).

## Pre-agent audit
- Facet placement: 156 grammars on text facets, 104 protocols on binary facets; 0 misplacements; 0 wrong dialects.
- Pack vs spr bodies already distinct (`samePackSpr=0`).
- Gap: `LanguageSpec` has only `grammar`/`grammar_path` — protocols not registered.
- Gap: few/no plugin `include_str!` of facet `.semio` into Rust registration → specs exist on disk but are not executed in-process.
- Gap: `verify_protocol_bytes` is SPK-magic shallow; must branch pack `frame` vs spr `record`.
- Host linker/Xcode still blocks full `cargo test` binaries.

## Parallel agents
1. Wire protocol LanguageSpec (Grok)
2. E2E dialect conformance harness (Composer)
3. Differentiate grammar/protocol bodies (Composer)

## Done when
- Dialect sweep exits 0.
- Pilots register grammar+protocol and tests call Recognizer + verify_protocol_bytes.
- Evidence files in this ticket folder.

## Dialect sweep (parent)

- Script: `🔧️e2e-semio-dialect-sweep.mjs`
- Result: **ok** — 156 grammars (text) + 104 protocols (binary), 0 misplacements.
- Fixed 22 HOT protocols missing `start frame` / `start record`.
- Remaining e2e: LanguageSpec protocol fields, pilot `include_str!`, Recognizer/verify_protocol_bytes in Rust tests (agents in flight).

## Parent follow-up
- Pilot agent had no edits; wired 25 include_str constants + dialect tests.
- LanguageSpec protocol fields + Pack/Spr roles.
- dsl re-exports dsl_grammar; tests use ::dsl:: to avoid artifact module shadowing.
- verify_protocol_bytes branches pack frame vs spr record.

## Differentiate pass confirmed
- Re-ran dialect sweep + `🔧️e2e-differentiate-specs.mjs`: 156/156 grammars, 104/104 protocols, 0 rewrites needed.

## Conformance harness confirmed
- Bun sweep exit 0 (156 grammar / 104 protocol).
- cargo check -p semio-framework-os-kernel-dsl-grammar OK on this host.
- Tests present in dsl_grammar + fixture-sweep as listed in 🧪e2e-conformance-evidence.md.
- cargo test still blocked by Xcode license.
