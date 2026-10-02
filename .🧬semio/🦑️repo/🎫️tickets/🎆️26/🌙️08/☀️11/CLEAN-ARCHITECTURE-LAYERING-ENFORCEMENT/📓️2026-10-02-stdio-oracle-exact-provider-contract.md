# Exact Stdio Oracle Provider Contract

The closed authored fixture requires artifact providers to depend only on lower artifact-free families and the neutral Repo host. Hub consumes every provider; generated original artifact hosts select only actual declared providers/families. Production sources are held until coordinated start.

## Exact Family Packages

- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🎒️archive`: `semio-s-plugin-stdio-archive-test-oracle`; actual dependency roots `flate2`, `semio-repo-test-host`, `zip`.
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📃️document`: `semio-s-plugin-stdio-document-test-oracle`; actual dependency roots `lopdf`, `pdf-writer`, `quick-xml`, `semio-repo-test-host`, `zip`.
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📊️tabular`: `semio-s-plugin-stdio-tabular-test-oracle`; actual dependency roots `csv`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📰markup`: `semio-s-plugin-stdio-markup-test-oracle`; actual dependency roots `quick-xml`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔊️audio`: `semio-s-plugin-stdio-audio-test-oracle`; actual dependency roots `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖼️raster`: `semio-s-plugin-stdio-raster-test-oracle`; actual dependency roots `gif`, `image`, `png`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🧊️mesh`: `semio-s-plugin-stdio-mesh-test-oracle`; actual dependency roots `semio-repo-test-host`, `stl_io`, `tobj`.

## Exact Artifact Providers

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las`: `semio-s-artifact-stdio-las-test-oracle`; namespace `artifacts::las`, 1 original mounts; actual dependency roots `las`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html`: `semio-s-artifact-stdio-html-test-oracle`; namespace `artifacts::html`, 1 original mounts; actual dependency roots `html5ever`, `markup5ever_rcdom`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw`: `semio-s-artifact-stdio-epw-test-oracle`; namespace `artifacts::epw`, 1 original mounts; actual dependency roots `csv`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip`: `semio-s-artifact-stdio-zip-test-oracle`; namespace `artifacts::zip`, 2 original mounts; actual dependency roots `semio-repo-test-host`, `zip`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif`: `semio-s-artifact-stdio-gif-test-oracle`; namespace `artifacts::gif`, 2 original mounts; actual dependency roots `gif`, `semio-repo-test-host`, `semio-s-plugin-stdio-raster-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4`: `semio-s-artifact-stdio-mp4-test-oracle`; namespace `artifacts::mp4`, 1 original mounts; actual dependency roots `mp4`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg`: `semio-s-artifact-stdio-svg-test-oracle`; namespace `artifacts::svg`, 3 original mounts; actual dependency roots `quick-xml`, `semio-repo-test-host`, `semio-s-plugin-stdio-markup-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3`: `semio-s-artifact-stdio-mp3-test-oracle`; namespace `artifacts::mp3`, 1 original mounts; actual dependency roots `id3`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc`: `semio-s-artifact-stdio-ifc-test-oracle`; namespace `artifacts::ifc`, 6 original mounts; actual dependency roots `ruststep`, `semio-repo-test-host`, `semio-s-plugin-stdio-part21-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf`: `semio-s-artifact-stdio-bcf-test-oracle`; namespace `artifacts::bcf`, 1 original mounts; actual dependency roots `quick-xml`, `semio-repo-test-host`, `zip`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary`: `semio-s-artifact-stdio-binary-test-oracle`; namespace `artifacts::binary`, 1 original mounts; actual dependency roots `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv`: `semio-s-artifact-stdio-csv-test-oracle`; namespace `artifacts::csv`, 1 original mounts; actual dependency roots `csv`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step`: `semio-s-artifact-stdio-step-test-oracle`; namespace `artifacts::step`, 8 original mounts; actual dependency roots `ruststep`, `semio-repo-test-host`, `semio-s-plugin-stdio-part21-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv`: `semio-s-artifact-stdio-tsv-test-oracle`; namespace `artifacts::tsv`, 1 original mounts; actual dependency roots `csv`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx`: `semio-s-artifact-stdio-xlsx-test-oracle`; namespace `artifacts::xlsx`, 3 original mounts; actual dependency roots `calamine`, `quick-xml`, `rust_xlsxwriter`, `semio-repo-test-host`, `semio-s-plugin-stdio-document-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf`: `semio-s-artifact-stdio-pdf-test-oracle`; namespace `artifacts::pdf`, 10 original mounts; actual dependency roots `lopdf`, `semio-repo-test-host`, `semio-s-plugin-stdio-document-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx`: `semio-s-artifact-stdio-docx-test-oracle`; namespace `artifacts::docx`, 3 original mounts; actual dependency roots `quick-xml`, `semio-repo-test-host`, `semio-s-plugin-stdio-document-test-oracle`, `zip`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md`: `semio-s-artifact-stdio-md-test-oracle`; namespace `artifacts::md`, 1 original mounts; actual dependency roots `comrak`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml`: `semio-s-artifact-stdio-xml-test-oracle`; namespace `artifacts::xml`, 2 original mounts; actual dependency roots `quick-xml`, `semio-repo-test-host`, `semio-s-plugin-stdio-markup-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png`: `semio-s-artifact-stdio-png-test-oracle`; namespace `artifacts::png`, 1 original mounts; actual dependency roots `png`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg`: `semio-s-artifact-stdio-jpg-test-oracle`; namespace `artifacts::jpg`, 2 original mounts; actual dependency roots `image`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi`: `semio-s-artifact-stdio-avi-test-oracle`; namespace `artifacts::avi`, 1 original mounts; actual dependency roots `riff`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx`: `semio-s-artifact-stdio-pptx-test-oracle`; namespace `artifacts::pptx`, 3 original mounts; actual dependency roots `quick-xml`, `semio-repo-test-host`, `semio-s-plugin-stdio-document-test-oracle`, `zip`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav`: `semio-s-artifact-stdio-wav-test-oracle`; namespace `artifacts::wav`, 1 original mounts; actual dependency roots `riff`, `semio-repo-test-host`, `semio-s-plugin-stdio-audio-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt`: `semio-s-artifact-stdio-txt-test-oracle`; namespace `artifacts::txt`, 1 original mounts; actual dependency roots `bstr`, `csv`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl`: `semio-s-artifact-stdio-stl-test-oracle`; namespace `artifacts::stl`, 1 original mounts; actual dependency roots `semio-repo-test-host`, `stl_io`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg`: `semio-s-artifact-stdio-dwg-test-oracle`; namespace `artifacts::dwg`, 2 original mounts; actual dependency roots `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf`: `semio-s-artifact-stdio-dxf-test-oracle`; namespace `artifacts::dxf`, 1 original mounts; actual dependency roots `dxf`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff`: `semio-s-artifact-stdio-tiff-test-oracle`; namespace `artifacts::tiff`, 2 original mounts; actual dependency roots `semio-repo-test-host`, `tiff`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate`: `semio-s-artifact-stdio-deflate-test-oracle`; namespace `artifacts::deflate`, 1 original mounts; actual dependency roots `flate2`, `semio-repo-test-host`, `semio-s-plugin-stdio-archive-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj`: `semio-s-artifact-stdio-obj-test-oracle`; namespace `artifacts::obj`, 1 original mounts; actual dependency roots `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf`: `semio-s-artifact-stdio-gltf-test-oracle`; namespace `artifacts::gltf`, 1 original mounts; actual dependency roots `json`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply`: `semio-s-artifact-stdio-ply-test-oracle`; namespace `artifacts::ply`, 1 original mounts; actual dependency roots `ply-rs`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json`: `semio-s-artifact-stdio-json-test-oracle`; namespace `artifacts::json`, 1 original mounts; actual dependency roots `json`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio`: `semio-s-artifact-stdio-semio-test-oracle`; namespace `artifacts::semio`, 1 original mounts; actual dependency roots `json`, `semio-repo-test-host`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp`: `semio-s-artifact-stdio-bmp-test-oracle`; namespace `artifacts::bmp`, 1 original mounts; actual dependency roots `image`, `semio-repo-test-host`, `semio-s-plugin-stdio-raster-test-oracle`.
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note`: `semio-s-artifact-note-note-test-oracle`; namespace `note::artifacts::note`, 1 original mounts; actual dependency roots `dxf`, `semio-repo-test-host`, `semio-s-plugin-stdio-document-test-oracle`, `semio-s-plugin-stdio-markup-test-oracle`.

## Explicit Consumer Artifact Bindings

- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program`: `semio-s-artifact-stdio-xlsx-test-oracle`, `semio-s-plugin-stdio-archive-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las`: `semio-s-artifact-stdio-las-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html`: `semio-s-artifact-stdio-html-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw`: `semio-s-artifact-stdio-epw-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip`: `semio-s-artifact-stdio-zip-test-oracle`, `semio-s-plugin-stdio-archive-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif`: `semio-s-artifact-stdio-gif-test-oracle`, `semio-s-plugin-stdio-raster-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4`: `semio-s-artifact-stdio-mp4-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg`: `semio-s-artifact-stdio-svg-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3`: `semio-s-artifact-stdio-mp3-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc`: `semio-s-artifact-stdio-ifc-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf`: `semio-s-artifact-stdio-bcf-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary`: `semio-s-artifact-stdio-binary-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv`: `semio-s-artifact-stdio-csv-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step`: `semio-s-artifact-stdio-step-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv`: `semio-s-artifact-stdio-tsv-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx`: `semio-s-artifact-stdio-xlsx-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf`: `semio-s-artifact-stdio-pdf-test-oracle`, `semio-s-plugin-stdio-document-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx`: `semio-s-artifact-stdio-docx-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md`: `semio-s-artifact-stdio-md-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml`: `semio-s-artifact-stdio-xml-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png`: `semio-s-artifact-stdio-png-test-oracle`, `semio-s-plugin-stdio-raster-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg`: `semio-s-artifact-stdio-jpg-test-oracle`, `semio-s-plugin-stdio-raster-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi`: `semio-s-artifact-stdio-avi-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx`: `semio-s-artifact-stdio-pptx-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav`: `semio-s-artifact-stdio-wav-test-oracle`, `semio-s-plugin-stdio-audio-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt`: `semio-s-artifact-stdio-txt-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl`: `semio-s-artifact-stdio-stl-test-oracle`, `semio-s-plugin-stdio-mesh-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg`: `semio-s-artifact-stdio-dwg-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf`: `semio-s-artifact-stdio-dxf-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff`: `semio-s-artifact-stdio-tiff-test-oracle`, `semio-s-plugin-stdio-raster-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate`: `semio-s-artifact-stdio-deflate-test-oracle`, `semio-s-plugin-stdio-archive-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj`: `semio-s-artifact-stdio-obj-test-oracle`, `semio-s-plugin-stdio-mesh-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf`: `semio-s-artifact-stdio-gltf-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply`: `semio-s-artifact-stdio-ply-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json`: `semio-s-artifact-stdio-json-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio`: `semio-s-artifact-stdio-semio-test-oracle`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp`: `semio-s-artifact-stdio-bmp-test-oracle`, `semio-s-plugin-stdio-raster-test-oracle`.

## Source Preservation

The complete caller cohort remains 257 actual source inputs; 72 original mounts and 13 shared/law implementation inputs retain source digests and explicit owned-import substitutions. STEP's exact decode function body is retained independently by `8646c3b5923f0cd35658055fddd79a5b4522ca3e8f8acd5936a1655b5fee93bb` in the shared lower grammar. These source witnesses do not replace original native scenarios and provider/family unit laws.
