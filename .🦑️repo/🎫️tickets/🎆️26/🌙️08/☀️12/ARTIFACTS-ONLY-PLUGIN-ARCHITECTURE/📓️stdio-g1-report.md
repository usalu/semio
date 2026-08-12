# stdio g1 — binary, txt, json, xml, csv, md

## Converted (4): json, xml, csv, md

Each gained `pub fn declaration() -> ArtifactDeclaration` at its artifact root
(`🗿️artifacts/<x>/🦀️component.rs`), covering everything the standard-level `⚙️engine::register()`
did: `.schema(...)`, `.inferences([...])`, `.composers(...)` (fully-qualified
`standards::<std>::[subsets::any::]engine::io_registry::entries()` — the real `&'static
[ComposerEntry]`, never the artifact root's own shadowing `io_registry::entries()` wrapper, which
returns `&[&ComposerEntry]`), `.languages(pilot_languages())` (verbatim copy of the engine's
`register_pilot_languages()`, leaked via `OnceLock` since `dsl::LanguageSpec` isn't `const
fn`-constructible), and `.document_codec_bare::<Snapshot, Mutation>(schema)`. json and xml also got
`.subset_validators(...)` — their `✳️i-json`/`✳️valid` subsets each call `register_subset_validator`
from `🚪️io/` (not `⚙️engine/`, so freely readable/referenceable), which is exactly what that field
covers; built fresh per-declaration via `subset_validator_entry_of::<JsonIJsonValidator>()` /
`::<XmlValidValidator>()` rather than reaching into those modules' private `validator_entry()`
OnceLocks — first artifacts in the repo to populate `.subset_validators()`.

Plugin root (`✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`): the four `crate::artifacts::<x>::
engine::register();` lines removed; `.artifact(crate::artifacts::<x>::declaration())` added
immediately after each `.artifact_kind(...)` call, matching `🗒️note`'s adjacency.

No `⚙️engine/` files touched anywhere — old `register()` bodies are left in place, orphaned/uncalled,
same precedent as `🔋️energy`'s report.

## Left imperative (2): binary, txt — genuine gap, not invented around

Both call `register_schema_specs()` (→ `dsl::registry::register_schema_spec`) from their
`⚙️engine::register()`, alongside the covered calls. This is a **third, separate registry** from
`.languages()`/`register_language` (confirmed by reading `dsl::register_language`'s body — a plain
`HashMap<id, LanguageSpec>` insert, no relation to `dsl::registry::register_schema_spec`'s own
`fn() -> RecordSpec` catalog) and has no `ArtifactDeclaration` field. `csv`/`json`/`md`'s own engine
doc comments confirm this is deliberate elsewhere ("unlike json/csv...", "`register_schema_spec` is
deliberately NOT called here") — only binary/txt's `TxtSnapshot`/`TxtDiff` and
`BinarySnapshot`/`BinaryDiff` genuinely derive `dsl::DslRecord`/`dsl::DslDiff`. Per instructions: did
not invent a field, did not drop the call. `crate::artifacts::{binary,txt}::engine::register();`
left untouched in the plugin root. Field that would cover this: a
`schema_specs: &'static [(&'static str, fn() -> os_dsl::schema::RecordSpec)]` +
`.schema_specs(...)` builder method calling `dsl::registry::register_schema_spec` per entry —
not added here, per instructions not to invent one.

## Bug caught and fixed before final check

First declaration draft for md used `standards::v_commonmark::subsets::any::engine::io_registry`
(copying xml/csv's deeper shape) — md's actual glue module path collapses `subsets::any` away
(`standards::v_commonmark::engine`, same shallow shape as binary/txt/json). Caught by the first
`cargo check` (`E0433: cannot find engine in any`), fixed, re-verified.

## Verification

`grep -rn "io_registry::entries"` across json/xml/csv/md: every call fully qualified
(`crate::artifacts::<x>::standards::...::engine::io_registry::entries()`), zero bare calls.

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/🎯️target cargo check -p semio-s-plugin-stdio --all-targets
    Finished `dev` profile [unoptimized] target(s) in 26.16s
```
Exit 0, `grep -c "^error"` → 0 (695 lib warnings / 787 test warnings, all pre-existing
`unnecessary qualification`/`unused import` style, none touching my files). Full log:
`scratch-stdio-g1-check-2.txt` (first attempt with the md bug: `scratch-stdio-g1-check-1.txt`).

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` (shared plugin root — edited only my 4 lines)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🦀️component.rs`

Not touched: `💾️binary`/`📄txt` artifact roots (left imperative, see above), any `⚙️engine/` file,
`🧬️mutations/`, any sibling group's artifact.
