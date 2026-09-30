# Physical Cargo Ownership Enforcement

The ownership enforcement slice is implemented and verified. Production fleet refactors and final shared-tree integration continue in parallel.

## Implemented Contract

The schema now requires ordered physical owner-role classifications. Both authored TOML and Cargo metadata can agree on a wrong valid role; that agreement no longer conceals an ownership violation. Unknown physical classifications and role-owner mismatches are independent gate failures.

Physical source patterns are an additional selector union with semantic source roles. General plugin services remain protected when nested beneath a plugin and declared as libraries/modules. Specific artifact and extension descendants remain distinct. Exact physical extension target segments also participate in target selection. No severity downgrade, compatibility facade, baseline exemption, or suppression was added.

Package discovery and Cargo enforcement now share the same ten-role vocabulary. Ninety-six direct artifact manifests previously declared the generic `s-module` role; their metadata is now `artifact`. Direct extension package owners must declare extension. The nine Stdio components currently declaring plugin under extension owners remain explicit ownership failures; their runtime PluginBuilder worlds and compiled descriptors require a coherent follow-on extraction, so metadata was not relabeled to conceal that mismatch.

## Test-Driven Evidence

The required owner-classification schema and neutral fixtures were authored first. The initial registered uncached Nx target failed: one pass, three failures, 157 assertions. Before the physical source selector implementation, the expanded corpus failed again: two passes, three failures, 217 assertions. The first owner-classification green run passed five tests and 255 assertions, using actual offline Cargo metadata, independent @iarna/toml parsing, and Ajv schema validation.

The corpus covers shared wrong valid metadata on source plugins/modules and target extensions, physical artifact mislabels, generic services, distinct path-segment decoys, optional platform declarations, and missing/malformed classification authority. It materializes complete neutral workspaces and requires exact parity of all graph declarations against independent Cargo output.

Runtime console evidence after the ninety-six metadata edits:

```text
[DEBUG] Canonical ownership: {"packages":278,"artifacts":103,"roleProblems":0,"violations":290}
```

This was an authored-inventory observation before the final physical selector change, not the final live Cargo oracle verdict. Counts differ from the prior increment because the checkout is shared and package composition changed concurrently; no causal reduction is claimed.

## Attribution

The final physical-selector corpus passed five tests and 289 assertions through the registered uncached Nx target. Library/coordinator typecheck passed in 23.6 seconds after adding explicit first-party types to three callbacks whose runtime array validation narrowed them to `any`.

Actual package discovery and taxonomy validation printed:

```text
[DEBUG] Discovery ownership: {"packages":292,"roles":{"plugin":46,"artifact":103,"s-module":6,"extension":26,"tool":7,"test":4,"hub":2,"framework":65,"product":11,"library":22},"unknownRoles":[],"taxonomyProblems":[]}
```

An intermediate uncached live Cargo snapshot failed: 301 strict violations (291 plugin/artifact, nine plugin/extension, one module/plugin), zero metadata problems, 3,171 local declarations and 278 packages. The [complete snapshot](./🔍️2026-09-30-continuation-cargo-live-violations.md) records every edge. Nine extension edges and the spatial-session artifact edge are exposed by the additional physical predicates; the implementation does not downgrade or excuse them.

Shared-file edits are restricted to the new ownership contract, validation, vocabulary, tests, and artifact role fields. No source behavior or unrelated Cargo fields were changed by the coordinator.

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️cargo/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️cargo-dependency-direction/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️cargo-dependency-direction/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️cargo-dependency-direction/🟦️.ts`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/Cargo.toml`

## Shared-Tree Integration Evidence

Bun workspace links and the authored lockfile were refreshed with `bun install --ignore-scripts`; the command exited zero and installed one package. The new artifact-owned Puzzle package is included by the authored workspace membership.

The coordinator registered the Stdio contract and Energy model canonical-architecture selections and the two new framework/GIS transport verification routes using the existing canonical launch producer. Only exact owned configurations and selector options were patched; the canonical-architecture family configuration retains its existing name, group, and order. Producer parity was observed through runtime console output.

The three Puzzle Cargo playground contributions now point to the outward application composition at `✏️s/🧑‍💻dev/🎭️variants/🧩️puzzle/🧬️schema/🔣️.json`, coordinated with the TypeScript executor. No legacy paths are retained.

The uncached registered Rust source gate completed with zero detected strict compile-time boundary edges across 2,191 framework files and 4,052 authored references. It nevertheless **failed** on one unsupported interpolated `include_str!` at `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:595`. An unsupported generated quote cannot prove its emitted file dependencies; the gate stays fail-closed. No passing Rust source verdict is claimed.

Additional coordinator-owned files: `bun.lock`, `.vscode/launch.json`, `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml`, and the complete live Cargo Markdown report.

The additional production-policy witness reproduced the extension/plugin overlap as a failing test: five passes, one failure, 299 assertions, with the neutral Cargo/TOML/Ajv corpus passing. The authored extension classifier was then tightened to extension only. Final regression and live gate results follow after execution.

The final uncached library/coordinator typecheck passed after the TypeScript resolver integration (3m56s under shared build contention). Root-owned git diff HEAD --check also passed at this stage.
