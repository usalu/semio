# 📓 JSON-Boilerplate Placeholder Fix (`Json{Snapshot,Mutations,Diff}{Text,Binary}`)

## Verdict

**Renamed.** These were not a deliberate shared contract — they were a copy-paste artifact. Each
affected file's docstring literally claimed the type belongs to `stdio.json`, which is false for
91% of them (only 6 of 318 total occurrences are actually inside
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/…`). `string`/`Uint8Array` as the substance is correct
for every artifact (text vs. binary wire representation) — only the naming/doc was wrong.

## Naming convention (derived, not invented)

Every affected file sits at `.../🧬️schema/{📸️snapshot,🧬️mutations,🔺️diff}/{📝️text,💾️binary}/🟦️component.ts`
or the `🚪️io/…` equivalent, always directly under one `✳️<subset>` root. The repo already has a
**correct, non-boilerplate exemplar of this exact shape** for the sibling `💡️inferences` kind
(e.g. `✏️s/🔌️plugins/🧱️block/…/💡️inferences/📝️text/🟦️component.ts`):

```ts
/** 📝️ Text representation for `block.block2d.inference`. */
export type Block2dInferenceText = string;
```

Two independent facts were derived from the codebase, not guessed:

1. **Type base name** — read from each subset's own `🧬️schema/🟦️component.ts`, which exports
   `export interface <Base>Artifact { ... }` (e.g. `Din18599Artifact`, `Puzzle2dArtifact`,
   `SequenceArtifact`, `SHomeArtifact`). `<Base>` is reused for `<Base>SnapshotText`,
   `<Base>SnapshotBinary`, `<Base>MutationsText`, `<Base>MutationsBinary`, `<Base>DiffText`,
   `<Base>DiffBinary` — mirroring how the original `stdio.json` file names its own six variants
   (`Json{Snapshot,Mutations,Diff}{Text,Binary}`) after its own `Json` base.
2. **Docstring dotted slug** — read from the artifact's own `🦀️component.rs`
   (`pub const X_DIALECT: Dialect = Dialect { artifact_kind: "s.<plugin>.<artifact>", ... }`),
   i.e. the *canonical* `plugin.artifact` id, not a name derived from the directory text or from
   `<Base>`. This matters: for 2 of 52 artifacts the canonical slug and `<Base>.toLowerCase()`
   diverge (`🔋️energy/🔋️model` → base `EnergyModel`, slug `energy.model`; `🪐️space/🏠️home` → base
   `SHome`, slug `space.home`) — exactly matching what the already-correct `💡️inferences`
   docstrings for those two artifacts already say (`energy.model.inference`,
   `space.home.inference`), confirming the slug is artifact_kind-sourced, not name-derived.

Result, e.g.:

```ts
/** 📝️ Text representation for `puzzle.puzzle2d.mutations`. */
export type Puzzle2dMutationsBinary = Uint8Array;   // (binary file)

/** 📝️ Text representation for `norm.din18599.snapshot`. */
export type Din18599SnapshotText = string;

/** 💾️ Binary representation for `space.home.diff`. */
export type SHomeDiffBinary = Uint8Array;
```

## Importer check (before renaming)

`rg -n 'JsonSnapshotText|JsonSnapshotBinary|JsonMutationsText|JsonMutationsBinary|JsonDiffText|JsonDiffBinary' -g '*.ts' -g '*.tsx' .`
→ 318 hits total, **all** of them the `export type ...` declaration line itself (312 boilerplate +
6 legitimate `stdio.json` originals). Zero `import { Json... }` by-name usages anywhere in the
repo — every consumer goes through plugin barrels (`export * as <ns> from "..."`), which are
namespace-scoped and unaffected by renaming the underlying type. Safe to rename with no other file
touched besides the 312 themselves.

## What was NOT touched (found, out of scope, flagged separately)

The exact same boilerplate contamination exists in the **non-`.ts` sibling files** in these same
directories — e.g.
`✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/…/📸️snapshot/📝️text/🔣️component.json` still has
`"$id": "https://semio.tech/schema/stdio.json/snapshot/text.json"` and `"title": "JsonSnapshotText"`.
The ticket's grep and scope were `.ts`-only, so these were left as-is and flagged as a background
task (see below) rather than fixed here.

## Mechanics

Two-pass script in scratchpad (not left in repo):
1. Built a 52-entry artifact map (`✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifact>` → `{base,
   plugin_slug, artifact_slug}`) by reading each subset's `🧬️schema/🟦️component.ts` for `<Base>`
   and each artifact's `🦀️component.rs` for the canonical `artifact_kind` slug pair (with one
   manual override for `🔋️energy/🔋️model`, whose `artifact_kind` is behind a named constant the
   regex didn't chase — resolved by hand-reading the constant).
2. For each of the 312 files, derived `{kind: Snapshot|Mutations|Diff}` and
   `{rep: Text|Binary}` from its own directory names and rewrote the 2-line file from the map.

## Verification (actual output)

- Re-grep: `rg -l 'for \`stdio\.json\`' -g '*.ts' "✏️s" | grep -v '🗄️stdio/🗿️artifacts/🔣️json'`
  → **empty** (exit code 1, no matches).
- Typecheck: `bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution
  bundler --esModuleInterop --skipLibCheck` over all 312 changed files at once → **0 diagnostics,
  exit code 0**. Sanity-checked the command itself against a deliberately broken file first
  (`Cannot find name 'ThisTypeDoesNotExist'`, exit code 2) to confirm tsc wasn't silently no-op'ing.
- `git status --porcelain | wc -l` → 1599 total dirty files repo-wide (other concurrent sessions'
  work per CLAUDE.md — not this ticket's). Cross-checked specifically: all 312 files in the
  affected list show as modified via `git status --porcelain -- <312 paths>`; nothing else was
  touched by the rewrite script.

## Counts

- 312 files changed (52 artifacts × 6 variants), 0 errors during rewrite.
- 52 distinct `Base` names resolved, 52 distinct canonical `plugin.artifact` slug pairs resolved
  (51 from `artifact_kind` regex match, 1 — `energy.model` — resolved manually).
