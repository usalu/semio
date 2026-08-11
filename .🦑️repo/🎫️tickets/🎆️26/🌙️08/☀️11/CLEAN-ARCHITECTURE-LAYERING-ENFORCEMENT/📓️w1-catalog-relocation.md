# W1 — Stdio Owner-Table Catalog Relocation

## Task
Move the stdio format roster / owner-table catalog out of the generic
`🧰️framework` tree into the `🗄️stdio` plugin that actually owns it, and
repoint `📜️script.ts`'s loader at the new canonical path with no legacy
fallback.

## Before / after paths
- Before: `🧰️framework/🔨️modules/🚪️io/📇️registry/📇️catalog.json`
- After: `✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json`
- Moved with `git mv` (working tree + index both reflect the rename;
  content unchanged, confirmed via directory listing before/after — file
  size 35042 bytes preserved).
- Old dir `🧰️framework/🔨️modules/🚪️io/📇️registry/` deleted (`rmdir`, was
  empty after the move). Its parent `🧰️framework/🔨️modules/🚪️io/` still
  holds its own `🦀️component.rs`, untouched.

## script.ts changes (region 🔧️PolicyRuleArtifactIo, ~line 6797)
- `POLICY_STDIO_OWNER_TABLE_REL` now points at
  `✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json`.
- `POLICY_STDIO_OWNER_TABLE_LEGACY_REL` constant deleted outright (was
  pointing at a closed ticket-folder path
  `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/STDIO-ARTIFACTS-AND-IO/🧪owner-table.json`).
- `policyLoadStdioOwnerTable`'s legacy-fallback branch (the `existsSync(legacyAbs)` /
  second `JSON.parse` read) deleted — the function now only ever reads
  `abs` (the one canonical path) and returns `null` if that's missing. No
  compatibility shim left behind.
- Docstring above the constants rewritten to describe the new home instead
  of the old "one-wave read fallback" note.
- Confirmed no remaining references to `POLICY_STDIO_OWNER_TABLE_LEGACY_REL`
  anywhere in `📜️script.ts` (`grep` after edit: 0 matches).

## Verification

### `bun ./📜️script.ts verify gate`
Ran to completion (exit 1). Failed at its very first step —
`nx run @semio-tech/plugin-registry:check` — on breaches entirely
unrelated to this change: missing `🦀️component.rs` files under
`✏️s/🔌️plugins/🪵️sourcing`'s `🗂️curate` app/artifact tree (glue.rs
`#[path = ...]` declarations pointing at files that don't exist on disk —
another session's in-flight work, not touched by me) plus a large batch of
pre-existing stdio-artifact *completeness* breaches (missing
`🧬️schema/`, `⚙️engine/🦀️component.rs`, etc. under individual artifacts
like `💬️bcf`, `🔣️json`, `🖊️dxf`, …) that are unrelated to catalog
location — those are the same "many inconsistencies to refactor" class of
finding this ticket's other waves are chewing through, not something my
catalog move introduced. Full output saved to
`w1-verify-gate-output.txt` (1MB+, gitignored-style scratch file, kept
in ticket folder per instructions). Grepped for any mention of the old or
new catalog path or of `POLICY_STDIO_OWNER_TABLE*` — zero hits either
way, confirming the catalog path itself was never implicated in the gate
failure.

**Important finding**: `verify gate` (`VerifyScript.runGate`) does *not*
directly invoke `policyStdioCatalogBreaches` / `policyIoSerializerMatrixBreaches`
/ `policyIoTerminalityBreaches` / `policyCodecFidelityBreaches` — those
are only reached via the separate `policy` lint aggregator
(`export const policy = defineLint(...)` at ~line 9248, which calls
`policyStdioArtifactsBreaches` at ~line 9297). `📜️script.ts` itself has a
comment on this exact distinction (~line 8217-8220): *"a DIFFERENT,
narrower gate pipeline than `bun ./📜️script.ts policy` ... `policy` is
the command that actually runs these rules; always verify against
`policy`, not `verify`, when touching anything under `//#region
🔧️PolicyRule*`"*. So I additionally ran `policy` directly.

### `bun ./📜️script.ts policy`
Ran to completion (exit 1) but crashed with an uncaught `ENOENT` before
ever reaching `policyStdioArtifactsBreaches` in the aggregator's call
order: `policyPackCompletenessBreaches` threw trying to read
`✏️s/🔨️modules/◻2d/⚙️engine/🦀️component.rs`, which doesn't exist on
disk right now — again another session's concurrent in-flight churn
(matches the known "Concurrent Cargo Workspace Churn" pattern), not a
file I own or touched. Full output in `w1-policy-output.txt`.

### Isolated direct verification (since both aggregate commands are
currently blocked by unrelated concurrent breakage upstream of the stdio
checks)
Wrote a small scratch script,
`w1-check-stdio-checks.ts` (kept in this ticket folder), that imports and
calls the four functions directly against `repoRoot =
"/Users/ueli/Documents/semio"` and reports breach counts plus any
`stdio-catalog-owner-table-missing` breach specifically. Ran with
`bun run`. Result (saved in `w1-check-stdio-checks-output.txt`):

```
[policyStdioCatalogBreaches] ran OK — 0 breach(es) total; owner-table-missing breaches: 0
[policyIoSerializerMatrixBreaches] ran OK — 0 breach(es) total; owner-table-missing breaches: 0
[policyIoTerminalityBreaches] ran OK — 0 breach(es) total; owner-table-missing breaches: 0
[policyCodecFidelityBreaches] ran OK — 0 breach(es) total; owner-table-missing breaches: 0
```

All four functions ran without throwing and found **zero** breaches,
including zero `stdio-catalog-owner-table-missing` breaches (the id
`policyStdioCatalogBreaches` emits specifically when
`policyLoadStdioOwnerTable` returns `null`). This confirms:
1. `policyLoadStdioOwnerTable` successfully resolves the catalog at its
   new path (`POLICY_STDIO_OWNER_TABLE_REL` after the edit).
2. The roster count in the moved file still matches the normative count
   (29), so `stdio-catalog-roster-count` didn't fire either.
3. All three downstream checks that also call
   `policyLoadStdioOwnerTable` (serializer matrix, terminality, codec
   fidelity) load and process the relocated table cleanly.

## Files touched
- Moved: `🧰️framework/🔨️modules/🚪️io/📇️registry/📇️catalog.json` →
  `✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json` (new dir created,
  old empty dir removed).
- Edited: `📜️script.ts` — `POLICY_STDIO_OWNER_TABLE_REL` /
  `POLICY_STDIO_OWNER_TABLE_LEGACY_REL` constants and
  `policyLoadStdioOwnerTable`'s fallback branch only (region
  `🔧️PolicyRuleArtifactIo`, lines ~6797-6853 before edit).
- Scratch (kept in ticket folder, not deleted per instructions):
  `w1-check-stdio-checks.ts`, `w1-check-stdio-checks-output.txt`,
  `w1-verify-gate-output.txt`, `w1-policy-output.txt`.

## Not touched / did not close
Did not run `ticket_close` or `ticket_reopen` — this is one file-ownership
slice of a larger shared ticket; other waves (geometry relocation,
registry genericization per the sibling progress files already in this
folder) are being handled by parallel agents.
