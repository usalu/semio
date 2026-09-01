# Historical Document Evidence — Design (before implementation)

Goal (from the assignment): a closed ticket's own documents are historical EVIDENCE, not live
references, and must not constrain the live tree. Generalize the existing per-*coordinate* frozen
mechanism to a precisely-bounded per-*document* exemption.

## 1. How candidates are admitted today

`🧹️normalization/🟦️.ts:3218` `repositoryReferenceCandidatePaths(repoRoot, taxonomy, context, cancelFile)`
is the single root that builds the set of paths the engine will scan as **reference sources**
(files whose text gets parsed for path-bearing tokens that may need rewriting when something
moves):

```ts
function repositoryReferenceCandidatePaths(repoRoot, taxonomy, context, cancelFile) {
  const ignored = (path) => isExcluded(path, taxonomy) || Boolean(context?.exactEvidencePaths.includes(path) || context?.transactionRoots.some(...));
  const paths = new Set<string>();
  for (const row of gitRows(repoRoot, taxonomy)) if (!ignored(row.path)) paths.add(row.path);
  for (const path of untrackedGitPaths(repoRoot, taxonomy)) if (!ignored(path)) paths.add(path);
  if (context?.ticketDir) for (const row of explicitTicketRows(...)) if (!ignored(row.path)) paths.add(row.path);
  return [...paths].sort(generatorPathCompare);
}
```

This is exactly `referenceClosure.candidateSource = "git-tracked-and-untracked-plus-explicit-ticket"`
in `🔣️taxonomy.json` — every tracked file (which includes every closed ticket's committed report)
sweeps into the candidate set unconditionally. `explicitTicketRows` additionally admits the
*active* ticket's untracked working files.

This function is the **single choke point** consumed by every reference-scanning path in the
engine:
- `incomingReferenceSnapshot` (:3310, line 3316) — builds `IncomingReferenceSnapshot.entries`,
  merged into the plan's `candidates` list at :4985, which is what actually produces
  `reference-syntax-unsupported` / `unsupported-path-syntax` violations (loop at :5064-5079, the
  `unsupportedReferenceTokens(...)` scan — this is the exact emitter for our target class, message
  format `${candidate.adapter} ${candidate.structuredLocation} contains unsupported path-bearing
  token ... targeting ...`, code `reference-syntax-unsupported`, structured location kind
  `"unsupported-path-syntax"` from `unsupportedReferenceTokens` at :4478).
- `capturePreflightReferenceBasis` (:6925) and `lexicalTargetIncomingReferences` (:6956, used for
  removal/embedded-root safety checks *and* post-transaction "stale reference" verification at
  :11122) — both call `repositoryReferenceCandidatePaths` directly.

Filtering at this one function therefore removes a path from **every** reference-scanning
obligation — plan-time violation scanning, apply-time preflight, and apply-time post-transaction
verification — with one change, not three.

## 2. How the existing frozen-evidence contracts neutralize references

`frozenMarkdownCoordinateEvidenceContracts` / `frozenCoordinateEvidenceContracts` (documented in
`📓️goal-frozen-contract.md`) freeze **individual byte-coordinates**: a contract pins one exact
document (`sha256` of the whole file) and a list of exact `(start, end)` spans plus a `sha256` of
each span's value. `isFrozenSourceCoordinateToken` / `frozenEvidenceCoordinateAuthority` (:6793,
:6830) look up whether a specific *token* at a specific *offset* in a specific *file* is in that
registered set; if so, that one token is exempted from rewriting/blocking, but every *other* token
in the same file is still live and still scanned.

This is why it cannot close the 74-row `unsupported-path-syntax` residue documented in
`📓️goal-frozen-report.md`: ~21 rows are unmarked prose the scanner's grammar deliberately never
admits as an "exact, mechanically re-derivable" span (headings/indented/blockquote already
excluded by design; loosening the grammar to admit raw prose would mean trusting the same loose
heuristic regex that raised the concern, which the frozen-report's own §1 follow-on explicitly
argues would defeat the contract's purpose). Two rows are absolute paths (not repo-relative,
correctly rejected). One is a substring of a larger span. ~42 are inside two live/unclear files
under a different, still-open ticket. None of these are fixable by registering more coordinate
entries — the grammar that admits coordinates is the limiting factor, not the registry.

The **coordinator's follow-up probe** shows this is not a long tail: every scope tried today,
however small, is 100% blocked by exactly this class (`🔢️number`: 28/28 unresolved rows are
historical-ticket/prose; `⏱️trace`: dominant fraction likewise). Per-coordinate registration would
require freezing spans in essentially every historical document in the repo, forever, and
re-freezing after every future rename — it cannot converge as a maintenance model.

## 3. Where a document-level rule attaches

The generalization is: **do not admit certain whole documents as reference candidates at all**,
rather than freezing tokens inside them one at a time. The natural insertion point is the `ignored`
predicate inside `repositoryReferenceCandidatePaths` (:3220) — it already has this exact shape
(`isExcluded(path, taxonomy) || ...`), so a new disjunct
`historicalDocumentEvidence(path, taxonomy, repoRoot)` composes cleanly and propagates through
every consumer listed in §1 automatically.

This is deliberately **not** the same mechanism as `isExcluded`/`taxonomy.exclusions`
(`pathExclusions`): that is a strictly-validated, closed 2-entry list (`compose/`, `temp/compose/`)
used for lexical-safety opacity, not an extensible population, and it also removes a path from
*every* taxonomy concern (inventory, package-boundary checks, etc.), which is broader than what we
want — ticket documents still need to exist, still need their own moves/inventory (if any) handled
normally; only their role as a *reference source* is being retired.

## 4. The population, precisely bounded

Two disjoint domains, both markdown, both authored narrative for humans, never machine-read at
runtime by anything outside their own ticket:

1. **`closed-ticket-report`** — files matching
   `**/.🧬semio/🦑️repo/🎫️tickets/🎆️[0-9][0-9]/🌙️[0-9][0-9]/☀️[0-9][0-9]/*/**` (the same governed
   ticket-path glob already validated elsewhere, e.g. `fixedFilenameContracts.cargo-cache-tag`)
   whose **basename** matches `^📓️.+\.md$`, **and** whose owning ticket's `🎫️ticket.json` has
   `status === "closed"` (reusing the existing `ticketManifestState` reader, :5990, already used by
   the ticket-important-history lifecycle mechanism).

   - The basename requires a **non-empty slug** after the `📓️` emoji. This is deliberate and
     excludes the bare kind-only leaf `📓️.md`. The repo's own convention (the subject of this whole
     ticket: "folders carry semantics, files are kind-only leaves") uses bare `📓️.md` / `📝️.md` /
     `📓️.json` as *machine-produced* evidence/fixture leaves inside semantically-named directories
     (confirmed real example:
     `.🧬semio/…/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-index-capture-66/🧫️run-*/evidence/📓️.json` and
     `…/evidence/📝️.md`, read directly by
     `🧰️framework/…/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-index-capture/🟦️.ts:22` — a real
     test that reads a specific ticket path at runtime). A slugged name like
     `📓️goal-frozen-report.md` is, by the same convention and by CLAUDE.md's own ticket protocol
     ("You MUST create a markdown file inside the ticket folder for every research or summary you
     do" — the slug *is* the document's identity, since folders don't carry a semantic name for a
     flat ticket root), always hand-authored narrative prose. No occurrence of a bare `📓️.md` exists
     in any ticket today (checked: zero matches repo-wide), so this guard costs nothing now and
     protects the load-bearing fixture population from ever being swept in by accident.
   - Requiring `status === "closed"` (not `"missing"` or `"invalid"`) is deliberately conservative:
     a ticket with a missing/invalid manifest could be mid-flight (e.g. a concurrent `ticket_open`
     not yet finished), not genuinely finished history. It also means the **active ticket's own**
     reports (this ticket, `status: "open"`) are never exempted while open — they remain live,
     reviewable, and reference-scanned exactly as before, satisfying "must not weaken anything for
     the active ticket's own inputs." The two ~255 MB anomalous files under
     `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL` (§6 below) are a second, real confirmation of this: that
     ticket currently has **no `🎫️ticket.json` at all** (verified: absent from the working tree,
     last tracked at commit `a8d1caf41f`, and the ticket is under heavy active edit right now per
     `git status`) — so `status === "closed"` is false, and my rule correctly leaves them exactly as
     blocked as the prior slice's own disposition ("do not freeze; flag to that ticket's owner").

2. **`cursor-plan-snapshot`** — files matching `.cursor/plans/*` whose basename matches
   `^.+\.plan\.md$`. These are Cursor IDE plan snapshots with no ticket-lifecycle concept at all
   (verified: `.cursor/plans/` is flat, ~430 files, no subdirectories, no manifest); they are
   unconditionally historical narrative from a separate tool, so `requiresClosedTicket: false`.

## 5. What is *not* touched

- `🎫️ticket.json` itself, `📌️important.md` / `📝️.md` (the existing `ticket-important` /
  `ticket-important-history` machine-governed lifecycle files), `ticket.md` (the primary document,
  separately projected by `projectTicketDocumentFiles`), and any file under a ticket folder that is
  **not** a `📓️<slug>.md` narrative report (test fixtures, evidence JSON, generated schema copies,
  `.rs` snapshots, logs) are unaffected — they keep exactly their current admission behavior.
- `pathExclusions` (the closed 2-entry opaque list) is untouched.
- `frozenMarkdownCoordinateEvidenceContracts` / `frozenCoordinateEvidenceContracts` are untouched
  and keep working exactly as before for every file that is *not* in the new population (e.g. any
  `.md` under an **open** ticket, or under any non-ticket path). For files that newly fall out of
  the candidate set, their existing frozen-coordinate entries simply become unreachable/unused —
  harmless, not wrong (verified: `validateObservedFrozenEvidenceNodes` at :6785 already treats an
  unobserved frozen path as a no-op, `if (!knownPaths.has(path)) continue;`).
- Production source, schemas, manifests, configs, and everything outside the two populations above:
  identical behavior, since the new predicate only ever returns `true` for paths matching one of the
  two narrow glob+basename+status shapes above.

## 6. Load-bearing check (required by the task)

Searched for any test or runtime code that reads a **closed**-ticket `📓️<slug>.md` report by path.
Found one real case of a ticket path read at runtime by a test —
`🧪️tests/🧬️mutation-inventory/🧪️source-index-capture/🟦️.ts:22` — but it reads
`…/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-index-capture-66` (a nested fixture directory of
kind-only-leaf evidence files: `evidence/📓️.json`, `evidence/📝️.md`), not a slugged `📓️<name>.md`
report, and that ticket is not closed regardless. No slugged `📓️<slug>.md` report is read by any
test or production code anywhere in the repo (only greps hit were generic ticket-path *glob
patterns* in taxonomy validators, not literal reads of specific report files). Conclusion: nothing
in today's repo is load-bearing on this population; the exemption is safe as designed. If that ever
changes (a future test wants to assert against a closed ticket's report content), the fix is to
give that specific file a different name (drop the `📓️` prefix or add machine metadata elsewhere),
not to weaken this rule — matching "no compatibility layer / no opt-out flag."

## 7. The two anomalous ~255 MB files

`.🧬semio/…/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-current-roster-54-current-2026-08-27T23-15-09-245Z-🧫️run-1KTP5R.md`
(≈162 MiB) and `…-2026-08-28T00-22-22-543Z-🧫️run-k7yzCQ.md` (≈272.7 MiB) — **not edited**. This
change does **not** make them moot: their owning ticket has no `🎫️ticket.json` (hence
`ticketManifestState` reads `"missing"`, not `"closed"`) and is under active edit right now, so
`requiresClosedTicket` correctly refuses to exempt them. This matches the prior slice's own
disposition exactly ("do not freeze; flag to that ticket's owner") — the design doesn't change the
outcome for these two files, it just reaches the same correct answer for a different, principled
reason (open ticket, not "can't freeze a growing file").

## 8. Implementation shape (schema-first, per CLAUDE.md)

`🔍️discovery/🟦️component.ts`: new exported type `HistoricalDocumentEvidencePopulation` (`grammar`,
`directoryPattern`, `leafPattern`, `requiresClosedTicket`, `reason`), a strict validator
`validateHistoricalDocumentEvidencePopulations` requiring **exactly** the two ids
`closed-ticket-report` / `cursor-plan-snapshot` with their exact field values hardcoded-checked
(mirroring `pathExclusions`'s and `cargoCacheTag`'s exact-value validation style — a closed,
non-extensible 2-entry contract, not a growing list), a new `Taxonomy.historicalDocumentEvidencePopulations`
field, wired into `validateTaxonomy`. A new descriptive key
`referenceClosure.historicalDocumentEvidence` (validated exactly, alongside the other
`referenceClosure` literals) documents the rule in the same self-describing vocabulary object the
task pointed at.

`🧹️normalization/🟦️.ts`: exported `historicalDocumentEvidence(path, taxonomy, repoRoot)` — matches
`directoryPattern` via the existing `taxonomy.pathMatcher` glob engine (already supports `**/`,
`*`, `[0-9]` classes — reused, not reinvented), `leafPattern` via `new RegExp(..., "u")` against
`posix.basename(path)` (same pattern as `mutationDirectoryPattern`'s existing use), and for
`requiresClosedTicket` populations extracts the ticket root via an anchored regex and reads its
`🎫️ticket.json` through the same `absolutePath` + `lstatOrNull` + `readFileSync` +
`ticketManifestState` sequence `projectTicketImportantFiles` already uses. Wired into the single
`ignored()` predicate in `repositoryReferenceCandidatePaths`.

## 9. Expected effect on `moves` / `edits` / `regenerations`

Ticket documents are never in any taxonomy `scope` used for this ticket's moves (they live under
`.🧬semio/…/🎫️tickets/`, never under `🧰️framework/🔨️modules/…`), so they are never inventory/move
candidates — `moves` and `regenerations` are structurally unaffected by this change (verified by
code path: they derive from `inventoryTaxonomy`'s scoped filesystem walk, entirely separate from
`repositoryReferenceCandidatePaths`). `edits` can only ever *decrease*: previously, if a token
inside one of these documents happened to be safely rewritable, the engine would silently rewrite
historical prose to the new path — which is itself a correctness bug relative to this ticket's own
stated rationale ("rewriting them would falsify the historical record"). Removing that possibility
is the intended, correct side effect, not a regression. Verified empirically in
`📓️goal-histref-report.md`.
