# Resolve Collided component.json Migration Stragglers

## Task

The repo-wide `🔣️component.json` → `🔣️.json` filename-canonicalization migration
skipped 330 mutation-descriptor directories (all `✏️s/🔌️plugins/*/🗿️artifacts/*/…/🧬️mutations/<kind>/`,
plus 5 under `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/`) where both
`🔣️.json` and `🔣️component.json` already existed, so a plain rename would have
collided. Task: determine per-directory which file is authoritative, merge to one
`🔣️.json` carrying the correct content, remove `🔣️component.json`, validate against
the derive's rules.

## Evidence and reasoning

**Tooling reads the canonical name.** `taxonomy.json`'s `mutationDescriptorFileKindId`
is `"json"`, whose `fileKinds.json` entry (`emoji: "🔣️", extensionChains: [".json"]`)
renders to `🔣️.json` via `canonicalFilenameForKind` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`. There is no
taxonomy entry that renders `component.json`; that name is purely legacy.

**Chronology.** Every `🔣️component.json` in the 330 was last touched by commit
`a8d1caf` (2026-08-27, "576"). Every `🔣️.json` was *newly created* (`git show`
confirms `new file mode` diffs, not edits) by one of two later commits:
- `f7b265d` (2026-08-28, "578") — a mass scaffold pass that stamped a generic
  placeholder into every mutation directory it covered, blind to any existing
  `component.json` content: `binaryTag: null`, `textOpcode: null`,
  `outcomeClasses: ["applied"]`, and a template `requiredLanguageSurfaces` (never
  including `typescript`/`graphql`/`protobuf` even when `.ts`/`.graphql`/`.proto`
  files sit right next to it).
- `67fb4216` (2026-09-01, "579") — a much later, large PDF-rebuild commit
  ("PDF rebuilt on lopdf", lopdf-engine generator additions, fixture PDFs) that
  re-ran the same generic stamping logic for directories the 578 pass hadn't
  reached yet (mostly `stdio/pdf` and `stdio/svg`). Its output is still a
  generic template, not hand-authored content — see below.

**Filesystem cross-check (the real test).** For every one of the 330 directories,
`requiredLanguageSurfaces` was recomputed from what's actually on disk (`🦀️.rs` →
rust, `🟦️.ts` → typescript, `🔗️.graphql` → graphql, `🛰️.proto` → protobuf, a
`🔣️*.schema.json` file → json-schema, a `📝️text/` or `💾️binary/` dir → those
surfaces). Result: **`component.json`'s declared surfaces matched the filesystem
in 307/330 dirs; the stub `.json`'s never matched cleanly (0/330)**. The stub's
"always include text+binary, never typescript/graphql/protobuf" template is a
generic default, not a real read of the directory.

**`displayName` (10 dirs where it differs)**: `component.json` has correct
acronym casing ("Change URI", "Remove AF Relationship", "Insert JavaScript
Action"); the stub has generic `str.title()` casing ("Change Uri", "Remove Af
Relationship", "Insert Javascript Action"). Confirms `component.json` is
hand-authored, the `.json` stub is templated.

**`payloadSchema` (23 dirs where it differs, all `stdio/pdf`)**: here the
stub actually wins. `component.json` carries a stale dotted-URN reference
(`"s.stdio.pdf.1.4.a#set-page-text"`) that matches no file on disk — a leftover
from before the PDF plugin had physical `🔣️.schema.json` files. The 579 stub
correctly names the real file (`"🔣️.schema.json"`), which is also confirmed as
the repo-wide convention by sampling 1915 already-migrated (non-collided)
mutation descriptors elsewhere in the repo.

## Merge algorithm applied

Per directory: start from `🔣️component.json` (authoritative for
`invertibility`, `diffParticipation`, `outcomeClasses`, `textOpcode`,
`binaryTag`, `composition`, `displayName`, and every identical field), then:
- **`requiredLanguageSurfaces`**: always recomputed from the filesystem
  (ground truth, independent of either source file).
- **`payloadSchema`**: kept from whichever of the two original values names a
  file that actually exists in the directory (`component.json` checked first
  since it's the base; falls back to the `.json` stub's value in the 23
  `stdio/pdf` cases where only that one resolves to a real file).

Result written to `🔣️.json` (14-key order matching
`MUTATION_LEAF_DESCRIPTOR_KEYS` in `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs`,
2-space indent, trailing newline, matching the existing convention), then
`🔣️component.json` removed.

## Validation

All 330 merged descriptors validated against the parser rules in
`🗣️dsl/✨️derive/🦀️.rs` (`parse_mutation_leaf_descriptor`): exactly the 14
declared keys, `schemaVersion == 1`, `owner` equals the directory path,
`semanticKind` is lowercase kebab-case, directory name (emoji stripped) equals
`semanticKind`, `kebab(aggregateVariant) == semanticKind`, `invertibility` /
`diffParticipation` / `composition` / `outcomeClasses` /
`requiredLanguageSurfaces` are valid enum members, `binaryTag` is `null` or an
integer, `payloadSchema` is non-empty. **330/330 passed, 0 errors.**

Note on `APPROVED_VERBS`: an initial heuristic (first hyphen-segment of
`semanticKind`) flagged 10 directories (`collapse-page-size`,
`embed-font-file` ×6, `touch-artifact`, `sign-out`, `sign-in`) as having an
"unapproved verb". This is a false positive — the actual `is_approved_verb`
check in `🎮️command/🦀️.rs` runs against the Rust source's
`SemanticDescriptor.verb` field, a field independent of the JSON
`semanticKind` string (confirmed by reading `embed-font-file/🦀️.rs`: its
`SEMANTICS.verb` is `"insert"`, an approved verb, even though the mutation
kind is spelled `embed-font-file`). Not a merge concern; not touched.

## Result

- 330/330 directories resolved. 0 could not be decided.
- 307 kept `component.json`'s surfaces unchanged (already correct); 23 got
  surfaces recomputed to match the filesystem where the stub or the legacy
  file disagreed with reality.
- 23 (`stdio/pdf`) got `payloadSchema` corrected to the real schema filename.
- 10 got `displayName` acronym casing corrected (kept `component.json`'s).
- All 330 `🔣️component.json` files removed.
- Frozen-evidence fixtures (`🧫️fixtures/🧪️remaining-package-purity-authority/🔣️.json`,
  `🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json`) were outside the 330-dir set
  and were not touched (confirmed via `git status --porcelain`).

Working scripts (`analyze.py`, `analyze2.py`, `analyze3.py`,
`merge_and_validate.py`) are kept under `📜️scripts/` in this ticket folder for
reproducibility/audit.
