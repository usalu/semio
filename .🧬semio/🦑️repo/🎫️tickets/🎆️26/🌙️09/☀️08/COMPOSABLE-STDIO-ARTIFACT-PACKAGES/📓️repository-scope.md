# Repository Artifact Scope

The user requested every artifact, using stdio as the motivating example. The initial execution pass addresses stdio’s 36 roots. Completion also requires the other first-party artifact owners; the scope must not silently stop at stdio.

The current Rust/TypeScript/schema source inventory identifies 93 artifact roots. This is an inventory, not a claim of implemented package boundaries.

| Owner | Count | Artifacts |
| --- | ---: | --- |
| ✏️s/🔌️plugins/✒️writer/🗿️artifacts | 1 | ✒️writer |
| ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts | 1 | ➗️equation |
| ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts | 3 | 🌀️generation2d, 🧊️generation3d, 🧩️assembly |
| ✏️s/🔌️plugins/🌊️flow/🗿️artifacts | 1 | 🌊️flow |
| ✏️s/🔌️plugins/🌍️gis/🗿️artifacts | 2 | 🏔️gisterrain, 🗺️gismap |
| ✏️s/🔌️plugins/🌿️vcs/🗿️artifacts | 1 | 🌿️vcs |
| ✏️s/🔌️plugins/🎞️animate/🗿️artifacts | 1 | 🎬️presentation |
| ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts | 1 | 🎥️shooting |
| ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts | 1 | 🎪️playground |
| ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts | 1 | 🎬️sequence |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts | 2 | ◻️2d, 🧊️3d |
| ✏️s/🔌️plugins/🏛️architect/🗿️artifacts | 1 | 🏛️program |
| ✏️s/🔌️plugins/🏭️process/🗿️artifacts | 1 | 🧊️process3d |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts | 1 | 💠️lowpoly |
| ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts | 1 | 🔌️wires |
| ✏️s/🔌️plugins/📋️forms/🗿️artifacts | 1 | 📋️forms |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts | 1 | 📏️layout |
| ✏️s/🔌️plugins/📐️cad/🗿️artifacts | 1 | 📐️cad |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts | 15 | ⚖️en1990, ⚡️din18599, 🌍️en1997, 🌬️din16798, 🏋️en1991, 🏛️en1992, 🏭️vdi3805, 📇️iso16757, 🔩️en1993, 🧩️en1994, 🧱️din4108, 🪨️en1996, 🪵️en1995, 🪶️en1999, 🫨️en1998 |
| ✏️s/🔌️plugins/📖️playbook/🗿️artifacts | 1 | 📖️playbook |
| ✏️s/🔌️plugins/📜️imperative/🗿️artifacts | 1 | 📜️procedure |
| ✏️s/🔌️plugins/📸️remodel/🗿️artifacts | 1 | 📸️remodeling |
| ✏️s/🔌️plugins/🔋️energy/🗿️artifacts | 1 | 🔋️model |
| ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts | 2 | ♻️rewriting, 🔌️jack |
| ✏️s/🔌️plugins/🕸️dag/🗿️artifacts | 1 | 🕸️dag |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts | 1 | 🖍️drawing |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts | 1 | 🖨️raster |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts | 36 | ☁️las, 🌐️html, 🌦️epw, 🎒️zip, 🎞️gif, 🎥️mp4, 🎨️svg, 🎵️mp3, 🏗️ifc, 💬️bcf, 💾️binary, 📊️csv, 📐️step, 📑️tsv, 📕️xlsx, 📖️pdf, 📜️docx, 📝️md, 📰️xml, 📷️png, 📸️jpg, 📼️avi, 📽️pptx, 🔊️wav, 🔤️txt, 🔺️stl, 🖊️dwg, 🖋️dxf, 🖼️tiff, 🗜️deflate, 🗽️obj, 🧊️gltf, 🧱️ply, 🧾️json, 🧿️semio, 🪟️bmp |
| ✏️s/🔌️plugins/🗒️note/🗿️artifacts | 1 | 🗒️note |
| ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts | 3 | ◻️2d, 🖐️5d, 🧊️3d |
| ✏️s/🔌️plugins/🧱️block/🗿️artifacts | 3 | ◻️2d, 🖐️5d, 🧊️3d |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts | 2 | 🏠️home, 🪐️space |
| ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts | 1 | 🗂️curation |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts | 1 | 🏃️run |

## Wider Language Inventory Check

A second repository-wide ripgrep inventory included Rust, TypeScript/TSX, Python, C#, C, C++, and artifact-definition JSON sources under visible first-party trees. It found 93 artifact roots. Additional roots outside the initially inventoried owner prefixes: none.
