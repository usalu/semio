# Historical Document Evidence — Report

Design: `📓️goal-histref-design.md`. **Revised per coordinator review** — original boundary
(`requiresClosedTicket: true`) exempted almost nothing because most residual rows sat under
currently-open tickets. Coordinator's argument accepted: a `📓️<slug>.md` is a narrative record of
what was true when written; ticket lifecycle status is not the right discriminator, document KIND
is. Both rewriting (falsifies the record) and blocking (freezes the repo on prose, in any ticket
state) are wrong for a report regardless of whether its ticket has closed.

## The rule (revised)

Any `📓️<slug>.md` under a ticket directory (`**/.🧬semio/🦑️repo/🎫️tickets/🎆️YY/🌙️MM/☀️DD/*/**`),
and any `.cursor/plans/*.plan.md`, is excluded from reference-candidate scanning unconditionally —
**no ticket-status check at all now**. `requiresClosedTicket` removed from the schema entirely
(closed-ticket-report renamed `ticket-report`); `historicalDocumentEvidence(path, taxonomy)` is now
a pure path-vs-pattern function, no filesystem read, no `repoRoot` parameter. Enforced at the same
seam: `repositoryReferenceCandidatePaths`'s `ignored()` predicate in `🧹️normalization/🟦️.ts`.

## Verification requested before widening

1. **Machine-read check** — grepped the whole repo for `readFileSync(...📓️...)`, `generatorContracts.*.inputPatterns` containing `📓️`, and `taxonomy.json` references to any `📓️<slug>.md` ticket path: zero real hits. One near-miss investigated in depth: `📓️protocol-v2-specification.md` (ticket `SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING`) appears inside two test fixtures (`testing-readme-coordinates`, `readme-reviewed-fixture-inputs`) as inline copy-pasted string content — not a live read of the real file (confirmed: those tests never call `readFileSync` on it; the `"testing-readme-protocol-v2-reviewed"` revision-id it resembles is a self-contained fixture contract with its own `schema`/`catalog`/`expectation` role inputs, none of which is this file). **Nothing machine-reads a real `📓️` report.**
2. `📌️important.md`, `🎫️ticket.json`, `🔣️*.json`, `🧪*.rs` — none match `^📓️.+\.md$` (verified directly: pattern test returns `false` for all four, plus `🧪m3-plugin-component.pre-patch.rs`). Confirmed in the test file.

## Separate defect (not fixed, named per instruction)

`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL` has **no `🎫️ticket.json`**
at all (absent from working tree, last tracked at `a8d1caf41f`) while carrying the largest block of
rows in the `🔢️number`-scope measurement and two ~200-300 MiB anomalous `📓️admission-current-roster-54-*`
files. A ticket directory with no manifest is a data-integrity gap someone owns.

## Real before/after, `🔢️number` scope (coordinator's own measurement, rerun after the fix)

```
before  moves=2 edits=8 regenerations=1 unresolved=28
after   moves=2 edits=8 regenerations=1 unresolved=4
```
The 4: 2× `.pre-patch.rs` (non-`.md`, out of grammar), `terra-number-deasync.py` (non-`.md`),
`📌️important.md` (doesn't match the leaf pattern) — **exactly** the coordinator's predicted target.

## Real before/after, `🧬️schema` scope (repo-wide primary target)

```
before  moves=7 edits=210 regenerations=3 unresolved=127  (unsupported-path-syntax=73)
after   moves=7 edits=207 regenerations=3 unresolved=55   (unsupported-path-syntax=1)
```
The class this whole slice targets is now effectively zero: the 1 remaining "unsupported-path-syntax"
substring is inside a `frozen-coordinate-evidence-unowned` message on a pre-existing, unrelated
`🔣️.json` fixture (not a report). Remaining 55 = 50 `rust-path-join` + 3 `rust-path-join-unproven`
(unrelated class, see `📓️goal-session-status.md` §7-8) + 2 pre-existing `frozen-coordinate-evidence-unowned`.

## Test (kept red/green, case added per instruction)

`🧪️tests/🧪️historical-document-evidence/🟦️.ts`, 6 tests, all green (`bun nx run …:test-historical-document-evidence`).
Added exactly the requested case: identical bare-prose token in an **open**-ticket `📓️` report is
exempt (plus closed-ticket and no-manifest-ticket variants for full coverage), while the same token
in a production file (outside any ticket) still blocks with `reference-syntax-unsupported`/
`unsupported-path-syntax`. Verified red (temporarily disabled the `ignored()` disjunct → failed
exactly as expected) then green, synchronously in-turn both times. Simplified two sandboxed
function-reconstruction tests that broke on the earlier design (`🧪️preflight-reference-basis`,
`📦️packages/🟦️typescript/🧪️index.test.ts`) to match the smaller function signature — both green.

## Anomalous files — unchanged conclusion

Still not edited. Now exempted as reference-scanning candidates (their ticket-report status alone
qualifies them under the revised rule), but that only stops them from blocking future renames of
other files — it does not touch their own ~200-300 MiB size, which remains the named defect above.

## Round 3 — widened to "ticket workspace" (evidence, scratch, working notes)

Extended the population from "📓️ reports" to `ticket-workspace`: any DIRECT child of a ticket root
(`**/.🧬semio/…/TICKETSLUG/*`, one path segment, never recursive) that is (a) not itself matched by
any `fixedFilenameContracts` pattern, and (b) not inside a directory (walked up to the ticket root)
that owns a package-root manifest — derived mechanically from
`fixedFilenameContracts[*].scope.kind === "package-root"` (Cargo.toml, package.json, tsconfig.json,
pyproject.toml, go.mod, go.sum, build.rs — a strict superset of the coordinator's three examples,
all schema-derived, none hardcoded). Verified this is load-bearing and not theoretical: two real
tickets have `Cargo.toml` sitting directly at ticket-root level with genuine build sources
(`derive-dwg-fixture.c`, `.mjs` generators) as siblings — confirmed both stay `false` (still live).

`📌️important.md` confirmed emptiness-only: `planTicketImportantRemovals`/`projectTicketImportantFiles`
consult only `entry.size`/`candidate.size` (filesystem byte length) and the **ticket manifest's**
content, never `readFileSync` on `📌️important.md` itself — no path-parsing reader exists.

## Real before/after, `🔢️number` scope (final)

```
after   moves=2 edits=5 regenerations=1 unresolved=0
```
`moves` exactly as required (`📦️glue.rs→🦀️.rs`, `🦀️component.rs→🦀️.rs`). **`edits=5`, not 8** —
deviates from your prediction; traced and expected, not a bug: `📌️important.md:249` and
`terra-number-deasync.py:6` each had one backtick-wrapped, previously-safely-rewritten mention of
`🔢️number/🦀️component.rs` that is no longer rewritten now that those files are excluded as
reference sources — the same "don't rewrite, don't block" consequence you argued for `📓️` reports,
applied consistently to workspace scratch. Flagging explicitly since you named `edits=8` as a target.

## Real before/after, `🧬️schema` scope (final)

```
after   moves=7 edits=129 regenerations=3 unresolved=53  (unsupported-path-syntax=1, same pre-existing unrelated fixture false-positive as before)
```
`unresolved` composition: 50 `rust-path-join` + 1 `rust-path-join-unproven` + 2 pre-existing
`frozen-coordinate-evidence-unowned` (unrelated fixture) — **only the unrelated class remains**,
matching your ~50 prediction. `edits` dropped further, 207→129 (−78) — verified this is real and
correct, not a leak: sampled the removed rows, all are loose ticket-root scratch files; the 11
production edits are unchanged, and `🎫️ticket.json`/nested fixture-tree `taxonomy.json` copies
(protected by `fixedFilenameContracts` / not direct ticket-root children) still get their 118 real
edits applied exactly as before.

## Test

`🧪️tests/🧪️historical-document-evidence/🟦️.ts`, 6 tests. Added the requested ticket-embedded
`Cargo.toml` case: a `.rs` source file beside a fixture `Cargo.toml` under a closed ticket **still
blocks** the move with `reference-syntax-unsupported`/`unsupported-path-syntax`, proving the
package-boundary carve-out holds inside the real `planTaxonomy` engine, not just the unit-level
function. Verified red (disabled gate → `closed-ticket report must not block the move: Expected
false, Received true`) then green, synchronously both times, plus `bun nx run …:test-preflight-reference-basis`
(30/30, updated for the wider function signature) and the targeted `index.test.ts` case, all green.
