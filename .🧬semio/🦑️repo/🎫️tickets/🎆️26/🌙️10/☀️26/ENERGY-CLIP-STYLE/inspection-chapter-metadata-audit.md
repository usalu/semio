# Chapter Metadata And Body Dependency Audit

Current collection metadata has 38 localized titles for exactly 38 manifest chapters, with no missing or extra keys. Each title was compared with the actual ordered segment scene names and selected clause ranges. Ventilation, heating, energy balance, BIM and optimization labels match their contents. Cooling Transmission and Humidity is broad but defensible: the chapter contains opaque transmission, thermal-mass day/summer excerpts and summer ventilation including latent load. No factual title mismatch was found.

## Proposed Presentation Edits And Existing Body Certificates

Inspected 119 native source render sidecars containing 119 source maps. None names any proposed hierarchy Python, hierarchy-render Python, presentation Python, browser Python/MJS/TSX, parent presentation schema or file currently under the new collection folder. This is dependency-membership evidence only, not freshness certification. Current runtime segment_state421–429 follows literal authored imports, then recursively includes qualifying assets beneath those Python owners. interaction.source_files explicitly includes the parent presentation catalog JSON, but not the parent schema, hierarchy or browser owners. Therefore preserve the existing parent catalog JSON and runtime/interaction files while changing the listed presentation-only files. Newly added collection assets remain excluded unless a body-import owner is expanded to their parent.

## Path Lengths

All proposed existing file paths are below256UTF-16 units, the stricter Windows measure. Exact lengths:

- 106 UTF-16 units: `C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🥞️hierarchy/🐍️.py`
- 116 UTF-16 units: `C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🥞️hierarchy/🖌️render/🐍️.py`
- 93 UTF-16 units: `C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🐍️.py`
- 104 UTF-16 units: `C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🐍️.py`
- 111 UTF-16 units: `C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟨️render.mjs`
- 105 UTF-16 units: `C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟦️.tsx`
- 101 UTF-16 units: `C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🧬️schema.json`
- 107 UTF-16 units: `C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/📚️collection/🐍️.py`
- 109 UTF-16 units: `C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/📚️collection/🔣️.json`
- 115 UTF-16 units: `C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/📚️collection/🧬️schema.json`

The inspection is read-only; no source state, sidecar, media or production file was modified. The live source-only refresh remains owned by root/executor.

## Current Literal-Import Closure Cross-Check

Independently replayed the production literal import discovery AST rule over 38 unique authored script files, then added the fixed shared Python owners used by segment_state. The union contains 65 Python dependency paths. Intersection with all ten proposed current presentation files, including recursive owner-asset containment for JSON: []. No rendering libraries were imported. This corroborates the existing sidecar membership result against current authored imports.
