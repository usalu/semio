# AGENTS.md Compliance Audit — Quiz Product and Teaching Proctor

Scope: everything created/changed by this ticket — `🧰️framework/🛍️products/❓️quiz/**`, `🎓️teaching/**`, plus the
shared-file diffs (`package.json`, `Cargo.toml`, `🧰️framework/🛍️products/🔣️.json`, `.gitignore`, `.vscode/launch.json`,
`.vscode/🧩️launch.seed.jsonc`, and the incidental `🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs` fix).
172 new/changed files audited against every concrete rule in `AGENTS.md`.

Verdict: the implementation is broadly compliant and unusually disciplined (i18n, accessibility, CQRS storage
shape, progress/cancellation are all genuinely implemented, not just described). Two concrete rule violations found
(launch.json registration) and two lower-severity test-coverage gaps. No comments-inside-definitions, no stray
scripts, no leaked runtime dependencies, no CRUD/legacy code found.

## Findings

### 1. `launch.json` missing three executable commands — **Medium**

AGENTS.md: *"All devs are using `launch.json` and never use the cli... You MUST register all executable commands
there."* Three `nx` targets introduced by this ticket have no row in `.vscode/launch.json` (nor in
`.vscode/🧩️launch.seed.jsonc`, which is otherwise kept in lockstep with it):

- `@semio-tech/quiz-react:typecheck` — defined at
  `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json:57`. Sibling `typecheck`
  targets elsewhere in the repo (`@semio-tech/ui-react:typecheck`, `@semio-tech/framework:typecheck`,
  `@semio-tech/framework-os:typecheck`, `os-hub-ts:typecheck`, `@semio-tech/repo-lib:typecheck`) are all registered in
  `.vscode/launch.json` — this one alone is not.
- `@teaching/proctor:check` — defined at `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json:74` (`proctor check
  <catalog>`, the catalog-validation operator command design.md §12 names explicitly). Not in `launch.json`, and no
  `check:teaching:proctor` root `package.json` script either — there is no zero-touch way to run it.
- `@teaching/proctor:rebuild` — defined at `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json:83` (drop and
  refold every projection with progress + Ctrl+C, per design.md §12). Same gap: no `launch.json` row, no root
  `package.json` script.

Everything else new is correctly wired: `@teaching/architecture-quiz` (dev/build/test/check/docker-image-build/
docker-image-check), `@teaching/proctor` (dev/build/test), `@semio-tech/quiz`/`quiz-react`/`quiz-rs` (test) all have
rows at `.vscode/launch.json:1965-2054` and `:4463-4514` (mirrored in the seed file). `test-quick`/`test-long`/
`test-exhaustive` and the bare `build` target of `quiz-rs` are *not* registered, but that matches the existing
repo-wide convention (e.g. `@semio-tech/framework-server-rs` also only registers `test`, and
`@semio-tech/presentation` registers none of its granular test variants) — not a new violation.

### 2. Two features validated in only one language, without the shared fixture/oracle scaffold — **Low**

AGENTS.md: *"You MUST create at least one language-agnostic test for every feature... You MUST create the same
output of a test with at least one third-party library."* Design.md's own convention is
`🧪️tests/<case>/{🥒️.feature,🟦️.ts}` + Rust twin under `📦️packages/🦀️rust`, used for 10 of the 24 quiz-core test
cases (answer-validation, sheet-assembly, seeded-randomness, badge-rules, leaderboard, sorting-concordance,
matching-concordance, profile-similarity, schema-conformance, learner-lifecycle — each with `.py`/`.ts`/`.feature`/
`.rs` and a real third-party oracle: scipy/jStat, numpy, ajv). Two cases fall short of that pattern:

- `🧰️framework/🛍️products/❓️quiz/🧪️tests/⚖️partial-credit-scoring/🟦️.ts` — does validate the classification
  distance/credit formula against `mathjs`'s `distance` (line 1, used at lines 278/296), i.e. it *does* have a
  third-party oracle, but it is TS-only: no `.feature`, no Rust twin, no shared `🧫️fixtures/` vectors. This overlaps
  with `🕸️profile-similarity` (which does have the full four-file treatment) — worth confirming the two aren't
  divergent implementations of the same formula.
- `🧰️framework/🛍️products/❓️quiz/🧪️tests/👁️read-views/🟦️.ts` — exercises `catalogView`/`learnerView`/`runView`
  (the Rust equivalents `catalog_view`/`learner_view`/`run_view` are covered instead by 7 inline `#[cfg(test)]`
  cases in `🔨️modules/👁️views/🦀️.rs:117-277`, not by a shared fixture). No third-party library plausibly computes
  the same bespoke view JSON, so full literal compliance may not be feasible here — but unlike `leaderboard` (which
  got the full `🥒️.feature` + fixture treatment for the same module), these three sibling view functions did not.

Both features do have real tests in both languages; the gap is the shared-fixture/oracle scaffolding the rest of the
suite is held to, not an absence of testing.

## Rules checked with no violations found

- **Comments inside definitions**: no in-body `//` comments, no block comments, no commented-out code anywhere in
  the `.rs`/`.ts`/`.tsx` scope (`^\s+//[^/!]`, trailing `// ` after code, and `/* */` all came back empty). Every
  docstring sampled (284 lines checked) starts with a fitting emoji, matches the rest of the repo's
  `//! 🔤️ …` / `/** 🔤️ … */` / `/// 🔤️ …` convention, and the `//#region 🔖️Name` fold markers match the existing
  framework-server convention — not stray commentary.
- **`[DEBUG]` / `console.log` / `println!` / `eprintln!`**: every hit (`🎓️teaching/🛂️proctor/🔨️modules/⌨️cli/🦀️.rs`,
  `🔨️modules/🧩️instance/🦀️.rs`, `🏗️bootstrap/🦀️.rs`, `🚀️deploy/🟦️.ts`) is permanent, purposeful `[INFO]`/`[ERROR]`
  operator output for the `proctor` CLI and the deploy script — no leftover `[DEBUG]` tags, no stray debugging
  output.
- **Scripts outside `📜️script.ts`**: none. The only non-`.rs`/`.ts`/`.tsx` scripts are `.py` oracle files, all inside
  `🧪️tests/<case>/`, consistent with the third-party-oracle test convention.
- **`project.json`/`package.json` wiring**: all 5 new `📋️project.json` files route every target through
  `bun ./📜️script.ts <cmd>`; new library `package.json` files correctly omit a `scripts` block (matching the
  `presentation`/`presentation-react` precedent), and `🎓️teaching/🏛️architecture/❓️quiz/…/package.json` scripts all
  call `bun nx run …`; the root `package.json` additions all call `bun nx run …` too.
- **Runtime dependencies on external libraries**: `@semio-tech/quiz`'s `package.json` has no `dependencies` at all
  (ajv/jstat/mathjs are `devDependencies`, oracle-only). `@semio-tech/quiz-react` depends only on internal workspace
  packages plus `react`/`react-dom` (established repo-wide precedent, same as `presentation-react`). `d3-scale` is
  correctly a `devDependency` — confirmed it is imported only by the test oracle
  (`🧪️tests/🕷️radar-geometry/🟦️.tsx:9`), never by the shipped radar module
  (`🎯️targets/⚛️react/🔨️modules/🕸️radar/🟦️.tsx` imports only `react` and sibling modules). On the Rust side,
  `rusqlite` (bundled) and `axum` in `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/Cargo.toml` are both precedented
  (hub already uses both the same way) and encapsulated: every `rusqlite::*` type stays behind private `fn`s in
  `🔨️modules/🗄️storage/🦀️.rs` (no `pub fn` leaks a `rusqlite` type), satisfying the "behind an interface, never
  exported" clause.
- **i18n, no default language**: `🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts` defines a fully parallel `en`/`de`
  bundle (150+ keys, type-enforced equal shape via `typeof QUIZ_BUNDLE_EN`), no hardcoded literal English strings
  found in any JSX (`aria-label="…"`, `title="…"`, `placeholder="…"`, or plain JSX text checked, all empty). Locale
  resolution follows design.md exactly: explicit persisted choice → browser language list → English tie-break, no
  built-in default. `🧪️tests/🗣️translation-completeness/🟦️.tsx` mechanically enforces same-keys, non-empty labels,
  matching placeholders, and that the client uses exactly the registered keys (nothing missing, nothing unused).
- **Accessibility**: every interactive task module (`↕️sorting`, `🃏️matching`, `🗂️classification`, `🪪️identity`,
  `🏠️home`, `🏆️leaderboard`, `🏁️results`, `▶️run`, `👋️introduction`, `🧩️task`) has `aria-*`/`role=`/`onKeyDown`/
  `tabIndex` usage. The radar chart (`🔨️modules/🕸️radar/🟦️.tsx`) is an owned SVG with `role="img"` +
  `aria-labelledby` + `<title>`, and always ships a `<details>`/`<table>` text alternative
  (lines 82-123) — matches design.md §10 verbatim.
- **Customization / multi-device**: `🎨️.css` drives theme via `data-theme` custom properties (light/dark, lines
  25-36) and text size via `--quiz-text-scale` (line 6, applied at line 18), with `@media (max-width: 60em)` and
  `@media (max-width: 40em)` breakpoints (lines 854, 869) for desktop→tablet→mobile.
- **CQRS / event sourcing**: `🎓️teaching/🛂️proctor/🔨️modules/🗄️storage/🦀️.rs` only ever `INSERT`s into the event
  log (`proctor_event`, append-only); the `INSERT OR REPLACE`/`DELETE` statements found are all against derived
  state — snapshots, projections, checkpoints, leases, sessions, an immutable content-addressed blob store — i.e.
  exactly the four storage roles design.md §9 names, upserted as fold targets, not a mutable source of truth. No
  direct field-level `UPDATE` anywhere. `decide_learner`/`evolve_learner` are pure per design.md §8.
- **Progress + cancellation**: the run player's submission uses a real `AbortController`
  (`🎯️targets/⚛️react/🔨️modules/▶️run/🟦️.tsx:39,53,86,104,207-208`), and the proctor's `rebuild` CLI reports
  10%-step progress and honours Ctrl+C/SIGTERM/SIGBREAK between batches
  (`🎓️teaching/🛂️proctor/🔨️modules/⌨️cli/🦀️.rs:142-189`).
- **Legacy / deprecation / migration / compat**: zero matches in the ticket's own code; the only two `grep` hits
  (`🔮️oracles/🔣️.json:13`, `🧪️tests/🌀️mt19937-generator/🟦️.ts:4`) both refer to **numpy's own** `_legacy_seeding`
  API name, not to anything in this codebase.
- **Concise / duplicated code**: no evidence of copy-pasted production logic; the only repetition is a one-line
  `const T = (en) => ({ en, de: en })` text-builder helper reimplemented per self-contained test file, which is
  trivial and consistent with each `🧪️tests/<case>/` directory being an independent unit — not a rule violation.

## Incidental shared-file change

`🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs` gained a `rehydrate` function (with matching unit
test in the same PR) so a freshly placed activation replays its snapshot + event tail before deciding, instead of
starting from empty state after a restart. This is a real, load-bearing framework correctness fix the proctor's
restart/rebuild story depends on — well-tested, in scope, not a stray edit.
