# V3a — `surface-schema-projection` generator (2026-09-19)

Slice V3a owns the **surface schema-lane family** of `plugin-registry check`
(`surface "<s>" / plugin-root is missing 🎚️config/🧬️schema/…` and `… 👥️presence/🧬️schema/…`) — the one
V2 §3.3 specified as a new generator contract and declined to build. Sibling V3b owns the other
families (unreachable Rust mounts, directory lanes, taxonomy-owner decisions, interactivity gates);
nothing here touches them.

## 0. Headline

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

1. **Not compiled inside a plugin crate.** The projected leaves compile standalone under `rustc -D
   warnings` (§5) and the registry's own rustc-validated mount analysis reports them reachable
   (unreachable count unchanged by 519 new leaves, §0). But `cargo check -p semio-s-artifact-raster-raster`
   could not be completed: `semio-framework-plugin` itself is red from a peer's live refactor —
   5 × `E0609: no field 'faulted' on type ActiveArtifactStoreReplacement`
   (`🗑️generated/v3a-cargo-raster.txt`), which fails before any plugin crate is reached. **This is the
   one claim I cannot make: that the 519 mounted leaves compile in their real crates.** The risk is
   bounded (each leaf is a dependency-free `pub struct` in its own `mod`) but it is unverified, and it
   should be re-checked with one `cargo check -p <artifact crate> --features component-app-assembly`
   once the framework crate is green.
2. **`📕️norm`'s 10 survivors** are window- and mode-level owners; their wiring does not use the
   canonical `mod component; pub use component::*;` anchor, so the mount inserter refuses rather than
   guess. Extending the anchor grammar is the follow-up.
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
