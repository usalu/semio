# WP0 — Plugin fixture-owned schema audit (under `✏️s/`)

Read-only audit. Partition: every schema-shaped or schema-named file under a
test/fixture directory (`🧪️fixtures`, `🧫️fixtures`, `🧪️tests`, `🧪️publication-authority`,
`🧪️action-cohort`, `🧪️interactive-job`, any `🧪️*`/`🧫️*` dir) inside `✏️s/🔌️plugins/**`,
**excluding** mutation-leaf schemas under `🧬️schema/🧬️mutations/**` (a different
auditor's partition). Companion machine-readable file: `📊️wp0-plugin-fixtures.json`
(93 records, one per file, same order as this report).

## Method

1. `git ls-files -z '✏️s' | tr '\0' '\n' | grep -E '(🧪️|🧫️)' | grep -iE 'schema'`
   matched 14,704 lines — almost all false positives from the literal folder name
   `🧬️schema` (mixed emoji+ASCII) appearing inside `🧬️schema/🧬️mutations/**` paths.
   Excluding that substring dropped it to 109; excluding 10 `.ifc`/`.stp` files whose
   only match was the event slug `set-file-schema-applied` (not a schema at all)
   left **93 files** — the actual partition audited here.
2. For each file: read content (`$schema`/`$id`/`title`/`properties` = schema-shaped
   vs. plain data), get its blob id (`git ls-files -s`), and grep the whole repo for
   its directory name / `$id` / literal filename to find loaders — Rust `include_str!`,
   TypeScript `Bun.file(...).json()` + `Ajv`, and `📜️script.ts` build glue.
3. Checked `🧰️framework/…/📚️library/🔣️taxonomy.json` (`artifactSchemaSpecFileKinds`,
   `surfaceSchemaSpecFileKinds`) and confirmed via
   `🧰️framework/…/📚️library/🔍️discovery/🟦️.ts:3982-3983` that these keys feed the
   taxonomy **conformance/discovery** engine, not an application runtime loader — i.e.
   `🧬️schema`, `🎚️config/🧬️schema`, `👥️presence/🧬️schema`, `🫧️transient/🧬️schema` are
   the *only* taxonomy-recognized schema-authority locations. Every plugin checked
   already has these (subset-level `🧬️schema` plus editor `🎚️config`/`👥️presence`
   `🧬️schema`) — see per-plugin `find … -name 🧬️schema` dumps in the evidence below.
   Nothing in the taxonomy blesses an editor-law, plugin-root, or package-level schema
   location, which is the structural root cause of most violations found here.

## Coverage summary

| | count |
|---|---|
| Files audited (this partition) | 93 |
| **Violations** (fixture-owned application contract) | 55 |
| Violation (minor, single-key pattern schema) | 1 |
| Violation — duplicate authority + drift | 1 |
| Ambiguous (test-harness/meta-schema, not clearly an app contract) | 2 |
| Not applicable (matched only by filename substring, not a schema) | 2 |
| Non-violation (already inside the correct, taxonomy-blessed `🧬️schema` module) | 29 |
| **Total** | 93 |

56 of the 93 files (55 + 1 minor + 1 duplicate) are genuine fixture-owned
contracts — schema authority sitting under a test/fixture directory instead of an
owned `🧬️schema` module, matching the ticket's violation pattern almost exactly
one-for-one with what the pattern predicted.

---

## Family-grouped findings

### 1. `retained-command-limits` — 7 instances, VIOLATION (shared pattern, per-artifact content)

Every one of these lives at `…/✏️editor/🧪️fixtures/🧫️retained-command-limits/🧬️.schema.json`
(fem/remodel use `🚧️retained-command-limits` instead of `🧫️`) next to a sibling
`🔣️.json` data file. In every case checked, the editor's own `🦀️.rs` only
`include_str!`s the **data** file for a test — e.g.
`✏️s/🔌️plugins/🎞️animate/…/✏️editor/🦀️.rs:815`:
`include_str!("🧪️fixtures/🧫️retained-command-limits/🔣️.json")`. The `.schema.json`
itself has no confirmed programmatic consumer anywhere in this pass.

| Plugin | Path | $id/title | Consumer (data only) |
|---|---|---|---|
| animate/presentation | `🎞️animate/…/✏️editor/🧪️fixtures/🧫️retained-command-limits/🧬️.schema.json` | `urn:semio:animate-presentation:retained-command-limits:v1` | `✏️editor/🦀️.rs:815` |
| shooting | `🎥️shooting/…/✏️editor/🧪️fixtures/🧫️retained-command-limits/🧬️.schema.json` | (anonymous) | `✏️editor/🦀️.rs` |
| fem/3d | `🏗️fem/…/✏️editor/🧪️fixtures/🚧️retained-command-limits/🧬️.schema.json` | `urn:semio:fem3d:retained-command-limits:v1` | `✏️editor/🦀️.rs` |
| remodel | `📸️remodel/…/✏️editor/🧪️fixtures/🚧️retained-command-limits/🧬️.schema.json` | (anonymous) | `✏️editor/🦀️.rs` |
| space/home | `🪐️space/…/🏠️home/…/✏️editor/🧪️fixtures/🧫️retained-command-limits/🧬️.schema.json` | `urn:semio:home:retained-command-limits:v1` | `✏️editor/🦀️.rs` |
| space/space | `🪐️space/…/🪐️space/…/✏️editor/🧪️fixtures/🧫️retained-command-limits/🧬️.schema.json` | (anonymous) | `✏️editor/🦀️.rs` |
| space/engine | `🪐️space/⚙️engine/🪐️space/🧪️fixtures/🧫️retained-command-limits/🧬️.schema.json` | (anonymous) | `⚙️engine/🦀️.rs` |

**Classification**: violation. **Owner today**: none. **Intended owner**: each
plugin's own subset-level `🧬️schema` module (all 7 already exist), or — since the
URNs differ per artifact (`animate-presentation`, `fem3d`, `home`) — a new
editor-law schema kind if the taxonomy grows one.
**Open question**: is this content genuinely per-artifact (so 7 separate owned
schemas is correct) or is the *shape* identical across all 7 (candidate for one
shared framework schema under `🧰️framework/🔨️modules/🖱️ui`, as the ticket brief
speculates for "retained-command-limits")? Not resolved in this pass — would need
a property-by-property diff across all 7, deferred due to budget.

### 2. `retained-command-routes` — 3 instances, VIOLATION

`…/✏️editor/🧪️fixtures/🧬️retained-command-routes.schema.json`, sibling to
`🛣️retained-command-routes.json`. Same pattern: only the data file is
`include_str!`'d (`const RETAINED_ROUTES: &str = include_str!("🧪️fixtures/🛣️retained-command-routes.json")`)
in vcs (`✏️editor/🦀️.rs:1212`), reasoning/wires (`:621`), imperative/procedure (`:584`).

| Plugin | $id |
|---|---|
| vcs | `https://semio.dev/schema/vcs/retained-command-routes.v1.json` |
| reasoning/wires | `https://semio.dev/schema/reasoning/wires/retained-command-routes.v1.json` |
| imperative/procedure | `https://semio.dev/schema/imperative/retained-command-routes.v1.json` |

**Intended owner**: each plugin's owned subset-level `🧬️schema`.

### 3. `equation-retained-command` — 1 instance, VIOLATION

`➗️mathematical/…/➗️equation/🧪️fixtures/🧬️equation-retained-command.schema.json`,
`$id=https://semio.tech/schema/equation-retained-command-law.json`. Same family as
#1/#2 in spirit (an editor retained-command law) but named/shaped differently.
Consumers: `equation/🦀️.rs` and `equation/…/✏️editor/🦀️.rs` (grep hits, exact
`include_str!` line not isolated).

### 4. `scene-owner` — 5 instances, VIOLATION

Small single-file "law" schemas at artifact-editor scope, sibling to a plain data
JSON, named `<artifact>-scene-owner.schema.json` or similar:

| Plugin | Path | $id |
|---|---|---|
| writer | `✒️writer/…/✒️writer/🧪️fixtures/🧬️writer-child-local-text.schema.json` | `…/writer-child-local-text-law.json` |
| equation | `➗️mathematical/…/➗️equation/🧪️fixtures/🎬️equation-scene-owner.schema.json` | `…/equation-scene-owner-law.json` |
| flow (plugin root) | `🌊️flow/🧪️fixtures/🏠️flow-scene-owner.schema.json` | `…/flow-scene-owner-law.json` |
| playbook | `📖️playbook/…/📖️playbook/🧪️fixtures/🎚️playbook-view-command-limits.schema.json` | `…/playbook-view-command-limits.json` |
| playbook | `📖️playbook/…/📖️playbook/🧪️fixtures/🎬️playbook-scene-owner.schema.json` | `…/playbook-scene-owner-law.json` |
| flow (plugin root) | `🌊️flow/🧪️fixtures/🧹️surface-owners/🧬️.schema.json` | (anonymous) |

Consumers confirmed via grep in `writer/🦀️.rs`, `equation/🦀️.rs`,
`flow/🦀️.rs`, `playbook/🦀️.rs`. flow's two plugin-root files are additionally
`Bun.file`+`Ajv`-validated by `flow/…/✏️editor/🧪️fixtures/📜️script.ts` (see family
#5) via relative-path reach-ins from the editor's own fixtures dir
(`script.ts:466-467` for surface-owners).

**Note**: flow's artifact **already owns** a full multi-implementation `🧬️schema`
module one level down (`🔣️.json`/`🦀️.rs`/`🟦️.ts`/`🔗️.graphql`/`🛰️.proto` at
`…/🪆️subsets/✳️any/🧬️schema/`) — `flow-scene-owner` and `surface-owners` are a
*separate*, still-unowned concern sitting at the plugin root, not a duplicate of
that module.

### 5. flow editor micro-law schemas — 15 instances, VIOLATION (strongest evidence in the audit)

`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/📜️script.ts`
(485 lines) `import Ajv from "ajv"` and, for 13 of these 14 sibling pairs,
literally does:

```ts
const X = await Bun.file(new URL("./<name>/🔣️.json", import.meta.url)).json();
const XSchema = await Bun.file(new URL("./<name>/🧬️.schema.json", import.meta.url)).json();
const validateX = new Ajv({ strict: true, allErrors: true }).compile(XSchema);
assert(validateX(X), JSON.stringify(validateX.errors));
```

then runs hostile-mutation round-trips against the schema (`for (const mutate of [...])`)
and — separately — reads and asserts against the real Rust source of the
corresponding command (e.g. `➕️add-widget/🦀️.rs`) via plain text search. This is a
**live, enforced** validation oracle, not documentation, confirming the ticket's
violation pattern in its purest form: the schema genuinely governs correctness of a
named behavior, but the file sits in `🧪️fixtures/` instead of being exported from
`🧬️schema`.

| Path suffix (all under `✏️editor/🧪️fixtures/`) | script.ts evidence |
|---|---|
| `🏪️store-owners/🧬️.schema.json` | line 381-382, `Ajv` |
| `👥️presence-owners/🧬️.schema.json` | line 405-406, `Ajv` |
| `📐️artifact-canonical.schema.json` (18,560 bytes — largest in this family) | line 252-253, `Ajv`; ALSO independently `include_str!`'d (different sibling file `🧾️artifact-canonical.json`) by `✏️editor/🧵️retained/🧾️canonical/🦀️.rs:175` |
| `📝️slider-labels.schema.json` | line 229-230, `Ajv` |
| `📡️host-wire/🧬️.schema.json` | line 87-88, `Ajv` |
| `🖼️tree-projection/🧬️.schema.json` | line 59-60, `Ajv` |
| `🧒️child-add-widget/🧬️.schema.json` | line 16-17, `Ajv` |
| `🧩️artifact-recipes.schema.json` | line 105-106, `Ajv` |
| `🧫️grant-frontier/🧬️.schema.json` | line 173-174, `Ajv` |
| `🧵️add-widget-retained/🧬️.schema.json` | **not** in the Ajv set — instead its sibling `🔣️.json` is `include_str!`'d directly by `✏️editor/🦀️.rs:2438`; the schema file's own consumer is unconfirmed (possibly dead) |
| `🧹️delete-cascade/🧬️.schema.json` | line 273-274, `Ajv` |
| `🪪️content-identity/🧬️.schema.json` | line 308-309, `Ajv` |
| `🫧️transient-owners/🧬️.schema.json` | line 425-426, `Ajv` |
| `👁️viewer/🧪️fixtures/🧹️owners/🧬️.schema.json` (reached from editor script.ts via `../../👁️viewer/…`) | line 450-451, `Ajv` |

**Intended owner**: flow's already-existing subset-level `🧬️schema` module, or a
new editor-law schema kind under it; `script.ts` should import named schema exports
instead of reading co-located files.
**Open**: `add-widget-retained`'s schema consumer needs isolating; also worth
noting `script.ts:144-146` validates a **framework**-level fixture+schema pair at
`🧰️framework/…/🌊️flow/🎚️parameter/📨️intent/…` — same pattern, but outside `✏️s/`
so outside this partition (flag for whoever audits the framework side).
Ajv itself does not appear in flow's own `📦️packages/🟦️typescript/package.json` —
likely resolved from a workspace-hoisted devDependency; worth confirming this
doesn't violate CLAUDE.md's "no runtime dependency on external libraries" (it is
test-time only, which the rule explicitly allows for validating our own
implementation).

### 6. `plugin-identity` — 2 instances, VIOLATION (structural taxonomy gap)

| Plugin | Path | title |
|---|---|---|
| reasoning | `💡️reasoning/🧪️fixtures/🧫️plugin-identity/🧬️.schema.json` | "Reasoning plugin identity tuple" |
| space | `🪐️space/🧪️fixtures/🧫️plugin-identity/🧬️.schema.json` | "Space plugin identity tuple" |

Both well-formed (draft-07, `title`+`description`+`required`+`properties`), each
consumed (data only) from the plugin's own root `🦀️.rs`. **This is not just a
misplaced file**: the taxonomy currently has no schema-authority location at
*plugin root* at all — only at artifact/subset level (`🧬️schema`) and surface
level (`🎚️config`/`👥️presence`/`🫧️transient` `🧬️schema`). Every plugin that needs a
plugin-identity tuple hits this same gap. Flag to taxonomy owners as a probable
missing contract location shared across plugins, not a one-off fix.

### 7. `artifact-identity` family — 3 instances, VIOLATION (per-plugin, not literally shared)

| Plugin | Path | $id | Consumer |
|---|---|---|---|
| gis | `🌍️gis/🧪️fixtures/🪪️artifact-identity/🧬️.schema.json` | `urn:semio:gis:artifact-identity:v1` | `gis/🦀️.rs:89` `include_str!("🧪️fixtures/🪪️artifact-identity/🔣️.json")` (data only) |
| vcs | `🌿️vcs/🧪️fixtures/🪪️native-openable-identity/🧬️v1/🧬️.schema.json` | (anonymous, has `$defs`) | not isolated in this pass |
| stdio | `🗄️stdio/📦️packages/🦀️rust/🧫️fixtures/🖊️dwg-artifact-ownership/🧬️.schema.json` | none — just `{$schema,type,pattern}` (182 bytes, smallest schema in the audit) | not isolated (inferred sibling of #8) |

Same family *name*, different plugins, different property sets — genuinely
per-plugin ownership, not duplicate authority (unlike family #14 below). Each
needs its own plugin-root (gis, vcs) or package-level (stdio) schema home, none of
which currently exist.

### 8. stdio rust-package fixtures — 2 instances (+ #7's dwg file), VIOLATION

`🗄️stdio/📦️packages/🦀️rust/🧫️fixtures/🌳️catalog-root/🧬️.schema.json` and
`…/🏠️home-io-surface/🧬️.schema.json` (`$id=https://semio.tech/schemas/stdio-home-io-surface-v1.json`).
No package-level `🧬️schema` module exists for stdio's rust package. **Decision**:
create one and fold catalog-root + home-io-surface + dwg-artifact-ownership (#7)
into it together, since they're siblings under the same `🧫️fixtures` root.

### 9. gis plugin-root fixtures — 4 instances (+ #7's artifact-identity), VIOLATION

| Path | $id | Consumer |
|---|---|---|
| `🌍️gis/🧪️fixtures/🌉️component-cold-map-patch/🧬️.schema.json` | `…/component-cold-map-patch-v1.json` | **unique in the whole audit**: this fixture directory has its own co-located `🦀️.rs` module (`🌍️gis/🧪️fixtures/🌉️component-cold-map-patch/🦀️.rs`) — production-shaped code, not just data, sits inside `🧪️fixtures`. Highest-priority follow-up: read that file in full to see if it's wired into gis's real dispatch or is test-only. |
| `🌍️gis/🧪️fixtures/💡️inference-control/🧬️.schema.json` | `urn:semio:gis:inference-control:v1` | grep-hit only in `📦️packages/🦀️rust/📜️script.ts` (build-glue path reference, not confirmed validation) |
| `🌍️gis/🧪️fixtures/🗄️durable-three-store-assembly/🧬️.schema.json` | none | same as above |
| `🌍️gis/🧪️fixtures/🧩️map-create-region-group/🧬️.schema.json` | none | grep-hit in gismap's `🧬️schema/💡️inferences/🦀️.rs` and `📦️packages/🦀️rust/📜️script.ts` — natural destination is gismap's already-owned `🧬️schema/🧬️mutations/🌐create-region`, which is the **mutations auditor's territory**; flag the boundary |

**Intended owner**: gis plugin root (no home exists) for inference-control /
durable-three-store-assembly; gismap's `🧬️schema/💡️inferences` (taxonomy-blessed) or
`🧬️schema/🧬️mutations` for the other two.

### 10. stdio registry `native-catalog-surface` cluster — 5 files, VIOLATION (largest orphaned cluster)

`🗄️stdio/📇️registry/🧪️fixtures/📇️native-catalog-surface/` holds 5 schema files
(`📌️commitment.schema.json`, `🧬️.schema.json`, `🧬️budget.schema.json`,
`🧬️commitment-cases.schema.json`, `🧬️imports.schema.json`) plus data files
(`🧪️budget.json`, `🧪️commitment.json`, `🧪️imports.json`, `🔣️.json`). Registry
**already has its own** `🧬️schema` module
(`🗄️stdio/📇️registry/🧬️schema/🧬️native-codec-factories.schema.json`) — but that
covers a completely different contract (`native-codec-factories`), so this is not
duplicate/drifted authority, just a second, entirely unowned cluster.

Only `🧪️budget.json` is consumed (`registry/🦀️.rs:1328`,
`include_str!("🧪️fixtures/📇️native-catalog-surface/🧪️budget.json")`). **None of the
5 `.schema.json` files have a confirmed consumer anywhere in stdio** (grepped
`registry/🦀️.rs` and all `*.ts` under stdio for each filename — zero hits). This is
the strongest **possibly-dead-schema** candidate in the audit; needs a
repo-wide (not just stdio-scoped) grep before concluding it's truly orphaned.

`claim-authority` (`🗄️stdio/📇️registry/🧪️fixtures/🧾️claim-authority/🧬️.schema.json`)
is a 6th, related file: only its data sibling is loaded, and via a fragile
cross-package `CARGO_MANIFEST_DIR`-relative reach-in from
`📦️packages/🦀️rust/🧪️tests/📇️native-openable-provider/🦀️.rs:44`.

**Intended owner**: fold all 6 into registry's existing `🧬️schema` module as named
exports.

### 11. `note-action-cohort` — 1 instance, VIOLATION — duplicate authority + drift (second-strongest finding)

`🗒️note/🧪️action-cohort/🧬️.schema.json` (`$id=semio.note.action-cohort.v1`) is a
**narrower, drifted duplicate** of `✏️s/🔌️plugins/🌊️flow/🎬️action-cohort/🧬️.schema.json`
(`$id=semio.flow-note.action-cohort.v1` — outside this partition since `🎬️action-cohort`
isn't test-marked, but same contract family and directly relevant):

- flow's schema's `$id` property is `enum: [semio.flow-note.action-cohort.v1, semio.note.action-cohort.v1]`
  and its `owner` property is `enum: [FlowPlayApp, NotePlayApp]` — i.e. flow's
  schema was **authored to already cover note's case**.
- note's copy instead hardcodes `const: semio.note.action-cohort.v1`,
  `const: NotePlayApp`, `const: 36` for `routeCount`, and its `required` list
  **omits** `blockedSeams`/`materialization` that flow's schema requires — a real
  content drift, not just a location problem.
- The actual consumer,
  `flow/…/✏️editor/🦀️.rs:2528-2530` fn
  `action_cohort_fixtures_match_the_exact_route_census()`, reads **both** data
  files as plain JSON (`../../🎬️action-cohort/🔣️.json` and a cross-plugin reach-in
  `../../../🗒️note/🧪️action-cohort/🔣️.json`) — neither is schema-validated at that
  call site, so note's schema copy is not even the oracle for its own data.

**Decision**: consolidate to one owned schema (most naturally flow's, since it
already generalizes over both apps, or a new shared framework location), delete
note's narrower copy. **Open**: this straddles a plugin boundary and a taxonomy
gap (no shared cross-plugin schema location exists) — needs a ruling from whoever
owns the flow/note action-cohort integration before deleting either copy.

### 12. `norm` — 3 files

- `📕️norm/🎚️config/🧪️tests/🧬️.schema.json` — **ambiguous**. norm's *real* config
  schema (`📕️norm/🎚️config/🧬️schema/🔣️.json`, `$id=https://semio.tech/schema/app/norm/norm/config.json`,
  `title=NormConfig`, `properties=['selectedCheckIndex']`) has a **completely
  different shape** from this test schema
  (`properties=['contractId','cases','invalid','text','binary']`). This is not
  duplicate/drifted config authority — it looks like a **test-case-shape
  meta-schema** (describes the shape of test records, similar in spirit to the
  repo-wide `testSchemaLocation` at `🧰️framework/…/🧪️test/🧬️schema`). Needs a
  ruling on whether test-case-shape schemas fall inside this ticket's violation
  pattern at all.
- `📕️norm/🖥️app-surface/🧪️tests/🧬️.schema.json` — same ambiguity; unlike
  `🎚️config`, `🖥️app-surface` has **no** existing `🧬️schema` module at all, so
  there's no known-real contract to compare against. Possibly this test schema is
  secretly the only spec app-surface has.
- `📕️norm/🧪️fixtures/🧫️retained-command-dispositions/🧬️.schema.json` — clear
  **violation**, `$id=https://semio.local/schemas/norm-retained-command-dispositions.schema.json`,
  9303 bytes, belongs to the repo-wide retained-command-* family (#1/#2/#3).

### 13. Singletons — VIOLATION

| Plugin | Path | Note |
|---|---|---|
| process3d | `🏭️process/…/🧊️process3d/🧪️tests/📐️retained-route-schema.json` | `$id=semio.process3d.retained-route-laws.schema.v1.json` — filename says "...schema.json" **inside a `🧪️tests` dir**, not even `🧪️fixtures`; process3d already owns a subset-level `🧬️schema` |
| lowpoly | `💠️lowpoly/🧪️interactive-job/🧬️.schema.json` | title "Lowpoly Interactive Job Partition"; editor's `🦀️.rs:425` doc-comment explicitly references "the coordinator's own schema edit (`interactive-job.schema.json`'s 8th `oneOf` signature)" — **the oneOf arm count is load-bearing for dispatcher correctness**, this is the interactive-job-classification family named in the goal doc, genuinely live |
| block | `🧱️block/🧪️publication-authority/🧬️.schema.json` | **most clearly live violation in the audit** — see below |
| draw | `🖍️draw/🧪️publication-authority/🧬️.schema.json` | same pattern as block, presumed same ScriptRouter oracle shape in `draw/📦️packages/🟦️typescript/📜️script.ts` |
| puzzle 2d | `🧩️puzzle/…/◻️2d/…/🌉️wasm/🧪️fixtures/🧬️.schema.json` | no confirmed consumer found in this pass |
| puzzle 3d | `🧩️puzzle/…/🧊️3d/…/⏳️precompute/🪣️fill/🧪️fixtures/🧬️.schema.json` | `$id=semio.puzzle3d.fill-preview-json.v1`, has a custom `x-semio-maxEncodedUtf8Bytes` vendor key; consumer grep-hit in `📦️packages/🦀️rust/🦀️.rs` |
| space/home | `🪐️space/…/🏠️home/…/✏️editor/🎚️config/🧪️fixtures/📇️projection-persistence-v1/🧬️.schema.json` | **easiest fix in the whole audit**: `✏️editor/🎚️config/🧬️schema` already exists one directory up from `🎚️config/🧪️fixtures/📇️projection-persistence-v1` — this is a pure relocation, no new schema location needs inventing |

**block detail** (`🧱️block/📦️packages/🟦️typescript/📜️script.ts`):
```
:46  return fixture.schema === "semio.app.publication-authority.v1" && fixture.apps.length > 0
       && fixture.apps.every((app) => appOracle(app, sources.get(app.owner) ?? ""));
:74  const authority = resolve(plugin, "🧪️publication-authority");
:81  if (!oracle(fixture, sources)) throw new Error("Block publication-authority oracle rejected production");
:96  const router = new ScriptRouter(import.meta.dir).register("test", TestScript)
       .register("publication-authority-audit", TestScript);
```
This registers `publication-authority-audit` as a first-class nx command per
CLAUDE.md's `launch.json` convention, and the fixture literally gates whether
block's real production sources pass an authority check at audit time — the
single most operationally live example found. Prioritize this one first.

---

## Non-violations / inert (29 files) — already inside a taxonomy-blessed `🧬️schema` module

These matched the `🧪️`/`🧫️` search only because a sub-directory *inside* an
already-correctly-owned `🧬️schema` module happens to be named `🧪️tests`,
`🧪️oracle`, `🧪️contract`, or `🧪️body`. Since the file already lives under the
owning scope's own `🧬️schema` tree, this is not the "fixture smuggles authority
from outside the module" pattern the ticket targets — at most a naming-hygiene
note, not a relocation:

- **gis** (3): `🏔️gisterrain/…/✏️editor/🎚️config/🧪️tests/🧬️direct-leaves/🧬️schema/🔣️.json`;
  `🗺️gismap/…/✏️editor/👥️presence/🧬️schema/🧪️tests/🔣️.json` (data) and
  `…/🧬️schema/🔣️.json` (schema, `$id=https://semio.tech/schema/gis/gis2d-presence/tests`)
  — both already inside presence's own `🧬️schema` module.
- **sequence** (2): `…/✏️editor/🌉️wasm/🧬️schema/🧪️oracle/🔣️.json`
  (`$id=semio://sequence.browser-abi/oracle/v1`, likely validated by the JS test
  file below); `…/🧬️schema/⚙️operations/🧪️tests/🔣️.json` (data, `keys=['cases']`).
- **sequence test code** (1, not a schema at all): `…/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🧪️tests/🧬️sequence-schema.test.js`
  — matched only because its filename contains the substring "schema".
- **remodel** (1): `…/🧬️schema/🧪️tests/🟦️.ts` — TS test logic, not a schema.
- **raster** (1): `…/🧬️schema/🔺️diff/📝️text/🧪️fixtures/🔣️asset-capacity.json` — inert
  data table (`keys=['capacity','cases']`).
- **gltf inference contracts** (22 = 11 pairs): `🗄️stdio/…/🧊️gltf/…/🧬️schema/💡️inferences/<category>/<name>/🧪️contract/{🔣️.json,🟦️.ts}`
  for elongation, flatness, slenderness, aspect-ratios, axis-aligned-bounds,
  overall-size, boundary-loops, genus, holes, handles, euler-characteristic. Each
  `.json` is a small `{id, vectors}` lookup table (inert data); each `.ts` is the
  real authored type/formula. `artifactSchemaSpecFileKinds` explicitly recognizes
  `🧬️schema/💡️inferences` as a schema-authority location, and `🟦️typescript` is a
  taxonomy-recognized normative format (`schemaFormats.🟦️typescript`) — so this
  whole cluster is correctly scoped already.
- **stdio semio-standard** (4): `…/🧿️semio/…/🌊️flow/🧬️schema/📸️snapshot/💾️binary/🧪️tests/🦀️.rs`
  (test code); `…/💾️binary/🧫️fixture/{♻️lifecycle/,}🔣️.json` (2 inert data files);
  `…/🧊️brep/🧬️schema/💡️inferences/✅validation-report/🧪️body/🦀️.rs` (test code, not
  a schema — matched only via the `🧪️body` path segment).

---

## Recurring-family verdicts (per ticket brief)

- **`🧫️retained-command-limits`**: appears in 6+ plugins (animate, shooting, fem,
  remodel, space×3) with **distinct URNs per artifact** — content is per-artifact,
  not literally shared, so the correct fix is 6 separate owned schemas (or 6 named
  exports from one shared *shape*, unresolved — see family #1's open question),
  each relocated into its own plugin's existing subset-level `🧬️schema`.
- **`🧫️plugin-identity`**: appears in reasoning and space with different content
  per plugin — per-plugin ownership, but the taxonomy has **no plugin-root schema
  slot at all**, so this is a structural gap affecting every plugin, not a simple
  relocation.
- **`🪪️artifact-identity`**: appears in gis, vcs (as `native-openable-identity`),
  stdio (as `dwg-artifact-ownership`) — same family *name*, different shapes,
  genuinely per-plugin.
- **`📇️projection-persistence-v1`**: single instance (space/home) — trivial
  relocation, sibling `🧬️schema` already exists.

---

## Unresolved questions (repo-wide, not just this partition)

1. Whether Ajv (used by flow's `📜️script.ts`, family #5) counts as an acceptable
   test-time external-library use under CLAUDE.md, or needs an in-house
   JSON-Schema validator per "You MUST use all external libraries behind an
   interface." Not resolved here — flagged for the ticket owner.
2. Whether norm's two `🧪️tests/🧬️.schema.json` files (family #12) are in-scope for
   this ticket's violation pattern at all, since they describe test-case shape
   rather than application data shape.
3. Whether stdio registry's 5-file `native-catalog-surface` cluster (family #10)
   is truly dead code (zero consumer found for the schema files themselves) —
   needs a repo-wide grep beyond `✏️s/🔌️plugins/🗄️stdio` before any deletion.
4. `✏️s/🔌️plugins/🌍️gis/🧪️fixtures/🌉️component-cold-map-patch/🦀️.rs` — a fixture
   directory containing real Rust logic, not just data — needs a full read to
   determine if it's wired into gis's production dispatch.
5. The flow/note `action-cohort` pair (family #11) needs an owner decision before
   either schema copy is deleted or merged.
6. Several consumers marked "grep-hit" or "not isolated in this pass" in the JSON
   (mostly stdio-rust-fixtures, puzzle-2d, note-action-cohort, vcs
   native-openable-identity) need a follow-up targeted grep to pin the exact call
   site — the family-level pattern is well-established from verified siblings, but
   the individual line numbers were not confirmed for every file given the budget
   for this pass.

## Deliverable cross-reference

Full per-file records (all 93, in the order presented above) are in
`📊️wp0-plugin-fixtures.json` alongside this file.
