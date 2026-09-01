# Census — `🎭️actor` / `📡️replication` / `🎠️kernel`

Baseline commit: `bb06c41f73f0122fbed315b7487428b976f99921` (matches `📓️goal-session-status.md`).

## Round 1 (before any change)

```
actor        moves=55 edits=216 regenerations=4 unresolved=96
replication  moves=53 edits=69  regenerations=2 unresolved=41
kernel       moves=43 edits=132 regenerations=2 unresolved=29
```

(Prompt's stated baseline was 97/42/33 — small deltas are other slices' concurrent progress on
shared files, not measurement noise; not investigated further, out of this slice's scope.)

## Family breakdown, round 1

| code | actor | replication | kernel |
|---|---:|---:|---:|
| reference-syntax-unsupported | 69 | 8 | 22 |
| semantic-stem-ambiguous | 11 | 4 | 2 |
| directory-kind-unresolved | 6 | 2 | 0 |
| package-implementation-destination-unresolved | 5 | 0 | 0 |
| semantic-stem-unresolved | 5 | 27 | 5 |

### `reference-syntax-unsupported` (99 rows) — mostly NOT this slice's to fix

- 76 of the 99 (actor 59, kernel 17) are prose in **another ticket's** reports:
  `.🧬semio/…/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/…`. That ticket has a `Cargo.toml` sitting
  **directly at its own ticket root** (plus two more nested repro crates), which makes
  `historicalEvidenceBoundaryOwns` (🧹️normalization/🟦️.ts:3369) treat the WHOLE ticket directory as
  package-owned — including its own `📓️` reports and `📥️worker-bootstrap/📓️*.md` children — so the
  `ticket-report`/`ticket-workspace` exemption never applies to them.
  **This looked like a bug and is not one.** The neighbouring test
  (`🧪️tests/🧪️historical-document-evidence/🟦️.ts`) has a case for exactly this shape —
  `"a loose C source sibling of a real ticket-root Cargo.toml stays live — the whole directory is
  package-owned"` — and the guard's own docstring explicitly rejects narrowing the boundary check by
  file extension: *"narrowing it by extension would silently stop protecting exactly this kind of
  file."* I traced the code path (guard-chain lesson from §13) and confirmed this is deliberate,
  tested behaviour, not a proxy bug — disproving my own initial hypothesis before touching anything,
  per the ticket's own warning against forced fixes. **Not fixed. Not in scope**: the real remedy is
  for that ticket's owner to move its root `Cargo.toml` into a subdirectory, which is their ticket,
  not this slice's.
- Remaining rows (actor 10, replication 8, kernel 5) are `.dependency-cruiser.cjs` / `📜️script.ts` /
  package-root `🟦️component.ts` etc. referencing the pre-move `🟦️glue.ts`/`🦀️component.rs` paths that
  the `moves` array itself will rewrite once applied — expected, self-resolving on `apply`, not
  independent defects.

### `semantic-stem-ambiguous` "test-case, test-fixture-member" (17 rows, all 3 scopes) — **the shared family**

Root cause read directly in `matchDirectoryKind` (🧹️normalization/🟦️.ts:2209): a `🧪️fixture.json` /
`🧪️schema.json` sitting beside a component always resolves both "test-case" and "test-fixture-member"
by emoji+regex, and disambiguation depends entirely on the immediate parent's kind id being present in
`test-case.parentKindIds`. The list (71 entries) covers `content`, `credit`, `fault`, `inbox`,
`lifetime`, `response`, `return`, `causal-add`, `local-interaction`, `mutation-leaf-contract`, … but
not `input`, `root`, `transport`, or any of the five actor domain-concept directories below — so every
new domain-concept folder these three modules introduce reproduces the same ambiguity. One shared fix.

### `directory-kind-unresolved` (8 rows) — five brand-new, unregistered domain concepts (actor) + two (replication)

Real, on-disk, implementation-neutral concepts with **zero existing registration** (checked exact id
+ regex candidates against all 213 `semanticDirectoryKinds` before adding anything):

| concept | emoji (as used on disk) | scope |
|---|---|---|
| metadata | 📋️ | actor |
| patch | 🩹️ | actor |
| admission | 🏘️ | actor |
| instance | 🚪️ | actor |
| output | 📥️ | actor |
| root | 🌳️ | replication |
| transport | 📡️ | replication |

None collides with an existing kind of the same emoji once given its own exact id (`metadata`
literal-matches ahead of the pre-existing regex-based `asset-table-subject`, which also uses 📋️).

### `semantic-stem-unresolved` (37 rows)

- **actor** (5): `📬️mailbox.ts`, `🖼️wire-turn.ts`, `🧵️shard-client.ts`, `🧵️shard-runtime.ts`,
  `🧵️turn-scheduler.ts` — named TS files in `📦️packages/🟦️typescript` with zero registered
  directory-kind twin. Each also drove a matching `package-implementation-destination-unresolved`
  row (`packageImplementationDestination` at 🧹️normalization/🟦️.ts:3277 reuses the same
  `canonicalFile` resolution) — one fix clears both codes, 10 rows total.
- **replication** (27): 20 are `🧫️fixtures/wire/📦️*.bin` wire-protocol fixtures. Root-caused to a
  **wrong registered emoji**: `fileKinds.binary.emoji` was `💾️`, but **all 40** `.bin` files
  repo-wide (0 exceptions) use `📦️` — the exact "check what the tree actually uses" trap called out
  in the brief. Fixing the registration alone doesn't resolve the fold (no `asset-*-subject` kind
  used 📦️ either), so a matching `asset-binary-subject` kind was added, mirroring
  `asset-video-subject`/`asset-table-subject`. The other 7: `📃️query.json(.schema.json)`,
  `🔐️topology-authority.json(.schema.json)` (both single-occurrence, unregistered emoji, no
  collision), and `🛂️schema.json` ×3 occurrences in this scope (`mutation-leaf-contract`,
  `mutation-leaf-source-contract`, `causal-add` — all three already valid `test-case` parents). `🛂️`
  is registered **nowhere** in the schema despite 9 real occurrences repo-wide (library, os/spr, and
  here) always in the same role (a fixture-case validation schema) — genuine cross-module vocabulary
  gap, not a one-off.
- **kernel** (5): `🧬️wire.json` — content is a wire-format/record-frame descriptor, i.e. actually a
  *schema*; extended the existing `schema` kind's alternation (`^(schema|mutations|contract)$` →
  `+|wire`) rather than minting a synonym (single occurrence repo-wide, checked). `📇️descriptor-load`
  and `🚪️turn-patch-owner` (two files each, `.json`+`.schema.json`): new domain concepts, zero
  collision on their emoji.
- **actor** (1 more): `🚪️lifetime/🧪️fault.fixture.json`. Not a vocabulary gap — a **missing
  extension-chain registration**. `.schema.json` already owns the dedicated `json-schema` fileKind;
  `.fixture.json` (used here and in one other, unrelated, open ticket) had none, so the whole
  `"fault.fixture"` string became the semantic stem (embedded dot, unmatchable). Added `json-fixture`
  mirroring `json-schema` exactly (`fileKindResolutionRules` entry included — `loadCatalogTaxonomy()`
  enforces the schema owns every extension chain exactly once, caught this immediately).

## Fixes applied (🔣️taxonomy.json, `🧹️normalization/🟦️.ts` untouched)

1. 17 new `semanticDirectoryKinds` entries: `metadata`, `patch`, `admission`, `instance`, `output`,
   `mailbox`, `turn-scheduler`, `shard-runtime`, `shard-client`, `wire-turn`, `root`, `transport`,
   `test-fixture-schema-authority` (🛂️), `asset-binary-subject` (📦️, parented to `assets`), `query`,
   `topology-authority`, `descriptor-load`, `turn-patch-owner`.
2. `schema` kind slugPattern extended: `^(schema|mutations|contract)$` → `^(schema|mutations|contract|wire)$`.
3. `test-case.parentKindIds` extended with `metadata, patch, admission, instance, output, root,
   transport, input` (71 → 79 entries).
4. `fileKinds.binary.emoji` corrected `💾️` → `📦️` (0/40 real files used 💾️).
5. New `fileKinds.json-fixture` (🔣️, `.fixture.json`, role `schema`) + matching
   `fileKindResolutionRules.physical-json-fixture-json` entry.

Every addition was checked against the full existing registry first (`allowEmojiOnly`, exact-id vs.
regex collision, `parentKindIds` interaction) before writing — no synonym minted, no existing kind
weakened. `discovery.validateTaxonomy()` returns 0 errors after the change.

## Predicted effect (verify against round-2 plan output in `📓️goal-ark-report.md`)

- actor: all 27 non-refsyn rows should clear (6 directory-kind-unresolved, 11 semantic-stem-ambiguous,
  5 package-implementation-destination-unresolved, 5 semantic-stem-unresolved) → unresolved 96 → ~69
  (only the foreign-ticket prose + self-resolving pre-move references remain).
- replication: all 33 non-refsyn rows should clear → unresolved 41 → ~8.
- kernel: all 7 non-refsyn rows should clear → unresolved 29 → ~22 (17 foreign-ticket + 5 self-resolving).

## Correction found during round-2 verification

Round-2 plans (real output) showed actor 55→71 moves / 96→106 unresolved, replication 53→64 / 41→29
(with a **new** 20-row `semantic-stem-ambiguous: asset-binary-subject, asset-subject` family — a
regression I introduced), kernel 43→50 / 29→23.

Root-caused the replication regression by reading `resolveFileKind` (🧹️normalization/🟦️.ts:2243-2301):
when a file's on-disk leading emoji equals its **registered fileKind's own emoji**, that emoji is
stripped as mere decoration before semantic-stem folding — the stem then re-enters
`matchDirectoryKind` with **no emoji**, which matches *any* permissive-slug `assets`-parented kind
regardless of emoji (only `inferWithoutEmoji: false` entries opt out, and neither `asset-subject` nor
my new `asset-binary-subject` set it). Checking the established precedent (`video` fileKind emoji
🎬️ vs. `asset-video-subject` folder emoji 🎥️ — deliberately different) confirmed this is the actual,
working convention: **file-kind-leaf emoji and subject-folder emoji are intentionally distinct**, so
the folder-fold keeps the leaf's original emoji as disambiguating evidence. My initial "fix the wrong
📦️/💾️ registration" was itself the bug — reverted `fileKinds.binary.emoji` back to `💾️` (the file-kind
leaf emoji), keeping `asset-binary-subject` at `📦️` (the folder emoji, matching the real on-disk
convention). Re-verified: replication → moves=64, **unresolved=8** (was 41). See `📓️goal-ark-report.md`
for all three scopes' final numbers.

**Generalizes beyond these three scopes**: any future `asset-*-subject` addition must keep its
`fileKinds` emoji different from its paired `semanticDirectoryKinds` subject-folder emoji, or it will
silently collide with `asset-subject` (and any other default-`inferWithoutEmoji` sibling) the moment a
no-emoji stem reaches `matchDirectoryKind`. Worth a repo-wide grep for any other place a fileKind's own
emoji was set equal to its subject-folder counterpart.
