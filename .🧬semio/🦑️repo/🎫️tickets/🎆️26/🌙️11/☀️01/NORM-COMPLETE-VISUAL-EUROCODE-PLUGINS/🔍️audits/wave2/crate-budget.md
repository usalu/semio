# Crate budget (wave 2, crate-budget)

Scripts: `🧰️scripts/crate-budget/{lib,split_mounts,twin_leaves,protocol_twin,en1999_parts,en1993_clauses}.py`. Each registers the new crate in the artifact `Cargo.toml` (member + workspace dep), every `Cargo.lock` that knows the parent, `📜️script.ts`, the parent manifest; `verify build-owners sync` and `check-cargo-authority.py` pass (0 errors). No cargo/rustc run; `- ready for rebuild: crate-budget` appended to build-en1991/1992/1993/1996/1997/1998/1999.

## Done
- Mutation leaf crates (31 files, now 19-25): `en1992-mut-member-fields-liquid-bridge`, `en1996-mut-loads-cases`, `en1996-mut-lintels`, `en1991-mut-gravity-floors`, `en1991-mut-snow-roofs`, `en1996-mut-walls-geometry`, `en1997-mut-pile-lateral`, `en1999-mut-collection-rows-connection-shell` (old crates `pub use` the moved modules; kind dirs stay in place).
- en1999-checks: `📏️parts` -> `en1999-check-parts` (4007 -> 2771 lines, 198 KB); checks root `pub use`s it.
- Facet twins (`twin_leaves.py`: generated ts/graphql/json/proto texts into `<crate>-{typescript,graphql,json-schema,proto}` crates, `LEAF` const, root const unchanged): en1996-diff (3769 -> 410), en1996-mutations (-> 1068), en1998-mutations (-> 2246 with protocol twin), en1999-mutations (-> 1691), en1997-diff (-> 681), en1997-mutations (-> 1545), en1992-mutations (-> 1223), en1991-mutations (4572 -> 1084). `protocol_twin.py` moves the binary `.protocol.semio` into `<crate>-protocol` (en1993, en1998, en1997, en1991 mutations; en1993 262695 -> 230991 B).
- en1993-clauses -> `-clauses-core` (2048 lines) + `-clauses-special` (1751) behind the unchanged `clauses` facade.
- en1990-diff already 2060 lines (owner compacted); en1994 long path already renamed to `change-beam-stud-cross-spacing-m` (generators assert < 256 bytes in `leaf-kinds/common.py` and `en1994-leaf-kinds.py`).

## Not done
- en1991-scene (3994 lines, 248 KB) and en1993-scene (240 KB): modules call each other through `pub(crate)` items and root `Ctx`; kit calls up into collections/wave3/dynamics and unit tests reach internals and `diagrams`. A split needs visibility edits and moving tests (not whole-file moves). Proposal: foundation crate (kit, layout, domain modules, `pub` items) + facade crate with diagrams/handles and the unit tests.
- en1996-masonry (3865 lines): one strongly connected module graph (catalogue <-> load-case-checks <-> factors <-> fire <-> conformance); needs function moves (shear/concentrated/compression, governing) into a lower file first.
- Model crates (en1991/1996/1997): owned by CRATE-BUDGET B. en1999-model 3966 lines now near gate.
- Remaining near-budget: en1993-checks-special 234 KB, en1995-scene 230 KB, en1992/en1997-checks ~213-219 KB.
