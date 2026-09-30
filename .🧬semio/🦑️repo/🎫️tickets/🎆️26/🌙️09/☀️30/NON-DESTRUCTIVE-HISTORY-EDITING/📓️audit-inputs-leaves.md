# 📓️ Audit — input descriptors (W1-D) and puzzle 2d leaves (W1-F)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Read-only audit (no source edit, no cargo). Sonnet auditor, first pass 05:10–05:40,
resumed 06:50–07:05 after the coordinator's session reset. Contract: `📋️design.md` §6, §8 and `AGENTS.md`. Inputs: `📓️w1-d-report.md`
(§1–§7, including the follow-ups: root unions, cycle guard, integer refs, collecting lint, hidden stop, color, vector facets, os.store id
split, runtime stdio registry), `📓️w1-f-report.md`, and, because it changes the picture, `📓️w2-s-report.md`.

Everything below was re-checked against the tree as of about 06:58 (manifest `🦀️.rs`/`🟦️.ts` last written 06:18). The reader files are
still moving (line numbers shifted by ~70 between my two reads), so line references are "at audit time".

Abbreviations: `M` = `🧰️framework/🔨️modules/🛂️manifest`, `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`, `PZ` =
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any`, `MUT` = `PZ/🧬️schema/🧬️mutations`, `T` = this ticket folder,
`SCR` = `T/🗑️generated/audit-inputs-leaves` (my scratch: scanners, censuses, probes).

## 0. Verdict

The reader, the corpus and the puzzle 2d leaves are in good shape. Rust and TypeScript agree rule by rule, the leaf arithmetic matches an
independent recomputation, and the leftovers of the deleted shapes are gone from code. The weak points are at the seams:

- The derive's `payload_value` works on the wrapped leaf **type**, and that type is not always the type the payload schema describes (§1.1).
- The two safety nets meant to catch that (the payload-parity lint and any derive-level law) do not look at the affected leaves (§2.1).
- The descriptor vocabulary cannot say "nullable" (§2.2).
- Consumers cannot address everything the reader emits (§2.3, §2.4).

| # | Rank | One line |
|---|---|---|
| 1.1 | **critical** | Enum-payload leaves (`phase`/`value`): `payload_value`/`with_payload_value` act on the enum, the schema describes the inner payload. 20 leaves flat-schema (svg 9, xml 6, json 5), 140 leaves expose a dead `Restore` editor. |
| 2.1 | major | Payload-parity lint reports success on zero fixtures for whole artifacts; no derive-level round-trip law beyond the counter fixture. |
| 2.2 | major | `ActionArgDef` cannot express nullable / clearable inputs (331 top-level inputs in 255 leaves, 18 of 36 puzzle 2d leaves). |
| 2.3 | major | `time_travel_input_at` cannot address item fields of arrays of objects and is variant-blind; union descriptors reuse ids (`/value` twice). |
| 2.4 | major | `HostMutationRosterEntry` (host mirror) has no `inputs`; the roster inputs never reach the host. |
| 2.5 | major | 57 committed plugin descriptors (216 × `"kind": "vec3"`, plus the `.semio` twins) are stale; run bootstrap decodes them. Known, owned by the coordinator. |
| 2.6 | major | Input-UI census regressed for norm: 0 → 852 findings (W2-S-A rewrites schemas and drops `x-semio-ui`). |
| 2.7 | major | Hard bounds in the puzzle schemas are stricter than the Rust diffs; only the three new leaves enforce `mutation.invariant`; no invariant fixture; the Python twin has no invariants. |
| 2.8 | major | `snapSource` (100+ puzzle inputs) has no consumer in any host. |
| 3.x | minor | Glossary German (about 25 entries), reader edge cases, perf, doc drift, AGENTS.md nits, three `[DEBUG]` lines in the puzzle editor. |

Closed since my first snapshot (no action, listed so nobody re-opens them): the schema ↔ payload key drift (283 leaves) is fixed by W2-S
and guarded by its lint, `x-semio` keys are gone from the glTF leaves, `snapshot-patch`/`os.store` ids are split, the runtime resolves the
stdio registry (§5).

## 1. Critical

### 1.1 Enum-payload leaves: accessors and schema describe different values

**Evidence.**

- The derive wraps whatever type the aggregate variant holds: `OS/🗣️dsl/✨️derive/🦀️.rs:1746` (`ToValue::to_value(payload)`) and `:1747`
  (`<#payload_ty as FromValue>::from_value(value)`).
- For 140 stdio leaves (glTF 120, svg 9, xml 6, json 5) that type is an adjacently tagged enum, not the payload struct.
  Example: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/✍️set-text/🦀️.rs:11-27`.

  ```rust
  #[value(rename_all = "camelCase")] pub struct SetTextPayload { path, text }
  #[value(tag = "phase", content = "value", rename_all = "camelCase")] pub enum SetTextMutation { Apply(SetTextPayload), Restore(SvgDiff) }
  ```

  The aggregate holds the enum (`…/🎨️svg/…/🧬️mutations/🦀️.rs:28`, `SetText(SetTextMutation)`). Both types share one `🧬️schema/🔣️.json`.
- `payload_value()` of an `Apply` op is therefore `{"phase":"apply","value":{"path":…,"text":…}}`.
  The `set-text` schema still documents `SetTextPayload` (title `SetTextPayload`, properties `path`, `text`).
- So the reader emits inputs `/path`, `/text`, but the value to edit has `phase`/`value`. `draft_time_travel_input` sets `/text` beside
  `phase`/`value` (`OS/🔌️plugin/⏪️time-travel/🦀️.rs:1080-1120`). The candidate is schema-invalid (`additionalProperties: false`, `path` missing)
  or, if the validator did not compile, is refused later by the enum's `from_value`. Every draft is refused, so these leaves cannot be edited.
- State now (`SCR/scan_leaves` style recount, 06:57):

  | Shape of the enum leaf's schema | Leaves | Verdict |
  |---|---|---|
  | root `oneOf` on `phase` (glTF, converted by W2-R stdio-b / W2-S-B1) | 120 | matches `{phase, value}` |
  | flat `properties` of the inner payload (svg 9, xml 6, json 5) | 20 | **mismatch** |

- The 120 glTF leaves match the value shape, but a `Restore` op (made by every rebase/inverse) is "editable" too: `time_travel_editable`
  is only `!may_emit_foreign_steps && input_schema().is_some()` (`⏪️time-travel/🦀️.rs:644-646`). It opens an editor whose only live input is the
  `/phase` selector; `restore` → `apply` yields `{phase:"apply", value:<diff>}`, which is refused. It is a dead editor.

**Why nothing caught it.** See 2.1: svg/xml/json have no `🦠️mutation/🔣️.json` fixtures, so the parity lint sees `0/0` (measured).

**Fix (concrete).**

1. Give the leaf a hook instead of letting the derive assume the wrapped type is the payload. Add to `MutationKind<P, Op>`
   (`OS/📡️spr/🎮️command/🦀️.rs:219`) three defaulted methods: `fn payload_input_schema(&self) -> Option<&'static str>` (default: the leaf's
   `PAYLOAD_SCHEMA`), `fn payload_input_value(&self) -> DslValue` (default: `to_value()`), and
   `fn with_payload_input(&self, DslValue) -> Result<Self, ValueError>` (default: `from_value`).
   The derive forwards to them (`derive/🦀️.rs:1745-1748`, `:1831-1840`).
2. Override in the 140 enum leaves: `Apply(p)` → `p.to_value()` / rebuild `Apply`; `Restore(_)` → `input_schema() == None`, so it is not editable.
   The `oneOf(phase)` schemas of the 120 glTF leaves then go back to the payload, which also removes 120 `hidden`/`any` `/value` duplicates.
   Alternative (weaker): convert the 20 flat schemas to `oneOf(phase)` like glTF. That exposes `Restore` as a diff editor, so the hook is preferred.
3. Add the law in 2.1 so this cannot recur.

## 2. Major

### 2.1 The safety nets are vacuous where the risk is

**Evidence.**

- `bun ./📜️script.ts schema mutation-payloads --under ✏️s/…/🗄️stdio/🗿️artifacts/🧾️json` prints
  `0/0 fixture payload(s) of 15 leaves … 0 finding(s)`. The same for `🎨️svg` (0/0, 29 leaves) and `📰️xml` (0/0, 15 leaves).
  A "0 findings" exit for a directory whose fixtures it never read is a false green.
- Repo-wide stdio: 476 fixtures for 988 leaves, so at least 512 stdio leaves have no witness.
- The lint's own note admits the limit ("the fixture is the witness"). Its layout model cuts `payload_value` from the *aggregate* wrapper
  (external/adjacent/internal) and never models a leaf-level wrapper such as `phase`/`value`.
- The only derive-level test is the counter fixture (`OS/📡️spr/🎮️command/🧪️tests/🧪️mutation-payload/🦀️.rs`, 2 tests).
  Nothing runs `with_payload_value(payload_value())` over a real aggregate.

**Fix.**

- (a) Make a lint that sees no fixture for a leaf report it: `unwitnessed` per leaf, with a ratchet count. Strict mode fails on a directory with
  leaves and zero fixtures.
- (b) Add the derive-level law to the shared testkit: for every aggregate, for each op of `demo_mutation_cases()` (every stdio crate has one):
  1. `schema = op.input_schema()?`;
  2. `instance = mutation_input_instance(schema, resolver, op.payload_value())`;
  3. validate `instance` with `OwnedJsonSchemaValidator`;
  4. assert `op.with_payload_value(op.payload_value()) == op`.
  This is language-agnostic in effect (the schema is the contract) and independent of fixture layout.
- (c) Static check (fixture independent): a leaf whose Rust payload type is an enum with `tag/content` must have a root `oneOf` on that tag, or
  the hook of 1.1.

### 2.2 The descriptor vocabulary has no "nullable"

**Evidence.**

- Rust `input_type` and TS `inputType` drop `"null"` from `type: [X, "null"]` and from `oneOf/anyOf` with a bare `{"type":"null"}`
  (`M/🦀️.rs` `input_type` ~1206, `resolve` ~1374; `M/🟦️.ts` `inputType` ~690, `resolve` ~783). `ActionArgDef` then carries only `required`.
- Absent (omit) and `null` (clear) collapse into "optional". Counted over the leaf schemas: **331 top-level nullable inputs in 255 leaves**
  (plus nested). In puzzle 2d, 18 of 36 leaves (`newText`, `newRoot`, `newVisible`, `newLocked`, `newScale`, `newRadius`, …).
- The only place that says "empty resets" is prose: `Der Größenfaktor …; leer setzt auf 1 zurück`. A toggle cannot send `null`, a slider cannot
  either, and a stepper cannot either.

**Fix.** Add `nullable: bool` to `ActionArgDef` (not to `ArgSchema`: it modifies the slot, and the wire stays `skip_serializing_if`), set by the
reader when a `null` branch was stripped. Mirror it in TS, add corpus cases (`nullable-scalar`, `nullable-enum`, `nullable-vs-optional`), and
give hosts a "clear" affordance for `nullable` inputs. The session's `time_travel_coerce` then maps the clear action to `DslValue::Null`.

### 2.3 Reader output that the session cannot address

**Evidence** (`⏪️time-travel/🦀️.rs:499-515`, identical in my 05:10 and 06:57 reads):

- `time_travel_input_at` finds `/handles/0/angle` as: `handles` (`Array`) → `"0"` parses → `segments.next().is_none()` is false → `None`.
  Only one index segment is accepted, so fields inside array items are unreachable, although the reader builds `Array { items: Object { fields } }`
  for them (puzzle `create-node.handles[]`, `add-node-handle`, `replace-kind-catalogs`, norm and architect lists).
- `inputs.iter().find(|input| input.key() == first)` is group-blind. A union reuses ids across variants:
  `variant_inputs` copies every member's fields with `group = variant`. Reproduced on glTF `create-node`
  (`SCR` probe `gltf_dump`): `/phase`, `/value` (group `apply`, object), `/value` (group `restore`, hidden `any`), so the id `/value` appears twice.
  Whichever variant the op currently is, the first descriptor wins.

**Fix.** Declare `(id, group)` the key of a descriptor (an id is a payload pointer, so it cannot be made unique by decoration) and make the resolver take
the current value of the union selector, so it picks the group of the variant the op is in. Make the array arm recurse: index segment, then, for
`ArgSchema::Array { items: Object { fields } }`, the field named by the next segment, returning that field's descriptor (`element = false`). Add both
to a session corpus case (union `restore` op, `/handles/0/angle`).

### 2.4 The roster's `inputs` stop at the guest

**Evidence.** `WireMutationRosterEntry.inputs` (`OS/🔌️plugin/🦀️.rs:4837`) is guest side. The host mirror
`HostMutationRosterEntry` (`OS/🔌️plugin/🖥️host/🦀️.rs:7075-7089`) still lists `mutation_id, verb, entity, kind, record, contributor,
artifact_kind`, and claims to be "field-for-field JSON-shape-identical". It has no `deny_unknown_fields`, so the guest's `inputs` are silently
dropped on decode. `🏃️run` (`OS/🏃️run/🦀️.rs:1791-1807`) builds host rows from the descriptor, without inputs either. Design §6 says roster rows
carry inputs; today only the guest-side wire does, and no consumer reads it.

**Fix.** Either add `inputs: HostMutationInputs` (a mirror of `WireMutationInputs`, or re-export the manifest types the host already depends on:
`ActionArgDef` and `InputSchemaError` live in `semio_framework`) and fill it from the descriptor; or state in design/docs that inputs are only for the in-plugin
session and drop the roster field (and its per-boot cost).

### 2.5 Stale committed plugin descriptors

**Evidence.** 57 `✏️s/🔌️plugins/**/🔣️.json` still carry 216 × `"kind": "vec3"` (git-grep at 06:57; e.g. `🎥️shooting/🔣️.json:4303`), string
`description`s and the old shooting `ConfigFieldSpec` shapes; the `🛂️.descriptor.semio` twins are stale too. `🏃️run` decodes the committed
descriptor at boot (`OS/🏃️run/🦀️.rs:1694-1704`, `🏗️bootstrap/🦀️.rs:122-138`), and `ArgSchema` no longer has a `vec3` tag. W1-D open item 1 and W2-S item 4
already list it; it is a merge gate rather than a code fault.

**Fix.** Coordinator: `describe` regeneration per plugin, then the freshness test (`bun nx run @semio-tech/plugin-registry`) before Wave 3.

### 2.6 Input-UI census regressed under the parity rewrite

**Evidence.** `bun ./📜️script.ts schema mutation-inputs --census` (06:56): **5130/5269 inputs, 857 findings**; W1-D reported 5317/5323 and 72
findings at 06:10. Owner `norm`: 548 leaves, 837 inputs, 703 declared, `labelMissing` 701, `optionLabelMissing` 32, `refUnresolved` 55
(`s/norm/din16798/snapshot.json`), `leafUncatalogued` 64. All other 34 owners: 0, except stdio 3 and wfc 2 (`os/store/{link,child}`,
catalogue staleness, W2-S item 1). Sample: `s.norm.din18599.mutation.change-attachment — optionLabelMissing … option Detached has no label`.
The norm parity executor (W2-S-A, still running) re-derives schemas from Rust and drops the W2-R `x-semio-ui` annotations.

**Fix.** Order matters: run the parity rewrite first, then re-run `🧪️w2-r-norm-annotate-inputs.py`; or teach the parity tool the
"keep every `x-semio-ui`" rule that W2-S's aggregate rule already has. Gate on `schema mutation-inputs` = 0 for norm before closing.
The 64 `leafUncatalogued` leaves need real `$id`s (en1990/en1991), then the `schema generate`.

### 2.7 Puzzle schemas versus Rust behaviour, invariants and fixtures

**Evidence.**

- W1-F added hard bounds to existing leaves: `exclusiveMinimum: 0` on `newScale`, `newRadius`, `newWidth`, `newHeight`, node `radius/width/height/scale`,
  handle `radius/scale`; `minItems: 1` and `uniqueItems` on `targets`. Only the three new leaves emit `mutation.invariant`
  (`git grep invariant` over `MUT` → `drag/rotate/scale-selection` diffs only). `scale-node/🔺️diff/🦀️.rs:6-18` accepts `newScale ≤ 0`;
  `move-node/🔺️diff/🦀️.rs:6-19` accepts NaN. This contradicts the "move/drag/rotate/scale ⇒ Fatal non-finite/non-positive" family rule W1-F cites.
  A duplicated target is fine for Rust (deduplicated) but invalid for the schema.
- The new fixtures cover applied, mixed, partial, target-missing and no-op per leaf (15 quintets) but no `mutation.invariant` vector.
  A `scale-selection` with `factor: 0` or `-1` is representable in JSON.
- The Python second implementation `PZ/🧪️tests/◻️mutate-puzzle-2d-1/🐍️.py:346-370` (`selection_transform`) has no positivity or finiteness check, so it would
  silently transform with `factor = -1`; it agrees with Rust only on the committed vectors.
- Fail-open validation: `time_travel_editor` compiles the validator with `.ok()` and `draft_time_travel_input` treats `None` as valid
  (`⏪️time-travel/🦀️.rs:~1046-1050`, `:~1111-1113`). A schema the owned validator cannot compile silently drops every hard bound.

**Fix.** Make the code follow the schema (schema-first): add `Fatal mutation.invariant` to `scale-node`, `replace-node-geometry`, `move-node`, `move-target-region`,
`replace-node-handle` for non-finite and (where the schema says so) non-positive values, and add fixtures `<leaf>/🚫️rejects-a-non-positive-factor`
(`factor: 0`, `-1`) for scale-selection. Add the invariant branch to the Python twin. Fail closed when the validator is `None`.

### 2.8 `snapSource` has no consumer

**Evidence.** `git grep snapSource|snap_source` over `OS/📺️renderer` and `OS/🔌️plugin` finds nothing outside the manifest reader and tests. W2-B implements
`snaps` for slider/dial only (`📓️w2-b-report.md:135`). Every puzzle position input (`dx`, `dy`, `pivotX/Y`, `x`, `y`, `newX/Y`, extents; 60+ inputs) declares
`snapSource: {config: "gridFactor"}` (design §8: "dx/dy snap to the grid via snapSource"). The config key exists (`PZ/✏️editor/🎚️config/🦀️.rs:104`, `grid_factor`),
but nothing resolves `{config: key}` against a window config.

**Fix.** W2-B/W2-C: resolve `SnapSource::Config{key}` from the window config, `Step` from `step`, `Snapshot{pointer}` from the previewed document, in the
stepper and dial. Add a vitest/Rust case with the grid factor 0.5.

## 3. Minor

### 3.1 Glossary German (`M/🔣️input-labels.json`, 261 entries, all read; about 25 flagged)

Wrong or misleading:

| Key (line) | en → de | Issue and suggestion |
|---|---|---|
| `tag` (904) | Tag → **Tag** | German "Tag" is "day". Puzzle uses "Schlagwörter". Use `Schlagwort` / `Markierung`. |
| `before` (68) | Before → **Vor** | Not a label. `Davor einfügen` / `Vorheriges Element`. |
| `accessor` (4) | Accessor → **Accessor** | Untranslated. `Zugriffsfunktion`. |
| `scale` (792), `newScale` (520) | Scale → **Maßstab** | "Maßstab" is a map ratio. Puzzle labels say "Skalierung". Use `Skalierung` / `Neue Skalierung`, or `Größenfaktor` for node scale. |
| `newX`/`newY` (584…) | New X → **Neues X** | Odd gender. `Neue X-Position` / `Neue Y-Position`. |
| `frame` (224) | Frame → **Rahmen** | Wrong for gif/animation frames (`Einzelbild`). The key is ambiguous across domains. |
| `section` (812) | Section → **Querschnitt** | Right for en1993, wrong for layout/writer sections (`Abschnitt`). Global mapping, domain-specific wording: annotate those leaves instead. |
| `caseId` (96) | Load Case ID | The English label assumes structural load cases. |

Collisions (two keys, one label; a form shows two identical labels):

| Keys | Label |
|---|---|
| `turn` (972) / `rotation` (776), `newTurn` (556) / `newRotation` (516) | `Drehung` / `Neue Drehung` (puzzle already overrides `turn` → `Wendung`) |
| `gap` / `newGap` (440) / `newSpacing` (536) | `Abstand` / `Neuer Abstand` |
| `entry` (200) / `item` (296) | `Eintrag` |
| `block` (76) / `chunk` (116) | `Block` (`chunk` → `Datenblock`) |
| `at` (52) / `position` (712) | `Position` |

Inconsistent register: `child` (104) → `Kindelement` but `childId` → `Kind-ID`; `primitive` (720) → `Primitiv` (`Grundkörper`); `patch`, `snapshot`, `asset`,
`widget` stay English with no stated rule; `fromPort`/`toPort` en "From/To Port" but de "Quellanschluss/Zielanschluss" (fine, but en drifts). The
snake_case duplicates (`child_id`, `mesh_id`, `new_name`, `node_id`, `step_id`, `stream_index`, `primitive_id`) were added because the wire keys drifted; W2-S
fixed the keys, so the aliases can go once no leaf uses them. The file has 261 names, the W1-D report says 258.

Fix: one pass using the two tables above as the worklist; add a Rust/TS test that no two glossary keys share a de label unless listed in an explicit
allow-list (`entry`/`item`, `at`/`position`), so collisions cannot creep back.

### 3.2 Reader details (parity holds; these are edges shared by both languages)

- `ArgSchema::Vector` drops an exclusive component bound (`min.filter(|_| !min_exclusive)`, TS `component.minExclusive ? undefined`) and ignores the item's own
  `x-semio-ui` (both languages), unlike plain numeric arrays which inherit and let the item win. A vector of positives lets the UI offer `0`, which the schema then
  refuses at draft time. Add `min_exclusive`/`max_exclusive` or document.
- `default` on a `$ref` sibling is lost: `resolve` replaces the node by the target, so `input.node.get("default")` reads the target's default only.
- An `$anchor`-style `$ref` (`#foo`) resolves to the document root in both readers (`fragment.split('/').skip(1)`); reject a non-pointer fragment as `refUnresolved`.
- A union variant's author-declared `group` is overwritten by the variant value (`field.group = Some(variant.value)`).
- Name-based reference inference (`<x>Id(s)`) will turn any string id field (`pluginId`, `requestId`, …) into a `Reference` with no `domain`. It is only lint-visible
  through the census; restrict inference to kinds the interaction declares, or require `ref`.
- `mutation_input_instance` re-parses the whole schema on every draft (each slider tick). `registered_input_schema_document` scans every registered JSON
  schema with `text.contains(id)` and parses each hit until `$id` matches (`M/🦀️.rs:1146-1155`). Cache the reader and an `$id → text` index in the editor/registry.
- The meta-schema `InputLabelLocales` hard-codes `["en","de"]` (`M/🧬️schema/🔣️.json`) while `SHELL_LOCALES` is generated from the ui axes: two places to change for a third locale.
- Glossary is loaded with `.expect` (`M/🦀️.rs` `input_label_glossary`); a bad glossary would trap a wasm plugin. It is test-guarded, so this is only hygiene.

### 3.3 Puzzle 2d leaf notes (semantics are correct; see §4 for the evidence)

- Labels print numbers with `.` (`Faktor 0.5`); German should use `,`. `puzzle2d_selection_number` (`MUT/🦀️.rs:257-261`) is locale-blind.
- `scale-selection` calls its pivot "Bezugspunkt X/Y" but the factor description says "vom **Drehpunkt** aus" (`Spreizt Positionen vom Drehpunkt aus`).
- `replace-edge-geometry`/`connect-handles` `rotation`, `turn`, `tilt` have no unit or description; if they are angles they want the dial like `angle`.
- `replace-kind-catalogs.newCatalogs` reads to a 5-level object tree (nodes → representations, handles, attributes, authors); mark it `widget: hidden` or edit it as JSON.
- A node id that equals a region id always classifies as the node (`puzzle2d_selection_diff`, first tuple arm); ids are namespaced by minting prefix, so this is theoretical.
- `resize-target-region` allows 0/negative extents on purpose (brush strokes); the annotation is honest, the description should say so.
- Three `[DEBUG]` `eprintln!` lines are in the puzzle editor hot paths (`PZ/✏️editor/🦀️.rs:1862, 2046, 3018`: `transform_selection`, `render_body`, `dispatch_emit`): W2-D's, in flight, must be gone before close.

### 3.4 Docs and AGENTS.md

- Design §6 still says `ArgSchema::Reference { kind, … }`, `payload_schema()` on `SemanticMutation`, `id` = the JSON pointer. Landed: `kinds` (serde tag collision), accessors on
  `Mutation<P>` named `input_schema`/`payload_value`/`with_payload_value`, and `id` = single-segment pointer relative to the parent (`/id` inside `/handle`). Also landed and not in
  §6: `ReferenceIdType`, `Vector` facets, `ArgPresentation::Color`, `InputSchemaAudit`. Update §6 (and add `nullable` once 2.2 lands).
- Docstring emojis repeat inside the reader region (`🚫️` ×2, `📚️` ×2, `🧭️` ×3, `🔀️` ×3, `🧬️` ×3, `🧺️` ×3); AGENTS.md wants unique ones.
- Two JSDoc comments sit inside the body of `inputSchemaReader` (`M/🟦️.ts:757` `recover`, `:941` `guarded`), which the rule "no comments inside definitions" forbids
  (move them above the function as one docstring or into a region header).
- `time_travel_editor` uses `.expect("an editable operation declares its input schema")` on a plugin path; it is guarded today, but return `NotEditable` instead.
- Implementation of `PluginBuilder::schema_documents` checks only "same plugin or a direct dependency" (`OS/🔌️plugin/🏗️builder/🦀️.rs:712-716`) while its docstring says "whose
  artifacts this guest hosts". Align the two; and note three components (stdio, image, media) now embed the whole registry JSON.

## 4. What I checked and found correct

### 4.1 Reader Rust ⇄ TS (`M/🦀️.rs` region `🔖️MutationInputs` ↔ `M/🟦️.ts` region `🔖️MutationInputs`)

| Rule | Rust | TS | Result |
|---|---|---|---|
| `$ref` resolution (local `#/…`, foreign by `$id`, RFC 6901 unescape, 32-hop cap) | `target` ~1273, `resolve` ~1374 | `target` ~766, `resolve` ~783 | identical, including the fragment-without-slash quirk (3.2) |
| Cycle guard | `guarded` ~1334, keys `document#fragment`, seeded with `#` | `guarded` ~942 | identical; the direct `$ref: "#"` and `Node→children→Node` are corpus cases |
| Nullable union unwrap | only `{type:"null"}` alone, one concrete branch | same | identical |
| `allOf` merge, depth 32 | `members` | `members` | identical |
| Discriminated root union | `variants`/`variant_inputs` | same names | identical; selector Segmented ≤ 4 else Select; `group` = variant |
| Inference (`<kind>Id(s)`, `new` prefix, string-only) | `input_inferred_reference_kind` | `inputInferredReferenceKind` | identical |
| Number hard bounds (integer exclusives fold, `floor+1`/`ceil-1`) and snap/soft/scale/precision/displayFactor rules and their error order | `input_number` ~1751 | `number` ~855 | identical order |
| Vector facets, `color` (3/4 comps within 0..1), hidden stop | `schema()` ~1622, `input()` ~1548 | `valueSchema` ~1024, `input` ~1097 | identical |
| Labels/locales (`{en,de}` or `{native,reuse}×{en,de}`, every cell) | `input_localized_text` | `inputLocalizedText` | identical |
| Collecting mode (`recover`, dedupe, first finding == fail-fast error) | `mutation_input_audit` ~1121 | `mutationInputAudit` ~724 | identical |
| `argControl`/`control()` | `control` ~830 | `argControl` ~587 | identical, including Color and Vector facets |
| Reference id conversion (`id_value`, `reference_id_text`, ±2^53−1) | `M/🦀️.rs` ~340-380 | `referenceIdValue/Text` | corpus table of 13 rows, both agree |

Verified by running (foreground): `bun test M/🧪️tests/🧪️mutation-inputs/🟦️.ts` **78/78**; `.venv/bin/python …/🐍️.py` **144 jsonschema verdicts over 35 cases + 13 reference ids**;
`schema mutation-payloads --census` **2719/2757 clean** (76 findings: norm 64, wfc 4, stdio 2, rest 0); `oracle exhaustive` for `◻️mutate-puzzle-2d-1` **142/142** (Python second implementation);
W1-F's `🧪️w1-f-check-puzzle2d-inputs.ts` **103 inputs, 0 failures**. Rust cargo tests were not run (rule); the Rust reader was compared by reading and through the shared corpus.
The corpus is thin at the edges (see 3.2), and a repo-wide Rust-vs-TS read of all ~2,900 real leaf schemas does not exist: only the TS lint reads them. Add a Rust test that
emits the canonical descriptors of every catalogued leaf and compare with the TS lint's output.

### 4.2 Derive accessors, six aggregates

`with_payload_value(payload_value())` round-trips wherever the leaf type is the payload type, because both go through the leaf's own `ToValue`/`FromValue`. It is the *schema* side that
must be checked (1.1).

| Aggregate | Tagging | Leaf type | `payload_value` | Verdict |
|---|---|---|---|---|
| `Puzzle2dMutation` | internal `mutation`, camelCase | struct, camelCase | `{targets, dx, dy}`; bridges for `Value`/`Puzzle2dPlaySnapshot` forward all four members (`MUT/🦀️.rs:480-500, 661-680`) | correct; 189/189 fixtures clean |
| `LowpolyMutation` | external, no `#[value]` | struct, camelCase | `{id, newPosition}` | correct; 17/17 clean |
| `PngMutation` | adjacent `mutation`/`payload`, kebab | struct (`ChangeHeaderMutation` is a struct, camelCase + `deny_unknown_fields`) | leaf struct | correct; `SetSnapshot` reads to the big snapshot object via a foreign `$ref` |
| `SvgMutation` (also xml, json, gltf) | adjacent `mutation`/`payload`, kebab | **enum** `phase`/`value` | `{phase, value}` | **wrong for 20 leaves, dead `Restore` editor for 140** (1.1) |
| `EquationMutation` (mathematical) | external | struct | was snake_case `new_label` against a camelCase schema (283 leaves like it); now `#[value(rename_all = "camelCase")]` and fixtures updated | fixed by W2-S |
| hand-written: config/presence/transient aggregates (`impl Mutation<…> for …` in ~40 files), `impl Mutation<Value> for Puzzle2dMutation`, `SetInteractionState` (hand-written `ToValue` delegating to `state`) | — | — | default `input_schema() == None` (not editable, roster `Opaque`); the puzzle bridges forward; `SetInteractionState`'s value equals its schema | correct and safe by default |

Residual key drift on the tree (my scan, 3,013 leaf sources): 3 stdio leaves without `rename_all` (`zip` `set-archive-comment.comment_utf8` ×2, `wav` `patch-data.remove_count/move_to`);
every other flagged entry is the new tag const or a doc-comment artifact of my scanner.

### 4.3 Deleted shapes

`git grep` over code, JSON, TS and docs outside `.🧬semio`: no `ConfigFieldShape`, `ConfigFieldSpec`, `CommandFieldSpec`, `ArgSchema::Vec3` or `ActionArgDef::vec3`. The
only `vec3` left in the manifest tree is a local variable name in `M/🧪️tests/🔬️app-label/🦀️.rs:266` (it builds `ActionArgDef::vector(…, 3)`). The remaining `"kind": "vec3"` are the stale descriptors in 2.5.

### 4.4 Puzzle 2d leaves

Independent recomputation (plain Python, no repo code) of five fixtures: `rotate-selection` `turns-two-nodes` (`node-a (0,0)→(30,-10)`, `node-b (40,20)→(10,30)`,
handle angles `+π/2`), `skips-the-region`, `scale-selection` `doubles-two-nodes`, `halves-node-region` (region corner `(-12.5,-12.5)→(3.75,-1.25)`, extent
`80.5×60.5→40.25×30.25`), `drag-selection` `drags-node-and-region`: all equal the committed `after` snapshots.

- Diff on any base: every survivor is patched whole from the BASE, in document order, and only if it changes (`puzzle2d_selection_diff`, `MUT/🦀️.rs:168-225`).
  Dedupe, classification precedence (node, then region), locked and missing handling, warning order (missing → locked → axis-aligned region) and
  `partial` targets = only the skipped ids match the Python model and the fixtures.
- Codes: partial = Warning `mutation.partial`, none survives = Error `mutation.target-missing` (before no-op), identity or nothing moves = Warning `mutation.no-op`, non-finite or
  `factor ≤ 0` = Fatal `mutation.invariant` (`drag/rotate/scale-selection/🔺️diff`), all with the default diff.
- Inverse (`puzzle2d_selection_inverse`, `MUT/🦀️.rs:230-254`): absolute setters computed from the base (`move-node`, `replace-node-handle`, `move-target-region`,
  `resize-target-region`), so it restores the base exactly; only x, y, handle angle, region corner and extent can differ because the forward diff spreads `..entry.clone()`.
  `move-node` (`📍move-node/🔺️diff`) writes x and y only, so no side effect.
- Pivot and rotation semantics equal the engine: `rotate_point_about` (`OS/♾️infinite/🎲️board/🦀️.rs:609`) is the leaf's formula, counter-clockwise in board space,
  handles turn by `+radians` (`update_transform_drag`), an axis-aligned region never rotates, the pivot is the mean of the selected node centres (locked counted).
  The old editor `Puzzle2dTransform` (HEAD `✏️editor/🦀️.rs:1028-1240`) used the same formulas with per-collection centroids; the leaf's single recorded pivot is a documented
  narrowing. W2-D's consumer (`🖱️select/🦀️.rs`, `Puzzle2dSelectionRecord::{then, applies_to, refused_as_locked}`) follows the contract: rotate yields node ids, composed ticks add
  offsets/angles and multiply factors only for equal targets and pivots.
- Schemas versus Rust types (script `SCR/pz2d_types.py`): all 36 leaves match field names, required-ness (Option ⇒ not required, `[T,"null"]`), integer versus number,
  fixed array lengths and `minimum: 0` on unsigned; `x-semio-ui` obeys the reader's own rules (103 inputs, 0 failures, snaps inside bounds, log scale from a positive bound).
- Wire: binary tags 33/34/35 present in `💾️binary/📡️.protocol.semio`; text grammar has the three productions; TS twin union arms exist (`MUT/🟦️.ts:292-294`); the aggregate schema has 36
  `$ref`s. Fixtures: 15 quintets, file names within the taxonomy limit.

### 4.5 Census and lints as they stand

- `schema mutation-inputs`: 5130/5269, 857 findings (2.6). Outside norm: stdio 3, wfc 2, everything else 0.
- `schema mutation-payloads`: 2719/2757, 76 findings; the 8 wfc/stdio ones are catalogue staleness (W2-S §6).
- Strict-Ajv sweep over all leaf schemas: not conclusive (my harness resolves foreign `$ref`s incompletely); the format and keyword findings I saw
  (`double`, `float`, `base64`, `owner`, `category`) predate W2-S's oracle change. Rely on W2-S's `🧪️w2-s-strict-compile.ts` (654/654 and 304/304).
- `verify taxonomy report --scope M` crashes on the peer's `📐️cad-draw-path-projection` frozen digest (pre-existing), so the placement of `🔣️input-labels.json` and the new
  test directories is unverified.

## 5. Findings from my first pass that are closed

| First-pass finding | Resolution |
|---|---|
| 283 leaves whose `ToValue` emits snake_case while schema/inputs are camelCase (norm 223, shooting 26, layout 23, …), plus ~36 structural drifts and 10 reverse ones | W2-S aggregate rule and per-owner fixes; lint `schema-mutation-payload-parity` with corpus and Ajv oracles (22 tests). Residual: 3 stdio leaves (§4.2). |
| glTF enum leaves with flat schemas (69) | 120/120 now `oneOf(phase)`; remaining 20 (svg/xml/json) are 1.1. |
| `x-semio`/`x-semio-mutation` on glTF leaves | removed (W1-D §6.5). |
| refUnresolved for `snapshot-patch`, `os/store/{link,child}` | ids split, registry exported at runtime (W1-D §6.7, §7); only catalogue refresh missing. |

## 6. Worklist for the coordinator (in order)

1. Route 1.1 to W1-D (hook + enum leaves) with the law of 2.1(b) and W2-S for the lint change 2.1(a).
2. Route 2.3 and the `Restore` editability to W2-A; 2.8 to W2-B/W2-C.
3. Route 2.2 to W1-D (`nullable`), then W2-B/W2-C (clear affordance) and the corpus.
4. Route 2.6 to W2-S-A (annotation preservation) and rerun the W2-R norm annotator afterwards.
5. Route 2.7 to a W1-F follow-up (invariants on the 5 sibling leaves, one invariant fixture, Python twin).
6. Route 2.4 to W1-D/W2-A, 3.1 to a W1-D glossary pass, 3.3 first bullet and the `[DEBUG]` lines to W1-F/W2-D.
7. Before Wave 3: descriptor regeneration (2.5), `schema generate`, and the freshness test.

## 7. Files touched by this audit

- Written: `T/📓️audit-inputs-leaves.md`.
- Scratch (mine, `SCR`): `scan_leaves.py`, `pz2d_types.py`, `ajv_all_leaves.ts`, `leaf-files.txt`, `derive-sites.txt`, `scan-result.json`, `census*.txt`/`census.json`, `strict*.txt`, `taxonomy-manifest.txt`,
  `oracle-exhaustive.txt`. No source, fixture, schema or ticket state was modified.
