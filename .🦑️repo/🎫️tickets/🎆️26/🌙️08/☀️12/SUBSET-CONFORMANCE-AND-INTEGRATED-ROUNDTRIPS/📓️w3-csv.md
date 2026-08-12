# W3 CSV Reference (`stdio.csv` / `rfc4180` / `✳️any`)

Completed: 2026-08-12. Ticket: `26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`.

## Scope

Owning subset `✳️any` for `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/`.

## Moves

| From | To |
|------|-----|
| `🏅️standards/🔖️rfc4180/⚙️engine/` | `🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/⚙️engine/` |
| `📚️examples/🎬️demo/` | `🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/📚️examples/🎬️demo/` |

## Manifest (`🪆️subsets/🔣️component.json`)

- `*`: `archetype: owning`, `examples: ["demo"]`, `ioFidelity: exact`, import/export `txt/utf-8/*`.

## Facets (pre-existing, verified present)

| Facet | Status |
|-------|--------|
| Schema (snapshot/diff/mutations) | present under `✳️any/🧬️schema/` |
| Inferences | present (`🧾outline`, text/binary leaves) |
| IO import/export | present (`txt/utf-8/✳️any` deserializer + serializer) |
| Engine | relocated to subset |

## Glue (`📦️packages/🦀️rust/📦️glue.rs`)

- Engine mounted under `standards::v_rfc4180::subsets::any::engine` (+ standard-level `pub use` shim).
- Artifact shims (`csv::engine`, `csv::examples`) target subset paths.
- No UCAS roster edits.

## Verification

```bash
export CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS/🎯️target-w3-stdio"
cargo test -p semio-s-plugin-stdio csv::
```

**Result:** blocked — pre-existing `semio-framework-plugin` `E0499` at `🔌️plugin/🦀️component.rs:5790` (documented in `📓️w1-macro.md`). No new errors observed in stdio glue paths before framework-plugin failure.

## Changed files

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/⚙️engine/` (moved)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/📚️examples/` (moved)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/🔣️component.json`
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`
