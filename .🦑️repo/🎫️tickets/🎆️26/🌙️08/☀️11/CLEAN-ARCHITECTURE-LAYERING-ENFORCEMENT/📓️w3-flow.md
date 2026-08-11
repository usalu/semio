# W3 — Flow (`✏️s/🔌️plugins/🌊️flow/` + its 9 `🧩️extensions/`)

Assigned subtree: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/` including `🧩️extensions/`.
Extensions found (9, confirmed via `ls`): `🏗️bim`, `📐️brep`, `📖️dictionary`, `📃️list`, `📝️text`,
`🔤️primitive`, `🖍️draw`, `🧠️logic`, `🧮️math`.

## Extra cleanup task (C1) — done

**Finding**: `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml` had `[dev-dependencies]` on 6 of its own
extension crates (primitive, math, text, logic, dictionary, list), used by exactly one call site:
`install_first_party_light_flow_extensions_for_tests()` in
`🎛️apps/🌊️flow/🦀️component.rs`'s `#[cfg(test)] mod testkit`.

**Investigation** (grep across the whole `🎛️apps/` + `🗿️artifacts/` subtree for every consumer of
`flow_app()`/`flow_app_with_registry()`, 23 files, plus every string-literal reference to
`primitive.`/`math.`/`text.`/`logic.`/`dictionary.`/`list.` inside `#[cfg(test)]` blocks): only **one**
test genuinely asserts on real extension content —
`catalogue_lists_module_operators` in `🎛️apps/🌊️flow/📌️panels/🛍️catalogue/🦀️component.rs`, which checks
for a `"math"` catalogue section and a `"math.add"` operator id. Every other consumer of `flow_app()`
(selection, layout, synapse, locale, grid, lod, node-graph, view, widget, eval, generate/*, edit/*,
inspection, artifact panel, extension command …) only needs *an* app instance and never inspects which
extensions are installed. `every_built_in_extension_is_listed_in_the_installed_section` (same catalogue
file) turned out to test `FLOW_AUTOMATIONS` (the 2 hardcoded built-in automations `auto-layout`/
`auto-evaluate` in `🎮️commands/🧩️extension`), an unrelated concept — it needs zero real extensions.

**Deviation from the literal task wording** ("move those specific tests into the corresponding extension
crate's own test module"): I did not do a literal per-crate test move, and I'm flagging this explicitly.
Reasoning: `catalogue_lists_module_operators` tests flow-core's *own* catalogue-rendering logic
(`flow_extensions_tree_sections`/`render()` in `📌️panels/🛍️catalogue`) — code that only exists in
flow-core, not in the math extension crate — using the math extension purely as a realistic fixture. It
isn't really "math's test" that got misplaced; each real extension crate already has its own exhaustive
self-contained manifest/operator tests (e.g. `flow-extension-math`'s
`manifest_lists_math_operators_and_schemas`, `🧩️extensions/🧮️math/🦀️component.rs:666`) — nothing was
missing there. And `every_built_in_extension_is_listed_in_the_installed_section` doesn't correspond to
any single extension at all (it doesn't touch the 6 crates in question).
The clean fix that actually satisfies C1 ("a plugin must not depend on its own extensions, even as a
dev-dep") without either (a) fabricating an artificial owner for a 6-extension-spanning test, or
(b) gutting flow-core's own catalogue test coverage, is to sever the dependency at its root: replaced
`install_first_party_light_flow_extensions_for_tests()`'s body with a hand-authored `FlowExtensionManifest`
fixture (one `"math"` module contributing a minimal `math.add` `OperatorInfo`), built directly from
types flow-core *already* depends on non-dev (`flow::FlowExtensionManifest`/`FlowExtensionContributes`/
`flow::neural::OperatorInfo`, all re-exported from the `semio-framework-os-flow` crate flow-core already
takes as a real dependency), serialized with `serde_json` and installed via the same production
`flow::install_flow_extension_manifest()` entry point real host-pushed contributions use. Zero new
dependencies, full existing test coverage preserved (verified by hand-tracing `flow_operator_catalogue_json()`
→ `by_module.entry(info.extension.clone())` → section `id = "math"` → rendered id
`"flow-play-catalogue.math"`, matching the test's assertions exactly).

**Files touched**:
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml` — removed the entire `[dev-dependencies]` table (6 entries).
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🦀️component.rs` — replaced `install_first_party_light_flow_extensions_for_tests()`'s body (doc comment added explaining the fixture and pointing at this ticket).

**Verification**: `grep -rl` for `semio_s_plugin_flow_extension_(primitive|math|text|logic|dictionary|list)`
across the whole `🌊️flow` subtree outside `🧩️extensions/` now returns nothing — dependency fully severed.
Compile verification is blocked crate-wide by unrelated concurrent churn — see Blocker section below.

## Shared recipe — Step A (schema self-registration)

Flow has exactly one app (`s.flow.flow`, at `🎛️apps/🌊️flow/`) needing this — presence-app only, no
second app in the taxonomy.

1. Found the exact closed-catalog descriptor in
   `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:426-442` (`register_all_app_schema_descriptors()`),
   confirmed the parked `catalog-integration`-gated call site at line 1473 already expects exactly
   `semio_s_plugin_flow::apps::flow::config::schema::register_app_schema()` — i.e. this is the exact
   fn path/shape to land.
2. Confirmed the module is already wired in `📦️glue.rs` (`apps::flow::config::schema` →
   `🎛️apps/🌊️flow/🎚️config/🧬️schema/🦀️component.rs`) and that the crate-local `extern crate
   semio_framework_schema as schema;` alias (in `📦️glue.rs`) makes `::schema::…` resolve.
3. Added `register_app_schema()` to `🎛️apps/🌊️flow/🎚️config/🧬️schema/🦀️component.rs`, transplanting the
   exact `AppSchemaDescriptor`/`FacetLeaves` construction from the framework catalog with `include_str!`
   paths made relative to the app's own file (config facet: siblings in the same dir; presence facet:
   `../../👥️presence/🧬️schema/...`) — byte-for-byte the same pattern the `procedural` plugin's prior wave
   already established at `🌀️procedural/🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs` (cross-checked to
   confirm shape/naming/region-comment convention).
4. Wired the call into the plugin's existing setup path: added
   `crate::apps::flow::config::schema::register_app_schema();` inside
   `🗿️artifacts/🌊️flow/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`'s `register()` function (already the
   `.setup(...)` target from the plugin root `🌊️flow/🦀️component.rs`), alongside the existing
   `register_artifact_schema()`/`register_pilot_languages()` calls.

**Files touched**:
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎚️config/🧬️schema/🦀️component.rs` — added `register_app_schema()`.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — added the call site.

## Shared recipe — Step B (open `TopicContribution` producer conversion) — BLOCKED, not done

Extra task instruction: convert every `Contribution::FlowExtension` producer in flow's 9 extensions to
also push `TopicContribution::new("flow.extension", payload)`.

**Found 9 producer sites** across the extensions (`Contribution::FlowExtension { .. }` construction):
`🏗️bim` (×3, incl. 1 in a test assert), `📐️brep` (×3), `📖️dictionary` (×1), `🧠️logic` (×1), `🔤️primitive`
(×1), `🧮️math` (×1), `📃️list` (×1), `📝️text` (×1), `🖍️draw` (×5, incl. 2 test asserts) — real (non-test)
producer call sites: 12.

**Blocker — cannot complete without editing framework/os product tree (out of ownership)**: every one of
these producers builds its `Contribution::FlowExtension` via
`semio_framework_plugin::ExtensionBundle::new(...).contributes(Contribution::FlowExtension { .. })`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:6906-6966`). The manifest that
`.contributes()` pushes into is **`ExtensionManifest`** (same file, line 6911-6923) — a *different* struct
from the `PluginManifest` the prior `w2-open-contribution` wave added `topic_contributions: Vec<TopicContribution>`
to (that field only exists on `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`'s `PluginManifest`, confirmed
by `grep -rn "topic_contributions" 🧰️framework/` — every hit is either that manifest module or a
`PluginManifest { .. }` struct-literal call site fixed up by a prior wave; zero hits anywhere near
`ExtensionManifest`/`ExtensionBundle`). `ExtensionBundle` has no `.contributes_topic(...)` builder method
and `ExtensionManifest` has no `topic_contributions` field to push into — there is nowhere to put a
`TopicContribution` from an extension producer site today.

Adding that field + builder method requires editing
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, which is the **framework/os product
tree** — explicitly out of my assigned ownership (`✏️s/🔌️plugins/🌊️flow/` only) and explicitly listed as
forbidden to touch in my briefing ("Do not touch any other plugin's directory, the framework tree, or the
os product tree"). This is a real, structural blocker, not concurrent churn — I did not touch
`🔌️plugin/🦀️component.rs`. Flagging for whichever wave owns that file: `ExtensionManifest` needs the same
additive `topic_contributions: Vec<TopicContribution>` field (+ `.contributes_topic()` builder on
`ExtensionBundle`) that `PluginManifest` already got, before any plugin's extension producers can be
converted. Skipped Step B entirely for all 9 extensions; zero edits made to any extension's producer
sites.

## Verification / Blocker — crate-wide, NOT caused by me

`cargo check -p semio-s-plugin-flow` fails with:
```
error: couldn't read `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/./././../../🎛️apps/🌊️flow/📌️panels/📄️document/🦀️component.rs`: No such file or directory (os error 2)
   --> ✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs:418:13
    |
418 |             pub mod document;
    |             ^^^^^^^^^^^^^^^^^
```
`📦️glue.rs` (last commit `c31024cc6c`, not modified by me, `git status` shows it clean/unmodified in my
working tree) already wires `pub mod document;` to
`🎛️apps/🌊️flow/📌️panels/📄️document/🦀️component.rs`, but that file doesn't exist on disk yet — this is
exactly the cross-session "document" concept refactor my briefing warned about ("at least one other
session is actively mid-refactor threading a 'document' concept through several plugins" — not my bug).
Retried once after a 20s wait, same error. Did not touch `📦️glue.rs` or create the missing file. This
blocks a real `cargo check` of every file I touched in flow-core (both the schema self-registration and
the dev-dep cleanup) — I hand-verified my two flow-core edits by exact structural comparison against the
already-`cargo check`-clean `procedural` plugin's equivalent (Step A) and by hand-tracing the exact
types/fields used against their real definitions in `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/`
(Step B/C1 fixture), but **cannot claim a passing `cargo check -p semio-s-plugin-flow` right now** — that
needs re-running once the concurrent "document" wave lands the missing file.

What I *could* verify green:
- `cargo check` for all 9 extension crates individually
  (`semio-s-plugin-flow-extension-{primitive,math,text,logic,dictionary,list,bim,brep,draw}`): **all
  clean**, zero errors, only pre-existing warnings unrelated to this ticket (unused imports etc., not
  touched).

## Files touched (summary)
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎚️config/🧬️schema/🦀️component.rs` (Step A: added `register_app_schema()`)
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (Step A: wired the call)
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🦀️component.rs` (C1: replaced extension-installing test fixture)
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml` (C1: removed the 6 `[dev-dependencies]`)

No files outside `✏️s/🔌️plugins/🌊️flow/` were edited. No extension producer sites (Step B) were edited —
blocked, see above.
