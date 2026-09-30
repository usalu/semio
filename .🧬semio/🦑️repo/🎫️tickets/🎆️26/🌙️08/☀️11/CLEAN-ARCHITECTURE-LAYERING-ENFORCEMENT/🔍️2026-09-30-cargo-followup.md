# Cargo Ownership Follow-up

Read-only exploration on 2026-09-30. Reviewed earlier boundary and semantic-direction reports and re-ran `cargo metadata --format-version 1 --no-deps` successfully (exit 0). No compilation, tests, implementation, ticket status changes or modifying Git operations. Metadata was streamed in memory; no generated output files remain.

## Current Physical Edges

Classified by resolved manifest/package directory rather than crate-name prefixes. Every listed dependency is local and explicitly present in current metadata, including optional/platform/dev declarations. These are declared edges, not a claim about activation in a particular build.

### implementation: 59 edges ({'normal': 50, 'dev': 9})

| Source manifest | Target package owner | Kind | Optional / platform |
| --- | --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust` | dev | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust` | dev | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/📦️packages/🦀️rust` | dev | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust` | dev | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust` | dev | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/📦️packages/🦀️rust` | dev | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust` | dev | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/📦️packages/🦀️rust` | dev | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/📦️packages/🦀️rust` | dev | False / `all` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` | `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust` | normal | False / `cfg(not(target_os = "wasi"))` |

### physical-os: 15 edges ({'normal': 14, 'dev': 1})

| Source manifest | Target package owner | Kind | Optional / platform |
| --- | --- | --- | --- |
| `🧰️framework/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` | dev | False / `all` |
| `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` | normal | True / `all` |
| `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🔨️modules/📚️compiler/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust` | normal | False / `all` |
| `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` | normal | False / `all` |

## Ownership Interpretation

The physical OS kernel package declares `package.metadata.semio.role = "framework"` in its manifest lines 8–10. Thus the physical neutral-module→OS-owner list must not be presented as 15 violations of current semantic roles. It identifies owner-placement/public-contract work; the metadata classifies the kernel as framework. The graph neural engine, infinite canvas and infinite DAG artifact are also physically beneath the OS product and need explicit policy decisions. The UI kernel edge is optional; value derive's kernel edge is dev-only. No build edges occur in either selected physical boundary set.

The framework→implementation set contains 48 normal fixture-sweep artifact dependencies, Flow's normal stdio-semio dependency and nine Flow extension dev dependencies, and renderer-wgpu's normal Puzzle dependency behind `cfg(not(target_os = "wasi"))`. The renderer source search for `puzzle::`/`semio_s_plugin` found no direct Rust use; a broader `puzzle` search found only a renderer comment. This makes removal of the manifest edge a promising tiny slice, but macro/generated references and feature/build behavior still need checking before claiming it unused.

Plugin host has no direct s dependency in current Cargo metadata. Its concrete guest integration tests still discover implementation component bytes: `🔌️plugin/🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs:36` walks `✏️s/🔌️plugins`; rows 368–371 select Note/GIS/stdio components and pin concrete schemas. `🧪️tests/🔬️poll-turn-memory/🦀️.rs:109` targets a Procedural component path. These are runtime test-input ownership edges outside Cargo's package-dependency graph, not hidden stdio Cargo linkage.

## Smallest Clean Slices

1. **Check and remove renderer's unreferenced Puzzle dependency.** The current declaration is `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml:44`. Search all source/build/macro/config inputs and run the owner source-check with the same native cfg. If no consumer exists, delete the declaration directly and its stale allowlist entry; no extraction or forwarding module is needed. This would eliminate one actual normal framework→s edge.

2. **Relocate fleet fixture composition to the s integration owner.** The fixture-sweep crate is already an explicit test leaf (`role = "test"`), with only a `[[test]]` target at manifest lines 17–19. The fleet suite imports real implementation snapshots at `🧪️tests/🔬️fleet-example-sweep/🦀️.rs:12–70`; these belong to an implementation composition owner. Move its manifest/router/project and concrete fleet laws under an existing suitable s integration domain, updating Cargo workspace membership, workspace aliases, Nx root/sourceRoot, extraction/coverage assertions, and generated launch metadata atomically. Preserve the existing real example assets in their owners and neutral kernel grammar/protocol laws in framework. Do not relocate unused historical neutral law files merely to preserve an old folder, and do not add a forwarding crate. This moves 48 actual normal Cargo composition edges out of framework in one coherent ownership change. The current script invokes framework-owned extraction helpers in `📜️script.ts:4`; move the concrete extraction expectation with the concrete fleet owner and retain any generic law receipt mechanism in repo library.

3. **Move Flow concrete extension tests before touching its production BREP implementation.** Nine extension dev dependencies are used in `🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:70–93` and `🌊️flow/🧪️tests/🔌️port-types/🦀️.rs:74–76`. Implementation-owned integration laws should depend downward on Flow; neutral registry/port laws should exercise neutral fixtures. The normal stdio edge is different: `🌊️flow/📐️brep-geometry/🦀️.rs:11–12` directly consumes concrete BREP kernel and tessellation types, with error/topology/mesh types exposed later. Clean options are moving this domain-specific module and registration into the s BREP extension, or extracting a real neutral geometry protocol/engine with all consumers updated. An interface that still names stdio-owned errors/transfer classes would not remove ownership coupling. The fixture at `🌊️flow/🧪️tests/📐️brep-invoke/🦀️.rs:24` remains stdio-owned and should move only its consuming integration law.

## Proposed Verification for Implementation

Existing routes inspected: `bun nx run @semio-tech/dsl-fixture-sweep-rs:source-check`, owner `test`/`test-long`/`test-exhaustive` targets, and root canonical owner aggregation. Existing launch source-check row is `.vscode/launch.json:5189–5192`. Do not run broad Cargo builds for this exploration. A future change should add a schema-first language-neutral Cargo boundary corpus with normal/build/dev/platform/optional/path/workspace-alias cases, independently validate it using Cargo metadata, and enforce physical areas plus explicit semantic-role policy. Cargo packages with a missing role must fail classification rather than disappear; stale violation exemptions must fail. Actual runtime/test verification and regenerated launch rows are required before asserting the extraction works.

## Focused Renderer and Fleet Follow-up

### Renderer Puzzle Declaration

Source inspection strengthens the unused-declaration conclusion:

- Renderer package manifest line 44 declares alias `puzzle` solely under `cfg(not(target_os = "wasi"))`; no feature enables it separately. It is therefore admitted on native and `wasm32-unknown-unknown`, and absent from WASI. Native binary additionally requires `native-bin`.
- `📦️packages/🦀️rust/📚️library/🦀️.rs:3–12` mounts renderer registration and source outside WASI, with native runtime manifest parsing outside wasm32. The registration file exports only `action_args_json!`, expanding into `semio_framework::dsl_value!`; it does not name Puzzle.
- `build.rs` only includes the canonical builder. `🏗️builder/🦀️.rs` generates SVG icon embeds and copies the logo. It emits no plugin/crate registration or Puzzle symbol.
- A framework-wide Rust/TS/JSON/TOML search for `puzzle::`, `extern crate puzzle`, `use puzzle`, `semio_s_plugin_puzzle` and Puzzle Cargo feature names found only old explanatory policy comments and generated component artifact taxonomy entries, not Rust consumers. The mounted renderer source and engine's sibling elements/tests were included in this search.
- Puzzle-named renderer functions do exist, for example `🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:3572–3581`, but they parse JSON/MIME payloads using framework/serde types. A Puzzle function name is not use of the Puzzle crate. Native runtime manifests load generic completed component descriptors/bytes; that mechanism does not require linking the guest crate into the renderer.

The exact minimal clean change is removing the one manifest dependency line and the corresponding stale known-violation entry in `🧑‍💻dev/🧪️tests/🧹️layering-policy/🟦️.ts:72`, rewriting its surrounding obsolete exception explanation, and refreshing Cargo.lock only if renderer's dependency list changes there. Keep actual component artifact staging/activation intact. No source extraction, alias, shim, or forwarder is indicated by current authored inputs. Static evidence is strong; unusedness is not compiler-confirmed in this read-only audit.

The actual owning Nx project is `@semio-tech/framework-renderer-wgpu`, registered in the target's TypeScript package, not its Rust package (there is no Rust `📋️project.json`/`📜️script.ts`). Existing validation routes are `test-wgpu-unit` (Cargo library laws), `test-native` (budgeted Cargo crate tests), `check-wasm` (locked/offline browser Rust library check), and `native-build` (native binary with `native-bin`). `test-wgpu-unit` has launch registration at `.vscode/launch.json:4675`; `native-build` at line 18708; browser check uses the generated project-target picker. A future implementation should begin with fresh Cargo metadata proving the direct edge disappears, then compiler-check both native and browser cfg using those owner routes. The library test route compiles native mounted modules; the native binary route additionally covers binary-only code. Existing upstream compilation debt may block confirmation and must remain an explicit failure. No command here was executed.

### Fleet Composition Owner and Neutral Seams

The appropriate destination is a fleet test leaf beneath the existing implementation dev owner: proposed `✏️s/🧑‍💻dev/🧹️fixture-sweep`, with its package at `📦️packages/🦀️rust` and concrete suite at `🧪️tests/🔬️fleet-example-sweep`. `✏️s/🧑‍💻dev` already owns implementation dev composition; `✏️s/🧪️tests` has no current files and does not provide a package/router to reuse. A neutral s module would remain an inappropriate owner for a fleet's concrete plugin imports. This is a proposed domain leaf, not an assertion that it already exists.

Only the fleet composition should move: its test-only Cargo package, package script/project, real-snapshot registry/laws, fleet-owned preservation expectations and concrete discovery receipts. Keep authored examples/schema fixtures with their current artifact owners. Keep kernel-only M5 mounts and their generic parser/grammar/protocol helpers in framework. The existing `🧹️fixture-sweep/🧪️tests/🧹️fixture-sweep/🦀️.rs` explicitly describes itself as kernel-only and is mounted by the kernel; blindly moving the entire current folder would break that ownership.

Neutral interfaces already exist: the fleet registry's callback type is `fn(&str) -> Result<(), String>` (`fleet-example-sweep/🦀️.rs:77`); registry entries invoke generic `os_store::ArtifactDsl::envelope_id()` and `os_store::test_support::check_dsl_fixture_text_laws::<Snapshot>` (lines 81 onward). Keep these generic trait/law helpers with their neutral current authority, while the implementation owns which concrete Snapshot types it registers. The generic `runExactCargoLaws` and typed group/receipt mechanism belongs to repo test infrastructure; the concrete package/target/law inventory must belong to fleet composition. No new runtime dependency or generic API carrying implementation-owned snapshot types is needed.

Atomic edits must include root Cargo member path/name, package-local relative dependencies, Nx root/sourceRoot/cwd, root DSL orchestration's owner contribution, generated launch metadata, and the fixture-extraction schema/expected path/digest inventories. One concrete pre-existing mismatch needs reconciliation: the current Cargo manifest mounts `fleet-example-sweep`, whereas `🗣️dsl/🧪️tests/🧹️fixture-sweep/🟦️.ts:206` still requires the test target to be `🧪️tests/🧹️fixture-sweep/🦀️.rs` (kernel-only suite); its runner/coverage assumptions still mention M5 counts. This audit did not execute the source-check, so it reports the source mismatch rather than a runtime failure. The relocation must rewrite this preservation contract around the actual fleet target and keep separate kernel verification, without accepting stale historical paths via compatibility branches.
