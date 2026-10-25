# emoji-d: docstring laws green in the three products

Status 2026-10-25 13:55 CET: `verify docstrings emoji-first` and `verify docstrings emoji-unique` have ZERO findings inside the closures of the dashboard, the architecture course and print. Docstrings only; no code changed.

## Law semantics (checked first)

- `emoji-unique` is per FILE (design §21.2, `runDocstringEmojiCensus`): an emoji, with U+FE0F stripped, may not open more than one docstring of one source. It is not repo-wide or per module, so the 38,737 repo-wide repeats are many small per-file repeats, not a scope bug.
- The law is correct against "unique and fitting" in AGENTS.md; no law change and no vector were needed.
- `emoji-first` accepts Extended_Pictographic / So / Sm / Regional_Indicator / keycap openers; a docstring run that restarts after a blank `///` line is a new opener.
- Both laws scan only git-tracked `*.rs/*.ts/*.tsx`, excluding `.🧬semio`, `*.d.ts` and `🤖️generated`.

## Scope decision (9822 tracked sources)

- Explicit roots: `🧰️framework/🛍️products/{🦑️repo,📓️print,🎤️presentation,📚️course,🐾️pets,🖥️server}`, `🧰️framework/🔨️modules/🖱️ui` (ui-rs / ui-react), `🎓️teaching/**`, `♻️mit-bestand/🎤️präsentation` (projektetage). Bericht sources live under print (the only other `bericht` sources are in tickets, which the law excludes).
- Plus every non-OS framework module that a crate of those products reaches through its Rust path-dependency closure (36 module roots: kernel, actor, manifest, replication, process, value, pack, io, job, schema, geometry, machine, canvas, ...).
- NOT covered on purpose: the OS product that print-rs reaches (161 crates, about 7772 repeats); it is not one of the three products.

## What was done

- Edits: only the first grapheme of a docstring opener was replaced (the law strips U+FE0F when comparing; the applier appends U+FE0F as the house style does). Round 1: @@R1@@ files, round 2 (non-OS module closure plus leftovers): @@R2@@ files, round 3 (files that peers had edited during the run): @@R3@@ files; @@FILES@@ files in total including five manual fixes.
- Every pick is chosen per item from the item's declaration and docstring text (keyword hints from `lexicon.ts`, reviewed by hand; weak or pun-only candidates were denied and re-picked manually), and verified unique within its file by the applier, which re-reads each file right before editing and refuses on drift, on a hot mtime (< 30 min) or on any remaining duplicate in the file.
- Manual fixes outside the applier (docstring only):
  - `🖱️ui/⌨️tui/📝️text/🔤️tables/🦀️.rs`: opener `🏽` (a skin-tone modifier) became `🤚️`.
  - `🖱️ui/🎯️targets/⚛️react/🧬️element-state/🟦️.ts`: two `/** … @default … */` openers got `🚦️` and `⏳️`.
  - `📚️library/🏗️build/🧪️tests/🕸️graph/🧰️support/🟦️.ts`: opener got `🧰️`.
  - `⏯️tool-run/📐️definition/🦀️.rs` and `🛂️manifest/🖼️view-model/🦀️.rs`: a docstring had been fused onto the end of a code line (`pub settings: …,    /// 🪟️ …`), which made its continuation line a bare opener without emoji; the docstring now starts on its own line (whitespace only) and the view-model opener is `🌿️` to stay unique.

## Verification

- `bun ./📜️script.ts verify docstrings emoji-unique --files-from <scope>`: `files=9822 reusing=0 repeats=0`, exit 0 (before the last rounds: reusing=8 repeats=52, all in files peers were still editing).
- `emoji-first`: the gate has no scope option and prints at most 200 of its findings, so the full `runDocstringCensus` hit list was filtered by scope: 274 repo-wide (bim, wfc, fem, cas, ... outside the products), 0 in scope, oracle agrees.
- Syntax proof of every touched source: 1133 TypeScript files through the Bun transpiler and 937 Rust files through `rustfmt --emit stdout` (+7 TS files of round 3): 0 errors. Edits are comment-only, so types cannot change.
- `cargo check` / `tsc` of the touched packages was NOT run: swap sat at 5.6 of 6.1 GB with about 50 rustc/cargo processes of peers, and a build would have risked the OOM kill documented in memory. Run `repo cargo` on the touched crates or the nx typecheck once the fleet quiets down.

## Notes for the next agent

- Peers add new docstrings continuously. Re-run the two laws before closing; the tools are in `emoji-d-tools/`: `scan.ts <list> <out>` (scoped hit lists), `items.ts` → `cands.ts` (items with candidate emojis), `auto.ts` (greedy pick from reviewed candidates, deny list inside), `apply.ts <cands> <picksDir> --dry|--go <state>` (guarded applier), `parse-check.ts`, `first-full.ts`.
- A fused `impl TextFace {    /// 🎨️ …` line remains in `🖱️ui/🔤️typography/🧩️component/🔠️text-faces-and-kerning/🦀️.rs:37`; the law does not flag it, so it was left alone.
- The repo-wide debt outside these closures is unchanged: emoji-first 274, emoji-unique about 38.7k at the start (OS product, plugins, bim, wfc, fem, ...).
