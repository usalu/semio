# Current Oracle Cargo Workspace Prerequisite Audit

Read-only source audit, 2026-10-03. No Cargo or manifest modifications. Root reported Forms13 stopped before assertions first on a missing HTML oracle path, then multiple workspace roots naming S/HTML/MD. Those are measured historical invocations; they do not establish the current source still fails.

## Current measured binding state

Current `✏️s/Cargo.toml:143` already contains exact excludes for PNG, MD and HTML oracle package directories alongside broad `**/🔮️oracles/**`, standards and guest patterns. The named three are absent from the explicit member list. Root confirmed these exact exclusions arrived concurrently; this agent and Root did not author them. HTML and MD oracle manifests each declare their own `[workspace]` at1, package role=test at10, explicit edition/version, lib path ../../🦀️.rs at15, optional independent library behind oracles feature18, and a direct existing repo test host dependency. This matches every other inspected oracle manifest convention.

HTML artifact package Cargo47 and MD Cargo48 now use existing ../../🔮️oracles/📦️packages/🦀️rust dev-dependencies with features=[oracles]. Every path= value in all46 oracle manifests resolves to an existing filesystem path, including library entry and local package dependencies. No current missing path was found by this filesystem check. This is not Cargo package-name or full workspace metadata verification.

The coherent permanent repair for the **measured** HTML/MD multiple-root binding is explicit S-workspace exclusion of their actual standalone oracle roots, preserving standalone [workspace], role=test, dev dependency and oracles feature. Current source already expresses that repair. Do not add nested shim manifests, change source/lib ownership, delete [workspace] indiscriminately, move providers into retired directories, remove dev dependencies or disable assertions. No additional measured binding change is justified before the next meaningful selected native run.

## Remaining source qualification

All46 actual oracle packages declare standalone [workspace]; none is explicitly an S member.43 rely on the broad exclusion pattern without a concrete directory exclusion. This is the full similar roster below, not43 proven remaining failures. The historical multiple-root failure despite the broad pattern is reason to verify exact dependency-reachable exclusions when a new oracle enters the selected graph; it does not establish every one now participates in Cargo discovery or fails.

If the permanent policy is to make every standalone oracle boundary explicit, hand-author all46 exact package directory exclusions in S authority, preserving test features and package ownership. This is a coherent source normalization but exceeds the narrow measured HTML/MD repair; it is unnecessary to rerun a metadata-only busy loop here. Repository workspace.metadata.semio.repository at145–147 lists **/📦️packages/🦀️rust/Cargo.toml for semantic inventory; that is not Cargo members authority and must not be substituted for it. Follow current canonical script/Nx test lane with the authentic unchanged Forms13 selection when prerequisites permit; no alternate workspace or nested manifest lane was introduced.

## Exact declared oracle roster

Each entry is relative to ✏️s, has [workspace], has no explicit S member entry, and has all literal manifest paths present. E means exact S exclude143; G means broad oracle exclusion only.

- G: `semio-s-plugin-stdio-archive-test-oracle` — `🔌️plugins/🗄️stdio/🔮️oracles/🎒️archive/📦️packages/🦀️rust`
- G: `semio-s-plugin-stdio-document-test-oracle` — `🔌️plugins/🗄️stdio/🔮️oracles/📃️document/📦️packages/🦀️rust`
- G: `semio-s-plugin-stdio-tabular-test-oracle` — `🔌️plugins/🗄️stdio/🔮️oracles/📊️tabular/📦️packages/🦀️rust`
- G: `semio-s-plugin-stdio-markup-test-oracle` — `🔌️plugins/🗄️stdio/🔮️oracles/📰markup/📦️packages/🦀️rust`
- G: `semio-s-plugin-stdio-audio-test-oracle` — `🔌️plugins/🗄️stdio/🔮️oracles/🔊️audio/📦️packages/🦀️rust`
- G: `semio-s-plugin-stdio-part21-test-oracle` — `🔌️plugins/🗄️stdio/🔮️oracles/🔤️part21/📦️packages/🦀️rust`
- G: `semio-s-plugin-stdio-drawing-test-oracle` — `🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/📦️packages/🦀️rust`
- G: `semio-s-plugin-stdio-raster-test-oracle` — `🔌️plugins/🗄️stdio/🔮️oracles/🖼️raster/📦️packages/🦀️rust`
- G: `semio-s-plugin-stdio-mesh-test-oracle` — `🔌️plugins/🗄️stdio/🔮️oracles/🧊️mesh/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-las-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🔮️oracles/📦️packages/🦀️rust`
- E: `semio-s-artifact-stdio-html-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-epw-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-zip-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-gif-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-mp4-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-svg-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-mp3-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-ifc-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-bcf-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-binary-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-csv-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-step-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-tsv-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-xlsx-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-pdf-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-docx-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🔮️oracles/📦️packages/🦀️rust`
- E: `semio-s-artifact-stdio-md-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-xml-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🔮️oracles/📦️packages/🦀️rust`
- E: `semio-s-artifact-stdio-png-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-jpg-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-avi-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-pptx-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-wav-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-txt-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-stl-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-dwg-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-dxf-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-tiff-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-deflate-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-obj-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-gltf-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-ply-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-json-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-semio-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-stdio-bmp-test-oracle` — `🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🔮️oracles/📦️packages/🦀️rust`
- G: `semio-s-artifact-note-note-test-oracle` — `🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔮️oracles/📦️packages/🦀️rust`

No compiler, parser or native assertion success is claimed. Forms13 remained before assertions in the reported invocations; current source repair is bounded evidence awaiting the next actual selected execution.
