# W3 XML Reference (`stdio.xml` / `1.0` / `✳️any` + `✳️valid`)

Completed: 2026-08-12. Ticket: `26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`.

## Scope

- Owning `✳️any` under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/`.
- Derived `✳️valid` — TypeScript conformance mirror + negative example.

## Moves (`✳️any`)

| From | To |
|------|-----|
| `🏅️standards/🔖️1.0/⚙️engine/` | `🏅️standards/🔖️1.0/🪆️subsets/✳️any/⚙️engine/` |
| `📚️examples/🎬️demo/` | `🏅️standards/🔖️1.0/🪆️subsets/✳️any/📚️examples/🎬️demo/` |

## Manifest (`🪆️subsets/🔣️component.json`)

- `*`: `archetype: owning`, `examples: ["demo"]`, `ioFidelity: exact`, import/export `txt/utf-8/*`.
- `valid`: `archetype: derived`, `derivesFrom: "*"`, `hardCodes` for doctype-missing + root-name-mismatch, `negativeExamples: ["no-doctype"]`.

## `✳️valid` derived improvements

### TypeScript mirror

`🪆️subsets/✳️valid/🧬️schema/🟦️component.ts` — full `checkValidConformance()` mirroring Rust `derived_analysis::check_valid_conformance` (doctype presence, declared/actual root name, standalone+external subset soft check, always-on advisory). Inline `import.meta.vitest` tests for conforming/missing-doctype/mismatch/standalone cases.

### Negative example

- `🪆️subsets/✳️valid/📚️examples/🚫️no-doctype/`
- Asset: `🖼️assets/broken.xml` — well-formed `<root/>` without `<!DOCTYPE>`.
- Rust leaf asserts `stdio.xml.valid.doctype-missing` hard code.

### Extended Rust tests

`🪆️subsets/✳️valid/🚪️io/🦀️component.rs` — `negative_no_doctype_example_fails_compose_with_declared_hard_code` uses vendored negative asset via glue module `valid::examples::no_doctype`.

## Glue (`📦️packages/🦀️rust/📦️glue.rs`)

- `v1_0::subsets::any` — engine + examples mounts.
- `v1_0::subsets::valid::examples::no_doctype` mount added.
- Standard-level `engine` re-export; artifact shims updated.

Phantom `🏅️标准` tree **not** deleted (W5).

## Verification

```bash
export CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS/🎯️target-w3-stdio"
cargo test -p semio-s-plugin-stdio xml::standards::v1_0::subsets::valid
cargo test -p semio-s-plugin-stdio xml::
```

**Result:** blocked — pre-existing `semio-framework-plugin` `E0499` at `🔌️plugin/🦀️component.rs:5790`.

TypeScript mirror tests run via Vitest when stdio TS project is included in workspace test matrix (not executed in this worker due to Rust compile gate).

## Changed files

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/⚙️engine/` (moved)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/📚️examples/` (moved)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/🔣️component.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/📚️examples/🚫️no-doctype/` (new)
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`
