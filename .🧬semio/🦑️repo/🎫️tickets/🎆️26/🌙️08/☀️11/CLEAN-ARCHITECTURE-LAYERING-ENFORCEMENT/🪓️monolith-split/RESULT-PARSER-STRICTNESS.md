# Result: PARSER-STRICTNESS (prefix `pst`)

Executor: Sonnet, 2026-10-16 20:45 to about 23:00. No sub-agent, no modifying git command, no ticket tool. Logs are in `🗑️generated/pst-*`, tools in `pst-tools/`, pre-edit snapshots of every foreign file I changed in `🗑️generated/pst-snap/` (named by hash, mapped by `manifest.json`, one entry per edit round and label). Nothing was deleted.

`PP` below is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection`.

## Verdict

DONE for the three tasks, with a registry of 177 parsers that no schema can own yet (task 2) and two limits stated under "Not verified".

| Check | Result |
| --- | --- |
| Generated parsers refuse an unknown key exactly where the schema says `additionalProperties: false` | yes; 340 existing parsers restated for it (336 in `✏️s`, 4 in `🌎️hub`, 136 leaves), every later one is born closed |
| Ajv-oracle test (`bun test PP/🧪️tests/🎯️ajv-oracle`) | 156 pass, 0 fail, 710 expectations (was 73 pass, 449); corpus 10 cases, unknown-key cases included, `lenient` category removed |
| Ajv differential over the real schemas, all 836 generated parsers outside the registry | 59 454 single-point mutations, 59 426 agree; the 28 left are listed in section 3 |
| `schema parsers check` over `✏️s` and `🌎️hub` | `fresh: pending=0 dangling=0 helperless=0 drift=0 orphans=0 unowned=177`, exit 0, 36 s; every drift is gated now, not only three classes |
| Consumer tests, all 573 bun test files under `✏️s` and `🌎️hub` (per test, junit) | before 4855 pass / 508 fail, after 4857 pass / 506 fail; 0 newly failing, 2 no longer failing (both unrelated to a parser I regenerated, I do not claim them) |
| `tsc` on the 86 migrated roots | 32 diagnostics before, 13 after, 0 new (19 pre-existing ones are gone because the migrated parser now matches the declared type) |
| `tsc` on all 191 touched leaves | 14 diagnostics; 12 are in both earlier baselines and 2 are older ones outside the baseline roots (section 4) |
| `tsc` on the 12 generator files | 0 diagnostics; negative control (an injected type error) is reported, so the run is live |
| Rust re-export | `pub use crate::{Filters, SortDirection, TableSort};` added; stub-layer oracle `NEW 0`, negative control `NEW 1`; `schema check` no longer reports the two findings |

## 1. Strictness (task 1)

### Design

The refusal is data-driven by the schema and nothing else:

- A new helper `<prefix>Closed(row, at, keys)` is part of the helper block (`PP/🚪️guard-helpers`). It throws `<at>.<key>: unknown property` for every own key outside `keys` and returns the row. A file that predates it receives the helper once, in front of the array reader (`withClosedHelper`); the gate fails with `HELPERLESS` when a parser calls it and no block defines it.
- `PP/🧾️parser-render`: an object node with `additionalProperties: false` is read as `Closed(Object(x, at), at, [keys])` at the root of an export, in nested inline objects, in map values, in array items and in tuple members. A tagged union closes each variant that is closed (`case "k": Closed(row, at, [...]); return {...}`), an externally tagged union closes each single-field variant, and an open object (absent or `true`) stays open and drops the key. A closed object with no properties is `Closed(..., [])` (only `{}` passes). A closed variant that also states `allOf`, and `patternProperties` next to `additionalProperties: false`, are refused as unrenderable instead of being rendered wrongly (0 occurrences today).
- A closed object is no longer "opaque" (`{ "type": "object", "additionalProperties": false }` states its structure: nothing), so the declared type no longer overrides it.
- A tuple `items` list in the schema wins over the declared tuple alias (it states `integer` where the alias only says `number`; this removed 14 lenient mutations of the dxf group codes).

Departures from the ticket tool and from the previous TypeScript module, each one needed by this slice (the oracle pins them):

1. `restate` (new flag `--restate open,stale,untyped-map,self-call | all`, root target `schema-parsers-restate`) re-projects every parser the audit finds drifted for the named reasons. A regenerated or replaced parser **keeps its place in the file** (`parserRange`, a splice that refuses when another top-level declaration starts before the closing brace); before, `--replace` dropped the function and appended it.
2. The audit has a new reason `open` (the parser equals the previous projection and only lacks the refusal). `--strict` is gone: `check` gates every drift except the registry of section 2; a listed parser that no longer drifts fails the gate (`ORPHAN`).
3. Declared types: `declaredOptionalFields` also reads one-line interfaces (one-line `export interface X { a: T; b?: U }` declarations were invisible to it, among them the zip, html, binary, stl, obj diff and semio drawing leaves). An optional nullable field whose declared type has no top-level `null` collapses `null` to absent (`== null ? undefined : ...`, what the hand parsers did; 180 of the first 260 migration diagnostics). A fixed-size array (`minItems == maxItems`, 1 to 16) is read as a tuple. A property with a schema `default` that the declared type requires is filled when absent (`row[f] === undefined ? <default> : ...`); a property the declared type marks optional is not.
4. `export { local as parseX }` and `export type { A as X }` count as declared, and a call uses the local binding (`reexportedBinding`). Without this the first restate wrote duplicate `parsePoint3` and friends into the semio-drawing snapshot leaf and broke 4 test files; I removed that block by hand from my own snapshot and fixed the generator.
5. Integers beyond 2^53, CRLF and the `__proto__` key are unchanged; the corpus now has four raw probes with `__proto__` on closed, open and empty objects.

### The Ajv oracle

Corpus `PP/🧪️tests/🧫️fixtures/🎯️ajv-oracle/🔣️.json` (language-agnostic JSON): the four `lenient` entries moved to `invalid`; new case `closed` with 13 exports (closed, open, `additionalProperties: true`, empty closed, nested closed in object, array item and map value, nullable by reference and inline, tuple member, tagged union with a closed and an open variant, externally tagged union, reference holder, defaults) and 4 raw probes. A new category `tolerated` pins the instances both Ajv and the parser admit although they carry an undeclared key (only where the schema leaves the object open), `defaulted` pins input and output of default filling. Test file `PP/🧪️tests/🎯️ajv-oracle/🟦️.ts` (232 lines): the existing 73 tests, plus: the audit finds no drift or missing helper in the projected sources, a second projection with every restate reason rewrites nothing, an older file (helper block without `Closed`, open parser) is restated in place, receives the helper once, agrees with Ajv and type-checks under `strict`, and the gate test (drift fails, a listed parser passes with `unowned=1`, a listed parser that no longer drifts fails, restate repairs it).

### Regeneration and the real schemas

`schema parsers apply --closing --formats 🟦️typescript --restate open` (ticket log `pst-apply-open*.log`): 336 parsers in 134 leaves of `✏️s` plus 4 in 2 leaves of `🌎️hub`, +927 / -340 lines (the rest is the helper, 4 lines per leaf). Ajv differential of those 340 (`pst-tools/pst-verify.ts`, synthesizes a minimal and a full instance from the schema and mutates every position to `null`, `"bad"`, `-1`, `0.5`, `true`, `[]`, `{}`, deleted, plus an unknown key): 28 524 cases, 28 458 agree, 4 stricter (VcsTagsDelta, fixed in section 2), 62 lenient on unknown keys, 0 lenient otherwise. The ones I inspected (topology and bounds parsers, bounding boxes, the vcs delta, the draw layer address, obj face vertices) delegate to a hand-written parser that was still open at that time (`$ref` to a `stale` export such as `EquationTopology`); after the migration of section 2, 19 of that kind are left on the final population, and they delegate into the registry (section 3).

### Consumer tests and the real-data question

Run: `pst-tools/pst-run-tests.sh` (per file `bun test --reporter=junit`, 5 in parallel, 240 s cap), baseline taken before the first edit (`pst-tests-before2`, 573 files, 4855 pass, 508 fail, 36 files that do not load). No log of any run contains `unknown property`: **no test feeds a closed parser a document with a key its schema does not declare**, and no real fixture or example parsed by a test carries one. I did not find a producer or schema to fix for this class. I did not scan JSON data files that no test parses (section "Not verified").

Every other failure I met while migrating was a real finding about a hand parser doing more or something else than its schema, recorded in section 2.

## 2. The 379 older parsers (task 2)

Count at the start: the previous slice reported 379 ungated parsers (362 stale, 17 unrenderable). The robust `parserRange` of this slice also sees one-line functions the old regular expression skipped, so the audit starts at **398** (379 stale, 19 unrenderable). Now: **221 are generated and gated, 177 are hand-written and listed in a registry**, and `check` is fresh.

### How

`schema parsers apply --closing --formats 🟦️typescript --restate all`, followed by a type-check and the tests of every round, looped: the parsers that failed `tsc` or a test went into the registry and the next round regenerated the rest (rounds `pst-apply-stale`, `restate3` to `restate12` in `pst-snap/manifest.json`). A parser stays hand-written only with a reason the registry explains.

### The registry

`PP/🧱️unowned/🔣️.json` (data) with `PP/🧱️unowned/🟦️.ts` (loader and validation: a row without an explained reason is refused). The gate fails on a drifted parser that is not listed and on a listed parser that does not drift; `restate` skips listed parsers. 177 rows in 35 leaves:

| Reason | Rows | Where | Why |
| --- | --- | --- | --- |
| `in-memory-model` | 148 | architect 71, puzzle 3d and 5d diffs 21, obj 16, draw 10, lowpoly 9, raster 8, semio drawing 6, xml 2, stl 2, ply 2, pdf 1 | the parser validates the owned in-memory model (exact Binary64 and Binary32 words as `{ bits: bigint }`, bigint counters, `Uint8Array`), the schema states the JSON transport (plain numbers, decimal strings); regenerating it changes what it admits and fails `tsc` or the tests. `parseBinary64` takes the bigint word, the schema `Binary64` is 16 hex digits. Precondition: the schema states the in-memory model through a binding to the framework value parsers |
| `unrenderable` | 19 | raster 5, lowpoly 3, shooting 2, semio (stdio) 2, architect, sequence, draw, svg, xml, tiff, ply 1 each | no renderer covers the schema form, or the parser is generic over type arguments (`IndexedTripleDiff<A, B>`) |
| `schema-disagrees-with-type` | 5 | html node, svg diff (2), semio audio diff, semio text artifact | schema and declared type differ (the audio schema is a stub with `schema` and `bytes` against `sampleRate`, `format`, `channels`, `tags`; the html node is a union in TypeScript and one object with optional fields in the schema) |
| `iterative-walk` | 2 | draw `DrawingLayerNode` and the semio drawing `DrawNode` | explicit stack for unbounded trees ("deep trees" tests); the generator only recurses |
| `normalizing-reader` | 2 | zip `ZipSnapshot`, `ZipEntry` | fills defaults for fields the schema requires (and builds `defaultZipEntryMetadata()`); `supplies the shared empty archive defaults before editing` depends on it |
| `cross-field-rule` | 1 | raster `RasterConfig` | refuses a selection whose target differs from the paint target; the schema has no such rule |

### Schema changes ("the schema is incomplete, tighten it, then regenerate")

All JSON edits keep the file's formatting (round-trip checked, `pst-tools/pst-schema-edit.py`) and have a snapshot.

- `activeUtilityId` (required string, `x-semio-state: presence`) added to the 8 presence schemas of process3d, lowpoly, cad, raster, note and puzzle 2d, 3d and 5d. The TypeScript interface, the GraphQL and the hand parser all required it; the JSON schema did not state it. **Not changed: the Rust presence structs of these modules do not have the field** (serde ignores it on input, so nothing breaks; the Rust owners decide whether to add it).
- `required` added: `VcsTagsDelta` (`removed`, `inserted`, `moved`), `BinarySnapshot` (`bytes`, `schema`), `DxfTables` (`layers`, `styles`, `linetypes`); in each the declared type and the parser already required them.
- `default` added where the hand parser filled a value for a field the declared type requires and the schema left optional: shooting `ShootingMaterial.stroke` (`"#000000"`), `ZipEntriesDiff` (`removed`, `modified`, `added`: `[]`), xml `XmlDocument` (`prolog`, `epilog`: `[]`), stl `StlArtifact` (`solidName`: `""`, `triangles`: `[]`), obj diff (18 arrays: `[]`), puzzle 2d `Puzzle2dArtifact.targetRegions` (`[]`) and `Puzzle2dTargetRegion` (`hidden`, `locked`: `false`), home config `retiredLocalStudioIds` (`[]`), semio drawing snapshot (`styles`, `layers`: `[]`). A default is an annotation: validation by Ajv is unchanged.
- I reverted every `required` I first added for draw, zip, obj, stl, xml, semio drawing (restored from my snapshots) once the tests showed the hand parser was the lenient one (zip) or an in-memory parser (the rest).

### What the migration found in the old parsers

- Hand parsers that accept more than their schema (the schema is stricter): zip snapshot (defaults for required fields), raster config (cross-field rule). Registered, not changed.
- Hand parsers that do more than the schema: the 8 presence fields above, the 4 `required` sets, the 13 default sets.
- 19 pre-existing `tsc` diagnostics of the 86 roots disappeared (fem and ifc inference tuples, zip and binary optionals, shooting patches).
- Generator defects found by the migration and fixed: duplicate declarations over re-exports (item 4 above), tuple and null mismatches (item 3), the declared tuple alias that dropped `integer`, one-line interfaces.

Not migrated and not in the registry: nothing in the audited region (the gate proves it). **Outside the gate:** 2 480 exports declare a parser, the audit covers 1 013 of them (836 generated, 177 listed). The other 1 467 (190 leaves) are `export function` parsers in leaves without a helper region, `const`/re-export forms, or functions above the region. They are hand-written or re-exports, were never reported by the generator and are not part of the 379.

## 3. Residual disagreements of the generated parsers with Ajv (final differential)

836 parsers, 59 454 cases, 59 426 agree; 333 parsers had an instance the harness could not synthesize and only get the cases it could build (503 are fully exercised).

| Count | Kind | Cause |
| --- | --- | --- |
| 19 | lenient on an unknown key | inside a registry parser the generated parser delegates to (`DrawingLayerRelocation.from`, `Svg*.value`, `Obj*Modified.diff`) |
| 4 | lenient (deleted `data` or `metadata`) | `ZipDiff`, `ZipEntriesDiff` delegate to the registry `ZipEntry` (normalizing reader) |
| 5 | parser stricter than schema | `HtmlArtifact` and `BinaryArtifact` (generated, older than this slice): the schema states `schema` and `entries` or `bytes: string`, the declared types `schema`, `doctype`, `root` or `bytes: number[]` |

## 4. Rust re-exports (task 3)

`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs`: `pub use crate::Filters;` became `pub use crate::{Filters, SortDirection, TableSort};`. Both names are `pub` at the crate root (`🗂️curation/🦀️.rs` lines 105 and 113), the precedent is the `Filters` line itself. These were the two `schema-export-incomplete` findings that the generator's protobuf fill created in the previous slice.

- Static: `python3 pos-tools/pos.py run pst-tools/pst-sourcing.json --cfgs lib,test --label pst-sourcing` (union of `semio-s-artifact-sourcing-curation`, base = `b84bf526c7b`): lib 2 350 errors in both, `NEW 0 GONE 0`; test 2 in both, `NEW 0`; the typeck canary (E0308, E0616, E0624) is present on both sides. Negative control (`--mutate` of the same line to `SortDirectionX`): `NEW 1`. The 2 350 errors are stub-layer noise common to both sides.
- `rustfmt +nightly-2026-07-20 --edition 2021 --check` on the leaf: my line is clean; the one diff it reports is the older `FacetLeaves` literal at line 24 (not mine).
- `bun ./📜️script.ts schema check --report …`: no finding left for the module's `TableSort` or `SortDirection`; the total went from 17 998 (after the edit) to 17 991 at the end, 14 findings gone and 7 new ones in framework and stdio registry directories that I did not touch.
- Real check, for the compile queue (not run; headroom is refused and the fleet mutex applies):
  `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-artifact-sourcing-curation --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/Cargo.toml`

## Edits to parent-held files (minimal, each recorded)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts`: `apply` also accepts `--restate`, the message names it, `check` no longer passes `strict`, the docstring says so (4 lines).
- Root `📋️project.json`: target `schema-parsers-restate` (`apply --closing --formats 🟦️typescript --restate all`); `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: `▶️schema-parsers-restate` (3_dev, 950.002715) between apply and report. Both launch files parse with TypeScript's JSONC parser, `📋️project.json` with `JSON.parse`. `package.json` needs nothing.
- `PP/🟦️.ts` is 523 lines, `🧾️parser-render` 329, `📖️declared-types` 234, `🧱️unowned/🟦️.ts` 37 plus the registry (897 lines of data). The new folder `🧱️unowned` resolves in the real taxonomy matcher as `crate-module-children` (`tx3-tools/sim.ts`: 19 directories, 0 unresolved, 0 ambiguous, longest path 176 bytes).

## Files changed

- Generator: `PP/🟦️.ts`, `PP/🧾️parser-render/🟦️.ts`, `PP/📖️declared-types/🟦️.ts`, `PP/🚪️guard-helpers/🟦️.ts`, `PP/🧪️tests/🎯️ajv-oracle/🟦️.ts`, `PP/🧪️tests/🧫️fixtures/🎯️ajv-oracle/🔣️.json`; new `PP/🧱️unowned/🟦️.ts` and `PP/🧱️unowned/🔣️.json`.
- Regenerated leaves: 181 TypeScript leaves under `✏️s/🔌️plugins/**/🧬️schema/**` and 2 under `🌎️hub/🧩️compositions/🪐️space/…/🧬️schema` (full list: the manifest, label `open`, `stale`, `restate3` to `restate12`).
- Schema documents: 8 presence, vcs diff, binary snapshot, dxf snapshot, shooting, zip diff, xml snapshot, stl, obj diff, puzzle 2d, home config, semio drawing snapshot (section 2).
- Rust: the one re-export line. Root: `📋️project.json`, the two launch files. Parent: `🧬️schema/🟦️.ts`.
- Tools (`pst-tools/`): `pst-verify.ts` (Ajv differential), `pst-run-tests.sh`, `pst-collect.py`, `pst-compare.py` (consumer tests), `pst-snap.py`, `pst-restore.py`, `pst-restore-parser.py`, `pst-schema-edit.py`, `pst-registry.py`, `pst-tsc-compare.py`, `pst-tsc-detail.py`, `pst-badparsers.py`, `pst-required-fix.py`, `pst-list-generated.ts`, `pst-unaudited.ts`, `pst-audit-dump.ts`, `pst-sourcing.json`, and a few smaller ones.

## Commands run (all from `/Users/ueli/Documents/semio`)

1. Baselines: `bun ./📜️script.ts schema parsers check [--strict]`; `pst-run-tests.sh` over the 573 bun test files; `tsc -p 🗑️generated/pst-tsconfig-drift.json` (86 roots, 12 GB heap).
2. Generator: `bun test PP/🧪️tests/🎯️ajv-oracle/🟦️.ts` after each change (final 156 pass); `tsc -p 🗑️generated/pst-tsconfig-gen.json` (12 files, exit 0); `tsc` with a deliberate error (reported).
3. Regeneration: `bun ./📜️script.ts schema parsers report|apply --closing --formats 🟦️typescript --restate open|stale|all` (the last apply restated 11 parsers in 6 leaves).
4. After every round: `tsc -p …`, `pst-tsc-compare.py`, the consumer tests (rounds `stale`, `mig`, `mig2`, `final`), `pst-verify.ts` on the restated rows, `bun ./📜️script.ts schema parsers check`.
5. Rust: `rustfmt --check`, `pos.py run … --cfgs lib,test` and the negative control, `bun ./📜️script.ts schema check --report …` (three times).
6. Refusals: `schema parsers apply` without flags (refused with the new message), `check --strict` (unknown argument).

## Not verified

- The launcher routes (`bun repo run workspace:schema-parsers-check|restate`, `…:test-parser-projection`): the Nx graph phase takes minutes under this load and was not run; the CLI route and `bun test` were.
- The 112 vitest files under `✏️s` and `🌎️hub` were not run (no baseline); nothing in them was known to import a regenerated leaf directly, but I did not prove it.
- JSON data files that no test parses (examples and fixtures outside tests) were not validated against the closed parsers; the Ajv differential proves the parsers equal the schemas, not that all data does.
- The 1 467 declared parsers outside the audited region were not looked at.
- Rust, wasm, cargo: not compiled. Windows and Linux: not run (the code uses `node:path`, `git` and `node:fs`).

## Hand-offs

1. Extend the audit to the 1 467 declared parsers outside the helper region, then migrate or register them (the registry mechanism already supports it).
2. The 148 `in-memory-model` rows: a schema binding that states the owned in-memory words (annotation on the framework `Binary64`, `Binary32`, `U64` definitions naming `@semio-tech/framework/value/ieee754` and the one-argument call) would let the generator emit them; it also decides whether the architect, obj, stl, ply, pdf, lowpoly, raster, draw schemas keep describing the transport.
3. Rust presence structs of process3d, lowpoly, cad, raster, note and puzzle 2d/3d/5d lack `activeUtilityId`.
4. Schema against declared type: html node (union), audio diff (stub schema), text artifact (`runs` opaque), `HtmlArtifact`, `BinaryArtifact`.
5. Zip: decide whether the schema stops requiring the fields its normalizing reader defaults; raster: state the selection rule in the schema or keep it in code.
6. `generatorContracts` row in `📚️library/🔣️taxonomy.json` (open from the previous slice).
