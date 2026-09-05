# Framework Strict Artifact-Identity Admission Audit

Status: **RED — source admission presently permits malformed legacy identity and foreign ownership.** This is a current-source review only; no build or runtime execution was performed.

## Exact admission defect

`ArtifactKindId` is deliberately strict: it accepts only `s.<plugin>.<artifact>` and exposes the owning second segment at [`🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:103-155`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:103).  The old declaration channel does not enforce it:

- [`app::ArtifactDeclaration::preflight`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:3474) first registers its definition into the candidate registry, then uses `if let Ok(canonical) = ArtifactKindId::parse(&self.kind)`.
- A two-segment or otherwise malformed kind therefore skips the ownership check and reaches later registration.  `ArtifactDeclaration::builder` derives `kind` straight from `ArtifactDefinition.identity` ([`🦀️.rs:3035-3041`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:3035)), making the bypass reachable from every old `.artifact(...)` caller.

The new tree has a separate gap.  Its root `kind` is typed and syntactically canonical ([`🦀️.rs:27201-27205`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27201)), but its preflight receives no `plugin_id` ([`🦀️.rs:27267`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27267)). `PluginBuilder::try_build` commits it before walking the old channels ([`🏗️builder/🦀️.rs:635-642`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:635)). Thus `Plugin::builder("a")` can currently declare `s.b.document`; this is valid syntax but false ownership.

`artifact_definition(...)` is a third admission channel ([`🏗️builder/🦀️.rs:225-230`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:225)). It is only passed to `ArtifactDefinitionRegistry::register`, whose validation is definition/capability consistency and duplicate claims, not canonical artifact-kind ownership ([`🦀️.rs:2865-2902`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:2865)). It needs the same gate.

## Current first-party reachability census

### Active old channel — 18 legacy artifacts, all invalid after the correct strict gate

There are 16 source roots with a live fluent `.artifact(...)` call.  One is Norm's 15 now-canonical declarations; the other 15 roots carry 18 old two-segment definition identities.  These must be converted source-first, not exempted.

| Plugin root | Current definition identity | Canonical document identity already used elsewhere / required owner root |
| --- | --- | --- |
| `🔋️energy/🦀️.rs:35-38` | `s.model` | `s.energy.model` (`🗿️artifacts/🔋️model/🦀️.rs:304`) |
| `🏭️process/🦀️.rs:26-29` | `s.process3d` | `s.process.process3d` |
| `📜️imperative/🦀️.rs:32-35` | `s.procedure` | `s.imperative.procedure` |
| `💠️lowpoly/🦀️.rs:28-31` | `s.lowpoly` | `s.lowpoly.lowpoly` |
| `🌊️flow/🦀️.rs:25-28` | `s.flow` | `s.flow.flow` |
| `🖨️raster/🦀️.rs:31-34` | `s.raster` | `s.raster.raster` |
| `📏️layout/🦀️.rs:35-38` | `s.layout` | `s.layout.layout` |
| `🌍️gis/🦀️.rs:27-32` | `s.gismap`, `s.gisterrain` | `s.gis.gismap`, `s.gis.gisterrain` |
| `📸️remodel/🦀️.rs:25-28` | `s.remodeling` | `s.remodel.remodeling` |
| `🎥️shooting/🦀️.rs:25-28` | `s.shooting` | `s.shooting.shooting` |
| `🌀️procedural/🦀️.rs:251-256` | `s.generation2d`, `s.generation3d` | `s.procedural.generation2d`, `s.procedural.generation3d` |
| `🪐️space/🦀️.rs:584-593` | `s.home`, `s.space` | `s.space.home`, `s.space.space`; the plugin id itself is currently the false one-letter `"s"` |
| `🏛️architect/🦀️.rs:26-29` | `s.program` | `s.architect.program` |
| `🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️.rs:48-51` | `s.playground` | `s.demonstrator.playground` |
| `📐️cad/🦀️.rs:26-29` | `s.cad` | `s.cad.cad` |

The exact identity sources are `ArtifactDefinition::new(ArtifactIdentity::parse(...))`, for example energy at [`.../🔋️model/🦀️.rs:304`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs:304), GIS map at [`.../🗺️gismap/🦀️.rs:322`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:322), and Home at [`.../🏠️home/🦀️.rs:62`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🦀️.rs:62). The old builder copies those values to the untyped declaration kind, so the strict check will reject every row until the entire identity tree is rebased.

For each row, the migration unit is the *definition root plus its descendants*: capability identity strings, claimed native dialect coordinates, composer claims, schema/inference owner metadata, and all artifact identity lookups must move together.  Schema, report, and codec payload identifiers only remain unchanged when they are genuinely external schema/format values rather than a claim of this artifact root.  In particular, do not paper over this by accepting `s.<artifact>` or by renaming distinct `computation.*` output kinds.

### New declaration tree — syntactically valid but owner checks are missing

There are 16 live `.declare_artifact(...)` roots. Fifteen currently omit mandatory `.package_id(...)`; only VCS supplies `semio:vcs`. This is a runtime `PluginBuilder::try_build` rejection at the ready-builder assembly boundary, not a typestate compile frontier and not an ownership substitute. The two active canonical-owner mismatches are material:

- [`✏️s/🔌️plugins/📖️playbook/🦀️.rs:31-34`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📖️playbook/🦀️.rs:31) builds plugin `playbook-play`, while its declaration is `s.playbook.playbook` at [`.../📖️playbook/🦀️.rs:311`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs:311).
- [`✏️s/🔌️plugins/💡️reasoning/🦀️.rs:21-24`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/🦀️.rs:21) builds `reasoning-mindmap`, while its declaration is `s.reasoning.wires` at [`.../🔌️wires/🦀️.rs:397`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🦀️.rs:397).

The source-clean choice is to make each plugin root match its declared semantic owner (`playbook`, `reasoning`) and give it the matching `semio:<plugin>` component package identity; do not mutate correct document kinds to preserve historical root slugs. All other current new-tree kinds are `ArtifactKindId::parse(...)` values and need a cross-owner audit once the gate is installed.

### Definitions not currently fed to an old live builder

The same census finds 23 two-segment `ArtifactDefinition` roots in declaration-tree/deferred sources: sourcing, writer, dag, both trinity artifacts, sequence, reasoning, all three block artifacts, playbook, draw, both FEM artifacts, procedural assembly, animate, all three puzzle artifacts, forms, mathematical equation, note, and VCS. They are not proof of a present old-channel runtime path, but they are first-party authority sources and will reintroduce the bypass if an old registration is restored. Rebase them in the same source-owned identity packet rather than retaining a second identity vocabulary.

The Stdio registration is the positive control: its `runtime_assembly` and `definition_only_assembly` already require `definition.identity() == s.stdio.<artifact>` ([`✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:653-673`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:653)); its dynamic `.artifact_definition`/`.artifact` fan-in is at [`✏️s/🔌️plugins/🗄️stdio/🦀️.rs:243-250`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🦀️.rs:243).

## Smallest strict-admission closure

1. Add one internal helper which parses an artifact root with `ArtifactKindId::parse`, maps parse failure to a stable `plugin-assembly.artifact-kind` error, then requires `kind.plugin() == plugin_id`, reporting `plugin-assembly.artifact-owner` otherwise. It must run before the definition candidate is accepted.
2. Call it for all three builder channels:
   - old `ArtifactDeclaration::preflight`, replacing the `if let Ok` branch;
   - `artifact_definitions` before `ArtifactDefinitionRegistry::register` in `PluginBuilder::try_build`;
   - each new `declarations::ArtifactDeclaration.kind`, by threading `&plugin_id` through both preflight and commit.
3. Make the top-level builder preflight every channel before calling the new-tree commit. Otherwise a later old-channel rejection can occur after `commit_artifact_declarations` has populated global schema/codec/io registries. The new tree's own preflight is already separable at [`🦀️.rs:27267-27301`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27267); use it once in the all-channel preflight phase and only then commit.
4. Rebase the 41 definition roots as cohesive plugin packets, beginning with the 18 currently live old-channel rows. Add `.package_id("semio:<plugin>")` to every live root before treating its plugin as executable; this builder requirement is already enforced at [`🏗️builder/🦀️.rs:620-628`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:620).

## Required focused laws

All must use a fresh process/unique registry fixture and invoke `Plugin::builder(...).try_build()`, not a helper in isolation.

1. A legacy two-segment definition (`s.fixture`) rejects `plugin-assembly.artifact-kind`; no artifact/schema/codec/io/app row is committed.
2. A syntactically canonical foreign old declaration (`s.other.document` under `owner`) rejects `plugin-assembly.artifact-owner` before registration.
3. A foreign `artifact_definition` rejects through the same owner boundary, proving the definition-only channel cannot bypass it.
4. A foreign new declaration (`s.other.document`) rejects under `owner`; assert all global registries are unchanged. Reuse the existing declaration fixture at [`🦀️.rs:27687-27739`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27687), whose public `kind` field can be replaced with another parsed canonical value.
5. A mixed-channel plugin with a valid new declaration plus a rejected old/definition-only candidate leaves zero rows from the valid new declaration. This proves all-channel preflight precedes the new-tree commit.
6. Positive controls: `stdio` definition-only and runtime assembly plus one newly canonicalized non-stdio plugin build successfully; assert the manifest plugin id, package id, and every admitted root's `ArtifactKindId.plugin()` agree.

The existing declaration testkit helpers themselves predate mandatory package identities: [`🦀️.rs:7026`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:7026), [`🦀️.rs:7055`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:7055), and fixture route test [`🦀️.rs:27784`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27784) omit `.package_id(...)`. Repair those test builders to use the exact matching `semio:<fixture-id>` first; otherwise the new laws fail at `try_build` before exercising their intended admission assertions.

## Acceptance / nonclaims

Accept this packet only when the three channels reject malformed and foreign roots before any global registration, all first-party active roots use one canonical identity hierarchy, and the listed unique-process laws pass. A source grep, a parser-only unit test, or a compile that does not execute the registration transaction is not runtime admission proof. This packet does not claim catalog activation, codec availability, app rendering, or document-open readiness.
