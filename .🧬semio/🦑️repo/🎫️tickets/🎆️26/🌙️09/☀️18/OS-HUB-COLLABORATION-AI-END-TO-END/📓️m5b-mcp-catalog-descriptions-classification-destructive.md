# 📓️ M5b — semio MCP catalog: descriptions, classification, destructive (G11 G8 + G9)

Slice M5b · 2026-09-24 · worker under the session-10 preamble (private `CARGO_TARGET_DIR=.tmp-ticket-0918/wp-m5b/target`,
`CARGO_INCREMENTAL=0`, freeze on stdio/gis/kernel pack+store/`semio-framework-plugin`). Captures:
`.tmp-ticket-0918/wp-m5b/generated/*.txt`. Labels: **measured** (executed, captured), **tested** (law run),
**unverified** (written, not exercised).

## 0. Headline

- **G8, measured through the real server** (`.mcp.json` argv, scripted client, before/after captures): `draw
  rectangle` → `draw.addLayer` 20.13 vs 8.96, `add layer` → `raster.addLayer` 13.80 vs 13.38, `export pdf` →
  `layout.exportPdf` 14.38 vs 13.53 — differentiated, non-tied; `canvasPointerMove`/`canvasEscape` not in the catalog,
  0 raw input events in any of three input-worded queries. Most of this was already live when the slice started
  (M5a's source work + W1's full descriptor regeneration, verified in §1–§2); the slice closed what was still open.
- **Destructive now covers exports/saves to user paths, resets and whole-document replaces**: the capability audit
  asks the two questions it could not ask before (§3.1); 40 declarations landed at source in 17 plugins (§3.2–3.3);
  `layout.exportPdf` went live `destructive=false approval=never` → **`true / whenDestructive`**. Installed-catalog
  audit: **50 → 18 findings** (4 frozen `replace-text`, 14 waiting on W1's regeneration of lowpoly/remodel/shooting/space).
- **Classification**: cad camera/projection-param and forms preview-runner steps → `Chrome` (no longer published);
  layout/cad descriptions filled — the six non-frozen named plugins have **0 undescribed agent verbs** (en+de).
- **Laws over the real installed catalog** (new `search::long`, 4 laws) + one fixture law: `search::` **19/19 green**.
- **G9**: `resources/list` → `semio://workspace/artifacts` ×1, 0 duplicates — already fixed (M5b-proto), measured live.
- **Freeze-blocked** (§9, verbatim): gis descriptions/classification, framework `replace-text` destructive, and the
  typed-payload → JSON Schema derivation (`capabilities_describe` still reflects hand-declared args only).


## 1. Inherited state (verified in tree)

`git status`/`git diff --stat` on `🌉️mcp/` is **empty**: everything M5a (and the earlier protocol-conformance M5b/M5br)
wrote has been auto-committed. Verified by grep, not by trusting the reports:

| inherited (M5a / M5b-proto / M5br / M7) | where | state |
|---|---|---|
| `manifest::CapabilityAudience {Agent, Input, Chrome}` + `derive_audience` (Interaction→Input, View∧¬palette→Chrome, else Agent) | `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:764-800` | present |
| `ActionSemantics.description: Option<LocalizedLabel>` (en/de pair, no default), `use_when`, `audience` | same, `:856` | present |
| `CapabilityEffects.destructive` + `ApprovalMode::WhenDestructive` gate | same + `🛡️policy/🦀️.rs` | present |
| SDK `AppBuilder::action_describe / action_use_when / action_audience / action_destructive` | `🔌️plugin/🦀️.rs:5266-5350` (**frozen crate**) | present |
| catalog `compile()` → `compile_with_audiences(…, AGENT_AUDIENCES)`; app-scope `app.actions` walked | `🗂️catalog/🦀️.rs:837-845` | present |
| BM25 7 fields: id×3, title×3, use_when×2, description×2, **param names×1.5**, **artifact kind×1**, owner+**plugin display name**×0.5 | `🔎️search/🦀️.rs` | present |
| `capabilities.search` `limit`/`cursor`/`total`/`nextCursor` | `🌉️mcp/🦀️.rs` | present |
| capability audit (`GESTURE_ROUTE_WORDS`, `DESTRUCTIVE_VERB_WORDS`, `audit_source`, `semio-os-mcp audit`, nx `capability-audit-check`) | `🗂️catalog/🦀️.rs:957-1168` | present |
| 8 M5a fixture laws in `🔎️search/🧪️tests/🔬️quick` | — | present |
| G9 dedupe: `resources.retain(seen.insert(uri))` + law `notify::quick::resources_list_never_repeats_a_uri` (M5b-proto) | `🧠️context/🦀️.rs:222-228` | present |

**Descriptors: W1 regenerated all of them since M5a.** `🐍️m5a-descriptor-census.ts` today
(`wp-m5b/generated/m5b-descriptor-census-before.txt`): **60/60 registry rows decode, 0 skipped**; 2073 capabilities, 1546
agent-published, 128 declared audiences, 460 described (en+de), 124 destructive, **0 undeclared gesture routes**.
So M5a's source work IS live now — the remaining gaps are per-verb authoring gaps, measured in §2.

**What is new in this slice** is only what §2's measurement shows is still missing (§3).


## 2. Before capture — three queries through the real server

`🐍️m5b-catalog-probe.ts` spawns `semio-os-mcp` with `.mcp.json`'s `semio` argv verbatim (`stdio --folder
.🧬semio/🔗space/os-mcp --scopes …`; only the executable is the one built from this tree in the private target dir,
`cargo build -p semio-framework-os-mcp --bin semio-os-mcp`, 2 m 18 s). Capture `wp-m5b/generated/m5b-before.txt`,
2026-09-24T14:58Z, catalogHash `27fda405…`, 0 registry skip lines. **measured**

| query | top hit | top-1 vs top-2 | top-5 distinct | empty descr. (top 10) |
|---|---|---|---|---|
| `draw rectangle` | `draw…#editor.addLayer` | 20.17 vs 8.97 | 5/5 | 0/10 |
| `add layer` | `raster…#editor.addLayer` | 13.83 vs 13.41 (draw.addLayer) | 5/5 | 3/10 |
| `export pdf` | `layout…#editor.exportPdf` | 14.41 vs 13.57 (draw.exportDocument) | 4/5 | 7/10 |

So G8's two runtime acceptance lines already hold on the live catalog (non-tied differentiated top hits;
`canvasPointerMove`/`canvasEscape` are not in the catalog — `capabilities_describe` answers not-found, and the queries
`canvas pointer move` / `escape the canvas` / `pointer down` return 0 raw input events). What the capture exposes instead:

1. **Export to a user path is not destructive.** `layout.exportPdf`: `destructive=false approval=never`. Every
   `export*`/`save*` shell verb in the catalog (61 shell verbs census) has the same state — an agent can write files
   to the user's disk with no approval.
2. **Replace-whole-document verbs still unmarked**: `draw.commitDocument` ("commits a supplied snapshot as the next
   revision" — a full-document replace), `flow`/`procedural generation2d`/`demonstrator playground`
   `setActiveExample` (Mutation, destructive=false), `lowpoly.replaceSnapshotJson`.
3. **Descriptions**: in the seven named plugins, `wp-m5b/generated/m5b-verb-gap-census-before.txt`
   (`🐍️m5b-verb-gap-census.py`): draw 0, note 0, raster 0 undescribed; layout 3 (`translate/rotate/scaleSelection`),
   forms 3 (`resetTry/previousStep/nextStep`), cad 3 (`patchCadPlayReference/setCamera/setProjectionParam`),
   **gis 19 of 29** (frozen crate, §9).
4. **Classification leaks**: cad `setCamera`/`setProjectionParam` (camera pose, `WindowConfig` lane) and forms
   `resetTry/previousStep/nextStep` (preview-runner window state, payload is `window_id`/`window_kind_id`) are
   published as agent verbs.
5. **Input schemas are only as good as hand-declared args**: `layout.addPage` answers `properties:{}` — correct, its
   payload `AddPage {}` is empty — but the census finds **75 agent verbs in the seven plugins whose typed payload has
   fields the descriptor does not declare** (e.g. `note.moveBlock(block_id,target_row_id,drop_position)`,
   `cad.addObject(typology)`, `layout.exportPdf(page_id)`). §9 has the derivation, blocked by the freeze.
6. G9: `resources/list` → 15 resources, `semio://workspace/artifacts` ×1, 0 duplicates — **already fixed live**.


## 3. Changes landed

### 3.1 Gateway (`semio-framework-os-mcp`, not frozen) — the audit now asks the two missing questions

`🗂️catalog/🦀️.rs` `🔖️Audit`:

- `DESTRUCTIVE_VERB_WORDS` (delete-class) gains `reset`, `replace`, `overwrite`, and is asked of **`Mutation` and
  `Shell`** verbs (a shell `deleteSpace`/`removeMember` discards content outside any document history).
- New `DOCUMENT_REPLACE_WORDS` (`setactiveexample`, `setfixturejson`, `setspecjson`, `setsnapshot`,
  `commitdocument`, `loaddocument`, `setdocument`) — asked of **`Mutation` only**, because the same id on a shell
  verb navigates (measured: `space.studio.setActiveExample` emits `Effect::Navigate`, it replaces nothing).
- New `USER_PATH_WRITE_WORDS` (`export`, `download`, `save`) + finding
  `CatalogAuditFinding::UnmarkedUserPathWrite` — asked of **`Shell` and `View`** verbs (a `View`-kind
  `exportDocument` hands the file to the host just the same).

This is the schema-first answer to "derive where the framework knows": the framework knows `kind`, so the audit
derives *which question* to ask from it; the per-verb *answer* is an authored declaration. `semio-os-mcp audit`
and nx `capability-audit-check` pick it up unchanged. Measured on the installed descriptors with the new binary:
**50 findings over 60 descriptors** (`wp-m5b/generated/m5b-audit-installed-before-regen.txt`), every one of which
is closed at source below except the four `replace-text` rows (frozen SDK, §9.2).

Fixture `🧪️tests/🧱️source-builders/🦀️.rs` mirrors the real plugins: cad `saveSelected/saveInPlay/saveCurrent`, note
`saveDownload`, draw `exportDocument` → `.destructive()`.

### 3.2 Plugin sources — the seven named plugins (gis frozen → §9.1)

| plugin | destructive (approval `WhenDestructive`) | audience | description (en+de) |
|---|---|---|---|
| draw | `commitDocument` (whole-document replace), `exportDocument` (user path) | — | — (all 25 described already) |
| note | `saveDownload` | — | — |
| raster | — | — | — (no gap) |
| layout | `exportPng`, `exportSvg`, `exportPdf`, `exportPackage` | — | `translateSelection` (+use_when), `rotateSelection`, `scaleSelection` |
| forms | `exportFixture` | `resetTry`, `previousStep`, `nextStep` → **Chrome** (preview-runner window state, payload is `window_id`) | — |
| cad | `saveCurrent`, `saveSelected`, `saveInPlay` | `setCamera`, `setProjectionParam` → **Chrome** (camera pose, `WindowConfig` lane) | `patchCadPlayReference` |

Each is one chain step at the verb's own builder chain, after its declaration (checked: a `.action_describe` placed
before the `.action_with` that declares the id is silently lost — layout's first placement was, and was moved).

### 3.3 Plugin sources — every other non-frozen audit finding

Inserted by `🐍️m5b-declare-destructive.py` directly after the verb's own unique `.action_interactive_job("<id>", …)`
step (refuses on 0 or >1 anchors; `.await` kept for the bare-`AppBuilder` space chain): animate `resetGrid`,
`exportVideoFromDeck`; architect `exportProgram`, `exportRegistersCsv`; flow `setActiveExample`; procedural
generation2d `setActiveExample`, generation3d editor+viewer `exportDocument`; process `exportModel` (by hand, no
job anchor); puzzle 2d/3d/5d `exportFixture`; remodel `exportQcReport`, `resetPlaceholderMesh`; shooting
`exportActiveShot`, `exportAllShots`, `resetFixture`, `saveDownload`; space `deleteSpace`, `removeMember`,
`requestDeleteArtifact`, `exportMedia`, `exportStudioDsl`, `exportStudioPack`; trinity `resetRule`; lowpoly
`exportMesh`, `replaceSnapshotJson`; demonstrator playground `setActiveExample` (its composed cad/procedural/process/
puzzle rows inherit the upstream marks on regeneration). Not marked, deliberately: `space.studio.setActiveExample`
(navigates), note `loadRequest` / cad `loadRawRequest` (imports that add content, and open a host file picker a
human answers anyway).


## 4. Laws / crate tests

All with `CARGO_TARGET_DIR=.tmp-ticket-0918/wp-m5b/target CARGO_INCREMENTAL=0`, `-p semio-framework-os-mcp`.

| run | result | capture |
|---|---|---|
| `cargo check --all-targets` after the gateway + law edits | **EXIT 0**, 93 warnings (all pre-existing; the one in my new file — `unnecessary qualification` — fixed) | `m5b-check-1.txt` |
| `cargo test --lib search::quick` (8 M5a laws + the new one) | **15 passed / 0 failed** — **tested** | `m5b-test-search-quick.txt` |
| `cargo test --lib quick::` (every quick module of the crate) | **392 passed / 3 failed** | `m5b-test-quick-all.txt` |
| `cargo test --lib search::` incl. the new installed-catalog laws, before regeneration | 16 passed / 3 failed (a fixture-id typo in my new quick law, since fixed, + laws 3–4 below, red by design) | `m5b-test-search.txt` |
| `cargo test --lib search::` after the five descriptors were regenerated (§7) | **19 passed / 0 failed** — **tested** | `m5b-test-search-after.txt` |
| wasm32 `cargo check` of the 17 touched plugin crates | **EXIT 0**, 1 m 35 s (warnings only, none in edited lines) | `m5b-wasm-check.txt` |
| `semio-os-mcp audit --folder .` (= `capability-audit-check`) | 50 findings before regeneration → **18** after | `m5b-audit-installed-{before,after}-regen.txt` |

The 3 `quick::` failures are not in code this slice touched and sit in files peers are editing right now
(`git status` shows `🔀️dispatch`, `🖥️ui`, `🧬️schema` modified by others): `actions::quick::hub_gis_approval_history_…`
(dispatch), `schema::quick::the_json_mirror_publishes_exactly_the_registry_exports` ("🔣️.json is stale — regenerate it
with `schema-mirror`"; no schema type changed in this slice), `ui::quick::a_bound_cancel_hook_obeys_…` (ui).

**New laws.**

- `search::quick::user_path_writes_and_document_replacement_gate_on_approval` (fixture): `exportDocument`,
  `saveDownload`, `setFixtureJson` carry `destructive` + `WhenDestructive`; the additive import `loadRequest` does not.
  **tested, green.** Law 8 (audit) extended: a regressed `exportDocument` is an `UnmarkedUserPathWrite`. **green.**
- `search::long` — **over the real installed registry + committed descriptors**, i.e. the catalog `.mcp.json`'s server
  compiles (`🔎️search/🧪️tests/🔬️long/🦀️.rs`, new):
  1. `the_installed_catalog_answers_three_intent_queries_with_a_clear_winner` — `draw rectangle` → `draw.addLayer`
     (> 1.5× runner-up, 5 distinct top-5 scores); `add layer` → an `addLayer`, strictly above #2; `export pdf` → an
     `export*` verb, strictly above #2. **tested, green.**
  2. `the_installed_catalog_publishes_no_raw_input_event` — `canvasPointerMove/canvasEscape/canvasPointerDown/
     engagementInput` (draw), layout `canvasPointerMove`, cad `worldPointerDown`, note `inkApplyEvents` absent;
     every entry `Agent`; 0 `UndeclaredGestureRoute` findings. **tested, green.**
  3. `the_authored_plugins_describe_every_published_verb_in_en_and_de` (draw/note/raster/layout/forms/cad).
     Red before regeneration (it listed exactly the 9 verbs §3.2 classifies/describes), **green after**.
  4. `the_authored_plugins_gate_destructive_and_user_path_verbs_and_keep_chrome_out` — no audit finding for the six,
     named exports/saves/replaces destructive + `WhenDestructive`, cad camera + forms preview steps unpublished.
     Red before regeneration (its left side was exactly the 13 source fixes of §3.2), **green after**.


## 5. After capture

Same probe, binary rebuilt from this tree (`semio-os-mcp-after`), after the five authored descriptors were
regenerated: `wp-m5b/generated/m5b-after.txt`, 17:5xZ. **measured**

| | before | after |
|---|---|---|
| `draw rectangle` top-1 / top-2 | addLayer 20.17 / 8.97 | addLayer 20.13 / 8.96 |
| `add layer` top-1 / top-2 | raster.addLayer 13.83 / 13.41 | raster.addLayer 13.80 / 13.38 |
| `export pdf` top-1 / top-2 | layout.exportPdf 14.41 / 13.57 | layout.exportPdf 14.38 / 13.53 |
| raw input events in 3 input queries | 0 | 0 |
| `layout.exportPdf` destructive / approval | **false / never** | **true / whenDestructive** |
| `draw.setSnapshot`, `draw.deleteLayer`, `note.deleteSelection` | true / whenDestructive | unchanged |
| `resources/list` `semio://workspace/artifacts` | ×1 | ×1 |

Rankings move only by the second decimal (the BM25 corpus changed by the new descriptions and the removed chrome
verbs). `search::long` (4/4 green, `m5b-test-search-after.txt`) proves the rest on the same committed descriptors:
cad `setCamera`/`setProjectionParam` and forms `resetTry/previousStep/nextStep` are no longer published; layout
`translate/rotate/scaleSelection` and cad `patchCadPlayReference` carry en+de descriptions; every export/save/replace
of the six is `WhenDestructive`. Census after (`m5b-verb-gap-census-after.txt`): draw/note/raster/layout/forms/cad
**0 undescribed**, gis 19 (frozen); descriptor census total destructive **124 → 155**.


## 6. G9 — `semio://workspace/artifacts` once

Already fixed in source before this slice (`🧠️context/🦀️.rs` retains resources by first-seen URI; law
`notify::quick::resources_list_never_repeats_a_uri`, M5b-proto 2026-09-20). Measured live twice through the real server
with `--folder` bound (so the workspace's own re-report is exercised): **15 resources, `semio://workspace/artifacts` ×1,
0 duplicate URIs** (`m5b-before.txt`, `m5b-after.txt`). The law ran green inside `quick::` (392 passed). No change needed.


## 7. Descriptor regeneration requests (W1)

`.tmp-ticket/wp-w1/requests/m5b.txt` (17 crates, reason, expected fields; updated 17:55).

Done by me in **one** wasm-mutex hold (queued 17:14, held **17:40:45 → 17:49:38**, ~9 min, released before the
coordinator's 10-min limit; I will not take the lock again): the wasm32 compile proof of all 17 touched plugin crates
(`cargo check --target wasm32-wasip2 -p …`, **EXIT 0, 1 m 35 s**, `m5b-wasm-check.txt` — the compile-atomic proof of
§3.2/§3.3), then `describe` of layout, draw, forms, cad, note (each rc=0, `m5b-describe-*.txt`). Someone else's
describe after 17:10 had already picked up the §3.3 marks for animate, architect, flow, procedural, process, puzzle,
trinity and demonstrator's own `setActiveExample`.

**Still waiting on W1** (the 14 remaining non-frozen audit findings): lowpoly, remodel, shooting, space.

**Note for W1/coordinator:** `describe` serializes the *current* tree, so the regenerated `📐️cad/🔣️.json` also carries
peers' in-flight cad state — it dropped the `s.stdio.dwg` import/export counterpart relations and changed the
example `artifactJson` (a peer has `✏️s/🔌️plugins/📐️cad/🏭️bridge/🦀️.rs` modified). W1's own regeneration from the
same tree emits the same bytes; if those relations must survive, the fix is in the cad source, not the descriptor.


## 8. Live-agent-loop approval steps

`lsof -nP -iTCP -sTCP:LISTEN` at 17:15: in 7600–7899 only a hub on **7891** (`os-hub-7611` binary, pid 51991, not
mine); in 6000–6399 only a Python server on 6099 — **no `s` shell serve is listening**, so the gate
(`live-agent-loop-check`, which needs a live note shell) could not be rerun, and per the brief I did not start one.

The approval SKIPs the brief names are **already gone at runtime** — inherited, not this slice's: M5a's note
regeneration (2026-09-20) made `note.deleteSelection` destructive, and session 10's G7 run
(`.tmp-ticket/wp-g7/generated/g7-live-agent-loop.txt`, 20/20) shows `PASS (e1) Approve Once`, `PASS (e2) Deny returns
the typed refusal`, `PASS (e3) a silent client returns the typed timeout`. This slice widens the set of verbs that
reach that gate (exports, saves, resets, whole-document replaces); the gate's own mechanism is unchanged.


## 9. Freeze-blocked hunks (verbatim)

Written here instead of landed: the session-10 FREEZE covers `semio-framework-plugin` (`🔌️plugin/🦀️.rs`), the gis and
stdio crates.

### 9.1 gis — descriptions (19 undescribed agent verbs) + classification (`semio-s-plugin-gis`, frozen)

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, after the chain step
`.action_interactive_job("proposeBoundsRegion", InteractiveJobClassification::Migrated)`:

```diff
             .action_interactive_job("proposeBoundsRegion", InteractiveJobClassification::Migrated)
+            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole map with one of the plugin's declared playground examples.", "Ersetzt die gesamte Karte durch eines der deklarierten Beispiele des Plugins."))
+            .action_describe("patchPositions", LocalizedLabel::native("Replaces the coordinates of one or more map features from a JSON position list.", "Ersetzt die Koordinaten eines oder mehrerer Kartenobjekte aus einer JSON-Positionsliste."))
+            .action_describe("patchRoutes", LocalizedLabel::native("Sets one named property on several routes at once.", "Setzt eine benannte Eigenschaft auf mehreren Routen gleichzeitig."))
+            .action_describe("patchRoute", LocalizedLabel::native("Sets one named property of one route — its name, colour or waypoints.", "Setzt eine benannte Eigenschaft einer Route — Name, Farbe oder Wegpunkte."))
+            .action_describe("addFeature", LocalizedLabel::native("Adds a new point, line or area feature to the map.", "Fügt der Karte ein neues Punkt-, Linien- oder Flächenobjekt hinzu."))
+            .action_use_when("addFeature", vec!["add a marker".into(), "draw a route on the map".into(), "add an area".into()])
+            .action_describe("moveFeature", LocalizedLabel::native("Moves one map feature to new coordinates.", "Verschiebt ein Kartenobjekt an neue Koordinaten."))
+            .action_describe("renameFeature", LocalizedLabel::native("Renames one map feature.", "Benennt ein Kartenobjekt um."))
+            .action_describe("deleteFeature", LocalizedLabel::native("Removes one feature from the map by id.", "Entfernt ein Objekt anhand seiner Id aus der Karte."))
+            .action_describe("openSource", LocalizedLabel::native("Opens the selected feature's source URL through the host.", "Öffnet die Quell-URL des ausgewählten Objekts über den Host."))
+            .action_describe("proposeBoundsRegion", LocalizedLabel::native("Asks the inference service to propose a bounds region for human review; never writes the map itself.", "Bittet den Inferenzdienst, eine Begrenzungsregion zur menschlichen Prüfung vorzuschlagen; schreibt die Karte nie selbst."))
+            .action_audience("toggleLayerVisibility", semio_framework_plugin::CapabilityAudience::Chrome)
+            .action_audience("fitWorld", semio_framework_plugin::CapabilityAudience::Chrome)
+            .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
+            .action_audience("setRenderMode", semio_framework_plugin::CapabilityAudience::Chrome)
+            .action_audience("setVectorStyle", semio_framework_plugin::CapabilityAudience::Chrome)
+            .action_audience("setLodMode", semio_framework_plugin::CapabilityAudience::Chrome)
+            .action_audience("focusFeature", semio_framework_plugin::CapabilityAudience::Chrome)
+            .action_audience("setLayerStrokeScale", semio_framework_plugin::CapabilityAudience::Chrome)
```

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`:

```diff
             .action_interactive_job("setExaggeration", InteractiveJobClassification::Migrated)
+            .action_describe("setExaggeration", LocalizedLabel::native("Sets the vertical exaggeration factor of the terrain surface.", "Legt den Überhöhungsfaktor der Geländeoberfläche fest."))
```

After landing + `gis` descriptor regeneration, add `"gis"` to `AUTHORED_PLUGINS` in `🔎️search/🧪️tests/🔬️long/🦀️.rs`.

### 9.2 Framework window kit `replace-text` — derived destructive at the source (`semio-framework-plugin`, frozen)

The three stdio text editors and the demonstrator publish `replace-text` (a whole-text replacement) with
`destructive=false`; it is minted by the SDK's `TextWindowKit`, so the fix belongs to the framework, once, for every
editor that composes the kit. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (`editable_window_kind`, ≈ line
32866):

```diff
                 vec![ActionDefinition::bounded_catalog("replace-text", LocalizedLabel::native("Replace Text", "Text ersetzen"), ActionKind::Mutation)
-                    .with_args(vec![ActionArgDef::text("text", LocalizedLabel::native("Text", "Text")).required()])],
+                    .with_args(vec![ActionArgDef::text("text", LocalizedLabel::native("Text", "Text")).required()])
+                    .describe(LocalizedLabel::native("Replaces the whole text of the document with the given text.", "Ersetzt den gesamten Text des Dokuments durch den angegebenen Text."))
+                    .destructive()],
```

Clears the four `replace-text` findings (`stdio` ×3, `demonstrator` ×1) once stdio/demonstrator descriptors regenerate.

### 9.3 `capabilities_describe` schema derived from the typed payload (`semio-framework-plugin`, frozen)

Today the input schema is projected from hand-declared `ActionArgDef`s only; §2.5 counts 75 agent verbs in the seven
plugins whose `app_commands!` payload has fields but whose descriptor declares none. The payload's typed field
grammar already exists at runtime in every guest: each command enum derives `dsl::DslVariants`, whose
`variants()` yields `(wire keyword, fn() -> dsl::RecordSpec)`, and `app_commands!` holds the `$id as $key` pairing.
The derivation therefore belongs in the macro, emitted once for every app:

```diff
 impl $Name {
     pub const TOOL_JOB_IDS: &'static [&'static str] = &[$($id),*];
+
+    /// 📐️ Every row's typed payload projected into manifest argument declarations, keyed by manifest id.
+    pub fn payload_args() -> Vec<(&'static str, Vec<$crate::ActionArgDef>)> {
+        let variants = <Self as ::dsl::DslVariants>::variants();
+        [$(($id, $key)),*].iter().map(|(id, key)| (*id, variants.iter().find(|(keyword, _)| keyword == key).map(|(_, spec)| $crate::payload_arg_defs(&spec())).unwrap_or_default())).collect()
+    }
```

plus, in the same crate, `pub fn payload_arg_defs(spec: &dsl::RecordSpec) -> Vec<ActionArgDef>` (one `ActionArgDef`
per `FieldSpec`, id = the field key in the camelCase the action decoder reads, `required` = `!optional`; `Bool` →
toggle, `Int/UInt/Count` → integer, `Float/Quantity/Angle` → number, `Text/Ref/Embed` → text, `Enum(tags)` →
select over the tags, `List/Tuple/Coord/Dir/Dim/Range` → JSON array, `Record/Map/Value` → JSON text) and an
`AppBuilder::payload_args(rows)` step that fills `args` of every declared action whose `args` is empty. Each editor
chain then gains one step, `.payload_args(<Cmd>::payload_args())`, and the catalog needs no change: it already
projects `ActionArgDef`s into a JSON Schema. **Precondition found while designing it:** several plugins decode MCP
arguments through hand-written per-verb tables whose keys differ from the payload field names (e.g. cad
`SetCamera { pane }` ← `"surfaceId"`), i.e. a second, hand-kept schema; the derivation must land together with
decoding every action generically from the payload's own `FromValue`, or the published schema will name keys the
decoder does not read. That makes it a framework-wide change, not a catalog one — hence not transcribed by hand here.


## 10. Honest gaps

1. **The live catalog changes only when the descriptors are regenerated.** Every §3.2/§3.3 declaration is plugin
   source; the MCP compiles the committed `🔣️.json`. Regeneration is W1's (request `.tmp-ticket/wp-w1/requests/m5b.txt`,
   17 crates). Five were regenerated here (§7); lowpoly, remodel, shooting and space still publish their 14
   exports/deletes/resets without approval until W1 regenerates them.
2. **gis (19 undescribed, 8 chrome leaks), `replace-text` (4 rows) and the typed-payload schema derivation are frozen**
   (§9). The typed-payload derivation additionally needs every action decoded generically from its payload — a
   framework-wide change larger than this slice.
3. **`use_when` stays English-only** (`Vec<String>`, inherited from M5a); German queries match through the German
   `description`/`label` only.
4. **No third-party oracle for the BM25 ranking** (AGENTS.md asks for one): the laws pin outcomes, not an independent
   reimplementation's scores.
5. The audit lexicons ask questions, they do not classify: a delete-class verb named without one of their words is
   invisible to them (e.g. a `purgeCache` would be caught, a `dropAll` would not).


## 11. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗂️catalog/🦀️.rs` | audit: delete-class lexicon (+`reset/replace/overwrite`, Mutation+Shell), new `DOCUMENT_REPLACE_WORDS` (Mutation), new `USER_PATH_WRITE_WORDS` + `UnmarkedUserPathWrite` (Shell+View) |
| `…/🌉️mcp/🔎️search/🦀️.rs` | mounts `🧪️tests/🔬️long` |
| `…/🌉️mcp/🔎️search/🧪️tests/🔬️long/🦀️.rs` (new) | 4 installed-catalog laws |
| `…/🌉️mcp/🔎️search/🧪️tests/🔬️quick/🦀️.rs` | law 8 extended; new `user_path_writes_and_document_replacement_gate_on_approval` |
| `…/🌉️mcp/🧪️tests/🧱️source-builders/🦀️.rs` | fixture cad saves, note `saveDownload`, draw `exportDocument` destructive |
| `✏️s/🔌️plugins/{🖍️draw,🗒️note,📏️layout,📋️forms,📐️cad}/…/✏️editor/🦀️.rs` | §3.2 |
| `✏️s/🔌️plugins/{🎞️animate,🏛️architect,🌊️flow,🌀️procedural (gen2d editor, gen3d editor+viewer),🏭️process,🧩️puzzle (2d/3d/5d),📸️remodel,🎥️shooting,🪐️space (engine, home, space),🔱️trinity,💠️lowpoly,🎪️demonstrator}/…` | §3.3 |
| `.tmp-ticket/wp-w1/requests/m5b.txt` | regeneration request |
| ticket folder | `🐍️m5b-catalog-probe.ts`, `🐍️m5b-verb-gap-census.py`, `🐍️m5b-declare-destructive.py`, `📜️m5b-wasm-check.sh`, `wp-m5b/verbscan.py` |

No `.vscode/launch.json` row: no new runnable command (the new laws run inside the existing `test` target, the audit
inside the existing `⚖️gate🌉️os-mcp🚨️capability-audit`).


Cleanup: private `wp-m5b/target` deleted; no process of this slice is running (mutex wrapper pid 96132 exited 17:49:38,
lock released to W1 at 17:49:51); no hub or serve was started.
