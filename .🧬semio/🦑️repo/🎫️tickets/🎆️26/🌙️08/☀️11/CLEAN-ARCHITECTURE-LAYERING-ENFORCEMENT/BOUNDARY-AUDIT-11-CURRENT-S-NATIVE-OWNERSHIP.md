# Current S Native Ownership Audit 11

Read-only source audit on 2026-10-08. No Cargo invocation, test, runtime, physical deletion, Git mutation, or source mutation was performed. Root and S AGENTS were read. The existing root ticket is reused by delegation; ticket lifecycle and RepoMCP availability are owned by the parent agent.

## Exhaustive Finite Inventory

Walked every Cargo.toml below actual `✏️s`, omitting only node_modules, target, .git, .nx, and ticket generated directories. Parsed 394 manifests with Bun.TOML: 256 workspace-bearing manifests, 261 packages, 138 intrinsic packages with 138 unique original names, 123 package manifests in standards/guest/oracles/bridge scopes. These scopes were inventoried rather than silently discarded. There are 133 intrinsic defining workspaces; the remaining 123 workspaces belong to excluded scopes. Every intrinsic package is an explicit member of its actual nearest defining owner and package.workspace resolves to that same owner. Every workspace-inherited dependency used by every scanned package has a provider in its nearest workspace. All 3712 resolved local dependency edges, including target/dev/build sections and excluded packages, point to existing Cargo.toml files. These are source existence observations, not Cargo semantic resolution or compilation results.

S owns precisely three neutral members, preserving their actual module ownership. S providers are framework-job, serde, serde_json. There is no fabricated aggregate S source entry. Five actual direct plugin workspace owners own seven intrinsic parent packages: Trinity Jack LSP; Norm registry contract; Block catalog and root; Stdio registry contract and root; WFC engine. Their member/provider inventories contain no descendant artifact or extension path dependency. Stdio root depends on its own registry contract outside artifact descendants.

A lexical scan of all 163 Rust source files in those five parent plugin scopes excluded descendant artifacts/extensions/bridge directories. Its 14 matching references are intrinsic own contracts, tests, or oracle doc links; no production descendant crate reference was found by that scan. This lexical evidence is not a complete compiler module closure or semantic proof. The seven actual lib entry points were separately read.

## Remaining Admission Weaknesses

The permanent law `✏️s/🧪️tests/🗂️native-ownership/🟦️.ts` checks S provider neutrality but does not check plugin parent providers against descendant artifacts/extensions. Its dynamic owner loop checks declared members only: an unlisted new intrinsic package can be entirely ignored. It does not inventory all package manifests independently, confirm nearest ownership, validate every inherited provider, validate direct/target/dev/build paths, or establish path existence independently. It skips standards/guest/oracles/bridge ownership scopes completely. The test can therefore remain green while descendant providers, direct paths, undeclared packages, or a foreign explicit workspace are added.

The path corpus compares POSIX normalization against native resolve on literal /owner and assumes slash-separated native output. On native Windows, resolve yields drive/backslash paths, so path.startsWith(base + '/') cannot represent the same projection. POSIX absolute-path screening also does not reject a Windows drive/UNC member. Its uniqueness test uses raw member strings rather than normalized paths, allowing syntactically different aliases. No Windows execution was performed.

Naming remains inconsistent with current intrinsic taxonomy: `semio-s-artifact-norm-contract` and `semio-s-artifact-stdio-contract` now define parent registry contracts outside artifact descendants. Their retained original names are not evidence of an artifact dependency; a clean taxonomy decision remains separate from owner correctness. No rename or compatibility layer was introduced in this audit.

Physical plugin/artifact deletion and full resolve/compile acceptance remain unproved by this audit. Metadata-only existence cannot settle native source changes or transitive framework dependencies. Shared files may change after this snapshot; hashes below identify the observed bytes.

## Complete Intrinsic Package Inventory

| Name | Manifest | Actual Nearest Owner |
| --- | --- | --- |
| semio-s-spatial-kernel-semio-session | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/Cargo.toml | ✏️s |
| semio-s-imperative-extension-sdk | ✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/📦️packages/🦀️rust/Cargo.toml | ✏️s |
| semio-s-imperative | ✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust/Cargo.toml | ✏️s |
| semio-s-composition-laws | ✏️s/🧑‍💻dev/🧩️composition/📦️packages/🦀️rust/Cargo.toml | ✏️s/🧑‍💻dev/🧩️composition |
| semio-s-fixture-sweep | ✏️s/🧑‍💻dev/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml | ✏️s/🧑‍💻dev/🧹️fixture-sweep |
| semio-s-flow-composition | ✏️s/🧑‍💻dev/🌊️flow/📦️packages/🦀️rust/Cargo.toml | ✏️s/🧑‍💻dev/🌊️flow |
| semio-s-dev-cad | ✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/Cargo.toml | ✏️s/🧑‍💻dev/📐️cad |
| semio-s-dev-services | ✏️s/🧑‍💻dev/💡️services/📦️packages/🦀️rust/Cargo.toml | ✏️s/🧑‍💻dev/💡️services |
| semio-s-plugin-trinity-jack-lsp | ✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🔱️trinity |
| semio-s-artifact-trinity-jack-shell | ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🐚️shell/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack |
| semio-s-artifact-trinity-jack | ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack |
| semio-s-artifact-trinity-rewriting | ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting |
| semio-s-artifact-remodel-remodeling | ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling |
| semio-s-artifact-raster-raster | ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster |
| semio-s-artifact-flow-flow | ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow |
| semio-s-plugin-flow-extension-brep | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep |
| semio-s-plugin-flow-extension-dictionary | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary |
| semio-s-plugin-flow-extension-bim | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim |
| semio-s-plugin-flow-extension-logic | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic |
| semio-s-plugin-flow-extension-primitive | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive |
| semio-s-plugin-flow-extension-math | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math |
| semio-s-plugin-flow-extension-list | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list |
| semio-s-plugin-flow-extension-draw | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw |
| semio-s-plugin-flow-extension-text | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text |
| semio-s-artifact-process-process3d | ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d |
| semio-s-plugin-process-metal | ✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal |
| semio-s-plugin-process-wood | ✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood |
| semio-s-plugin-process-robotic | ✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic |
| semio-s-plugin-process-concrete | ✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete |
| semio-s-artifact-norm-contract | ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm |
| semio-s-artifact-norm-en1995 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995 |
| semio-s-artifact-norm-en1999 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999 |
| semio-s-artifact-norm-din4108 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108 |
| semio-s-artifact-norm-en1998 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998 |
| semio-s-artifact-norm-en1991 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991 |
| semio-s-artifact-norm-en1990 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990 |
| semio-s-artifact-norm-en1996 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996 |
| semio-s-artifact-norm-en1994 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994 |
| semio-s-artifact-norm-en1992 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992 |
| semio-s-artifact-norm-din16798 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798 |
| semio-s-artifact-norm-din18599 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599 |
| semio-s-artifact-norm-en1993 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993 |
| semio-s-artifact-norm-iso16757 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757 |
| semio-s-artifact-norm-vdi3805 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805 |
| semio-s-artifact-norm-en1997 | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997 |
| semio-s-artifact-cad-cad | ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad |
| semio-s-plugin-cad-aec-building | ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building |
| semio-s-plugin-cad-spatial-shape | ✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape |
| semio-s-plugin-cad-aec-building-structure | ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure |
| semio-s-plugin-cad-aec-building-energy | ✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy |
| semio-s-artifact-demonstrator-playground | ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground |
| semio-s-plugin-block-catalog | ✏️s/🔌️plugins/🧱️block/🗂️catalog/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🧱️block |
| semio-s-artifact-block-3d | ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d |
| semio-s-artifact-block-5d | ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d |
| semio-s-artifact-block-2d | ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d |
| semio-s-plugin-block | ✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🧱️block |
| semio-s-artifact-dag-dag | ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag |
| semio-s-artifact-stdio-contract | ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio |
| semio-s-artifact-stdio-bcf | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf |
| semio-s-artifact-stdio-mp4 | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4 |
| semio-s-artifact-stdio-stl | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl |
| semio-s-artifact-stdio-avi | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi |
| semio-s-artifact-stdio-dxf | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf |
| semio-s-artifact-stdio-ifc | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc |
| semio-s-artifact-stdio-epw | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw |
| semio-s-artifact-stdio-wav | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav |
| semio-s-artifact-stdio-bmp | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp |
| semio-s-artifact-stdio-mp3 | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3 |
| semio-s-artifact-stdio-svg | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg |
| semio-s-artifact-stdio-jpg | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg |
| semio-s-artifact-stdio-semio | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio |
| semio-s-artifact-stdio-html | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html |
| semio-s-artifact-stdio-deflate | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate |
| semio-s-artifact-stdio-zip | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip |
| semio-s-artifact-stdio-step | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step |
| semio-s-artifact-stdio-pptx | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx |
| semio-s-artifact-stdio-xml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml |
| semio-s-artifact-stdio-md | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md |
| semio-s-artifact-stdio-tiff | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff |
| semio-s-artifact-stdio-txt | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt |
| semio-s-artifact-stdio-binary | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary |
| semio-s-artifact-stdio-png | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png |
| semio-s-artifact-stdio-dwg | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg |
| semio-s-artifact-stdio-json | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json |
| semio-s-artifact-stdio-xlsx | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx |
| semio-s-artifact-stdio-ply | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply |
| semio-s-artifact-stdio-docx | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx |
| semio-s-artifact-stdio-pdf | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf |
| semio-s-artifact-stdio-las | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las |
| semio-s-artifact-stdio-gif | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif |
| semio-s-artifact-stdio-gltf | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf |
| semio-s-artifact-stdio-csv | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv |
| semio-s-artifact-stdio-tsv | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv |
| semio-s-artifact-stdio-obj | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj |
| semio-s-plugin-stdio | ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio |
| semio-s-artifact-reasoning-wires | ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires |
| semio-s-artifact-sequence-sequence | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence |
| semio-s-artifact-writer-writer | ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer |
| semio-s-artifact-animate-presentation | ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation |
| semio-s-artifact-space-home | ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home |
| semio-s-artifact-space-space | ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space |
| semio-s-artifact-procedural-generation3d | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d |
| semio-s-artifact-procedural-generation2d | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d |
| semio-s-artifact-vcs-vcs | ✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs |
| semio-s-artifact-gis-gismap | ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap |
| semio-s-artifact-gis-gisterrain | ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain |
| semio-s-artifact-wfc-3d | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d |
| semio-s-artifact-wfc-grid3d | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d |
| semio-s-artifact-wfc-bitmap | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap |
| semio-s-artifact-wfc-2d | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d |
| semio-s-artifact-wfc-grid2d | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d |
| semio-s-plugin-wfc-engine | ✏️s/🔌️plugins/🀄️wfc/⚙️engine/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🀄️wfc |
| semio-s-artifact-imperative-procedure | ✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure |
| semio-s-plugin-imperative-logic | ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic |
| semio-s-plugin-imperative-effect | ✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect |
| semio-s-plugin-imperative-math | ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math |
| semio-s-plugin-imperative-control | ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control |
| semio-s-plugin-imperative-text | ✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text |
| semio-s-artifact-sourcing-curation | ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation |
| semio-s-plugin-sourcing-beams | ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams |
| semio-s-plugin-sourcing-slabs | ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs |
| semio-s-plugin-sourcing-windows | ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows |
| semio-s-artifact-note-note | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note |
| semio-s-artifact-forms-forms | ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms |
| semio-s-artifact-architect-program | ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program |
| semio-s-artifact-shooting-shooting | ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting |
| semio-s-artifact-mathematical-equation | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation |
| semio-s-artifact-layout-layout | ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout |
| semio-s-artifact-puzzle-3d | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d |
| semio-s-artifact-puzzle-5d | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d |
| semio-s-artifact-puzzle-2d | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d |
| semio-s-artifact-fem-3d | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d |
| semio-s-artifact-fem-2d | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d |
| semio-s-artifact-draw-drawing | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing |
| semio-s-artifact-playbook-playbook | ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook |
| semio-s-plugin-playbook-procedural | ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural |
| semio-s-artifact-lowpoly-lowpoly | ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly |
| semio-s-artifact-energy-model | ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model |

## Excluded Package Inventory

| Name | Manifest | Actual Nearest Owner |
| --- | --- | --- |
| semio-trinity-mutation-bridge | ✏️s/🔌️plugins/🔱️trinity/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🔱️trinity/🏭️bridge |
| semio-remodel-mutation-bridge | ✏️s/🔌️plugins/📸️remodel/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/📸️remodel/🏭️bridge |
| semio-raster-mutation-bridge | ✏️s/🔌️plugins/🖨️raster/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🖨️raster/🏭️bridge |
| semio-flow-mutation-bridge | ✏️s/🔌️plugins/🌊️flow/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🌊️flow/🏭️bridge |
| semio-process-mutation-bridge | ✏️s/🔌️plugins/🏭️process/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🏭️process/🏭️bridge |
| semio-norm-mutation-bridge | ✏️s/🔌️plugins/📕️norm/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/📕️norm/🏭️bridge |
| semio-cad-mutation-bridge | ✏️s/🔌️plugins/📐️cad/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/📐️cad/🏭️bridge |
| semio-demonstrator-mutation-bridge | ✏️s/🔌️plugins/🎪️demonstrator/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🎪️demonstrator/🏭️bridge |
| semio-block-mutation-bridge | ✏️s/🔌️plugins/🧱️block/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🧱️block/🏭️bridge |
| semio-dag-mutation-bridge | ✏️s/🔌️plugins/🕸️dag/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🕸️dag/🏭️bridge |
| semio-s-plugin-stdio-drawing-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/📦️packages/🦀️rust |
| semio-s-plugin-stdio-raster-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖼️raster/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖼️raster/📦️packages/🦀️rust |
| semio-s-plugin-stdio-mesh-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🧊️mesh/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🧊️mesh/📦️packages/🦀️rust |
| semio-s-plugin-stdio-archive-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🎒️archive/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🎒️archive/📦️packages/🦀️rust |
| semio-s-plugin-stdio-tabular-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📊️tabular/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📊️tabular/📦️packages/🦀️rust |
| semio-s-plugin-stdio-document-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📃️document/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📃️document/📦️packages/🦀️rust |
| semio-s-plugin-stdio-markup-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📰markup/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📰markup/📦️packages/🦀️rust |
| semio-s-plugin-stdio-part21-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔤️part21/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔤️part21/📦️packages/🦀️rust |
| semio-s-plugin-stdio-audio-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔊️audio/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔊️audio/📦️packages/🦀️rust |
| semio-s-artifact-stdio-bcf-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-mp4-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-stl-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-avi-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🔮️oracles/📦️packages/🦀️rust |
| riff-avi-codec | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| semio-s-artifact-stdio-dxf-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🔮️oracles/📦️packages/🦀️rust |
| dxf-r12-any-fixture-generator | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust |
| semio-s-artifact-stdio-ifc-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-epw-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-wav-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-bmp-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🔮️oracles/📦️packages/🦀️rust |
| image-bmp-codec | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| semio-s-artifact-stdio-mp3-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-svg-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🔮️oracles/📦️packages/🦀️rust |
| quick-xml-svg-codec | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| semio-s-artifact-stdio-jpg-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🔮️oracles/📦️packages/🦀️rust |
| jpeg-jfif-codec | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| semio-s-artifact-stdio-semio-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🔮️oracles/📦️packages/🦀️rust |
| semio-stdio-drawing-oracle-probe | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📖️reader/📦️packages/🦀️rust |
| semio-drawing-json-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/📦️packages/🦀️rust |
| drawing-v1-svg-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| semio-cad-oracle-probe | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📖️reader/📦️packages/🦀️rust |
| semio-cad-json-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/📦️packages/🦀️rust |
| semio-mesh-json-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/📦️packages/🦀️rust |
| semio-brep-json-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/📦️packages/🦀️rust |
| semio-semio-v1-base-bridge | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🏭️bridge |
| semio-document-json-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/📦️packages/🦀️rust |
| semio-s-artifact-stdio-html-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-deflate-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-zip-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-step-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-pptx-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-xml-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🔮️oracles/📦️packages/🦀️rust |
| quick-xml-oracle-codec | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| semio-s-artifact-stdio-md-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-tiff-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🔮️oracles/📦️packages/🦀️rust |
| tiff-6-0-byte-order-reader | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📖️reader/📦️packages/🦀️rust |
| tiff-ifd-codec | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| semio-s-artifact-stdio-txt-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-binary-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-png-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🔮️oracles/📦️packages/🦀️rust |
| png-codec | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| semio-s-artifact-stdio-dwg-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-json-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🔮️oracles/📦️packages/🦀️rust |
| json-rfc8259-base-serde-json-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/📦️packages/🦀️rust |
| semio-s-artifact-stdio-xlsx-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-ply-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-docx-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-pdf-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🔮️oracles/📦️packages/🦀️rust |
| pdf-1-7-e-lopdf-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| pdf-1-7-h-lopdf-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| pdf-1-7-x-lopdf-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| pdf-1-7-base-fixture-generator | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust |
| pdf-1-7-base-lopdf-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| pdf-1-7-vt-lopdf-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| pdf-1-7-ua-lopdf-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| pdf-1-7-a-lopdf-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| pdf-1-4-x-lopdf-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| pdf-1-4-base-lopdf-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| pdf-1-4-a-lopdf-engine | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| semio-s-artifact-stdio-las-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🔮️oracles/📦️packages/🦀️rust |
| las-1-0-any-fixture-generator | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust |
| semio-s-artifact-stdio-gif-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🔮️oracles/📦️packages/🦀️rust |
| gif-89a-extension-reader | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/📖️reader/📦️packages/🦀️rust |
| gif-89a-any-fixture-generator | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust |
| gif-87a-any-reader | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust |
| semio-s-artifact-stdio-gltf-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🔮️oracles/📦️packages/🦀️rust |
| gltf-2-0-any-resource-reader | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📖️reader/📦️packages/🦀️rust |
| semio-s-artifact-stdio-csv-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-tsv-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🔮️oracles/📦️packages/🦀️rust |
| semio-s-artifact-stdio-obj-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🔮️oracles/📦️packages/🦀️rust |
| tobj-obj-reader | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/📦️packages/🦀️rust |
| obj-3-0-any-fixture-generator | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust |
| semio-reasoning-mutation-bridge | ✏️s/🔌️plugins/💡️reasoning/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/💡️reasoning/🏭️bridge |
| semio-sequence-mutation-bridge | ✏️s/🔌️plugins/🎬️sequence/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🎬️sequence/🏭️bridge |
| semio-writer-mutation-bridge | ✏️s/🔌️plugins/✒️writer/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/✒️writer/🏭️bridge |
| semio-animate-mutation-bridge | ✏️s/🔌️plugins/🎞️animate/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🎞️animate/🏭️bridge |
| semio-space-mutation-bridge | ✏️s/🔌️plugins/🪐️space/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🪐️space/🏭️bridge |
| semio-procedural-mutation-bridge | ✏️s/🔌️plugins/🌀️procedural/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🌀️procedural/🏭️bridge |
| semio-vcs-mutation-bridge | ✏️s/🔌️plugins/🌿️vcs/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🌿️vcs/🏭️bridge |
| semio-gis-mutation-bridge | ✏️s/🔌️plugins/🌍️gis/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🌍️gis/🏭️bridge |
| semio-wfc-mutation-bridge | ✏️s/🔌️plugins/🀄️wfc/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🀄️wfc/🏭️bridge |
| semio-imperative-mutation-bridge | ✏️s/🔌️plugins/📜️imperative/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/📜️imperative/🏭️bridge |
| semio-sourcing-mutation-bridge | ✏️s/🔌️plugins/🪵️sourcing/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🪵️sourcing/🏭️bridge |
| semio-s-artifact-note-note-test-oracle | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔮️oracles/📦️packages/🦀️rust |
| note-oracle-codec | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust |
| semio-note-mutation-bridge | ✏️s/🔌️plugins/🗒️note/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🗒️note/🏭️bridge |
| semio-forms-mutation-bridge | ✏️s/🔌️plugins/📋️forms/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/📋️forms/🏭️bridge |
| semio-architect-mutation-bridge | ✏️s/🔌️plugins/🏛️architect/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🏛️architect/🏭️bridge |
| semio-shooting-mutation-bridge | ✏️s/🔌️plugins/🎥️shooting/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🎥️shooting/🏭️bridge |
| semio-equation-oracle-probe | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust |
| equation-1-any-json-engine | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust |
| semio-mathematical-mutation-bridge | ✏️s/🔌️plugins/➗️mathematical/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/➗️mathematical/🏭️bridge |
| semio-layout-mutation-bridge | ✏️s/🔌️plugins/📏️layout/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/📏️layout/🏭️bridge |
| semio-puzzle-mutation-bridge | ✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🧩️puzzle/🏭️bridge |
| fem3d-1-any-json-engine | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust |
| fem2d-1-any-json-engine | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust |
| semio-fem-mutation-bridge | ✏️s/🔌️plugins/🏗️fem/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🏗️fem/🏭️bridge |
| semio-drawing-oracle-probe | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust |
| drawing-1-any-json-engine | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust |
| semio-draw-mutation-bridge | ✏️s/🔌️plugins/🖍️draw/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🖍️draw/🏭️bridge |
| semio-playbook-mutation-bridge | ✏️s/🔌️plugins/📖️playbook/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/📖️playbook/🏭️bridge |
| semio-lowpoly-mutation-bridge | ✏️s/🔌️plugins/💠️lowpoly/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/💠️lowpoly/🏭️bridge |
| semio-energy-mutation-bridge | ✏️s/🔌️plugins/🔋️energy/🏭️bridge/Cargo.toml | ✏️s/🔌️plugins/🔋️energy/🏭️bridge |

## Workspace Member And Provider Inventory

### ✏️s

Members: ["🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust","🔨️modules/📜️imperative/📦️packages/🦀️rust","🔨️modules/📜️imperative/🧩️extension_sdk/📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔌️plugins","🧑‍💻dev"]

Providers: {"semio-framework-job":{"path":"../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🧑‍💻dev/🧩️composition

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework":{"path":"../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-flow-flow":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust"},"semio-framework-artifact-infinite-dag":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust"},"semio-framework-artifact-playbook-playbook":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-flow":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-os-kernel-neural-engine":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-trace":{"path":"../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"},"semio-framework-ui":{"path":"../../../🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-procedural-generation2d":{"path":"../../🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust"},"semio-s-artifact-procedural-generation3d":{"path":"../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust"},"semio-s-artifact-sequence-sequence":{"path":"../../🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-imperative":{"path":"../../🔨️modules/📜️imperative/📦️packages/🦀️rust"},"semio-s-plugin-flow-extension-brep":{"path":"../../🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust","default-features":false},"semio-s-plugin-flow-extension-draw":{"path":"../../🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust"},"semio-s-plugin-flow-extension-math":{"path":"../../🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust","default-features":false},"semio-s-spatial-kernel-semio-session":{"path":"../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🧑‍💻dev/🧹️fixture-sweep

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-flow-flow":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust"},"semio-framework-artifact-playbook-playbook":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust"},"semio-framework-artifact-space-collection":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/📦️packages/🦀️rust"},"semio-framework-artifact-space-space":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust"},"semio-framework-artifact-workflow-workflow":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-s-artifact-animate-presentation":{"path":"../../🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust"},"semio-s-artifact-block-2d":{"path":"../../🔌️plugins/🧱️block/🗿️artifacts/◻️2d/📦️packages/🦀️rust"},"semio-s-artifact-block-3d":{"path":"../../🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🦀️rust"},"semio-s-artifact-block-5d":{"path":"../../🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🦀️rust"},"semio-s-artifact-cad-cad":{"path":"../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust"},"semio-s-artifact-dag-dag":{"path":"../../🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust"},"semio-s-artifact-draw-drawing":{"path":"../../🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust"},"semio-s-artifact-fem-2d":{"path":"../../🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust"},"semio-s-artifact-fem-3d":{"path":"../../🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust"},"semio-s-artifact-gis-gismap":{"path":"../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust"},"semio-s-artifact-gis-gisterrain":{"path":"../../🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust"},"semio-s-artifact-imperative-procedure":{"path":"../../🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/📦️packages/🦀️rust"},"semio-s-artifact-layout-layout":{"path":"../../🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust"},"semio-s-artifact-lowpoly-lowpoly":{"path":"../../🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust"},"semio-s-artifact-mathematical-equation":{"path":"../../🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust"},"semio-s-artifact-norm-din16798":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🦀️rust"},"semio-s-artifact-norm-din18599":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📦️packages/🦀️rust"},"semio-s-artifact-norm-din4108":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🦀️rust"},"semio-s-artifact-norm-en1990":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📦️packages/🦀️rust"},"semio-s-artifact-norm-en1991":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/📦️packages/🦀️rust"},"semio-s-artifact-norm-en1992":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust"},"semio-s-artifact-norm-en1993":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/📦️packages/🦀️rust"},"semio-s-artifact-norm-en1994":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/📦️packages/🦀️rust"},"semio-s-artifact-norm-en1995":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📦️packages/🦀️rust"},"semio-s-artifact-norm-en1996":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/📦️packages/🦀️rust"},"semio-s-artifact-norm-en1997":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/📦️packages/🦀️rust"},"semio-s-artifact-norm-en1998":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/📦️packages/🦀️rust"},"semio-s-artifact-norm-en1999":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/📦️packages/🦀️rust"},"semio-s-artifact-norm-iso16757":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/📦️packages/🦀️rust"},"semio-s-artifact-norm-vdi3805":{"path":"../../🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🦀️rust"},"semio-s-artifact-note-note":{"path":"../../🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust"},"semio-s-artifact-procedural-generation2d":{"path":"../../🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust"},"semio-s-artifact-procedural-generation3d":{"path":"../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust"},"semio-s-artifact-process-process3d":{"path":"../../🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust"},"semio-s-artifact-puzzle-2d":{"path":"../../🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust"},"semio-s-artifact-puzzle-3d":{"path":"../../🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust"},"semio-s-artifact-puzzle-5d":{"path":"../../🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust"},"semio-s-artifact-raster-raster":{"path":"../../🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust"},"semio-s-artifact-reasoning-wires":{"path":"../../🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust"},"semio-s-artifact-remodel-remodeling":{"path":"../../🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust"},"semio-s-artifact-sequence-sequence":{"path":"../../🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust"},"semio-s-artifact-shooting-shooting":{"path":"../../🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust"},"semio-s-artifact-sourcing-curation":{"path":"../../🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust"},"semio-s-artifact-space-home":{"path":"../../🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🦀️rust"},"semio-s-artifact-trinity-jack":{"path":"../../🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust"},"semio-s-artifact-trinity-rewriting":{"path":"../../🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust"},"semio-s-artifact-vcs-vcs":{"path":"../../🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📦️packages/🦀️rust"},"semio-s-artifact-writer-writer":{"path":"../../🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🦀️rust"}}

### ✏️s/🧑‍💻dev/🌊️flow

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework":{"path":"../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-flow-flow":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust"},"semio-framework-os-flow":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust"},"semio-s-spatial-kernel-semio-session":{"path":"../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🧑‍💻dev/📐️cad

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-3d":{"path":"../../../🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust"},"semio-framework-os-flow":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust"},"semio-framework-os-kernel-neural-engine":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust"},"semio-s-artifact-stdio-step":{"path":"../../🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust"},"semio-s-spatial-kernel-semio-session":{"path":"../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🧑‍💻dev/💡️services

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-os-kernel":{"path":"../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-os-mcp":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust"},"semio-framework-os-renderer-wgpu":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust"},"semio-s-artifact-gis-gismap":{"path":"../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🔱️trinity

Members: ["🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🗿️artifacts","🧩️extensions"]

Providers: {}

### ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack

Members: ["🐚️shell/📦️packages/🦀️rust","📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-flow-flow":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-geometry":{"path":"../../../../../🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust"},"semio-framework-graph":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust"},"semio-framework-graph-layout-run":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-md":{"path":"../../../🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-svg":{"path":"../../../🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"semio-s-artifact-trinity-jack":{"path":"📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-graph":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-infinite":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-replication":{"path":"../../../../../🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-docx":{"path":"../../../🗄️stdio/🗿️artifacts/📜️docx/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-md":{"path":"../../../🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust"},"semio-s-artifact-stdio-pdf":{"path":"../../../🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"semio-s-artifact-trinity-jack":{"path":"../🔌️jack/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🔱️trinity/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-s-artifact-stdio-avi":{"path":"../../../🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🦀️rust"},"semio-s-artifact-stdio-gltf":{"path":"../../../🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-jpg":{"path":"../../../🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-las":{"path":"../../../🗄️stdio/🗿️artifacts/☁️las/📦️packages/🦀️rust"},"semio-s-artifact-stdio-mp4":{"path":"../../../🗄️stdio/🗿️artifacts/🎥️mp4/📦️packages/🦀️rust"},"semio-s-artifact-stdio-obj":{"path":"../../../🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust"},"semio-s-artifact-stdio-ply":{"path":"../../../🗄️stdio/🗿️artifacts/🧱️ply/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-stl":{"path":"../../../🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📸️remodel/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-s-artifact-stdio-bmp":{"path":"../../../🗄️stdio/🗿️artifacts/🪟️bmp/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../../../🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-gif":{"path":"../../../🗄️stdio/🗿️artifacts/🎞️gif/📦️packages/🦀️rust"},"semio-s-artifact-stdio-jpg":{"path":"../../../🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-svg":{"path":"../../../🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-tiff":{"path":"../../../🗄️stdio/🗿️artifacts/🖼️tiff/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🖨️raster/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-flow-flow":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust"},"semio-framework-artifact-infinite-dag":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust"},"semio-framework-artifact-playbook-playbook":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-md":{"path":"../../../🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-mesh-engine":{"path":"../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-step":{"path":"../../../🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust"},"semio-s-spatial-kernel-semio-session":{"path":"../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/🌊️flow/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../../../🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-gltf":{"path":"../../../🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-ifc":{"path":"../../../🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-obj":{"path":"../../../🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-step":{"path":"../../../🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust"},"semio-s-artifact-stdio-stl":{"path":"../../../🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-s-artifact-process-process3d":{"path":"../../🗿️artifacts/🧊️process3d/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-s-artifact-process-process3d":{"path":"../../🗿️artifacts/🧊️process3d/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-s-artifact-process-process3d":{"path":"../../🗿️artifacts/🧊️process3d/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-s-artifact-process-process3d":{"path":"../../🗿️artifacts/🧊️process3d/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🏭️process/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/📕️norm

Members: ["📇️registry/🧬️contract/📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🗿️artifacts","🧩️extensions"]

Providers: {"pack":{"path":"../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-fem-2d":{"path":"../../../🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-fem-2d":{"path":"../../../🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-norm-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📕️norm/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../../🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../../../🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-obj":{"path":"../../../🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-step":{"path":"../../../🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust"},"semio-s-artifact-stdio-stl":{"path":"../../../🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-s-artifact-cad-cad":{"path":"../../🗿️artifacts/📐️cad/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/📐️cad/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xlsx":{"path":"../../../🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../../../🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🎪️demonstrator/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🧱️block

Members: ["📦️packages/🦀️rust","🗂️catalog/📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🗿️artifacts","🧩️extensions"]

Providers: {"semio-framework":{"path":"../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-dsl-record":{"path":"../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust"},"semio-framework-dsl-record-derive":{"path":"../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust"},"semio-framework-value":{"path":"../../../🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../../../🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../../../🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../../../🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🧱️block/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-flow-flow":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust"},"semio-framework-artifact-infinite-dag":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-graph-layout-run":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-md":{"path":"../../../🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-svg":{"path":"../../../🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xml":{"path":"../../../🗄️stdio/🗿️artifacts/📰️xml/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🕸️dag/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio

Members: ["📇️registry/🧬️contract/📦️packages/🦀️rust","📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🗿️artifacts","🧩️extensions","🔮️oracles/🎒️archive/📦️packages/🦀️rust","🔮️oracles/📃️document/📦️packages/🦀️rust","🔮️oracles/📊️tabular/📦️packages/🦀️rust","🔮️oracles/📰markup/📦️packages/🦀️rust","🔮️oracles/🔊️audio/📦️packages/🦀️rust","🔮️oracles/🔤️part21/📦️packages/🦀️rust","🔮️oracles/🖊️drawing/📦️packages/🦀️rust","🔮️oracles/🖼️raster/📦️packages/🦀️rust","🔮️oracles/🧊️mesh/📦️packages/🦀️rust","🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust","🗿️artifacts/☁️las/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🌐️html/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🌦️epw/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🎒️zip/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust","🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust","🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/📖️reader/📦️packages/🦀️rust","🗿️artifacts/🎞️gif/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🎥️mp4/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/🎨️svg/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🎵️mp3/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🏗️ifc/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/💬️bcf/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/💾️binary/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/📊️csv/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/📐️step/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/📑️tsv/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/📕️xlsx/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust","🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/📖️pdf/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/📜️docx/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/📝️md/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/📰️xml/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/📷️png/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/📸️jpg/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/📼️avi/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/📽️pptx/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🔊️wav/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🔤️txt/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🔺️stl/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🖊️dwg/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust","🗿️artifacts/🖋️dxf/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📖️reader/📦️packages/🦀️rust","🗿️artifacts/🖼️tiff/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🗜️deflate/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/📦️packages/🦀️rust","🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust","🗿️artifacts/🗽️obj/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📖️reader/📦️packages/🦀️rust","🗿️artifacts/🧊️gltf/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🧱️ply/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/📦️packages/🦀️rust","🗿️artifacts/🧾️json/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/📦️packages/🦀️rust","🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📖️reader/📦️packages/🦀️rust","🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/📦️packages/🦀️rust","🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/📦️packages/🦀️rust","🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/📦️packages/🦀️rust","🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📖️reader/📦️packages/🦀️rust","🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/📦️packages/🦀️rust","🗿️artifacts/🧿️semio/🔮️oracles/📦️packages/🦀️rust","🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust","🗿️artifacts/🪟️bmp/🔮️oracles/📦️packages/🦀️rust"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖼️raster/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🧊️mesh/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🎒️archive/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📊️tabular/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📃️document/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📰markup/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔤️part21/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔊️audio/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xml":{"path":"../📰️xml/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../🎒️zip/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../🔤️txt/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../🔤️txt/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-step":{"path":"../📐️step/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../🔤️txt/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xml":{"path":"../📰️xml/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/📦️packages/🦀️rust","🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📖️reader/📦️packages/🦀️rust","🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/📦️packages/🦀️rust","🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/📦️packages/🦀️rust","🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/📦️packages/🦀️rust","🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/📦️packages/🦀️rust","🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📖️reader/📦️packages/🦀️rust","🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-3d":{"path":"../../../../../🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-geometry":{"path":"../../../../../🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust"},"semio-framework-graph":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-math":{"path":"../../../../../🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust"},"semio-framework-mesh-engine":{"path":"../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-avi":{"path":"../📼️avi/📦️packages/🦀️rust"},"semio-s-artifact-stdio-bcf":{"path":"../💬️bcf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-bmp":{"path":"../🪟️bmp/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-docx":{"path":"../📜️docx/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dxf":{"path":"../🖋️dxf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-gif":{"path":"../🎞️gif/📦️packages/🦀️rust"},"semio-s-artifact-stdio-gltf":{"path":"../🧊️gltf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-ifc":{"path":"../🏗️ifc/📦️packages/🦀️rust"},"semio-s-artifact-stdio-jpg":{"path":"../📸️jpg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-las":{"path":"../☁️las/📦️packages/🦀️rust"},"semio-s-artifact-stdio-md":{"path":"../📝️md/📦️packages/🦀️rust"},"semio-s-artifact-stdio-mp3":{"path":"../🎵️mp3/📦️packages/🦀️rust"},"semio-s-artifact-stdio-mp4":{"path":"../🎥️mp4/📦️packages/🦀️rust"},"semio-s-artifact-stdio-obj":{"path":"../🗽️obj/📦️packages/🦀️rust"},"semio-s-artifact-stdio-pdf":{"path":"../📖️pdf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-ply":{"path":"../🧱️ply/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-pptx":{"path":"../📽️pptx/📦️packages/🦀️rust"},"semio-s-artifact-stdio-step":{"path":"../📐️step/📦️packages/🦀️rust"},"semio-s-artifact-stdio-stl":{"path":"../🔺️stl/📦️packages/🦀️rust"},"semio-s-artifact-stdio-svg":{"path":"../🎨️svg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-tiff":{"path":"../🖼️tiff/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../🔤️txt/📦️packages/🦀️rust"},"semio-s-artifact-stdio-wav":{"path":"../🔊️wav/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xml":{"path":"../📰️xml/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../🎒️zip/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📖️reader/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📖️reader/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-deflate":{"path":"../🗜️deflate/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xml":{"path":"../📰️xml/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-3d":{"path":"../../../../../🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-flow":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../🔤️txt/📦️packages/🦀️rust"},"semio-s-spatial-kernel-semio-session":{"path":"../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xml":{"path":"../📰️xml/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../🎒️zip/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../🔤️txt/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../🔤️txt/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust","🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📖️reader/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-deflate":{"path":"../🗜️deflate/📦️packages/🦀️rust"},"semio-s-artifact-stdio-jpg":{"path":"../📸️jpg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../📷️png/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📖️reader/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-deflate":{"path":"../🗜️deflate/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../🎒️zip/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-mesh-engine":{"path":"../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../🔤️txt/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xml":{"path":"../📰️xml/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../🎒️zip/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../🔤️txt/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xml":{"path":"../📰️xml/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../🎒️zip/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust","🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust","🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust","🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/📦️packages/🦀️rust","🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/📦️packages/🦀️rust","🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/📦️packages/🦀️rust","🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust","🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust","🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust","🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust","🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-deflate":{"path":"../🗜️deflate/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust","🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust","🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/📖️reader/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-binary":{"path":"../💾️binary/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/📖️reader/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📖️reader/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../🧾️json/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📖️reader/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../🔤️txt/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/📦️packages/🦀️rust","🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-contract":{"path":"../../📇️registry/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../🔤️txt/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-graph":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust"},"semio-framework-graph-layout-run":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-md":{"path":"../../../🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/💡️reasoning/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-flow-flow":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust"},"semio-framework-artifact-infinite-dag":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-md":{"path":"../../../🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🎬️sequence/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-graph":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-docx":{"path":"../../../🗄️stdio/🗿️artifacts/📜️docx/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-md":{"path":"../../../🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust"},"semio-s-artifact-stdio-pdf":{"path":"../../../🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"semio-s-artifact-trinity-jack":{"path":"../../../🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/✒️writer/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../../../🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-gif":{"path":"../../../🗄️stdio/🗿️artifacts/🎞️gif/📦️packages/🦀️rust"},"semio-s-artifact-stdio-html":{"path":"../../../🗄️stdio/🗿️artifacts/🌐️html/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-md":{"path":"../../../🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust"},"semio-s-artifact-stdio-mp4":{"path":"../../../🗄️stdio/🗿️artifacts/🎥️mp4/📦️packages/🦀️rust"},"semio-s-artifact-stdio-pdf":{"path":"../../../🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-pptx":{"path":"../../../🗄️stdio/🗿️artifacts/📽️pptx/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-svg":{"path":"../../../🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xml":{"path":"../../../🗄️stdio/🗿️artifacts/📰️xml/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🎞️animate/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-artifact-space-space":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust"},"semio-framework-os-infinite":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-space-space":{"path":"../🪐️space/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xlsx":{"path":"../../../🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../../../🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-artifact-space-collection":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/📦️packages/🦀️rust"},"semio-framework-artifact-space-space":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-infinite":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🪐️space/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-3d":{"path":"../../../../../🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust"},"semio-framework-actor":{"path":"../../../../../🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust"},"semio-framework-artifact-flow-flow":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust"},"semio-framework-artifact-infinite-dag":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust"},"semio-framework-artifact-playbook-playbook":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-mesh-engine":{"path":"../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust"},"semio-framework-os-flow":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust"},"semio-framework-os-infinite":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-trace":{"path":"../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"},"semio-framework-ui":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-styling":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../../../🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-gltf":{"path":"../../../🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-las":{"path":"../../../🗄️stdio/🗿️artifacts/☁️las/📦️packages/🦀️rust"},"semio-s-artifact-stdio-obj":{"path":"../../../🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust"},"semio-s-artifact-stdio-ply":{"path":"../../../🗄️stdio/🗿️artifacts/🧱️ply/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-step":{"path":"../../../🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust"},"semio-s-artifact-stdio-stl":{"path":"../../../🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-flow-flow":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust"},"semio-framework-artifact-infinite-dag":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust"},"semio-framework-artifact-playbook-playbook":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-flow":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust"},"semio-framework-os-infinite":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-ui":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-styling":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../../../🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dxf":{"path":"../../../🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-pdf":{"path":"../../../🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-svg":{"path":"../../../🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🌀️procedural/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xlsx":{"path":"../../../🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../../../🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🌿️vcs/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-os-mcp":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../../../🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dxf":{"path":"../../../🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-pdf":{"path":"../../../🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-svg":{"path":"../../../🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]},"tokio":{"version":"1"}}

### ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-gis-gismap":{"path":"../🗺️gismap/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../../../🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-gltf":{"path":"../../../🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-las":{"path":"../../../🗄️stdio/🗿️artifacts/☁️las/📦️packages/🦀️rust"},"semio-s-artifact-stdio-obj":{"path":"../../../🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust"},"semio-s-artifact-stdio-ply":{"path":"../../../🗄️stdio/🗿️artifacts/🧱️ply/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-stl":{"path":"../../../🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🌍️gis/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🀄️wfc

Members: ["⚙️engine/📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🗿️artifacts","🧩️extensions"]

Providers: {"semio-framework-dispatch-macros":{"path":"../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-geometry":{"path":"../../../🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust"},"semio-framework-graph":{"path":"../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-geometry":{"path":"../../../../../🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust"},"semio-framework-graph":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"semio-s-plugin-wfc-engine":{"path":"../../⚙️engine/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-geometry":{"path":"../../../../../🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust"},"semio-framework-graph":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-infinite":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-ui":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-ui-styling":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"semio-s-plugin-wfc-engine":{"path":"../../⚙️engine/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-plugin-wfc-engine":{"path":"../../⚙️engine/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-geometry":{"path":"../../../../../🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust"},"semio-framework-graph":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-flow":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust"},"semio-framework-os-infinite":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-ui":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-styling":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"semio-s-plugin-wfc-engine":{"path":"../../⚙️engine/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-geometry":{"path":"../../../../../🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust"},"semio-framework-graph":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-flow":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust"},"semio-framework-os-infinite":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-ui":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-styling":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"semio-s-plugin-wfc-engine":{"path":"../../⚙️engine/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🀄️wfc/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-md":{"path":"../../../🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {}

### ✏️s/🔌️plugins/📜️imperative/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-obj":{"path":"../../../🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-stl":{"path":"../../../🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../../../🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-s-artifact-sourcing-curation":{"path":"../../🗿️artifacts/🗂️curation/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-s-artifact-sourcing-curation":{"path":"../../🗿️artifacts/🗂️curation/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-s-artifact-sourcing-curation":{"path":"../../🗿️artifacts/🗂️curation/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🪵️sourcing/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust","🔮️oracles/📦️packages/🦀️rust"]

Providers: {"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-actor":{"path":"../../../../../🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../../../🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dxf":{"path":"../../../🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-pdf":{"path":"../../../🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-svg":{"path":"../../../🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔮️oracles/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🗒️note/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-playbook-playbook":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xlsx":{"path":"../../../🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../../../🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📋️forms/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-tsv":{"path":"../../../🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xlsx":{"path":"../../../🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../../../🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🏛️architect/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-bmp":{"path":"../../../🗄️stdio/🗿️artifacts/🪟️bmp/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../../../🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-gif":{"path":"../../../🗄️stdio/🗿️artifacts/🎞️gif/📦️packages/🦀️rust"},"semio-s-artifact-stdio-jpg":{"path":"../../../🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-pdf":{"path":"../../../🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-svg":{"path":"../../../🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-tiff":{"path":"../../../🗄️stdio/🗿️artifacts/🖼️tiff/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🎥️shooting/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust","🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust"]

Providers: {"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-number":{"path":"../../../../../🧰️framework/🔨️modules/🔢️number/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-md":{"path":"../../../🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/➗️mathematical/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../../../🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dxf":{"path":"../../../🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-pdf":{"path":"../../../🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-svg":{"path":"../../../🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📏️layout/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"js-sys":"0.3.83","wasm-bindgen":"0.2.106","pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-3d":{"path":"../../../../../🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-graph":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-trace":{"path":"../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../../../🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-gltf":{"path":"../../../🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-las":{"path":"../../../🗄️stdio/🗿️artifacts/☁️las/📦️packages/🦀️rust"},"semio-s-artifact-stdio-obj":{"path":"../../../🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust"},"semio-s-artifact-stdio-ply":{"path":"../../../🗄️stdio/🗿️artifacts/🧱️ply/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-stl":{"path":"../../../🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"js-sys":"0.3.83","wasm-bindgen":"0.2.106","pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-graph":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-infinite":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-trace":{"path":"../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-puzzle-3d":{"path":"../🧊️3d/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-obj":{"path":"../../../🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-stl":{"path":"../../../🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../../../🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"js-sys":"0.3.83","wasm-bindgen":"0.2.106","pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-geometry":{"path":"../../../../../🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust"},"semio-framework-graph":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust"},"semio-framework-hash":{"path":"../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-infinite":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-trace":{"path":"../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-puzzle-3d":{"path":"../🧊️3d/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../../../🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dxf":{"path":"../../../🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-pdf":{"path":"../../../🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-svg":{"path":"../../../🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🧩️puzzle/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-trace":{"path":"../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-fem-2d":{"path":"../◻️2d/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-md":{"path":"../../../🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust"},"semio-s-artifact-stdio-obj":{"path":"../../../🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-stl":{"path":"../../../🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust"]

Providers: {"pack":{"path":"../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust","package":"semio-framework-pack"},"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-async":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust"},"semio-framework-async-macros":{"path":"../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"},"semio-framework-dispatch-macros":{"path":"../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-os-kernel":{"path":"../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-schema":{"path":"../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-framework-ui-scene":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"},"semio-framework-value-derive":{"path":"../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-md":{"path":"../../../🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust"},"semio-s-artifact-stdio-obj":{"path":"../../../🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-stl":{"path":"../../../🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🏗️fem/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**","🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust","🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust"]

Providers: {"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-graph":{"path":"../../../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-trace":{"path":"../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"},"semio-s-artifact-stdio-deflate":{"path":"../../../🗄️stdio/🗿️artifacts/🗜️deflate/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-pdf":{"path":"../../../🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-svg":{"path":"../../../🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xml":{"path":"../../../🗄️stdio/🗿️artifacts/📰️xml/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust

Members: ["."]

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🖍️draw/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-playbook-playbook":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework-artifact-flow-flow":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust"},"semio-framework-artifact-playbook-playbook":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-plugin-flow-extension-brep":{"path":"../../../🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust","default-features":false},"semio-s-plugin-flow-extension-math":{"path":"../../../🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust","default-features":false},"semio-s-spatial-kernel-semio-session":{"path":"../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust"},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/📖️playbook/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-tool-machine":{"path":"../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-cad-cad":{"path":"../../../📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust"},"semio-s-artifact-stdio-dwg":{"path":"../../../🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust"},"semio-s-artifact-stdio-gltf":{"path":"../../../🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-las":{"path":"../../../🗄️stdio/🗿️artifacts/☁️las/📦️packages/🦀️rust"},"semio-s-artifact-stdio-obj":{"path":"../../../🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust"},"semio-s-artifact-stdio-ply":{"path":"../../../🗄️stdio/🗿️artifacts/🧱️ply/📦️packages/🦀️rust"},"semio-s-artifact-stdio-png":{"path":"../../../🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-stl":{"path":"../../../🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/💠️lowpoly/🏭️bridge

Members: []

Exclusions: []

Providers: {}

### ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model

Members: ["📦️packages/🦀️rust"]

Exclusions: ["**/🏅️standards/**","**/👽️guest/**","**/🔮️oracles/**","**/🏭️bridge/**"]

Providers: {"semio-framework":{"path":"../../../../../🧰️framework/📦️packages/🦀️rust"},"semio-framework-artifact-reference":{"path":"../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"},"semio-framework-job":{"path":"../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"},"semio-framework-plugin":{"path":"../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"},"semio-framework-tool-run":{"path":"../../../../../🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"},"semio-framework-ui-contract":{"path":"../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"},"semio-s-artifact-stdio-csv":{"path":"../../../🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust"},"semio-s-artifact-stdio-epw":{"path":"../../../🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🦀️rust"},"semio-s-artifact-stdio-json":{"path":"../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"},"semio-s-artifact-stdio-semio":{"path":"../../../🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"},"semio-s-artifact-stdio-txt":{"path":"../../../🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust"},"semio-s-artifact-stdio-xlsx":{"path":"../../../🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🦀️rust"},"semio-s-artifact-stdio-zip":{"path":"../../../🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust"},"serde":{"version":"1.0.228","features":["derive"]},"serde_json":{"version":"1.0.149","features":["raw_value"]}}

### ✏️s/🔌️plugins/🔋️energy/🏭️bridge

Members: []

Exclusions: []

Providers: {}

## Observed SHA-256 Bytes

- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/Cargo.toml`: `a05bd246da37b1747d45bb7a6a39bc573624b694e9bbc21811fc5be17db0bf7e`
- `✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/📦️packages/🦀️rust/Cargo.toml`: `322231c503f3a9931b820dc0c232e472a1376426e90adedeb4376c953667ca54`
- `✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust/Cargo.toml`: `262f4497570e9bbdf335a12ddf2e7a5a35c806358e8d60013101ea6f65303333`
- `✏️s/Cargo.toml`: `0e2aac4b1ab979bd95d6e0542276a2bb4f5c66fae5ddd365988359ef44efbd5d`
- `✏️s/🧑‍💻dev/🧩️composition/Cargo.toml`: `9e047fa9e2d23168cbc30b86b0396ceff5ef6fe9597980b2f6bdb39cb4f35f2b`
- `✏️s/🧑‍💻dev/🧩️composition/📦️packages/🦀️rust/Cargo.toml`: `df9298f3c5024e6f512b8f41df4accd232e5197a4cf780a02ebf982d0462fcda`
- `✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.toml`: `99401ea698802f4fb67220d17cd741c82ee45f6101a99d140d62272a590e3043`
- `✏️s/🧑‍💻dev/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml`: `08ad93f5fc5d4209f6e3aadbe4b2c452d14273831c0e067d57fb8607a869d170`
- `✏️s/🧑‍💻dev/🌊️flow/Cargo.toml`: `1d6cbdcc283b7f963a24c080a2bf027189b56deaa659f5333684192a3a448b53`
- `✏️s/🧑‍💻dev/🌊️flow/📦️packages/🦀️rust/Cargo.toml`: `0a9640ca62714a92b02d1e68fcbe33ea88cf68ee1cc2ec4344be89d5d910aaa8`
- `✏️s/🧑‍💻dev/📐️cad/Cargo.toml`: `1218030d7e286cf77351e5d3bf5e862ac7fe3d4c9f70bd210849db536a92b4aa`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/Cargo.toml`: `30f08a25008055dc18031b5e2f46b389f7d2fbbf61d92fda8ea5f7cc18634921`
- `✏️s/🧑‍💻dev/💡️services/Cargo.toml`: `d9d78fcdf7be72b9c9c48c0cbaa2659eb959cc3de82c218f7558f0c9f44a2c10`
- `✏️s/🧑‍💻dev/💡️services/📦️packages/🦀️rust/Cargo.toml`: `cc7afbcc3429458acfc51efd9dfcacc274ebb60d2b0f787943d1b3cd36be33e7`
- `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust/Cargo.toml`: `9ace69fb0072a5401c6c6bc15ba72272f958ad165b99b003c256e4c022903f58`
- `✏️s/🔌️plugins/🔱️trinity/Cargo.toml`: `24088d0d99c8ebbd019aafc7f301bad84d6fca46749fd3ce138896d038b328bc`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/Cargo.toml`: `e5d6efff781086961ed66c3e7ed80e594e9bd1aaa2fba1c249651a877de356b0`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🐚️shell/📦️packages/🦀️rust/Cargo.toml`: `5247f05c53959269539480cbccd32ca4710bd62154ab92a13f7492a410872f42`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/Cargo.toml`: `cba0eb2b5693212e7dcb224c89e3b6e49c26c6c7cf8a3f5cfa78714b65a62165`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/Cargo.toml`: `671164637198074d7d398b2a3222d1d2dded46275027126db71e01b211ad7a7c`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust/Cargo.toml`: `ea30d3ebff49d6f8adfd10073269b5732aec80e4d5c6e90bb85a8f9e1f49b417`
- `✏️s/🔌️plugins/🔱️trinity/🏭️bridge/Cargo.toml`: `6c3b58fbe2cb8fb230d19c398b6bf5c7817e3f7bf8690bce19fd2c153da3066b`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml`: `5a3f492a9123ec8910b1d7ed00f1aae8ab4477dcdc40d71e611210bae7fd9e9a`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/Cargo.toml`: `87989a0025c711bd0b43c9b93a06a1532a6f24b76679eb611e2154a27b0e284b`
- `✏️s/🔌️plugins/📸️remodel/🏭️bridge/Cargo.toml`: `fb896276cd70ee84dd20d37bb14c6d6c075cd130e40839831fab2fed1b35348e`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/Cargo.toml`: `3667e413d319af252f1a2642e33bd46e2dbe7a7cd8935bdead2e83c4ec26179d`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust/Cargo.toml`: `2f97c77758d4219b8af475c1550397f7cf0a9bf6279f6f140fa5e1712945cbb0`
- `✏️s/🔌️plugins/🖨️raster/🏭️bridge/Cargo.toml`: `6078d2e7a7665c71a2f5afafa4d892d1b556c571ce582fdf5e05d042d5ded769`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml`: `d6fe240ba346492d28f49685b3e9b3b74f10996ea6ac2c5cc4dd8421a846efea`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/Cargo.toml`: `91c6165cb92114d4d5b61352ce7fb1b4f12db7395fdf74fdf1370e548ed70c18`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/Cargo.toml`: `f4e88acf3eb9a1693793d68f27678d0bda2c725ab9515f074d12acaf882ecf3d`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/Cargo.toml`: `cd8019064ada7145df6f9806c8563ab17dce19fc108c1900358f433f10e7808e`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/📦️packages/🦀️rust/Cargo.toml`: `b87e411f34c602c6cd581fed545b8758e11a4bc28287a8e5e35f54bd6058310a`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust/Cargo.toml`: `2924d288da22d725bbfb083a59011c7b9f12ac9c9283ac94781a2aae49e62b29`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/📦️packages/🦀️rust/Cargo.toml`: `c60282f7e781479407361281d9a4f9a44fa1c9cbd5b229d29493cba4a9e1b804`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/📦️packages/🦀️rust/Cargo.toml`: `135f63306561193821e1962eeed57f59bcc40bb972f4df5cf0b8653a4a2aae8f`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/Cargo.toml`: `69e4018fa376d0be9853a0c0e552f2326f433d7d850e48f86f09fd2483a8456c`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust/Cargo.toml`: `cbaf2e9e59836fb3e218f0194eca8f0a9142cafdb2a80e598ded0bf642e2679e`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust/Cargo.toml`: `ebd6719314bc4aaf212ae2db6ee0c38b5b00dacfc0b0d5549307e9cdeb30e026`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/📦️packages/🦀️rust/Cargo.toml`: `660ebfa2ce46bdb82608cb732889bab9c17465339c77a1730648851b432ce870`
- `✏️s/🔌️plugins/🌊️flow/🏭️bridge/Cargo.toml`: `6ee3587f0a91c0be63bfcbc8dcb1af7cf857f3ae3fb2af53a2f09842ab734638`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml`: `b376861404e330c96e5b824205aedccebbc6f57728b544bce02e5405c26234c8`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/Cargo.toml`: `d64ad706ea5d94920589ce1f07560b70c8504223c9bb83dc27784b4dc1735342`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/Cargo.toml`: `f281286b3d24038ca94cf9eba667de3eb75579950e1146adce650468d6bc2dd7`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/📦️packages/🦀️rust/Cargo.toml`: `517446f1234c641c0f07fbe2a3f26bb1bf0d8466ec81dc18500e3f4ee8ca8834`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/Cargo.toml`: `f281286b3d24038ca94cf9eba667de3eb75579950e1146adce650468d6bc2dd7`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/📦️packages/🦀️rust/Cargo.toml`: `2fdd7baa75df8df1332ab4a859aa20e44c710b9bc0278bd4935a7066ac6b030a`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/Cargo.toml`: `f281286b3d24038ca94cf9eba667de3eb75579950e1146adce650468d6bc2dd7`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/📦️packages/🦀️rust/Cargo.toml`: `54c720a2d147b06857717b5dd71b776f7cb10fd458d536c92bfc57ad15fb36c7`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/Cargo.toml`: `f281286b3d24038ca94cf9eba667de3eb75579950e1146adce650468d6bc2dd7`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/📦️packages/🦀️rust/Cargo.toml`: `a18b8ceb694fa492727147d08c6db93a66e89908ca22983b5515c94d73a39a5b`
- `✏️s/🔌️plugins/🏭️process/🏭️bridge/Cargo.toml`: `a459080d03963b999f6562265b3fbdf6f4ca824dc5f135be53beccde9c091587`
- `✏️s/🔌️plugins/📕️norm/Cargo.toml`: `bdf462d386cc211292756e82c4daac5c682e079c83e73cbb368d6ff48e7a9bc2`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml`: `d4faa0551cd5ccc84d38a2bafb7ab0a128fab1b3ab4deb858efe9b5a8a30263a`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/Cargo.toml`: `20905b11364e4cdbb3d3e7ca800fa5d3e08b13e30123ce200d36af02b7f77f8e`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📦️packages/🦀️rust/Cargo.toml`: `a6d5b1b120f5c03aa943a8f7c9e5880a814db8aaaeb56bfc51c063b1d37c42f5`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/Cargo.toml`: `20905b11364e4cdbb3d3e7ca800fa5d3e08b13e30123ce200d36af02b7f77f8e`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/📦️packages/🦀️rust/Cargo.toml`: `1148227d4ced306cca09c30ba43506b5341b48b5ee2e59a558295f4f6b45000d`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/Cargo.toml`: `20905b11364e4cdbb3d3e7ca800fa5d3e08b13e30123ce200d36af02b7f77f8e`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🦀️rust/Cargo.toml`: `e993398134f41e55964ec22273de6dfb95d00f6eec038a777bf94d14f1d69e2f`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/Cargo.toml`: `20905b11364e4cdbb3d3e7ca800fa5d3e08b13e30123ce200d36af02b7f77f8e`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/📦️packages/🦀️rust/Cargo.toml`: `8c185e159f38bf387236a9d726e8c32a57e3637590a39db916c1fea74bc76816`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/Cargo.toml`: `20905b11364e4cdbb3d3e7ca800fa5d3e08b13e30123ce200d36af02b7f77f8e`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/📦️packages/🦀️rust/Cargo.toml`: `987e569c1d4c838890cef2b7b84bacdb9acef6f7d0ed60f2bf1b9df441a9820d`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/Cargo.toml`: `20905b11364e4cdbb3d3e7ca800fa5d3e08b13e30123ce200d36af02b7f77f8e`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📦️packages/🦀️rust/Cargo.toml`: `02e98d4006a425bb018d8606c627e7a4c97e5838b9b8208b13082f03807f9359`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/Cargo.toml`: `20905b11364e4cdbb3d3e7ca800fa5d3e08b13e30123ce200d36af02b7f77f8e`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/📦️packages/🦀️rust/Cargo.toml`: `6455b0b40787c425e50b0335d786f1fca1671ed271f93029034fa456291a18a2`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/Cargo.toml`: `20905b11364e4cdbb3d3e7ca800fa5d3e08b13e30123ce200d36af02b7f77f8e`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/📦️packages/🦀️rust/Cargo.toml`: `bd2ec27821aded2caec6acca813dc6b37797cbfc6556d6090dd6e3e6e8ac7654`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/Cargo.toml`: `fd6760912f7d4b580361c4bd2f1432ea62e6fa28b3b2a6a8b0104160a5d11005`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust/Cargo.toml`: `8ca2dfacf02a75d2bd2604a57606773d8d4f35865a38058cac7bc0c2232c8206`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/Cargo.toml`: `20905b11364e4cdbb3d3e7ca800fa5d3e08b13e30123ce200d36af02b7f77f8e`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🦀️rust/Cargo.toml`: `235f450d889a85bae14e4b36bd83f43941f95a91e019c789f5b892f449c61050`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/Cargo.toml`: `17f00a18d51140d83fbc92e45d2ae17e40251050b0aed61a82363b500a286592`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📦️packages/🦀️rust/Cargo.toml`: `fae6127d14ad30c5f9426cedbceeea9926bf8467ec5cbee6fedf227387b64c83`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/Cargo.toml`: `fd6760912f7d4b580361c4bd2f1432ea62e6fa28b3b2a6a8b0104160a5d11005`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/📦️packages/🦀️rust/Cargo.toml`: `b5b5fa9e799f418b1b0b704fe1645e358bb90c74859eb9c247a76fc7fdd63ab2`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/Cargo.toml`: `20905b11364e4cdbb3d3e7ca800fa5d3e08b13e30123ce200d36af02b7f77f8e`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/📦️packages/🦀️rust/Cargo.toml`: `83b5d412d95ca65df6520711cff47f8f1c934b87c8a86577d3c4da4b93acc94d`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/Cargo.toml`: `20905b11364e4cdbb3d3e7ca800fa5d3e08b13e30123ce200d36af02b7f77f8e`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🦀️rust/Cargo.toml`: `a07c75c6aec935c339ca43a4323df86be8d1cc400de92848b67dfb95266c8081`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/Cargo.toml`: `20905b11364e4cdbb3d3e7ca800fa5d3e08b13e30123ce200d36af02b7f77f8e`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/📦️packages/🦀️rust/Cargo.toml`: `55927fc3bed47e03bdd193418f20da3af17d392c4965488672caa04e205691fd`
- `✏️s/🔌️plugins/📕️norm/🏭️bridge/Cargo.toml`: `6720ac5918f1f59000ca2c90b9e4f1b924e77f30b4db2f2866e04fa74b57fff3`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml`: `dfd000c99a1efa3c8fe2b02986a990117ad58226d316bea9bd1de6314049c9c3`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/Cargo.toml`: `d3ec9f24a1d6ed6d617de6dd03f3f75530f61e8ccddc607d2aadd5bde301f99f`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/Cargo.toml`: `32ac022dc4bee03e28b895b1f401651ce1b03bb0287f099e1be9615f439e4f85`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🦀️rust/Cargo.toml`: `8019e6d802d926ec4d44ccea5d16cdaf0f8859707ad9e34486f57d03226f6bff`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🦀️rust/Cargo.toml`: `d93759683411408fee1acf79057888d96c590feca72a69b20ef8a16d7992952b`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🦀️rust/Cargo.toml`: `3cff9293c20c1a48a87df35d60d22433443b4ace6bd256ce48a7aeaadafb62e6`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🦀️rust/Cargo.toml`: `355413db9870bddc895625705ae0eda13c2e4f563ae2cf7813d1778f6873a131`
- `✏️s/🔌️plugins/📐️cad/🏭️bridge/Cargo.toml`: `618aaecadcb53de335753bd673aedfce02080093a0b84c479d959867beb2c988`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/Cargo.toml`: `5531a732b753b0d41a214f3468c2dc7b559ede5b36d1a60beb83d7b7a42ef058`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust/Cargo.toml`: `56f0138ec5affb3385bdd5bcdc542268cbca15043c8340a229d58e8f8372f416`
- `✏️s/🔌️plugins/🎪️demonstrator/🏭️bridge/Cargo.toml`: `53e1caeea01b229e7145a0483a5b3a77d07ea2f063684605f85c43fdb4baa51f`
- `✏️s/🔌️plugins/🧱️block/Cargo.toml`: `d94c78d06a6282a2b36a7cccaa6ec8243e0a223b08130e68211b3d44ca74ad9c`
- `✏️s/🔌️plugins/🧱️block/🗂️catalog/📦️packages/🦀️rust/Cargo.toml`: `4843b5cef1162ba3daf88519be2a0cd196a12f578a3f1b5e690d599f5cf35791`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/Cargo.toml`: `ec16dedc4bf3c0421d84ac2d03f324a71c0e341c99b10cfe795b435ec32d6ce3`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`: `ca4677e14e2f31dd37e878188291830fd1d5cb254b2d6a089b92f1cde752198c`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/Cargo.toml`: `d3a8a670a573cabf7dfd0a956cdb1ca1fe6d25c615562abde0a682961988df0e`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml`: `6b0eb75e5f68995a69ba41246a5e619466f2ab20eb9bb28f0839a89959f2d077`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/Cargo.toml`: `3ff2432451fb26a7dec26bf1c35146c9e97de3ed628d5179092a04796ba19412`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml`: `7a35a5f6163183eb08dd7d9edf5b289ef710fda269a9f1bc32a46b59108d991e`
- `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml`: `63579476a8a443650008d77cadb824ff75326897cf3292f32a2e4eafac8e2394`
- `✏️s/🔌️plugins/🧱️block/🏭️bridge/Cargo.toml`: `6cf66e5245aee87c395f9ebe16abc7d4a9a49bd9d81e3286ce4f033d9611aec6`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/Cargo.toml`: `76b1b070461b14542aaeff588d5c88d6735dd78b84ed6db5200b7fb2ca20fa89`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/Cargo.toml`: `a6cb5ff3b20247b9c5c76cac5411978b630e9168a7dc20854b6a4095ad7ffbad`
- `✏️s/🔌️plugins/🕸️dag/🏭️bridge/Cargo.toml`: `b03375424590735c2001344aa49064fc06353bb4b5bfacef2f2e0187debd49d4`
- `✏️s/🔌️plugins/🗄️stdio/Cargo.toml`: `7a81b39496c84b14344c964cda08f5b364699d7f707be816f58709ff204474b4`
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/📦️packages/🦀️rust/Cargo.toml`: `9c37ca6e54616d8f2aebf7c74557450a5be9f2da82dfb3a334810fa5526f7464`
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖼️raster/📦️packages/🦀️rust/Cargo.toml`: `08c7fd3cccbd53850440e613457ee885505e62cedae8b1229390abe18ce84621`
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🧊️mesh/📦️packages/🦀️rust/Cargo.toml`: `48153c480f4b2be5aede33175132e6fa4910dc7b83ce181c8cf58da264417474`
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🎒️archive/📦️packages/🦀️rust/Cargo.toml`: `863b2e50d77ecd00a50111d5fcc70c7b1c72b8104389ec558156ee7e7fe9903b`
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📊️tabular/📦️packages/🦀️rust/Cargo.toml`: `9b54692833bf74e52c74a05e760c3a91aae2663d7615ab9a4ce18299dfd89a2c`
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📃️document/📦️packages/🦀️rust/Cargo.toml`: `dafd086ee1f9b07f8465bb841f7a289476eb591d78215c97119a77749fefdb92`
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📰markup/📦️packages/🦀️rust/Cargo.toml`: `df62ce37082cda18c7e0330a7104be83e9f00966867b42a270c7a6faf0b2815f`
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔤️part21/📦️packages/🦀️rust/Cargo.toml`: `e7d33118aca2e757f791ca758d6876769b634745845d4b801b9c25e9164f92ac`
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔊️audio/📦️packages/🦀️rust/Cargo.toml`: `ff641accecf62dbb542051d2d7aedfd715c08447e92ef2e7110d7b15f2290b8b`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml`: `7dd67b3f523d333a1da6863a29828c40c7265be6c49469d9ce0312beb5d62602`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/Cargo.toml`: `442c4dc5c950ff95caa5e3dd343d352a1a183a4f5ec52a027ceaf53657b368fc`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `aa33528a917994e4420f1281ce055752d5dd5bcd1d18c23681f7f9b70f550d53`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/📦️packages/🦀️rust/Cargo.toml`: `a5ae0d7459382f23167da83dc41d79b37b241af4e1234b94fd63645c8464f01b`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/Cargo.toml`: `f4b55a2e8c0708ae82e9bce23a668f18bf3e751dfeffa92fa17ce807dbf5fcb5`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `6dc93c75dba57e2aef798ccfa56de1bfe98f91672ae10168032662bce8ef6f6e`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📦️packages/🦀️rust/Cargo.toml`: `7f7a7efdb1611ac24017be6c2e061aa4f74573392fd371080cf15715beb8688e`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/Cargo.toml`: `bbcccba02692fe1906d99cdacd607ceaccd0d2dce174f58998edc81831b0b433`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `b4ae80f9fb65453dbcb5b48b9f5d00f2f95a6ebb1f8aead2f61d5f00986e8db7`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust/Cargo.toml`: `459129b358556f696a4a58be370932029cb0f8a8fe5ec0fb4e5cc8d6b65fa4a9`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/Cargo.toml`: `5f4dc724db1de3716d7d52f252a0b980767f527384f3c2a8db8daf7a16c08935`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `a82a64e73d78f7ab3eea727ac0ec44c0142f3ac3ebcd119c969a59512edac63a`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `7a1061e03a6cbe621af516ceb4e279afc6b21b68c7cc5972d35ff0d343b0832c`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🦀️rust/Cargo.toml`: `6640d8b1fb0c7f1febc862f8ebe18e329db9c5882982d9008e77c8e7fe9a07ce`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/Cargo.toml`: `44ec87575db188f02a8ba49ab0531922b4daa6f72dadd86bad704dbc6b4a7f8a`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `9fc8c41e505204d75a343564e511f5535256f062674de04638c66bf7856f5fd2`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.toml`: `f80df4ac9fd96f7dec5669c00e175de4b5acb240b3f5ec93805687ec6519c86e`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/Cargo.toml`: `eef1a56436e1b0797dd9342cefa0a79cce598e35d332b23911dd2cb0907eda03`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/Cargo.toml`: `9daf4c392c7781d9f4427f87dd62cf573353fcce1222a2efb1a3d48cac4e29a1`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `71743fa14789e441d7449b27222726718753dbc99c047d69da8c16c6b4d59b8e`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🦀️rust/Cargo.toml`: `b9684ef442fa6be36469b2fe76744dc08e4c9ddfac3e1e7649c5f48a35dfd24c`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/Cargo.toml`: `98faf8fd6ad0b6f61077ef1d1f211d901f97c2d894df4d2db198c05e55fa32af`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `dd00eee33c8af4e7e7ec9b72cc18c5879007caa80ca199dda14a87c8ebf6e688`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🦀️rust/Cargo.toml`: `ef9ff01cdad278eb2fdc967d866de444b70cdb8c4e2f864b444f89274a803bf6`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/Cargo.toml`: `d420889a7d58f5fdb6fcd7a3225eacf5897829501938f4fb2c61a1d819a82abc`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `f1489a996e9ac8c6a08c6d7f93ba64a74e7203ae59e56e7aaea163aa8838edf1`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🦀️rust/Cargo.toml`: `41d9a836de04f54405471faca43b76ab580e3d0edd30dccaa4ded3e757f36929`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/Cargo.toml`: `8d232ea25e990b1cb4ef043578db027a7f51fe79641fe491fc21b6c5d4259e78`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `a3b7fbc5ea4e329c07befd8848e40d00dd6dafb0ad64d7ad8d9f7288b6f5ba50`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `6cd6bb733c94bf59fa91f0970e9b00c5cc5fbcfb458ad918df9ac97aef5f4771`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/📦️packages/🦀️rust/Cargo.toml`: `6c5528bc58790955714f9840731997d41ce4a9428db1f3f1d8db2692688d8d1f`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/Cargo.toml`: `f4b55a2e8c0708ae82e9bce23a668f18bf3e751dfeffa92fa17ce807dbf5fcb5`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `e610a281b622f124526b37d83e658876431d520f85b9835379edbe6e1420c60a`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/📦️packages/🦀️rust/Cargo.toml`: `7db2867b7e7c5d575a0187cf82d3db361a533e2ea5e8d591b0c1745810bd45aa`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/Cargo.toml`: `e967d5ed466ec474fc3b3fac9aa9984882d1cea90a23508b97012e4e9a1f3a8a`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `872bf24b91f341133861cff1435cb9f0d37c702aac1ac7c1b335e89dee54c05a`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `66d61751a5a5fe1c7baf548bc06673b8d2c13131f31fbd7c23c769eb264d1f99`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust/Cargo.toml`: `73f128143be3b971322154a41fcf58a7e4ab1326fab117e6fb08fb9a09bb3a49`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/Cargo.toml`: `bbdee44a935e6a1e69ce6e31aea3235e2b486ca12321fa4bf4d582678ca289ab`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `eb47d0de076d72c0c239075f43d0fe09bdd001d6f6d273482898e6dc817bb7ca`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `1ef19237ab962c28ed856b0cda06704ab066e16b2898d18891b858ded2031ef4`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🦀️rust/Cargo.toml`: `0c2a660a5eda92670613435804d2692d705b3a10f66a03108b53cd0783e50a0f`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/Cargo.toml`: `f8bc29a6737a660a7440f5c882f973e82cb118492e1c22daf8960bdcc897da23`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `867ae82e7e1d8abba657d36c33d777d8f7dd39d197fca30fd9e867176eac3e23`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml`: `1cb93f9ef679c84305319bb8608e82c46eb68d598d95e17c00197afb5b18610f`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml`: `6518b2fa86568a9f21aa3c2fc3867519d431ba046d10c0dd9521cf859934a18d`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `2bc77ccfd5815618d00ef140295bed3420fa7c437f2f78db517c4fbead7ccae4`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml`: `3cb46f95a3a9d1c2c2ee29202ee5326d9fc30bfcbabf86e08ebeda26dbbace36`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml`: `ccb47c8706585d8f9edc7c7c91fee016db1d51e03177e41d09f39b0fb10f48ec`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml`: `4dd449494b8cea2a43d480240b28ab8fbf68813831c00a36a8c0dc44d1cb998d`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml`: `e44c90cc05b0ac277e363da774f1a6c5dfd9b6e6fa130d5553751b7b3b7535b7`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🏭️bridge/Cargo.toml`: `6980f2910c44abfca7ca94c17c93f6492a6449b274732decbbade0d0ec65005c`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml`: `09ae5734c5eb2b7ec80351adf82b8438f0c040600d54e73ca9c96ae92276d65a`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust/Cargo.toml`: `769765efc5150a7447397417fefe6a052eac233a206cbe8e51068a338fdcd60e`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/Cargo.toml`: `4e3151500a4cbbad83efc881821f65d99ac2539c8ab664a88c51c7b677871970`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `58680f5329305ec231ef3a88f09f520b08066f7b26c0d73828c826cb496c5861`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/📦️packages/🦀️rust/Cargo.toml`: `747bebb0d34c6a71b9594bfc3f6db49a4d0c69fede27c012be356be8c7764626`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/Cargo.toml`: `04e15175d1b49e516cb9f3f80351b48ad4a04caf39430643af84b3949834fe19`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `a39e8c8cb9ab359b57f67e18b7ac9ff717ecb6836be555d1a12ace256268380f`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📦️packages/🦀️rust/Cargo.toml`: `83d1fa26d509d4599dd8052793fbe5ecb16c409e3ccd04507f160ca4331dcb07`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/Cargo.toml`: `f6bc686f95733d39e02d04c34ab1c96ddf81fe163471a368e512e5cb368ae3c3`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `30a4cdbb3e684f01061005035cc36dfdc169f19f6d8cb0d00a4821b07e60aef6`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust/Cargo.toml`: `4f1d8ca366a4edfb04f26d1345fb1bcd73aad1bf0f1789e64c360c640abe353d`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/Cargo.toml`: `1a9cc388133d2686e1b13bb03c9d24b91b5c7d3461e46600cffcdf29c6fcf245`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `f3ba4d4a6db4abaac03a72307b1f783f5e5d6854f596a7c38c011eea3fa0b5a8`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/Cargo.toml`: `d2a60243af1f64e5180b4173e8aca05d5d3d62e2d3b20020b2f2b48e186c9d04`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/Cargo.toml`: `dd961612e5e47c0311046920d9960febe77e8e17ac2b10ddceac2fbde829df79`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `89d34a5224f5a2419ad7ea9ecf21c4c3cb31a5d9be9018f200806f6f22937108`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/📦️packages/🦀️rust/Cargo.toml`: `31d5f63c32db9843977a4aa56afcaaca5b51b9d19e05fba650bd216cbb826d0b`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/Cargo.toml`: `3ae7c46b35cca91a47d53f6194beb7f078a188eb46ad5a712434b61cce60f4bf`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `773d0651f91e6e03b46a1915474c5511954be5434e19c28d154e2c8e70d96bed`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `4813b9fa58b4115f2a7419006c751d079f1720ee4a2bf3ac6dd473149ae595df`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/📦️packages/🦀️rust/Cargo.toml`: `99eb9d11df834691b2005f8575962da38c6485807c64d03229dcb8643cc94cce`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/Cargo.toml`: `f2c5113ec7ba729d7a65c5ae788fb1486ce11a23e4c8b43255ab149219a0eb4b`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `16424652aea53f9294568b4b3394c54c838746a2a7a73688196fa990dc280d62`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust/Cargo.toml`: `456283935a9dc61ecad12d492f686b1ce854665ec71b0ca28373e8b96deb178c`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/Cargo.toml`: `23483d2eaffad7e59fa55772ecef70d6a63f6ea6e09a6b002502e1a292b2ef82`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `8d33d309ff5ec94513c36d1c76fbb584913c5697fe6995db01592f16167349fc`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml`: `8a2eb68325be0e42fae59ad775cbbca8b7375f7653ea19a2e034e0d9e6d6bc62`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `4b2fb2cae03593174ec21bb2bcbe57984f095385090917517098e05b3547f268`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/📦️packages/🦀️rust/Cargo.toml`: `be85545b39015b973ad7516603bc05478da3b42bb7daa3a31ca8f6f19b005524`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/Cargo.toml`: `98a952623ecc8be1e61e05dc48c02907a038de3ebde4a6be0a054f5cccc4de73`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `c5aad2efd6577ac39099e6c3f17a7cbdadeec21e1fe1d4a79e86db5f660dce18`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/Cargo.toml`: `bb410aa6c39594940fecc5541a862108849ce82ae8b179ac73cd4fb181188317`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/Cargo.toml`: `386d5a5ba539803b20c60118aad06f60967b5f2ba492252dea2340aec013a2b3`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `d1e7dacb733246279a2b04276f85d22347a7c0cf193d8d02069ce5d23e433d1a`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/📦️packages/🦀️rust/Cargo.toml`: `2132102c21c8de6ac19992bece72d2eecc0905f648951c15a4ec52a9566e09b8`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/Cargo.toml`: `921eb6adc20baa69e18bd84aa5e673b36b125f8c2a598dd543a984f3e1875b82`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `a418fab529a2811d4fc5748af60ae3453ac7dd0445754f69aa15514113afbd23`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `2bf4896c224cdc24af7bc7276ce6e7804dded045f692bec6dfdaa034937f7aa2`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust/Cargo.toml`: `1a9fc8d5321e30e604501042f111b945992a71093cee3730e53840dbe65646a5`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/Cargo.toml`: `ed954c80aa3ed185eaab4053845f265ab69110cdb438d32cf3b3846d0ed3a223`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `265f5a42888d88ffddc3dd04d68c373c541524ec07c4fed8d37d98540895ce69`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust/Cargo.toml`: `a0d0e00848b16e2e78702e32a2511e91d231253ad331ef4bc79e15f02a0b8664`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/Cargo.toml`: `711328000fe0c08ce9013f40415df53dd6647a2d9294d1d22f8c6e968b96c596`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `53f5d2cd11cef55aa3a84acbadf341f820acab1d6fdbb1f18a1ece5bcbb1c3c4`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml`: `3c48b5c7e72d44a9ecf592bf60244d289192163d4a1eecd7eea7a407e3f1ec07`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust/Cargo.toml`: `36f11871b157037b79dc9503eb6e12b8b6834c6b56f83f7873132d3d02eefa3d`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/Cargo.toml`: `1bbaac5e8cf686ab480378251c2e7c120d29d7bf8c14b5c03d6a901946dfb10b`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `77ca53a47b896fc4323acb6b4226d36551f6a8092164ba62377d208e3ae47333`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🦀️rust/Cargo.toml`: `197c5689eb7e1876d17b070fc127f0149a6d89faced9aeea076f1cb8bfbb3791`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/Cargo.toml`: `ae2aace9092ef4e9dc362d40bb5d7eae314009492d0e6ad7df4e0e6676cbe015`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `db558dab4dc768a443c315fd352da436e39e22decfdf2bfb440219dfd7f5f5a6`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/📦️packages/🦀️rust/Cargo.toml`: `31b2ab5e9c9b43e84c790efc5f491010bae0fa5fa79c282b4bc28e21bdb1f690`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/Cargo.toml`: `66fe9db5477ac1d81735876028e03ec5fe901da0f9288a59b5940bad2ba28b2e`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `0856d04a2e1380ab3b4275e041026bd61dc47ec72d70ed4c672f6392ab36743c`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/📦️packages/🦀️rust/Cargo.toml`: `b52d4feb8df5866141a6d3b6c7352e44ea0c8c75e3f2add274a7014978102aaa`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/Cargo.toml`: `cdc4a6e91d927bc364b4677018ee3fddc0eed05f9309e030f45e0ab4090d3283`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `6b8bbfc93a12da7671d04994b1b0c734ed04a0f1ba77c2422117866497ffb149`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `c2b8a4e683f3632eee14154a159c80fda96fb9123f5670e38914c9a8e8e4bdad`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `1ffb2d56fd3e6f9f24caef2a91ec38a0497b679979246e60cd69ede0f77d80e5`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `dc0589c103ade831a6dc849b179556dac5f38362751976df283305d8a40d2544`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.toml`: `04a69a92299a6b638b2aeab38546a664bf697a0fea6ada3ff2b62b85bbde0911`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `43c828344c686da93431ace16a8756d2079910b76bbd6f9cacfa2eb12ec4cca6`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `b75028f6e7ff3b42557f476ceeaf9c65e00fa7970a45d2218aa498034c24168e`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `1bef5dbdc9ce30d03d5b39c341a261ed306109408b798a5c47612597a139e89d`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `7ed2ec0c6ad837a91e2a5a4fd998dfb5f651c222b07291cde7b8320110a58df8`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `bfebe4195b85fc806e1862ee2a46f7d3e1363ee4b261a3ee574d9ece518a8298`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `0ba4819ec5c71942cf5b47d162940cea91b4d09b68fbc6a303dbcc93aefa92dd`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `ca5751f9e667fe5be06465b97dd311ef5f530321a2dc02c81c7e9f23ccc24e2c`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/Cargo.toml`: `d4b03d81a28e5ead8286e43da9dc1c21dda69aa3cacda1d0a644d1757078da4f`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/Cargo.toml`: `c077fd0107ae6aa4e9ebeedb083a8c96bf8ae63b114e525af79ebc4fb19c9db4`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `7159fa7585cdd07053a33e2f602f3534e6d33ec5d97384b9c4acf67d359c9990`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.toml`: `4a8991c06d92a0a4810919627b2965d22433a074df13240f7b041096b7ebd46a`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📦️packages/🦀️rust/Cargo.toml`: `908ded6bb656481f8d8bc86b12e113fa22dbef00f275e65449ebf5796675046e`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/Cargo.toml`: `370f1b51da9cc7b82e4be3a9f30f75280e448f644d5ba80b168b1fb0b51366fc`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `40c4bae00af694dd1d0828f5c148d44b5f79ac569d248d9f6201a644ceb331c3`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml`: `d491daa86c7e376da1292bb3368c33ee43a4bf40da034d0ddea2de20ec643633`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.toml`: `5f5dcb2536b65fb5bb7f30e9589e7dde60bc044297f2bd2315e2d7b53841f3e7`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml`: `123cd279719803e6d332bd36055bea03aeecf4b250302cd90bb1d65388059e99`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📦️packages/🦀️rust/Cargo.toml`: `2360ac01b3a58631913b0688bc58cee64b5c30034a8ea0b0da9e49557979c3fb`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/Cargo.toml`: `5e12daa09531b9bedcab545d5705e3b3c6c4e8ada7d056e7f79d7917c53340d3`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `3828b92381f43467ee925e63dae631b583e942b6291c7113650507ae2fffb452`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml`: `15fa0639e56d94b5bd278748b4c10140e6f95ba480315a95250dd1325f485264`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust/Cargo.toml`: `607140eca64090d1926b99f22c7d14d20fba837719e07eba75051cde2db9c54f`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/Cargo.toml`: `b7414e045a322005c972abed7690d463f72279f854c265cc19c72ec03ea68969`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `5b47a30f7ec464a84d99e23f7e55a3c758c0de58d8ea4150ae4c6a5c37468690`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust/Cargo.toml`: `18ea863167d36bc06c2540229ea788c955b23ff94a75451667b33d2549333ece`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/Cargo.toml`: `b2cbb43a490dcd29885162e2d0f6490b3d60cec58a06f63ad7c861d433b92b83`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `2f88105a2f6262215d1260f0c68489aaeb1dfd9671c9956dd8f084c4f30e393d`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🦀️rust/Cargo.toml`: `b785a96c9efa624b0d3b10f6da48ec0626199259c14af0badd8b4c7117f7c152`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/Cargo.toml`: `4f6ce06b99781496a0bb43d1803c08cf5ba8101075f408990c0bf3045eaa3302`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `9fa90e1f7424e693c708cecd7475646957a6f882b567352036536015e79c1921`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/📦️packages/🦀️rust/Cargo.toml`: `e2448402345d818b451e2f0bea2272545ad5fa9dd26c85c8c7a04bc795ab5256`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.toml`: `d81d2dd6f9b4f1a196d3db8e3ac356786678230e5da7a5843098d948885450bd`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust/Cargo.toml`: `8704eb538f02177fa496eb1dcf67949c7e412e4fe9dfecb79801fbbc06159e83`
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml`: `a9f4392a02d91167077697e42c327921b9f07bc7f23acc8c9e17c172fcf60072`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/Cargo.toml`: `24e8729b08150f6f2acd00d042c9ceba99fbef77f087d31ca2daa61784f3b887`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/Cargo.toml`: `169b1066d1512800e53532950c8b399bdc03ada08da49a815e31337b48a491df`
- `✏️s/🔌️plugins/💡️reasoning/🏭️bridge/Cargo.toml`: `6b147ed02d9a11d2ed553763d5c29dd871f6067b110460bba6f26d0c68fb357a`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/Cargo.toml`: `50833ef78cbe14ae0255eb2de461c306c7a4c9ff0199c800054706c341c60f55`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust/Cargo.toml`: `21359ea8eaafb01de171895331f46b8be6496aaec1098f1ab6b3bc953e0627bf`
- `✏️s/🔌️plugins/🎬️sequence/🏭️bridge/Cargo.toml`: `477ed7dfa768f660257c19f88a72fadb485fea8593e505d8f9ea3a281d41e85d`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/Cargo.toml`: `e9ac6cabb52bf9d3c0f04bf4dfe7525347d23532dc7a4040fbf1a3409f597ca1`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🦀️rust/Cargo.toml`: `fac10f2e8277d0a8f8c4e300846e664d0e61f12cf2ee5c3378a3ce6a86822efa`
- `✏️s/🔌️plugins/✒️writer/🏭️bridge/Cargo.toml`: `28c996314d3fd18572913eb476e9790aed27bc3fe5caaebad61147f8f58a11da`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/Cargo.toml`: `cf3f5cea354f8a7a64999c20ff9e27f00ce59ca3548820a905f894855dbe8c40`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust/Cargo.toml`: `1ebe733ad952d8f8b058f46f00ffe6be6a9605367de2795a8809707808d87f29`
- `✏️s/🔌️plugins/🎞️animate/🏭️bridge/Cargo.toml`: `b652d584b72e80061936ada5b67532df86944e8deb55c548fe689393218abca7`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/Cargo.toml`: `a64416bc55272cc4f39621aa2926a0ab80d4a7a809e86d6845e7979c4573f634`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🦀️rust/Cargo.toml`: `6e4338ec3f3efe2fc9f6b51b282767137d18352889bbf6b93abe8073edc30950`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/Cargo.toml`: `397ef599ec43e77e1261fad1bb124fd47d326eea0e8ed4762b5a3a70addb8462`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/Cargo.toml`: `8d451383b5748f1dba66bb3e4e6cfb0656a1daf4ef671086807cc2518b7921aa`
- `✏️s/🔌️plugins/🪐️space/🏭️bridge/Cargo.toml`: `069af07e8cbfba0a3cbead92936a037d4644f487f9484eebcd4c2dcae5ba3425`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml`: `a8e63731ddafed19e2dc7c4e5f5050e3943e3fad8a42f6ec4e82ac95905090e2`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml`: `405c2e4217fb12f2fb266552a47f121f9c860f6e39623a07d1e079f274e29985`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.toml`: `bdff88c50a6c05274d1c35060ddc531294de2b6e38c9e7a2c6555a6bab576a22`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/Cargo.toml`: `9ea8467c662106555a0220b0b94469a6cf38465e6a3cfb9798e947716990635b`
- `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/Cargo.toml`: `6689abb594f12dd97fcba0dedc1649da40908b004c37e52d6cbaa11b0b38325a`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/Cargo.toml`: `0dfb7b2aa90d01afb9979c349eae4564772df2c6123f8671802b1a0e3019214f`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📦️packages/🦀️rust/Cargo.toml`: `963d43b719f3cc2806c61fc62c29bcb28d235c70b49e3453b8dd250478af001f`
- `✏️s/🔌️plugins/🌿️vcs/🏭️bridge/Cargo.toml`: `6cc070c4ee5adc9af9f069fea0bd1a052990d7599f9fb48a21e97f5604a04f2c`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml`: `0983e24a921725509e6c53c8c14f82999969064fe7b51098d9aea95815fda832`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/Cargo.toml`: `5d0429b8e2bb3c5c991ca1846d213caaea1ced8deba6ac3864c556c09070ed79`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/Cargo.toml`: `d1fd8f26bb449eaa3e8d68f4fa6db0c7f680651575e53c173befb36829cbd3ea`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust/Cargo.toml`: `6a999157334d812c31f6609ee2a4a3e7f4b535594030737a65a1c465b451bee8`
- `✏️s/🔌️plugins/🌍️gis/🏭️bridge/Cargo.toml`: `7c422bd9935ed51baa20b71cc5bb5bd48535ac683440147565f57af0b12c60e9`
- `✏️s/🔌️plugins/🀄️wfc/Cargo.toml`: `8017c76d6f32f2d945fd0a61471465d4098e3a899dc21d32650b168df2ffcb1d`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/Cargo.toml`: `734f78f6cb61161bc5cf691b53abea7592c3384914801faa0f778afe255bb439`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`: `f2a079a5b0a122e318b4b2b9540cdbc35d9a3e0ace9ba86c27613d318eb4f6c6`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/Cargo.toml`: `a2ee1c25642bd5041bf643fa3fc3406a11933406ccf02a6d0473c57fb778aeab`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/📦️packages/🦀️rust/Cargo.toml`: `771036733c42c2253ae3f37f3a7b48504497179dd7bffd8832dd0eff9e389f5b`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/Cargo.toml`: `b1950351c780c80ec7266f93028bd16919dd93dfac7dded907fd6fa900890d23`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/📦️packages/🦀️rust/Cargo.toml`: `9b11cf4b6bc44b8e6b3b689ed2ca57787c0d5ab7d0fc6ea9a3b9ede1b6b72554`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/Cargo.toml`: `bc4e0d0e3c80b69cdc808ee814711af57934790eaa0b7df696df32757c4618fa`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml`: `32da548b88c2180a0668c0374ad2cb1e669f4bccea7260bc2907d23e9a0569f0`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/Cargo.toml`: `7ab55c5898ac035f7669db1f70ff1270642f987370a2bb7510da1cc44ac34fa8`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/📦️packages/🦀️rust/Cargo.toml`: `0257d1cebf9a0bf79b50f34d2c41619de09534da9e6d17d917e736aee9b7ac04`
- `✏️s/🔌️plugins/🀄️wfc/⚙️engine/📦️packages/🦀️rust/Cargo.toml`: `627fcdc1c5e026ddb8c5c19defec3191dc71c6313d2df72dfe183ce19abed815`
- `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/Cargo.toml`: `e090bf87aef7a1e354d9daf2cfa9a3313aa7cd2b4b35906e6f6e5d60bbf6a9d7`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/Cargo.toml`: `66f8a8c1f6a9979bbed3d1e8408c000c189d03d118f6ef6e8cabd952437b9801`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/📦️packages/🦀️rust/Cargo.toml`: `a746d2877bf2f1fb969a65a7ca893e7a5fcbf6daac248402c8bed7f5fc681962`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/📦️packages/🦀️rust/Cargo.toml`: `649b956eba62bb7b4b7deebcf57b3b1d44f335c9f72afb304eee66c87c44cc1b`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/📦️packages/🦀️rust/Cargo.toml`: `ea85476ed50ebeb094f89bac0ad72fe3e692558bde49050e77002ab4ba7b6f75`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/📦️packages/🦀️rust/Cargo.toml`: `773a623f176148bb2feba21cd7462207887a866a156ebeb7cdf74483cd6dbed1`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/📦️packages/🦀️rust/Cargo.toml`: `dcedcf84b3b35e8cc83e8f01982f6af5506d2526443cc41062bc93e02585b31d`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/Cargo.toml`: `2846ed3cc032e033317bd24e0da494f99705cca2417e14feb744a67ad1bb32dd`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/📦️packages/🦀️rust/Cargo.toml`: `ddf1a7810c57a4aefb306d7fefdedddcd5a92a744ac216fe33775b3dc4ae8a94`
- `✏️s/🔌️plugins/📜️imperative/🏭️bridge/Cargo.toml`: `e64439ae6343ea84d5efb9d53e79c16804d1638d0563837949ea1fc2b349a844`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/Cargo.toml`: `73b5de72e9928abcd94eac07ebedf2f39fdf8c5eda16427f8e61dd0be408ecc2`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust/Cargo.toml`: `4c298a8bad7ea2a3822fd066e44222d8cc62019af3c64cb07ed9ac5f44aff766`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/Cargo.toml`: `ee5523080fd461a9fe5fafb5755006987a8813cb5f645c3a436f4babee17631d`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust/Cargo.toml`: `0f240b605793729f26da32a1145cb849e57075159dd13962494b634cb61a2a81`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/Cargo.toml`: `ee5523080fd461a9fe5fafb5755006987a8813cb5f645c3a436f4babee17631d`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust/Cargo.toml`: `91489d25832c59e16201972bc36bd0fb8dc86ebc8788e01e9b7c26073fdcd609`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/Cargo.toml`: `ee5523080fd461a9fe5fafb5755006987a8813cb5f645c3a436f4babee17631d`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust/Cargo.toml`: `cba9a1e038552a38a43f607c7ff09f15849539f17e60dcaac1187eec9ad6ba9d`
- `✏️s/🔌️plugins/🪵️sourcing/🏭️bridge/Cargo.toml`: `ca7d3b858498dc40e6a153366cfbff5ab86811a2d3f4988fa5667ea334db9c61`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/Cargo.toml`: `2d99722005250d55297fc10364bcfd144055542aa2124e48bd8414dfd056e25a`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`: `03330a50f1db090afc66347a403a4ce37ad9ef923abef55b9c407e105f3b442c`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml`: `80434db1b0f2c89e05ebd0d6717571a70b1ca73985b8f376f4dc80d5b9d2b9e8`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/Cargo.toml`: `4bfb984e92908ff28d41a6409c9403d626a749a13d02544c14be340fe43e9c90`
- `✏️s/🔌️plugins/🗒️note/🏭️bridge/Cargo.toml`: `0617ecdff80b8cede11c0a59132d3529f4645f598844297a65773574d60f2ea7`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/Cargo.toml`: `c6e489b9396685944213eea3c395feac3a1d66cd1bb700210d2e3e42ef38702c`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/Cargo.toml`: `adb9c3e01b5e35cd178666e6acf061b82390fd72be988793ac69b78d4eedc75a`
- `✏️s/🔌️plugins/📋️forms/🏭️bridge/Cargo.toml`: `60ea1c95739e704bf23b4487a394fe58bc476a1a7f247bfd6da64c59e1904838`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/Cargo.toml`: `c26f7d5ea6d28dbbcf8f25c33e0c8b716efbb27407d75c29fe17a3d1e2e47431`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/📦️packages/🦀️rust/Cargo.toml`: `d040bc71cb3fa5bde2f2e885e5639ada0d0ee4d73c81ae80b6ef972f8adb44c0`
- `✏️s/🔌️plugins/🏛️architect/🏭️bridge/Cargo.toml`: `80bdd0074bc2dbdae12e490e9f94ea69fc688d148f7e3198ad8c302b044ffb79`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/Cargo.toml`: `08ed76737ff85d8c4de81d252dd5484c7331a2d01cdac8b7f077a8854198b01d`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust/Cargo.toml`: `9a88ab3738cfbffb2091213d66d562a90f98960e1b6e9ecdc1265e55d8c538bf`
- `✏️s/🔌️plugins/🎥️shooting/🏭️bridge/Cargo.toml`: `6eb496b9015ed9b27d11ac361dcdaf2f38e23e6cfdf6f715822022ff0cddde85`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/Cargo.toml`: `256ec09804b6db4e0c2415a0d3d3d7abdee81a477bcd22c11fe34775a806a871`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml`: `5c871396bb56d3e84d223db72b99b8ac66cebb3b9f13051d0f12871b67efe4df`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml`: `961cd989c710fa6eb2803bd6970e43b4086c9f25aec462c0fec0152086bf6ae6`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust/Cargo.toml`: `2b013a3e4bfaae23077460911d61c9b53559b0cfaa71b4a577ff208912bd91c6`
- `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/Cargo.toml`: `184d0c794132b4a2efcbf21993ab9caeefbb244847bfc5699695f14f92a6f2d5`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml`: `78a95d112ccbde54e42df188ed2863649109a4ad39321d3269b1f1e03c35ac75`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/Cargo.toml`: `89828bfaab5fe49a8446cec2a6d40186bd032b8d8c883fa1a7b29a4c25dda8ba`
- `✏️s/🔌️plugins/📏️layout/🏭️bridge/Cargo.toml`: `462ee9d352856ccded59ca70a58796f9fa189f31cd2f18907d771ad6d8ca3cb8`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/Cargo.toml`: `88d77bf2565d8116157ca2fbe30443a7b05a92f4a23620c3bf7e0d5a14f37687`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`: `3f4a1d339aee1f1d5e7d5b14c2e19ccc2af815ca0f98ce37cd0ece6651be5553`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/Cargo.toml`: `ef650fe8bfabb49302193073a60351278e5041e8b029efe0f6c4e4113e6951b7`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml`: `2312ec3a2e1ec22e0c6694f4274ddb3cab3f2af7a781905ba69beb68eebb8ea3`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/Cargo.toml`: `94f8e7d4c019affaa4a5d22a933ccdf873599eb6c50db715bd38f6c636d32375`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml`: `37512fa099b1cd9d4565af0e7b78762e25dbb5928a9896022633488a147d84b0`
- `✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/Cargo.toml`: `16cc9de2788f0336a0c16f341a2635483766033e6fe2e78900ec943e2b72d59c`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml`: `433cba80b6227ce9dd415a7849b91596d3f8e1e6a024c560869cce40b1f4f3bc`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml`: `8f779556465a07960a2629f515531651847ecb72be2e9a5cbcf3539473abe8fb`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`: `15dc90f48e0c276b12fd563957bd1500ff5b85d3bbafad9fef717333d71974a4`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/Cargo.toml`: `20a174c401665a41bf5d5c1a14c6d506796ef23c8c1e25d5bc4b5a8620b239e1`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml`: `f1ff49cd2f785b9ed294d7e66a738ede9515b717e40c592eb4df59fef5600221`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml`: `9b2751963fcffb6d13ce7de8bde6524479beb25107b3015382782c0f09ce8d12`
- `✏️s/🔌️plugins/🏗️fem/🏭️bridge/Cargo.toml`: `28d13fb1bb19423479cb2b5e3cff89431215ab2a949928f1be22f7cc60ac5de7`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.toml`: `763582f8ff02e15cc06f66032cf26e76d48b691fe57aee337667d04f79adfc62`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml`: `ec77b534b1ece02ac396c311a8425be24cc6c9b5ac521023054c1d6a3a61fa1c`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml`: `f52a8056bf5c4f48b332bb973e8976beee8da70088fe1fe2b2f50ad59c4b8322`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/Cargo.toml`: `a5b0d4179e7c9ce884a9ca9393c7bf1d2b1d4f3d3c7c6af07c4525fd8297f67d`
- `✏️s/🔌️plugins/🖍️draw/🏭️bridge/Cargo.toml`: `0052525887131290443cdc56382cf3a1e672309b86a0d3b0778c963969c59e02`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/Cargo.toml`: `6ef4ad3c9fa03c7906803ac235beb3da4c1631afeaa5efd7f8038671cae56407`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/Cargo.toml`: `4d07dcd62ca1256ffcf9b0976a87238fd685fdfd238ef018faf591ad9276ad96`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/Cargo.toml`: `e5efef0c626d5446ac74a164cd8b6607d185b716b35f7e123603c6e8d7f14733`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`: `23164793397a9691aff02fa02efbecdcc37739d6744497f7e9b483bb3b287a2f`
- `✏️s/🔌️plugins/📖️playbook/🏭️bridge/Cargo.toml`: `0c7c1bee2ca18ebf43e7ef1c6a43ad458fdcbdc6b35ef74bec35a49bdc6ec856`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/Cargo.toml`: `ca2cacd421ae2106f12a356c484552448fa84155c7c28cdbe191851ff676ca0d`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml`: `933755207f53b89a74b1869407115bb19e913d50715ccc4776b2f4aa4ab4657c`
- `✏️s/🔌️plugins/💠️lowpoly/🏭️bridge/Cargo.toml`: `04d895b2065f6c753c98db9fb2bfe90f23576087017f5b6c7dc6dea2175333f0`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml`: `eb9c4e067ad5f1889c17b5c33b272af2b97babcf35d8c6a4687618be189d92bf`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/Cargo.toml`: `b4e90e920be86e8d568727182e973f1b22fa15e180981f376d240e27c3fd9e21`
- `✏️s/🔌️plugins/🔋️energy/🏭️bridge/Cargo.toml`: `007283e47a6ce32443a6fa1a405e73dc312a8c16dff99d62b423b40d5834936f`
- `✏️s/🧪️tests/🗂️native-ownership/🟦️.ts`: `1a9b9c6f8ba1dca0e1d54b210ae81b6239538197c564c92082e398db6613383c`
