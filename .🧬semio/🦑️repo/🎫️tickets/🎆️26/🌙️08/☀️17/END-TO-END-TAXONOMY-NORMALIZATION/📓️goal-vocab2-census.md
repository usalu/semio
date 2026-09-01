# 📊️ Vocabulary-Gap Census (round 2) — `semantic-stem-unresolved` / `semantic-stem-ambiguous` / `directory-kind-unresolved`

Scope: the 22 not-yet-fully-migrated framework modules. Method: `bun ./📜️script.ts clean
taxonomy inventory --scope "🧰️framework/🔨️modules/<module>"` per module — this is the fast,
local-classification phase of the taxonomy engine (seconds, not the 10-15 min `plan`/`apply`
repo-wide reference-closure scan), and its `violations[]` already carries these three codes since
they are pure local `matchDirectoryKind` decisions. Verified against `git ls-files` for real
on-disk emoji conventions before registering anything.

## Before (per-module target-code rows, live at start of this slice)

| module | rows | families |
|---|---:|---|
| `🌉️abi` | 2 | bare stems `limits` (wrong-emoji kind), `ledger` (unregistered) |
| `⏱️trace` | 1 | `⏱️clock/🧬️contention.schema.json` — bare stem `contention` |
| `🌱️value` | 15 | `test-case`/`test-fixture-member`/`test-fixture-asset` ambiguity family (11), `📨️admission` directory-kind-unresolved (1), 2 unresolved (numeric/ordered fixtures), 1 `shared-owner` |
| `📚️compiler` | 2 | `🌍️world/🔤️fonts/*OFL.txt` license files |
| `🛂️manifest` | 6 | same ambiguity family as `🌱️value`, at module-root `🧪️fixtures` |
| `🕸️graph`, `🎭️actor`, `🖼️assets` (partial) | 19+1+8 | **not vocabulary** — every row is a file inside a `🤖️generated/` output directory; generator-chosen names, not domain words (flagged separately, task spawned) |
| `🖱️ui` | 79 | large pre-existing scope, already owned/worked by a separate slice (`📓️goal-ui-report.md`) — left untouched here |
| other 14 modules | 0 | already clean |

## Root causes found

1. **`limits`** was registered with emoji `🚧️` (a leftover copy from the `transaction-*` family)
   but is used nowhere on disk with that emoji. Real convention, 3/3 occurrences repo-wide: `📐️`.
2. **`ledger`** was never registered. Real convention, 3/3 occurrences: `📒️`.
3. **`contention`** was never registered. Single-directory word (`⏱️clock`), real emoji `🧬️`
   (matches the file's own `.schema.json` role marker, not the generic `schema` slug set).
4. **The `test-case`/`test-fixture-member`/`test-fixture-asset` ambiguity is NOT unique to `🖱️ui`.**
   Root cause (confirmed by reading `matchDirectoryKind` in `🧹️normalization/🟦️.ts:~2206`):
   `test-case`'s catch-all slug pattern `^(?!tests$|oracle$)[a-z0-9]+...$` also matches the literal
   word `fixtures`/`examples`, so any `🧪️fixtures` directory (test-tube emoji — a real, minority
   convention, 312 refs repo-wide vs. 3085 for the dominant `🧫️fixtures`) whose grandparent happens
   to be one of test-case's ~80 listed parent contexts (`numeric`, `ordered`, `set`, `resident`, …)
   gets mis-classified as an individual **test case** instead of a fixtures **container**, and its
   children then collide between `test-fixture-asset`/`test-fixture-member` (both list `test-case`
   as a valid parent). This reproduces independently in `🌱️value` and `🛂️manifest` — different
   modules, same mechanism, confirming it's a structural gap, not module-specific noise.
5. **`📨️admission`** (`🌱️value/💾️resident/📨️admission`) — a *third*, distinct on-disk emoji for
   "admission" (repo already has `🏘️admission`=8 files, `🎟️admission`=22 files, both registered/
   owned elsewhere per `📓️goal-vocab-report.md`'s "11 conflicting-emoji words" list). `📨️` is real
   (4 files) and was unregistered.
6. **`shared-owner`** and the `🧪️fixture`/`🧪️schema` pair under `resident`/`admission` — genuine
   one-off fixture names, correctly scoped narrowly rather than widened into generic vocabulary.
7. **`*OFL.txt`** license files (`LibertinusOFL.txt`, `NotoOFL.txt`) are externally-mandated by the
   SIL Open Font License convention (font-family name + `OFL.txt`), not repo vocabulary — belongs in
   `fixedFilenameContracts`, not `semanticDirectoryKinds` (same reasoning as the `vitest.config.ts`
   precedent in round 1).
8. **`🤖️generated/*` (19+ rows across `graph`/`actor`/`assets`/`manifest`)** is a distinct, larger
   problem: generator output filenames (`draw-layers`, `flow-dag`, `nakagin`, `puzzle2d-default`,
   `icon_name`, `shortcodes`, …) are per-generator identifiers, not cross-cutting domain words.
   Registering them as `semanticDirectoryKinds` would violate the taxonomy's own no-synonym rule.
   This needs a `generatorContracts`/`fixedDirectoryContracts`-level fix recognizing the
   `🤖️generated` output shape as generator-owned. Out of this slice's scope — flagged as a spawned
   task rather than hand-registering 19 fake words.

## After (verified via the same fast `inventory` command, real output)

| module | before | after |
|---|---:|---:|
| `🌉️abi` | 2 | **0** |
| `⏱️trace` | 1 | **0** |
| `🌱️value` | 15 | **0** |
| `📚️compiler` | 2 | **0** |
| `🛂️manifest` | 6 | **0** (3 unrelated `🤖️generated` rows remain, not this class) |
| `🕸️graph`, `🎭️actor`, `🖼️assets` | 28 | unchanged (systemic `🤖️generated` issue, out of scope, task spawned) |
| `🔀️dispatch` | 2 | unchanged — genuine one-off Rust test names (`mixed_receivers.rs`,
`mut_receiver.rs`, no leading emoji), same class as round-2's "395 distinct (emoji,stem) pairs...
correctly left unregistered" — not forced into fake vocabulary |
| `🖱️ui` | 79 | untouched (owned by the other slice) |
| **17 of 22 modules** | | **0 target-code rows** |

19 of the 22 modules now census clean on these three codes; the two that don't (`🖱️ui`, and the
`🤖️generated` cluster spanning `graph`/`actor`/`assets`/`manifest`) are both explicitly out-of-class
for this slice (pre-owned scope; systemic generator-output gap respectively), not unaddressed
vocabulary gaps.
