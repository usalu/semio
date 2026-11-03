# crate-budget B (wave 2)
Model crates over the source-boundary gate after the IO-ORPHAN mount (en1991 4082, en1996 4021, en1997 4035 lines; later also en1992 4186, vdi3805 4276, iso16757 4433).

Finding: the closure was dominated by the four `include_str!` facet texts of `SNAPSHOT_LEAVES` (json schema ~2.2k lines/54 KB, ts ~300, graphql, proto), not by row types. Moving rows would not have reached the budget, so the orphan-rule-sensitive code (snapshot struct, codecs, sqlite, records) stays untouched in the model crate.

Fix: `🧰️scripts/crate-budget/model_twin_leaves.py ARTIFACT` (idempotent) creates `<model>-typescript|graphql|json-schema|proto` twin crates (each `pub const LEAF` via `include_str!("../<file>")`, files stay in place so generators still write them), registers members/workspace deps/Cargo.lock/script.ts, and rewires `SNAPSHOT_LEAVES` to `<crate>::LEAF` (same pattern as en1993-model).

Result (`source-boundary-scan.py`): no crate over the gate except `en1999-mut-collection-rows` (43 files, owned by A). Model crates: en1991 1169 lines/93 KB, en1996 1258/82 KB, en1997 1132/82 KB; largest twin (json) 2.2k lines/55 KB.
Applied to: en1991, en1992, en1996, en1997, vdi3805, iso16757.
Checks: `check-cargo-authority.py` 28 crates 0 errors; `verify build-owners sync` passed (no changes). No cargo run.
