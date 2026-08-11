# Wave 3 — Batch B: draw, energy, fem, forms, layout

Agent scope: `✏️s/🔌️plugins/{🖍️draw,🔋️energy,🏗️fem,📋️forms,📏️layout}` (whole subtrees).
No plugin in this batch has a `🧩️extensions/` subdirectory (confirmed via `find` before starting) — audit note in the assignment holds.

## Step A — Schema self-registration

Transplanted each app's `AppSchemaDescriptor` construction out of the framework's closed
`register_all_app_schema_descriptors()` (`🧰️framework/🔨️modules/🧬️schema/🦀️component.rs`) into a new
`register_app_schema()` fn inside the app's own `🎚️config/🧬️schema/🦀️component.rs`, following the exact
pattern already used by `🌊️flow`'s `s.flow.flow` (only reference implementation found in the repo before
this batch). Framework's closed catalog left untouched, as instructed.

| Plugin | App(s) | Descriptor id | File edited | Call site wired |
|---|---|---|---|---|
| 🖍️draw | draw | `s.draw.draw` | `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🎚️config/🧬️schema/🦀️component.rs` | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/⚙️engine/🦀️component.rs::register()` |
| 🔋️energy | — none | — | — | — |
| 🏗️fem | fem2d | `s.fem.2d` | `✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs::register()` |
| 🏗️fem | fem3d | `s.fem.3d` | `✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs::register()` |
| 📋️forms | forms | `s.forms.forms` | `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🎚️config/🧬️schema/🦀️component.rs` | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/⚙️engine/🦀️component.rs::register()` |
| 📏️layout | layout | `s.layout.layout` | `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎚️config/🧬️schema/🦀️component.rs` | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/⚙️engine/🦀️component.rs::register()` |

**🔋️energy**: `🎛️apps/🦀️component.rs` is an empty facet stub — the plugin registers zero document apps
(`plugin()` in energy's root only wires artifacts/engines, no `.register_document_app::<...>()` calls) and
has no id in framework's closed app-schema catalog either. Step A is a genuine no-op here, as anticipated
by the assignment.

Each new `register_app_schema()` call was added immediately after the existing `register_artifact_schema()`
call inside that app's owning artifact's `⚙️engine::register()` (the function already reached from the
plugin root's `Plugin::builder(...).setup(...)`), matching the wiring pattern used by every already-open
plugin (flow, process/3d, cad, sequence, animate/present, procedural, imperative, sourcing/curate, puzzle,
playbook).

## Step B — Open contribution producer conversion

`grep -rn "Contribution::" ✏️s/🔌️plugins/{🖍️draw,🔋️energy,🏗️fem,📋️forms,📏️layout}/` found constructions
in exactly one plugin: **forms** (`✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🦀️component.rs` lines 446 and 612,
plus consumer-side `match` arms at lines 94/97/170/173).

Verified both construction sites are **not real producers**:
- Both live inside `#[cfg(test)] mod testkit` / `#[cfg(test)] mod tests` — they are test fixtures that
  simulate what an *external* extension plugin (`"forms-module-procedural"`) would send, used to exercise
  forms's own consumer path (`render_extension_question`).
- Forms's Cargo.toml `[package.metadata.semio]` declares `consumes = ["forms.questionKind"]` and has no
  `contributes` entry at all — forms is a pure **consumer** of `Contribution::FormsQuestionKind`, never a
  producer, confirmed against the real production producer pattern seen elsewhere in the repo (a
  `.contributes(Contribution::X { .. })` call chained onto `Plugin::builder` inside a `🧩️extensions/`
  subdirectory, e.g. `🌊️flow/🧩️extensions/🖍️draw`, `📖️playbook/🧩️extensions/🌀️procedural`,
  `🏭️process/🧩️extensions/🔩️metal`). None of forms/draw/energy/fem/layout have a `🧩️extensions/` dir.
- Recipe step B.3 explicitly says not to touch consumers — matching on `Contribution::FormsQuestionKind`
  inside `render_extension_question`/`contribution_label` is a consumer, left untouched.

Cargo.toml `[package.metadata.semio]` checked for all five plugins — none declare a `contributes` entry,
confirming zero real Step-B producer sites across this whole batch, including forms.

**Result: Step B is a no-op for all five plugins in this batch.** The per-plugin note in the assignment
("forms's producers construct `Contribution::FormsQuestionKind` — convert each") does not hold up under
inspection: those are test-fixture constructions of what a *different* plugin's contribution payload looks
like, not forms's own manifest declaration. Reported here rather than force a change onto test-only code
that has no `PluginManifest`/`topic_contributions` vec to push alongside (the constructions build bare
`Vec<ProgramContributionEntry>` test fixtures, a shape with no parallel open-registry field).

## Verification — `cargo check`

```
cargo check -p semio-s-plugin-draw -p semio-s-plugin-energy -p semio-s-plugin-fem -p semio-s-plugin-forms -p semio-s-plugin-layout
```

None of my edited files (the five `🎚️config/🧬️schema/🦀️component.rs` files, the five artifact `⚙️engine`
`register()` call sites) appear in any error — confirmed by grepping the full check output for those exact
paths (zero hits). All errors present are pre-existing/concurrent and unrelated to this batch's edits:

- **🖍️draw, 📋️forms, 📏️layout** (1 error each): `couldn't read .../🎛️apps/<app>/.../📄️document/🦀️component.rs`
  — `glue.rs` already declares `pub mod document;` pointing at a file that doesn't exist yet. This is the
  concurrent "document" concept threading refactor called out in my task instructions (not my bug — did not
  touch, noted only).
- **🔋️energy** (5 errors), **🏗️fem** (16 errors): all in `semio-s-plugin-stdio`'s IO serializer/deserializer
  glue for the JSON/CSV/MD document formats (`JsonSnapshot`/`CsvSnapshot`/`MdSnapshot` field-shape
  mismatches — `headers`/`rows`/`body` vs `records`/`blocks`, `serde_json::Value` vs stdio's own
  `JsonValue`). These are entirely inside `🗄️stdio`'s artifact IO subset code
  (`🗿️artifacts/.../🚪️io/{📤️export,📥️import}/...`), a plugin outside my assigned subtree, being reshaped
  by another concurrent session (`semio-s-plugin-stdio` shows in `Checking` right before these errors and
  itself compiled with only warnings — the errors surface downstream, in fem/energy's own
  IO-subset-generated code that calls into stdio's changed shapes). Not caused by, or fixable within, my
  assigned files.

None of the above blocks Step A or Step B work: every file I touched compiles cleanly on its own merits: no
error is anchored inside any file I edited.

## Files touched

- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🎚️config/🧬️schema/🦀️component.rs` — added `register_app_schema()`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — wired the call
- `✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs` — added `register_app_schema()`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — wired the call
- `✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs` — added `register_app_schema()`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — wired the call
- `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🎚️config/🧬️schema/🦀️component.rs` — added `register_app_schema()`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — wired the call
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎚️config/🧬️schema/🦀️component.rs` — added `register_app_schema()`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — wired the call
- `✏️s/🔌️plugins/🔋️energy/` — no files touched (Step A and Step B both genuine no-ops)

No files were touched outside this batch's five plugin subtrees. Did not run the plugin-registry generate
target, did not touch framework's closed schema catalog, did not touch any git state, did not close/reopen
the ticket.
