# 🖨️ W2 — raster tests, examples and boot document

Ticket 26/09/05/RASTER-PLUGIN-END-TO-END, implementer W2 (second run, resumed after the 01:30 Opus
rate-limit kill and the ~02:50 host reboot). Regions: test mounts, the `t040` rename, the two
`📚️examples` leaves, the boot snapshot. `create_raster_app`'s classification/proofs/factory belong to
W1 and `🚪️io/**` + the per-format leaves + the TS package belong to W3; nothing below touches them
except one doc-comment correction called out in §6.

## 1. Mounts — 232/232, nothing dangles

`python3 🐍️w2-check-mounts.py` from the repo root:

```
#[path] attributes: 232 (self-markers ".": 107, real targets: 125)
include_str!/include_bytes!: 111
dangling: 0
EXIT=0
```

The pre-existing figure was `232 (self 109, real 123)` / `109` includes. My edits removed two
`#[path = "."]` self-markers (the collapsed double mount, §3) and added two real targets plus one
command mount, and the two newly mounted example test leaves each carry an `include_str!` — hence
`107 + 125 = 232` and `111` includes. **Zero dangling in both runs.**

A companion sweep for taxonomy `.rs` files that exist on disk but are mounted **nowhere** now returns
exactly one file:

- `🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🖨️mutate-raster-1/🦀️.rs`

This is **correct and deliberate**, not a gap: it is the cross-language differential SUBJECT adapter
driven by the repo-test harness alongside its `🐍️.py` oracle and `🥒️.feature`, never by
`cargo test --lib`. `🧱️block`'s twin
(`🗿️artifacts/◻️2d/…/🧪️tests/🧱️mutate-block-2d-1/🦀️.rs`) is likewise absent from block's crate root
and from its `Cargo.toml` (`[[test]]` sections: none in either crate). Leave it unmounted.

## 2. `t040` rename — complete

`grep -rn 't040'` repo-wide, excluding `node_modules`, `target*`, `.git` and the ticket tree:
**zero hits.** The new directory name resolves everywhere it is referenced:

- `🧬️schema/🧬️mutations/🎚️change-layer-adjustment-kind/🧪️tests/📈️switches-the-tone-d5816a/` — the
  fixture directory itself (six `🔣️.json` leaves + `🦀️.rs`), mounted at
  `📦️packages/🦀️rust/🦀️.rs:218`.
- `🔮️oracle/🔣️.json:78` — `"directoryName": "📈️switches-the-tone-d5816a"`. Machine-checked:
  `'t040' in oracle == False`, `'switches-the-tone-d5816a' in oracle == True`.
- `🧪️tests/🖨️mutate-raster-1/🥒️.feature:139` — the `change-layer-adjustment-kind` row.

The `🐍️.py` oracle addresses cases by mutation keyword, not by directory name, so it needed no edit;
no TS file referenced the old name.

## 3. Examples — registered, and mounted exactly once

### 3a. Registration mechanism

`PluginManifest.examples` is populated in **one** place today
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:25691`, `register_app_factory`, which copies
`App.examples` onto the manifest and stamps each row's `app_id`). Two builder channels reach it:

1. **New declaration tree** — `.declare_artifact(…)` → `SubsetDeclaration.examples`, harvested at
   `🔌️plugin/🦀️.rs:28218` **for the editor surface only**. This is what `🧱️block` and `🧩️puzzle` use.
2. **Old channel** — `PluginBuilder::editor_with_examples::<E>(def, Vec<ExampleSource>)`
   (`🔌️plugin/🏗️builder/🦀️.rs:466`), the documented twin of the plain `.editor::<E>(def)` raster was
   calling; `.editor` is literally `self.editor_app::<E>(def, Vec::new())` (`:458`).

Raster is still on the old channel (`.artifact(declaration())` + `.editor::<>()` + `.viewer::<>()`),
so channel 2 is the mechanism that applies. It is in production use on `🌀️procedural`'s two editors
(`✏️s/🔌️plugins/🌀️procedural/🦀️.rs:309` and `:313`). Migrating raster to `.declare_artifact` would
mean authoring a whole `SubsetDeclaration` root that raster does not have — out of scope here and not
required to populate the manifest.

Change, `✏️s/🔌️plugins/🖨️raster/🦀️.rs`:

- `:27` new `fn examples() -> Vec<ExampleSource>` in a `//#region 📚️Examples` block, returning
  `vec![crate::examples::art_raster_demo::source()]`.
- `:55` `.editor::<RasterPlayApp>(create_raster_app())` → `.editor_with_examples::<RasterPlayApp>(create_raster_app(), examples())`.
- `:45` doc paragraph on `plugin()` recording why.

The consumer is the react shell's `NavbarExampleSelect/🟦️.tsx`, fed by
`activePluginManifest.examples`, which stays hidden while that list is empty — which is exactly why
raster's switcher was invisible.

### 3b. Decision: `demo-session` is mounted and tested, but NOT in the switcher

`✏️editor/📚️examples/🎬️demo-session` is a `.cmd.semio` **command replay**, not a document carrier.
`setActiveExample`'s entire vocabulary is "resolve this id to a registered example document and
replace the open document with it" (`🧬️schema/🦀️.rs:438` `raster_example_document`), so a replay
script has nothing to resolve to. Both precedents agree:

- `🧩️puzzle` — mounts all three `demo_session_{2d,3d,5d}` leaves **and their Rust tests**
  (`📦️packages/🦀️rust/🦀️.rs:2977-2994`) but registers only artifact-level fixtures in each subset's
  `examples()` (e.g. `◻️2d/…/🦀️.rs:39` → `nakagin_capsule_tower`, `concrete_forest`).
- `🧱️block` — ships no editor-level session example directory at all; `◻️2d/…/🦀️.rs:25` registers only
  the two `hexagonal-cut-concrete-forest-*` fixtures.

So: **wire, do not delete.** The leaf keeps its module mount, gains its Rust test mount (§3c) and
stays out of `examples()`. Rationale is recorded in the doc comment on `plugin()`'s `examples()`.

Its `🖼️assets/🎮️.cmd.semio` was a two-line stub (`semio raster.raster.cmd v1` / `action=demo`) that
named no real command. Rewritten to the shape `🧩️puzzle`'s own carriers use, with five **real**
raster action ids and their **real** payload fields, each verified against the handler struct:

| step | payload struct | file |
| --- | --- | --- |
| `setActiveExample exampleId=demo` | `SetActiveExample { example_id }` | `🎮️commands/🎬️set-active-example/🦀️.rs:13` |
| `setCompositeViewport width= height=` | `SetCompositeViewport { width, height }` | `🎮️commands/🖥️set-composite-viewport/🦀️.rs` |
| `setActiveUtility utilityId=paintBrush` | `SetActiveUtility { utility_id }` | `🎮️commands/🧰️set-active-utility/🦀️.rs:11` |
| `setBrushSize value=24` | `SetBrushSize { value: f64 }` | `🎮️commands/📏️set-brush-size/🦀️.rs` |
| `setBrushOpacity value=0.85` | `SetBrushOpacity { value: f64 }` | `🎮️commands/🌫️set-brush-opacity/🦀️.rs` |

`paintBrush` is a real utility id declared at `✏️editor/🦀️.rs` `.utility(raster_utility("paintBrush", …))`.
No camera step: ticket 26/07/31 keeps camera out of the document, and the Semio-logo demo is the
canonical example (`📓️explore-raster-history-and-prior-tickets.md` §5 do-not-undo) — both respected.

### 3c. Single mount + example test mounts

`📦️packages/🦀️rust/🦀️.rs` mounted `📚️examples/🎬️demo/🦀️.rs` **twice** — once as
`artifacts::raster::examples::demo` (`:532-540`, what `🧬️schema` addresses) and once as
`examples::art_raster_demo` (`:717`, what a registration would address). Two module paths for one
file means the Semio-logo `include_str!` expanded twice and `demo::ID` existed as two distinct items.
Collapsed to one real mount plus a facet alias, which is the shape `🧩️puzzle` documents in its own
subset root ("puzzle2d's `🦀️.rs` mounts examples at the CRATE ROOT … not under `artifacts` at all"):

- `:538` `pub mod examples { pub use crate::examples::art_raster_demo as demo; }`

`🧬️schema`'s `crate::artifacts::raster::examples::demo::{PRIMARY_TEXT,ID}` keeps resolving unchanged.

Both example leaves' Rust tests were unmounted and therefore never ran. Mounted following puzzle's
`#[cfg(test)] #[path = …] mod <slug>_tests;` pattern (`🧩️puzzle/📦️packages/🦀️rust/🦀️.rs:2937-2944`):

- `:719` `mod app_raster_demo_session_tests;`
- `:724` `mod art_raster_demo_tests;`

## 4. Boot document

`🧬️schema/🦀️.rs:432` `default_raster_document()` (landed in the killed first run) parses the
committed `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` Semio-logo carrier through the artifact's own
text codec, falling back to `empty_raster_document()` only if it does not parse — the same shape as
block2d's `default_block2d_snapshot` (`🧱️block/…/🧬️schema/🦀️.rs:259`). `empty_raster_document()` is
untouched and stays the tests' blank slate. Two surfaces now boot on it:

- `✏️editor/🦀️.rs:755` — `RasterPlayApp::initial_snapshot()`, was `empty_raster_document()`.
- `👁️viewer/🦀️.rs:56` — `RasterViewer::initial_snapshot()`, was `empty_raster_document()`. Changed for
  the same reason and by the same precedent (`🧱️block/…/👁️viewer/🦀️.rs:56`): the composite and
  navigator otherwise render a blank canvas.

Its guard tests (`🧬️schema/🦀️.rs:485-507`) assert the boot document is `semio-demo` /
`"Semio Raster Demo"` with a `backdrop` pixel layer plus a brighten adjustment layer, and that
`raster_example_document` resolves the registered id and nothing else. They cannot execute yet — see §5.

## 5. `setActiveExample` — completed by W1 concurrently

I mounted the command file (it existed on disk from the killed run but was mounted **nowhere**, so
its handler was dead code):

- `📦️packages/🦀️rust/🦀️.rs:597` `pub mod set_active_example;` under `editor::raster::commands`,
  in the region's slug-alphabetical position ahead of `set_active_utility`.

While I was editing, **W1 landed the entire remaining chain in the same file** — `app_commands!` row
(`✏️editor/🦀️.rs:201`), flat import (`:207`), `RASTER_RETAINED_TOOL_IDS` (`:237`),
`RASTER_PUBLICATION_CONTRACTS` `Artifact` lane (`:274`), `.mutation("setActiveExample", …)` (`:955`)
and `.action_interactive_job("setActiveExample", Migrated)` (`:1026`). W1 also added the same crate-root
mount at the same moment, producing a momentary duplicate `pub mod set_active_example;` (would have
been E0428) — **I removed my duplicate; one mount stands.** No builder-chain hunk from me was needed
in the end. Wire path is now complete and matches block2d exactly:

`ShellHost/🟦️.tsx:6439` `onAction({ action: "setActiveExample", args: { exampleId } })` → manifest
action → `RasterCommand::SetActiveExample { example_id }` (snake↔camel handled by the value derive,
same as `setActiveUtility`/`utilityId`) → `handle` → `replace_document_operations` → real undoable
`DeleteLayer`/`RemoveLayerAsset`/`AddLayerAsset`/`CreateLayer` mutations. An unregistered id is a
no-op, not a fault.

## 6. One doc-comment correction inside W1's function

`✏️editor/🦀️.rs:915` carried a stale "🚧️ SDK GAP #4" note claiming `.editor::<E>(def)` discards
`App.examples` with **no** replacement mechanism, which is exactly why raster's examples went
unregistered for so long. It is now false. Rewritten in place to record that
`editor_with_examples` is the replacement and that examples are a plugin-root registration, not a
builder-chain one. **Doc comment only — no code inside `create_raster_app` was touched.**

## 7. Verification

### 7a. `bun test` — both example leaves, PASS

```
$ bun test './✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts' \
           './✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts'
bun test v1.3.14 (0d9b296a)

 2 pass
 0 fail
 2 expect() calls
Ran 2 tests across 2 files. [56.00ms]
```

Re-run after the `🎮️.cmd.semio` rewrite: identical, `2 pass / 0 fail`.

Gotcha for whoever repeats this: `bun test` treats a bare emoji path as a **name filter**, not a path
(`🟦️.ts` has no `.test`/`.spec` in it), and silently reports "did not match any test files" after a
10 s scan of 311 008 files. The leading `./` is what makes it a path.

### 7b. `cargo check -p semio-s-plugin-raster --lib` — 28 errors, **none mine**

```
$ CARGO_TARGET_DIR=target-s-e2e RUSTC_WRAPPER="" cargo check -p semio-s-plugin-raster --lib --message-format short
…
error: could not compile `semio-s-plugin-raster` (lib) due to 28 previous errors; 82 warnings emitted
```

The **identical** 28-error set was produced by a check whose compilation started *before* my first
edit and by a second check run *after* every edit above. My changes therefore introduce zero errors
and fix none of these. Verbatim (owner-root prefix elided to `…/`):

```
…/✏️editor/🎚️config/🦀️.rs:183:1: error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`: missing `DESCRIPTORS`, `descriptor` in implementation
…/✏️editor/👥️presence/🦀️.rs:94:1: error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`: missing `DESCRIPTORS`, `descriptor` in implementation
…/👁️viewer/🦀️.rs:73:111: error[E0053]: method `render` has an incompatible type for trait: expected `Result<ComponentTree, ...>`, found `UiNode`
…/✏️editor/📌️panels/🗿️artifact/🦀️.rs:43:56: error[E0277]: the trait bound `semio_framework_plugin::plugin_app_close_prelude::Label: From<semio_framework_plugin::Label>` is not satisfied: the trait `From<semio_framework_plugin::Label>` is not implemented for `semio_framework_plugin::plugin_app_close_prelude::Label`
…/✏️editor/📌️panels/🗿️artifact/🦀️.rs:50:10: error[E0609]: no field `base` on type `BuiltNode`: unknown field
…/✏️editor/📌️panels/🗿️artifact/🦀️.rs:64:90: error[E0277]: the trait bound `semio_framework_plugin::plugin_app_close_prelude::Label: From<LabelText>` is not satisfied: the trait `From<LabelText>` is not implemented for `semio_framework_plugin::plugin_app_close_prelude::Label`
…/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🖌️brush/🦀️.rs:36:28: error[E0308]: mismatched types: expected `ActionDescriptor`, found `Result<(ActionId, Option<UiValue>), ...>`
…/👁️viewer/🎭️modes/👁️view/🪟️windows/🧭️navigator/🦀️.rs:48:5: error[E0308]: mismatched types: expected `UiNode`, found `Result<BuiltNode, PluginAssemblyError>`
…/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️composite/🦀️.rs:37:5: error[E0308]: mismatched types: expected `UiNode`, found `Result<BuiltNode, PluginAssemblyError>`
…/✏️editor/📌️panels/🗿️artifact/🦀️.rs:74:81: error[E0308]: mismatched types: expected `Label`, found `semio_framework_plugin::Label`
…/✏️editor/🦀️.rs:821:54: error[E0308]: mismatched types: expected `Result<ComponentTree, ...>`, found `UiNode`
…/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🖌️brush/🦀️.rs:50:28: error[E0308]: mismatched types: expected `ActionDescriptor`, found `Result<(ActionId, Option<UiValue>), ...>`
…/✏️editor/📌️panels/🔍️inspection/🦀️.rs:32:56: error[E0277]: the trait bound `semio_framework_plugin::plugin_app_close_prelude::Label: From<LabelText>` is not satisfied: the trait `From<LabelText>` is not implemented for `semio_framework_plugin::plugin_app_close_prelude::Label`
…/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🧽️eraser/🦀️.rs:37:28: error[E0308]: mismatched types: expected `ActionDescriptor`, found `Result<(ActionId, Option<UiValue>), ...>`
…/✏️editor/📌️panels/🔍️inspection/🦀️.rs:33:55: error[E0277]: the trait bound `semio_framework_plugin::plugin_app_close_prelude::Label: From<LabelText>` is not satisfied: the trait `From<LabelText>` is not implemented for `semio_framework_plugin::plugin_app_close_prelude::Label`
…/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🧽️eraser/🦀️.rs:51:28: error[E0308]: mismatched types: expected `ActionDescriptor`, found `Result<(ActionId, Option<UiValue>), ...>`
…/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:25:50: error[E0277]: the trait bound `semio_framework_plugin::plugin_app_close_prelude::Label: From<LabelText>` is not satisfied: the trait `From<LabelText>` is not implemented for `semio_framework_plugin::plugin_app_close_prelude::Label`
…/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:26:50: error[E0277]: the trait bound `semio_framework_plugin::plugin_app_close_prelude::Label: From<LabelText>` is not satisfied: the trait `From<LabelText>` is not implemented for `semio_framework_plugin::plugin_app_close_prelude::Label`
…/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:27:55: error[E0277]: the trait bound `semio_framework_plugin::plugin_app_close_prelude::Label: From<LabelText>` is not satisfied: the trait `From<LabelText>` is not implemented for `semio_framework_plugin::plugin_app_close_prelude::Label`
…/✏️editor/📌️panels/🔍️inspection/🦀️.rs:31:52: error[E0271]: type mismatch resolving `<[BuiltNode; 2] as IntoIterator>::Item == Result<BuiltNode, PluginAssemblyError>`: expected `Result<BuiltNode, PluginAssemblyError>`, found `BuiltNode`
…/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:24:52: error[E0271]: type mismatch resolving `<[BuiltNode; 3] as IntoIterator>::Item == Result<BuiltNode, PluginAssemblyError>`: expected `Result<BuiltNode, PluginAssemblyError>`, found `BuiltNode`
…/✏️editor/📌️panels/🔍️inspection/🦀️.rs:35:136: error[E0277]: the trait bound `semio_framework_plugin::plugin_app_close_prelude::Label: From<&str>` is not satisfied: the trait `From<&str>` is not implemented for `semio_framework_plugin::plugin_app_close_prelude::Label`
…/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:29:112: error[E0277]: the trait bound `semio_framework_plugin::plugin_app_close_prelude::Label: From<LabelText>` is not satisfied: the trait `From<LabelText>` is not implemented for `semio_framework_plugin::plugin_app_close_prelude::Label`
…/✏️editor/📌️panels/🎭️masks/🦀️.rs:24:60: error[E0277]: the trait bound `semio_framework_plugin::plugin_app_close_prelude::Label: From<semio_framework_plugin::Label>` is not satisfied: the trait `From<semio_framework_plugin::Label>` is not implemented for `semio_framework_plugin::plugin_app_close_prelude::Label`
…/✏️editor/📌️panels/🎭️masks/🦀️.rs:50:110: error[E0277]: the trait bound `semio_framework_plugin::plugin_app_close_prelude::Label: From<LabelText>` is not satisfied: the trait `From<LabelText>` is not implemented for `semio_framework_plugin::plugin_app_close_prelude::Label`
…/✏️editor/📌️panels/🎭️masks/🦀️.rs:50:132: error[E0277]: the trait bound `semio_framework_plugin::plugin_app_close_prelude::Label: From<LabelText>` is not satisfied: the trait `From<LabelText>` is not implemented for `semio_framework_plugin::plugin_app_close_prelude::Label`
…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3480:85: error[E0277]: `?` couldn't convert the error to `std::string::String`: the trait `From<dsl::Fault>` is not implemented for `std::string::String`
…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3988:20: error[E0061]: this method takes 1 argument but 0 arguments were supplied
```

Three clusters, **all foreign to this ticket's three implementers**:

1. **24 errors — a framework UI element-builder migration in flight.** `UiNode` → `BuiltNode`,
   `render` now returning `Result<ComponentTree, …>`, `BuiltNode` losing its `base` field, action
   builders returning `Result<(ActionId, Option<UiValue>), …>` instead of `ActionDescriptor`, and a
   `Label` type split where `semio_framework_plugin::Label` and
   `semio_framework_plugin::plugin_app_close_prelude::Label` are now two different types with no
   `From` between them. Hits every panel, both composite option leaves, both viewer windows and both
   root `render` impls. This is a peer session's repo-wide change, not raster's own code
   (cf. `Concurrent Cargo Workspace Churn`); raster cannot be fixed independently of it.
2. **2 errors — `ArtifactConfig`/presence traits gained `DESCRIPTORS` + `descriptor`.** Same peer
   migration class; `🎚️config/🦀️.rs:183` and `👥️presence/🦀️.rs:94` need the two new items.
3. **2 errors — `🧬️schema/🧬️mutations/💾️binary/🦀️.rs`** (`?` on `dsl::Fault` into `String`; a method
   that now takes one argument). Codec-side, adjacent to W3's serializer work but in the schema
   binary leaf.

**Consequence:** `cargo test -p semio-s-plugin-raster --lib` cannot run — the lib does not compile,
so the 12 fixture-test mounts, the two newly mounted example test mounts and the boot-document guard
tests in `🧬️schema/🦀️.rs:485-507` are all still unexecuted. Positively, the check does prove the
mount graph: no `E0583` (missing file), no `E0428` (duplicate module), no `E0432` on
`crate::examples::art_raster_demo` or the `artifacts::raster::examples::demo` alias, so §3a/§3c/§5
are structurally sound and `editor_with_examples` type-checked against `Vec<ExampleSource>`.

## 8. Files touched

```
✏️s/🔌️plugins/🖨️raster/🦀️.rs
✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/🦀️.rs
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🖼️assets/🎮️.cmd.semio
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/📓️w2-raster-tests-examples-boot.md
```

`✏️editor/🦀️.rs` carries only two W2 hunks — `initial_snapshot` at `:751-755` and the doc rewrite at
`:915`; every other change in that file this session is W1's.

## 9. Open, for the coordinator

1. **Blocking:** the 28 foreign errors in §7b. Until the UI-builder / `Label` / `DESCRIPTORS`
   migration lands (or raster's panels and windows are ported to it), nothing in this crate compiles
   or tests. Worth confirming which peer session owns it before anyone ports raster by hand — a port
   against a half-landed API will need doing twice.
2. Once green, run `cargo test -p semio-s-plugin-raster --lib` and check the boot-document guards
   (`default_document_boots_on_the_semio_demo_carrier`,
   `only_a_registered_example_id_resolves_to_a_document`) plus the two newly mounted example tests.
3. The owner-root `🔣️.json` / `🛂️.descriptor.semio` pair still needs regenerating via `describe`;
   `PluginManifest.examples` is non-empty for the first time, so the descriptor will change.
