# Plugin Artifact App Law Capability Audit

Forms full native run 4 fails because downstream tests import artifact_app_laws while the SDK module and root reexport are gated by the existing artifact-app-testing feature. This feature is already default-off. Consumers need explicit dev-dependency requests. Runtime dependency declarations will remain unchanged.

The bounded source scan found 98 nearest Cargo owners and 0 unmapped source paths. The SDK own tests need no downstream request.

- /Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml

## Applied Capability Requests

Added artifact-app-testing to dev dependencies of 97 consumers, using their existing SDK dependency identity. All changed manifests parse with Bun.TOML. Runtime dependency tables compare identically before and after. Native compilation remains pending.

## Artifact Local Test Contexts

Full Forms and DAG native run 5 passes the shared SDK capability boundary, then exposes stale local artifact_app_laws names. Their actual local helper modules are context. Retargeted local references in 2 files without aliases or changes to the shared SDK namespace. DAG's dereference error appeared with unresolved application types and must be re-evaluated after name resolution.

- ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-node/🧪️tests/🔬️unit/🦀️.rs
