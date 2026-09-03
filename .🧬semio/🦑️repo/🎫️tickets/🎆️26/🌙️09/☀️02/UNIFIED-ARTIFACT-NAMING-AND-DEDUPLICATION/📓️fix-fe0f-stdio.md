🩹 Fix FE0F on stdio's `txt`/`xml` artifact kind names

## Scope
Two names from `taxonomy.json`'s `semanticDirectoryMemberKinds["members-of-artifacts"].memberNames`
carry U+FE0F (variation selector-16) that some `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/…` disk paths and
many textual references lacked: `📄️txt` (U+1F4C4 U+FE0F "txt") and `📰️xml` (U+1F4F0 U+FE0F "xml").

## Finding: disk was already fully renamed
Before touching anything, I verified codepoints directly (never trusted terminal rendering — matched
by Python/ripgrep on exact codepoints). `os.walk` over the whole repo found **zero** directory or file
names anywhere still missing the selector for either `txt` or `xml` — including inside every other
plugin's own local artifact mirror (writer, mathematical, fem, block, space, gis, …), not just stdio.
The two stdio directories themselves (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️txt` and `.../📰️xml`) already
carry U+FE0F, confirmed by `git status` showing them as pending `R` (rename) entries from earlier work
on this same ticket (`rename_fe0f_dirs.py`, `sweep_missing_fe0f.py` already present in this folder).
**No `mv` was needed** — the rename half of the task was already done by a prior wave.

What remained was stale **textual references** — path strings that still spelled the old (no-selector)
name, now pointing at directories that had already moved. I also caught the big shared stdio aggregator
file (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs`) being actively fixed by a concurrent session
mid-scan — its 54 stale `xml` references were live (54 bad) at my first baseline snapshot and gone (0
bad, 54 good) a few minutes later, before I ever edited it. I re-baselined after noticing this so my
counts below reflect the state I actually acted on, not the stale first snapshot.

## Scoping decision
Per the ticket's concurrency instructions ("stay strictly inside the txt and xml artifacts and the
shared stdio wiring lines that reference them"), I fixed references only in:
- everything under `✏️s/🔌️plugins/🗄️stdio/` (the plugin itself, its own `📇️registry`, `🧪️oracle`, and
  every artifact kind's own `#[path]`/JSON/`.grammar.semio`/`.protocol.semio`/`.feature`/`.ts` files that
  mention `txt` or `xml` as an import/export "any format" mirror or the two kinds' own definitions)
- `✏️s/🔌️plugins/🔒️policy-allowlist.json` (references literal stdio paths)
- `🧰️framework/🔨️modules/🖼️assets/📃️list/📋️mimes/📊️.csv` (the `Kind` column literally tags rows
  `stdio.txt` / `stdio.xml`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️.rs`
  (`STDIO_CONFORMANCE_GRADUATED`, explicitly stdio-scoped)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs` (a docstring naming `📄txt` as the example app)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
  (a fixture literally pointing at the stdio path)

I deliberately left **untouched**:
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — explicitly forbidden by the
  ticket; its one stale mention is inside a long historical comment string (`📄txt->txt` as a slug-algorithm
  example), not a live reference.
- Every *other* plugin's own `#[path]` aggregator (`✒️writer`, `➗️mathematical`, `🌀️procedural`, `🌊️flow`,
  `🌍️gis`, `🌿️vcs`, `🎞️animate`, `🎥️shooting`, `🎬️sequence`, `🏗️fem`, `🏛️architect`, `🏭️process`,
  `💡️reasoning`, `📖️playbook`, `📜️imperative`, `📸️remodel`, `🔋️energy`, `🔱️trinity`, `🕸️dag`, `🧱️block`,
  `🪐️space`, `🪵️sourcing`) — these reference their own private `artifacts/📄txt` "any format" mirror, not
  stdio's directory, and are out of this ticket's stated scope (61 stale `txt` occurrences left there;
  0 stale `xml` occurrences existed outside stdio+wiring). A follow-up ticket would need to sweep those.
- Historical ticket-folder logs/scratch files under other `.🧬semio/…/🎫️tickets/…` folders (hundreds of
  old `*.txt` cargo-test transcripts happen to contain the stale string) — archival, not live code.
- Build/dep caches (`.nx`, `target*`, `.fingerprint`, `storybook-static`) — regenerated, not source.

## Reference counts (measured, not assumed)
Using `rg -P` matching the exact codepoints `\x{1F4C4}(?!\x{FE0F})txt` and `\x{1F4F0}(?!\x{FE0F})xml`,
re-baselined after the concurrent session's partial fix landed:
- **txt**: 112 stale references repo-wide → 51 in scope (fixed) / 61 out of scope (other plugins, left alone)
- **xml**: 89 stale references repo-wide → all 89 in scope (fixed); 0 existed outside stdio+wiring

**91 files changed** in total (listed in full in `🗑️generated/` scratch output during the session; the
change was a pure content substitution, one line-diff per stale occurrence — 245 insertions / 244
deletions per `git diff --stat`, the 1-line asymmetry being an unrelated pre-existing blank-line diff).

## Codepoint assertions (post-fix, repo-wide, excluding node_modules/target/dist/.git)
```
rg -P '\x{1F4C4}(?!\x{FE0F})txt' → 0 hits inside scope (61 remain, all in explicitly out-of-scope other-plugin files)
rg -P '\x{1F4F0}(?!\x{FE0F})xml' → 0 hits anywhere in scope, 0 anywhere out of scope
rg -P '\x{1F4C4}\x{FE0F}\x{FE0F}txt|\x{1F4F0}\x{FE0F}\x{FE0F}xml' → 0 hits repo-wide (no double-insertion)
```
No directory or file **name** anywhere in the repo is missing the selector (verified before starting) —
only file *content* needed fixing.

## Cargo verification
`cd /Users/ueli/Documents/semio && RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --target wasm32-wasip2 --message-format short`
(crate name confirmed from `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml`).

**Result: exit code 0 (clean).** `semio-s-plugin-stdio` compiles for `wasm32-wasip2` with no errors.
The tail of `--message-format short` output is 184 pre-existing warnings from the dependency crate
`semio-framework-plugin` (dead code, unused imports/variables, visibility lints, redundant `.clone()`)
— none of them reference `txt` or `xml`, none are new (they're in shared kernel/reactor code untouched
by this fix), and none relate to the concurrent zip work either. No error mentions `txt` or `xml`
anywhere in the output. The stdio crate's own txt/xml wiring compiles.

## Concurrency note
Another session was actively restructuring the stdio `zip` artifact throughout this work (per the
ticket's warning). I stayed out of `zip` entirely. I also observed that session (or a related one)
had already fixed the big shared aggregator file's `xml`/`txt` lines between my first and second
baseline scan — no conflict, just overlapping progress on the same ticket goal.
