# 📓️ W2-R architect — input-UI rollout for 🏛️architect and 📸️remodel

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-R-architect, 2026-09-30. Contract: `📋️design.md` §6,
`🧭️plan.md` "W2-R brief", manifest `$defs/InputUi`. Scope: every mutation leaf payload schema under
`✏️s/🔌️plugins/🏛️architect/` (266 program leaves + the editor `replace-config` / `replace-presence` leaves) and
`✏️s/🔌️plugins/📸️remodel/` (36 remodeling leaves) — 304 files.

## 1. Outcome

**DONE, VERIFIED.**

- The strict lint reports **0 findings in both scopes**, and every input carries a UI descriptor:
  - architect: 332 of 332 inputs, up from 195 of 332;
  - remodel: 56 of 56 inputs, up from 13 of 56.
- There are no root-union `malformed` or `refUnresolved` findings in this scope.
- **Every committed fixture payload validates: 402 of 402.** Before this work, 263 of the 402 failed their leaf schema; §3
  explains why.
- 6,591 `x-semio-ui` annotations validate against the manifest `$defs/InputUi`.
- The strict Ajv oracle shows no regression.

## 2. Census

| owner | leaves | inputs | declared before → after | findings before → after |
|---|---|---|---|---|
| architect | 268 | 332 | 195 → **332** | 137 labelMissing → **0** |
| remodel | 36 | 56 | 13 → **56** | 40 labelMissing + 3 optionLabelMissing → **0** |

The lint reports only the first refusal per top-level input. Every nested record field was annotated, so the reader
accepts each whole input tree, not just its first field.

## 3. Architect leaf schemas were untrue; they now match the Rust payload

Before any annotation was possible, the architect leaf schemas had to be corrected: 263 of the 402 committed fixtures, which
are Rust-serialised payloads, failed them. Annotating the old schemas would have produced select options, required flags and
id widgets that the Rust `FromValue` decoder rejects.

`🧪️w2-r-architect-program.py` therefore rebuilds each record input from three sources:

- **Structure** (`required`, nullability, nesting) comes from the snapshot `$defs` of `🧬️schema/📸️snapshot/🔣️.json`. All 131
  committed record payloads validate against those `$defs`.
- **Enum values** come from the Rust enums (`rename_all = "camelCase"`).
- **Integer types** come from the Rust field types. `u32` and `u64` become `integer` with `minimum 0`.

The payload structure (the Rust types) is unchanged; only its description was corrected. The corrections made:

| correction | count |
|---|---|
| `Option<T>` field typed non-null → `anyOf [T, null]` | 1,234 |
| PascalCase enum values (`"Draft"`) → Rust camelCase (`"draft"`), including `constructionDocuments` | 400 |
| `required` lists (the header `tags`/`notes`/`description` are optional; `Option` fields without `skip` are required) | 257 |
| `EntityId` typed `integer ≥ 0` → `string` (every rename/delete/disconnect `id`, and the Governance ids and `authorIds`) | 134 |
| `create-activity` / `replace-activity`: the `activity` input was a UI state enum (`Waiting/Loading/Idle/Finished`) → the real `Activity` record | 2 leaves |
| certain hard bounds added (`latitude` −90..90, `longitude` −180..180) | 4 |

Two things were left unchanged:

- The editor `replace-config` and `replace-presence` leaves and all remodel leaves already matched Rust, so they received only
  annotations.
- The remodel `originLat` / `originLon` gained the same hard bounds that its `update-geo-params` diff already enforces.

## 4. Annotation rules (design §6)

### Labels and descriptions

- Every input has a hand-written en/de label using architectural-programming terminology:
  - DIN 18205 Bedarfsplan
  - DIN 276 Kostengruppe
  - DIN 277 Fläche/Rauminhalt
  - DIN 18040 lichte Breite/Bewegungsfläche/Rampenneigung
  - DIN 4109/18041 Akustikklasse
  - DIN EN 16798-1 Behaglichkeit
  - DIN EN 17037 Tageslicht
  - DIN EN 1998-1/NA Erdbebenzone
  - TRGS 510 Lagerklasse
  - BauNVO GRZ/Art der baulichen Nutzung
  - RACI roles: Verantwortlich, Entscheidungsbefugt, Konsultiert, Mitwirkende
  - HOAI-aligned phases: Vorplanung, Entwurfsplanung, Ausführungsplanung, Vergabe, Bauausführung
- Some labels follow the field's parent, for example:
  - `ProgramElement.level` → Geschoss
  - `ProgramElement.volume` → Rauminhalt
  - `ApprovalRecord.conditions` → Auflagen
  - `Risk.probability` → Eintrittswahrscheinlichkeit
- Descriptions are written where a name is not self-explanatory.
- Every date string without its own description gets an ISO 8601 hint.

### Enums

- All 32 Rust enums have en/de option labels. The script asserts that each label set equals the Rust variants.
- Enum-valued arrays carry their `options` on the `items` node.
- The existing `analysis_kind_picker_options` and `report_kind_picker_options` labels were reused.

### References

- Every `EntityId` input is `{widget: reference, role: target, ref {kind, domain: "program", granularity: "entity"}}`. The
  program interaction domain has a single `entity` granularity.
- The kind comes from an explicit table covering all 292 `EntityId` fields: `stakeholder`, `programElement`, `requirement`, …,
  or `entity` for generic subject references.
- An entry's own `id`:
  - is the target reference in `replace-*`;
  - is identity text in `create-*`, `connect-*` and the singleton facets;
  - is the target reference in rename/delete/disconnect.

### Numbers

- Units come from the Rust `#[dsl(unit = …)]` attributes and from field names: m, m², m³, kg, kW, dB, ms, %, d, a, and °.
- Steppers carry step and precision.
- `progressPercent` is a 0–100 slider.
- Money fields carry no unit, because the currency is a sibling field.

### Remodeling (`🧪️w2-r-architect-remodeling.py`)

- **Terminology** is German photogrammetry usage:
  - Passpunkt, Passpunktmessung
  - innere and äußere Orientierung
  - Bündelblockausgleichung
  - DOM/DGM, Orthofoto
  - WGS 84 for the origin, because the geo engine uses WGS84
- **Soft ranges** of the parameter sliders are taken from the editor's own `setXxxParams` action sliders.
- **Vectors:**
  - quaternion wxyz, dims 4;
  - translation in m, dims 3;
  - pixel position in px, dims 2.
- **References:**
  - asset ids are `{kind: asset, domain: assets, granularity: asset}`;
  - streams, cameras, ground control points and content have a kind but no domain, because no selection domain exists for
    them.
- **Hidden** (still labelled): packed base64 blobs, the composed mesh child handle, and the derived watertightness and QC
  report fields.

## 5. Verification (all run, all green)

| command | result |
|---|---|
| `bun ./📜️script.ts schema mutation-inputs --under "✏️s/🔌️plugins/🏛️architect"` (cwd `🧪️test`) | exit 0, 332/332, 0 findings |
| `bun ./📜️script.ts schema mutation-inputs --under "✏️s/🔌️plugins/📸️remodel"` | exit 0, 56/56, 0 findings |
| `.venv/bin/python 🧪️w2-r-architect-check-fixtures.py` | exit 0. It uses Python `jsonschema` Draft 7 with `referencing`. Every committed `🦠️mutation` fixture validates against its leaf schema: **402/402**, up from 139/402. All **6,591** `x-semio-ui` annotations validate against `$defs/InputUi` with 0 errors. |
| `bun 🧪️w2-r-architect-check-inputs.ts --before 🗑️generated/w2-r-architect/leaf-schemas-before.json` | exit 0. The W1-D TS reader reads 304 leaves and 388 inputs with 0 failures. The strict Ajv oracle (`semioSchemaAjvV1`, `x-semio-ui` registered) compiles 274 leaves now, the same 274 as before: **0 regressions**. |
| `bun ./📜️script.ts schema --under <scope> --json` (contract gate) | **0 findings** on any mutation leaf in either scope. The findings elsewhere are pre-existing: snapshot exports, window config dialects, catalog rows for window configs. |
| script idempotence | re-running both scripts produces byte-identical files |

Rust was not compiled, because the brief forbids cargo. The derive only `include_str!`s the leaf schema and does not parse it at
compile time, so the edit is content-only for Rust.

## 6. Findings outside this WP (not fixed)

1. **Snapshot enum corruption.** `DeliveryPhase` reads `constructionArtifacts`, but the Rust variant is
   `ConstructionDocuments`, i.e. `constructionDocuments`. This is fallout from the documents→artifacts sweep. It appears in:
   - `🧬️schema/📸️snapshot/🔣️.json:7583`;
   - the combined `🧬️schema/🔣️.json:7583`;
   - the TS twin `🧬️schema/🟦️.ts:865`;
   - and it is baked into the built wasm.

   The leaf schemas now carry the Rust value. The snapshot `$defs` also type `u32`/`u64` fields as plain `number`.
2. **Architect aggregate union rejects tagged ops.** `🧬️mutations/🔣️.json` is a `oneOf` of leaf `$ref`s, but the architect
   leaves declare no `mutation` const while every committed op carries one. With `additionalProperties: false`, the aggregate
   rejects every tagged op. Puzzle 2d leaves carry the const. This is structural; the owner or a W1-D follow-up should
   decide.
3. **Remodel strict-Ajv formats.** 30 of the 36 remodel leaves do not compile in the strict Ajv oracle, before and after this
   work. The cause is the unregistered schemars formats `uint32`, `uint64`, `float`, `double` and `base64`. Either the strict
   format vocabulary registers them or the formats are dropped.
4. **Central chores.**
   - `📚️library/🔣️schema-catalog.json` hashes are stale for the 304 edited leaves. A single `schema generate` should run after
     all W2-R groups land.
   - The architect and remodel plugin descriptors and roster `inputs` need the central `describe` regeneration, which is W1-D
     open item 1.
5. **Presence nullability.** `replace-presence.adjacencyKindFilter` is `Option<AdjacencyKind>`, but its schema is
   non-nullable. The committed fixtures pass. If `ToValue` emits `null` for `None`, a payload with no filter would be refused.
   That leaf schema is generated by `ArtifactSchema`, so it was left alone.

## 7. Files

- **Modified:** 304 leaf payload schemas:
  - `✏️s/🔌️plugins/🏛️architect/…/✳️any/🧬️schema/🧬️mutations/*/*/🧬️schema/🔣️.json` (266);
  - `…/✏️editor/{🎚️config,👥️presence}/🧬️schema/🧬️mutations/📸️replace-*/🧬️schema/🔣️.json` (2);
  - `✏️s/🔌️plugins/📸️remodel/…/✳️any/🧬️schema/🧬️mutations/*/🧬️schema/🔣️.json` (36).
- **Created, ticket-root input scripts (kept):**
  - `🧪️w2-r-architect-program.py`: truth plus annotation for architect, including the label, option, reference and unit
    tables;
  - `🧪️w2-r-architect-remodeling.py`;
  - `🧪️w2-r-architect-check-fixtures.py`;
  - `🧪️w2-r-architect-check-inputs.ts`.
- **Generated, in `🗑️generated/w2-r-architect/`:**
  - `leaf-schemas-before.{tar,json}` (the pre-rollout originals);
  - census and strict-lint outputs before and after;
  - `fixtures-{before,after}.txt`, `inputs-after.txt`, `test-schema-*-after.txt`;
  - analysis dumps.
