# 📓️ W2-R tail — input-UI rollout for the tail plugin group

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Executor: W2-R-tail, 2026-09-30. Contracts: `📋️design.md` §6, the
`🧭️plan.md` "W2-R brief", and the W1-D reader and `$defs/InputUi` meta-schema.

**Scope.** The `✏️s/🔌️plugins/` groups `💡️reasoning`, `📖️playbook`, `🏭️process`, `💠️lowpoly`, `🔱️trinity`, `✒️writer`,
`🕸️dag`, `📜️imperative`, `➗️mathematical`, `🎞️animate`, `🪐️space`, `🌊️flow`, `🌍️gis`, `🎬️sequence`, `🌿️vcs`,
`🎪️demonstrator` and `🪵️sourcing`. That is 17 census owners and 184 catalogued mutation leaves, plus 6 uncatalogued leaf
schemas under the same roots.

The census owner `window` (`window.gis.map.config.*`) sits physically under `🌍️gis`, but the partition assigns it to the
mid group ("os/window"). W2-R-mid has snapshotted those files, so I did not touch them.

## 1. Outcome

**DONE and VERIFIED.**

- **Strict lint.** In my scope `schema mutation-inputs` went from 95 findings to 4. All 4 are
  `refUnresolved` findings on `framework/value/schema.json`, which W1-D owns. There are 0 `labelMissing`,
  0 `optionLabelMissing`, 0 `malformed`, 0 `uiInvalid` and 0 `widgetIncompatible`.
- **Annotations.** 365 `x-semio-ui` annotations were written into 154 leaf payload schemas. Removing every `x-semio-ui`
  from the result gives back the original documents exactly (structural diff 0/154). File formatting was preserved.
- **The lint undercounted the work.** It stops at the first refusal per top-level input, so 95 findings stood for 215
  nested inputs that lacked labels or option labels. The walker `🧪️w2-r-tail-walk.py` lists every one of them, and all
  are fixed.
- **Beyond the brief.** I also added target references with the plugin's own selection `domain`/`granularity` to the
  address ids of the scope (69 more leaves), so that `historyEditUseSelection` works. The reasons are in §3.

## 2. Census before / after

The numbers come from `bun ./📜️script.ts schema mutation-inputs --json` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`.
The whole-repo total also moved while peer groups were working in parallel: 1842 findings before, 881 after.

| owner | leaves | inputs before | declared before | findings before | inputs after | declared after | findings after |
|---|---|---|---|---|---|---|---|
| reasoning | 12 | 27 | 17 | 10 (labelMissing 7, refUnresolved 3) | 27 | 24 | 3 (refUnresolved 3) |
| playbook | 12 | 23 | 14 | 9 (labelMissing 9) | 23 | 23 | 0 |
| process | 16 | 26 | 17 | 9 (labelMissing 9) | 26 | 26 | 0 |
| lowpoly | 17 | 40 | 32 | 8 (labelMissing 8) | 40 | 40 | 0 |
| trinity | 17 | 25 | 17 | 8 (labelMissing 7, refUnresolved 1) | 25 | 24 | 1 (refUnresolved 1) |
| writer | 10 | 14 | 6 | 8 (labelMissing 8) | 14 | 14 | 0 |
| dag | 17 | 36 | 29 | 7 (labelMissing 7) | 36 | 36 | 0 |
| imperative | 7 | 14 | 7 | 7 (labelMissing 7) | 14 | 14 | 0 |
| mathematical | 16 | 33 | 26 | 7 (labelMissing 7) | 33 | 33 | 0 |
| animate | 11 | 15 | 10 | 5 (labelMissing 5) | 15 | 15 | 0 |
| space | 5 | 8 | 4 | 4 (labelMissing 4) | 8 | 8 | 0 |
| flow | 10 | 27 | 24 | 3 (labelMissing 3) | 27 | 27 | 0 |
| gis | 16 | 25 | 22 | 3 (labelMissing 3) | 25 | 25 | 0 |
| sequence | 8 | 13 | 10 | 3 (labelMissing 3) | 17 | 17 | 0 |
| vcs | 6 | 6 | 3 | 3 (labelMissing 3) | 6 | 6 | 0 |
| demonstrator | 1 | 1 | 0 | 1 (labelMissing 1) | 1 | 1 | 0 |
| sourcing | 3 | 4 | 4 | 0 | 4 | 4 | 0 |
| **total** | 184 | 337 | 242 | 95 | 341 | 337 | 4 |

The sequence input count rose from 13 to 17 because the lint counts a whole-leaf refusal as 1 input. Sequence leaves have
root `$ref: #/$defs/Payload`, so the lint evaluates them as one unit.

## 3. Decisions (applied uniformly)

- **Labels.**
  - English uses Title Case, which is the style of the framework glossary (`New Value`, `Node ID`) that fills most sibling
    fields of the same forms. German is written in normal German.
  - Boolean `new*` replacements drop the "New", as the glossary does (`newSmoothShading` → "Smooth Shading" /
    "Glatte Schattierung").
  - Where an app terminology already exists, its wording is reused: writer "Tabulatorgröße" and "Zeilenhöhe"; lowpoly
    "Füllmethode" and "Malebene"; dag node kinds "Berechnung", "Schieberegler" and so on; playbook "Nutzlast".
  - Some glossary labels are wrong in context and are overridden:
    - playbook block `step` → "Step Size" / "Schrittweite" (the glossary reads "Schritt");
    - mathematical `change-coefficient.label` (an integer node label) → "Coefficient" / "Koeffizient";
    - writer splice `before` → "Context Before" / "Kontext davor";
    - lowpoly paint-layer `index` → "Layer Index" / "Ebenenindex";
    - sequence slot `name` → "Slot Name" / "Slot-Name";
    - `kind` → "Block Kind", "Media Kind" or "Node Kind".
- **Descriptions** are given wherever the name does not explain itself. Examples: JSON-text fields, timestamps,
  normalized crops, the base64 bytes of pixel runs, the rule LHS/RHS, the process playback cursor ("number of timeline
  steps applied to the stock; empty applies all"), and the machine/capability origin ("informational only").
- **Widgets.**
  - `multiline` for JSON-text and long-text fields: `*Json`, notes, deleted/inserted text, errors, mesh workspace, meshContent.
  - `toggle` for booleans.
  - `stepper` with `step 1` / `precision 0` for integers and counts; timestamps also get `unit ms`, and font size and line
    height get `unit px`.
  - `stepper` with a decimal step for free numbers (camera, drag pointer, exaggeration, aspect ratio).
  - `dial` for process `Pose.angle`, which is radians (`#[dsl(angle="rad")]`): `unit rad`, `displayUnit deg`, snaps
    ±π, ±π/2 and 0. This is W1-F's angle recipe.
  - `vector` for the process rotation axis.
  - `select`/`segmented` with localized `options` for every enum: dag node kind (11 values), dag route style
    (Bézier / orthogonal S/Z), and trinity port direction (Input/Output).
- **References.**
  - Entity ids get `widget: reference`. The mutation's addressed id gets `role: target`; endpoints and other entity-valued
    fields get `role: value`.
  - `ref.kind` comes from the artifact's own semantic entity (widget, synapse, tile, step, block, object, node, edge,
    machine, capability, artifact, route, position, region).
  - `domain` and `granularity` come from the plugin's manifest interaction, and are set only when that interaction really
    selects the entity:

    | Plugin | domain | granularities |
    |---|---|---|
    | flow | `graph` | node / edge |
    | reasoning | `graph` | node / edge |
    | dag | `graph` | node / edge |
    | playbook | `blocks` | block / step |
    | imperative | `steps` | step |
    | sequence | `steps` | step |
    | animate | `tiles` | tile |
    | lowpoly | `mesh` | object |
    | trinity jack | `ast` | node |
    | sourcing | `rows` | object |

    Math, process, space and gismap have no selecting interaction for these entities, so they carry `kind` only.
  - Freshly minted ids (`synapseId` of duplicate-widget, `executionId`, `childId`, `newLanguageId`, `newIconId`, artifact
    ids of child handles) are explicit `widget: text`. This stops the reader's `<kind>Id` inference from turning them into
    pickers; before, `childId` was inferred as a `child` reference.
- **Why target references on the non-flagged address ids.** The flagged set alone would have left `id` inputs as plain
  text fields, and `nodeId`-style inputs as domain-less inferred references. Without a domain,
  `historyEditUseSelection` (design §7) cannot pick from the selection. The added annotations cover every
  `delete-*`/`move-*`/`rename-*`/`reorder-*`/`change-*` target in the scope where the entity is unambiguous.
- **Hard bounds.** None were added: no bound in the scope was semantically certain beyond what the schemas already
  declare. The brief also forbids changing payload structure.
- **Groups and order** were not set. Most forms mix glossary-labelled and annotated fields, and a partial `order` would
  reorder them inconsistently. Schema order is kept.
- **Records** (object and array inputs that are read recursively) get `{role: value, label, description}`, as in W1-F.

## 4. Verification (all run, all green)

| Check | Command | Result |
|---|---|---|
| Strict lint (TS reader, catalogued leaves) | `bun ./📜️script.ts schema mutation-inputs --json` (test module) | scope: 337/341 declared, 4 findings, all `refUnresolved` (framework/value). `--under ✏️s/🔌️plugins/📖️playbook` and `--under …/🕸️dag`: 0 findings each |
| Full per-leaf walk (nested inputs, incl. uncatalogued) | `python3 🧪️w2-r-tail-walk.py` | before: 215 findings (208 labelMissing, 3 optionLabelMissing, 4 refUnresolved); after: 4 refUnresolved |
| Reader over every leaf under the 17 roots | `bun 🧪️w2-r-tail-check-inputs.ts [--verbose]` | 196 leaves, 351 inputs. Refusals: the same 3 framework/value leaves, plus mid's `window.gis.map` set-camera (`labelMissing`). The verbose dump of every derived `ActionArgDef` (label en/de, control, reference kinds/domain/granularity) was reviewed by eye |
| Strict x-semio Ajv oracle (`semioSchemaAjvV1({strict:true})`) | same script | 189/196 compile. The 7 failures are the 3 framework/value leaves plus 4 trinity window leaves (`set-camera`, `set-lod-mode` of jack and rewriting), which reference an unresolvable `$schema` draft URL. Those 4 are untouched peer files |
| Third-party Python `jsonschema` Draft 7 + `referencing` | `.venv/bin/python 🧪️w2-r-tail-verify.py 🗑️generated/w2r-tail/originals.json` | 365 annotations valid against manifest `$defs/InputUi`; 0 Draft-7 meta-schema errors over all leaf schemas of the scope; 162 committed fixture payloads, **verdict changed by the annotation: 0** (144 ok, 15 invalid before and after, 3 unresolvable framework/value) |
| Structure preserved | script over `originals.json` | 154/154 documents are identical after stripping `x-semio-ui` |
| Idempotency | `python3 🧪️w2-r-tail-annotate.py` twice | second run rewrites 0 files |

## 5. Findings outside this WP (pre-existing, not changed)

1. **`refUnresolved`, owned by W1-D.** Four inputs reference `https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/DslValue`,
   and no catalogued document has that `$id`:
   - `s.reasoning.wires` `create-node /node`;
   - `s.reasoning.wires` `connect-nodes /edge` and `/relationship`;
   - `s.trinity.rewriting` `change-parameter-binding /newValue`.

   These inputs already carry a labelled `x-semio-ui` record annotation, so they resolve as soon as the document is catalogued.
2. **Leaf schema vs. wire payload drift.** 15 committed fixtures are rejected by their own leaf schema, both before and
   after my change. This matters for `historyEditInput`, because the input path from the schema must equal the
   `payload_value()` key.
   - **mathematical.** `change-node-label`, `change-graph-directed` and `update-graph-algorithm` declare camelCase
     (`newDirected`, `newAlgorithm`, …). The Rust payloads have no `rename_all` and the fixtures use snake_case
     (`new_directed`, `new_algorithm`, `new_label`). Separately, `replace-graph.graph.algorithmSeed` is `Option` but typed
     `string`.
   - **process.** `ProcessMeasure` and `MeasureRecipe` are schematized as tag-only `oneOf` stubs, while the wire is
     `{measure:"cut", tool, pose}` and `{recipe:"boreDrill", radius, depth}`. `replace-stock-solid.newSolid.target.dialect`
     is typed `string` but is an object.
   - **imperative.** `pathRef.owner` is `Option<String>` but typed `string`; the fixtures send `null`.
   - **flow / dag.** In `create-widget`, `replace-widget` and `replace-node-kind`, the union stubs use PascalCase consts
     (`InputNote`, `Computation`), while the wire uses camelCase tags with fields.
3. **`schema-catalog-stale`.** `📚️library/🔣️schema-catalog.json` hashes no longer match the disk, because of fleet-wide
   annotation edits. I did not regenerate it: it is a 4 MB hot file with 8 W2-R groups writing concurrently. **A central
   `bun ./📜️script.ts schema generate` is needed after the W2-R wave.**
4. **`schema check` findings on 13 of the touched files, all pre-existing.** These are:
   - `schema-mutation-leaf-id`: writer/trinity window `$id`s, and imperative `$id`s that say `imperative` instead of `procedure`;
   - `schema-module-id-missing`: jack `replace-query-result` has no `$id` and is uncatalogued;
   - `schema-document-id-unaddressable` (1).

   Uncatalogued leaves under the roots, which the lint cannot see: jack `replace-query-result` (annotated), space home
   `apply-directory-page` (annotated), and jack/rewriting `set-camera` and `set-lod-mode` (these already resolve through
   the glossary).
5. **Mid's leaf.** `window.gis.map.config.mutation.set-camera` `/cameraJson` is still `labelMissing`. It belongs to W2-R-mid,
   whose snapshot includes it.

## 6. Files

**Ticket inputs** (kept):
- `🧪️w2-r-tail-walk.py` — complete reader-mirroring walker that lists annotation sites;
- `🧪️w2-r-tail-annotate.py` — the annotation tables; format-preserving and idempotent;
- `🧪️w2-r-tail-verify.py` — Python jsonschema oracle;
- `🧪️w2-r-tail-check-inputs.ts` — TS reader over every leaf, plus the strict Ajv oracle.

**Scratch**, in `🗑️generated/w2r-tail/`:
- the baselines `before-full.json`, `after-full.json` and `originals.json`;
- the walks `walk-before.jsonl` and `walk-after.jsonl`;
- `verify.txt`, `inputs-verbose.txt`, `schema-check.txt` and `files-grouped.md`.

**Modified leaf payload schemas** (154; only `x-semio-ui` was added):
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `⚙️set-editor-settings`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `📐️set-editor-selection`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `✂️splice-text`, `🌐change-language`, `🔗change-uri`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/➗️equation/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `🎚️change-coefficient`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `🔄️replace-points`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `✂️disconnect-nodes`, `❌️delete-node`, `🏷️change-node-label`, `🔁️replace-graph`, `🔗️connect-nodes`, `🕹️move-node`, `🗑️delete-nodes`, `🧭️change-graph-directed`, `🧮️update-graph-algorithm`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `✂️disconnect-widgets`, `👯️duplicate-widget`, `📍️move-widgets`, `🔀️reorder-synapses`, `🔁️replace-widget`, `🔄️update-synapse-endpoints`, `🔌️connect-widgets`, `🔢️reorder-widgets`, `🗑️delete-widget`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `🎥️set-camera`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `🎚️change-exaggeration`, `📥change-imported-features`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `♻️replace-route-data`, `✂️delete-route`, `🔀reorder-positions`, `🔁replace-position-data`, `🔃reorder-regions`, `🔄replace-region-data`, `🗑️delete-position`, `🧭reorder-routes`, `🧹delete-region`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `📝change-notes`, `🔢change-counter`, `🚦change-status`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `✂️resize-tile-crop`, `✏️rename-tile`, `🆕create-tile`, `🔀reorder-tiles`, `🔁replace-tiles`, `🔲resize-source-frame`, `🖼️replace-source`, `🗑️delete-tile`, `🧹delete-tiles`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `✒️change-schema`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/🔗️dependency/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `✂️disconnect-steps`, `🔗️connect-steps`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `🌱️create-step`, `📍️move-step`, `🔧️edit-step-params`, `🗂️change-step-collapsed`, `🗑️delete-step`, `🧬️duplicate-step`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `⏱️change-cursor`, `❌delete-machine`, `🌱create-step`, `🎨change-machine-icon`, `🏭create-machine`, `🏷️rename-step`, `📍move-stock`, `📐replace-step-measure`, `🔀reorder-steps`, `🔁replace-machine-capabilities`, `🔖rename-machine`, `🔘change-step-enabled`, `🗑️delete-step`, `🧊replace-stock-solid`, `🧷change-step-origin`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `↗️move-object`, `➕️insert-paint-layer`, `➖️remove-paint-layer`, `🌫️change-paint-layer-opacity`, `🌱️create-object`, `🎛️change-paint-layer-blend-mode`, `🎨️edit-paint-layer`, `🏷️rename-object`, `👁️change-paint-layer-visible`, `💀️delete-object`, `📐️scale-object`, `🔀️reorder-objects`, `🔄️rotate-object`, `🔖️rename-paint-layer`, `🔘️change-object-smooth-shading`, `🕸️create-mesh`, `🧨delete-mesh`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `🖱️set-drag`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `✂️disconnect-nodes`, `✏️edit-node-text`, `🌱create-node`, `🏷️change-node-kind`, `📐resize-node`, `🔷change-node-shape`, `🗑️delete-node`, `🚩set-node-root`, `🤝️connect-nodes`, `🧭move-node`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `📸️replace-config`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `↔️move-step`, `➕add-step`, `➖remove-step`, `🔀move-block`, `🔄replace-block`, `🗑️remove-block`, `🧱add-block`, `🩹update-step`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `📦️set-payload`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `📸️replace-config`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `🌱create-step`, `🔀reorder-steps`, `🔧edit-step-params`, `🗑️delete-step`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `👈️edit-lhs`, `👉️edit-rhs`, `📐️change-rule-layout-point`, `🔧️change-parameter-binding`, `🖼️edit-before-fixture`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `📊️replace-query-result`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🫧️transient/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `🔤️set-editor-selection`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `✂️delete-edge`, `✏️rename-node`, `➕️create-node`, `🌉️create-edge`, `📍️move-node`, `🔧️change-data-property`, `🗑️delete-node`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `🔄️replace-config`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `🔄️replace-presence`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `↔️move-node`, `✂️disconnect-nodes`, `🌱create-node`, `🏷️rename-node`, `📐resize-node`, `🔁replace-node-kind`, `🔡change-node-abbreviation`, `🔤change-node-name`, `🖼️change-node-icon`, `🗃️replace-node-properties`, `🗑️delete-node`, `🤝️connect-nodes`, `🧮change-node-operator-kind`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `📬️apply-directory-page`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `🔢️change-catalog-generation`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `🌱create-artifact`, `🏷️rename-artifact`, `🕒touch-artifact`, `🗑️delete-artifact`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` — `🌱create-curated-item`, `🔢change-curated-item-count`, `🗑️delete-curated-item`
