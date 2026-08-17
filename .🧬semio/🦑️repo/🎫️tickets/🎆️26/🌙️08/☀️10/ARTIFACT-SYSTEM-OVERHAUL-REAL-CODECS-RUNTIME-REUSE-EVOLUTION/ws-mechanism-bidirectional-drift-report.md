# WS: Bidirectional facet-mirror-drift check

## What changed

File: `📜️script.ts` (only file touched besides this report + scratch scripts below).
Function: `policyFacetMirrorDriftBreaches` — at edit time it sat at line 9444 (function body ends
~9491) before the change; after the change it sits at **line 9471** (body ends **9547**), inside
the existing `//#region 🔧️PolicyRuleSchemaOverhaulS2` region. `git diff --stat` for `📜️script.ts`:
165 insertions(+), 4 deletions(-), 1 file changed.

The rule previously only checked "every Rust field appears (as a substring) somewhere in each
sibling `.ts`/`.graphql`/`.json`/`.proto` leaf" (forward direction). That direction alone cannot
catch a sibling still shaped like the generic W1b scaffold (`entries: [{key, value}]`) sitting next
to/instead of the real fields, because "every real field is a substring of the sibling text" can be
trivially true while the sibling ALSO declares a bunch of fields that don't correspond to anything
real. This wave adds the missing reverse direction: harvest each sibling's own declared field names
and flag any that have no real Rust counterpart.

### New code, in the existing `policyFacetMirrorDriftBreaches` region

- `POLICY_FACET_MIRROR_DRIFT_TS_RE` / `_TS_KEYWORDS` + `policyFacetTsFieldNames` — TS interface/type
  member harvester (comment+string-stripped via the existing `policyStripTsCommentsAndStrings`).
- `POLICY_FACET_MIRROR_DRIFT_GRAPHQL_RE` + `policyFacetGraphqlFieldNames` — strips `#` comments and
  parenthesized argument lists, then regexes `name:` field lines.
- `policyFacetJsonFieldNames` — `JSON.parse`s the leaf and recursively collects every key under any
  `"properties"` object (incl. nested under `definitions`/`$defs`); throws on invalid JSON, which the
  caller turns into a `sib:PARSE_ERROR` entry instead of crashing the policy run.
- `POLICY_FACET_MIRROR_DRIFT_PROTO_RE` / `_PROTO_SKIP_RE` + `policyFacetProtoFieldNames` — numbered
  message-field regex, skipping `enum`/`oneof`/`message`/`package`/`syntax`/`import` lines.
- `policyFacetSiblingFieldNames(sib, content)` — dispatches to the right harvester by file suffix.
- `policyFacetRustTagFieldNames` + `POLICY_FACET_MIRROR_DRIFT_SERDE_TAG_RE` — pulls
  `#[serde(tag = "kind")]` discriminant VALUES out of the raw (unstripped) Rust source. These are
  real fields every non-Rust mirror must spell out (no native tagged-union sugar), but
  `policyFacetRustFieldNames` never sees them since there's no `kind: SomeType` struct-field
  declaration — only this attribute.
- `policyFacetRustVariantFieldNames` + `POLICY_FACET_MIRROR_DRIFT_VARIANT_RE` — camelCases
  PascalCase enum variant names (`SetHeaderFields { .. }`, `Rgb { r, g, b }`) found at line-start
  followed by `{`/`(`. Same problem as the serde-tag case but for the externally-tagged / `oneof`
  convention: proto spells a mutation enum's variant names out as message/oneof-arm identifiers
  (`SetHeaderFields set_header_fields = 3;`), which `policyFacetRustFieldNames` (fields-only) never
  captures either.
- Main loop: for each sibling, on top of the existing `missingFields` substring check, it now also
  harvests `siblingFields`, builds `compareFields = camelFields ∪ tagFields ∪ variantFields`, and
  computes `extraFields = siblingFields` minus anything that exactly matches `compareFields` OR is
  case-insensitively CONTAINED by-substring in a `compareFields` entry ≥4 chars (handles the
  disambiguating-prefix convention seen in stdio.json's real mirror: Rust's plain `value` field
  becomes `boolValue`/`stringValue` per enum variant in GraphQL/proto, which have no tagged-union
  sugar of their own). `schema` is exempted (every facet root always carries it). If `extraFields.length
  > 0`, `${sib}:extra:${extraFields.length}` is appended to the SAME `missingBySibling` array the
  forward check already builds — same breach `id`, same `kind: "stdio-artifacts/facet-mirror-drift"`,
  same `POLICY_FACET_MIRROR_DRIFT_ALLOWLIST` key format. No new allowlist, no new exported function,
  no aggregator signature change (`policySchemaOverhaulS2Breaches` is untouched).
- Summary/reason/solution text on the breach record now says "missing and/or extra" and explains
  both directions; wording only, same fields.

## Verification

### Baseline
`git status --porcelain -- 📜️script.ts` was clean before the first edit (confirmed).

### CLI smoke test
`bun run ./📜️script.ts policy` runs to completion, no crash, no `TypeError`/`ReferenceError`/stack
trace anywhere in output (21636 lines). Exits 1 because of pre-existing HIGH-priority breaches from
concurrent work (`os-state-authority/*`, `budget/no-budget-null`, dependency-cruiser circular-import
errors under `verify`) — all unrelated to this rule (`facet-mirror-drift` is `priority: "low"`, so it
never prints via the CLI's default high-only filter regardless of this change; this matches a prior
session's own documented finding, not a new discovery).

### Real signal (bypasses the CLI's high-only filter)
Two scratch scripts in this ticket folder (kept, reusable):

- `ws-scratch-check-shape.ts` — imports the exported `policySchemaOverhaulS2Breaches` (not
  `policyFacetMirrorDriftBreaches` itself, which isn't exported) and filters `kind ===
  "stdio-artifacts/facet-mirror-drift"`. Output: `ws-scratch-check-shape-output.txt`.
- `ws-scratch-allowlist-bypass-check.ts` — the aggregator's allowlist SUPPRESSES breaches for facets
  still listed in `POLICY_FACET_MIRROR_DRIFT_ALLOWLIST`, so "0 breaches shown" for an allowlisted
  standard is ambiguous (clean vs. still-drifting-but-silenced). This script reimplements the exact
  same harvester math directly against json/png/zip/gif/bmp's facet dirs, bypassing the allowlist
  entirely, to get ground truth. Output: `ws-scratch-allowlist-bypass-output.txt`.

**Before/after count** (reconstructed from the same run's own breach summaries — every breach
that would ALSO have fired under the old forward-only logic vs. one that fires ONLY because of the
new reverse check):
- BEFORE (forward-only): 64 real `facet-mirror-drift` breaches (across all non-allowlisted
  schema-owning facets repo-wide)
- AFTER (forward + reverse): 66 real breaches
- 2 NEW breaches surfaced purely by the reverse check (both previously invisible to the forward-only
  rule): `stdio/semio/v1/subsets/drawing`'s `📸️snapshot` and `🧬️mutations` facets.
- Of the 66 AFTER breaches, 27 mention an `:extra:` token (most are on top of an already-drifting
  facet that also has missing fields; only the 2 above are extra-only/newly-surfaced).
- 0 `:PARSE_ERROR:` (no malformed JSON sibling encountered in this sweep).

### Known-real mirrors stayed clean (ground truth via the allowlist-bypass script)
Directly checked, allowlist bypassed, all 3 facets (snapshot/diff/mutations) per artifact:
- `🔣️json` (rfc8259/any) — 0 `:extra:` breaches across all 3 facets × 4 siblings.
- `📷️png` (1.2/any) — 0 `:extra:` breaches across all 3 facets × 4 siblings.
- `🖼️bmp` (v3/any) — 0 `:extra:` breaches across all 3 facets × 4 siblings.
- `🗜️zip` — the allowlist-bypass script's subset-discovery walk found 0 schema-owning subset dirs
  for this artifact path (glob mismatch in the throwaway script, not the real policy code — the
  real aggregator run does enumerate zip via `policyListStdioSchemaOwningEntries` and shows 0
  breaches for it too); not independently re-verified beyond that, flagged here for transparency
  rather than silently claimed as "checked."
- `🎞️gif` — correctly DOES show `:extra:` (`[name,entries]` / `[name,data,entries]`) on its
  `📸️snapshot`/`🧬️mutations` facets' ts/graphql/json siblings. This is the fix working as intended:
  the function's own pre-existing docstring already says "gif's TS mirror is still literally zip's,
  per the master plan's own opening finding" — `entries`/`name`/`data` are zip's real field names
  leaking into gif's still-scaffolded mirror. This is a true positive, not noise.

### Bugs found and fixed during verification (all in my own new code, not pre-existing)
1. **Serde-tag discriminants** (`kind`): stdio.json's `JsonValue` enum uses
   `#[serde(tag = "kind")]`; every real mirror spells `kind` out as an actual field, but no Rust
   struct declares a `kind` field — fixed via `policyFacetRustTagFieldNames`.
2. **Case-sensitive substring bug**: my first cut at "sibling field renamed with a disambiguating
   prefix" (`boolValue`/`numberLexeme`/… vs. Rust's plain `value`/`lexeme`) used a case-SENSITIVE
   substring check, so `"boolValue".includes("value")` was `false` (capital `V` from the prefix
   convention). Fixed by lower-casing both sides before the substring test.
3. **Externally-tagged enum variant names** (`SetHeaderFields`, `Rgb`, …): png's `Rgb { r, g, b }`
   color variant and bmp's `BmpMutation::SetHeaderFields { .. }` mutation variant both surface their
   PascalCase variant name as a real field/message/oneof-arm identifier in proto, with no
   corresponding named Rust struct field anywhere — fixed via `policyFacetRustVariantFieldNames`.

### One known, accepted residual false-positive class (documented, not fixed)
`stdio/semio/v1/subsets/drawing`'s `📸️snapshot` facet's TS/JSON siblings locally re-declare the
shape of `SemioPoint2`/`SemioRgba`/`SemioTransform` (imported via `use …::engine::geometry::{…}` —
these types are DEFINED in a different `.rs` file, not this facet's own). Their constituent fields
(`x`,`y`,`z`,`w`,`r`,`g`,`b`,`a`,`translation`,`rotation`,`scale`) are legitimately not in this
file's local Rust field set, so the reverse check currently flags them as `:extra:`. This is a
structural limitation of a single-file textual heuristic (this rule was never a real cross-file type
checker even before this wave — the SAME limitation already existed for the forward direction, just
never triggered), not something a regex tune inside this facet's own comparison logic can close
without resolving Rust `use` imports across files, which is out of scope for this heuristic. Left as
an accepted, understood, narrow edge case (2 breach records) rather than papered over with an
allowlist entry — per the task's explicit instruction, I did not seed/prune
`POLICY_FACET_MIRROR_DRIFT_ALLOWLIST` in this wave; a future session can decide whether to allowlist
`stdio/semio/v1/subsets/drawing` citing this ticket, or teach the harvester to also resolve
`engine::geometry`'s own field set.

### `verify` gate
`bun run ./📜️script.ts verify` fails, but on `dependency-cruiser` circular-import errors entirely
inside `🧰️framework/…` module graph (764 pre-existing violations, none touching `📜️script.ts` or
anything I edited) — consistent with this repo's documented pattern of concurrent-session churn
causing unrelated build/verify noise. Not caused by this change: my edit adds no new imports and
touches no file besides `📜️script.ts`'s own function bodies.

## Files touched
- `/Users/ueli/Documents/semio/📜️script.ts` — the one edit (see above).
- `/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/ws-scratch-check-shape.ts` (new, kept, reusable)
- `/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/ws-scratch-check-shape-output.txt` (new, latest run's output)
- `/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/ws-scratch-allowlist-bypass-check.ts` (new, kept, reusable)
- `/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/ws-scratch-allowlist-bypass-output.txt` (new, latest run's output)
- `/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/ws-mechanism-bidirectional-drift-report.md` (this file)

## Not done (out of scope per the assignment)
- Did not touch `POLICY_FACET_MIRROR_DRIFT_ALLOWLIST` (no seeding/pruning this wave).
- Did not touch the forward-direction `missingFields` logic or `policyFacetRustFieldNames` itself
  (pre-existing behavior — e.g. it also picks up private/internal struct fields like a local
  `Parser` implementation type's `bytes`/`pos`/`line`/`col`, inflating `camelFields` — confirmed
  during verification but left alone since it predates this wave and isn't part of the assignment).
- Did not run `cargo check` — no `.rs` file was touched.
- Did not call `ticket_close`/`ticket_reopen`, did not run any git commit/stash/checkout/reset.
