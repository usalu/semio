# 📓️ Test case directory emoji identity (Opus executor)

Job: rename every `🧰️framework/🛍️products/🦑️repo/🔨️modules/*/🧪️tests/<case>` directory that lacks a
leading emoji grapheme, update every reference, and verify with `discover` / `contract` / `parity`.

## 1. Finding: the rename set was already empty

`📋️plan.md` §5 decided that a case directory is one leading emoji grapheme plus a kebab slug, and
`📓️opus-statute-hygiene.md` §1 landed `testCaseSlugPattern` in
`🔨️modules/📚️library/🔣️taxonomy.json`:

```
^(?:[0-9#*]️?⃣|[\u{1F1E6}-\u{1F1FF}]{2}|\p{Extended_Pictographic}\p{Emoji_Modifier}?[︎️]?(?:‍\p{Extended_Pictographic}\p{Emoji_Modifier}?[︎️]?)*)[a-z0-9]+(?:-[a-z0-9]+)*$
```

Every one of the **163** case directories under the repo product was matched against that exact
pattern (compiled from the taxonomy file, not retyped). **0 breaches.** The cases named in the task
brief as examples already carry an emoji, chosen by whoever authored them rather than by the brief's
suggestions — both are valid under the pattern:

| brief's example | actual on disk |
| --- | --- |
| `🔗️graphql/🧪️tests/document-parsing` | `🔗️graphql/🧪️tests/📃️document-parsing` |
| `🗣️languages/🧪️tests/section-parsing` | `🗣️languages/🧪️tests/📑️section-parsing` |
| `📡️events/🧪️tests/payload-encoding` | `📡️events/🧪️tests/📦️payload-encoding` |
| `📐️model/🧪️tests/slug-normalisation` | `📐️model/🧪️tests/🔤️slug-normalisation` |

`git ls-tree HEAD` has no `🦑️repo/🔨️modules/*/🧪️tests` paths at all — the whole module tree is new in
this ticket's working set, so the directories were *created* emoji-prefixed rather than renamed
later. **No `mv` was performed and no file was modified by this executor.**

The one name my first shell heuristic flagged, `📚️library/🧪️tests/1️⃣g1-contract`, is a false
positive: it leads with the keycap sequence `1️⃣` (U+0031 U+FE0F U+20E3), which the pattern's first
alternative accepts.

## 2. References audited (all already consistent)

* `.vscode/🧩️launch.seed.jsonc` — 17 `--case` arguments, every one emoji-prefixed, every one
  resolving to an existing directory. Same for `.vscode/launch.json`. No stale slug, so **no
  regeneration was needed** and the seed was left untouched (concurrent agents are editing it).
* `📋️project.json` targets, `🔮️oracle/🔣️.json`, `🧫️fixtures` adapter paths, READMEs — a repo-wide
  grep for the bare (non-emoji) forms of the four example slugs returns only *capability* names
  (`graphql-document-parsing`, `repo-event-payload-encoding`, `repo-section-parsing`) and one schema
  title, none of which are directory references.

## 3. Verification

### `discover`

```
[discover] 414 test case(s)
```

98 of the 163 repo-product case directories appear. The other 65 are correctly absent: they carry no
`🥒️.feature` (they are vitest-only library cases — 68 dirs under `📚️library/🧪️tests` hold zero
`🥒️.feature` files — plus `🧪️test/🧪️tests/🧭️contribution-directory-ownership`, which is a
schema/JSON case). Every gherkin-backed case is listed.

### `contract`

`testing/taxonomy case-slug` breaches under the repo product: **0**.

The full breach set still carries 90 `case-slug` hits, all of them in *other* products and outside
this task's scope — e.g. `🧰️framework/🛍️products/📓️print/🧪️tests/3d-projection`. Left alone.

### `parity fundamental --owner …` (`SEMIO_TEST_BUDGET_MS=600000`)

```
🧾️yaml          cases=2 executed=5  passed=5  failed=0 errored=0 parity=4/4
🪪️identity      cases=2 executed=5  passed=5  failed=0 errored=0 parity=4/4
🔎️search        cases=1 executed=3  passed=3  failed=0 errored=0 parity=3/3
📡️events        cases=4 executed=27 passed=27 failed=0 errored=0 parity=27/27
📐️model         cases=3 executed=13 passed=13 failed=0 errored=0 parity=8/8
🗣️languages     cases=5 executed=26 passed=26 failed=0 errored=0 parity=16/16
🔗️graphql       cases=8 executed=21 passed=21 failed=0 errored=0 parity=18/18
🚚️move          cases=5 executed=28 passed=28 failed=0 errored=0 parity=12/12
🗂️codebase      cases=4 executed=16 passed=16 failed=0 errored=0 parity=8/8
🏠️workspace     cases=5 executed=17 passed=17 failed=0 errored=0 parity=13/13
📊️metrics       cases=4 executed=23 passed=23 failed=0 errored=0 parity=19/19
🌳️tree          cases=5 executed=12 passed=12 failed=0 errored=0 parity=0/0
🎛️dashboard     cases=1 executed=1  passed=1  failed=0 errored=0 parity=0/0
🎫️tickets       cases=5 executed=57 passed=57 failed=0 errored=0 parity=42/42
🎯️goals         cases=4 executed=9  passed=9  failed=0 errored=0 parity=6/6
🏃️test-runner   cases=5 executed=27 passed=27 failed=0 errored=0 parity=18/18
📜️statutes      cases=5 executed=18 passed=18 failed=0 errored=0 parity=9/9
📝️todos         cases=4 executed=11 passed=11 failed=0 errored=0 parity=7/7
🔌️mcp           cases=4 executed=16 passed=16 failed=0 errored=0 parity=14/14
🧑️contributors  cases=3 executed=7  passed=7  failed=0 errored=0 parity=5/5
🧩️providers     cases=4 executed=29 passed=29 failed=0 errored=0 parity=22/22 not-exercised=1
🪝️hooks         cases=5 executed=54 passed=54 failed=0 errored=0 parity=45/45
🧪️test          cases=1 executed=8  passed=8  failed=0 errored=0 parity=12/12
```

23 of 24 owners green, `failed=0 errored=0` throughout.

## 4. Exogenous blockers observed (NOT owned here, NOT touched)

1. **`⌨️cli` cannot be exercised.** The Rust cli crate does not compile mid-run:
   `error[E0599]: no method named `write` found for struct `FsGoalStore``, so
   `semio-framework-repo-cli` fails to build. A concurrent agent is refactoring the goal store; the
   brief explicitly reserves crates and the cli to them.
2. **`🧪️test` dotnet host is missing a source file.**
   `CSC : error CS2001: Source file '…/🧪️test/📦️packages/🔷️dotnet/🔷️host.cs' could not be found`.
   The case still passes 8/8 because the dotnet subject is one of several; the missing file belongs
   to a concurrent agent's in-flight dotnet host work.
3. **`parity … --owner 🦑️repo` (whole product in one invocation) crashes the runner** on Windows
   before finishing:
   `EFAULT: bad address in system call argument, rm '…\test-framework-products-repo-modules-tree-693abd-goal-statute-territory-trees-subject-go\📤️results.jsonl'`
   thrown from `rmSync` at `🔨️modules/🧪️test/📜️script.ts:739`. This is a Bun-on-Windows emoji-path
   defect in the harness, unrelated to case naming — the same owner passes 12/12 when run on its
   own. It belongs with the `📓️opus-windows-fixes.md` work, so `📜️script.ts` was left untouched and
   per-owner invocations were used instead.

Also noted while running: an earlier attempt failed on 29 compile errors in
`📐️model/📦️packages/🦀️rust/🦀️.rs` (unresolved `flat`, `humanize_seconds`). A concurrent agent fixed
that crate during this session; the `📐️model` row above is from the successful later run.

## 5. Files changed

None. The renames this ticket called for were already in place, every reference already resolved,
and the remaining failures are owned by concurrent executors. No `🗑️generated` folder was created —
scratch output went to the session scratchpad and is gone.
