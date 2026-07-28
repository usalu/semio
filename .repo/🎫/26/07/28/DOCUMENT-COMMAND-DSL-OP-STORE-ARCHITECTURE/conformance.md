# Per-App Conformance Checklist (Wave W8 tracker)

One row per crate containing `DocumentApp` impls (31 crates, 39 impls). Columns map to the
target pillars; ✓ = verified with a law test, ◐ = present but unverified/partial, ✗ = missing,
— = pending audit. W8 agents fill their row(s) and cite test names in their wave report.

Legend: Ent = typed entities · Diff = OperationDiff · Inv = composed inverses (no snapshot
backwards) · Dsl = DocumentDsl + law · Pack = DocumentPack + law · OpT = OpText + law ·
OpB = OpBinary + law (after W2) · Auth = author_id threading (after W8 framework change)

| Crate | Ent | Diff | Inv | Dsl | Pack | OpT | OpB | Auth |
|---|---|---|---|---|---|---|---|---|
| animate/plugin | — | — | — | — | — | — | — | — |
| architect/plugin | — | — | — | — | — | — | — | — |
| cad/plugin (+cad/rs) | — | — | — | — | — | — | — | — |
| draw/plugin | — | — | — | — | — | — | — | — |
| fem/plugin (+fem/2d,3d) | — | — | — | — | — | — | — | — |
| flow/plugin (+flow/core) | — | — | — | — | — | — | — | — |
| forms/plugin | — | — | — | — | — | — | — | — |
| gis/plugin | — | — | — | — | — | — | — | — |
| imperative/plugin (+core) | — | — | — | — | — | — | — | — |
| infinite/board/port/directed/dag/plugin | — | — | — | — | — | — | — | — |
| layout/plugin (+layout/rs) | — | — | — | — | — | — | — | — |
| lowpoly/plugin (+core) | — | — | — | — | — | — | — | — |
| mathematical/plugin | — | — | — | — | — | — | — | — |
| norm/plugin | — | — | — | — | — | — | — | — |
| note/plugin | — | — | ✗ snapshot inverses (lib.rs:328-343) | — | — | — | — | — |
| playbook/plugin (+module/procedural, playbook/rs) | — | — | — | — | — | — | — | — |
| procedural/plugin | — | — | — | — | — | — | — | — |
| process/plugin (+process/3d) | — | — | — | — | — | — | — | — |
| puzzle/plugin (+2d/3d/5d) | — | — | — | — | — | — | — | — |
| raster/plugin | — | — | — | — | — | — | — | — |
| reasoning/mindmap/plugin | — | — | — | — | — | — | — | — |
| remodel/plugin | — | — | — | — | — | — | — | — |
| s/plugin (home, studio) | — | — | — | — | — | — | — | — |
| sequence/plugin (+core) | — | — | — | — | — | — | — | — |
| shooting/plugin (+shooting/rs) | — | — | ◐ WS-F wave-1 migrated | — | — | — | — | — |
| sourcing/plugin | — | — | — | — | — | — | — | — |
| trinity/plugin (+ram, rewrite) | — | — | — | — | — | — | — | — |
| vcs/plugin | — | — | ◐ WS-F wave-1 migrated | — | — | — | — | — |
| writer/plugin | — | — | — | — | — | — | — | — |
| framework/plugin (VcsDocumentApp wrapper) | n/a | n/a | n/a | n/a | n/a | n/a | n/a | owner |
| compose/client/lib (KitSnapshot — not a DocumentApp; JSON pack bridge) | ✗ | — | — | ✗ | ✗ G19 | — | — | — |

Non-plugin `DocumentStore` users to sweep in W3 (dispatch_json deletion): trinity/ram,
trinity/rewrite/engine, playbook/rs, imperative/core, animate/present, fem/2d, fem/3d,
puzzle/3d, puzzle/5d, framework/product/os/core (OsStore), s/rs (StudioStore).
