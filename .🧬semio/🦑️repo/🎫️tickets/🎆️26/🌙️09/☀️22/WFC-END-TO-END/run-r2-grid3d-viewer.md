# Grid3d Authored Viewer

The read-only viewer previously called the synchronous solver on every render. That hidden expensive work has been removed. The viewer now projects only authored pins from its supplied current document, filtering missing tile IDs, out-of-bounds cells and masked cells. It keeps no solve cache and therefore cannot display an earlier document's cached solution. Editor live/completed solver results remain revision-bound and owned by the resumable job.

The existing pure World3d scene projection and camera framing are reused. There is no fallback solver or editor import in the viewer. Old documentation claiming inline solve support was corrected; the example-render law now describes an authored scene.

Four portable schema-first cases cover unsolved boot (no invented assignments), valid/invalid/masked pins, current document replacement and canonical coordinate order. Existing Ajv validates the fixture. An independent NumPy projection reproduces all4 expected outputs. Both edited Rust files pass pinned syntax-only parsing. Native execution of the viewer law remains pending.

Worked files under grid3d standard1/any/viewer/modes/view/windows/preview: root Rust source; tests/unit Rust; tests/authored-preview JSON and JSON Schema. The actual checked-in emoji paths are used in source. Ticket retained input grid3d-authored-preview-oracle.py supplies the third-party oracle.

The independent neutral headless native gate24680 remains live and frozen; viewer edits are outside that package's source closure.
