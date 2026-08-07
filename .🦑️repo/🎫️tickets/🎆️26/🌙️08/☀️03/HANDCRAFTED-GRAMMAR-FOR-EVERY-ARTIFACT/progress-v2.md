# Progress v2

- P0 bootstrap: contracts + collision + ownership written
- Deleted empty 📡️protocol module tree
- **P2 family kits (2026-08-07):** Rewrote all seven `📖️family-*.grammar.semio` stubs into typed shared vocabularies; extended `family-sheet` with `QUANTITY`, `assign`, `clause-ref`, `eng-node`/`eng-record` (F8 eng on sheet, no new `family-eng` dir). Skipped `📡️family-*.protocol.semio` — see `p2-family-protocol-note.md`. Fixed `include_str!` paths on graph/sheet/catalog/recipe grammar tests; added grammar parse tests on scene/geo/embed. `bun probe-p2-grammars.mjs` structural check **PASSED** (see `🧪probe-p2-grammars-result.txt`). Rust `parse_grammar` probe (`probe-p2-grammars/`) still blocked by unaccepted Xcode SDK license (linker exit 69).
- **W5 fan-out (2026-08-07):** processed=48 pilots_skipped=4 files_written=240 files_missing=0 examples_padded=160

## P4 lowpoly pilot (2026-08-07)

Handcrafted domain-driven specs for `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/`.

### LowpolyOperation variants (from `🔧️op/🦀️component.rs`)
1. ObjectsAdd — keyword `objects-add`
2. ObjectsRemove — keyword `objects-remove`
3. ObjectsMove — keyword `objects-move` (`to-index`)
4. ObjectsPatch — keyword `objects-patch`
5. AddPaintLayer — keyword `add-paint-layer`
6. RemovePaintLayer — keyword `remove-paint-layer`
7. PatchPaintLayer — keyword `patch-paint-layer`
8. PaintStroke — keyword `paint-stroke`
9. SetProjection — keyword `set-projection`

### Done
- Rewrote dsl/op/diff grammars: typed fields only, no catch-all `prop`, no `mesh-json`; structured `mesh { vertices/halfedges/faces }` with VEC3; `use family-scene` + `scene { layer* }`.
- Pack protocol: framing magic `0x894C57504C0D0A1A` (0x89 LWPL 0x0D 0x0A 0x1A); segments Objects/PaintLayers/Projection + Mesh structs.
- Spr protocol: format u8 + ordinal varint + record tags 1..9 matching Operation variants (not generic body-only).
- Wired `COMPONENT_GRAMMAR_SEMIO` / `COMPONENT_PROTOCOL_SEMIO` + paths on dsl/op/diff/pack/spr.
- Wired `register_pilot_languages` for Document/Ops/Diff/Pack/Spr in engine; glue `setup` calls `engine::register()`.
- `default_projection()` now builds a unit box programmatically (derive cannot parse structured mesh yet).
- Examples: structured DSL (~1KB), richer op text, pack 141B / spr 105B placeholders; ticket `seed-lowpoly-examples.mjs`.

### Files changed
- `🗣️dsl/📖️component.grammar.semio`, `🗣️dsl/🦀️component.rs`
- `🔧️op/📖️component.grammar.semio`, `🔧️op/🦀️component.rs`
- `🔺️diff/📖️component.grammar.semio`, `🔺️diff/🦀️component.rs`
- `🎒️pack/📡️component.protocol.semio`, `🎒️pack/🦀️component.rs`
- `📡️spr/📡️component.protocol.semio`, `📡️spr/🦀️component.rs`
- `⚙️engine/🦀️component.rs`, `📦️packages/🦀️rust/📦️glue.rs`
- `📚️examples/♻️reuse/{🗣️dsls,🔧️ops,🎒️packs,📡️sprs}/♻️reuse/*`
- ticket: `seed-lowpoly-examples.mjs`, `mcp-unavailable-lowpoly.txt`, this progress entry

## P3/M4 policy scanners (2026-08-07)

Armed five high-priority breach scanners in root `📜️script.ts` and wired into `policy` export + `VerifyScript.runGate`.

### Functions added
- `policySpecDistinctnessBreaches`
- `policyGenericSpecBreaches`
- `policyDeclaredUseBreaches`
- `policySpecWiringBreaches`
- `policyEmptyExampleBreaches`
- aggregator `policyHandcraftedSpecP3Breaches`

### Exemptions seeded (must shrink to empty by P6)
| Set | Count |
|---|---|
| `POLICY_SPEC_DISTINCTNESS_EXEMPTIONS` | 0 |
| `POLICY_GENERIC_SPEC_EXEMPTIONS` | 0 |
| `POLICY_DECLARED_USE_EXEMPTIONS` | 0 |
| `POLICY_SPEC_WIRING_INCLUDE_EXEMPTIONS` | 220 |
| `POLICY_SPEC_WIRING_REGISTER_EXEMPTIONS` | 44 |
| `POLICY_EMPTY_EXAMPLE_EXEMPTIONS` | 20 |
| **Total** | **284** |

Note: distinctness / generic / declared-use currently find zero live offenders after mid-migration handcrafts; empty Sets stay armed. Wiring + empty-example exemptions list remaining corpus debt. `bun ./📜️script.ts policy` reports 0 `handcrafted-grammar/*` high breaches with these exemptions.

### Verify gate
`VerifyScript.runGate` runs `policyHandcraftedSpecP3Breaches` after the OS exclusive state authority block and fails on any high-priority finding.
