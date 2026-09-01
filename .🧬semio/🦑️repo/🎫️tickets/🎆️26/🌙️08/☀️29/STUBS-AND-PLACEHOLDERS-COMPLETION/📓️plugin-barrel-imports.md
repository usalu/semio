# Plugin barrel imports — 390 broken specifiers repaired

## Scope

16 plugin TypeScript barrels at `✏️s/🔌️plugins/<plugin>/📦️packages/🟦️typescript/📦️index.ts`
each had `export * as <name> from "../../🗿️artifacts/<artifact>/<rest>"` lines missing the
`🏅️standards/🔖️<std>/🪆️subsets/✳️<subset>/` segment, so the specifiers never resolved.

Plugins touched: `📕️norm`, `🧩️puzzle`, `🧱️block`, `🌍️gis`, `🏗️fem`, `🔱️trinity`, `📏️layout`,
`📖️playbook`, `🔋️energy`, `🖨️raster`, `🎥️shooting`, `🏛️architect`, `🏭️process`, `📜️imperative`,
`📸️remodel`, `🪐️space`.

## Method

Wrote a one-off Python script (not committed — lived in the session scratchpad) that, per barrel:

1. Parsed every `export * as NAME from "REL";` line.
2. Resolved `REL` relative to the barrel and checked existence; left already-correct lines
   (`🪐️space`'s `space_index_schema`) untouched.
3. For a broken line, split `REL` into `<artifact>/<rest…>` after the `../../🗿️artifacts/` marker,
   then globbed `‹artifact-dir›/🏅️standards/*/🪆️subsets/*/‹rest…›` on disk for a file that actually
   exists, plus a direct `‹artifact-dir›/‹rest…›` fallback.
4. Exactly one match → rewrote the specifier to that real relative path.
5. Zero matches → dropped the export line and recorded name + reason (see below). No candidate was
   ever invented.
6. More than one match → would have been logged as ambiguous for manual review — **this never
   happened**: every broken specifier in all 16 barrels had exactly one candidate on disk.

Confirmed the intended shape first against two barrels **not** on the broken list
(`🌊️flow`, `✒️writer`), both of which already use `🏅️standards/🔖️1/🪆️subsets/✳️any/…`.

## Result: every fix resolved to `🏅️standards/🔖️1/🪆️subsets/✳️any/`

Checked explicitly: none of the 353 repaired lines resolved to any standard/subset other than
`🔖️1`/`✳️any` — every artifact touched by these 16 barrels only has that one standard+subset on
disk, so there was no real ambiguity to adjudicate.

## Per-plugin counts (fixed / removed / already-ok, out of broken total)

| plugin | fixed | removed | already-ok |
|---|---|---|---|
| 📕️norm | 165 | 15 | 0 |
| 🧩️puzzle | 33 | 3 | 0 |
| 🧱️block | 33 | 3 | 0 |
| 🌍️gis | 22 | 2 | 0 |
| 🏗️fem | 22 | 2 | 0 |
| 🔱️trinity | 22 | 2 | 0 |
| 📏️layout | 11 | 1 | 0 |
| 📖️playbook | 11 | 1 | 0 |
| 🔋️energy | 11 | 1 | 0 |
| 🖨️raster | 11 | 1 | 0 |
| 🎥️shooting | 2 | 1 | 0 |
| 🏛️architect | 2 | 1 | 0 |
| 🏭️process | 2 | 1 | 0 |
| 📜️imperative | 2 | 1 | 0 |
| 📸️remodel | 2 | 1 | 0 |
| 🪐️space | 2 | 1 | 1 |
| **total** | **353** | **37** | **1** |

353 + 37 = 390, matching the ticket's measured broken-line count exactly.

## Every export removed, and why

All 37 removals are the same facet across every artifact in all 16 plugins: a
`*_decomposer` export pointing at `<artifact>/🪓️decomposer/🟦️component.ts`. Evidence the facet
is genuinely unimplemented, not just misfiled:

- `find "✏️s/🔌️plugins" -iname "*decomposer*"` → **zero hits** anywhere in the plugins tree (not
  under the corrected standards/subset path, not under the short pre-fix path, not anywhere else).
- `grep -rl "decomposer" "✏️s/🔌️plugins" --include="*.rs"` → **zero hits**. No Rust component
  implements a decomposer either, so this isn't a TS-only gap.

Removed lines (name — original broken specifier):

- 📕️norm: `iso16757_decomposer`, `vdi3805_decomposer`, `din4108_decomposer`, `din16798_decomposer`,
  `en1990_decomposer`, `en1991_decomposer`, `en1992_decomposer`, `en1993_decomposer`,
  `en1994_decomposer`, `en1995_decomposer`, `en1996_decomposer`, `en1997_decomposer`,
  `en1998_decomposer`, `en1999_decomposer`, `din18599_decomposer` (all `<artifact>/🪓️decomposer/🟦️component.ts`)
- 🧩️puzzle: `puzzle2d_decomposer`, `puzzle5d_decomposer`, `puzzle3d_decomposer`
- 🧱️block: `block2d_decomposer`, `block5d_decomposer`, `block3d_decomposer`
- 🌍️gis: `gisterrain_decomposer`, `gismap_decomposer`
- 🏗️fem: `fem2d_decomposer`, `fem3d_decomposer`
- 🔱️trinity: `rewrite_decomposer`, `jack_decomposer`
- 📏️layout: `layout_decomposer` (matches the example called out in the ticket brief)
- 📖️playbook: `playbook_decomposer`
- 🔋️energy: `model_decomposer`
- 🖨️raster: `raster_decomposer`
- 🎥️shooting: `shooting_decomposer`
- 🏛️architect: `program_decomposer`
- 🏭️process: `process3d_decomposer`
- 📜️imperative: `imperative_decomposer`
- 📸️remodel: `remodel_decomposer`
- 🪐️space: `home_decomposer`

No blank lines were left behind; each barrel's remaining lines are contiguous.

## Ambiguity

None encountered — see "Result" above. The script's ambiguous-candidate branch never triggered for
any of the 16 barrels.

## 🪐️space — the pre-fixed line

`space_index_schema` (`../../🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️component.ts`)
was already correct and left byte-for-byte untouched. The barrel also carries a pre-existing
comment (not written by this pass) noting the `home_*` lines predate a "per-subset surface
migration" — that comment is now stale in one respect: `home_schema`/`home_io` are fixed and
resolve; only the removed `home_decomposer` remains a genuine gap. Left the comment as-is since
editing unrelated prose wasn't in scope for this repair.

## Verification (actually run)

**1. Existence sweep over every barrel in the plugins tree** (not just the 16), script written to
scratchpad, checked every `export * as … from "…";` line's resolved target:

```
GRAND TOTAL export lines: 502, broken: 0
```

All 41 discovered barrels (`find … -iname "*index.ts*"`) report zero broken specifiers, including
the ones never touched by this pass (`🌊️flow`, `✒️writer`, `📐️cad`, etc.) — confirming nothing
outside the 16 target files was disturbed.

**2. `bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler
--esModuleInterop --skipLibCheck --allowImportingTsExtensions` on each of the 16 barrels**,
individually, output captured in full:

- Errors by code: `208× TS2307`, `158× TS2304`, `134× TS2374`, `2× TS2552`. Total 502 error lines.
- **Zero of these errors are located in a barrel `📦️index.ts` file itself** — confirmed with
  `grep "📦️index.ts(" tsc_output.txt` → no matches. Every barrel's own `export * as … from "…"`
  specifiers type-check clean; tsc successfully loaded every one of the 502 resolved modules.
- All reported errors are **downstream**, inside the artifact `🟦️component.ts` source files the
  barrels now correctly point at:
  - `TS2307` (`Cannot find module`): downstream component files import per-mutation TS stubs by
    relative path (e.g. `block/◻2d/…/🧬️mutations/🟦️component.ts` imports
    `./✏️rename-node-kind/🦠️mutation/🟦️component`) whose `.rs`/`.json` exist but the TS file itself
    was never written — a pre-existing missing-implementation gap in those mutation leaves, not a
    barrel-path problem, and out of this ticket's scope (this repo is mid-flight on exactly that
    kind of stub completion elsewhere, per this same ticket's other logs).
  - `TS2304`/`TS2552` (`Cannot find name`): undefined type names inside `norm`/`fem`/`architect`
    schema/diff component files (e.g. `qK`, `En1995Artifact`, `FemElement`) — pre-existing content
    bugs in those files, unrelated to import paths.
  - `TS2374` (duplicate index signature): pre-existing type-authoring bug inside `puzzle`'s
    `🧊️3d`/`🖐️5d` diff component files.
  - None of these match the ticket's called-out known-noise categories (`import.meta.env/glob`,
    `TS17004`/`TS5097`) because those come from `🎨️styling`/`.tsx` files, which none of these 16
    barrels touch — but they are the same class of thing: real, but pre-existing and out of scope.

**3. `git status --porcelain -- <16 files> | wc -l` → 16`**, one line per file, all `M` (modified),
nothing else. Confirmed scope is exactly the 16 barrels this pass edited.

## Unfinished / out of scope

- The 37 `*_decomposer` facades are gone, not implemented. If the decomposer facet is wanted, that
  is new Rust+TS authoring, not a path fix, and wasn't attempted here.
- The downstream TS2307/TS2304/TS2374 errors surfaced by the tsc run above point at real
  unfinished stubs elsewhere in the tree (per-mutation TS files, undefined schema type names,
  duplicate index signatures). Not touched — out of this ticket's stated scope (barrel import
  paths only) and likely already tracked by this ticket's sibling logs
  (`📓️wasm-facade-wiring.md`, `📓️final-stub-leaves.md`, `📓️stub-census.md`).
