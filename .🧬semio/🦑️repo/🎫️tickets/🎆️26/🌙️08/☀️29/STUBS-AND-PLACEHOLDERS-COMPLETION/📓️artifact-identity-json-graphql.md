# Artifact identity fix — `.json` / `.graphql` schema mirrors

Scope: the `.json` and `.graphql` per-facet schema mirrors under `✏️s/🔌️plugins/**` that were
copy-pasted from `🗄️stdio/🗿️artifacts/🔣️json` and never re-identified (claimed `$id`/`title`/header
comment `stdio.json` while living under a different artifact). `.g4`/`.ebnf`/`.proto` were out of
scope (sibling agent).

## Discovery

```
rg -l 'stdio\.json' --glob '!node_modules/**' -g '*.json' -g '*.graphql' "✏️s" | grep -v '🗄️stdio/🗿️artifacts/🔣️json'
```
→ 349 hits.

## Discriminating identity-claims from legitimate references

Of the 349 hits, 32 were legitimate references to the real `s.stdio.json` artifact and were left
untouched:

- **11× `🔌️plugins/*/🔣️descriptor.json`** (plugin root) — `exportStdioKinds`/`importStdioKinds`
  arrays and `dependency.counterpart.artifactKind: "s.stdio.json"` interop declarations, e.g.
  `✏️s/🔌️plugins/🌍️gis/🔣️descriptor.json:179` (`"exportStdioKinds": ["stdio.dwg", …, "stdio.json", …]`)
  and `✏️s/🔌️plugins/🖨️raster/🔣️descriptor.json:6851` (`"counterpart": {"artifactKind": "s.stdio.json", …}`).
- **17× `.../🧪️oracle/🔣️.json`** — `stdio.json` appears only inside prose `rationale`/`_comment`
  strings describing *another* file's defect (e.g. playground's oracle doc quotes the grammar
  placeholder text `"schema" SP "stdio.json"` as evidence for a *different* subset's problem), not
  as this file's own identity.
- **1× `🗄️stdio/🗿️artifacts/🔣️component.json`** — the stdio plugin's own collection manifest,
  legitimately lists `{"directory": "🔣️json", "id": "s.stdio.json", …}` as one of ~34 member
  artifacts.
- **2× `🗄️stdio/🗿️artifacts/{🧊️gltf,🧿️semio}/🧬️schema/📜️artifact-definition.json`** — each
  correctly self-identifies (`"id": "s.stdio.gltf"` / `"id": "s.stdio.semio"`) and lists
  `"s.stdio.json"` only inside its own `"dependencies"` array.
- **1× `📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/🔣️component.json`** — lists
  `"s.stdio.json"` inside `importDialects`/`exportDialects`, a legitimate capability declaration.

Rule applied: a hit was fixed only when `stdio.json`/`"s.stdio.json"` appeared as *this file's own*
`$id`, `title`, GraphQL header comment, or embedded `type Document`/scalar identity — never when it
appeared inside an array of imported/exported/dependency kinds or inside prose.

The remaining **317 files were true identity mis-claims** and were fixed.

## Convention derived (cross-validated against the repo's own already-correct files, not invented)

Two shapes exist among the 317, both mechanical:

**1. Representation level (`.../📝️text/🔣️component.json` + `🔗️component.graphql`, 152 dirs × 2
files = 304 files).** For each, the sibling `🟦️component.ts` in the *same* directory (already
fixed by the prior agent) was read directly and trusted as source of truth — titles are
idiosyncratic per artifact (`EnergyModelDiffText`, `SHomeDiffText`, `GisTerrainSnapshotText`) so
deriving them mechanically from directory names would have been wrong:

- `title` = the `.ts` sibling's exported type name verbatim, e.g.
  `✏️s/…/📙️din18599/…/📸️snapshot/📝️text/🟦️component.ts:2`: `export type Din18599SnapshotText = string;`
  → JSON `"title": "Din18599SnapshotText"`.
- GraphQL header = `# <dotted-slug> text grammar schema`, where `<dotted-slug>` is copied from the
  `.ts` docstring `/** 📝️ Text representation for \`norm.din18599.snapshot\`. */` verbatim.
- `$id` = `https://semio.tech/schema/s/<plugin>/<artifact>/<facet-singular>/text.json`, where
  `<plugin>`/`<artifact>` are read from the artifact's own top-level `🦀️component.rs`
  `pub const …_DIALECT: Dialect { artifact_kind: "s.<plugin>.<artifact>", …, subset: SubsetId::ANY }`
  (e.g. `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🦀️component.rs:162`), and `<facet-singular>` is
  `diff`/`snapshot` unchanged or `mutations→mutation`. Cross-validated against the repo's own
  already-correct `💡️inferences/📝️text/🔣️component.json` files (100% present, zero in scope) —
  every one already follows `s/<plugin>/<artifact>/inference/text.json`, e.g.
  `s/block/block2d/inference/text.json`, `s/energy/model/inference/text.json`,
  `s/space/home/inference/text.json` — confirming both the URL shape and that `<plugin>/<artifact>`
  must come from the Rust dialect constant, not the directory name (`◻2d` → `block2d`,
  `🖐️5d` → `puzzle5d`, `🔋️model` stays `model` under plugin `energy`, `🏠️home` stays `home` under
  plugin `space` — exactly the "canonical slug diverges from `Base.toLowerCase()`" cases flagged in
  the brief).

**2. Facet aggregate level (`.../🧬️schema/{🔺️diff,🧬️mutations,📸️snapshot}/🔣️component.json`, no
`/📝️text/`, 13 files — raster, draw ×3, playbook ×2, layout ×3, forms ×3).** No `.graphql`
counterpart was in scope here — those say `JsonDiff`/`JsonMutation`/`JsonSnapshot` but never the
literal string `stdio.json`, so `rg -l 'stdio\.json'` never matched them; left alone per the
ticket's own discovery method.

- `title` = `<Base><Facet>`, `Base` read from the subset's own `🧬️schema/🟦️component.ts`
  `export interface <Base>Artifact`, `<Facet>` = `Diff`/`Mutation`(singular)/`Snapshot`.
- `$id` = `https://semio.tech/schema/s/<plugin>/<artifact>/<facet-singular>.json`.
- Cross-validated against `🧱️block/◻2d`'s own **already-correct, non-stub** aggregate files
  (`🧬️schema/🔺️diff/🔣️component.json` → `$id: s/block/block2d/diff.json`, `title: Block2dDiff`;
  `🧬️mutations/🔣️component.json` → `$id: s/block/block2d/mutation.json`, `title: Block2dMutation`
  — singular even though the directory is plural `🧬️mutations`; `📸️snapshot/🔣️component.json` →
  `Block2dSnapshot`) and against the repo-wide `mutation/<verb>.json` / `mutation/<verb>/payload.json`
  `$id` convention already used under every artifact's real mutation-vector schemas (`vcs`,
  `sequence`, `demonstrator`, …) — all singular `mutation`, never `mutations`.
- Only `$id`/`title` were changed; the placeholder `{schema, value}` body was left as-is (populating
  real per-artifact schema content is a separate, much larger task, not an identity fix).

## Result

- **317 files fixed** (152 text-level dirs × {json, graphql} = 304, + 13 facet-aggregate `.json`).
- **32 files correctly left untouched** (legitimate references, see above).
- Scripts used (scratch, not committed):
  `/private/tmp/claude-501/-Users-ueli-Documents-semio/c17a0f0b-94f9-4f2f-bbd0-8ff82df33749/scratchpad/fix_identity.py`
  (derives the plan from `🦀️component.rs`/`🟦️component.ts`, writes `plan.json`) and
  `apply_identity.py` (regex-substitutes only the `$id`/`title` line or the graphql header line,
  leaving everything else byte-identical).

## Verification (actually run)

- **JSON validity**: all 165 touched `.json` files parsed with `json.load` — 0 failures.
- **`$id` uniqueness**: `rg -o '"\$id":\s*"[^"]*"' -g '*.json' "✏️s" | sort | uniq -c | sort -rn` →
  **0 values appear more than once** repo-wide after the change.
- **Re-grep for stray identity claims**: re-ran the exact discovery command after the fix → 32
  hits remain, and every one of the 32 matches the confirmed-legitimate patterns above (verified
  programmatically, 0 unmatched) — i.e. no `.json`/`.graphql` outside the real `stdio.json`
  artifact still claims `stdio.json` as its own identity.
- **Change-set size**: `git status --porcelain -- <the 317 planned paths>` → **exactly 317** `M`
  lines, matching the plan 1:1 (no more, no fewer).

## Unfinished / out of scope (flagging, not fixing)

- The 13 facet-aggregate directories' own sibling `🟦️component.ts`/`🔗️component.graphql` (e.g.
  `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️component.ts`)
  are **still** the `JsonDiff`/`JsonMutation`/`JsonSnapshot` stub — not yet re-identified, and not
  in this ticket's `.json`/`.graphql`-only, `rg 'stdio\.json'`-discovered scope (they don't contain
  the literal string `stdio.json`, only `Json*`). Also true for their `.graphql` counterparts. This
  is real remaining stub debt but belongs to whoever owns `.ts`/facet-aggregate `.graphql` cleanup,
  not this ticket.
- `.g4`/`.ebnf`/`.proto` under the same 349 directories are explicitly the sibling agent's scope —
  not touched here.
