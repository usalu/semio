# Report — `🎭️actor` / `📡️replication` / `🎠️kernel`

Baseline commit: `bb06c41f73f0122fbed315b7487428b976f99921`. Full family analysis in `📓️goal-ark-census.md`.

## Real output, before / after

```
                before(r1)              after(final, real pasted output)
actor        moves=55 unresolved=96   moves=71 unresolved=106   (see note below)
replication  moves=53 unresolved=41   moves=64 unresolved=8
kernel       moves=43 unresolved=29   moves=50 unresolved=23
```

Round-1 and replication-r3 lines are copy-pasted CLI output; actor/kernel final numbers are from the
round-2 run (actor: `🗑️temp/ark-actor-run2.log`, kernel: `🗑️temp/ark-kernel-run2.log`).

No scope reached `unresolved=0`. None is safe to `apply` yet.

## What the fixes did (🔣️taxonomy.json only; `🧹️normalization/🟦️.ts` untouched)

Registered 17 new implementation-neutral `semanticDirectoryKinds` (real on-disk domain concepts with
zero prior registration, each checked against all 213 existing kinds for collision first): `metadata`,
`patch`, `admission`, `instance`, `output`, `mailbox`, `turn-scheduler`, `shard-runtime`,
`shard-client`, `wire-turn`, `root`, `transport`, `test-fixture-schema-authority` (🛂️),
`asset-binary-subject` (📦️), `query`, `topology-authority`, `descriptor-load`, `turn-patch-owner`.
Extended `schema` kind's alternation with `wire` (reuse, not a synonym — single real occurrence,
content is a wire-format record-frame descriptor). Extended `test-case.parentKindIds` (71→79) with
`metadata, patch, admission, instance, output, root, transport, input` — this was **the single
largest shared family across all three scopes** (17 of the ~64 non-refsyn rows): a `🧪️fixture.json`/
`🧪️schema.json` beside any component always resolves both `test-case` and `test-fixture-member`
candidates, and disambiguation depends entirely on the parent id being in that allow-list. Added
`fileKinds.json-fixture` (mirroring `json-schema`) plus its `fileKindResolutionRules` entry for
`.fixture.json`, an unregistered extension chain already in use in two places repo-wide.

This cleared **all 27 non-refsyn rows in actor, all 33 in replication, all 7 in kernel** on the first
verification pass (moves rose, e.g. actor 55→71). `discovery.validateTaxonomy()`: 0 errors throughout.

## A real regression, found and fixed before reporting

Verification also surfaced a NEW 20-row `semantic-stem-ambiguous` family in replication
(`asset-binary-subject, asset-subject`) that my own first pass caused: I "corrected" the registered
`fileKinds.binary` emoji from `💾️` to `📦️` to match what's on disk. That was wrong — I read
`resolveFileKind` (🧹️normalization/🟦️.ts:2243) and found that when a file's on-disk emoji equals its
**own fileKind's registered emoji**, the emoji is stripped as decoration *before* semantic-stem
folding, so the stem then hits `matchDirectoryKind` with no emoji at all, where it matches *any*
default (`inferWithoutEmoji`-eligible) `assets`-parented kind — not just the one I intended. The
established, working precedent (`video` fileKind emoji 🎬️ vs. its `asset-video-subject` folder emoji
🎥️) confirmed file-kind-leaf and subject-folder emoji are **deliberately kept different** so this
exact evidence chain survives. Reverted `binary` to `💾️`, kept `asset-binary-subject` at `📦️`.
Re-verified live: replication → **moves=64, unresolved=8** (was 41; real pasted CLI output above).

**Generalizes beyond these three scopes**: any future `asset-*-subject` registration must keep its
paired `fileKinds` emoji different from the subject-folder emoji, or it silently collides with
`asset-subject` the instant a no-emoji stem reaches `matchDirectoryKind`. Worth a repo-wide grep for
any other fileKind whose emoji was set equal to a sibling subject-folder kind's emoji.

## Remaining unresolved (real, not re-fixed this pass)

- **~76 rows (actor 59, kernel 17), unchanged**: `reference-syntax-unsupported` prose in a DIFFERENT,
  foreign ticket (`FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG`), which has a `Cargo.toml` sitting directly
  at ITS OWN ticket root. Traced this fully: `historicalEvidenceBoundaryOwns`
  (🧹️normalization/🟦️.ts:3369) correctly treats the whole ticket directory as package-owned once a
  manifest sits at its root — verified via the neighbouring test
  (`🧪️tests/🧪️historical-document-evidence/🟦️.ts`), which has a matching case
  ("a loose C source sibling of a real ticket-root Cargo.toml stays live") and a docstring that
  explicitly rejects narrowing this by file extension. **This looked like a bug and is not one** — a
  disproved hypothesis, not a fix. The real remedy is that foreign ticket moving its root `Cargo.toml`
  into a subdirectory; out of this slice's scope.
- Actor's r2 run surfaced 4 new rows purely from the 16 newly-unlocked moves: 2 more
  `reference-syntax-unsupported` referencing the same foreign ticket (now for `shard-client.ts`/
  `turn-scheduler.ts`, which are now real move targets), and 1 `generator-preview-invalid`
  (`.vscode/launch.json`, "Reference edit preimage mismatch at 🔣️taxonomy.json") — a transient
  stale-plan artifact from `🔣️taxonomy.json` being concurrently edited mid-run by a sibling, the same
  structural risk already documented in `📓️goal-session-status.md` §12 ("ANY concurrent edit anywhere
  invalidates a plan between plan and apply"); not re-run again given the turn's time budget.

## Not re-verified this turn

Actor and kernel's round-2 numbers above are the LAST real run I have (before the binary-emoji
revert, which does not touch either scope — neither has `.bin` fixtures). I did not have turn-budget
left to re-run actor/kernel a third time after the replication fix; their reported numbers are
final and already reflect all vocabulary additions except the emoji revert (irrelevant to them).
