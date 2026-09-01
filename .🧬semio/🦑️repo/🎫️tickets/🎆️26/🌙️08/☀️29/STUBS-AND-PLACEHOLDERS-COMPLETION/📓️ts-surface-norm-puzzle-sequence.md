# TS Surface Repair — 📕️norm, 🧩️puzzle, 🎬️sequence

Scope per orchestrator: `📕️norm` (8 errors), `🧩️puzzle` (6 errors), `🎬️sequence` (1 error). All three now compile with 0 `error TS` lines under the mandated strict `tsc --noEmit` invocation.

## Problem 1 — `qK: qK[]` self-referential type (📕️norm / en1990, 3 sites)

Rust evidence: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🦀️component.rs:24`
```rust
pub type En1990QkChild = store::ArtifactChild<semio_s_plugin_stdio::...::table::schema::snapshot::SemioTableSnapshot>;
```
and `store::ArtifactChild<S>` itself (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:2560-2568`):
```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound = "")]
pub struct ArtifactChild<S> {
    pub child_id: String,
    pub target: crate::os_io::ArtifactRef,
    #[serde(skip)] local_owner: ...,
    #[serde(skip)] _snapshot: PhantomData<S>,
}
```
`local_owner`/`_snapshot` are `#[serde(skip)]`, so the wire shape is exactly `{ childId: string, target: ArtifactRef }`. `📏️layout` (already-fixed sibling scope) mirrors this identically as `ArtifactChildHandle`/`ArtifactRef` in both its snapshot and diff `🟦️component.ts` — I followed that precedent verbatim.

`En1990Diff` in Rust (`…/🔺️diff/🦀️component.rs:10-30`) has **no** `artifact` field — the doc comment there explicitly says the former `artifact: Option<Box<En1990Artifact>>` slot "is removed — dead code … shaped exactly like the banned `SetSnapshot` vocabulary." The TS diff file had a stray `artifact?: En1990Artifact` field plus a locally-redeclared (wrong) `En1990Artifact` interface that doesn't exist in Rust's diff struct — both removed.

Changes:
- `🧬️schema/🟦️component.ts:7` — `qK: qK[]` → `qK: ArtifactChildHandle`; added `ArtifactRef`/`ArtifactChildHandle` mirrors.
- `🧬️schema/📸️snapshot/🟦️component.ts:7` — same fix + same two mirror interfaces (`En1990Snapshot` has no `selectedCheckIndex`, matching Rust `En1990Snapshot`, which also lacks it).
- `🧬️schema/🔺️diff/🟦️component.ts` — removed dead `artifact?: En1990Artifact` field and the local `En1990Artifact`/`En1990QkEntry`/`En1990QkList` declarations (none exist in Rust's `En1990Diff`); `qK?: En1990QkList` → `qK?: ArtifactChildHandle` (matches Rust `pub q_k: Option<En1990QkChild>`); added `ArtifactRef`/`ArtifactChildHandle` mirrors locally (each file is its own TS module, no cross-file collision).

## Problem 2 — missing `En199XArtifact` types (📕️norm / en1995–en1999, 5 sites)

Rust evidence, e.g. `…/📘️en1995/…/🔺️diff/🦀️component.rs:10-12`:
```rust
pub struct En1995Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::en1995::schema::En1995Artifact>>,
    ...
```
Unlike en1990, this `artifact` field is genuinely still present for en1995–en1999 (checked all five Rust diff structs — all keep it). Each sibling `🧬️schema/🟦️component.ts` (non-diff) already declares the full `En199XArtifact` interface (confirmed field-for-field present, e.g. `En1995Artifact` in `…/en1995/…/🧬️schema/🟦️component.ts`). Fix was `import type`, not re-declaration, per the ticket's guidance.

Changes (all 5: en1995, en1996, en1997, en1998, en1999), in `…/🔺️diff/🟦️component.ts`:
```ts
import type { En199XArtifact } from "../🟦️component.ts";
```
inserted after the file's header comment — matching the existing `import type { X } from "../🟦️component.ts";` convention used elsewhere in the repo (e.g. `🔱️trinity/…/jack/…/🔺️diff/🟦️component.ts`).

Also fixed a pre-existing (non-blocking) typo while in the en1995 file: `selectedCheckIndex?: number | null | null;` → `number | null` (duplicate union member, harmless to `tsc` but clearly a copy-paste artifact).

## Problem 3 — duplicate index signatures (🧩️puzzle, TS2374, 6 sites)

**5d** (`🖐️5d/…/🔺️diff/🟦️component.ts`): `Puzzle5dKindCompatibility` was declared **twice** — once at line 93 with the full field set (`source?`, `target?`, `bidirectional?`, `important?`, `specificity?`, plus `[key: string]: unknown`), and again at line 99 as a bare `{ [key: string]: unknown; }`. TS interface-merges same-named declarations, so the two identical `string` index signatures collided (TS2374). Rust evidence, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️component.rs:304-315`, confirms the full field set (`source`, `target`, `bidirectional`, `important`, `specificity`) with no extra fields — the line-93 declaration already expresses that shape plus the codegen's usual "open" index signature (consistent with sibling types like `Puzzle5dFastener`/`Puzzle5dPart` in the same file). Fix: deleted the redundant bare redeclaration at line 99; line 93 alone still expresses the Rust shape.

**3d** (`🧊️3d/…/🔺️diff/🟦️component.ts`): `Puzzle3dTargetVolume` and `Puzzle3dReference` were each declared **twice**, byte-identically (`{ id: string; [key: string]: unknown; }`) — once at lines 96/100, and again verbatim at the very end of the file (lines 215/216, a clear copy-paste duplicate with no surrounding content). Rust evidence: `Puzzle3dTargetVolume`/`Puzzle3dReference` structs at `🧊️3d/🦀️component.rs:228` and `:256`. Fix: deleted the trailing duplicate two-line block; the earlier declarations (already an accurate opaque `{id, ...unknown}` mirror consistent with how this file treats other richer domain types) are unaffected.

## Problem 4 — untyped `.js` import (🎬️sequence, TS7016, 1 site)

`📦️index.ts:8` imports `createSequenceBrowserFeatures` from `…/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🟨️sequence-browser.js`, which has no declaration file.

The ticket pointed at `🌊️flow/…/🌉️wasm/…/🟨️flow-browser.d.ts` as precedent — I searched for it (`find … -iname "*flow-browser*"`, `find … -iname "*.d.ts" ! -path "*node_modules*" ! -path "*/pkg/*"`) and it does **not** exist anywhere in the current tree; flow's plugin has no `🌉️wasm` bridge directory at all right now. `🟨️sequence-browser.js` is the only `.js` file imported from any `.ts` file in the whole `✏️s/🔌️plugins` tree, and no `.d.ts` precedent exists to follow — so I authored one from scratch against the real exported surface.

Read both `🟨️sequence-browser.js` (the sole export, `createSequenceBrowserFeatures`) and `🟨️sequence-host.js` (`createSequenceHost`/`createSequenceFeatures`, which `sequence-browser.js` wraps) line-by-line to trace every returned method's actual decode function (`text`→`string`, `boolean`→`boolean`, `optionalString`→`string|undefined`, `json`→`unknown`, or the default `identity`→raw `Uint8Array`), and wrote `🟨️sequence-browser.d.ts` next to the `.js` declaring:
- `SequenceTask<T>` (the `{ requestId, result, cancel, subscribe }` shape every call returns)
- One features interface per facet (`document`/`editing`/`execution`/`viewport`/`input`/`layout`/`selection`/`preview`/`playback`/`lifetime`), each method typed to its real resolved value per the decode function actually used at that call site in `🟨️sequence-host.js`
- `SequenceBrowserOptions` (`source`/`imports`/`instantiate`/`resolveCanvas`/`render`/`schedule`/`maximumInFlight`, matching the destructured parameter list in `sequence-browser.js`)
- `createSequenceBrowserFeatures(options): Promise<SequenceFeatures>`

No `any`, no `@ts-ignore`. `sequence-host.js` itself is never visited by `tsc` (only the `.js` that TS actually tries to resolve — `sequence-browser.js` — needed a sibling declaration; TS doesn't chase further because `allowJs` is off).

## Verification

```
$ bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler \
    --esModuleInterop --skipLibCheck --allowImportingTsExtensions <plugin>/📦️packages/🟦️typescript/📦️index.ts \
    2>&1 | grep -c 'error TS'
norm:     8 → 0
puzzle:   6 → 0
sequence: 1 → 0
```

`git status --porcelain` against exactly the 11 files touched (10 edited + 1 new `.d.ts`) returns 11 lines — confirms no scope creep. Repo-wide `git status` shows ~1600 files from concurrent sibling sessions (🧱️block, 🏗️fem, 🏛️architect, and various ticket-folder churn); none of that is mine and none of it was touched.

## Files touched

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️component.ts`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️component.ts`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️component.ts`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️component.ts`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️component.ts`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️component.ts`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️component.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️component.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️component.ts`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🟨️sequence-browser.d.ts` (new)

## Unfinished / notes

Nothing unfinished within scope. Not touched (out of scope, flagged only if the orchestrator wants a follow-up): `Puzzle5dKindCompatibility`'s TS fields are all optional (`source?`, `target?`, …) even though it's used as `Puzzle5dKindCompatibilityList.values: Puzzle5dKindCompatibility[]` — a whole-list-replace value, not a sparse per-item patch — while Rust's `source`/`target` are non-optional `String`. That predates this fix (present before I touched the file) and isn't a `tsc` error, so I left it as-is rather than expanding scope.
