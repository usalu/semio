# Explorer census — plugin-identity conformance for all 59 registry rows

Ticket `26/09/05/S-END-TO-END` · Sonnet 5, read-only. Context: `📋️plan.md` 21:36–05:45 (lane K, the `space`
plugin-identity rename) and `📓️opus-space-plugin-identity.md` §Headline. Goal: find every OTHER row that
carries the same disease that cost lane K four rebuilds — builder id / `.package_id()` / artifact-kind
owner / extension identity drifting from the Cargo component package the registry, deployment catalog and
`describeBuiltPlugin` all derive from — **before** the running `plugin-build-catalog-*` rebuild reaches
them.

Method: `python3` walking `🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json` (59 rows) + `grep -rn` over
`✏️s/🔌️plugins/**/🦀️.rs`, cross-referenced against the framework gate implementations in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` and
`…/🔌️plugin/🏗️builder/🦀️.rs`. No cargo/build/search-tool use. Every claim below cites the exact
file:line read; two early false leads (a decoy `const EXTENSION_ID`/`.package_id()` match that wasn't the
one actually passed to the builder) are called out so they aren't repeated by whoever fixes this.

## Headline — 2 real conformance bugs found, same class as `space`'s

1. **`reasoning-mindmap`** — FAIL(a)/(e). Root `🦀️.rs` still says `builder("reasoning")` /
   `.package_id("semio:reasoning")`, self-consistent with each other but not with the Cargo component
   (`semio:reasoning-mindmap`), the deployment catalog row, or the registry. This is **exactly** the bug
   space had before lane K: internally consistent, externally wrong. It will describe/build fine in
   isolation (`try_build()`'s own package-suffix check passes because both literals already agree with
   each other) and then fail `describeBuiltPlugin`'s cross-check against the registry the moment Wave 2
   rebuilds it, the same "Plugin descriptor identity mismatch" class of stub lane K hit four times.
2. **`imperative-extension-effect`** — the extension analogue of the same bug. Its `ExtensionBundle::new(EXTENSION_ID, "Imperative Core", MODULE_VERSION)` call passes `EXTENSION_ID = "imperative-extension-core"` (`✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/🦀️.rs:116`), a stale identity/label left over from what looks like a copy of a sibling module, while the Cargo component package is `semio:imperative-extension-effect` (`…/📣️effect/📦️packages/🦀️rust/Cargo.toml:11`) and the registry row is `imperative-extension-effect`. The file's own domain code (`LogPrint`/`StateSet`/`StateIncrement`) is genuinely the "effect" module — only the identity constant and the bundle label were never renamed. This extension currently has **no owner descriptor pair yet** (see inventory below), so the mismatch hasn't surfaced as a build failure yet; it will, the moment Wave 2 reaches it.

No other row (of the other 57) has a builder/package-id/extension-id mismatch. See the full table below.

## What is NOT a live gate failure (found, but structurally inert or not yet exercised)

- **`remodel`**'s artifact schema descriptor carries the id `s.remodeling.remodeling` (owner segment
  "remodeling", not "remodel") at 4 sites under
  `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/…` — but the `ArtifactDeclaration`'s
  gated `kind` field comes from `ArtifactDefinition::new(ArtifactIdentity::parse("s.remodel.remodeling")?)`
  (`…/📸️remodeling/🦀️.rs:64`), which IS `remodel`-owned. `preflight_artifact_identity` (`🔌️plugin/🦀️.rs:2118`,
  called from `ArtifactDeclaration::preflight` at `:3514`) only ever inspects `self.kind` — never the raw
  schema-descriptor id string — so this drifted literal does not trip gate (b) today. It is still worth a
  cosmetic fix (naming drift risk for the next person who greps `s.remodel.` and misses `s.remodeling.`).
- **`procedural`**'s two document artifacts are correctly owned (`s.procedural.generation3d`,
  `s.procedural.generation2d`), but their editor's config/presence sub-schemas use `s.generation.3d.config`
  / `s.generation.3d.presence` / `s.generation.2d.config` / `s.generation.2d.presence`
  (`…/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/{🎚️config,👥️presence}/🧬️schema/🦀️.rs`, and the
  `2d` siblings) instead of `s.procedural.generation3d.config` etc. `check_surface_id`
  (`🔌️plugin/🦀️.rs:28062`) only compares `subset.dialect.artifact_kind` (the top-level kind), never these
  config/presence ids, so this also isn't gated. Same class of cosmetic drift as remodel's.
- **25 of 26 extensions never call `.depends_on(<owner>, version)`**, only `.extends(<owner>)` — confirmed
  by grepping every extension's root `🦀️.rs` for `.depends_on(`. The one exception,
  `cad-extension-aec-building`, calls both (`…/🏢️aec-building/🦀️.rs`) and is the only extension that also
  calls `.contributes(...)` (a real composite mutation/inference onto `s.cad.cad`). The framework's
  `register_contributions` (`🔌️plugin/🦀️.rs:4434`) — which returns `DependencyNotDeclared` when a
  contribution's target-kind owner isn't in `manifest.dependencies` — is only invoked for `.contributes(...)`,
  never for the topic-only `.contributes_topic(...)` all 25 other extensions use. So today none of them
  breach anything. **Latent risk**: if any of them ever grows a real `.contributes()` mutation/inference
  onto their host's artifact (the natural next step for, e.g., `flow-extension-brep` or any
  `process-extension-*`), it will hard-fail assembly with `DependencyNotDeclared` unless a matching
  `.depends_on()` is added first. The registry's `dependsOn: ["flow"]` etc. for these rows is **not** read
  from the compiled manifest — it's synthesized at generation time from the Cargo
  `[package.metadata.semio] extends = "…"` line
  (`🔌️plugin/📇️registry/📜️script.ts:279-286`, comment: "contract freeze §4 rule 1: for an extension,
  extends is always dependsOn[0]"), so the registry looking "correct" is not proof the built component's
  own `manifest.dependencies` is non-empty.
- **All 26 extensions never call `.package_id(...)`** (`ExtensionBundle::package_id`,
  `🔌️plugin/🦀️.rs:33023`, confirmed unused by grep across every extension root file). `manifest.package_id`
  therefore stays the empty-string default for every one of them — this is why 10 already-built pairs show
  `"packageId": ""` and why the other 16 will too the moment they get a first descriptor. I could not find
  an analogous `.ok_or_else(...)` fatal check for extensions the way `try_build()` has one for plugins
  (`🏗️builder/🦀️.rs:639-641`), so I cannot confirm from source alone whether this is fatal to
  `describeBuiltPlugin`'s cross-check or merely cosmetically empty — flagging as an open question for
  whoever owns lane B's "shared extension describe route," since it affects literally every extension row.
- **`gate (c)` (`interactive-job.catalog-controller`) cannot recur outside `space`.** Every plugin's
  document apps are built through `Editor::builder(DIALECT)` / `Viewer::builder(DIALECT)`
  (`🔌️plugin/🦀️.rs:27684-27689`), which hard-codes
  `AppBuilder::new(surface_app_id(&dialect, AppRole::Editor), …)` — the app `id` **and** default
  `controller_id` (`AppBuilder::new`, `🔌️plugin/🦀️.rs:4867-4871`, `controller_id: id.clone()`) are both
  derived from the canonical surface id with no public setter to override `controller_id` independently.
  `space`'s bug was possible only because `home`/`studio`/`space-index` are the one plugin that builds its
  apps through a bespoke hand-rolled path (`⚙️engine/🪐️space/🦀️.rs`) instead of `Editor::builder`. I found
  no other plugin using that bespoke path (confirmed `Editor::builder(...)` present for `demonstrator` too),
  so gate (c) is structurally unreachable for the other 32 plugins today. **One adjacent observation, not
  a gate failure**: ~29 plugins declare a short legacy `const <PLUGIN>_<SURFACE>_CONTROLLER_ID: &str =
  "<plugin>-play"` (e.g. `TRINITY_JACK_PLAY_CONTROLLER_ID = "trinity-jack-play"`,
  `…/🔱️trinity/🗿️artifacts/🔌️jack/…/✏️editor/🦀️.rs:32`; full list in the appendix below) used only to stamp
  `ActionDescriptor.controller_id` on window-chrome payloads — a **different** string from the real,
  correctly-derived `definition.controller_id` (confirmed via `registry.controller_id()` at the same
  file's `:520`, and via the `bounded_first_step_tool_proofs!` macro's own `controller: "s.trinity.jack@1/*#editor"`
  field at `:71`). I traced ShellHost's `action.controllerId !== session.app.controllerId` comparison
  (`🏛️ShellHost/🟦️.tsx:4397`) and it only redirects to a **spawned** panel app when they differ, falling
  back to the current session either way when no spawned app matches — i.e. it looks inert for an ordinary
  single-instance editor, which is presumably why it hasn't been reported as a live bug. This is exactly
  the shape of thing that broke `space`'s Home/Studio (which uniquely run in ShellHost's `hostMode`
  spawned-app path), so it's worth a runtime check rather than trusting this static read — I could not
  prove or disprove a live effect from source alone.

## Full 59-row table

Legend: **(a)** builder/package_id self-consistency + match to Cargo/registry pluginId · **(b)** every
declared artifact kind's owner segment == plugin id (gate only fires for `.artifact()`/`declare_artifact()`
declarations, i.e. plugin rows) · **(c)** `definition.controller_id` == surface app id (structurally
guaranteed by `Editor`/`Viewer::builder`, see above — marked n/a where no bespoke path exists) ·
**(d)** `surface_dependency_breaches`/`register_contributions` dependency-declaration gate · **(e)**
`describeBuiltPlugin` manifest identity vs registry pluginId · **Pair** = current `🔣️.json` owner
descriptor state on disk.

| # | pluginId | role | Cargo pkg matches registry | (a) | (b) | (c) | (d) | (e) | Pair | Verdict |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | animate | plugin | yes | PASS | PASS | n/a(struct) | n/a | PASS | present | PASS |
| 2 | architect | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 3 | block | plugin | yes | PASS (`builder("block")`/`semio:block`, `🧱️block/🦀️.rs:205,208`) | PASS | n/a | n/a | PASS | MISSING | PASS (needs pair) |
| 4 | cad | plugin | yes | PASS | PASS | n/a | n/a | PASS | present (only valid pair as of 06:00 log) | PASS |
| 5 | cad-extension-aec-building | extension | yes | PASS (`EXTENSION_ID`, `🏢️aec-building/🦀️.rs`) | n/a | n/a | PASS (`.extends("cad").depends_on("cad",…)` + `.contributes(...)` all present) | PASS | PLACEHOLDER (`pluginId:"empty"`, `role:"plugin"`) | PASS (stale pair) |
| 6 | cad-extension-aec-building-energy | extension | yes | PASS | n/a | n/a | PASS (trivial — `.contributes_topic` only) | PASS | STALE (`packageId:""`) | PASS (stale pair) |
| 7 | cad-extension-aec-building-structure | extension | yes | PASS | n/a | n/a | PASS (trivial) | PASS | STALE (`packageId:""`) | PASS (stale pair) |
| 8 | cad-extension-spatial-shape | extension | yes | PASS | n/a | n/a | PASS (trivial) | PASS | STALE (`packageId:""`) | PASS (stale pair) |
| 9 | dag | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 10 | demonstrator | plugin | yes | PASS (`builder(PLUGIN_ID)`, `PLUGIN_ID="demonstrator"`, `🪪️manifest/🎪️demonstrator/🦀️.rs:23,48,51`) | PASS | n/a (uses `Editor::builder`) | PASS (lane H: 7 `.depends_on` kept, `plugin-assembly.surface-dependency-gate`) | PASS | present | PASS |
| 11 | draw | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 12 | energy | plugin | yes | PASS (`builder("energy")`/`semio:energy`, `🔋️energy/🦀️.rs:39,42` — NB the doc comment 3 lines above the real call is a decoy that also contains the string `Plugin::builder(...)`, read past it) | PASS | n/a | n/a | PASS | present | PASS |
| 13 | fem | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 14 | flow | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 15 | flow-extension-bim | extension | yes | PASS (literal `"flow-extension-bim"` passed directly, `🏗️bim/🦀️.rs:731,774` — NB a decoy `const EXTENSION_ID = "bim"` at `:762` is a *different*, module-scoped topic slug, not the bundle identity) | n/a | n/a | PASS (trivial) | PASS | MISSING | PASS (needs pair) |
| 16 | flow-extension-brep | extension | yes | PASS (literal) | n/a | n/a | PASS (trivial) | PASS | STALE (`packageId:""`) | PASS (stale pair) |
| 17 | flow-extension-dictionary | extension | yes | PASS (literal) | n/a | n/a | PASS (trivial) | PASS | STALE (`packageId:""`) | PASS (stale pair) |
| 18 | flow-extension-draw | extension | yes | PASS (literal) | n/a | n/a | PASS (trivial) | PASS | MISSING | PASS (needs pair) |
| 19 | flow-extension-list | extension | yes | PASS (literal) | n/a | n/a | PASS (trivial) | PASS | STALE (`packageId:""`) | PASS (stale pair) |
| 20 | flow-extension-logic | extension | yes | PASS (literal) | n/a | n/a | PASS (trivial) | PASS | STALE (`packageId:""`) | PASS (stale pair) |
| 21 | flow-extension-math | extension | yes | PASS (literal) | n/a | n/a | PASS (trivial) | PASS | STALE (`packageId:""`) | PASS (stale pair) |
| 22 | flow-extension-primitive | extension | yes | PASS (literal) | n/a | n/a | PASS (trivial) | PASS | STALE (`packageId:""`) | PASS (stale pair) |
| 23 | flow-extension-text | extension | yes | PASS (literal) | n/a | n/a | PASS (trivial) | PASS | STALE (`packageId:""`) | PASS (stale pair) |
| 24 | forms | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 25 | gis | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 26 | imperative | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 27 | imperative-extension-control | extension | yes | PASS (`EXTENSION_ID="imperative-extension-control"`, `🎮️control/🦀️.rs:36,60`) | n/a | n/a | PASS (trivial) | PASS | MISSING | PASS (needs pair) |
| 28 | **imperative-extension-effect** | extension | yes | **FAIL** — `EXTENSION_ID = "imperative-extension-core"` (`📣️effect/🦀️.rs:116`), bundle label `"Imperative Core"` (`:137`), Cargo pkg is `semio:imperative-extension-effect` | n/a | n/a | PASS (trivial) | **FAIL(e)** — manifest identity ≠ registry pluginId | MISSING (bug not yet surfaced) | **FAIL(a/e)** |
| 29 | imperative-extension-logic | extension | yes | PASS | n/a | n/a | PASS (trivial) | PASS | MISSING | PASS (needs pair) |
| 30 | imperative-extension-math | extension | yes | PASS | n/a | n/a | PASS (trivial) | PASS | MISSING | PASS (needs pair) |
| 31 | imperative-extension-text | extension | yes | PASS | n/a | n/a | PASS (trivial) | PASS | MISSING | PASS (needs pair) |
| 32 | layout | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 33 | lowpoly | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 34 | mathematical | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 35 | norm | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 36 | note | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 37 | playbook | plugin | yes | PASS | PASS | n/a | n/a | PASS | MISSING | PASS (needs pair) |
| 38 | playbook-module-procedural | extension | yes | PASS (`MODULE_PLUGIN_ID="playbook-module-procedural"`, `🌀️procedural(playbook)/🦀️.rs:28`) | n/a | n/a | PASS (trivial) | PASS | MISSING | PASS (needs pair) |
| 39 | procedural | plugin | yes | PASS | PASS-gated / **drift not gated** (see notes: `s.generation.{2d,3d}.{config,presence}`) | n/a | n/a | PASS | present | PASS (cosmetic drift noted) |
| 40 | process | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 41 | process-extension-concrete | extension | yes | PASS | n/a | n/a | PASS (trivial) | PASS | MISSING | PASS (needs pair) |
| 42 | process-extension-metal | extension | yes | PASS | n/a | n/a | PASS (trivial) | PASS | MISSING | PASS (needs pair) |
| 43 | process-extension-robotic | extension | yes | PASS | n/a | n/a | PASS (trivial) | PASS | MISSING | PASS (needs pair) |
| 44 | process-extension-wood | extension | yes | PASS | n/a | n/a | PASS (trivial) | PASS | MISSING | PASS (needs pair) |
| 45 | puzzle | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 46 | raster | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 47 | **reasoning-mindmap** | plugin | yes | **FAIL** — `builder("reasoning")`/`.package_id("semio:reasoning")` (`💡️reasoning/🦀️.rs:21,24`) vs Cargo `semio:reasoning-mindmap` (`…/📦️packages/🦀️rust/Cargo.toml:14`) and deployment row `{"pluginId":"reasoning-mindmap","directoryName":"💡️reasoning-mindmap"}` (`📇️registry/📦️deployment/🗺️catalog.json:50`) | self-consistent today (`s.reasoning.wires*`, owner="reasoning"==current builder id) but **will FAIL the instant (a) is fixed** unless every kind literal is renamed too | n/a | n/a | **FAIL(e)** | present (built before the drift, or from a prior identity) | **FAIL(a/e)** |
| 48 | remodel | plugin | yes | PASS (`builder("remodel")`/`semio:remodel`, `📸️remodel/🦀️.rs:25,28`) | PASS-gated / **drift not gated** (see notes: schema id `s.remodeling.remodeling*`) | n/a | n/a | PASS | present | PASS (cosmetic drift noted) |
| 49 | sequence | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 50 | shooting | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 51 | sourcing | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 52 | sourcing-module-beams | extension | yes | PASS | n/a | n/a | PASS (trivial) | PASS | MISSING | PASS (needs pair) |
| 53 | sourcing-module-slabs | extension | yes | PASS | n/a | n/a | PASS (trivial) | PASS | MISSING | PASS (needs pair) |
| 54 | sourcing-module-windows | extension | yes | PASS | n/a | n/a | PASS (trivial) | PASS | MISSING | PASS (needs pair) |
| 55 | space | plugin | yes | PASS (lane K fix landed: `builder("space")`/`semio:space`, `🪐️space/🦀️.rs:805,808`) | PASS | PASS (fixed by lane J/K — `S_HOME_CONTROLLER_ID`/`S_PLAY_CONTROLLER_ID`/`SPACE_INDEX_CONTROLLER_ID` now hold the full surface id) | n/a | PASS (rebuild 8 described cleanly per plan.md 04:55) | present | PASS |
| 56 | stdio | plugin | yes | PASS — **computed at runtime**: `component_package_id()` (`🗄️stdio/🦀️.rs:192-211`) parses its own `include_str!`-embedded Cargo.toml rather than using a literal; test at `:230` pins it to `"semio:stdio"`. Builder id is the literal `"stdio"` (`:244`) | PASS | n/a | n/a | PASS | MISSING | PASS (needs pair) |
| 57 | trinity | plugin | yes | PASS (`builder("trinity")`/`semio:trinity`, `🔱️trinity/🦀️.rs:33,36`) | PASS | n/a (see appendix note on `TRINITY_JACK_PLAY_CONTROLLER_ID`) | n/a | PASS | MISSING | PASS (needs pair) |
| 58 | vcs | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |
| 59 | writer | plugin | yes | PASS | PASS | n/a | n/a | PASS | present | PASS |

## Descriptor-pair inventory (🔣️.json owner descriptor)

- **19 rows have no pair file at all** (will get one fresh from the in-flight Wave 2 rebuild):
  `block`, `playbook`, `stdio`, `trinity` + 15 extensions (`flow-extension-bim`, `flow-extension-draw`,
  `imperative-extension-{control,effect,logic,math,text}`, `playbook-module-procedural`,
  `process-extension-{concrete,metal,robotic,wood}`, `sourcing-module-{beams,slabs,windows}`) — matches
  the 03:50 baseline's "19 rows … have no owner descriptor pair" exactly.
- **1 true placeholder** matching the brief's description literally: `cad-extension-aec-building`'s
  `🔣️.json` has `"manifest": {"pluginId": "empty", "label": "Empty", …}` and `"role": "plugin"` (wrong
  role too) — a completely generic stub, not even shaped like this extension.
- **9 more rows have a real, correctly-shaped pair but an empty `"packageId": ""`**, which the brief's "4
  CAD extensions" undercounts: `cad-extension-aec-building-energy`, `cad-extension-aec-building-structure`,
  `cad-extension-spatial-shape` (the other 3 CAD extensions) **plus** `flow-extension-brep`,
  `flow-extension-dictionary`, `flow-extension-list`, `flow-extension-logic`, `flow-extension-math`,
  `flow-extension-primitive`, `flow-extension-text` (7 flow extensions). Root cause for all 10 (and,
  prospectively, the other 16 extensions once they get a first pair): no extension anywhere calls
  `ExtensionBundle::package_id(...)` (confirmed by grep, see above) — this is a describe-route-wide gap,
  not a per-plugin bug.
- So **30 of 59 rows** (19 missing + 1 placeholder + 10 stale) need a fresh, correct pair; this matches
  the ballpark of the 06:00 log's "1/59 valid... 32 pairs lack packageId" figure (32 ≈ 26 extensions + a
  handful of plugin pairs also pre-dating the packageId requirement).

## Ranked fix list

1. **`imperative-extension-effect`** (`✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/🦀️.rs:116,137`):
   change `const EXTENSION_ID: &str = "imperative-extension-core";` → `"imperative-extension-effect"`, and
   the bundle label at `:137` `"Imperative Core"` → `"Imperative Effect"` (cosmetic but user-visible).
   Zero other files reference this constant outside the file (confirmed: `grep -rn` for
   `imperative-extension-core` found only this one definition site plus its own two use sites at
   `:121,137`). This has **no owner descriptor pair yet**, so fixing it now costs nothing extra — fixing
   it after Wave 2 assembles it costs a rebuild, exactly like lane K's four.
2. **`reasoning-mindmap`** (`✏️s/🔌️plugins/💡️reasoning/🦀️.rs:21,24`): change
   `Plugin::<ReasoningApps>::builder("reasoning")` → `builder("reasoning-mindmap")` and
   `.package_id("semio:reasoning")` → `.package_id("semio:reasoning-mindmap")`. This alone would then
   trip gate (b), so in the SAME change rename every owned artifact-kind literal under
   `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/**` from owner `reasoning` to `reasoning-mindmap`:
   the `ArtifactIdentity::parse("s.reasoning.wires")` root and its 10 capability-id siblings at
   `🗿️artifacts/🔌️wires/🦀️.rs:332-381` (`s.reasoning.wires.schema.artifact`,
   `s.reasoning.wires.inference.artifact`, `.composer.*` ×6, `.codec.document`, `.localization.*` ×2), plus
   the `#[artifact_schema(id = "s.reasoning.wires"...)]` attributes at
   `🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/{🦀️.rs,📸️snapshot/🦀️.rs,🔺️diff/🦀️.rs}` and
   `💡️inferences/🦀️.rs`, and the presence/config schema ids under `✏️editor/{👥️presence,🎚️config}/🧬️schema/🦀️.rs`.
   This is the exact multi-site, single-identity-string rename shape lane K already automated for `space`
   — treat it as the same recipe. Also update the deployment catalog row's `directoryName` expectation and
   any fixture pinning `"reasoning"` as the plugin id if one exists (not found in this pass — worth a
   second grep pass by whoever executes the fix, since I did not do a repo-wide, non-plugin-tree sweep for
   this literal the way lane K's census did for `s`→`space`).
3. **Cosmetic, non-blocking, do when convenient**: `remodel`'s schema id `s.remodeling.remodeling*` → `s.remodel.remodeling*` (4 sites under `📸️remodel/🗿️artifacts/📸️remodeling/…`), and `procedural`'s editor config/presence ids `s.generation.{2d,3d}.{config,presence}` → `s.procedural.generation{2d,3d}.{config,presence}` (4 sites under `🌀️procedural/🗿️artifacts/{🧊️generation3d,🌀️generation2d}/…/✏️editor/{🎚️config,👥️presence}/🧬️schema/🦀️.rs`). Neither trips a build gate today; both are pure naming-drift risk.
4. **Owner of lane B's extension-describe route should confirm** whether `ExtensionManifest.package_id`
   being permanently empty (no extension anywhere calls `.package_id(...)`) is tolerated by
   `describeBuiltPlugin`'s cross-check or will block all 26 extension pairs — I could not find the
   equivalent of `try_build()`'s `package_id.ok_or_else(...)` fatal check for the extension assembly path
   from source alone, so this needs either a source pointer I missed or one live build to observe.
5. **Not urgent, but log for the record**: 25/26 extensions declare `.extends(<owner>)` with no matching
   `.depends_on(<owner>, version)` — harmless today (none of them call `.contributes(...)`, the only gated
   path) but will hard-fail `register_contributions`'s `DependencyNotDeclared` the day any one of them
   gains a real mutation/inference contribution. `cad-extension-aec-building` is the only one that already
   does both correctly and can be used as the template.
