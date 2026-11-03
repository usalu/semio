# Trace and Tools (wave 2, small items)

## 1. Trace/spec drift
- EN 1996 M-43: code implements the confined-masonry check (`m-43.` in scene members, catalogue row, oracle). The spec said out_of_scope, so the spec was stale. Fixed in `📐️specs/en1995-en1996-target.md` (heading and one line; prestressed masonry stays out of scope). Trace kind `check` kept.
- EN 1997 HYD-02, HYD-03: clauses exist (10.4). Added to `📐️specs/en1997-en1998-target.md` section 2.7 (heading and two lines). Trace unchanged.
- EN 1997 DEW-02 (well group, catalogue/oracle exist): added to spec 2.14 (one line each for DEW-01..03) and added the trace row `DEW-02` (check `en1997.dew-02.`, entity layerWaters, EN + DE) plus specIds entry in en1997 `🧭️spec-trace/🔣️.json`.
- GI-17: already folded into GI-13 in the trace (checks `gi-13.`, `gi-17.`, description says code entry); GI-01..16 now have rows. No change.
- EN 1999 A-33: spec says connectors (8.5.10, 8.9), trace is the pin check; consistent, kept. M-95 stays a spec stub (not changed).
- Twins regenerated with codegen `--write` (no freeze.txt present); strict verifier: all ten ok (en1997 now 61 ids).

## 2. DSL patch tool (`🛠️asset-laws/patch.py`, `dslmodel.py`)
- Enum spelling now comes from the Rust DslScalar `#[dsl(key = "...")]` of each variant (new `KEYS`/`spell()`), with kebab-casing only when no key exists. No re-casing of keyed values (`GR1A` stays `gr1a`).
- There is no `--check` flag; the default run is the dry run. Ran over all artifacts: 22 same, 14 CHANGED, 0 errors, 0 "not a variant" warnings, no `gr1-a`. en1990 road-bridge showcase asset is `same` and keeps `gr1a` (6) and `gr1b` (2).
- The 14 CHANGED assets (en1991 2, en1993 2, en1994 3, en1997 3, en1998 4) only add empty collections for new snapshot fields; not applied (owners' call). Output: `🗑️generated/trace-and-tools/patch-check.txt`.

## 3. Schema `$defs`
- en1991 schema: merged `definitions` (26) into `$defs` (now 33), refs `#/definitions/` rewritten, content verified equal. en1990 also used `definitions` (16): renamed to `$defs` the same way, since the dual read is dropped. All other codes already `$defs`.
- `🧰️scripts/showcase/common.py` `defs()` reads only `$defs`; showcase common loads for en1990/1991/1992.
- Twin generate: identical, written=0 (twins are Rust-derived). Strict check: drift=2 only in en1997 `🧬️mutations/🛰️.proto` and `🔗️.graphql` (not from this work; en1997 owner mid-edit, regenerate there).

## Build marks
- ready for rebuild: trace-and-tools (en1990, en1991, en1997 latest.md)
