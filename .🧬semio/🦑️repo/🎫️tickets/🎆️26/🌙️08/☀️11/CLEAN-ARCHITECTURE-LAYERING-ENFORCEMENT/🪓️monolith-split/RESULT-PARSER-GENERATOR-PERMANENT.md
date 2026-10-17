# Result: PARSER-GENERATOR-PERMANENT (prefix `pgp`)

Executor: Sonnet, 2026-10-16 19:05 to about 20:50. No sub-agent, no modifying git command, no ticket tool. Logs are in `🗑️generated/pgp-*`, tools in `pgp-tools/`, pre-edit snapshots of every foreign file I regenerated or fixed in `🗑️generated/pgp-snap/`. Differential sandboxes were built outside the repo (the session scratchpad) and are gone after a restart; every command that rebuilds them is in `pgp-tools/`.

## Verdict

DONE. The TT2 generator (`wp4c-project-exports.py`, 914 lines, SHA-256 `9f153638…c3d4`, a frozen copy is `pgp-tools/pgp-oracle-wp4c-project-exports.py`) is now permanent TypeScript tooling, wired as `bun ./📜️script.ts schema parsers report|apply|check`. It reproduces the ticket script byte for byte on every route TT2 ran, with one deliberate class of departures that fixes a real defect of the Python tool (section 3). The 14 untyped-map parsers are regenerated and verified against Ajv (section 5), and the generator is part of the freshness gate (section 6), which found and now passes three pre-existing defects the Python tool could not see.

| Check | Result |
| --- | --- |
| Byte-identity with the ticket script (5 TT2 routes, hub route, apply-all) | identical trees on every route; apply-all differs only where the Python tool emitted a self-calling parser (section 2) |
| Reproduces the files TT2 generated | 7 of the 9 stdio files and the draw inferences file are byte-identical to the live files; `html` differs by the 6 lines of TT2's documented hand fix; `md` has the same lines in another order |
| Ajv-oracle test, `bun test` | 73 pass, 0 fail, 449 expectations (`parser-projection/🧪️tests/🎯️ajv-oracle`) |
| 14 untyped-map parsers vs Ajv, before | 14 of 14 lenient on map values (12 to 118 accepted bad mutations each) |
| Same parsers, after | 0 lenient on map values, 0 stricter than the schema; only the unknown-key leniency of section 5 remains |
| `tsc` on the 29 touched TypeScript files and their import closure | exit 0, 0 diagnostics |
| Freshness gate `bun ./📜️script.ts schema parsers check` | `fresh: pending=0 dangling=0 drift=0`, exit 0, 20 s |
| Same gate on the tree before my fixes (negative control) | `stale: dangling=1 drift=15`, so it catches drift |

## 1. Where the generator lives and how it is wired

The brief named a `generate` family with `owned-schema` freshness and `check-generated` routes. What the repo has, read from the code:

- `generate` (`📚️library/🎛️commands/🏭️generate`) covers `config`, `taxonomy`, `plugin-glue`, `scale-fixture`; none of them projects a schema.
- The schema-first projection pipeline is the `schema` command family (`📚️library/🎛️commands/🧬️schema/🟦️.ts`, `SchemaScript`: `generate`, `check`, `verify`, `docs`, `compile`, `entries`, `oracle`, `audit`, `test`). It owns the scope catalog and the `schema-*` targets of the root `📋️project.json`.
- Freshness lives in `verify gate` (`🎛️commands/✅️verify/🟦️.ts`): "generated catalog freshness", "framework owned-schema binding freshness" (`@semio-tech/framework-rs:check`), "ui locale/terminology axes freshness". `check-generated` is the per-owner target name of `generatorContracts` rows in `📚️library/🔣️taxonomy.json`.

So the permanent module is a child of the schema command family, following the `verify/🎨️storybook-freshness` precedent: `📚️library/🎛️commands/🧬️schema/🪞️parser-projection/` (all paths at most 189 UTF-8 bytes; `sim.ts` resolves every new folder: `crate-module-children`, `tests`, `members-of-tests`, `fixtures`; 0 unresolved, 0 ambiguous). Files (each starts with a unique-emoji docstring; no comment inside a definition):

| File | Lines | Owns |
| --- | --- | --- |
| `🟦️.ts` | 449 | options, `projectParsers` (two-pass plan, `settle`, import and drop handling), `auditParsers`, `checkParsers`, report rendering |
| `🧾️parser-render/🟦️.ts` | 288 | `parse<Export>` rendering: nullable, `oneOf` (tagged and externally tagged), `allOf`, tuples, maps, cross-document `$ref`, declared-type following |
| `📖️declared-types/🟦️.ts` | 159 | readers of the TypeScript already on disk (interfaces, aliases, declared and imported parsers) |
| `📐️type-render/🟦️.ts` | 166 | GraphQL, protobuf and TypeScript type renderers |
| `🗂️documents/🟦️.ts` | 195 | `SchemaCatalog` (module and document discovery), `SourceStore` (one read per file, CRLF kept), `$id` index, guard prefix |
| `🚪️guard-helpers/🟦️.ts` | 59 | the helper block, generated from the Python tool's own output |
| `🔡️json-text/🟦️.ts` | 74 | Python-compatible JSON text, code-point order, dictionary helpers |
| `🧪️tests/🎯️ajv-oracle/🟦️.ts` + `🧪️tests/🧫️fixtures/🎯️ajv-oracle/🔣️.json` | 109 + 1215 | the Ajv-oracle test and its language-agnostic corpus |

Wiring (the smallest edits to parent-held files):

- `🎛️commands/🧬️schema/🟦️.ts` (library, +37/-2): import, route `parsers`, and one method `parsers()` (Ctrl-C and SIGTERM cancel between modules, progress to stderr every 250 modules, `apply` without `--closing`/`--replace` refuses unless `--all` is given, because the default of the retired tool rewrites 779 files).
- `🎛️commands/✅️verify/🟦️.ts` (library, +2): one freshness step `[verify] schema parser projection freshness…` running `bun repo run workspace:schema-parsers-check`, next to the owned-schema binding step.
- Root `📋️project.json` (+27): `schema-parsers-report`, `schema-parsers-apply` (`--closing --formats 🟦️typescript`), `schema-parsers-check`.
- `📚️library/📦️packages/🟦️typescript` (library, +6 and +9): route `test parser-projection` in `📜️script.ts` (a peer has other uncommitted edits in that file; mine is one six-line block) and target `test-parser-projection` in `📋️project.json`.
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: `▶️schema-parsers-apply` (3_dev, 950.00271), `⚖️schema-parsers-check` (4_gate, 900.008952), `▶️schema-parsers-report` (3_dev, 950.00272) in the `schema-*` block, and `⚖️test-parser-projection📚️library🟦️` (4_gate, 900.04085) between `test-os-source-topology` and `test-path-emoji-statutes`. Both files parse with TypeScript's JSONC parser (`ts.parseConfigFileTextToJson`).
- `package.json` needs nothing: no `schema-*` target is listed there.

Not done, hand-off: a `generatorContracts` row in `📚️library/🔣️taxonomy.json` (parent-held; the outputs are in-place edits of mixed hand-written and projected tracked files, which the preview protocol does not model).

## 2. Byte-identity proof

Method: a copy of every `🧬️schema` leaf of `✏️s` (10 056 files, 58 MB) is cloned twice; the frozen Python oracle runs on one clone (`REPO` derived from its location, 7 levels above the script), the TypeScript module on the other, with the same arguments; stdout rows and the whole trees are compared (`pgp-tools/pgp-diff.sh`, final run `pgp-final-diff.sh`, log `🗑️generated/pgp-final-diff.log`). Only the capped `RUST-ABSENT` excerpt (the first 30 rows) is excluded from the row comparison, because Python lists documents in directory order and TypeScript in code-point order; the counts in the summary line are compared.

| Route | Result |
| --- | --- |
| `apply` (default, all formats, whole `✏️s`, 779 files written) | stdout equal; trees differ in 525 TypeScript leaves, all by the self-call fix below |
| `apply --closing --formats 🟦️typescript --under …/🗄️stdio` from the restored TT2 pre-state (9 files) | trees identical |
| `… --closing --replace Vec3 --under …/🖋️dxf`, `… --replace MdBlockAdded,MdListItemAdded --under …/📝️md` | trees identical |
| `apply --replace DrawingTopology --under …/🖍️draw` | trees identical |
| `apply --replace` of the 14 names (14 files) | trees identical |
| hub `apply --replace SpaceConfig,SpacePresence --under 🌎️hub` | stdout and trees identical |

Reproduction of what TT2 generated (restored pre-state, then the TypeScript module): `csv`, `epw`, `tsv`, `pdf` 1.4, `json` base diff, `dxf` diff, `dxf` snapshot and draw inferences are byte-identical to the live files; `html` differs by the 6 lines of TT2's hand fix of the interfaces (documented in `RESULT-TS-TYPE-ERRORS-2.md`); `md` has the same lines as the live file in another order (the replaced functions were appended at different moments).

Deliberate departures from the Python tool (everything else is a port; the run above proves it):

1. **Self-calling parsers.** For an export whose schema is opaque and whose declared type is the export's own name, Python emitted `export function parseX(value, at) { return parseX(value, …); }`, an infinite recursion. The apply-all run produced 526 of them; the live tree has one (`parseJsonValue` in the `json` base diff leaf, committed in HEAD, so `JsonValueDiff` with `kind: "replace"` overflowed the stack). The root value of an export never resolves to the parser of its own name now (a field of the same type still recurses). Both the TypeScript and the Python trees were type-checked (685 changed leaves, `types: []`, so the `node:` diagnostics are common to both): the TypeScript output has 2 diagnostics more than Python's, both because two hand-declared types are `Record<string, never>` (`WiresDiff`, `RemodelingNoPatch`) and the terminating `GuardObject` result is `Readonly<Record<string, unknown>>`; Python hid the same mismatch behind the recursion. The apply-all output as a whole carries 770 type errors in both trees, which is why `apply` now needs `--all`.
2. `--under` that matches no module is an error (Python wrote nothing and said nothing); mistyped flags are refused.
3. Integers beyond 2^53 stay exact (`18446744073709551615` is printed as such; a plain `JSON.parse` printed `18446744073709552000`).
4. Module discovery lists files through `git ls-files` (tracked and untracked, not ignored) and walks the directories only for a tree that is no work tree; a walk of the live `✏️s` took more than 120 s under load, the listing takes 15 s. Documents and exports are ordered by code point.
5. The index of `$id` to leaf also covers `🌎️hub` (Python indexed `✏️s` only).
6. A file with CRLF line ends keeps them (Python normalised on write).
7. New: `audit` and `check`.

Documented limits (in the module docstring): a schema property named like an array index cannot keep its document order in a JavaScript object, so that export is refused instead of rendered; floats print in JavaScript form (`1.0` as `1`) while no schema of the corpus has such a bound (the apply-all trees are identical); types of cross-document `$ref` targets are not imported, only their parsers.

## 3. The language-agnostic Ajv test

`parser-projection/🧪️tests/🎯️ajv-oracle/🟦️.ts`, run with `bun test` (registered as `bun repo run @semio-tech/repo-lib:test-parser-projection`; not run through the launcher, see "Not verified"). The corpus (`🧫️fixtures/🎯️ajv-oracle/🔣️.json`) is plain JSON: 9 cases (scalars with bounds, objects, nullable in both encodings, tagged and externally tagged unions, tuples, maps, opaque nodes, cross-document references with `allOf`, declared types), 10 documents, 26 exports and 23 probe sets; per export the instances every implementation must accept (`valid`), refuse (`invalid`), the ones the schema admits but the projection refuses (`stricter`) and the ones the schema refuses but the projection tolerates (`lenient`), plus two raw-JSON map probes with `__proto__`. The test writes the documents and their hand-written seeds into a temporary tree, runs `projectParsers` (apply), compiles the sources, type-checks them with `typescript` under `strict`, and compares each parser with an independent Ajv draft-07 validator (`valid`: Ajv admits and the parser returns an equal value; `invalid`: both refuse; `stricter` and `lenient`: pinned in both directions). Result: 73 pass, 0 fail.

What the oracle pinned (all three are properties of the generator, recorded as expectations so a change flips visibly):

- **Stricter than the schema:** integers beyond 2^53 (Ajv admits `1e300` for `{"type": "integer"}`), tuples whose schema states no `minItems`/`maxItems` (Ajv admits shorter and longer arrays), and opaque `{"type": "object"}` / `{"type": "array"}` nodes whose hand-written declared type is richer (the parser follows the type; the schema is the defect, the same class TT2 recorded for `dxf`, `html`, `md`).
- **More lenient than the schema:** unknown properties. `additionalProperties: false` is not enforced by any projected object parser (Ajv refuses, the parser drops the key). This is the only disagreement left after section 5.
- The oracle found the self-calling parser (the `opaque` case hung the first run).

## 4. Foreign files fixed because the generator or the gate showed they were broken

All snapshots are in `🗑️generated/pgp-snap/`.

- **`🪵️sourcing` editor config.** `Filters.sort` pointed to `…/s/sourcing/curation/artifact.json#/$defs/TableSort`, which does not exist, so Ajv could not compile the document (`can't resolve reference`, reproduced on the snapshot) and the leaf called `parseFilters`, which was never declared (a `ReferenceError` at run time). The schema now defines `SortDirection` and `TableSort` locally and `sort` is `anyOf [TableSort, null]` (the leaf and Rust already allow `null`); the permanent tool then projected `parseFilters`, `parseSortDirection`, `parseTableSort` (`--closing`) and the missing protobuf `SortDirection` message (`apply --all --formats 🛰️protobuf`). Verified against Ajv: 148 of 153 mutations agree, the other 5 are unknown-key leniency. Side effect: `schema check` now reports `schema-export-incomplete` for `TableSort` and `SortDirection` in the module's `🦀️.rs` (3 rows before the proto fill, 2 after): the Rust leaf needs `pub use crate::{SortDirection, TableSort};` in its re-export region, which is plugin Rust (not touched).
- **`🧾️json` base diff leaf.** The committed `parseJsonValue` called itself. It now mirrors the snapshot leaf's parser (`parseJsonValue` and a new `parseJsonMember`, with this leaf's guard prefix); 17 of 17 probes agree with Ajv on the snapshot `JsonValue`.
- **`📸️remodel` inference leaf.** The interface `RemodelingInference` lacked `relativeCameraPoses` and `RemodelingBoundingBox.min/max` were `number[]` against `[number, number, number]` (3 TS2322/TS2353 before and after the map fix). The field was added to the interface and `RemodelingBoundingBox` was regenerated; the file now type-checks.

## 5. The 14 untyped-map `💡️inferences` files

Scan: `pgp-tools/pgp-scan-untyped-maps.py` (output `🗑️generated/pgp-untyped-scan.txt`) over `✏️s` and `🌎️hub`. TT2's list names 13 inferences files and two hub configs (and counts 14). The files that really carry the defect (a schema-typed map read by `GuardObject` without per-value parsing) are 14 inferences files in `✏️s` plus the two hub space configs (which are `🧬️schema/🟦️.ts`, not inferences). `Puzzle5dInference` also looked untyped but already is typed (a hand-written loop with `Object.defineProperty`, 0 lenient mutations before); its regeneration was reverted and the gate classifies untyped maps by `Object.entries(` count, so it is not flagged.

Command (live tree, snapshots first): `schema parsers apply --replace FlowTopology,PresentationTopology,ShootingTopology,SequenceTopology,FormsTopology,LayoutTopology,PlaybookTopology,ProcedureTopology,RemodelingInference,RasterTopology,SemioFlowTopology,SemioMeshInference,SemioGraphTopology,Puzzle2dFlatPosition --formats 🟦️typescript` and `… --replace SpaceConfig,SpacePresence --under 🌎️hub`. Each regenerated function is the same function with `Object.fromEntries(Object.entries(…).map(…))` and the value parser (`…GuardInteger`, `parseRemodelingPoseDelta`, `parsePuzzle2dFlatPositionXy`, an inline `{min, max}` parser for the mesh `aabb`, `parseSpaceWindowCamera`); the function moves to the end of the file (replace drops and appends). Ajv differential (`pgp-tools/pgp-verify-replaced.ts`: minimal and full synthesized instance, every single-point mutation to `null`, `"bad"`, `-1`, `0.5`, `true`, `[]`, `{}`, deletion, plus an unknown key):

| Parser | Before: accepted but Ajv refuses (map values) | After: map values | After: agree / cases |
| --- | --- | --- | --- |
| FlowTopology, ProcedureTopology | 14 | 0 | 101 / 103 |
| PresentationTopology, ShootingTopology, SequenceTopology, FormsTopology, LayoutTopology, PlaybookTopology, RasterTopology, SemioFlowTopology, SemioGraphTopology | 12 each | 0 | 101 / 103 |
| RemodelingInference | 78 | 0 | 320 / 328 |
| SemioMeshInference | 118 | 0 | 184 / 184 |
| Puzzle2dFlatPosition | 38 | 0 | 80 / 84 |
| SpaceConfig, SpacePresence (hub) | not measured (hub is outside the sandbox) | 0 | 253 / 257, 158 / 162 |

**Every disagreement that remains** (0 stricter than the schema, 0 on map values): the parser tolerates an unknown property where the schema says `additionalProperties: false` (2 per parser, 8 for `RemodelingInference`, 4 for `Puzzle2dFlatPosition`, `SpaceConfig`, `SpacePresence`, none for `SemioMeshInference`, whose root allows extra keys). It is the generator property of section 3, not a schema bug; enforcing it would change the byte output of every object parser, so it is a hand-off (the oracle pins the current behaviour). No opaque schema node stood behind a stricter parser in these 16 files, so no schema was tightened for them; the opaque `dxf`/`html`/`md` nodes of TT2 are unchanged.

## 6. Freshness gate

`schema parsers check [--strict]` (root target `schema-parsers-check`, step in `verify gate`) over `✏️s` and `🌎️hub` fails when:

1. the closing projection would still write a file (`PENDING`), or a call dangles or needs a parser that cannot be emitted (`DANGLING`);
2. a parser below the helper region differs from the projection of the current schema and reads a schema-typed map as an opaque object (`untyped-map`) or only calls itself (`self-call`, in any parser of the file, exported by the schema or not).

`--strict` gates every difference. Without it the remaining differences are counted, not gated: `ungated-style-drift=379` parsers (89 inline a structure the current renderer calls through `parse<Child>`, 7 differ only in binding names, 17 have no projection at all today, 1 is the hand-written typed map, 265 differ in other ways that I did not classify). Regenerating them is a per-owner decision and was not done (about 200 files owned by others).

Live result: `fresh: pending=0 dangling=0 drift=0`, exit 0 (`bun ./📜️script.ts schema parsers check`, 20 s under load 25). The negative control on the sandbox copy of the tree before my fixes: `dangling=1` (`parseFilters`), `drift=15` (14 untyped maps and the self-calling `parseJsonValue`), `ungated=380`.

The repository-wide `schema check` (18 000 findings, many thousand pre-existing: `schema-export-incomplete` 12 971, `schema-ref-unresolved` 901, …) reports 0 findings for any path under `parser-projection`.

## Commands run (all from `/Users/ueli/Documents/semio`)

1. Baseline of the oracle: copy and SHA of the ticket script; `pgp-make-sandbox.py` (10 056 files).
2. Differentials: `pgp-diff.sh <name> <args>` (many runs during development), the final `pgp-final-diff.sh` (log above), `pgp-selfrec-compare.py` (525 files, 526 self-calling parsers replaced; svg/xml diff nullable wrappers reviewed by hand), `pgp-tsc-outputs.py fa` (685 changed leaves, Python 770 and TypeScript 772 diagnostics).
3. `bun test ./…/🎯️ajv-oracle/🟦️.ts`: 73 pass (log `pgp-ajv-oracle-run3.log`; the first run failed at the type-check of my own corpus seed, the second hung on the self-call).
4. `node --max-old-space-size=8192 node_modules/typescript/bin/tsc -p 🗑️generated/pgp-tsconfig.json --noEmit` over the 29 touched TypeScript files: exit 0.
5. Live: `schema parsers apply --closing` for the sourcing config, `apply --replace` for the 14 and the hub pair, `apply --all --formats 🛰️protobuf` for the sourcing proto; `bun pgp-verify-replaced.ts` before (sandbox) and after (live); `bun pgp-verify-jsonvalue.ts`.
6. `bun ./📜️script.ts schema parsers check|report|apply` (CLI route, including the refusals); `bun ./📜️script.ts schema check --report …` (inventory); the TS-only `sim.ts` for the new folders.
7. `NX_FORCE_REUSE_CACHED_GRAPH=true bun repo run workspace:schema-parsers-check` (launcher route): no verdict. The launcher stayed in the Nx graph phase for the 900 s I allowed and my alarm killed it (`error: script "repo" was terminated by signal SIGALRM`, `pgp-repo-run-check.log`); the same check through `bun ./📜️script.ts schema parsers check` passed in 20 s. A package script run raw is refused (`Original parent Script process invocation required`), so the `test-parser-projection` route can only run through the launcher; the test itself ran under `bun test`.

## Not verified

- The launcher routes `bun repo run workspace:schema-parsers-check` and `bun repo run @semio-tech/repo-lib:test-parser-projection` depend on the Nx graph and were not shown to pass: see command 7. The module, its CLI route and the test were run directly.
- Rust, wasm, `cargo`: nothing was compiled; no `🔌️plugin/**` Rust file was touched.
- The 379 ungated stale parsers were not regenerated or type-checked.
- Windows and Linux were not run (the code uses `node:path` separators converted to `/`, `git` and `node:fs` only).

## Hand-offs

1. `generatorContracts` row for the projection in `📚️library/🔣️taxonomy.json` (parent-held).
2. Enforce `additionalProperties: false` in object parsers (changes every object parser's bytes; the oracle's `lenient` lists show where).
3. The 379 style-drifted parsers: regenerate per owner with `schema parsers apply --replace <Name> --under <module>`, then re-run `check --strict`.
4. Sourcing editor config: `pub use crate::{SortDirection, TableSort};` in its `🦀️.rs` re-export region (plugin Rust).
5. Cross-document type imports in the type renderer (only parser imports are added today).
6. The two `Record<string, never>` opaque exports (`WiresDiff`, `RemodelingNoPatch`) when apply-all mode is ever used.
7. Opaque `dxf`/`html`/`md` schema nodes of TT2 (unchanged).

## Files changed

New: the nine files of `parser-projection` listed in section 1. Edited, library: `🎛️commands/🧬️schema/🟦️.ts`, `🎛️commands/✅️verify/🟦️.ts`, `📦️packages/🟦️typescript/📜️script.ts`, `📦️packages/🟦️typescript/📋️project.json`. Edited, root: `📋️project.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`. Regenerated (`✏️s/🔌️plugins/…/🧬️schema/💡️inferences/🟦️.ts`): 🌊️flow, 🎞️animate (presentation), 🎥️shooting, 🎬️sequence, 📋️forms, 📏️layout, 📖️playbook, 📜️imperative (procedure), 📸️remodel (also the interface and `RemodelingBoundingBox`), 🖨️raster, 🗄️stdio semio flow, mesh and graph, 🧩️puzzle 2d; hub: `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎚️config` and `…/👥️presence` (`🧬️schema/🟦️.ts`). Fixed: `🪵️sourcing/…/✏️editor/🎚️config/🧬️schema/{🔣️.json,🟦️.ts,🛰️.proto}`, `🗄️stdio/…/🧾️json/…/🧱️base/🧬️schema/🔺️diff/🟦️.ts`. Nothing was deleted; no `[DEBUG]` log was added.
