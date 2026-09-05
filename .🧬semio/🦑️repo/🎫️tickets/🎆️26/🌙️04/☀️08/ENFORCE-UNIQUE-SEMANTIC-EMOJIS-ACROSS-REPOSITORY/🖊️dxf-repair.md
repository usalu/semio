# DXF Emoji Repair

The final artifact root is `🖋️dxf`, distinct from sibling DWG. Handpicked DXF R12 subset identities are `🧱️blocks`, `🧩️entities`, `📰️header`, and `📊️tables`. Each subset now has a distinct `🔮️oracle`; editor/viewer options use `☑️options`. Snapshot mutations use `📸️set-snapshot`; mutation tests use their subset identity, and the circle-radius scenario uses `⭕️`. The catalog scenario directory resolves to the physical mutation test directory.

All 38 fixture directory names are explicitly authored in the existing generator's `FIXTURE_COORDINATES`. Fixture semantic IDs remain unchanged. Applied and rejected outcomes have distinct meaningful sibling identities. Paired carriers are `⬅️before.dxf` and `➡️after.dxf`; the drafting plate is `📐️drafting-plate/📐️drafting-plate.dxf`. Generator output defaults to the fixture's owning subset and supports isolated output for verification.

Exact source references, ownership fields, scenario declarations, Stdio mounts, and central oracle overrides were repaired in-place. No Git state was modified.

Verification:

- Full DXF scoped filename audit: 266 files, 181 directories, 440 governed entries; all eight categories zero.
- Repository fixture verifier: 38 fixtures, zero file problems.
- Existing third-party `dxf` generator release build succeeded and generated all 38 bundles into the ticket's temporary output folder.
- All 39 JSON documents parsed; all 38 generated manifest identities and 58 generated file paths, byte counts, and SHA-256 digests matched the committed fixtures.

The workspace-wide completion audit remains open while other artifact trees are repaired.
