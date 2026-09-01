# Lowpoly TypeScript package — verification pass on decomposer/IO/index.ts fixes

This is a verification/re-confirmation pass over the work already recorded in
`📝️typescript-implementation.md` (same ticket, earlier run). I independently re-derived the same
diagnosis before reading that file, then used it to cross-check my own findings. Below is the final,
proven state.

## State found in the working tree (uncommitted, already applied)

Both owned files already contained the fix when this pass started:

- `📦️packages/🟦️typescript/📦️index.ts` — the `lowpoly_decomposer` export line is gone, and the
  `lowpoly_schema` / `lowpoly_io` paths were corrected to include the missing
  `🏅️standards/🔖️1/🪆️subsets/✳️any/` segment (without that segment neither path resolves on disk).
- `$A/🚪️io/🟦️component.ts` — implemented per the cad pattern (see below), 1495 bytes, not the
  71-byte `export {};` stub the ticket brief described.

`git diff` confirms these are real uncommitted working-tree changes, not something I imagined:

```
- export * as lowpoly_schema from "../../🗿️artifacts/💠️lowpoly/🧬️schema/🟦️component.ts";
- export * as lowpoly_decomposer from "../../🗿️artifacts/💠️lowpoly/🪓️decomposer/🟦️component.ts";
- export * as lowpoly_io from "../../🗿️artifacts/💠️lowpoly/🚪️io/🟦️component.ts";
+ export * as lowpoly_schema from "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️component.ts";
+ export * as lowpoly_io from "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🟦️component.ts";
```

I verified independently rather than trusting the prior file at face value:

- `find "$L" -iname "*decomposer*"` on the live tree — no hits anywhere under lowpoly.
- `git log --diff-filter=D --all -- "$L/🗿️artifacts/💠️lowpoly/🪓️decomposer/"` — the decomposer dir
  (`🟦️component.ts` + `🦀️component.rs`) was deleted in commit `0e2007af53` ("Migrate domain plugin
  artifacts to standards tree with txt stubs"). Its TS content was a 6-line generic
  `Decomposition<T> { parts, confidence, diagnostics }` interface facade — that shape now lives as
  `semio_framework_plugin::Composition`/`ArtifactComposition` in the framework (seen directly in
  `🚪️io/🦀️component.rs`'s `derived_composition` module), so nothing was lost, it was subsumed.
  Removing the dead export (not stubbing it) is correct.
- cad's `index.ts` has no `cad_decomposer` export at all — confirms "no facet dir on disk → no
  export" is the house pattern, not an oversight specific to lowpoly.
- Repo-wide, this same dead-export bug also exists in `🖨️raster` and `📸️remodel` (`grep` confirmed
  their `index.ts` files still export `raster_decomposer` / `remodel_decomposer` from paths that
  don't exist). Out of scope (not my exclusive files) — flagged as a handoff item below.

## Bug #2 — `$A/🚪️io/🟦️component.ts` implementation

Modelled directly on cad's `🚪️io/🟦️component.ts` (same shape, renamed `Cad*`→`Lowpoly*`):

- `LOWPOLY_IO_FORMATS` / `LowpolyIoFormat` — `["dwg", "gltf", "json", "las", "obj", "ply", "png",
  "stl", "txt"]`, taken verbatim from `import_stdio_kinds()` / `export_stdio_kinds()` in the
  sibling `🦀️component.rs` (both lists are identical and already include `stdio.txt` — the
  concurrent txt-owning agent's Rust-side work was already merged when this was written).
- `lowpolyIoAcceptFilter()` — builds a `.dwg,.gltf,...` file-picker accept string.
- `lowpolyIoExportMenu()` — `{ format, label }[]` with `label = format.toUpperCase()`.
- `LowpolyIoHostBridge` type + `installLowpolyIoHostBridge()` — host injects `exportMedia`/
  `importMedia` callbacks.
- `exportLowpolyMedia()` / `importLowpolyMedia()` — async wrappers that throw
  `[DEBUG] lowpoly io host bridge missing — installLowpolyIoHostBridge first` if no bridge is
  installed yet.

### i18n check (bug #3)
Verified cad's `cadIoExportMenu()` uses plain `format.toUpperCase()`, not `{ en, de }` — no
bilingual dict anywhere in cad's io facet. This is correct for both: format codes (DWG, GLTF, STL,
JSON, ...) are identifiers, identical in English and German, not translatable prose. Confirmed the
repo's real `{ en: "...", de: "..." }` convention does exist and is used correctly elsewhere in this
exact plugin (e.g. `📚️examples/🎬️demo/🟦️component.ts`: `label = { en: "Demo", de: "Demo" }`, and
`✏️editor/📚️examples/🎬️demo-session/🟦️component.ts`: `{ en: "Demo Session", de: "Demo-Sitzung" }`)
— so the pattern is known and applied where it applies; it correctly does not apply to file-format
codes. Nothing further to do for bug #3 in the io facet.

## `🫀️core.ts` — not applicable to lowpoly
cad's `index.ts` re-exports a merged `core` namespace consumed by cad's `🧩️extensions/*` crates.
lowpoly has no `🧩️extensions` directory (`find` confirmed), and no other plugin's extensions tree
references a lowpoly core. No `core.ts` was needed or added.

## Stub `🟦️component.ts` survey (bug #4)

`find "$L" -name "🟦️component.ts" -exec ... wc -c ... | sort -n` — 41 total component.ts files.
31 are under ~300 bytes; the table below covers all of them. None are in my owned set (only the now
fixed `🚪️io/🟦️component.ts` was), so none were edited.

| File (relative to `$A`) | Bytes | Verdict |
|---|---|---|
| `🚪️io/📤️export/🧵️serializers/…/{dwg,gltf,json,las,obj,ply,png,stl,txt}/…/🟦️component.ts` (9 files) | 11 each | Intentional facade — byte-identical `export {};` placeholder pattern to cad's own per-format serializer stubs (cad's have a one-line docstring, lowpoly's don't; cosmetic only). Each has a matching real `🦀️component.rs` sibling doing the actual work. Not owned, not touched. |
| `🚪️io/📥️import/🧩️deserializers/…/{dwg,gltf,json,las,obj,ply,png,stl,txt}/…/🟦️component.ts` (9 files) | 11 each | Same verdict as above (import side). |
| `🧬️schema/🧬️mutations/🟦️component.ts` | 61 | Intentional facade — content is byte-for-byte cad's own mutations facade with `cad`→`lowpoly` swapped (`/** 🧩 lowpoly 🧬️mutations WASM facade. */\nexport {};`). |
| `🧬️schema/🔺️diff/📝️text/🟦️component.ts` | 95 | Intentional facade — `export type JsonDiffText = string`, same class as cad's text/binary bridge-type stubs. |
| `🧬️schema/📸️snapshot/📝️text/🟦️component.ts` | 103 | Same class (`JsonSnapshotText`). |
| `🧬️schema/🔺️diff/💾️binary/🟦️component.ts` | 103 | Same class (`JsonDiffBinary = Uint8Array`), matches cad's `CadDiffBinary` pattern exactly. |
| `🧬️schema/🧬️mutations/📝️text/🟦️component.ts` | 105 | Same class (`JsonMutationsText`). |
| `🧬️schema/💡️inferences/📝️text/🟦️component.ts` | 111 | Same class (`LowpolyInferenceText`). |
| `🧬️schema/📸️snapshot/💾️binary/🟦️component.ts` | 111 | Same class (`JsonSnapshotBinary`). |
| `🧬️schema/🧬️mutations/💾️binary/🟦️component.ts` | 113 | Same class (`JsonMutationsBinary`). |
| `🧬️schema/💡️inferences/💾️binary/🟦️component.ts` | 119 | Same class (`LowpolyInferenceBinary`). |
| `📚️examples/🎬️demo/🟦️component.ts` | 143 | Not a stub — fully implemented (`id`, bilingual `label = { en, de }`, `icon`). Small by design. |
| `✏️editor/📚️examples/🎬️demo-session/🟦️component.ts` | 175 | Not a stub — fully implemented, bilingual label. |
| `🧬️schema/💡️inferences/📦bounds/🟦️component.ts` | 206 | Not a stub — real `LowpolyBounds { min, max }` interface. |

All type-alias / `export {};` rows above were cross-checked against cad's byte-for-byte equivalent
path and found structurally identical (cad's own equivalents are equally tiny placeholders) — this
is the repo's house pattern for facets whose real logic lives in Rust/WASM, not a gap.

## Files touched
- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📦️index.ts` — dead `lowpoly_decomposer` export
  removed; `lowpoly_schema` / `lowpoly_io` paths corrected to the standards-tree location.
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🟦️component.ts` —
  implemented (format list/type, accept-filter, export menu, host-bridge interface + installer,
  async export/import wrappers).

(Both changes were already present, uncommitted, in the working tree when this pass began — this
pass independently re-derived and verified them rather than assuming they were correct.)

## Handoff items (not touched — outside exclusive scope)

1. `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/package.json` is a stale copy from cad: its
   `description` says "CAD plugin TS: spatial factory runtime/model graph...", and its `scripts`
   block runs `bun nx run @semio-tech/cad-js:test` / `:generate` / `:fixture` (not lowpoly's own
   targets), with `dependencies` listing `@semio-tech/cad-js-module-*` packages that have nothing to
   do with lowpoly. The actual `nx run @semio-tech/lowpoly-js:test` target works correctly because
   `📋️project.json` defines its own `test` target (`bun ./📜️script.ts test`) which takes precedence
   over the package.json `scripts.test` — so this is latent, not currently breaking anything, but it
   is misleading and should be fixed by whoever owns `package.json`.
2. Repo-wide latent bug, same shape as bug #1 here: `✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/📦️index.ts`
   and `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/📦️index.ts` still export
   `raster_decomposer` / `remodel_decomposer` from `🪓️decomposer/🟦️component.ts` paths that do not
   exist on disk (confirmed with `grep`). Same fix applies: remove the dead export line. Not checked
   exhaustively across every other plugin.

## Verification (all commands run for real, output shown)

### Scoped typecheck of the two owned files
```
$ cd /Users/ueli/Documents/semio
$ bunx tsc --noEmit --target ESNext --module ESNext --moduleResolution bundler --strict \
    --esModuleInterop --isolatedModules --skipLibCheck --resolveJsonModule \
    "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🟦️component.ts"
(exit 0, no output)

$ bunx tsc --noEmit --target ESNext --module ESNext --moduleResolution bundler --strict \
    --esModuleInterop --isolatedModules --skipLibCheck --resolveJsonModule \
    --allowImportingTsExtensions \
    "✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📦️index.ts"
(exit 0, no output)
```

### Import graph actually resolves at runtime
```
$ bun -e 'await import("/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📦️index.ts")
    .then(m => console.log("[DEBUG] import ok, keys:", Object.keys(m)))
    .catch(e => { console.error("[DEBUG] import FAILED:", e); process.exit(1); })'
[DEBUG] import ok, keys: [ "lowpoly_io", "lowpoly_schema" ]
```

### Scoped typecheck proven real (typo-injection sanity check)
Per coordinator instruction, proved the scoped `tsc` invocations above actually check something
(rather than silently no-op'ing on these emoji paths) by injecting a deliberate error into each
owned file, confirming `tsc` fails, then restoring the original and confirming it is clean again
(byte-for-byte, via `diff` against a pre-edit backup copy in the ticket scratchpad).

```
$ echo 'export const __typoCheck__: ThisTypeDoesNotExist = 1;' >> ".../🚪️io/🟦️component.ts"
$ bunx tsc --noEmit ...(same flags as above)... ".../🚪️io/🟦️component.ts"
.../🚪️io/🟦️component.ts(33,29): error TS2304: Cannot find name 'ThisTypeDoesNotExist'.
(exit 2 — confirms the check is real)

$ cp <backup> ".../🚪️io/🟦️component.ts"   # restore
$ diff ".../🚪️io/🟦️component.ts" <backup> && echo RESTORE_CLEAN_MATCH
RESTORE_CLEAN_MATCH
$ bunx tsc --noEmit ...  ".../🚪️io/🟦️component.ts"
(exit 0, no output — clean again)
```

```
$ echo 'export * as lowpoly_bogus from "./this-path-does-not-exist.ts";' >> "📦️index.ts"
$ bunx tsc --noEmit ...(same flags, plus --allowImportingTsExtensions)... "📦️index.ts"
📦️index.ts(4,32): error TS2307: Cannot find module './this-path-does-not-exist.ts' or its
corresponding type declarations.
(exit 2 — confirms the check is real)

$ cp <backup> "📦️index.ts"   # restore
$ diff "📦️index.ts" <backup> && echo RESTORE_CLEAN_MATCH
RESTORE_CLEAN_MATCH
$ bunx tsc --noEmit ...  "📦️index.ts"
(exit 0, no output — clean again)
```

Both owned files are confirmed byte-identical to their pre-typo-test state after restore, and both
pass the scoped typecheck cleanly.

### `nx run @semio-tech/lowpoly-js:test` (fresh, cache bypassed) — run twice
First run (cache-bypassed):
```
$ bun nx run "@semio-tech/lowpoly-js:test" --skip-nx-cache
> nx run @semio-tech/lowpoly-js:test
> bun ./📜️script.ts test
[DEBUG] lowpoly interactive-job owned source/fixture ok: 19 Migrated, 28 BatchOnlyPendingRewrite
[DEBUG] lowpoly interactive-job Ajv hostile oracle ok: duplicate, missing lane, empty blocker, lane/preparation mismatch rejected

 NX   Successfully ran target test for project @semio-tech/lowpoly-js
```
Exit code 0.

Re-run at the very end of this pass, after a concurrent agent's further edits to
`📜️script.ts` (not owned by me — it drives the `interactive-job` route-oracle test, whose route
counts visibly changed between runs as that agent migrated more routes):
```
$ bun nx run "@semio-tech/lowpoly-js:test" --skip-nx-cache
[DEBUG] lowpoly interactive-job owned source/fixture ok: 46 Migrated, 1 BatchOnlyPendingRewrite
[DEBUG] lowpoly interactive-job Ajv hostile oracle ok: duplicate, missing lane, empty blocker, lane/preparation mismatch rejected

 NX   Successfully ran target test for project @semio-tech/lowpoly-js
```
Exit code 0 both times. Green throughout the concurrent `script.ts` churn; per CLAUDE.md this test
is not mine to fix even if it did fail (it is a different agent's file), but it never failed.

### Repo-wide typecheck — not run to completion (by coordinator instruction)
```
$ bunx tsc --noEmit -p tsconfig.json
```
Started this in the background early in this pass; after ~13 minutes it had not finished (this is a
~2M-LOC multi-language monorepo — the TS-relevant slice alone is hundreds of files, and other agents
are concurrently editing files across it). The coordinator confirmed a separate agent already
established the whole-repo pass reports **~5,983 pre-existing errors repo-wide**, so a full pass/fail
here would not be diagnostic of these two changes either way, and I killed the background process
(`kill 643`) rather than keep waiting on it. The scoped typechecks above (both files, both
typo-verified) plus the runtime import-graph proof are the correctness evidence for this change;
they do not depend on the rest of the monorepo typechecking cleanly.

### No other consumers reference the removed/changed exports
```
$ grep -rl "lowpoly_decomposer" . --include="*.ts"   # no hits
$ grep -rl "🪓️decomposer" "✏️s/🔌️plugins/💠️lowpoly"   # no hits
$ grep -rl "lowpoly_io\|lowpoly_schema" --include="*.ts" --include="*.tsx" . \
    | grep -v "📦️packages/🟦️typescript/📦️index.ts"   # no hits — no external consumers yet to break
```
