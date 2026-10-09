# 🏁️ Wave R11 Finisher Brief — read fully

You finish a work package that a previous agent left mid-way when the coordinator session died. Do NOT redo work that
exists; complete it, make it compile, test it, and prove it.

T = `C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️08\BIM-PLUGIN` (Git Bash `/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`).
S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## Read before coding
1. `T/r2-fleet-brief.md` (rules, gate, concurrency, per-artifact workspace) and `C:\git\semio\AGENTS.md`.
2. `T/r10-wave-brief.md` (laws + verification commands) and `T/r9-decisions.md` (binding rulings).
3. Your audit `T/r11-audit-<label>.md` — your starting point and task list — and your WP section in
   `T/r9-audit-completeness.md` §5.
4. Recipes as needed: `T/r3-golden-leaf.md`, `T/r6-exec-z-mutations.md`, `T/r7-exec-z-depth.md`,
   `T/r7-design-model-graph.md`, `T/r7-api-model-session.md`, `T/r7-exec-z-graph.md`, `T/r3-recipe-ui.md`,
   `T/r5-exec-u-tools.md`, `T/r3-recipe-io.md`, `T/r7-exec-z-codecs.md`, `T/r3-oracle-pattern.md`.
5. The previous agent's input scripts `T/r10-<label>-*` (intent; reuse them rather than rewriting).

## Hard rules (in addition to the fleet brief)
- Do NOT use the Grep tool (broken in this repo). Use Bash `grep -rn`, `find`. Quote emoji paths.
- Laws: snapshot = authored only; every derived value is a model-graph inference with real parents and honest
  `dependency()`; every leaf = sparse diff from payload + base reads, concrete inverse, sum-law test, ≥1 applied + ≥1
  rejected fixture, en+de labels, `x-semio-ui`; approved verbs only; no bim state/engine module; no legacy/compat code.
- Schema-first: JSON Schema + facets via generators `bun T/r3-f1-gen-mutation-facets.ts`, `bun T/r3-f1-gen-oracle.ts`,
  `bun T/r3-f1-gen-feature.ts`, `bun T/r3-f1-check-names.ts`; examples via `T/r4-x-examples-gen.ts` (+ `-rust.ts`).
  Generators touch shared files: run them near the end, re-run if a peer ran them since.
- Every feature: a language-agnostic `🥒️.feature` scenario AND a third-party oracle (python: shapely/ifcopenshell/lxml
  in `.venv`; TS: three) producing the same output as our implementation.
- Accessible UI (labels on every control and surface), en first then de, customizable, progress + cancellation for
  expensive work. Concise code, no comments inside definitions, docstrings start with a unique fitting emoji.
- Builds: ONLY through `"$T/🚦️gate.sh" <label> -- cargo …` (4 slots), foreground, timeout 600000. Logs to
  `T/🗑️generated/<label>/`. Agent `r11-baseline` is making `cargo test --lib` compile (missing `#[path]` test files);
  until `T/r11-exec-baseline.md` exists, verify with `cargo check … --lib` and write your code; then run the tests.
- Peers edit concurrently (other finishers + unrelated humans/agents). Re-read before each Edit; small surgical edits;
  keep files compile-atomic; never modifying git commands; if a peer's area breaks the build, wait/re-check and note it.
- NEW AGENTS.md rule: no file path may exceed 256 characters (measure from the drive root, e.g.
  `C:\git\semio\…`); choose short directory/test names; never copy version-controlled source files into the ticket folder.
- Never claim a test passes without running it. Temporary logs prefixed `[DEBUG] ` and removed before you finish.

## Done means
Every remaining task of your audit is implemented or explicitly justified as out of scope; `cargo test --lib` exact
counts with your area's tests green; wasm32-wasip2 lib check green; your oracle(s) and feature(s) run green; generators
re-run; report `T/r11-exec-<label>.md` (files touched, before → after, commands + exact results, open items).
Final message ≤ 12 lines.
