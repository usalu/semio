# Verifying the six demonstrator `defaults.exampleId` values

Two earlier explorations disagreed. Here is the hard evidence, traced through the actual
registration code, not assumption.

## The mechanism, precisely

`ShellHost`'s `exampleOptions` (the navbar dropdown, and the only thing gating whether
`dispatchActiveExample`/`resolveBootExampleId` treat an id as "real") is built at
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx:5668-5667`:

```
return (activePluginManifest?.examples ?? [])
  .filter((example) => example.appId === appId)
```

`activePluginManifest.examples` is `PluginManifest.examples: Vec<ExampleDefinition>`
(`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:3579`). It is populated in **exactly one
place**, `register_app_factory`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:24818-24830`):

```rust
pub fn register_app_factory(mut self, mut app: App, factory: ...) -> Self {
    ...
    self.manifest.apps.push(app.definition);
    for mut example in app.examples {
        example.app_id = app_id.clone();
        self.manifest.examples.push(example);
    }
    ...
}
```

`app.examples` (`App { definition, examples: Vec<ExampleDefinition> }`) comes from one of two
builder paths:

- **OLD path** — `PluginBuilder::editor::<E>(def: AppDefinition)`
  (`🔌️plugin/🏗️builder/🦀️component.rs:419-446`): unconditionally
  `let app = App { definition: def.clone(), examples: Vec::new() };` — **always empty**, no
  matter what the app's own `create_*_app()` builder call passed. Confirmed in-repo by the
  authors' own comments at e.g.
  `✏️s/🔌️plugins/🌀️procedural/…/✏️editor/🦀️component.rs:646-651`, verbatim: *"SDK GAP (contract
  §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a bare `AppDefinition` … there is
  no `.example(...)`/`.workflow(...)` on this builder, so the eight `PROCEDURAL_EXAMPLE_*`
  app-level example registrations … are dropped here."* Identical comments exist verbatim for
  cad (`📐️cad/…/✏️editor/🦀️component.rs:2156-2160`), gis2d
  (`🌍️gis/…/🗺️gismap/…/✏️editor/🦀️component.rs:944-947`) and sourcing/curate
  (`🪵️sourcing/…/🗂️curate/…/✏️editor/🦀️component.rs:792-796`).
- **NEW declared-tree path** — `commit_artifact_declarations`
  (`🔌️plugin/🦀️component.rs:27228-27231`): for a `SubsetDeclaration` registered via
  `.declare_artifact(...)`, `examples: if role == Editor { subset.examples... } else { vec![] }`
  — i.e. only apps that adopted `SubsetDeclaration` (design.md rule 2) actually get their
  `📚️examples/🎬️<slug>` facets wired into the manifest.

Which of the six panes' plugins use which path:

| plugin (pane) | registration call | result |
|---|---|---|
| `🌀️procedural` (generator) | `.editor::<Procedural3dPlayApp>(create_procedural3d_app())` — `🌀️procedural/🦀️component.rs:302` | OLD → `examples: Vec::new()` |
| `📐️cad` (koordinator) | `.editor::<CadPlayApp>(create_cad_app())` — `📐️cad/🦀️component.rs:31` | OLD → `examples: Vec::new()` |
| `🧩️puzzle` (aggregator) | `.declare_artifact(crate::artifacts::puzzle3d::artifact())` — `🧩️puzzle/🦀️component.rs:59`, subset at `🧊️3d/…/✳️any/🦀️component.rs:63-71` | NEW → `examples: [nakagin_capsule_tower, concrete_forest]` |
| `🪵️sourcing` (aussuchen) | `.declare_artifact(crate::artifacts::curate::artifact())` — `🪵️sourcing/🦀️component.rs:37`, subset at `🗂️curate/…/✳️any/🦀️component.rs:25-33` | NEW → `examples: [demo]` (**not** `demo-stock`) |
| `🏭️process` (bearbeiten) | `.editor::<Process3dPlayApp>(create_process3d_app())` — `🏭️process/🦀️component.rs:30` | OLD → `examples: Vec::new()` |
| `🌍️gis` (verfolgen) | `.editor::<Gis2dPlayApp>(create_gis2d_app())` — `🌍️gis/🦀️component.rs:39` (this is the editor mounted on the `gismap` artifact) | OLD → `examples: Vec::new()` |

So for **4 of 6 panes** (generator, koordinator, bearbeiten, verfolgen) `exampleOptions` is
**structurally guaranteed empty** — not "the id is missing from a populated list", but the list
itself never has any entries for that app, period.

Crucially, `ShellHost`'s boot-example effect (`🟦️component.tsx:6255-6278`) **short-circuits
when `exampleOptions.length === 0`**:

```
useEffect(() => {
  if (exampleOptions.length === 0 || !session) return;
  ...
  const exampleId = resolveBootExampleId(activeExampleId, exampleOptions, defaults.exampleId);
  ...
  dispatchActiveExample(exampleId);
}, [...]);
```

— so for those four panes `dispatchActiveExample`/`setActiveExample` is **never called at boot
at all**; `brand.ts`'s `defaults.exampleId` is inert dead configuration for them. Whatever the
pane shows on load is exclusively whatever `ArtifactEditor::initial_snapshot()` returns for that
app — a *separate*, hand-written function, coincidentally or intentionally equal to the intended
example in each case (verified below). The exampleId dropdown itself also never renders
(`exampleSelectElement`, `🟦️component.tsx:5680-5682`, returns `null` when `exampleOptions.length
=== 0`).

For aussuchen (sourcing/curate), `exampleOptions` is **non-empty but wrong**: it contains one
entry, `{id: "demo", label: "Demo"}`, never `"demo-stock"`. `resolveBootExampleId("",
[{id:"demo"}], "demo-stock")` (`Shell/🟦️component.tsx:294-301`) falls through both guards
(`activeExampleId` empty, `"demo-stock"` not in options) to `exampleOptions[0]?.id` = `"demo"`.
So at boot the shell actually **dispatches `setActiveExample({exampleId: "demo"})`** — and the
real handler only recognizes `""`, `EMPTY_EXAMPLE_ID`, or `DEMO_STOCK_EXAMPLE_ID`:

```rust
// ✏️s/🔌️plugins/🪵️sourcing/…/✳️any/✏️editor/🎮️commands/📄️set-active-example/🦀️component.rs:21-27
let text = match payload.example_id.as_str() {
    "" | EMPTY_EXAMPLE_ID => crate::artifacts::curate::dsl::EMPTY_CURATION_TEXT,
    DEMO_STOCK_EXAMPLE_ID => crate::artifacts::curate::dsl::DEMO_STOCK_TEXT,
    _ => return Err(Fault::from("sourcing.example.unknown")),
};
```

`"demo"` matches none of these arms → the boot dispatch **faults** (`Fault::from("sourcing.example.unknown")`)
before any mutation is built, so it happens to leave the document untouched (the fault fires
pre-emit) — the pane still shows correct content only because `initial_snapshot()` already
equals `demo-stock` (see below), not because the exampleId mechanism works.

For aggregator (puzzle3d), by contrast, everything lines up: `exampleOptions` really does
contain `"concrete-forest"`, it matches `defaults.exampleId`, `resolveBootExampleId` returns it
unchanged, and the real dispatch fires and works (below). This is the **only one of the six**
where the brand's declared mechanism (options list *and* dispatch) is not dead code.

## Per-pane detail

### 1. generator — `s.procedural.procedural3d@1/*#editor`, `hexagonal-mushroom-column`

1. **Options list**: none — OLD builder path, `examples: Vec::new()` (see table above). The
   `📚️examples/🎬️hexagonal-mushroom-column/🦀️component.rs` facet exists on disk with a real
   `ExampleSource`, but it is registered in nobody's `mod examples` that anything imports
   (`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs:1616-1631`'s `pub mod examples` block
   lists 10 facets and never mentions `hexagonal-mushroom-column`; more importantly that whole
   glue `examples` module is never referenced from anywhere else in the repo — dead code, not the
   real wiring). **Not present in the actual navbar options.**
2. **Content branch**: real. `set_active_example::handle`
   (`✏️s/🔌️plugins/🌀️procedural/…/🎮️commands/🎨️set-active-example/🦀️component.rs:35-48`) calls
   `example_snapshot(&payload.example_id)`, and `PROCEDURAL_EXAMPLE_HEX_COLUMN |
   "demo" => Some(PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT)` is a genuine, dedicated match arm
   (`🧬️schema/🦀️component.rs:313`).
3. **Populates the window**: yes. `default_snapshot()` (`🧬️schema/🦀️component.rs:286-288`)
   *is* `PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT` parsed into a real `FlowFixture`. Test
   `all_bundled_examples_emit_preview_meshes` (`✏️editor/🦀️component.rs:1663-1683`) asserts
   non-empty meshes/instances specifically for `"hexagonal-mushroom-column"`.
4. **Why it still renders correctly despite #1**: `ArtifactEditor::initial_snapshot()` for
   `Procedural3dPlayApp` is `crate::artifacts::procedural3d::schema::default_snapshot()`
   (`✏️editor/🦀️component.rs:214-216`) — i.e. the pane's own first-mount document already **is**
   the hexagonal mushroom column, independent of the (never-fired) `setActiveExample` dispatch.

**Verdict: PROVEN-GOOD** (content renders correctly), but purely via `initial_snapshot()`
coincidence — the brand's `defaults.exampleId`/`setActiveExample` wiring for this pane is dead.

### 2. koordinator — `s.cad.cad@1/*#editor`, `hexagonal-cut-concrete-forest-left`

1. **Options list**: none — OLD builder path (`📐️cad/🦀️component.rs:31`,
   `create_cad_app()`'s own comment at `✏️editor/🦀️component.rs:2153-2160` states the drop
   explicitly).
2. **Content branch / population**: `ArtifactEditor::initial_snapshot()` for `CadPlayApp` is
   `forest_play_scene()` (`✏️editor/🦀️component.rs:1811-1813`), which is
   `forest_play_document(FOREST_LEFT_MODEL_JSON, CAD_EXAMPLE_FOREST_LEFT)`
   (`🧬️schema/💡️inferences/🦀️component.rs:1016-1019`), and
   `CAD_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left"`
   (`🧬️schema/💡️inferences/🦀️component.rs:780`) — an exact string match to the brand's
   `exampleId`. The scene has a populated `shape_model`, real BREP-imported `nodes` (`node-root`
   labelled "Concrete Forest Left"), not a placeholder — proven by the dedicated regression test
   `initial_snapshot_is_cut_concrete_forest_not_placeholder_box`
   (`✏️editor/🦀️component.rs:2675`).

**Verdict: PROVEN-GOOD**, same caveat as generator: reached via `initial_snapshot()`, not via
the (dead, empty-options) `setActiveExample` boot path.

### 3. aggregator — `s.puzzle.puzzle3d@1/*#editor`, `concrete-forest`

1. **Options list**: present. `subset()`'s `examples: examples()`
   (`✏️s/🔌️plugins/🧩️puzzle/…/🧊️3d/…/✳️any/🦀️component.rs:63-71`) is
   `vec![nakagin_capsule_tower::SOURCE.clone(), concrete_forest::SOURCE.clone()]` (line 37),
   registered through the NEW `.declare_artifact(...)` path
   (`🧩️puzzle/🦀️component.rs:59`) → folded into `PluginManifest.examples` for the editor app id.
   `"concrete-forest"` **is** in the wired navbar options, and matches `defaults.exampleId`
   exactly, so `resolveBootExampleId` picks it unmodified and the boot effect actually fires
   `dispatchActiveExample("concrete-forest")`.
2. **Content branch**: real, dedicated match arm —
   `✏️s/🔌️plugins/🧩️puzzle/…/🎮️commands/🛍️set-active-example/🦀️component.rs:9-13`:
   ```rust
   } else if example_id == PUZZLE3D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
       Some(default_fixture())
   }
   ```
   with `PUZZLE3D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest"`
   (`✏️editor/🦀️component.rs:65`) and `default_fixture() -> CONCRETE_FOREST_EXAMPLE_FIXTURE.clone()`
   (`✏️editor/🦀️component.rs:279`).
3. **Populates the window**: yes — same fixture is also this app's `initial_snapshot()`
   (`✏️editor/🦀️component.rs:6455-6458`, `default_fixture()`), asserted non-empty by
   `initial_snapshot_is_the_concrete_forest_fixture`
   (`✏️editor/🦀️component.rs:7866-7869`: `object_count(&app) > 0`).

**Verdict: PROVEN-GOOD** — the only pane of the six where the brand's declared
`exampleId`→options-list→dispatch→content chain is fully intact end to end, not merely masked by
`initial_snapshot()`.

### 4. aussuchen — `s.sourcing.curate@1/*#editor`, `demo-stock`

1. **Options list**: `subset()`'s `examples: examples()`
   (`✏️s/🔌️plugins/🪵️sourcing/…/🗂️curate/…/✳️any/🦀️component.rs:14-16, 25-33`) is
   `vec![crate::artifacts::curate::examples::demo::source()]` — id `"demo"` only
   (`📚️examples/🎬️demo/🦀️component.rs:5`: `pub const ID: &str = "demo";`). **`"demo-stock"` is
   NOT in the wired options list** — only a same-plugin-but-different id, `"demo"`, is.
2. **Content branch**: real, dedicated match arm for `"demo-stock"` DOES exist —
   `✏️editor/🎮️commands/📄️set-active-example/🦀️component.rs:21-27`:
   ```rust
   let text = match payload.example_id.as_str() {
       "" | EMPTY_EXAMPLE_ID => crate::artifacts::curate::dsl::EMPTY_CURATION_TEXT,
       DEMO_STOCK_EXAMPLE_ID => crate::artifacts::curate::dsl::DEMO_STOCK_TEXT,
       _ => return Err(Fault::from("sourcing.example.unknown")),
   };
   ```
   with `DEMO_STOCK_EXAMPLE_ID: &str = "demo-stock"` (`✏️editor/🦀️component.rs:61`). But `"demo"`
   — the ONLY id actually reachable via the options list — is **not** one of the accepted arms,
   so dispatching the pane's own dropdown default faults with `sourcing.example.unknown`.
3. **Populates the window**: yes for `demo-stock` specifically —
   `default_document()` (`🧬️schema/🦀️component.rs:748-751`) parses `DEMO_STOCK_TEXT` into a
   `CurateSnapshot` with a real, validated `demo_stock()` catalogue/stock, and this is also the
   app's `initial_snapshot()` (`✏️editor/🦀️component.rs:663-665`). Confirmed non-empty by
   `initial_document_has_populated_demo_stock`
   (`✏️editor/🎮️commands/📄️set-artifact-json/🦀️component.rs:98`).
4. **What actually happens at boot**: since `exampleOptions = [{id:"demo"}]` is non-empty, the
   boot effect (unlike the four OLD-builder panes) does NOT skip — it computes
   `resolveBootExampleId("", [{id:"demo"}], "demo-stock") = "demo"` (not in options → falls back
   to `exampleOptions[0]`) and dispatches `setActiveExample({exampleId:"demo"})`, which **faults**
   per the match arm above. The fault fires before any `Emit` is constructed, so the document is
   left as whatever `initial_snapshot()` already set it to — `demo-stock` content — so the pane
   still visually shows the right thing, but only because the erroring dispatch is a no-op, not
   because the wiring works.

**Verdict: UNCERTAIN / PARTIALLY-BROKEN.** Content-wise the pane does show real `demo-stock`
geometry (PROVEN-GOOD in terms of rendered outcome), but the declared mechanism this ticket asks
about — options list containing the exampleId, then a working dispatch — is broken two ways at
once (wrong id in the list, and that wrong id faults the real handler). This is not a passive
"dead but harmless" case like the four OLD-builder panes; it is an active runtime fault on every
boot of this pane, masked only by the fault occurring pre-mutation.

### 5. bearbeiten — `s.process.process3d@1/*#editor`, `timber-beam-joinery`

1. **Options list**: none — OLD builder path (`🏭️process/🦀️component.rs:30`; drop comment at
   `✏️editor/🦀️component.rs:1349-1353`).
2. **Content branch / population**: `initial_snapshot()` for `Process3dPlayApp` is
   `crate::artifacts::process3d::schema::default_document()`
   (`✏️editor/🦀️component.rs:996-998`), which is
   `Process3dSnapshot::parse_dsl(TIMBER_EXAMPLE_DSL).unwrap_or_default()`
   (`🧬️schema/🦀️component.rs:326-328`), and `TIMBER_EXAMPLE_DSL =
   PROCESS_3D_TIMBER_EXAMPLE_TEXT` — matching `PROCESS3D_EXAMPLE_TIMBER: &str =
   "timber-beam-joinery"` (`✏️editor/🦀️component.rs:79`). Non-empty, proven by
   `default_document_parses_timber_example`
   (`🧬️schema/🦀️component.rs:954-958`: `assert!(!document.steps.child_id.is_empty())`).
   The dedicated `setActiveExample` handler
   (`✏️editor/🎮️commands/📄️artifact/🦀️component.rs:52-64`) also has a real fallback arm
   (`_ => default_document()`) that would apply if ever dispatched, though it never is (see #1).

**Verdict: PROVEN-GOOD**, same pattern as generator/koordinator: correct content via
`initial_snapshot()`, `setActiveExample`/options wiring dead.

### 6. verfolgen — `s.gis.gismap@1/*#editor`, `reuse-map`

1. **Options list**: none — OLD builder path. The editor mounted on the `gismap` artifact is
   `Gis2dPlayApp`, registered via `.editor::<Gis2dPlayApp>(create_gis2d_app())`
   (`🌍️gis/🦀️component.rs:39`); drop comment at `✏️editor/🦀️component.rs:944-947`. The modern
   facet replacement it points to (`📚️examples/🎬️demo`) is itself id `"demo"`, not `"reuse-map"`
   — there is **no** `📚️examples/🎬️reuse-map` facet anywhere in the gis plugin (only `🎬️demo` and
   `🎬️demo-session` directories exist). `"reuse-map"` is not a registered example id anywhere.
2. **Content branch**: `set_active_example::handle`
   (`✏️editor/🎮️commands/🎨️example/🦀️component.rs:25-38`) does **not** branch on the id value at
   all beyond emptiness:
   ```rust
   let next = if payload.example_id.is_empty() { GisMapSnapshot::default() } else { default_document() };
   ```
   i.e. any non-empty string (including `"reuse-map"`, but also any typo) loads the same
   `default_document()`. There is no dedicated `"reuse-map"` arm — it works only because
   `"reuse-map"` happens to be non-empty.
3. **Populates the window**: yes — `default_document()`
   (`🧬️schema/🦀️component.rs:299-303`) parses `REUSE_MAP_EXAMPLE_TEXT` into a `GisMapSnapshot`
   with real `positions`/`routes`/`regions`, confirmed non-empty by the `maphost` test asserting
   `!host.features.positions.is_empty()` (`✏️editor/🗺️maphost/🦀️component.rs:49`). This is also
   `Gis2dPlayApp::initial_snapshot()` (`✏️editor/🦀️component.rs:680-683`).
4. Note `GisMapSnapshot::default()` (what an **empty** id, or a boot-dispatched `""`, produces)
   is a genuinely empty document (`positions: Vec::new(), routes: Vec::new(), regions:
   Vec::new()` — `🧬️schema/📸️snapshot/🦀️component.rs:55-60`). Had the (empty) `exampleOptions`
   boot effect NOT short-circuited, `resolveBootExampleId` would have returned `""` and wiped the
   map — but since `exampleOptions.length === 0` for this OLD-builder app, that effect never
   runs, so this near-miss doesn't fire in practice.

**Verdict: PROVEN-GOOD**, same "dead wiring, correct `initial_snapshot()`" pattern, with the
extra wrinkle that the id string itself is never actually checked by the handler — it merely
happens to be non-empty.

## Summary verdict table

| pane | app id | exampleId | in wired options list? | real content branch? | populates window? | shows correct content at boot? | **Verdict** |
|---|---|---|---|---|---|---|---|
| generator | `s.procedural.procedural3d@1/*#editor` | `hexagonal-mushroom-column` | No (options structurally empty) | Yes, dedicated arm | Yes (meshes/widgets) | Yes, via `initial_snapshot()` | **PROVEN-GOOD** (content correct; brand-driven dispatch wiring is dead) |
| koordinator | `s.cad.cad@1/*#editor` | `hexagonal-cut-concrete-forest-left` | No (options structurally empty) | N/A (never dispatched) | Yes (real BREP shape/nodes) | Yes, via `initial_snapshot()` | **PROVEN-GOOD** (content correct; brand-driven dispatch wiring is dead) |
| aggregator | `s.puzzle.puzzle3d@1/*#editor` | `concrete-forest` | **Yes** | Yes, dedicated arm | Yes (objects/attractions) | Yes, via the real dispatch chain | **PROVEN-GOOD** (fully wired, not just coincidental) |
| aussuchen | `s.sourcing.curate@1/*#editor` | `demo-stock` | **No** (list has `"demo"` only) | Yes, dedicated arm for `demo-stock` — but unreachable, since the id the wiring actually dispatches (`"demo"`) faults | Yes, via `initial_snapshot()` | Yes visually, but only because the boot dispatch's fault is silently a no-op | **UNCERTAIN** (renders correctly by accident; the declared exampleId/options/dispatch mechanism is actively broken, not merely dormant) |
| bearbeiten | `s.process.process3d@1/*#editor` | `timber-beam-joinery` | No (options structurally empty) | Yes, generic fallback arm | Yes (real steps) | Yes, via `initial_snapshot()` | **PROVEN-GOOD** (content correct; brand-driven dispatch wiring is dead) |
| verfolgen | `s.gis.gismap@1/*#editor` | `reuse-map` | No (options structurally empty; no such facet exists) | No dedicated arm — any non-empty id maps to the same default | Yes (positions/routes/regions) | Yes, via `initial_snapshot()` | **PROVEN-GOOD** (content correct; brand-driven dispatch wiring is dead; the id string itself is never truly validated) |

**Bottom line for the two prior, disagreeing explorations**: both were partly right. Every one
of the six panes' `exampleId`s *does* correspond to real, non-empty, test-covered example content
somewhere in that app's own code — nobody is looking at total vaporware. But the specific
mechanism the ticket asks about — "the shell resolves `exampleId` then dispatches
`setActiveExample`" — is provably dead code (never fires) for 4 of 6 panes because their apps
still use the OLD `.editor::<E>()` builder, which always yields an empty
`PluginManifest.examples` for that app id; those four only display the right thing because each
app's independent `initial_snapshot()` happens to already equal the intended example. For
aussuchen the mechanism is not just dead but actively wrong (dispatches an id, `"demo"`, that the
handler explicitly rejects as unknown). Only aggregator (puzzle3d) has the declared
brand-mechanism genuinely working end to end.
