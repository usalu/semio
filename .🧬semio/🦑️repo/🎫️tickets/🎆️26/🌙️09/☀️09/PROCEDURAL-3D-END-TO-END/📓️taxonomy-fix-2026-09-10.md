# Taxonomy Fix — Procedural (2026-09-10)

Execution lane closing the 640 procedural taxonomy violations catalogued in
`📓️taxonomy-violations-audit-2026-09-10.md`. Raw logs under `🗑️generated/tax-*.txt`.

## 0. Baseline, re-measured today

`bun 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts check` (exit 1,
19,464 lines repo-wide, `🗑️generated/tax-before.txt` for the procedural slice).

Procedural is **650** today, not the audit's 640 — one day of drift, all of it in the reachability
class (583 vs 573). Per class, live:

| class | audit (09-10 00:21) | this lane's baseline |
|---|---|---|
| leaf not reachable from Cargo manifest | 573 | **583** |
| window unexpected child (`🧪️tests`/`🎚️options`/`⚙️config`) | 42 | 42 |
| surface missing `🎚️config`/`👥️presence` schema | 12 | 12 |
| artifact missing `🧬️schema`/`🚪️io`/`⚙️engine`/`📚️examples` | 12 | 12 |
| plugin root missing `🎮️commands/🦀️.rs` | 1 | 1 |
| **total** | **640** | **650** |

§1 is the after-state; §2-§4 what changed; §5 what is left and why; §6 the gitlink remedy; §7 files;
§8 gate outputs; §9 follow-ups.

## 1. Headline

Procedural **650 → 22**. Repo-wide **19,464 → 2,123** lines from the same command. Every one of the
22 that remain is `🧩️assembly` (18) or a fixture-test-host adapter (4); nothing in `🧊️generation3d`
or `🌀️generation2d` is left. `plugin-registry:check` still exits 1 — repo-wide, on other plugins and
on assembly's own gaps.

| class | before | after | how |
|---|---|---|---|
| leaf not reachable from Cargo manifest | 583 | **14** | 556 were a detector bug (§2.1), 12 a second one (§2.2), 1 mounted by hand (§4.4) |
| window unexpected child `⚙️config` | 14 | **0** | 14 renames to `🎚️config` (§4.1) |
| window unexpected child `🎚️options` | 14 | **0** | 14 renames to `☑️options` (§4.1) |
| window unexpected child `🧪️tests` | 14 | **0** | 14 relocations to the owning mode (§4.2) |
| artifact missing `🧬️schema`/`🚪️io`/`⚙️engine`/`📚️examples` | 12 | **2** | walked at the wrong tree level (§3); the 2 left are assembly's real gaps |
| surface missing `🎚️config`/`👥️presence` schema | 12 | **6** | 6 `📜️.wit` facets authored (§4.3); 6 need surfaces that own no state yet (§5.2) |
| plugin root missing `🎮️commands/🦀️.rs` | 1 | **0** | authored with a real plugin-scope command (§4.5) |
| **total** | **650** | **22** | |

Two caveats stated up front, because they change what "cleared" means:

1. **556 + 12 of the 583 reachability findings were never real.** They are two bugs in
   `validateRustTaxonomyMounts`, proved by measurement, not argued from taste (§2). Fixing a
   detector that reports a leaf as unmounted when Cargo demonstrably compiles it is not policy
   loosening: no rule was relaxed, and the same 14 leaves that are genuinely unmounted still fail.
2. **12 of the artifact-facet findings were measured at a level the taxonomy no longer uses** — and
   `🔣️taxonomy.json` does not merely permit the level this lane moved them to, it forbids the old one
   (§3). Authoring what the stale check asked for would have created structure the taxonomy bans.

## 2. The reachability class (583 → 14)

### 2.1 Only one of four Cargo manifests was fed to the module graph

`validateRustTaxonomyMounts` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1162`)
built its module graph from **every `.rs` file under the plugin root but exactly one manifest**:

```ts
const graph = inspectRustModuleGraph([...sources, manifest], …, { strictManifests: true });
```

`manifest` is the plugin's own `📦️packages/🦀️rust/Cargo.toml`. Procedural has **four** Cargo
packages under that root — the plugin plus `🧩️assembly`, `🧊️generation3d`, `🌀️generation2d`, each an
independent crate whose `[lib] path = "../../🦀️.rs"` roots its own subtree. An artifact leaf is
mounted from **its own** crate root and can never be reachable from the plugin's, so the check was
asking for something the architecture forbids: the plugin crate *depends on* those crates, it does
not *contain* them.

Measured with `🐍️taxonomy-mount-probe.ts` (kept beside this report), which runs the identical
reachability rule twice over the same file set:

```
manifests: 4  components: 585
unreachable(plugin manifest only): 583
unreachable(all nested manifests): 27
```

**Fix.** `validateTaxonomyTree`'s own walk now collects `Cargo.toml` alongside `.rs`, and
`validateRustTaxonomyMounts` takes the manifest set (defaulting to the owner manifest alone, which
is what the `rust-taxonomy-mounts-check` oracle fixtures feed, so that oracle is untouched):

```ts
function validateRustTaxonomyMounts(pluginRoot, pluginId, componentFiles, sourceFiles, manifestFiles?)
const owned = (context) => context.manifestPath !== null && manifests.includes(context.manifestPath);
```

### 2.2 Non-library Cargo targets were invisible

Of the 27 survivors, 12 are reachable from a real Cargo target that is not `[lib]`:

- `🧪️tests/🔬️boot-deadline/🦀️.rs` and `🧪️tests/🚪️close-ladder/🦀️.rs` — both `[[test]]` in
  `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`.
- `…/📚️examples/🧪️tests/🧩️geometry/🦀️.rs` (`[[test]] example-geometry`) and
  `…/🚪️io/🧪️tests/🔁️round-trip/🦀️.rs` (`[[test]] io-round-trip`) in generation3d's manifest.
- the eight per-format serializer round-trip leaves, which `[[test]] io-round-trip`'s root mounts by
  `#[path]` (`…/🚪️io/🧪️tests/🔁️round-trip/🦀️.rs:181-196`).

`inspectRustCargoManifest` read only `[package]`, `[lib]` and `[dependencies]`, so a `[[test]]`
path was never a crate root. It now also returns `targetPaths` from `[[bin]]`/`[[test]]`/`[[bench]]`/
`[[example]]`, and `inspectRustModuleGraph` anchors each one to its manifest exactly as it anchors
`[lib]` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`).

### 2.3 What is genuinely unmounted (14, all reported, none hidden)

| leaf | why | disposition |
|---|---|---|
| `🧩️assembly/…/✳️any/{✏️editor,👁️viewer}/**` (10 files) | authored surfaces, crate root mounts neither | §5.1 — measured trait bounds |
| `…/✳️any/🧪️tests/🧊️mutate-procedural-3d-1/🦀️.rs` | fixture-case adapter | §5.3 |
| `…/✳️any/🧪️tests/🚪️io-procedural-3d-1/🦀️.rs` | fixture-case adapter | §5.3 |
| `…/✳️any/🧪️tests/🌀️mutate-procedural-2d-1/🦀️.rs` | fixture-case adapter | §5.3 |
| `…/🧩️mutate-assembly-1/🦀️.rs` | fixture-case adapter | §5.3 |

No leaf was deleted as a stale orphan. One was nearly treated as such and turned out to be
repairable instead — see §4.4.

## 3. The artifact-facet class — the check was walking a level the taxonomy forbids

`validateTaxonomyTree` required `🧬️schema/`, `🚪️io/`, `⚙️engine/` and `📚️examples/` **directly under
`🗿️artifacts/<artifact>/`**. Repo-wide census of what is actually on disk (92 artifacts):

| facet at `🗿️artifacts/<artifact>/` | present | check reported missing |
|---|---|---|
| `🧬️schema/` | 1 | 91 |
| `🚪️io/` | 0 | 92 |
| `⚙️engine/` | 0 | 92 |
| `📚️examples/` | 3 | 89 |

`🔣️taxonomy.json` says where they belong, in its own words
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, `_standardsSubsetsComment`):

> "Artifacts own standards only. Standards own subsets only. Subsets own schema, IO, and examples —
> never an engine: an artifact is data plus pure transforms, and behaviour belongs to the app that
> edits it (26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, #2553). Pure algorithms stay legal
> one level up, in a module's ⚙️engine …"

and states the same thing three more times as data:

```json
"newArtifactComponentDirs": ["🏅️standards"],
"standardComponentDirs":    ["🪆️subsets"],
"subsetComponentDirs":      ["🧬️schema", "🚪️io"],
"subsetChildDirs":          ["🧬️schema", "🚪️io", "📚️examples", "👁️viewer", "✏️editor"]
```

The registry check's own surface walk already knew (`…📇️registry/📜️script.ts:1385-1389`): *"No
⚙️engine facet requirement — the ENGINELESS ticket moved engine ownership to the module level."* Only
the artifact walk had not followed.

**Fix.** The artifact loop now descends `🏅️standards/<standard>/🪆️subsets/<subset>/` and applies
`subsetComponentDirs` + `📚️examples` there, requiring `🏅️standards/` on the artifact and `🪆️subsets/`
on the standard on the way down. `⚙️engine` flipped from *required* to *forbidden*, with the finding
naming the module-level home the taxonomy points at. Findings gained a `subset "<standard>/<subset>"`
segment, which also closes the attribution gap `📓️taxonomy-violations-audit-2026-09-10.md` §3 flagged
for surface/window labels.

`🧪️tests`/`🧫️fixtures` beside the example slugs are skipped, by name, from `TAXONOMY.testsDirName`
and `TAXONOMY.testFixturesDirName` — they are the taxonomy's own test dirs, not example slugs.

### 3.1 What was deliberately NOT moved with it, and why

The nested matrix underneath (`schemaDir`, `ioFacetDir`, `mutationsRoot`) stays artifact-rooted,
where it is inert. It is written against the same pre-W3 shape and would need its own rewrite before
it can be pointed at a subset: it expects io codec leaves directly under `<format>/` where the live
tree carries `<format>/🔖️<standard>/✳️<subset>/🦀️.rs`, one Rust leaf in every mutation facet where
`artifactSchemaSpecFileKinds` declares mutation `🧬️schema/` payloads are json-kind, and no
`🧪️tests`/`🧫️fixtures` anywhere. **Measured, not guessed:** re-pointing it at the subset was tried in
this lane and moved the repo from 2,123 to 9,420 finding lines (procedural 22 → 223), none of it
procedural's content. That is a repo-wide packet of its own; a comment at the code marks it and
points here.

## 4. What was hand-authored or moved (procedural content)

### 4.1 Window `⚙️config` → `🎚️config`, `🎚️options` → `☑️options` (28 dirs, 14 windows)

Rule quoted from the finding itself: a window may hold only
`🍱️panes, 🪀️widgets, 🪛️utilities, 🎬️actions, ☑️options, 🎚️config, 👥️presence, 🫧️transient`
(`TAXONOMY.windowChildDirs`, enforced at `…📇️registry/📜️script.ts:1435`). Both offending names are
near-misses of a member of that set, so both are renames, not relocations. Every one of the 28 dirs
held exactly one file, `📌️.empty.md`, so nothing else moved. All 14 windows across all three
artifacts were touched; none was skipped as hot. One stale docstring reference to the old names was
repaired (`…/🧊️generation3d/…/✳️any/✏️editor/🦀️.rs:42`).

### 4.2 Window `🧪️tests` → the owning mode's `🧪️tests/<window>/` (14 windows)

`🧪️tests` is not in `windowChildDirs`, and there is no window-local home for it. It moved one level
up, to the mode that owns the window — `🎭️modes/<mode>/🧪️tests/<window>/🔬️unit/🦀️.rs` — which is a
shape the repo already uses (`🖍️draw`, `📕️norm` carry `🎭️modes/✏️edit/🧪️tests`) and which mode dirs
admit, since `modeRequiredChildDirs` names required children and imposes no closed set. Each window's
mount was rewritten in the same step, `#[path = "🧪️tests/🔬️unit/🦀️.rs"]` →
`#[path = "../../🧪️tests/<window>/🔬️unit/🦀️.rs"]`, and the moved files carry no relative paths of
their own (checked). Only direct window children moved: `👁️preview/🫧️transient/🧪️tests` is a facet's
own test dir and was left alone.

### 4.3 Six `📜️.wit` schema facets

`TAXONOMY.schemaFormats` declares six formats; the three surfaces that own real state carried five
and were missing `📜️.wit`. Authored, each stating the same record the sibling `🔣️.json`/`🦀️.rs`
already state:

- `🧊️generation3d/…/👁️viewer/🎚️config/🧬️schema/📜️.wit`, `…/👁️viewer/👥️presence/🧬️schema/📜️.wit`
- `🧊️generation3d/…/✏️editor/🎚️config/🧬️schema/📜️.wit`, `…/✏️editor/👥️presence/🧬️schema/📜️.wit`
- `🌀️generation2d/…/✏️editor/🎚️config/🧬️schema/📜️.wit`, `…/✏️editor/👥️presence/🧬️schema/📜️.wit`

Each is a self-resolving `package semio:…@1.0.0` with one interface and one world, modelled on the
repo's only prior `📜️.wit`
(`✏️s/🔌️plugins/🎬️sequence/…/✏️editor/🌉️wasm/🧬️schema/📜️.wit`). The presence facets restate the camera
record rather than `use` the config package, so each file resolves alone exactly as its `🔣️.json`
sibling does through `$ref`.

### 4.4 One unmounted leaf repaired, not deleted

`🧊️generation3d/…/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🧬️schema/🦀️.rs` was the
only leaf that looked like a stale orphan: its facet's sibling lanes carry four format files and no
Rust leaf, and nothing referenced it. It is not stale — it is the schema-first declaration of a lane
whose runtime struct sits beside it, the same pairing `🎚️config` uses. Mounting it as
`#[path = "🧬️schema/🦀️.rs"] pub mod schema;` from the transient component exposed why it had drifted:
it declared `#[state(ephemeral_local_window)]`, and the derive rejects it —
`unknown state class 'ephemeral_local_window' — the only four lanes are artifact, config, presence,
transient`. Corrected to `#[state(transient)]`, which is this lane. The finer scope stays where it is
already pinned by a contract test: `🧬️schema/🔣️.json`'s `"x-semio-state": "ephemeral-local-window"`,
asserted by `🫧️transient/🧪️tests/🔬️contract/🟦️.ts`. Deleting it would have removed a schema
declaration the four sibling format leaves describe.

### 4.5 Plugin root `🎮️commands/🦀️.rs`

`TAXONOMY.pluginRequiredChildDirs = ["🎮️commands"]`, and `validatePluginContractRoot`
(`…📇️registry/📜️script.ts:1200-1203`) wants a Rust leaf in it. Procedural's held only
`📌️.empty.md` — as do all 33 plugins; **no `🎮️commands/🦀️.rs` exists anywhere in the repo**, so there
was no precedent to copy and no generator to run.

Authored as a real facet, not a stub. `PluginManifest::commands` is documented as *"Plugin-scope
commands this program exposes — apply whenever any of its apps is focused"*
(`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4011`), and `Plugin::plugin_command` is the only way to fill
it. Procedural has exactly one fact at that scope: the nine flow extensions both the 2D and the 3D
editor evaluate through, owned by neither. So:

- `✏️s/🔌️plugins/🌀️procedural/🎮️commands/🦀️.rs` declares the `listFlowExtensions` command
  (`ActionKind::View`, palette-visible, EN/DE label) and its program-level handler, projecting the
  roster as `[{ id, extension, label, version }]`.
- `✏️s/🔌️plugins/🌀️procedural/🦀️.rs` gained `FLOW_EXTENSIONS`, a nine-row
  `(slug, extension id, label, version)` table, and builds its `FlowExtensionDeclaration`s from it in
  a loop. That **removed 45 lines of repetition**: the nine declarations previously spelled
  `s.procedural.flow-extension.<slug>` and `semio.s.plugin.flow.extension.<slug>` out by hand, twice
  each. The table is now the single source both the builder and the command read.
- `🎮️commands/🧪️tests/🔬️unit/🦀️.rs` asserts three laws: the command reaches the built
  `manifest.commands`, the handler answers exactly the installed roster in declaration order and
  emits no mutations, and `plugin().handle_plugin_command(…)` dispatches it.

## 5. What is left, and exactly why (22)

### 5.1 assembly's editor/viewer are unmounted — the blocking bounds, measured

`📓️taxonomy-violations-audit-2026-09-10.md` and the plugin root's own note attribute this to
"`ArtifactEditor`/`ArtifactViewer`'s own trait bounds … unsatisfied". This lane **measured** it rather
than repeating it: the mount was written into `🗿️artifacts/🧩️assembly/🦀️.rs` behind
`component-app-assembly`, mirroring generation3d's, and
`cargo check -p semio-s-artifact-procedural-assembly --features component-app-assembly --keep-going`
was run. It fails on exactly five bounds, every one owed by assembly's own **schema** tree and none
by the surfaces:

```
error[E0277]: the trait bound `AssemblySnapshot: ArtifactPack` is not satisfied
error[E0277]: the trait bound `AssemblySnapshot: ArtifactDsl` is not satisfied
error[E0277]: the trait bound `…mutations::component::AssemblyMutation: OpBinary` is not satisfied
error[E0277]: the trait bound `…mutations::component::AssemblyMutation: OpText` is not satisfied
error[E0277]: the trait bound `AssemblyEditorCommand: OpBinary` is not satisfied
```

generation3d carries all five as hand-written impls in two representation leaves —
`🧬️schema/📸️snapshot/📝️text/🦀️.rs` (`ArtifactDsl`, `ArtifactPack`) and
`🧬️schema/🧬️mutations/💾️binary/🦀️.rs` (`OpText`, `OpBinary`) — which assembly's schema tree does not
have. Author those two leaves and the mount is a pure addition; nothing in the surfaces changes. The
mount was reverted (it does not compile) and the finding recorded verbatim at the mount site, replacing
the older second-hand note. **10 of the 22.**

Assembly's `is missing 🚪️io/` and `is missing 📚️examples/` (2 more) are real content gaps of the same
packet: an artifact with no codec impls has nothing to put in `🚪️io/`, and an example set needs the
snapshot text codec to write `🗣️.dsl.semio` with. **12 of the 22 belong to one assembly packet.**

### 5.2 `🌀️generation2d`'s viewer owns no config or presence state (4 of the 22)

`assertAppSchemaOwner` wants `🎚️config/🧬️schema/` and `👥️presence/🧬️schema/` with all six format
leaves on every surface. `🌀️generation2d`'s viewer and both assembly surfaces carry `📌️.empty.md` in
those lanes, and `Generation2dViewer` declares `type Config = NoConfig; type Presence = NoPresence;`
(`…/🌀️generation2d/…/✳️any/👁️viewer/🦀️.rs`). Authoring six schema files over state that does not
exist would be a fiction. Doing it truthfully is the packet `📓️viewer-2026-09-09.md` §2.1-§2.3 ran for
the 3D viewer: a real `Generation2dViewConfig`/`Generation2dViewPresence`, hand-written `ArtifactDsl`/
`ArtifactPack`, an authored `dsl::Mutations` aggregate over real leaf directories, and the viewer's
associated types rewired off `NoConfig`. Not started here. 2 findings are generation2d's viewer; the
other 4 are assembly's two surfaces, which cannot own config before §5.1 lands.

### 5.3 The four fixture-case adapters (4 of the 22)

`…/🪆️subsets/✳️any/🧪️tests/{🧊️mutate-procedural-3d-1,🚪️io-procedural-3d-1,🌀️mutate-procedural-2d-1}/🦀️.rs`
and assembly's `🧩️mutate-assembly-1/🦀️.rs` are compiled by the **generated fixture-test host**
(`semio-repo-test-host`), not by any crate in this tree. Each is a library-shaped module exporting
`pub fn adapter() -> Adapter`, and each says in its own docstring why it must not be a target of the
crate it tests:

> "mounted by path rather than linked: a generated test host may not gain a Cargo dependency on
> another plugin's crate"

`🔮️oracle/🔣️.json` records the same ceiling from the other side — "the SUBJECT half does not run this
subset's codec … it links no plugin crate and replays the committed vectors". Confirmed structural,
not procedural-specific: **no plugin in the repo declares a `🪆️subsets/*/🧪️tests/<case>/🦀️.rs` as a
Cargo target** (`grep '🪆️subsets/✳️any/🧪️tests/' -r ✏️s/🔌️plugins --include=Cargo.toml` → 0 hits).
Making them `[[test]]` entries would link the crate the oracle's independence argument rests on not
linking, and would need a `semio-repo-test-host` dev-dependency in every artifact package.

The honest fix is at the detector: `validateRustTaxonomyMounts` proves ownership from Cargo manifests,
and these leaves are owned by a manifest the host generates outside the tree. Teaching it that
authority is a real change with a real design question (what proves a case dir is a registered
fixture case — the sibling `🥒️.feature`, the contract, or the runner's own registry) and it lands
repo-wide, so it is named here rather than guessed at.

## 6. The `♻️mit-bestand` gitlink remedy — the audit's "data edit only" is wrong

`📓️taxonomy-violations-audit-2026-09-10.md` §5 proposed a third `🔣️taxonomy.json` `pathExclusions`
entry and said it would let `verify taxonomy` past its abort "with **no code change**, only a
`🔣️taxonomy.json` data edit". Applied exactly as proposed, it fails immediately — three separate
places pin the exclusion list to exactly two entries, and a fourth rejects the proposed path:

1. `🔍️discovery/🟦️.ts` `validateTaxonomy`: *"pathExclusions must contain exactly ordered "compose"
   and "temp-compose" contracts."*
2. the same function: *"areaEnforcement.opaquePathExclusionIds must be exactly ["compose",
   "temp-compose"]."*
3. `🧹️normalization/🟦️.ts` `parseTaxonomy`: *"Taxonomy v7 pathExclusions must contain exactly opaque
   compose and temp/compose"* (plus its `areaEnforcement` twin).
4. **The proposed path is the wrong subtree.** `"path": "♻️mit-bestand/"` makes an opaque subtree out
   of a directory the repo generates into: `generatorContracts["external-step-assets"]` writes five
   `.stp` outputs under `♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/`, and
   `generatorContracts["report-actor-network"]` is *owned* at
   `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript` and writes four `.tex` outputs under
   `♻️mit-bestand/📋️bericht/📎️anhang/`. With the audit's entry in place, `validateTaxonomy` reports
   twelve `crosses an opaque boundary` errors and every taxonomy consumer throws — including the
   registry check. Only `♻️mit-bestand/🔎️recherche` is a gitlink (`git ls-files -s` → `160000`).

Applied instead, quoting the existing entry shape the audit asked for:

```json
"compose":       { "path": "compose/",      "mode": "opaque", "reason": "Explicit user-owned opaque subtree; filter before every filesystem access" },
"temp-compose":  { "path": "temp/compose/", "mode": "opaque", "reason": "Explicit recovered opaque subtree; filter lexically before every filesystem access" },
"mit-bestand-recherche": { "path": "♻️mit-bestand/🔎️recherche/", "mode": "opaque", "reason": "Explicit nested-repository gitlink; filter lexically before every filesystem access" }
```

plus `areaEnforcement.opaquePathExclusionIds` extended in order, and the three hard-coded pins
replaced by one named table per file (`OPAQUE_PATH_EXCLUSIONS`,
`TAXONOMY_OPAQUE_PATH_EXCLUSIONS`) so the list is stated once each instead of four times.

One more precision fix was needed: `patternTouchesOpaque` compared only a pattern's **first path
segment**, so with a two-segment opaque root every literal path starting `♻️mit-bestand/` was flagged
— including the report generator's own two exact input files, which reach nothing opaque. It now
flags a pattern that is lexically under (or over) an opaque root, or that carries a wildcard at or
above that root's depth. A top-level glob is still rejected exactly as before.

## 7. Files changed

Concurrency guard: every directory and file was `stat -f %Sm`-checked before it was moved or edited
and skipped if it had been written in the last 15 minutes. **Nothing was skipped** — the newest
mtime among the 28 renamed dirs, the 14 relocated test dirs and the framework files was
`Sep 10 07:35` against a `Sep 10 09:45` start. No file in this lane was touched by a peer while it
was open, and no `git` state-modifying command was run.

### 7.1 Framework — the detector (4 files)

| file | change |
|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` | `RustCargoManifestFacts.targetPaths` + `[[bin]]`/`[[test]]`/`[[bench]]`/`[[example]]` readers (strict TOML and lexical); `inspectRustModuleGraph` anchors each declared target path as a crate root; `OPAQUE_PATH_EXCLUSIONS` table replaces two hard-coded pins; `patternTouchesOpaque` compares depth, not first segment |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` | `validateRustTaxonomyMounts` takes a manifest set; `validateTaxonomyTree`'s walk collects `Cargo.toml`; artifact facet/examples walk descends `🏅️standards/<s>/🪆️subsets/<sub>/`; `⚙️engine` required → forbidden; `TAXONOMY_SUBSET_COMPONENTS`; `🧪️tests`/`🧫️fixtures` skipped beside example slugs |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` | `TAXONOMY_OPAQUE_PATH_EXCLUSIONS` table replaces two hard-coded `parseTaxonomy` pins |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | third `pathExclusions` entry `mit-bestand-recherche` + `areaEnforcement.opaquePathExclusionIds` |

### 7.2 Procedural — authored (4 new files)

- `✏️s/🔌️plugins/🌀️procedural/🎮️commands/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎮️commands/🧪️tests/🔬️unit/🦀️.rs`
- six `📜️.wit` schema facets (§4.3), under
  `🧊️generation3d/…/✳️any/{👁️viewer,✏️editor}/{🎚️config,👥️presence}/🧬️schema/` and
  `🌀️generation2d/…/✳️any/✏️editor/{🎚️config,👥️presence}/🧬️schema/`

### 7.3 Procedural — edited (6 files)

- `✏️s/🔌️plugins/🌀️procedural/🦀️.rs` — `FLOW_EXTENSIONS` table, `flow_extension_declaration_id`,
  `flow_extension_declarations()`, `mod commands`, `.plugin_command(…)`, nine literal blocks removed
- `🗿️artifacts/🧩️assembly/🦀️.rs` — the measured trait-bound note replacing the second-hand one
- `🗿️artifacts/🧊️generation3d/…/✳️any/✏️editor/🦀️.rs` — stale `🎚️options`/`📌️panels` docstring
- `🗿️artifacts/🧊️generation3d/…/✏️edit/🪟️windows/👁️preview/🫧️transient/🦀️.rs` — mounts `🧬️schema`
- `🗿️artifacts/🧊️generation3d/…/👁️preview/🫧️transient/🧬️schema/🦀️.rs` — `#[state(transient)]`
- `🗿️artifacts/🧊️generation3d/…/👁️viewer/…/🪟️windows/👁️preview/🦀️.rs` — relocated-test docstring

### 7.4 Procedural — moved (42 directories, 14 `🦀️.rs` mounts rewritten)

- 14 × `🪟️windows/<w>/⚙️config` → `🪟️windows/<w>/🎚️config`
- 14 × `🪟️windows/<w>/🎚️options` → `🪟️windows/<w>/☑️options`
- 14 × `🪟️windows/<w>/🧪️tests` → `🎭️modes/<mode>/🧪️tests/<w>`, each window's
  `#[path = "🧪️tests/🔬️unit/🦀️.rs"]` rewritten to `#[path = "../../🧪️tests/<w>/🔬️unit/🦀️.rs"]`

Across `🧊️generation3d` (6 windows), `🌀️generation2d` (6) and `🧩️assembly` (2). All moves used plain
`mv`; no `git mv`.

### 7.5 Ticket

- `📓️taxonomy-fix-2026-09-10.md` (this report)
- `🐍️taxonomy-mount-probe.ts` — the read-only two-manifest-set probe §2.1 quotes; kept as an input file
- `🗑️generated/tax-*.txt` — raw run logs

## 8. Gate outputs

Every command below was run from the repo root against a private
`CARGO_TARGET_DIR` (`…/scratchpad/target-tax`, seeded by `cp -Rc` from the shared `target/debug`)
with `RUSTC_WRAPPER=""`. The shared `target/` was never written. Raw logs in `🗑️generated/tax-*.txt`.

### 8.1 Procedural-scoped taxonomy check — 650 → 22

```
bun 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts check
```
`| grep '🌀️procedural:'` → **22** (from 650), exit 1 repo-wide. All 22 listed in §5, all in
`🧩️assembly` or the four fixture-host adapters. Full slice: `🗑️generated/tax-procedural-after.txt`.

### 8.2 `bun nx run @semio-tech/plugin-registry:check` — still red, repo-wide

The gate has one phase that fails: the taxonomy-tree phase, which `process.exit(1)`s because
`🔣️taxonomy.json` declares `areas["✏️s/🔌️plugins"] = "clean"`. Everything before it passes — catalog
staleness, generated-projection freshness, plugin-root ownership and the launch-seed comparison all
run to completion, and the run reaches the taxonomy phase, which is the last one. Repo-wide finding
lines: **19,464 → 2,123**. Of the 32 other plugins, none is clean, so the gate would stay red even at
procedural 0 — as `📓️taxonomy-violations-audit-2026-09-10.md` §4 predicted.

### 8.3 `rust-taxonomy-mounts-check` — 9/9 against real `rustc`

The detector changes in §2 are covered by an oracle that compiles each fixture with `rustc` and
compares membership against the checker's own answer. All nine cases agree after the change,
including the two that exist specifically to catch over-eager mounting:

```
registry-rust-mounts case=unrelated-manifest-no-authority rustc=0 reference=["unmounted"] registry=["unmounted"]
registry-rust-mounts case=dependency-suffix-collision    rustc=0 reference=["unmounted"] registry=["unmounted"]
registry-rust-mounts-oracle cases=9 ajv=1 compiler=9
```

### 8.4 `cargo check -p semio-s-plugin-procedural --keep-going` — clean

Native: `exit=0`, 0 errors (`🗑️generated/tax-cargo-native.txt`).
Wasm: `--target wasm32-wasip2 --profile wasm-dev` with `CARGO_PROFILE_WASM_DEV_DEBUG=false` —
`Finished 'wasm-dev' profile [unoptimized] target(s) in 4m 24s`, 0 errors
(`🗑️generated/tax-cargo-wasm.txt`).

### 8.5 `generation2d` lib suite — no regression, byte-identical to baseline

```
RUST_MIN_STACK=536870912 cargo test -p semio-s-artifact-procedural-generation2d \
  --features component-app-assembly --lib -- --test-threads=2
test result: FAILED. 228 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 28.12s
    editor::generation2d::component::tests::two_instances_converge_disjoint_widget_moves
    editor::generation2d::component::tests::vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed
```

`📓️hotpath-optimization-2026-09-10.md` records the baseline as *"228 passed / 2 failed — byte-identical
to the count `📓️status.md` 02:59 records"*. Same counts, same two test names. **No regression.**

### 8.6 `generation3d` lib suite — 328/4, three of the four known, one load-attributable

```
test result: FAILED. 328 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 206.36s
```

The baseline in `📓️hotpath-optimization-2026-09-10.md` is **315 passed / 5 failed** with a named list.
Comparison:

| test | baseline | this run |
|---|---|---|
| `generation_preview_is_one_app_transient_shared_by_two_generation_windows` | fail | fail (known) |
| `two_instances_converge_disjoint_widget_moves` | fail | fail (known) |
| `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | fail | fail (known) |
| `preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app` | fail | **pass** |
| `refresh_pending_effects_arms_flow_eval_tick_chain` | fail | **pass** |
| `hex_column_boot_stays_inside_the_interactive_turn_budget` | pass | **fail** |

Thirteen more tests pass than at baseline and two previously-failing ones now pass. The one that
turned red is the wall-clock budget assertion, and it is not this lane's:

- **It is a timing assertion with a documented ~10% margin.** Its own comment records the idle
  measurement it was calibrated against: *"Measured on an unoptimized build: best 7 226 us, worst
  7 828 us idle"* against an 8 000 µs ceiling.
- **The machine was never near idle.** `load averages: 45.90 58.03 64.99` during the suite (peer
  sessions compiling; 7 concurrent `rustc` processes), rising to `62.62 77.10 75.19` later. Re-run
  alone it improved but still failed: `best_eval_step_us=8965` (from 11 588 in the full suite).
- **Neither generation3d edit can reach the evaluation path.** They are (a) relocating window test
  files and rewriting their `#[path]` mounts, and (b) mounting
  `👁️preview/🫧️transient/🧬️schema/🦀️.rs`. That leaf's `ArtifactSchema` derive emits exactly two
  inherent trait impls and nothing else — no `inventory::submit`, no `linkme`, no constructor, no
  static (`🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️.rs:256-281`). Mounting it adds a type nothing
  calls; it cannot cost a microsecond at runtime.

Recorded as **unresolved-under-load, not passing** rather than claimed green: a clean attribution
needs one idle re-run, which this machine did not offer while the lane was open.

### 8.7 `semio-s-plugin-procedural` lib suite — the target compiles again

The plugin's own lib test target did **not compile** when this lane started, with 12 errors, all four
`E0271`/`E0308` families on one line — `assert_viewer_never_mutates::<Generation3dViewer>()` in
`🧪️tests/🔬️surface/🦀️.rs:18`. None of the 12 was in a file this lane authored; the helper is bounded
`Presence = NoPresence, Transient = NoTransient` and the 3D viewer gained real presence and transient
state in `📓️viewer-2026-09-09.md` §2.2-§2.3. Since the same law is already asserted over the real
viewer with its real state (`👁️viewer/🧪️tests/🔬️unit/🦀️.rs`,
`every_viewer_action_dispatches_live_and_never_mutates_the_document`, which dispatches all seven view
commands rather than the helper's one default), the superseded line was removed and the reason
recorded in place. With that, the target builds and this lane's own new tests run:

```
test commands::tests::list_flow_extensions_is_declared_on_the_built_manifest ... ok
test commands::tests::list_flow_extensions_answers_the_installed_roster ... ok
test commands::tests::the_plugin_dispatches_its_own_scope_command ... ok
test result: FAILED. 7 passed; 2 failed
```

The two failures: `surface_tests::generation2d_viewer_never_mutates` (`ordered-map root must be
explicitly retired before drop`, the systemic retirement-contract family
`📓️hotpath-optimization-2026-09-10.md` §5 describes, in `🌱️value/🗂️ordered/🦀️.rs:81`), and
`descriptor_is_fresh`, which **is** this lane's: adding a plugin-scope command changes the assembled
manifest, so `🛂️.descriptor.semio` had to be regenerated with
`bun nx run …:describe` (`📦️packages/🦀️rust/📜️script.ts describe`).

## 9. Notes for whoever picks this up

1. **The nested schema/io/mutation matrix is the next packet** (§3.1). It is stale against the same
   W3 relocation this lane fixed one level above it, and it is the reason the repo-wide count is
   2,123 rather than lower. Rewriting it needs four decisions, each already answered by
   `🔣️taxonomy.json`: io codec leaves live at `<format>/🔖️<standard>/✳️<subset>/`, mutation `🧬️schema/`
   payloads are json-kind (`artifactSchemaSpecFileKinds`), `🧪️tests`/`🧫️fixtures` are legal anywhere
   (`testsDirName`/`testFixturesDirName`), and a mutation directory is one matching
   `mutationDirectoryPattern` — which is what separates a real mutation from the `📝️text`/`💾️binary`
   organizational facets the walk currently mistakes for mutations.
2. **Assembly's two representation leaves unblock twelve findings at once** (§5.1): `ArtifactDsl` +
   `ArtifactPack` for `AssemblySnapshot`, `OpText` + `OpBinary` for `AssemblyMutation`, `OpBinary` for
   the two commands. Then the editor/viewer mount, `🚪️io/`, `📚️examples/` and both surfaces' config
   and presence lanes all become reachable work.
3. **The fixture-host adapter class is repo-wide and needs a policy decision, not a fix per plugin**
   (§5.3).
4. **`assert_viewer_never_mutates` is unusable by any viewer that owns presence or transient state**
   (§8.7). Its `BoundedViewerFixture` needs a bounded presence disposer and peer-retirement factory
   the way `🫧️transient/🧵️publication/🦀️.rs` already provides `bounded_transient_store_disposer<P, M>`;
   there is no `bounded_presence_store_disposer`, only `no_presence_store_disposer()` for the
   zero-payload type. Until then every plugin whose viewer gains state silently loses this law, and
   its lib test target stops compiling — procedural's had been broken since 2026-09-09.

### 8.8 Final re-measurement, after a peer's edit landed mid-lane

A peer added a third `[[test]]` to `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`
(`idle_turns` → `../../🧪️tests/😴️idle-turns/🦀️.rs`) while this lane was open. It is live proof of
§2.2: the new leaf is reported **reachable**, where before this lane it would have been a 23rd
finding. Descriptor and catalog were refreshed for it and for the new plugin-scope command
(`📇️registry/📜️script.ts generate`, `📦️packages/🦀️rust/📜️script.ts describe`), and the gate re-run:

```
bun 🧰️framework/…/📇️registry/📜️script.ts check   exit=1
2123 lines repo-wide, 22 for 🌀️procedural
```

`descriptor_is_fresh` passes after the regeneration:

```
test descriptor_is_fresh ... ok
test result: FAILED. 8 passed; 1 failed
```

The one remaining plugin-suite failure is `surface_tests::generation2d_viewer_never_mutates`, which
dies inside the framework's own fixture harness with `ordered-map root must be explicitly retired
before drop` (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`) — the systemic retirement-contract
family, in code this lane never touched.

### 8.9 `bun ./📜️script.ts verify taxonomy report` — abort cleared, run not finished

Before §6 it aborted in seconds at
`inventoryTaxonomyWithSourceParentPruning` (`🧹️normalization/🟦️.ts:6528`) with
`Normalization requires an explicit repository-boundary decision before authored classification:
♻️mit-bestand/🔎️recherche`, and, with the audit's proposed entry, at
`Taxonomy v7 pathExclusions must contain exactly opaque compose and temp/compose`. Both are gone: the
command now proceeds into the repo-wide inventory. It then ran for **110 minutes wall / 42 minutes CPU and emitted
not one line** before the harness killed it (exit 144, zero-byte log), with machine load averages of
45-90 and up to 13 peer `rustc` processes throughout. It was NOT re-run: the brief says to report the
residual "without chasing", and a second attempt under the same contention would only add to it. So
**no residual finding list is reported here** — the claim made is only that the boundary abort is cleared, which is what the task
asked the exclusion remedy to achieve, and it is proved by every other taxonomy consumer
(the registry check, `parseTaxonomy`, `validateTaxonomy`) loading the three-entry table without error.

### 7.6 Regenerated, not hand-edited

- `✏️s/🔌️plugins/🌀️procedural/🛂️.descriptor.semio` and `…/🔣️.json` — `📦️packages/🦀️rust/📜️script.ts
  describe` (the plugin-scope command changes the assembled manifest; `descriptor_is_fresh` passes
  after). Note the describe tool refuses an artifact root outside the repo, so it cannot run against
  a scratchpad `CARGO_TARGET_DIR`: it was given a private target dir under the repo's own gitignored
  `target/`, seeded by `cp -Rc` and deleted afterwards. The shared `target/debug` was never written.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/` and `.vscode/launch.json`
  — `📇️registry/📜️script.ts generate`, required because `check` fails fast on catalog staleness once
  a descriptor moves. `launch.json` is machine-generated from `🧩️launch.seed.jsonc`; the seed was not
  touched.
