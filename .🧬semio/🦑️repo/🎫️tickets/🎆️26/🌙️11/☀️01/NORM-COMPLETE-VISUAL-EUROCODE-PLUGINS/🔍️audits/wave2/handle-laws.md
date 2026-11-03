# Handle laws (wave 2)

Scope: `assert_handle_laws` (structural-scene `projection_kit::test_support`) wired into the ten Eurocode scene unit tests (`🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs`). No build or cargo run.

## Finding: no existing law call sites in the artifacts

`assert_scene_laws` and `assert_diagram_laws` have zero call sites under `🗿️artifacts` (or anywhere else in the repo besides the helper itself). The i18n audit's "used by 64 artifact tests" does not match the tree. There was no existing call pattern to copy, so the handle-law calls were placed after each variable-bound `handles(...)` / `handles::all(...)` call. Scene-law and diagram-law calls were not added (out of scope, no anchor).

## Changes

Imports: `use scene::projection_kit::test_support::{revision, entity, assert_handle_laws};` extended in en1990, en1991, en1992, en1993, en1995, en1996, en1999; `{revision, assert_handle_laws}` in en1994; new line `use scene::projection_kit::test_support::assert_handle_laws;` in en1997 and en1998.

| Artifact | Calls added | Notes |
|---|---|---|
| en1990 | 7 | fixture loop (`all` at 252) and single-document handle tests |
| en1991 | 8 | |
| en1992 | 13 | `assert_handle_laws(&built.handles)` after each `let (built, _) = built(..)` (fixture loops included) |
| en1993 | 5 | |
| en1994 | 3 | fixture loop (`list`) |
| en1995 | 13 | |
| en1996 | 3 | |
| en1997 | 3 | 2 `let` sites plus the `handles::all` loop, which now binds `let all` first |
| en1998 | 5 | fixture loops (`all`) |
| en1999 | 5 | |

Total: 65 call sites, 10 import edits.

Not covered (inline calls with no bound variable, to keep edits minimal): en1991 ~312 (`.iter().map(..)`), en1993 ~444 (`.into_iter()`), en1993 ~461 (`.len()`), en1995 ~313 and ~599 (inline `.iter()`), en1995 ~765 (filtered `for`) and ~808 (flat_map). These can be added by the owners if they want full coverage.

## German accessible names

Scanned all `loc(..)`, `named(..)` and `(format!, format!)` DE arguments in the ten scene `🕹️handles` and kit sources (about 715 unique strings) with the `looks_english` markers, plus the tuple names in en1991, en1997 and en1998. No DE handle name reads as English, so no text was changed.

Not verifiable statically: DE names built with data labels (`label_de`, `member.label_de`, `{id}`) are only checked by the heuristic at run time once the tests execute.

## Rebuild requests

Appended `- ready for rebuild: handle laws` to `🗑️generated/build-en1990/latest.md` through `build-en1999/latest.md` (all ten).

## Not done

- No cargo or rustc run, no builds started.
- No git writes.
- The `build-en*` directories and the helper scripts were left in `🗑️generated` for the ticket owner to clean up.
