# E2E semio dialect conformance evidence

**Ticket:** `2026/08/03/HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT`  
**Date:** 2026-08-06

## Bun sweep (`🔧️e2e-semio-dialect-sweep.mjs`)

| Metric | Count |
|--------|------:|
| Grammar on text facets (`📖️component.grammar.semio`) | 156 |
| Protocol on binary facets (`📡️component.protocol.semio`) | 104 |
| Placement / dialect / shape failures | **0** |

**Exit code:** 0  
**Artifact:** `🧪e2e-dialect-sweep.json` (manifest paths + counts)

## Facet file fixes this pass

No placement or dialect repairs required — inventory already conformed. (Prior W4bcd finish touched 1 writer diff grammar via `handcraft-w4bcd-finish.mjs` before this sweep.)

## Rust conformance harness

### `semio-framework-os-kernel-dsl-grammar` (`dsl_grammar`)

| Test | Purpose |
|------|---------|
| `repo_plugin_semio_specs_parse_with_expected_dialect` | Walk all plugin facet specs; `parse_grammar` + `SemioDialect::Grammar` / `::Protocol` |
| `ticket_e2e_dialect_sweep_manifest_matches_repo_inventory` | Manifest in ticket JSON matches live inventory; `failures == 0` |
| `writer_dsl_grammar_recognizes_shipped_fixture_tokens` | `Recognizer` on writer DSL body (fixture minus envelope line) |
| `handcrafted_dag_pack_protocol_spec_parses_as_protocol` | Pilot parse `dag.pack` (`start frame`) |
| `handcrafted_dag_spr_protocol_spec_parses_as_protocol` | Pilot parse `dag.spr` (`start record`) |

**Parser / verifier changes:** protocol directive lines (`version`, `schema`, `framing`, `field`, …) skipped during parse; `verify_protocol_bytes` branches on `start` (`frame` → SPK magic + 32-byte header; `record` → non-empty bytes only).

### `semio-framework-os-kernel-dsl-fixture-sweep` (dev test module)

| Test | Purpose |
|------|---------|
| `handcrafted_dag_pack_bytes_verify_against_pack_protocol_spec` | `DagDocument` pack encode + `verify_protocol_bytes` |
| `handcrafted_dag_spr_bytes_verify_against_spr_protocol_spec` | `DagOperation` spr encode + `verify_protocol_bytes` |
| `handcrafted_note_pack_bytes_verify_against_pack_protocol_spec` | Note pack encode + verify |
| `handcrafted_fem2d_pack_bytes_verify_against_pack_protocol_spec` | Fem2d default pack encode + verify |

## Cargo execution (this host)

| Command | Result |
|---------|--------|
| `cargo check -p semio-framework-os-kernel-dsl-grammar` | **OK** |
| `cargo test -p semio-framework-os-kernel-dsl-grammar …` | **Blocked** — Xcode SDK license not accepted (linker exit 69) |
| `cargo test` fixture-sweep | Not run (same linker constraint; crate compiles in check graph when workspace member) |

## Evidence paths

- `🔧️e2e-semio-dialect-sweep.mjs`
- `🧪e2e-dialect-sweep.json`
- `🧪e2e-conformance-evidence.md` (this file)
- `🧰️framework/…/dsl_grammar/📦️lib.rs` — sweep + recognizer tests
- `🧰️framework/…/dsl/🧪️fixture-sweep/📦️lib.rs` — pack/spr verify tests

## Summary

**Pass:** 260 facet specs inventoried, 0 sweep failures.  
**Fail:** 0 placement/dialect issues; Rust **test link** not executed on macOS host pending Xcode license.
