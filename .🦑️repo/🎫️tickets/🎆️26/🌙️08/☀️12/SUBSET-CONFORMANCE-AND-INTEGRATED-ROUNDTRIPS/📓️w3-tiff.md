# W3 TIFF Reference (`stdio.tiff` / `6.0` / `✳️any`)

Completed: 2026-08-12. Ticket: `26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`.

## Scope

Owning subset `✳️any` only — `✳️baseline` derived profile left untouched (W4/later worker).

Path: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/`.

## Moves

| From | To |
|------|-----|
| `🏅️standards/🔖️6.0/⚙️engine/` | `🏅️standards/🔖️6.0/🪆️subsets/✳️any/⚙️engine/` |
| `📚️examples/🎬️demo/` | `🏅️standards/🔖️6.0/🪆️subsets/✳️any/📚️examples/🎬️demo/` |

## Manifest (`🪆️subsets/🔣️component.json`)

- `*`: `archetype: owning`, `examples: ["demo"]`, `ioFidelity: exact`, import/export `binary/raw/*`.
- `baseline`: `archetype: derived`, `derivesFrom: "*"` (unchanged name/description).

## Facets (pre-existing, verified present)

| Facet | Status |
|-------|--------|
| Schema | present |
| Inferences | present (`📐dimensions`, text/binary, outline-style leaves) |
| IO import/export | present (`binary/raw/✳️any`) |
| Engine | relocated (full IFD-chain codec) |

## Glue (`📦️packages/🦀️rust/📦️glue.rs`)

- Engine + examples under `standards::v6_0::subsets::any`.
- Standard-level `engine` re-export + artifact shims updated.

## Verification

```bash
export CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS/🎯️target-w3-stdio"
cargo test -p semio-s-plugin-stdio tiff::
```

**Result:** blocked — same pre-existing `semio-framework-plugin` `E0499` compile failure.

## Changed files

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/⚙️engine/` (moved)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/📚️examples/` (moved)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🔣️component.json`
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`
