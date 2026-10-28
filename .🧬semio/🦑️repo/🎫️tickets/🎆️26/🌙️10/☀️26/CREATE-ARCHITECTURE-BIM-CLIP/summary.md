# Six Independent BIM Videos

Six separately selectable videos implement the requested architecture-course topics. Each has its own entry point, localized title, canonical animation, English/German narration text and output identity.

| Video | Topic |
| --- | --- |
| `bim_information` | Data versus information |
| `bim_objects` | Schema, class, instance, attributes, methods, inheritance and composition |
| `bim_representations` | Plan, section, elevation and 3D representations of one object |
| `bim_models` | Same-schema partial models versus purpose-specific model translation |
| `bim_exchange` | Interface contracts, standards, mesh exchange and FEM idealization |
| `bim_consistency` | GUIDs, metadata, links, revisions, stale analyses and explicit updates |

The original combined `bim` video is not registered. The shared BIM entry renders the six-video playlist.

[Source overview and render commands](C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/README.md)

## Verification

- The BIM suite contains 20 contracts. Verification included 28 focused BIM/shared tests, the additional browser metadata regression, and all 98 authored caption texts under both audience policies: 196 native layout calls.
- Existing Ajv, NumPy, Manim, PyAV and Pillow provide independent schema, geometry, layout, decoding and image checks. No runtime package was added.
- All 24 preview jobs completed: six topics × English/German × Speaker/Spectator. PyAV decoded all 1,942 frames. Each language's Speaker and Spectator outputs have identical frame counts.
- Encoded-caption checks found visible text in all 12 Speaker samples and no caption glyphs in all 12 Spectator samples. English and German visible text differ correctly. All six canonical scenes and representative composed movies passed visual review.
- Previews used 640 × 360 at 1 frame/s for the final smoke check; production render settings remain available through the existing commands.
- Maximum generated path length was 172 characters; source paths remain below 256.

[Media verification](media-verification.md) · [Encoded audience/language audit](media-audience.md) · [Test report](bim-tests.md) · [Content and primary-source audit](content-audit.md) · [Integration details](integration.md) · [File inventory](inventory.md)

## Narration and Retained Deliverables

The 49 bilingual narration clause pairs are authored. Prerecorded speech was not supplied or added; container audio tracks do not imply recorded narration. The renderer can attach canonical clause recordings when supplied.

Durable deliverables are the six authored video sources, shared semantic/visual interfaces, schemas, portable cases, tests, render instructions and ticket audits/input scripts. Temporary smoke previews, frames, compiler outputs and logs were removed under the repository's generated-output policy. The subsequently requested playback MP4 is retained and linked below.

Windows execution was verified using the existing repository/UV pipeline. The implementation retains its existing portable interfaces for native macOS/Linux and devcontainers; those hosts were not executed in this session.

No AGENTS file, Git history, worktree or goal status was changed. Unrelated quiz and other contributors' work was preserved.

## Deutsch

Sechs eigenständige BIM-Videos sind mit eigenen Titeln, Einstiegspunkten und Ausgabedateien angelegt. Deutsche und englische Sprechertexte sind vollständig; Audioaufnahmen stehen noch aus. Alle 24 Vorschauvarianten und die Untertitelumschaltung wurden geprüft. Die Quellen und Renderanleitung bleiben erhalten; temporäre Prüfausgaben werden gemäß Repository-Regeln entfernt.

## Requested Playback Preview

A 720p/24 fps German Speaker preview of the first video is now retained for playback. [Play video](C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/bim-preview/de/speaker/BIM_information.mp4). [Verification](preview.md).
