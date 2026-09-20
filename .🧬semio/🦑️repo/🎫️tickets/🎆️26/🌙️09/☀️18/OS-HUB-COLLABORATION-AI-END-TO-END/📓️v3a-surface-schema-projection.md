# V3a — `surface-schema-projection` generator (2026-09-19)

Slice V3a owns the **surface schema-lane family** of `plugin-registry check`
(`surface "<s>" / plugin-root is missing 🎚️config/🧬️schema/…` and `… 👥️presence/🧬️schema/…`) — the one
V2 §3.3 specified as a new generator contract and declined to build. Sibling V3b owns the other
families (unreachable Rust mounts, directory lanes, taxonomy-owner decisions, interactivity gates);
nothing here touches them.

## 0. Headline

> **Superseded by §12.** The numbers below are session 3's. Re-measured 2026-09-20: total **619**,
> schema-lane family **2**. Sections 8–16 are the current, measured state; 1–7 are kept as the
> design record.

| gate | before (03:59) | after (11:4x) |
|---|---|---|
| `plugin-registry check`, **total** | 1836 | **684** |
| … **schema-lane family (mine)** | **627** | **17** |
| … `📜️.wit` findings inside that family | 53 | **0** |
| … unreachable-from-Cargo-manifest (V3b's family) | 892 | 351 |

Captures: `🗑️generated/v3a-check-before.txt`, `v3a-check-after.txt` (both the real
`bun ./📜️script.ts check`, 1836 and 684 finding lines).

**Attribution, honestly.** Of the 1152-finding drop, **610 are this slice** (627 → 17 in my family).
The other 542 are V3b's unreachable-mount work, which landed concurrently — I measured the same
per-plugin numbers moving while I ran (`v3a-perplugin-before.txt` at 04:06 already showed 892 → 599).
The number I own end-to-end is the schema-lane family, and it is the one I report per plugin in §4.

Critically, the 519 Rust leaves the projection wrote added **zero** new
`is not reachable from Cargo manifest` findings — the generator emits the `#[path]` mount with the
leaf (§2.4). That was the design's whole risk and it is measured, not assumed: stdio's unreachable
count fell 284 → 209 across a run that added 352 new `🦀️.rs` leaves to that same plugin.

## 1. How the existing schema-first pipeline works

Studied on a fully conformant lane: `📋️forms` / `🖨️raster`
`🗿️artifacts/<a>/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/{🎚️config,👥️presence}/🧬️schema/`.

**What a lane contains.** Five leaves, one per `🔣️taxonomy.json` `schemaFacetKinds.🧬️data` format:
`🔣️.json` `🦀️.rs` `🟦️.ts` `🔗️.graphql` `🛰️.proto`, plus optional `🧬️mutations/<slug>/` children. The
`🔺️diff/` `📸️snapshot/` `💡️inferences/` children listed in `schemaChildDirs` belong to the **artifact**
schema facet (`🪆️subsets/<s>/🧬️schema/`), not to a surface state lane — `assertAppSchemaOwner` never
asks for them.

**Who is the source of truth.** Two levels, and they are different:

1. Across a lane's five leaves, `🔣️.json` is normative — `🔣️taxonomy.json` `surfaceSchemaSpecFileKinds`
   maps `🎚️config/🧬️schema` and `👥️presence/🧬️schema` to `json`, and the repo library's
   `📏️field-parity` law resolves every disagreement in its favour
   (`📚️library/🧬️schema/🗺️surface/⚖️laws/📏️field-parity/🟦️.ts:30-34`).
2. Above the lane, the **real Rust config type** is the truth the normative JSON must mirror:
   `🪞️config-fidelity` compares the lane's `🔣️.json` field-by-field against the `pub struct` named by
   the surface's `type Config = …` binding (`…/⚖️laws/🪞️config-fidelity/🟦️.ts:11-40`), and
   `policyDiscoverAppSchemaOwners` (`…/🗺️surface/🔍️owner-discovery/🟦️.ts`) reads that binding out of
   the surface component leaf.

So the chain is **surface `type Config`/`type Presence` binding → real `pub struct` → `🔣️.json` →
four mirrors**. Both ends are already machine-read by shipped code; only the middle was hand-carved.

**Which generator emits them today: none.** V2 §3.3's finding reproduced independently — all 23
`generatorContracts` declare literal `outputRoots`, exactly seven of which lie under `✏️s/🔌️plugins`
and all seven belong to `external-step-assets` (`.stp` fixtures). The one contract with a `🧬️schema`
output root, `schema-entity-catalog`, writes into `🧰️framework/🔨️modules/🧬️schema/🤖️generated/…`. The
root `📜️script.ts` `SchemaScript` (`:15078-15358`) `generate` verb rewrites the scope **catalog**
(`🔣️schema-catalog.json`), never a facet leaf. The plugin build consumes schema leaves and does not
emit them. Confirmed: these lanes were authored input, and there were ~2 600 of them missing.

**Rust→schema emitters that do exist** (checked per the brief's step 3): the `ArtifactSchema` derive
(`🧰️framework/🔨️modules/🧬️schema/✨️derive/`) registers a descriptor at **runtime** from a Rust type; it
emits no files. The repo library has the opposite direction — five **parsers**
(`📚️library/🧬️schema/🔍️field-discovery/{🦀️rust,🟦️typescript,🔗️graphql,🔣️json-schema,🛰️protobuf}/`) that
read each format into one neutral `PolicySchemaLeafExtract`. The projection is built on top of the
Rust parser and is verified by round-tripping through the other four (§5).

## 2. The `surface-schema-projection` contract

Implemented in the owning registry module — **no new script file**, per AGENTS.md:
`📇️registry/🧬️surface-schema/🟦️.ts` (new module), registered as the `surface-schema` verb on
`📇️registry/📜️script.ts:10`.

```
bun ./📜️script.ts surface-schema [--plugin <id>] [--check] [--dry-run] [--no-rust]
```

### 2.1 Owner discovery — the same predicate the gate applies

`discoverSurfaceSchemaOwners` walks `findNewContractPluginRoots` → `surfaceDirsForPlugin` + the plugin
root, and admits an owner **iff `🎚️config/` exists** — byte-for-byte the condition
`assertAppSchemaOwner` uses before it starts reporting (`🗿️taxonomy-validation/🟦️.ts:646-648`). It
finds **574 lanes**, against 627 findings (a wholly missing lane is 1 finding but 5 leaves, a partly
missing one is up to 5; the two numbers are not meant to match).

### 2.2 Source of truth — resolved, never invented

Per lane: read `type Config = …` / `type Presence = …` off the surface component (last `::` segment,
so `crate::cfg::HomeConfig` and `semio_framework_plugin::NoPresence` both resolve); fall back to the
repo library's own `policyAppPresenceTypeName` for presence when a surface binds no `type Presence`.
Then find that name's `pub struct` in order: the lane's own `🦀️.rs` → the plugin root's lane `🦀️.rs` →
a declaration anywhere under the plugin (indexed) → the framework sentinel file. No match ⇒ `blocked`,
reported, nothing written.

**The measurement that makes this tractable:** 269 of 574 surfaces bind `type Config = NoConfig` and
260 bind `type Presence = NoPresence` (`grep` census, §4). `NoConfig`/`NoPresence` are real framework
declarations — `pub struct NoConfig {}` at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:9907` and `:10018`. So the overwhelming majority
of the family projects the **empty object**, which is the faithful schema of a surface that owns no
config, not fabricated data. That distinction is the whole reason V1 §3.4 refused to hand-write them
and this slice could generate them.

### 2.3 Projection — deterministic, five formats, JSON normative

`projectSurfaceSchemaLane` emits exactly the leaves `taxonomySchemaFilenames(facetAbs)` demands and
throws if the taxonomy ever asks for a format it has no projection for (so a taxonomy change fails
loudly instead of writing a partial lane). Scalars go through one table keyed by the repo library's
own `policyCanonicalScalar` outputs, so every emitted leaf parses back to the shape it came from.

Two deliberate spelling decisions, both forced by the target languages:

* **GraphQL has no empty-object type** — `type X {}` is a syntax error. A field-less schema is emitted
  as the braceless `type X`, which the graphql reference implementation accepts (§5) and which
  invents no placeholder field. The repo library's GraphQL parser only recognised `type X {`, so it
  read an owned empty schema as "type absent"; fixed at
  `📚️library/🧬️schema/🔍️field-discovery/🔗️graphql/🟦️.ts:6-12`.
* **The Rust leaf is derive-free and dependency-free.** Authored leaves carry
  `#[derive(…, ArtifactSchema)]` + `#[state(config)]`, but `#[state(…)]` is consumed by that derive and
  `rustc` rejects it without one. A projected leaf must compile inside *any* of the 34 plugin crates
  without editing its `Cargo.toml`, so the state class rides a `/// 🏷️ @state <lane>` doc comment and
  the machine-read class stays on the normative `🔣️.json`'s `x-semio-state`. Proven by compiling every
  projected leaf with `rustc -D warnings` (§5).

### 2.4 The Rust mount — why the count would otherwise not move

Measured before designing (probe in `🗑️generated`, reproduced in `🐍️v3a-validate-plugin.ts`): dropping
an unmounted `🦀️.rs` into a lane converts one `missing 🧬️schema/` finding into **six** (five missing
leaves + one `is not reachable from Cargo manifest`). Writing the leaves without their mounts would
have *raised* the total.

`mountProjectedRustLeaf` therefore locates the plugin's own wiring module by resolving every literal
`#[path = "…"]` against its declaring file's directory until one equals the surface component leaf —
no name matching, no guessing — and inserts the canonical sibling block after that mount's
`pub use component::*;`. It **refuses** (reports, writes nothing) when the anchor is not the canonical
`mod component; pub use component::*;` shape or when the block already declares `pub mod config` /
`pub mod presence`. 103 wiring files were edited this way, all additive; one lane was refused on the
"already declares" rule and is listed in §6.

### 2.5 Idempotence, ownership, `--check`

Every leaf carries `SURFACE_SCHEMA_BANNER`. A lane whose leaves lack it is `authored` and is **never**
rewritten (48 lanes — all the hand-carved ones with `app_schema_descriptor()`, `$defs`, TS guards and
reserved proto fields stay exactly as they are). A banner-carrying leaf that differs from the
projection is `drift`; `--check` prints each drifted path and exits 1. Verified: second run reports
`0 written, 2 current`, and `--check` exits 0 (`🗑️generated/v3a-all-run.txt`).

### 2.6 Progress and cancellation

`runSurfaceSchemaProjection` takes an `AbortSignal` and an `onProgress(done, total, label)`; the CLI
wires `SIGINT`/`SIGTERM` to the controller and prints every 25 lanes. Cancellation throws naming the
lane count reached, so a half-run is visible rather than silent.

### 2.7 Performance — the first repo-wide run did not terminate

The first `--plugin 🗄️stdio` attempt exceeded 600 s and was killed: source resolution and mount
lookup each walked and re-parsed the whole plugin tree **per lane**, which for stdio is ~6 000 Rust
leaves × 352 lanes. Fixed with three per-run indexes (`RUST_FILE_INDEX`, `STRUCT_INDEX`,
`MOUNT_INDEX`, `🧬️surface-schema/🟦️.ts:133-170` and `:345-360`). The mount index deliberately caches
only the *file*, never the line number, because the projection inserts lines into those same files.
stdio now runs in **15 s**, the whole repo in **39 s**.

## 3. Lanes with no source of truth (nothing invented)

> **Superseded by §10.** This list is session 3's 5; it is **2** tonight — only the two `🪐️space`
> rows below survive. `🔋️energy`'s two were a generator defect, not a missing truth (§11); `🖍️draw`'s
> was fixed by the 11:24 anchor generalisation.

5 of 574, all reported by the run, none written:

| plugin / lane | reason | what the Rust says the schema should be |
|---|---|---|
| `🪐️space ✳️any/✏️editor` config | composite fields | `SpaceIndexConfig { members: Vec<SpaceIndexMember>, indexed_artifacts: Vec<SpaceArtifactRow>, presence: SpaceIndexArtifactPresence }` — needs three authored nested `$defs` |
| `🪐️space ✳️any/👁️viewer` config | `u64` field | `directory_authorization_generation: u64`; `policyJsonSchemaScalar` has no `uint64` format, so emitting one would break the round-trip parity the projection guarantees |
| `🔋️energy` (2 lanes) | no `type Config`/`type Presence` binding on the surface component | the surface declares no associated state type at all — an authoring gap, not a schema gap |
| `🖍️draw` (1 lane) | wiring module already declares `pub mod config` for this surface | the mount block exists with different contents; merging it is an authored edit, not a projection |

Per the brief's step 3: **no JSON schema was generated from a Rust type for these**, because the
repo has no Rust→JSON-Schema emitter to do it with — the only Rust-side schema machinery is the
`ArtifactSchema` runtime derive, and the root `📜️script.ts schema generate` writes the scope catalog,
not facet leaves (§1). Inventing the nested `$defs` by hand is the fabrication V1 §3.4 refused.

## 4. Before / after per plugin (schema-lane family only)

From the two real `check` captures, counted by the finding's plugin prefix:

| plugin | before | after | | plugin | before | after |
|---|---|---|---|---|---|---|
| `🗄️stdio` | 352 | **0** | | `🪵️sourcing` `🎞️animate` `🏗️fem` `🎪️demonstrator` `📐️cad` `🏛️architect` `🌿️vcs` `📖️playbook` `📜️imperative` `🕸️dag` `🌍️gis` `🏭️process` `🎥️shooting` `🖨️raster` `💠️lowpoly` | 4 each | **0** each |
| `📕️norm` | 72 | 10 | | `📸️remodel` `🌊️flow` `✒️writer` `🌀️procedural` `➗️mathematical` `💡️reasoning` | 2 each | **0** each |
| `🀄️wfc` | 30 | **0** | | `🔋️energy` | 8 | 4 |
| `🎬️sequence` `📏️layout` | 14 | **0** | | `🪐️space` | 8 | 2 |
| `🧱️block` `🧩️puzzle` | 12 | **0** | | `🖍️draw` | 4 | 1 |
| `🔱️trinity` | 11 | **0** | | **TOTAL** | **627** | **17** |
| `📋️forms` `🗒️note` | 9 | **0** | | | | |

Run order was as briefed: `🗄️stdio` first (352 → 0, capture `v3a-stdio-dryrun.txt`), then every plugin
(`v3a-all-run.txt`), re-measuring with `🐍️v3a-validate-plugin.ts` after each
(`v3a-perplugin-before.txt`, `v3a-perplugin-after.txt`) and finally the authoritative full `check`.

The 17 survivors are the 5 blocked lanes' leaves plus `📕️norm`'s 10, whose lanes sit under window and
mode owners whose wiring anchors are not the canonical shape — §6.

## 5. Tests and oracles

New vitest suite `📇️registry/🧪️tests/🧬️surface-schema/🟦️.ts` over the frozen fixture
`📇️registry/🧫️fixtures/🧬️surface-schema/🔣️.json` (3 cases: empty, scalars, collections+optionals).
**8/8 passing, run and captured:**

```
$ bunx vitest run --config 🧪️tests/🎚️config/🟦️.ts 🧪️tests/🧬️surface-schema/🟦️.ts
 Test Files  1 passed (1)      Tests  8 passed (8)
```

* **Language-agnostic fixture test** — the fixture is pure JSON data (`source` Rust text in, five
  `expected` leaf bodies out); the assertion is byte equality of the projector's output against the
  frozen bytes. Any spelling change in any of the five formats fails it.
* **Third-party oracle 1 — Ajv** (independent draft-07 implementation): compiles every projected
  `🔣️.json`, accepts a conforming instance built from `required`, and rejects an instance carrying an
  undeclared field (proving `additionalProperties: false` really bites).
* **Third-party oracle 2 — `graphql`** (the GraphQL reference implementation, the dev dependency T2
  installed): `parse()` and `buildSchema()` every projected `🔗️.graphql` against the framework's own
  `GRAPHQL_STATE_PREAMBLE`, and assert the named type is in the built schema. This is what proves the
  braceless empty-type spelling is real SDL.
* **Third-party oracle 3 — `rustc`**: compiles every projected `🦀️.rs` standalone with
  `--crate-type lib --edition 2021 -D warnings`, exit 0. This is what proves the derive-free leaf
  compiles in any crate without a manifest edit.
* **Cross-format parity through the repo library's own parsers** — reads each projected leaf back with
  `policyExtract{Rust,Graphql,Protobuf,JsonSchema}SchemaFields` and asserts identical type name and
  identical `name:optional:cardinality` for every field, plus `state === lane` on the normative leaf.
  This is the projection satisfying `📏️field-parity`, `🏷️type-name-parity` and `💧️state-purity` by
  construction rather than by claim.
* Two unit laws for the refusal paths: binding resolution through `::` paths, and the three shapes the
  projection refuses (composite type, map, optional-repeated).

`🐍️v3a-freeze-fixture.ts` (ticket folder) re-freezes the fixture from the live projector.

## 6. Honest gaps

> **Read §15 first.** Gaps 1 and 2 below were already **stale when they were written** — the fixes
> landed at 11:17–11:26 but the report was last saved at 11:14 and the session died at 11:36. Gap 3
> is re-examined in §15. Gaps 4–6 stand.

1. ~~**Not compiled inside a plugin crate.**~~ **CLOSED** — see §15.6: five real crates green,
   including `cargo check -p semio-s-artifact-energy-model` exit 0 tonight.
   *Original text, kept for the record:* the leaves compiled standalone under `rustc -D warnings`
   (§5) and were reported reachable, but `cargo check -p semio-s-artifact-raster-raster` could not be
   completed — `semio-framework-plugin` was red from a peer's live refactor, 5 ×
   `E0609: no field 'faulted' on type ActiveArtifactStoreReplacement`
   (`🗑️generated/v3a-cargo-raster.txt`), failing before any plugin crate was reached. That peer
   breakage has since cleared, which is what made §15.6 possible.
2. ~~**`📕️norm`'s 10 survivors** are window- and mode-level owners whose wiring is not the canonical
   anchor shape.~~ **WRONG DIAGNOSIS, and CLOSED** — the anchor grammar was already generalised at
   11:24 (`findSiblingModule`'s brace-depth scan, `🧬️surface-schema/🟦️.ts:436-451`) and no lane is
   refused for a mount reason any more. The 10 had a completely different cause: 13 empty residue
   directories left by a 2026-09-12 deletion. See §10.
3. **No `generatorContracts` entry in `🔣️taxonomy.json`.** V2 §3.3's recipe called for one, and it is
   not expressible: `outputRoots` must be **literal, glob-free, non-overlapping** repository paths
   (`📚️library/🧹️normalization/🟦️.ts:1549-1552` and the overlap check at `:1566-1570`), so this
   contract would need ~2 600 literal entries, and the only coarse path that covers them —
   `✏️s/🔌️plugins` — overlaps `external-step-assets`' seven roots. The generator is wired as an nx
   target and a launch row instead. Giving the taxonomy a pattern-based output root is a real
   follow-up for the taxonomy owner, and until then `check-generated` does not cover these leaves —
   `surface-schema --check` does.
4. **48 authored lanes are untouched by design**, so they remain hand-maintained and can still drift
   from their Rust types. Adopting them means regenerating richer content (`app_schema_descriptor()`,
   `$defs`, TS guards) — a separate, larger contract.
5. **The `📜️.wit` fix changes the artifact-level schema facet too** (`🗿️artifacts/<a>/🧬️schema/`), not
   only surface lanes, because all three call sites now resolve the facet kind. That is the taxonomy's
   declared law (`schemaFacetKinds.🧬️data` has five formats; `📜️interface`/wit is granted to exactly
   two enumerated `facetPathIdentities`), and no finding was *added* by it — but it is a wider blast
   radius than the family I was given.
6. The total 1836 → 684 is **not all mine**; see §0 attribution.

## 7. Files changed

**Root fix — the gate demanded a leaf its own taxonomy forbids**

| file:line | change |
|---|---|
| `…/📇️registry/🗿️taxonomy-validation/🟦️.ts:103-112` | new `taxonomySchemaFilenames(facetAbs)`: resolves a facet's required leaves through `schemaFacetFormatEntries` (the facet's **declared kind**) instead of the flat `schemaFormats` map, replacing `TAXONOMY_SCHEMA_FILENAMES` |
| `…/🗿️taxonomy-validation/🟦️.ts:4` | import `getWorkspaceRoot`, `schemaFacetFormatEntries` |
| `…/🗿️taxonomy-validation/🟦️.ts:393, 654, 673` | the three call sites now ask per facet — removes 53 false `missing …/📜️.wit` findings |

**The generator**

| file | change |
|---|---|
| `…/📇️registry/🧬️surface-schema/🟦️.ts` | **new**, 400 lines: contract, owner discovery, source resolution + indexes, five-format projection, Rust mount insertion, run loop with progress/cancel, `SurfaceSchemaScript` |
| `…/📇️registry/📜️script.ts:9,11` | registers the `surface-schema` verb |
| `…/📇️registry/📋️project.json` | `surface-schema` + `surface-schema-check` targets |
| `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json` | one launch row, regenerated via `@semio-tech/plugin-registry:generate` |

**Repo library**

| file:line | change |
|---|---|
| `📚️library/🧬️schema/🔍️field-discovery/🔗️graphql/🟦️.ts:6-12` | recognise the braceless `type X` an empty schema must use; previously read as "type absent" |

**Tests / fixtures**

| file | change |
|---|---|
| `…/📇️registry/🧪️tests/🧬️surface-schema/🟦️.ts` | **new**, 8 tests incl. Ajv / graphql / rustc oracles |
| `…/📇️registry/🧫️fixtures/🧬️surface-schema/🔣️.json` | **new**, 3 frozen projection vectors |

**Generated tree** — 519 lanes × 5 leaves under `✏️s/🔌️plugins/*/…/{🎚️config,👥️presence}/🧬️schema/`,
plus additive `#[path]` mount blocks in 103 plugin wiring modules.

**Ticket folder** — `🐍️v3a-lane-census.ts`, `🐍️v3a-validate-plugin.ts` (fast per-plugin gate, 25 s vs
the full gate's 4.5 min), `🐍️v3a-freeze-fixture.ts`; captures `v3a-check-before.txt`,
`v3a-check-after.txt`, `v3a-perplugin-before.txt`, `v3a-perplugin-after.txt`, `v3a-stdio-dryrun.txt`,
`v3a-all-dryrun.txt`, `v3a-all-run.txt`, `v3a-lane-census.txt`, `v3a-cargo-raster.txt`.

---

# Session 4 (2026-09-19 evening) — re-measured, gaps closed

Sessions 1–3 died at ~11:36; §§0–7 above stop there, and §6 gaps 1 and 2 were already **stale when
written** (the worker landed the fixes at 11:17–11:26 but never edited the report). Everything below
is re-measured tonight against the tree as committed, nothing inherited on trust.

## 8. What is actually in the tree (verified, not claimed)

`git log` shows the whole slice is already **committed** by the repo's auto-commit — `git diff --stat`
on my paths is empty because there is nothing *un*committed, not because nothing landed. Verified by
existence + `git log --oneline -- <path>`:

| artifact | state |
|---|---|
| `…/📇️registry/🧬️surface-schema/🟦️.ts` | present, 33 355 bytes, last written 11:24 |
| `…/📇️registry/🧪️tests/🧬️surface-schema/🟦️.ts` | present, 7 899 bytes |
| `…/📇️registry/🧫️fixtures/🧬️surface-schema/🔣️.json` | present, 8 143 bytes |
| `…/📇️registry/📜️script.ts:9,11` | `import { SurfaceSchemaScript }` + `.register("surface-schema", SurfaceSchemaScript)` — **confirmed in the live file** |
| `…/📇️registry/📋️project.json:91-108` | `surface-schema` (`cache: false`) + `surface-schema-check` (`cache: true`) targets |
| `.vscode/launch.json:4449,4460` + `.vscode/🧩️launch.seed.jsonc:2530,2541` | `📦️generate🧬️surface-schema` / `📦️check🧬️surface-schema` rows, both present in seed **and** generated file |
| `✏️s/🔌️plugins` working tree | 199 paths still unstaged/staged at session start — the projection's leaves and mount blocks |

## 9. Termination, progress, cancellation, idempotence — measured tonight

```
$ bun ./📜️script.ts surface-schema --check          # capture v3a-s4-check-mode.txt
[surface-schema] 25/574 🌀️procedural surface "✳️any/👁️viewer" config
… 23 progress lines …
[surface-schema] 574/574 🪵️sourcing surface "✳️any/✏️editor" presence
[surface-schema] 574 lane(s): 0 written, 522 current, 48 authored, 4 blocked, 0 drifted
EXIT=0                                               real 2m19s
```

* **Terminates repo-wide**: yes, 574/574, exit 0. (§2.7's non-termination is dead; the three indexes
  hold. 2 m 19 s tonight vs the 39 s of §2.7 is fleet load, not regression — this run had the full
  `check` gate and peer slices competing for 10 cores.)
* **Progress**: one line per 25 lanes plus a forced final line, naming plugin + surface + lane.
* **Cancellation**: `SIGINT`/`SIGTERM` are wired to an `AbortController` whose signal is polled at the
  top of every lane and throws `surface-schema-projection: cancelled after N/574 lane(s)`
  (`🧬️surface-schema/🟦️.ts:477`). Reviewed in source; not exercised by a live signal tonight — the
  honest wording is "implemented and code-read", see §13.
* **Idempotent**: `0 written, 0 drifted` on a second independent run, exit 0. This is the real
  idempotence proof — the run re-derives all 574 lanes from the Rust and finds every byte already
  equal.

Re-run after tonight's fixes (§10, §11), so idempotence is proven for the **changed** generator too
(`v3a-s4-check-mode-2.txt`, 1 m 21 s):

```
[surface-schema] 572 lane(s): 0 written, 522 current, 48 authored, 2 blocked, 0 drifted
EXIT=0
```

572 not 574 because the two `📕️norm` plugin-root lanes ceased to exist with their residue
directories; 2 blocked not 4 because `🔋️energy` is closed. Both blocked lines are `🪐️space` (§10).

## 10. The schema-lane family tonight: 16 → **2**, each survivor root-caused

`§4`'s "17 survivors" was the session-3 estimate. Re-counted tonight with the registry's **own**
`validateTaxonomyTree` (`🐍️v3a-validate-plugin.ts`, capture `v3a-s4-perplugin.txt` — 34 plugins,
632 findings total), the family stood at **16**, in exactly three places:

| plugin | findings | why |
|---|---|---|
| `📕️norm` plugin-root | 10 | `🎚️config/` + `👥️presence/` exist as **empty directories** |
| `🔋️energy ✳️any/✏️editor` config | 4 | lane has an authored `🔣️.json`, the four mirrors absent |
| `🪐️space ✳️any/{👁️viewer,✏️editor}` config | 2 | genuinely inexpressible — §3 |

`📕️norm`'s 10 were **not** the §6.2 anchor-grammar problem at all (that was fixed at 11:24 by
`findSiblingModule`'s brace-depth scan; no lane is refused for a mount reason any more). The real
cause, found by history rather than guessed:

```
$ git log --diff-filter=D --name-status -1 -- ✏️s/🔌️plugins/📕️norm/🎚️config
8add1df147  2026-09-12 21:17:59 +0200
D  ✏️s/🔌️plugins/📕️norm/🎚️config/🦀️.rs
D  ✏️s/🔌️plugins/📕️norm/🎚️config/🧬️schema/{🔣️.json,🦀️.rs,🟦️.ts,🔗️.graphql,🛰️.proto}
D  … 13 more
```

A commit on **2026-09-12** deleted norm's whole plugin-root config lane — correctly: norm's app
surface declares `ArtifactEditor<Config = NoConfig, ConfigMutation = NoConfigMutation>`
(`📕️norm/🖥️app-surface/🦀️.rs:451`), so the plugin root owns no config. What survived was checkout
residue: **13 empty directories, 0 files, 0 tracked paths** (`git ls-files` on both trees returns
nothing; `find … ! -type d` returns nothing). The gate admits a plugin-root as a schema owner purely
because `🎚️config/` *exists*, so 13 empty dirs manufactured 10 findings. Removing the residue
finishes a deletion the repo already made — it invents nothing and git is unaffected (nothing was
tracked). `📕️norm` 123 → 113 findings, schema-lane 10 → **0**.

`🪐️space`'s 2 are the only true residual, and they are the §3 pair, unchanged and un-inventable:

| lane | the Rust says | the decision the owner must take |
|---|---|---|
| `🪐️space ✳️any/✏️editor` config | `SpaceIndexConfig { members: Vec<SpaceIndexMember>, indexed_artifacts: Vec<SpaceArtifactRow>, presence: SpaceIndexArtifactPresence }` | **author three nested `$defs`** (one per composite) in the lane's `🔣️.json`, then the four mirrors project from it. Nobody but the space owner can decide those three nested shapes' field sets. |
| `🪐️space ✳️any/👁️viewer` config | `directory_authorization_generation: u64` | **grant the scalar table a `uint64`**: `policyJsonSchemaScalar` (`📚️library/🧬️schema/🔍️field-discovery/🔣️json-schema/🟦️.ts:11-16`) knows `int32/uint32/int64` and no `uint64`, so emitting one breaks the round-trip parity the projection guarantees. Either the library gains `uint64` (a repo-library owner decision, it changes every format's parser) or the field becomes `u32`/`i64` (a space owner decision). |

I did not take either decision. Fabricating them is exactly what V1 §3.4 refused.

## 11. `🔋️energy` — a real generator defect, fixed

The §2.5 ownership rule read *"any authored leaf ⇒ the lane is authored, never write"*. That is right
for the 48 complete hand-carved lanes and **wrong for a lane with holes**: `🔋️energy`'s config facet
carries a hand-carved `🔣️.json` (with `minimum`/`maximum`/`default`/`enum` the projection cannot
express) and **no mirrors at all**, so the generator ceded the lane and the gate kept reporting the
four absent leaves forever. No run, however many times repeated, could ever close them.

The rule is now *"a lane is ceded only when it is **complete**"*
(`🧬️surface-schema/🟦️.ts:470-483`, new exported predicate `surfaceSchemaLaneOwnership`):

| lane on disk | classification | what is written |
|---|---|---|
| all leaves banner-carrying | projected | the whole lane, on drift |
| all leaves present, some authored | **ceded** | nothing (the 48 hand-carved lanes, unchanged) |
| some authored, some **absent** | **hollow** | only the absent ones; every authored leaf is byte-untouched |
| all absent | projected | the whole lane |

Source of truth for the holes is the **ordinary** one — the surface's `type Config = EnergyModelConfig`
binding → the real `pub struct` — not the hand-carved JSON. That matters: the authored JSON spells
`"type": "integer"` with no `format` (→ `int32`) while the struct declares `u32`
(`🔋️energy/…/🎚️config/🦀️.rs:29-31`). Projecting from the struct keeps the mirrors **true to the
type**; `📏️field-parity` compares name, optionality and cardinality — *not* scalar
(`⚖️laws/📏️field-parity/🟦️.ts:28-45`) — so the loose JSON and the exact mirrors are parity-identical.

```
$ bun ./📜️script.ts surface-schema --plugin 🔋️energy
[surface-schema] 4 lane(s): 1 written, 3 current, 0 authored, 0 blocked, 0 drifted
```

Four mirrors written, the authored `🔣️.json` untouched (`git diff --stat` on it is empty), and the
mount landed on the lane's own `🦀️.rs` as `#[path = "🧬️schema/🦀️.rs"] pub mod schema;` — so
`🔋️energy` went 4 → **0 findings with unreachable still 0**.

**Owner follow-up, named not silently fixed:** the authored `🔣️.json` should gain
`"format": "uint32"` on its three integer properties to match `EnergyModelConfig`. That is an edit to
a hand-authored leaf the projection deliberately does not own.

## 12. The gate, re-measured end to end

```
$ bun ./📜️script.ts check                            # capture v3a-s4-check.txt, 619 finding lines
schema-lane family (mine)           2      (🪐️space ×2, §10)
missing …/📜️.wit                     0
not reachable from Cargo manifest  333      (V3b's family)
```

| gate | 03:59 | session 3 | **tonight** |
|---|---|---|---|
| `plugin-registry check`, total | 1836 | 684 | **619** |
| … schema-lane family (mine) | **627** | 17 | **2** |
| … `📜️.wit` inside it | 53 | 0 | **0** |

**615 of the 627 closed; the 2 that remain need a decision nobody but an owner can take (§10).**
Cross-checked independently: the fast per-plugin audit over the same `validateTaxonomyTree` reports
the identical 2 (`v3a-s4-residual-after.txt`), so the number is not an artefact of one run.

## 13. The test that discriminates

The suite is now **9 tests, all run and captured** (`v3a-s4-vitest.txt`):

```
$ bunx vitest run --config 🧪️tests/🎚️config/🟦️.ts 🧪️tests/🧬️surface-schema/🟦️.ts
 Test Files  1 passed (1)      Tests  9 passed (9)
```

The new one, `cedes a complete authored lane but fills the holes of a partly authored one`, builds
four real facet directories on disk (all-projected / all-authored / **hollow** / empty) and asserts
the full ownership verdict for each. **Proven to discriminate, not merely to pass** — I reverted the
predicate to the old rule (`ceded: authored.length > 0`) and re-ran:

```
AssertionError: a lane with holes must never be ceded — that is how 🔋️energy kept 4 findings:
expected true to be false
 Tests  1 failed | 8 passed (9)
```

then restored it and re-ran to 9/9. The other 8 (frozen fixture bytes, Ajv, the `graphql` reference
implementation, `rustc -D warnings`, cross-format parity through the repo library's five parsers, and
the two refusal laws) are §5, unchanged and re-run tonight.

## 14. The 30 capability-catalog descriptor skips (A1 §3) — this projection fixes **none** of them

The brief asked which of A1 §3's skips my projection closes as a side effect. Measured answer, so it
is not mistaken for progress: **zero**, and it could not be otherwise. Those skips are about a
*different artifact*. A1 §3 root-causes them as (a) 16 plugins with no `🔣️.json` **package
descriptor** at the plugin root, and (b) 15 committed descriptors whose emitted JSON is a stale
generation (`io.documentSchema` where `manifest::AppIo` now requires `artifactSchema`). That
`🔣️.json` is emitted by `describe`, which builds the plugin's `wasm32-wasip2` component,
jco-extracts its core module and self-hashes the pair
(`🔌️plugin/🖨️describe/🛂️descriptor-emission/🟦️.ts:119-133`). Nothing in that pipeline reads a
`🎚️config/🧬️schema/` lane — I grepped `🖨️describe` for `🧬️schema` and the only hits are its own
unrelated `🧬️schema/🔣️.json` in three test files. Closing them is regeneration per plugin, ≥ 3 h of
serial wasm builds plus an emitter defect, exactly as A1 says. **It is not this slice.**

The honest relationship runs the *other* way, and it is a risk this slice created, not a gift:
`describe` compiles the plugin crate, and the projection has now put **523 new `pub struct` leaves
and their `#[path]` mounts inside those same crates**. If one of them did not compile, every
`describe` in that plugin would fail and the skip list would get *worse*. That is why §15 cares about
real-crate compilation rather than the standalone `rustc` oracle.

## 15. Honest gaps after tonight

1. **`🪐️space`'s 2 findings are open and need an owner decision** — §10 names both exactly. I did not
   take them. This is the exact residual list the brief asked for, and it is 2, not 0.
2. **§6.3 stands, verified against the code tonight.** A `surface-schema-projection`
   `generatorContracts` entry is still not expressible. The two constraints are literal
   (`🧹️normalization/🟦️.ts:1550` — `must be one literal NFC repository path`, rejecting `* ? [ ]`)
   and non-overlapping (`:1563-1567` — the pairwise `startsWith(\`${b.path}/\`)` loop). 572 lanes × 5
   leaves is ~2 860 literal roots, and the only coarse path that covers them, `✏️s/🔌️plugins`, would
   overlap the 7 roots `external-step-assets` already owns there (confirmed by reading the live
   contract table: 23 contracts, `external-step-assets` holds 12 literal `.stp` paths, 7 under
   `✏️s/🔌️plugins`). **The owner decision, precisely:** the taxonomy already accepts a generator whose
   *inputs* are discovered rather than enumerated — `plugin-registry` carries
   `inputDiscovery.kind = "registry-catalog"` — but has no equivalent for *outputs*. Either grant
   `outputRoots` a discovery/pattern form, or accept ~2 860 literal entries. Until then
   `check-generated` does not cover these leaves; `surface-schema --check` (nx target
   `surface-schema-check`, launch row `📦️check🧬️surface-schema`) does, and it is green.
3. **Cancellation is code-read, not signal-exercised.** The `AbortSignal` is polled per lane and the
   CLI wires `SIGINT`/`SIGTERM` (`🧬️surface-schema/🟦️.ts:484`, `:540-543`). I did not deliver a live
   `SIGINT` to a running repo-wide projection tonight; the run only takes 1–2 min, so a half-run is
   cheap but the path is unproven at runtime. Say "implemented", not "observed".
4. **§6.4 stands**: the 48 complete authored lanes are still hand-maintained and can drift from their
   Rust. §11 narrows this — a lane with *holes* is no longer ceded — but a complete authored lane is
   still nobody's generated output. `🔋️energy`'s `🔣️.json` missing `"format": "uint32"` (§11) is one
   live instance of that drift, found only because I looked.
5. **§6.5 stands** (the `📜️.wit` fix widened to the artifact schema facet) and **§6.6 stands** (the
   headline total is shared with V3b — tonight's 619 total is 333 unreachable-mount findings, which
   are V3b's, plus 284 others; my family is 2 of the 619).
6. **§6.1 is CLOSED — the mounted leaves compile in their real crates.** This was the one claim
   session 3 said it could not make. It is made now, and measured:

   ```
   $ cargo check -p semio-s-artifact-energy-model          # capture v3a-s4-cargo-energy.txt
       Checking semio-s-artifact-energy-model v0.1.0 (…/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust)
   warning: `semio-s-artifact-energy-model` (lib) generated 1 warning
       Finished `dev` profile [unoptimized] target(s) in 37m 25s     EXIT=0
   ```

   The single warning is **not mine** — `unnecessary qualification` at
   `…/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs:40`, a peer's pre-existing file. The projected leaf
   produced **zero** diagnostics: `grep "🧬️schema/🦀️.rs"` over the whole capture returns nothing.
   Corroborating evidence, all captured:
   * session 3 already had four more real crates green — `semio-s-artifact-{stdio-csv,puzzle-2d,
     trinity-jack,wfc-2d}` all reached `Finished \`dev\` profile` with projected leaves mounted
     (`v3a-cargo-stdio-csv.txt` 11:17, `v3a-cargo-widen.txt` 11:26). Those captures **post-date** the
     report's last save at 11:14, which is the only reason §6.1 reads as if nothing compiled;
   * the new leaf also compiles standalone under `rustc --crate-type lib --edition 2021 -D warnings`,
     exit 0 — the same oracle the suite applies to every fixture case;
   * `🔋️energy` reports `unreachable 0` after the mount, so the `#[path]` resolves.

   **Still not claimed:** all 34 plugin crates. Five are proven; the remaining 29 are inferred from
   the fact that every leaf is the same dependency-free `pub struct` in its own `mod`, and from the
   gate's rustc-validated reachability analysis. Also **untested on `wasm32-wasip2`**, which is the
   target `describe` actually builds (§14) — a plain `cargo check` is the host target. Cost note for
   whoever widens this: **37 m 25 s for one crate** under fleet load, nearly all of it the shared
   framework dependency graph and the build-dir lock, not the leaf.
7. **My module typechecks clean.** `tsc 5.9.3 --strict --noEmit` over
   `🧬️surface-schema/🟦️.ts` reports **0 errors in that file** (`v3a-s4-tsc.txt`); the 4 errors it does
   print are pre-existing, in `🔎️discovery/🟦️.ts`, `🗿️taxonomy-validation/🟦️.ts` and an os test file —
   peer territory, untouched by me.

## 16. Files changed this session

| file:line | change |
|---|---|
| `…/📇️registry/🧬️surface-schema/🟦️.ts:470-483` | **new exported `surfaceSchemaLaneOwnership`** — the four-way lane ownership verdict (`authored` / `absent` / `hollow` / `ceded`) |
| `…/📇️registry/🧬️surface-schema/🟦️.ts:499-504` | the run loop cedes a lane only when `ceded` (complete), so a partly authored lane gets its holes filled |
| `…/📇️registry/🧬️surface-schema/🟦️.ts:512` | `stale` excludes every authored leaf, so a hole-fill can never overwrite hand-carved bytes |
| `…/📇️registry/🧬️surface-schema/🟦️.ts:485-493` | run-loop docstring records the completeness rule and why parity permits it |
| `…/📇️registry/🧪️tests/🧬️surface-schema/🟦️.ts:121-148` | **new test** `cedes a complete authored lane but fills the holes of a partly authored one` — 4 real facet dirs; proven to fail under the old rule (§13) |
| `✏️s/🔌️plugins/📕️norm/{🎚️config,👥️presence}/` | **removed** — 13 empty directories, 0 files, 0 tracked paths; residue of the 2026-09-12 deletion `8add1df147` (§10) |
| `✏️s/🔌️plugins/🔋️energy/…/✳️any/✏️editor/🎚️config/🧬️schema/{🦀️.rs,🟦️.ts,🔗️.graphql,🛰️.proto}` | **new**, projected from `EnergyModelConfig`; the authored `🔣️.json` untouched |
| `✏️s/🔌️plugins/🔋️energy/…/✳️any/✏️editor/🎚️config/🦀️.rs` | `#[path = "🧬️schema/🦀️.rs"] pub mod schema;` mount appended |
| `📜️script.ts:19015-19016` | **removed** the two dead `POLICY_PLUGIN_CLOSED_SHAPE_DESTINATIONS` rows for `📕️norm/{🎚️config,👥️presence}` — they key a pure lookup (`:19104`, `:19125`, both `?? fallback`, never iterated) on paths §10 removed, so they were unreachable. The `⚖️compliance` and `🖥️app-surface` rows, whose ruling is still live, are untouched. |

Nothing else was touched. `📜️script.ts`, `📋️project.json`, `.vscode/launch.json` and
`.vscode/🧩️launch.seed.jsonc` needed no edit — the registrations from session 3 are present and
verified (§8).

**Verified at runtime vs. by tests only.** Run and captured tonight: the full `check` gate (619),
the per-plugin audit (632 over 34 plugins), two repo-wide `surface-schema --check` runs, the
`🔋️energy` projection run, `cargo check -p semio-s-artifact-energy-model` (exit 0), the standalone
`rustc -D warnings` on the new leaf, `tsc --strict`, and the 9-test vitest suite including a
mutation proof that the new test fails under the old rule. **Not** exercised at runtime: a live
`SIGINT` into a running projection (§15.3), the other 29 plugin crates, and any `wasm32-wasip2`
build (§15.6). No claim in this report rests on a command I did not run.

**Captures** (`🗑️generated/`): `v3a-s4-check.txt` (the authoritative 619-finding gate),
`v3a-s4-perplugin.txt`, `v3a-s4-residual.txt`, `v3a-s4-residual-after.txt`, `v3a-s4-check-mode.txt`,
`v3a-s4-check-mode-2.txt`, `v3a-s4-energy-run.txt`, `v3a-s4-vitest.txt`, `v3a-s4-cargo-energy.txt`.
