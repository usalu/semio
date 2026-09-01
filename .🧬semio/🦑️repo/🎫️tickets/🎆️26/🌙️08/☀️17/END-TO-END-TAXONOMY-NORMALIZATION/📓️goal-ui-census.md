# 📊️ UI Module Census — `🧰️framework/🔨️modules/🖱️ui`

Baseline commit for every measurement below: `bb06c41f73f0122fbed315b7487428b976f99921` (matches
`📓️goal-session-status.md`'s pinned baseline).

Real, pasted `plan` output, run 1, before ANY edit in this slice:

```
[clean taxonomy plan] moves=812 roots=0 relocations=0 symlinks=0 removals=0 edits=1371 regenerations=4 unresolved=554
```

(The brief quoted 809/563; the small drift is other slices' repo-wide fixes landing between the
brief being written and this run — confirmed by diffing `🧹️normalization/🟦️.ts` /
`🔣️taxonomy.json`, both under heavy concurrent edit for the whole session.)

## 1. By code (run 1, `🗑️temp/🔣️ui-plan.json`)

| code | n |
|---|---:|
| `semantic-stem-unresolved` | 330 |
| `package-implementation-destination-unresolved` | 88 |
| `semantic-stem-ambiguous` | 59 |
| `reference-syntax-unsupported` | 54 |
| `directory-kind-unresolved` | 21 |
| `generator-preview-invalid` | 1 |
| `directory-kind-ambiguous` | 1 |

**87 of the 88 `package-implementation-destination-unresolved` rows are the SAME files as a
`semantic-stem-unresolved` row** (both codes fire off the identical `matchDirectoryKind` call, one
from `canonicalFile`, one from `packageImplementationDestination`) — so the two families overlap
almost completely rather than adding.

## 2. Family A — package/module stem vocabulary gap (330 + 88, ~87 shared paths)

`packageImplementationDestination` (`🧹️normalization/🟦️.ts:3277`) and `canonicalFile`'s semantic-stem
branch both call `matchDirectoryKind(bareWord, taxonomy, parentKindId)`. Every multi-file Rust/TS/JS
package in `🖱️ui` keeps several domain-named source files directly at the package root
(`🧠️runtime/📦️packages/🦀️rust/{context,dispatch,entity,gateway,present,reconcile,tracking}.rs`,
`🧬️contract/…/{accessibility,action,conformance,document,layout,limits,style,surface,text_edit}.rs`,
`🖼️render` + its 4 GPU-backend targets (`🌋️vulkan`,`🍎️metal`,`🧊️webgpu`,`🪟️d3d12`) each repeating
`{resources,pipelines,types,scene_target,frame_buffers,…}.rs`, `📦️packages/🦀️rust/🎯️targets/🧊️wgpu`
(21 files), `📦️packages/🟦️typescript/🎯️targets/⚛️react` (4 files), `🖥️host` (3 files) — none of these
~71 distinct stems had a registered `semanticDirectoryKinds` entry.

Two tool-config rows in this family are NOT vocabulary — they are misclassified because of an
ecosystem-boundary gap: `🎨️styling/📦️packages/🦀️rust/🧪️vitest.config.ts` sits inside a **Rust**
package boundary, so `classifyPackageRole` never reaches the already-landed
`packageSourceDispositions.vitest-config` disposition (`🦀️rust.allowedFixedContractIds` doesn't
list it, and `vitest-config`'s own `scope.ecosystemId` is hard-pinned to `🟦️typescript` regardless).
Not fixed here (see §6) — a concurrent slice was mid-flight on exactly this contract per
`📓️goal-pkgdest-report.md`. `uv.lock` under `🎨️styling/📦️packages/🐍️python` had no
`fixedFilenameContracts` entry at all (fixed, see §4).

## 3. Family B — the `🧪️conformance` fixture-example shape (69 + 46 + 36 + 30 of the 330)

`🧬️contract/📚️examples/🧪️conformance/{🧩️component,🩹️patch,🚫️rejection,📐️layout,🖥️composite}/<case>.
{expect,snapshot,patch}.json` and sibling `🧪️fixtures/*.{tsv,json,schema.json}` files. The PARENT
directories already resolve fine via `semanticDirectoryMemberKinds["members-of-members-of-examples"]`
(a named-member overlay, not a pattern) — the files inside were unresolved because:

1. `.expect.json` / `.snapshot.json` / `.patch.json` had no `fileKindResolutionRules` entry (only
   the sibling `.schema.json` → `json-schema` compound extension existed), so the stem never
   stripped past the dot (`"button.expect"`, not `"button"`) and could never match any slug pattern.
2. Even with a clean stem, the resulting bare case-name (`button`, `cycle`, `dangling-child`, …) needs
   `test-fixture-member`, whose `parentKindIds` didn't include `members-of-members-of-examples`.

Both fixed (§4). `🧪️fixtures/*.tsv` rows (`browser-host*.tsv`, `surface-port*.tsv`) are a DIFFERENT,
still-open shape: each file hand-picks its own descriptive leading emoji
(`📐️browser-host-limits.tsv`, `📒️browser-host.tsv`, `📨️browser-host-framing.tsv`) with no existing
directory kind registered for any of those specific (emoji, word) pairs, and each name is
effectively unique-per-fixture rather than a reusable domain concept — registering ~15 more
one-off kinds for names that will never repeat felt like the wrong shape to mint under this
slice's time budget; flagged, not fixed.

## 4. Fixed this slice (`🔣️taxonomy.json`, additive only, no legacy layer)

- **53 new `semanticDirectoryKinds`** for the Family A stems (`context`, `dispatch`… — full list in
  `git diff`), each reusing the exact emoji already found once on disk for that word where one
  existed (checked via `git ls-files`, same method as `📓️goal-vocab-census.md`), otherwise a fresh
  valid emoji.
- **3 new `fileKinds` + `fileKindResolutionRules`** compound extensions: `json-expect`
  (`.expect.json`), `json-snapshot` (`.snapshot.json`), `json-patch-fixture` (`.patch.json`) —
  same idiom as the pre-existing `json-schema` / `.schema.json` pair.
- **`test-fixture-member.parentKindIds`** += `members-of-members-of-examples`.
- **`test-case.parentKindIds`** += `members-of-elements` (fixes the `🧱️elements/<PascalCase>/
  🧪️story.tsx` half of Family C, §5).
- **`retained`** (🧵️, `parentKindIds:["schema"]`) and **`retained-resident-store`** (💾️, slug
  `^resident$`, `parentKindIds:["retained"]`) — `🖱️ui/🧬️contract/🧵️retained` is a genuine, previously
  unregistered domain concept; `💾️resident` under it is a SEPARATE concept from the already-registered
  `resident` kind (🎟️, ticket/session-lease meaning) despite sharing the English word — scoped, not
  merged, per the ticket's collision rule. `🎟️resident` (9 on-disk occurrences) keeps its existing
  registration untouched; the new kind only matches the memory-resident meaning under `🧵️retained`.
- **`fixedFilenameContracts.uv-lock`** (`**/uv.lock`, `package-root`/`🐍️python`) + added to
  `packageBoundaryRules.🐍️python.allowedFixedContractIds`.

## 5. A self-inflicted regression, found and fixed before it shipped

13 of the first 53 words (`accessibility`, `arena`, `conformance`, `context`, `cursor`, `draw`,
`element`, `gateway`, `math`, `present`, `schedule`, `theme`, `tree`) happen to ALSO be literal
`memberNames` of unrelated `semanticDirectoryMemberKinds` overlays elsewhere in the repo
(`members-of-apps`, `members-of-artifacts`, `members-of-assets`, `members-of-commands`,
`members-of-engine`, `members-of-examples`, `members-of-members-of-examples`,
`members-of-members-of-modules`, `members-of-modules`, `members-of-plan`, `members-of-plugins`,
`members-of-snapshot`, `members-of-styling`) — same emoji, same word, unrelated meaning. Registering
them as flat global kinds won the EXACT-id branch of `matchDirectoryKind` ahead of those overlays,
which silently changed the resolved `kindId` for every directory using that overlay chain, breaking
downstream contextual resolution for anything relying on the overlay's OWN id (proven concretely:
`🧬️contract/📚️examples/🧪️conformance` itself went from resolved to `directory-kind-unresolved`, and
its five overlay-chained children — `🧩️component`,`🩹️patch`,`🚫️rejection`,`📐️layout`,`🖥️composite` —
went with it, a **regression I introduced and caught by diffing two of my own plan runs**, not by
inspection).

Fix: added `parentKindIds: ["rust-language","typescript-language","javascript-language"]` to all 13
— they now only win in the package-root file-stem context they were meant for, and fall through to
the pre-existing overlay everywhere else. Verified with a second `git ls-files`-driven scan against
every `semanticDirectoryMemberKinds.memberNames` list repo-wide: the remaining 40 words have zero
collisions. Cross-checking every new word against `semanticDirectoryMemberKinds` before registering
— not just against other `semanticDirectoryKinds` ids — is the lesson worth carrying forward;
`📓️goal-vocab-census.md`'s own collision check (§6 there, the `input` case) only checked one
direction too.

## 6. Explicitly NOT registered (genuine conflicts, correctly left alone)

`document` (✳️/📄️/🧬️/🧪️ — 4 different meanings on disk repo-wide), `dispatch`, `geometry`, `host`,
`layout`, `paint`, `runtime`, `scene`, `shell`, `surface`, `window` (each 2–4 conflicting emoji found
repo-wide) — same shape as the already-documented `admission`/`handback`/`hash`/`instance`/
`metadata`/`output`/`patch`/`root`/`transport`/`typed`/`wire` list from
`📓️goal-vocab-census.md` §3. `input` stays untouched (already deliberately scoped, §6 there).
`vitest.config.ts`-in-a-Rust-package (§2) needs the ecosystem-boundary fix, not a vocabulary entry.

## 7. External interference observed mid-slice (not caused by this slice, not fixed by it)

A concurrent session registered a new `asset-binary-subject` `semanticDirectoryKinds` entry
(`📦️`, `parentKindIds:["assets","members-of-assets"]`) with a slug pattern that overlaps the
pre-existing `asset-subject` (🖼️, same `parentKindIds`) almost completely — every asset file whose
stem isn't `packages|content|payload|resource|resources|buffers` now matches BOTH, which is a
`semantic-stem-ambiguous` for essentially every file under `🖼️assets`, `🖼️assets/🔣️icons`, `.css`
globals, etc. (**471 new ambiguous rows** appeared in `🖱️ui` between two of my own plan runs, with
no edit of mine touching assets). This is out of this slice's scope to fix (not part of the ui
module's own vocabulary gap, and the kind is still being actively edited by its owner) — flagged
here so the coordinator doesn't attribute it to this slice's work. My own family
(`test-case`/`test-fixture-member` ambiguity) stayed flat at 58–59 rows across every run.

## 8. Also present, not this slice's to fix

- `reference-syntax-unsupported` (54 at baseline) — dominated by **absolute** `/Users/ueli/…` path
  mentions inside `📓️` reports of a *different, currently-open* ticket
  (`26/08/17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG`) naming `🧵️retained/💾️resident/🟦️component.ts`
  and similar. The `historicalDocumentEvidencePopulations` contract (`📓️goal-session-status.md` §10)
  covers ticket-relative prose; these are absolute-filesystem-path tokens, a different shape, and
  registering `retained`/`resident` makes the tool try (and fail) to rewrite them, which is a more
  honest signal than the previous silent non-match, not a new defect.
- `generator-preview-invalid` / `directory-kind-ambiguous` (1 each) — `🎨️styling/net/Elements.Styling`
  is an incomplete `.NET` target (`net` should be `dotnet` per the registered `dotnet-language` kind;
  the C# codegen's own safe-root list doesn't include the mis-named directory either). A directory
  rename, not a vocabulary gap — out of this slice's "vocabulary only" mandate, flagged for whoever
  owns that target.
- `🧬️contract/🧵️retained/🖼️surface` — `directory-kind-ambiguous` between `asset-subject` and
  `test-fixture-asset` — pre-existing, unrelated to anything touched here.

See `📓️goal-ui-report.md` for the final measured before/after.
