# Lowpoly TypeScript package — decomposer, IO facet, core.ts, stub audit

## Bug #1 — `lowpoly_decomposer` export: REMOVED (not implemented)

Evidence:
- `find "$LP" -iname "*decomposer*"` returns nothing — lowpoly has no `🪓️decomposer` facet dir on disk.
- The `🪓️decomposer` (and `🏗️builder`) facets are **soft-required** in the taxonomy validator
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1113-1120`, comment: *"Soft-require
  builder/decomposer until W5/W6 migrate every artifact"*) — they are only checked **if the dir exists**. Absence is
  not a violation.
- The reference plugin the ticket points to (cad) is fully migrated and its `index.ts` does **not** export a
  `cad_decomposer` at all — confirming "no facet on disk → no export" is the house pattern, not an oversight.
- This is a repo-wide latent bug, not lowpoly-specific: `raster`, `remodel`, `rewrite`, `jack` (trinity) all have the
  same eagerly-exported-but-nonexistent `🪓️decomposer` path in their `index.ts`. Out of scope here (not my exclusive
  files) — worth a follow-up ticket, flagged separately.

Fix: removed the `export * as lowpoly_decomposer from ".../🪓️decomposer/🟦️component.ts"` line from
`✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📦️index.ts`.

## Bonus bug found in the same file — broken `lowpoly_schema` / `lowpoly_io` paths

While fixing #1 I found the two remaining exports in `index.ts` were **also broken**: they pointed at
`../../🗿️artifacts/💠️lowpoly/🧬️schema/...` and `.../🚪️io/...`, skipping the mandatory
`🏅️standards/🔖️1/🪆️subsets/✳️any/` path segment that every facet actually lives under (confirmed with
`ls`/`find`, and by cad's own `index.ts` which includes that full path). These resolved to nonexistent files —
verified with `ls` (No such file or directory) before the fix, resolving correctly after. Fixed in the same file
since `index.ts` is in my exclusive scope; this was blocking bug #2 from actually being reachable through the
package barrel.

## Bug #2 — `$A/🚪️io/🟦️component.ts` implemented

Followed cad's `🚪️io/🟦️component.ts` pattern exactly (format list, accept-filter, export menu, host-bridge
interface + installer, async export/import wrappers that throw `[DEBUG] ... host bridge missing` if uninstalled).

Format list sourced from `$A/🚪️io/🦀️component.rs` `import_stdio_kinds()` / `export_stdio_kinds()` — both lists
are identical: `dwg, gltf, json, las, obj, ply, png, stl, txt` (alphabetized, mirroring cad's own list order).
`txt`'s Rust-side registration is owned by a concurrently-running agent per the brief — the TS list was written to
include it now so the TS and Rust surfaces stay in lockstep once that lands (Rust file already listed `stdio.txt` in
both kind lists at time of writing).

Exports: `LOWPOLY_IO_FORMATS`, `LowpolyIoFormat`, `lowpolyIoAcceptFilter()`, `lowpolyIoExportMenu()`,
`LowpolyIoHostBridge`, `installLowpolyIoHostBridge()`, `exportLowpolyMedia()`, `importLowpolyMedia()`.

### i18n note
The ticket asked to follow cad's export-menu i18n exactly. Checked: cad's `cadIoExportMenu()` labels are plain
`format.toUpperCase()` — no `{ en, de }` object. That's correct for this facet: format codes (DWG, GLTF, STL, ...)
are identifiers, not natural-language prose, and are identical in English and German — there is nothing to
translate. The repo's actual `{ en: "...", de: "..." }` house pattern (seen throughout `📚️examples` demo labels and
`app_labels!` blocks like cad's `🗣️terminology/🦀️component.rs`) was not needed here because no prose label exists
on this facet. Mirrored cad's `format.toUpperCase()` verbatim, satisfying "follow cad's io labels exactly."

## `🫀️core.ts` barrel — NOT implemented (justified skip)

Cad's `🫀️core.ts` exists to give cad's `🧩️extensions/*` crates (aec-building, aec-building-structure,
aec-building-energy) one merged `core` namespace (geometry + spatial + registry engine). Checked:
- `find "$LP" -maxdepth 1 -iname "*extensions*"` — lowpoly has **no** `🧩️extensions` directory.
- No other plugin's extensions reference a lowpoly core (grepped for "lowpoly" under every `🧩️extensions` tree —
  only cad's own `aec-building/Cargo.toml` matched, unrelated).

There is no consumer that needs a merged lowpoly core namespace, so no `🫀️core.ts` was added.

## Stub `🟦️component.ts` audit (`find "$LP" -name "🟦️component.ts" -size -300c`)

31 files matched (before my fix; 30 after, since `🚪️io/🟦️component.ts` is no longer a stub). Cross-checked every
one against cad's byte-for-byte equivalent path — all of them are equally tiny stubs on cad's side too (same
`export {};`-class generated facade placeholders for facets whose real logic lives in Rust/WASM, e.g. schema
mutations/snapshot/diff sub-facets, `💡️inferences` binary/text/bounds sub-facets, every `📥️import/🧩️deserializers`
and `📤️export/🧵️serializers` per-format leaf, and the `📚️examples/demo` + `demo-session` stubs). Verdict for all
30: **intentional generated-facade placeholder — house pattern, not a gap.** None fall inside my exclusive file
list anyway (only the one now-fixed `🚪️io/🟦️component.ts` did), so none were touched.

Full list for reference (all confirmed house-pattern, cad-equivalent-is-also-stub):
- `🧬️schema/🧬️mutations/🟦️component.ts`
- `🧬️schema/📸️snapshot/{💾️binary,📝️text}/🟦️component.ts`
- `🧬️schema/🧬️mutations/{💾️binary,📝️text}/🟦️component.ts`
- `🧬️schema/🔺️diff/{💾️binary,📝️text}/🟦️component.ts`
- `🧬️schema/💡️inferences/{💾️binary,📝️text,📦bounds}/🟦️component.ts`
- `📚️examples/🎬️demo/🟦️component.ts`, `✏️editor/📚️examples/🎬️demo-session/🟦️component.ts`
- `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/{json,ply,obj,txt,png,dwg,las,gltf,stl}/.../🟦️component.ts`
- `🚪️io/📤️export/🧵️serializers/🗿️artifacts/{json,ply,obj,txt,png,dwg,las,gltf,stl}/.../🟦️component.ts`

## Files touched (all within exclusive scope)
- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📦️index.ts` — removed broken `lowpoly_decomposer` export;
  fixed broken `lowpoly_schema` / `lowpoly_io` paths (missing `🏅️standards/🔖️1/🪆️subsets/✳️any/` segment).
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🟦️component.ts` —
  implemented (was a 71-byte `export {};` stub with a stale "WASM facades land in W7" marker; W7's owning ticket
  `26/08/06/S-MODULES-CRATE-CONSOLIDATION-AND-NAMING-FIX` is closed).

## Non-exclusive finding worth flagging (not touched)
`✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/package.json` is a stale copy from cad: its `description` says
"CAD plugin TS", its `test`/`generate`/`fixture` scripts run `bun nx run @semio-tech/cad-js:*`, and its
`dependencies` list `@semio-tech/cad-js-module-*` packages instead of any lowpoly equivalents. Not in my exclusive
file list — flagging for whoever owns `package.json`/`📜️script.ts`.

## Verification

### Scoped typecheck (no repo-wide tsconfig needed — file has zero imports)
```
$ cd /Users/ueli/Documents/semio && bun x tsc --noEmit --target ESNext --module ESNext \
    --moduleResolution bundler --strict --esModuleInterop --isolatedModules --skipLibCheck \
    "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🟦️component.ts"
(no output — clean)
```

### Repo-wide typecheck
```
$ bun x tsc --noEmit -p tsconfig.json
```
Ran in background (repo-wide, ~30k+ files, other agents editing concurrently). See ticket chat for pass/fail and,
if red, the filtered subset of errors under `✏️s/🔌️plugins/💠️lowpoly/` specifically (repo-wide red from unrelated
plugins is expected given concurrent work per CLAUDE.md and is not evidence against these changes).

### `nx run @semio-tech/lowpoly-js:test`
```
$ bun nx run "@semio-tech/lowpoly-js:test"
$ bun ./📜️script.ts nx run @semio-tech/lowpoly-js:test
> nx run @semio-tech/lowpoly-js:test
> bun ./📜️script.ts test
[DEBUG] lowpoly interactive-job owned source/fixture ok: 19 Migrated, 28 BatchOnlyPendingRewrite
[DEBUG] lowpoly interactive-job Ajv hostile oracle ok: duplicate, missing lane, empty blocker, lane/preparation mismatch rejected

 NX   Successfully ran target test for project @semio-tech/lowpoly-js
```
Exit code 0.
