# Canonical Schema Consumer Dependency Audit

The ArtifactSchema generator emits canonical state and composition crate paths. These first-party consumer manifests own actual Rust schema declarations and currently lack direct dependency declarations. This is a source audit, not compiler/test success. Existing registry and semantic model edits are preserved.

| Consumer Manifest | Missing Direct Packages | Actual Declaration Source |
| --- | --- | --- |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/📦️packages/🦀️rust/Cargo.toml` | state, composition | `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs` |
| `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/Cargo.toml` | state | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🦀️.rs` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🐚️shell/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📦️packages/🦀️rust/Cargo.toml` | composition | `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs` |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/Cargo.toml` | state, composition | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |

## Declared Consumer Dependencies

Direct canonical first-party state/composition dependencies were declared in 80 currently missing consumer manifests. Every relative target was checked against its physical Cargo.toml. These edits declare dependencies required by actual owned schema definitions; they do not alter semantic snapshots, implement compatibility or infer SQL schemas. No behavioral success is claimed.

## 2026-10-02 11:33 UTC: Compiled Prerequisite Progress

Root54907 advanced beyond the prior missing registry and direct dependency errors, then stopped before owner assertions on the Mutations generator register function naming removed kernel StateClass. Its single signature now names canonical schema-state, which Kernel already declares. ArtifactSchema's two state paths were observed reverted concurrently to the removed schema facade again and corrected in place, preserving all other expansion logic. Compilation failures remain unexecuted owner laws.

## 2026-10-02 11:45 UTC: Composition Producer Correction

Root76353 ended before owner assertions: Architect sampled the Office owner's now-repaired XLSX qualification errors; Remodeling sampled ArtifactSchema composition/state references concurrently moved back into the schema facade while facade exports were removed. The measured missing symbols were StateClass, ArtifactCompositionFields, ChildFieldRefs, ChildRefVisitor, ChildSlotSpec and LinkSlotSpec. Their emitted paths are now narrowly corrected to canonical state/composition packages, preserving ArtifactSchemaFields in schema and all actual expansion behavior. Direct consumer dependencies were already declared. Source/default/compatibility adapters are not introduced.
## 2026-10-02 12:15 UTC: Actual Explicit Locale Consumer Prerequisite

Root lane 95054 drained before any Architect or Remodeling owner assertions. The compiler reported PresenceBar calling the removed `Locale::default()` at its wgpu builder. Its two ordinary callers are local tests; real host callers already use the explicit localized entry. The builder now requires the actual caller Locale and both existing structural tests declare English explicitly. The independently existing English/German localized cases remain unchanged. No default-language implementation was restored. The single warm parent replacement lane is running; no snapshot behavior result is attributed to this prerequisite.

## 2026-10-02 12:31 UTC: Canonical Locale and Explicit Context Propagation

The replacement lane 11375 also drained before owner assertions: the manifest's Default-derived ViewModel could not satisfy either explicit axis. ViewModel now has a hand-authored constructor requiring caller Locale and Terminology and no Default implementation. The breadcrumb's already documented unknown-terminology fallback explicitly selects Native. Shell and retained Ui constructors require caller Locale, with their impossible Default implementations removed. Existing local test contexts supply their intended axes explicitly; the current canonical Locale owner is independently updating the generated axes and imports, so those unrelated changes remain preserved.

The kernel's locale include was compiling a second copy of the generated axis and moved label types, distinct from the new canonical package already consumed by the manifest and UI. Kernel now directly declares and uses the actual first-party UI Locale package for its public label-bearing trait vocabulary. No duplicate implementation or deleted module was restored. The 92 actual artifact/shared consumers discovered to use canonical Locale now declare that real package path, and each added relative path was checked against the existing canonical package directory.

Shared host-event forwarding preserves caller view axes when projecting the remembered roster. Addressed action preview now requires the caller's explicit context instead of synthesizing an implicit-language view. These host context changes still require their owning runtime verification; they are compilation prerequisite corrections, not proof of a passing full framework or universal SQLite result. Lowpoly manifest windows no longer freeze resolved engagement copy during locale-free declaration; the already-existing actual runtime engagement method resolves the caller view.

The one warm replacement lane 77150 selects Architect, Remodeling and the new WAV neutral control laws. Every earlier preassert lane remains recorded as a compilation failure.

## 2026-10-02 12:45 UTC: Measured Private Label Producer and Consumer Repairs

Lane 77150 drained on Tool Run's actual undeclared canonical Locale dependency. Its literal relative path was corrected and checked against the canonical Cargo manifest. Fifteen additional actual shared consumers using canonical Locale now declare physically checked package edges, with proc-macro packages and the Locale package itself excluded from that consumer pass.

Lane 69752 drained before all new owner assertions. It exposed Plugin's moved import before its inner module doc comment, private interaction mutation label paths, the generated Mutations return type, the declared-verb harness's implicit ViewModel construction, and OS config/Workflow's handwritten private label consumers. These exact locations now refer to the actual canonical package. The two Mutations label return paths and repeatedly superseded state-class producer path are direct canonical references again. The three framework kernel label fields preserve their exact optionality and use canonical types.

Root preserved the current unrelated canonical Locale owner changes, including private facade imports. No removed facade exports were restored for this retry. Thirty-four actual OS config/Workflow source files were ported without changing their bilingual label text, mutation semantics or authored SQLite schemas, and both owners declare the physically checked direct dependency.

The remaining Puzzle2d locale-free manifest construction is now neutral, with its existing caller-bound runtime measures/engagements preserved. Dispatch requires caller context when it resolves labels. Reserved jobs carry only optional ephemeral caller axes, copied from their actual ActionMeta; locked clipboard notifications resolve those explicit axes or return a typed fault. OS host instance construction now requires the caller ViewModel. The two existing host unit fixtures explicitly supply their intended axes. These host runtime changes still require owning verification and are not claimed green by the snapshot selector.

One new warm root lane 29715 is active for Architect, Remodeling and WAV. The previous compile failures are not snapshot feature REDs, and the universal goal remains incomplete.

### 2026-10-02 12:53 UTC — Stdio Contract Canonical Locale Edge

Root native lane 29715 reached the shared stdio registry contract and stopped WAV and Architect before snapshot assertions: fourteen unresolved canonical `semio_framework_ui_locale` imports per target. The Rust worker independently observed the same fourteen errors in Grid3d. The existing contract imports already refer to the canonical owner; its Cargo manifest lacked that direct dependency. Added the actual internal locale package path and confirmed that it resolves to the existing package manifest. No facade reexport or duplicate locale definition was added. Remodel remains running in the original lane; all affected feature verdicts remain unverified until their assertions execute.

### 2026-10-02 12:57 UTC — Direct Stdio Mutation Labels

Three independent owned lanes reached the same private facade label defect in Binary; root WAV additionally reached its own handwritten mutation labels. Ported the exact `protocol::LocalizedLabel` and `dsl::LocalizedLabel` references in 989 stdio source files to `semio_framework_ui_locale::LocalizedLabel`, across 36 actual owning artifact manifests. Existing bilingual strings and mutation execution remain intact. Each actual owning package declares its canonical internal locale edge, and any newly required relative path was checked against the physical package manifest. No compatibility alias or fallback language was introduced. Runtime validation remains pending the fresh owning lanes.

### 2026-10-02 13:02 UTC — Remaining Actual Mutation Label Consumers

Completed the same exact private-facade label prerequisite in 1999 remaining non-stdio source files (1427 in the first pass and 572 in the completed pass). The one consumer outside an ancestor artifact package is the Norm results window mutation; verified its actual `#[path]` inclusion and canonical direct dependency in the Norm registry contract. The final pass covered 20 actual packages. No handwritten mutation strings or execution were changed and no facade aliases were restored. Actual owner tests remain required; source coherence is not behavioral admission.

### 2026-10-02 13:13 UTC — Remodeling Native Compile Prerequisites

Root lane 92801 closed with no new feature assertions: WAV captured earlier private Binary/WAV labels, Architect captured the XLSX bool-closure inference defect, and Remodeling reached 19 actual owning compile errors. Corrected the actual bounded UI component label consumers to their existing UI contract owner; verified or added direct internal package edges. The first pass stopped at a genuinely absent Process3d contract edge, which the completed pass now declares. No conversion was added to the unrelated locale value. Root-added Remodeling fixture entry points now use the physically existing nested JSON serializer/deserializer modules, the actual generic declaration-tree `artifact` constructor and `declare_artifact` builder, and explicit `crate::MediaKind`. The test app follows its actual Editor/Viewer member types rather than assuming SemioMembers. Handwritten SQLite and Native JSON production remain unchanged pending authentic owner baselines. Fresh root lane 11984 is active on Architect/Remodel/WAV and the shared native IEEE laws.

### 2026-10-02 13:15 UTC — Current Value Derives and Dotted Dependency Form

Root lane 11984 closed. Shared native SQLite selected 28 laws, 27 passed and the new law stopped at its independent oracle stdout comparison because forced terminal color decorated console output; the IEEE reader had not yet run. Replaced console formatting with raw Bun stdout, leaving production Rust unchanged. Artifact targets stopped at eight duplicate derive macro imports in four current UI component scopes after the canonical Value package began exporting those macros; removed only the redundant explicit derive imports. Root had added an inline DAG UI contract dependency despite its existing dotted workspace declaration, causing TOML metadata admission to fail. Removed that duplicate while retaining the existing workspace declaration; exact manifest parsing follows. These are prerequisites, not owned feature baselines.

## Current Architect Native Assertion Results — 2026-10-02 at 13:24 UTC

Nextest `cfd45a0b-3d70-4107-9fcd-053bd30fea10` executed all 22 selected Architect laws: 21 passed, one failed in 13.615 s. The 2,096 ordinary tests were outside this selector, with no selected skips. Both new unsigned64 JSON roundtrips, all register fields, actual erased capability, exact native row admission, nested parent controllers, and physical INTEGER precision now pass. The sole genuine failure admitted Binary native output under an authored SQL schema ceiling one byte too small. Explicit schema-length admission is now mounted on both native directions after that measured failure; fresh runtime verification is pending.

The WGPU frame-worker output previously reported as missing by Source worker graph snapshots now has an exact current producer in the existing TypeScript package's generate-frame-worker target and script route. Root's current four-owner Nx native graph executed successfully. No graph bypass or producer change was made by Root; one fresh Source worker retry is warranted against this actual readback.

## Architect Selected Native Repairs Verified on 2026-10-02

The one warm replacement gate measured Architect 22/22 selected Native laws passing, Nextest `15c1ae18-3f93-4438-8a3d-0ada245f74f5`, 7.235 seconds, with 2,096 ordinary owner tests outside the selector and no selected skips. The actual negative/exact authored schema boundary now passes in both native directions. This completes the measured repair scope for JSON unsigned64, all 65 registers, exact INTEGER scalars, parent controllers, semantic row ceilings and actual declaration/I/O. Whole owning-package verification is the next separate scope, not claimed here.
