# Result rg-plug (plugin crate splitter, brief-1019)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command. `C` = this folder. Tools in `C/rgplug-tools/` (`rgp.py` helpers, `split_flat.py`, `split_nested.py`, `hoist.py`, `peel_json.py`, `peel_chunk.py`, `depgraph.py`, `gate.ts` single-crate canonical gate, `allclosure.ts`, `owners.ts`, `chk.sh`, `sweep.sh`, `specs/*.json`). First-touch backups of every modified file: `C/🗑️generated/rgplug-x/backup/` (hash named, `ledger.json`). Registries touched per new crate: plugin workspace `Cargo.toml` (member + `[workspace.dependencies.*]`), `📋️build-owners.json` (via `reconcileBuildOwnerRegistry`, scoped), artifact `📜️script.ts` package lists (energy, pdf, puzzle 5d). `Cargo.lock` files are NOT hand edited (stale locks, `update --workspace` needed, see Open items).

Gate = canonical `readRustSourceClosureV1` (host cfg, and `+test`), ratio = max(files/32, bytes/262144, lines/4000); admission refuses only when strictly above 1.0.

## Verdict table

| Crate (before prod / test) | After | Method |
| --- | --- | --- |
| energy-model-mut-window-layer (1.344) | window-layer 0.594 (19 f), new `-window-layer-optics` 0.781 (25 f) | split_flat: 8 radiative-property mutations to a sibling crate root `🧬️mutations/🌈️window-layer-optics` (leaf dirs stay, mounted by `#[path]`) |
| energy-model-mut-pv-system (1.156) | pv-system 0.594, new `-pv-system-performance` 0.594 | same, 6 electrical/thermal performance mutations |
| fem-3d-mesh (1.063 / 3.531) | mesh 0.219 / 0.906 (node change + selection move), new `-mesh-sections` 0.406 / 1.000, `-mesh-solids` 0.406 / 1.000, `-mesh-elements` 0.125 / 0.719 | split_nested: leaf mutation groups to new crates; importers re-pointed (crate ident only, module chain unchanged) |
| fem-2d-mesh (1.063 / 3.531) | mesh 0.219 / 0.906, new `-mesh-sections`, `-mesh-regions` (both 0.406 / 1.000), `-mesh-elements` 0.125 / 0.719 | same |
| fem-3d (1.050 / 1.186) | 0.580 / 0.717, new `-diff-document` 0.470 | the 123 KB diff JSON Schema document (`include_str!`) moved to its own crate |
| fem-2d (0.986 / 1.118) | 0.559 / 0.691, new `-diff-document` | same |
| puzzle-5d-puzzle5d (1.274 / 1.393) | 0.867 / 0.867 (3,453 lines) | see section Puzzle5d; plus new `semio-s-artifact-puzzle-5d-testkit` (0.119) |
| cad-cad-play (0.928) | 0.052, new `-cad-play-model-document` 0.877 | JSON document crate |
| stdio-dwg-read-objects (0.926) | 0.596, new `-dwg-read-objects-dynamic-blocks` 0.334 | hoist.py: blocks + associativity decoders (no upward references) |
| stdio-dwg-sqlite-objects (0.934) | 0.612, new `-dwg-sqlite-entities` 0.327 | entities + dimension + viewport codecs; reader/number/color imported privately from the objects crate |
| stdio-pdf-snapshot (0.906) | 0.469, new `-pdf-snapshot-surfaces` 0.469 | peel_chunk: editor and viewer config/presence schema mounts |
| stdio-pdf-diff (0.906) | 0.313, new `-pdf-diff-surfaces` 0.625 | peel_chunk: viewer mounts |

Not split (all inside the gate, reasons verified with `depgraph.py`/`modgraph.py`): docx-base-mutations (0.938), semio-value-mutations (0.906), puzzle-5d-mut-any (0.938): mutation leaves implement `MutationKind<Snapshot, AggregateEnum>` (concrete aggregate, not the generic `Op` of the energy leaves), so leaves cannot sit below the enum; docx xml-address/edit-rules/opc-layer use `DxfMutation`-style upward references. dxf-diff (0.923): clean layering (records below, codecs above) but every consumer imports the `schema::diff` namespace as one module, so a split needs a facade or a rewrite of all diff users. draw-drawing-editor (0.919), puzzle-5d-dispatch (0.941), puzzle-5d-store-preparation, puzzle-2d-retained, forms-forms-diff, stdio-tiff-snapshot, stdio-docx-base-model (0.906 each): children use `use super::*` / `crate::` into the app root namespace (play-app type cycle) or mirror mounts of real code; gismap-demo-asset (0.930): a single 243 KB DSL document, nothing to split.

## Puzzle5d details (pre-existing defects fixed on the way)

The crate at HEAD (and after the file split by split-s) carried remnants of the earlier carve; all removed because they duplicated live code of lower crates (verified by method-set comparison):
- `🍓️paste-job`, `🥦️import-job` (735 lines): byte-equivalent copies of the `impl Puzzle5dPasteJob/Puzzle5dImportJob` blocks in `🖌️session/*` (E0592 duplicates).
- `🥐️retained-command-works` (275 lines): inherent impls on types defined in the `gestures`/`kit` crates (E0116) plus a copy of `PUZZLE5D_RETAINED_TOOL_IDS` that `interaction` already defines.
- `🌽️command-variants` (171 lines): a second `Puzzle5dCommand` enum from the same macro as the `interaction` crate (the root never imported it any more).
- duplicate `pub struct Puzzle5dPlayApp` and duplicate `context_menu_body` (kept the `entities::PUZZLE5D_WORLD_WINDOW_KIND_ID` variant in `🔧️play-app-retained-commands-and-tool-proofs`), local `mod kit;` that mounted the same file as the `retained` crate.
- package root gained the missing `extern crate … as protocol;` and `as value_derive;` aliases; the dead `testkit` feature was dropped.
- `🧪️tests/🧰️testkit` became crate `semio-s-artifact-puzzle-5d-testkit` (dev-dependency of the three mirror crates, mirrors re-export it as `testkit` and `puzzle5d_app_manifest_for_tests`).
Not done: moving `create_puzzle5d_app` (393 lines) above the app; not needed for the gate.

## Verification

- Canonical gate before/after: numbers in the table (`gate.ts`, `gateall.sh`).
- Static resolve-level sweep (plugfix `sweep.py` + `classify.py`, rustc metadata on sandbox copies, externals neutralised) on every touched crate: no new DUP/UNDECLARED/MISSING of mine (remaining findings are pre-existing: fem-3d aggregator `change_solid` path through a private `use`, editor `edit` alias, puzzle5d externally-defined module members).
- `rustfmt` not run. Cargo checks: see Open items (not green yet at report time unless stated below).

## Open items

- Cargo verification blocked for most of the session: peers rewrote framework manifests (`semio-framework-plugin-tool-run`, `value-persistent` missing) and the lock needed `update --workspace` before every `check --locked`; use `C/rgplug-tools/chk.sh <workspace Cargo.toml> -p … --lib --tests`.
- Locks to refresh online: energy model, fem 3d, fem 2d, stdio pdf, stdio dwg, cad, puzzle 5d workspaces, `🌎️hub/Cargo.lock`, `🔋️energy/🏭️bridge/Cargo.lock`.
- Crates to compile (`--lib --tests`): energy-model-mut-{window-layer,window-layer-optics,pv-system,pv-system-performance}, -mutations, -mutation-fixtures, -editor; fem-{3d,2d}-mesh{,-sections,-solids,-regions,-elements}, fem-{3d,2d}-diff-document, fem-{3d,2d}, fem-{3d,2d}-mutations and editor/commands/interaction; cad-cad-play{,-model-document}; stdio-dwg-read-objects{,-dynamic-blocks}, stdio-dwg-sqlite-{objects,entities} and the dwg snapshot crate; stdio-pdf{,-snapshot,-snapshot-surfaces,-diff,-diff-surfaces}; puzzle-5d-puzzle5d, -testkit, and the three mirror crates (artifact `semio-s-artifact-puzzle-5d`, host-admission-tests, v1-unit-tests).
- Test-cfg overflow outside my scope: `semio-s-artifact-fem-3d-mutations` test closure 1.625 (52 files).
- fem `-mesh-sections/-solids/-regions` test closures sit at exactly 32 files (1.000).
