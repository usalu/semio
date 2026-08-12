# W3 Semio Mesh — Subset Conformance Roundtrip

Ticket: `26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`  
Subset: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh`

## Archetype & fidelity

| Field | Value |
|-------|-------|
| Archetype | **owning** |
| `IoFidelityClass` | **Canonical** (DSL wire + pack; no external native mesh format) |
| `subset!` macro | not used |

## Changed files

| File | Change |
|------|--------|
| `…/✳️mesh/🚪️io/🦀️component.rs` | `SemioMeshRoundtrip` + `cube_subset_integrated_roundtrip` in existing `derived_composition::tests` |
| `…/✳️mesh/📚️examples/🧊️cube/` | moved from `✳️any/📚️examples/🧊️cube` (mesh-owned example) |
| `…/✳️mesh/⚙️engine/🦀️component.rs` | mesh wire helpers (`parse_mesh_dsl`, pack encode/decode) |
| `…/🪆️subsets/🔣️component.json` | owning archetype, `examples: ["cube"]`, canonical fidelity |
| `📦️packages/🦀️rust/📦️glue.rs` | `mesh::engine`, `mesh::examples::cube` mounts |
| 13× `…/✳️{typed}/🚪️io/🦀️component.rs` | example path fix `../../✳️any/📚️examples/…` (compile glue) |

## Harness wiring

- **Test:** `cube_subset_integrated_roundtrip`
- **Location:** `✳️mesh/🚪️io/🦀️component.rs` (`derived_composition::tests`)
- **Dialect:** `s.stdio.semio` / `v1` / `mesh`
- **Asset:** cube DSL from `include_str!("../📚️examples/🧊️cube/…")`
- **Geometry primitives:** shared from `✳️any/⚙️engine/🧮️geometry` (not hollow reexport of full mesh body)

## Commands

```bash
export PATH="/usr/bin:/bin:/usr/sbin:/sbin:/Users/ueli/.bun/bin:/opt/homebrew/bin:$PATH"
export RUSTC_WRAPPER=
TICKET=$(ls -d .🦑️repo/🎫️tickets/🎆️26/*/☀️12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS)
export CARGO_TARGET_DIR="$TICKET/🎯️target-w3-stdio"
cargo test -p semio-s-plugin-stdio --lib cube_subset_integrated_roundtrip
```

## Results

| Step | Status |
|------|--------|
| `cargo check -p semio-s-plugin-stdio` | **PASS** |
| `cube_subset_integrated_roundtrip` | **PASS** |
| Log | `scratch-w3-stdio-roundtrip.txt` |

## Gaps

- Canonical fidelity only — no external native mesh IO in roundtrip harness (format bridges tested separately in IO conformance laws).
- `semio::text` conformance tests required `DiffCodec` in scope (resolved — crate compiles).
