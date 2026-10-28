# Uniform Section Preview Media

Current actual-helper preview input identity: `62efe597fff3`.

Both previews render real open/close animations through the production house, plant and room constructors. Every frame was decoded with PyAV; dimensions and nondecreasing timestamps were checked. This is bounded section verification, not a rerender of all five curriculum movies.

| Movie | Frames | FPS | Duration (s) | SHA-256 |
|---|---:|---:|---:|---|
| UniformRoomOpenings.mp4 | 22 | 10 | 2.200 | b5e7b9c88b25edfd5792816cbe03fb57851f58a289adcfb1aac11517788fbe2e |
| UniformSectionOverview.mp4 | 22 | 10 | 2.200 | 94622e1414b1abfae569a83c90346ec8020f1ed00dbef8c728f892f1361972ec |

## Endpoint Images

| PNG | SHA-256 | Matches Initial Preview |
|---|---|---|
| UniformRoomOpenings-closed-again.png | 2652194a3fa1ebacd7105bbb3c206fba90ad0e40684326750ad6bbf7a8a5bf01 | no |
| UniformRoomOpenings-closed.png | 2652194a3fa1ebacd7105bbb3c206fba90ad0e40684326750ad6bbf7a8a5bf01 | no |
| UniformRoomOpenings-open.png | bdadb4bd92ac35f09e6b747118f8b6de052b43fb761b108dda87f19f18ac9534 | no |
| UniformSectionOverview-closed-again.png | 47530f91efa5a9ad23e31afadd9e2fbea101f1ec3e5c3f6bec911fa14ceeec76 | no |
| UniformSectionOverview-closed.png | 47530f91efa5a9ad23e31afadd9e2fbea101f1ec3e5c3f6bec911fa14ceeec76 | no |
| UniformSectionOverview-open.png | da29da662d7bf337de8a7743c0b9f071804080cd0b05a75410f0d042811a8566 | no |

All six endpoint images are independently reviewed in sections-audit.md. The initial endpoint-only preview is retained in the audit chronology; the movies above establish actual frame generation through both directions of window movement. Reproduce with the managed authoring environment and `repo tools exec uv -- run --project <authoring-project> --python 3.12 --group test python -B <ticket>/sections-preview.py`, then this input with the printed identity.

This correction's temporary binaries, PNGs, fonts and logs were removed after acceptance. Unrelated concurrent output in the shared generated folder is preserved for its owner. This Markdown and the preview/verifier inputs remain.
