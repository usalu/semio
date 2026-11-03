# Trace fix (W3-TRACE-FIX)

Scope: the 81 implemented check rows whose spec-trace fragment was absent from non-test code (stale), per `🧰️scripts/norm-gaps/gaps.py`.

## Change

Each stale row's `realisation.checks` now holds its catalogue id (the quoted item id, which is the `Entry { id }` in the catalogue) instead of the old `en<code>.<id>.` fragment. The catalogue id is what the code emits, so the fragment is the id itself.

Rows changed per code:

- en1990: 5 (E90.10, E90.11, E90.12, E90.17, E90.18)
- en1991: 6 (E14.07, E14.12, E14.14, E14.15, E15.02, E15.05)
- en1995: 24 (waiting, see status)
- en1996: 9 (M-16, M-21, M-33, M-40, M-41, M-42, M-43, M-82, M-83)
- en1997: 37 (GI-02 ... GI-16, GEO-F03, HYD-02/03, SLS-02/03, PILE-05/06/08, RW-04/08/09/10, SLOPE-02/03/04, ANC-01..04, DEW-01..03)

Edited JSON: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<code>/🏅️standards/🔖️1/🪆️subsets/✳️any/🧭️spec-trace/🔣️.json`. Twins `🦀️.rs` regenerated with `write()` from `⚖️compliance/🧭️spec-trace/🧰️codegen/🐍️.py`. Script: `🗑️generated/trace-fix/fix.py`.

Note: en1991 JSON and twin also carry uncommitted edits from other owners (the diff stat is larger than the six rows). Those were left as they are.

## Verification

- `/home/deck/semio/.venv/bin/python -I 🐍️.py` in `⚖️compliance/🧭️spec-trace/🧰️codegen`: all ten codes ok, rc=0.
- `/home/deck/semio/.venv/bin/python -I 🐍️.py` in `⚖️compliance/🧪️tests/🌐️i18n-sinks`: ok, rc=0.
- `gaps.py` re-run: see the status section below.

## Status

- en1995 is still named in `🗑️generated/build-runner/freeze.txt` (`build-en1995 graph-plan since 17:02:07`). The 24 en1995 rows are NOT edited yet. The rule is never to edit an artifact while it is frozen, so they are pending a freeze release and a rerun of `fix.py en1995`.
- en1996 was unfrozen and its 9 rows were updated.

## Result at hand-back

- gaps.py after the edits: 24 stale rows remain, all en1995 (frozen). The 57 rows in en1990, en1991, en1996 and en1997 are fixed.
- Codegen check (all ten codes) and i18n-sinks: both ok after the edits.
- Next step: when `build-en1995` leaves `freeze.txt`, run `/home/deck/semio/.venv/bin/python -I 🗑️generated/trace-fix/fix.py en1995` from `⚖️compliance/🧭️spec-trace/🧰️codegen`, then re-run gaps.py (expect stale 0).
