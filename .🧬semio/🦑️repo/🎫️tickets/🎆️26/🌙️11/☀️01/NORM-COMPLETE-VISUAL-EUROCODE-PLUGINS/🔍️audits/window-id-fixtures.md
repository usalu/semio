# 🔍️ Window ID Fixtures Audit

Scope: stale references to the old Eurocode editor windows `norm-enXXXX-inputs` / `norm-enXXXX-results` (and viewer `-report`) outside the norm artifacts, after the visual windows (`-viewport`, `-diagrams`, `-properties`, `-checks`) replaced them.

Search: `grep -rn "norm-en199[0-9]-\(inputs\|results\|report\)"` over `.json`, `.ts`, `.tsx`, `.rs`, `.md` (excluding `node_modules` and `.🧬semio/`).

## Changed files

1. `🌎️hub/🧩️compositions/📕️norm/🧩️component/🔣️.json`
   - Editor `windowKinds` and `defaultLayout` for `en1990`, `en1992`, `en1993`, `en1994`, `en1996`, `en1998`, `en1999`: `-inputs` and `-results` replaced by `-properties` (Properties/Eigenschaften, canvas-2d), `-viewport` (Model/Modell, world-3d), `-diagrams` (Diagrams/Diagramme, canvas-2d), `-checks` (Checks/Nachweise, canvas-2d). Bodies, icons, labels and the layout follow the artifact sources (`🎭️modes/✏️edit/🦀️.rs` and the `🪟️windows/*` definitions).
   - Layout: row of Properties 22 / Model 50 / column 28 (Checks 50, Diagrams 50). Sizes are written as percent (22.0, 50.0, 28.0), matching the existing 42.0 / 58.0 convention in this file.
   - Caveat: this is a hand edit of a generated `describe` descriptor. The new windows reuse the action list of the old results window. The viewport's extra actions (transform, camera, layers, colour map, section, deformation, load case) and the editor-level viewport interaction are not reproduced. Regenerate the descriptor through the framework describe step to make it exact. No build was run.

2. `✏️s/🔌️plugins/📕️norm/🪟️results/🎚️config/🧪️tests/🧫️fixtures/🔬️window-ownership/🔣️.json`
   - `resultsWindowKinds`: `norm-en1990/1992/1993/1994/1996/1998/1999-results` replaced by `-checks`. The checks window owns the results config (`norm_results_window_config_owner!(ChecksWindowConfigOwner, …)`). `en1991`, `en1995`, `en1997` and the din, iso and vdi kinds are unchanged. The list still has 15 entries.
   - Ownership fixture moved from `norm-en1996-results` to `norm-en1996-checks` (instances, mutations, undo and redo, rejections).
   - `norm-en1996-inputs` replaced by `norm-en1996-properties` for the `norm-inputs` instance. It only serves as a wrong-kind rejection case.
   - Instance ids `norm-results-left`, `norm-results-right`, `norm-inputs` kept as-is (they are instance ids, not window kind ids).

3. `✏️s/🔌️plugins/📕️norm/🪟️results/🎚️config/🧪️tests/🔬️window-ownership/🟦️.ts`
   - Line 68 filter uses `norm-en1996-checks`.
   - Line 66 regex accepts `-(?:results|checks)$`, because the fixture now mixes both suffixes.

## Artifacts already converted (old windows deleted in source), references updated

`en1990`, `en1992`, `en1993`, `en1994`, `en1996`, `en1998`, `en1999`.

Their editor folders have only `✅️checks`, `📈️diagrams`, `🧾️properties`, `🧊️viewport`. Their viewer folders have no `📊️report`.

## Artifacts still pending (old windows still present in source, left untouched)

- `en1991` (`📥️inputs`, `📊️results` in editor, `📊️report` in viewer). Hub entries and fixture `-results` entry remain.
- `en1995` (`📥️inputs`, `📊️results` in editor, `📊️report` in viewer, plus the new windows). Hub entries and fixture `-results` entry remain.
- `en1997` (`📥️inputs`, `📊️results` in editor, `📊️report` in viewer). Hub entries and fixture `-results` entry remain.

Not part of the Eurocode grep, left untouched: `din16798`, `din18599`, `din4108`, `iso16757`, `vdi3805`.

## Verification

- Re-grep for `norm-en199[0-9]-(inputs|results|report)` outside the norm artifacts matches only the three pending artifacts' hub entries.
- Both JSON files parse. The hub round-trips with the same formatting (2-space indent, trailing newline).
- No builds or tests were run. The temporary edit script was deleted.
