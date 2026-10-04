# Test, fixture, oracle and parity-harness conventions — for the quiz product + proctor server

Explored read-only from `C:\git\semio`. Goal: document exactly how tests/fixtures/oracles/parity work
today so an implementer can write Protocol-v2-compliant tests for the new quiz product (declarative
quiz model, rank-correlation/weighted-inversion scoring, classification scoring, seeded-shuffle
randomization, badge rules, leaderboard aggregation) and the proctor server (event-sourced sqlite).

Canonical reference the whole system points back to:
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️protocol-v2-specification.md`
(not re-read here in full — this report documents the *current, working* shape from the router,
README, schema and three worked cases).

---

## 1. Language-agnostic tests: the `🧪️tests/<emoji-case>/` pattern

### 1.0 The layout, verbatim from the test domain's own README

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/README.md`:

```
<owner>/
├── 🖼️assets/                       static production data
├── 🧫️fixtures/                     immutable testing input owned by this domain
├── 📚️examples/<example>/           executable usage projects, with their own 🧪️tests
├── 🔮️oracles/<oracle>/             real reference or comparator implementations, with their own 🧪️tests
├── 🧪️tests/<emoji><kebab-case>/    one leading emoji identity, exactly as every other path segment
│   ├── 🧫️fixtures/                 immutable, private to this case
│   ├── 🥒️.feature                 the normative, language-neutral contract
│   ├── 🦀️.rs                      one adapter per implementation that claims the capability
│   ├── 🟦️.ts
│   ├── 🐹️.go
│   ├── 🐍️.py
│   └── 🔷️.cs
└── 📦️packages/<language>/          the implementations under test
```

> These four collection names are exact: `🧪️tests`, `🧫️fixtures`, `📚️examples`, and `🔮️oracles`.
> Test helpers and harness code belong inside the test-case implementation that uses them.

A test is owned by the **nearest language-neutral domain owner** (an artifact/standard/subset, a
repo-product module, etc.) — never by a language-specific package. One `🥒️.feature` is the
normative contract; each sibling `<ext>.ts/.rs/.go/.py/.cs` file is a **native adapter**, i.e. a
handwritten function per scenario in that language, not a generated binding. The directory name
carries exactly one leading emoji grapheme + kebab-case (`testCaseSlugPattern` in
`🔣️taxonomy.json`, quoted in §6).

### 1.1 Example A — `✒️mutate-writer-1` (Rust subject, Python in-repo second implementation as oracle)

Path: `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/✒️mutate-writer-1/`
Files: `🥒️.feature`, `🐍️.py`, `🦀️.rs` (no local `🧫️fixtures/` — it reads the subset-owned
`shared://🧬️mutations/<vector>/...` fixtures instead).

`🥒️.feature` (verbatim, in full):

```gherkin
@capability-writer-1-mutate
@oracle-writer-python-independent
@comparison-ordered-json-v1
@mutations-writer-1-any
Feature: Apply every typed writer document mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.writer.writer` document and its four typed mutations, written in
  Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, from rule 1 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the four committed vectors. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library, and why the previous answer was
  wrong. [...] this document holds NO PROSE. It is a handle record — an id, a language id, a URI and
  a composed child handle into an `s.stdio.semio@v1/document` — and nothing outside this repository
  models an editor document whose body is a child artifact addressed by content.

  ✅️ WHAT THIS CASE'S EVIDENCE ACTUALLY COVERS. Three of the four kinds are document-level scalar
  setters and are fully adjudicated [...] The fourth, `edit-text` [...] its committed vector pins
  `{status: no-op, messages: [{level: warn, code: mutation.no-op}]}` [...]

  🚧️ ONE OF THE NINE SCENARIOS IS REFUSED BY CLAUSE, and reported rather than worked around.
  `identity-round-trip`. The committed grammar is the repository-wide PLACEHOLDER [...]

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome vector shared://🧬️mutations/<vector>/🎯️outcome/🔣️.json
    When <id> is applied through apply_writer_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id              | vector                                                                |
      | rename-writer   | 🏷️rename-writer/🏷️renames-the-document-to-mission-brief          |
      | change-uri      | 🔗change-uri/🔗️republishes-the-brief-under-a-new-uri            |
      | change-language | 🌐change-language/🔤️switches-the-brief-from-plaintext-to-markdown |
      | edit-text       | ✏️edit-text/⚠️warns-that-the-brief-body-is-unchanged             |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    [... same Examples table, applies <id> then its own computed inverse ...]

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed jack document and print it back without losing or copying anything
    Given the real committed artifact asset://🎬️demo/🗣️.dsl.semio
    When the artifact is parsed to a WriterSnapshot, printed back to `.writer` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
```

Key observations from this one feature file:

* **Tag grammar** on the `Feature:` line: `@capability-<id>` (what production capability this proves),
  `@oracle-<id>` *or* `@no-oracle-<id>` (which registry entry judges it / the recorded refusal),
  `@comparison-<profile>` (how two projections are diffed), plus feature-specific tags like
  `@mutations-<catalog-id>`.
* **Tag grammar per scenario**: exactly one `@id-<slug>`, one `@level-<fundamental|quick|long|exhaustive>`,
  one `@mode-<differential|conformance|property|round-trip|error>`.
  `@mode-differential` = compared against an oracle result; `@mode-conformance` = checked against a
  committed language-neutral statement (see Example C); `@mode-round-trip` = parse→print→reparse law.
* `Scenario Outline` + `Examples:` is literal Gherkin — the *directory-of-vectors* pattern
  (`shared://🧬️mutations/<vector>/{📸️snapshot/⬅️before,🦠️mutation,📸️snapshot/➡️after,🎯️outcome}/🔣️.json`)
  is how one scenario is fixture-parameterized over several committed before/after/outcome triples.
* The feature prose is not decoration — it doubles as an audit trail: it states in English exactly what
  is and is not adjudicated, and names the one scenario it refuses to fake (`identity-round-trip`,
  refused because the committed carrier grammar is a repo-wide placeholder, not invented behavior).

`🐍️.py` (Python oracle adapter, in full) registers itself **oracle-only**:

```python
def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only — registering these
    handlers as subjects too would make the reference its own subject and manufacture a green
    self-comparison."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate_handler(kind))
        built = built.oracle("inverse-%s" % kind, inverse_handler(kind))
    return built.oracle("identity-round-trip", refuse_carrier)
```

It imports `from semio_repo_test import Adapter, Outcome` — a small host library, not the writer's own
Rust code — and hand-rewrites the mutation semantics from the JSON schema + a derivation-rules ticket,
never by reading the Rust source. Its handlers throw `AssertionError` with precise messages (e.g.
`touches_one`, `restores`, `equals_committed`) — the assertions embedded in the Python **are** the law,
independent of whatever the Rust subject does.

`🦀️.rs` (Rust subject adapter) is gated behind `#[cfg(feature = "sut")]` so the oracle-only build never
links it, reads the same committed fixture files via `ctx.fixture_bytes("shared://...")`, and registers
itself the opposite way:

```rust
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
    }
    #[cfg(feature = "sut")]
    { built = built.subject("identity-round-trip", subject::round_trip); }
    built
}
```

**How the `.feature` binds to code**: it is *not* Cucumber and *not* a generic Gherkin step-runner.
Prose/`Given/When/Then` text is documentation for humans; the only machine-read parts of the feature
are (a) the tag lines, (b) the `Scenario`/`Scenario Outline` id (from `@id-…` + the scenario title),
(c) the `Examples:` table (vector directory names), and (d) each scenario's doc-string JSON blob
(` """{"kind": "...", "vector": "..."}""" `), which is handed verbatim to the adapter as `ctx.doc_json()`
/ `doc_json(ctx)`. Each language's `adapter()` function is a **hand-written registry** mapping
`scenario-id → handler-function`; the harness (documented in §4) discovers `.feature` files, expands
`Scenario Outline`/`Examples` into concrete scenario ids, and for each one calls the registered handler
in the oracle role and/or the subject role(s), then diffs their `Outcome` projections. There is no
Gherkin parser executing steps as code — the step text is read only for the one `Given .../asset://…`
line a handler needs (see `uri_in`, `text_at` helpers below) plus the doc-string JSON.

### 1.2 Example B — `🔢️numstat-parsing` (Rust + Go subjects, TypeScript hosts a **real CLI** as oracle)

Path: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/🧪️tests/🔢️numstat-parsing/`
Files: `🥒️.feature` (47 lines), `🟦️.ts` (235 lines), `🦀️.rs` (70 lines), `🐹️.go`. No local fixtures dir
either — it reads `shared://🎞️git-transcript.json` and `shared://🌱️repository-recipe.json`.

`🥒️.feature` (verbatim, in full):

```gherkin
@capability-repo.metrics.numstat-parsing
@oracle-git-numstat-cli
@comparison-ordered-json-v1
Feature: A git numstat stream parses into per-commit line deltas
  `loc` reads its history from `git log --numstat`, so the parser has to survive every shape the
  real git binary emits: paths octal-escaped inside double quotes under `core.quotepath`, renames
  written as `old => new`, binary changes written as `-` and `-` with no line counts, merge commits
  that carry no numstat block at all under `--first-parent`, and paths the `loc` walk must never
  count (a dot-prefixed directory, the repository's own `.🧬semio` meta tree).

  The subjects replay the recorded transcript shared://🎞️git-transcript.json — the oracle rebuilds the
  same repository from shared://🌱️repository-recipe.json in a temporary directory, runs the real
  `git` binary over it, and answers from that — so agreement means the parser agrees with git and
  not merely with a file somebody typed.

  @id-recorded-transcript-matches-real-git
  @level-fundamental
  @mode-differential
  Scenario: The committed transcript is still exactly what git produces
    Given the recorded transcript and the recipe it was recorded from
    When the host reports the digests of the two log streams and the commit ids in them
    Then the recorded stream and a freshly recorded one agree digest for digest

  @id-parses-recorded-numstat-stream
  @level-fundamental
  @mode-differential
  Scenario: Every commit block becomes one record with weighted bucket sums
    Given the recorded `--no-merges` stream
    When the host parses it into commit records
    Then every implementation projects the same shas, timestamps, authors and per-bucket deltas

  @id-resolves-renames-and-quoted-paths
  @level-fundamental
  @mode-differential
  Scenario: Quoted Unicode paths are decoded and renames resolve to the new path
    Given the recorded `--no-merges` stream
    When the host reports every file row it saw
    Then the emoji paths are decoded, the rename names its source, the binary row carries no counts
    And the dot-prefixed and `.🧬semio` rows classify into no bucket

  @id-merge-commit-carries-no-file-rows
  @level-quick
  @mode-differential
  Scenario: A merge commit is a record with an empty file list, never a dropped commit
    Given the recorded stream taken without `--no-merges`
    When the host parses it into commit records
    Then the merge commit appears with no file rows and an empty delta
```

This is the important variant: `@oracle-git-numstat-cli` is a **`third-party-cli` oracle** — not an
npm/pip library at all, but the real `git` binary. The TypeScript adapter shells out to it directly:

```ts
/** 🖥️ Runs the real `git` binary and fails loudly — an oracle that silently degrades proves nothing. */
function git(cwd: string, args: string[], author?: Person, when?: number): string {
  const result = spawnSync("git", args, { cwd, env: { ...process.env, ...identity, ...stamp }, encoding: "buffer", maxBuffer: 64 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`git ${args.join(" ")} failed: ${result.stderr?.toString("utf8")}`);
  return result.stdout.toString("utf8");
}
```

and registers itself via the shared TS helper `defineTestAdapter` (contrast with Rust's `Adapter::new`
builder and Python's `Adapter(...).oracle(...)`):

```ts
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "recorded-transcript-matches-real-git": { oracle: (ctx) => { ... } },
    "parses-recorded-numstat-stream": { oracle: (ctx) => ({ projection: parseNumstat(live(ctx).noMerges) }) },
    "resolves-renames-and-quoted-paths": { oracle: (ctx) => ({ projection: parseNumstat(live(ctx).noMerges).flatMap((c) => c.files) }) },
    "merge-commit-carries-no-file-rows": { oracle: (ctx) => ({ projection: parseNumstat(live(ctx).withMerges) }) },
  },
});
```

Because every TS scenario key here is `oracle:`, never `subject:`, the TS adapter is excluded from the
subject role by `subjectImplementations()` (§4) — it "hosts a third-party reference and nothing else."
`🦀️.rs` and `🐹️.go` are the real subjects, both gated the same `#[cfg(feature = "sut")]` way, reading
the *committed* transcript (not re-running git) via `ctx.fixture_bytes("shared://🎞️git-transcript.json")`.

### 1.3 Example C — `🧩️mount-contract` (`@no-oracle`, `@mode-conformance`, Rust + Python + TypeScript twin)

Path: `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mount-contract/`
Files: `🥒️.feature`, `🐍️.py`, `🟦️.ts`, `🦀️.rs`; fixture at
`…/✳️any/🧫️fixtures/🧩️mount-contract/🔣️.json` (subset-level shared fixture, not case-local).

`🥒️.feature` head + one full scenario (verbatim):

```gherkin
@capability-wfc-bitmap-1-mount
@no-oracle-wfc-bitmap-mount-contract-statement
@comparison-ordered-json-v1
Feature: The bitmap subset mounts the identities, windows and examples it declares
  The committed `shared://🧩️mount-contract/🔣️.json` is the language-agnostic statement of
  what this subset mounts. `🦀️.rs` beside this file is its Rust half — it reads the real manifests
  the plugin builder produces — while `🐍️.py` and `🟦️.ts` are second readers that link nothing of
  this repository: the Python one checks the statement against the kind directories on disk, the
  TypeScript one checks its canonical forms.

  @id-surface-ids
  @level-fundamental
  @mode-conformance
  Scenario: The declared surface ids are the canonical ones
    Given the committed mount contract
    Then the editor app id is s.wfc.bitmap@1/*#editor
    And the viewer app id is s.wfc.bitmap@1/*#viewer
    And the OS artifact kind id is 2d.wfcbitmap
    And the inference tool id is s.wfc.bitmap.solve
```

The committed fixture `🧫️fixtures/🧩️mount-contract/🔣️.json` (in full):

```json
{
  "editorAppId": "s.wfc.bitmap@1/*#editor",
  "viewerAppId": "s.wfc.bitmap@1/*#viewer",
  "dialect": "s.wfc.bitmap@1/*",
  "artifactKindId": "2d.wfcbitmap",
  "inferenceToolId": "s.wfc.bitmap.solve",
  "windowKindIds": ["wfc-bitmap-input", "wfc-bitmap-output"],
  "windowLabels": [{"en": "Input", "de": "Eingabe"}, {"en": "Output", "de": "Ausgabe"}],
  "mutationKinds": ["change-seed", "resize-input", "set-input-pixels", "add-palette-color", "change-palette-color", "remove-palette-color", "resize-output", "change-model", "pin-pixel", "unpin-pixel"],
  "examples": [
    {"id": "rooms-16", "label": {"en": "Rooms 16", "de": "Räume 16"}, "inputWidth": 16, "inputHeight": 16, "paletteSize": 3},
    {"id": "flowers-24", "label": {"en": "Flowers 24", "de": "Blumen 24"}, "inputWidth": 24, "inputHeight": 24, "paletteSize": 4}
  ]
}
```

TS adapter — every scenario key is `subject:`, none is `oracle:` (a `@no-oracle` case still runs its
subjects, it just has no oracle to diff against):

```ts
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "surface-ids": { subject: (ctx) => {
      const contract = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(STATEMENT))) as MountContract;
      for (const [field, value] of Object.entries(CANONICAL_IDS)) if (contract[field as keyof typeof CANONICAL_IDS] !== value) throw new Error(`surface-ids: ${field} is ${contract[field as keyof typeof CANONICAL_IDS]}`);
      return { projection: { ...CANONICAL_IDS } };
    } },
    /* window-kinds, mutation-vocabulary, examples similarly */
  },
});
```

The matching registry entry (`🔮️oracles/🔣️.json` at the bitmap subset root):

```json
"noOracleDecisions": [
  {
    "id": "wfc-bitmap-mount-contract-statement",
    "capabilities": ["wfc-bitmap-1-mount"],
    "rationale": "The mount surface [...] is this repository's own registration contract, which no third party implements. It is stated once, language-neutrally, in `../🧫️fixtures/🧩️mount-contract/🔣️.json`; the Rust half reads the real manifests the plugin builder produces, and the TypeScript and Python halves are independent readers that check the same statement against the kind directories on disk.",
    "substitutes": ["specification-vectors", "independent-implementations"]
  }
]
```

This is the pattern to copy for **quiz scoring rules that are semio-native** (e.g. a badge-award rule
that is purely this repo's own policy, with no external reference): commit the rule's expected outputs
as a language-neutral JSON statement, write one reader per implementation language, tag
`@no-oracle-<id>` + `@mode-conformance`, and record a `noOracleDecisions` entry whose `substitutes`
includes `independent-implementations` (which requires **at least two** language readers — the parity
harness enforces this, see §4).

---

## 2. Test runners: bun test vs vitest vs cargo test, and how `📜️script.ts test [level]` is wired

There are **two separate layers**, and it is important not to conflate them:

### 2.1 Layer 1 — ordinary per-package unit tests (bun test / vitest / cargo test|nextest / pytest / go test / dotnet test)

Every package's `📋️project.json` **only** calls that package's own `📜️script.ts` (per the AGENTS.md
rule quoted at the top of this session) — verbatim from
`✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📋️project.json`:

```json
"test": { "executor": "nx:run-commands", "options": { "cwd": "…/📦️packages/🦀️rust", "command": "bun ./📜️script.ts test", "forwardAllArgs": true } },
"test-quick": { "executor": "nx:run-commands", "options": { "cwd": "…/📦️packages/🦀️rust", "command": "bun ./📜️script.ts test quick", "forwardAllArgs": true } },
"test-long": { ... "bun ./📜️script.ts test long" ... },
"test-exhaustive": { ... "bun ./📜️script.ts test exhaustive" ... }
```

and that package's `📜️script.ts` calls the shared library helper — verbatim,
`✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📜️script.ts`:

```ts
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-s-plugin-writer"], this.repoRoot, rest);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
```

and a TS package's equivalent (`✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/📜️script.ts`, in full)
calls `bun test` **directly as a subprocess of the running bun process** (`process.execPath` is the
`bun` binary), not through vitest:

```ts
class TestScript extends BundleScript {
  run(): void {
    runCmd(process.execPath, ["test", ...[...testFilePaths].map(path => resolve(this.repoRoot, path))], { cwd: this.repoRoot });
    console.log("[DEBUG] writer ts ok");
  }
}
```

(Note the literal `console.log("[DEBUG] writer ts ok")` — a real, committed example of the
`[DEBUG] `-prefix convention from AGENTS.md, discussed in §6.)

**`resolveTestLevel`** (shared library, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:1167`)
is the single source of truth for the level vocabulary and is what `test quick` / `test long` /
`test exhaustive` resolve against:

```ts
export const TEST_LEVELS = ["fundamental", "quick", "long", "exhaustive"] as const;
export type TestLevel = (typeof TEST_LEVELS)[number];

export function resolveTestLevel(segments: string[], minimum: TestLevel = "fundamental"): { level: TestLevel; rest: string[] } {
  const [first, ...restIfLevel] = segments;
  const requested = isTestLevel(first) ? first : activeTestLevel();
  const level = testLevelRank(requested) >= testLevelRank(minimum) ? requested : minimum;
  process.env.SEMIO_TEST_LEVEL = level;
  if (level === "exhaustive" && process.env.SEMIO_COVERAGE === undefined) process.env.SEMIO_COVERAGE = "1";
  return { level, rest: isTestLevel(first) ? restIfLevel : segments };
}
```

It sets `SEMIO_TEST_LEVEL` in `process.env` so **every child process spawned afterwards (vitest, cargo,
go, pytest, dotnet) inherits it without explicit plumbing** (doc comment, same file). Per-runner level
translation, all in the same file:

```ts
export function goLevelTestArgs(level = activeTestLevel()): string[] { /* -timeout, keeps -short through quick, -skip ^Test(Long|Exhaustive) above it */ }
export function vitestLevelArgs(level = activeTestLevel()): string[] { return ["--testTimeout", ms, "--hookTimeout", ms, "--teardownTimeout", ms]; }
export function bunTestLevelArgs(level = activeTestLevel()): string[] { return ["--timeout", String(testLevelBudgetMs(level))]; }
export function pytestLevelArgs(level = activeTestLevel()): string[] { /* -m "not quick and not long..." plus --timeout=<s> --timeout-method=thread */ }
export function dotnetLevelArgs(level = activeTestLevel()): string[] { /* --filter "Category!=long&Category!=exhaustive", --blame-hang --blame-hang-timeout <ms> */ }
```

**`cargo test` is never invoked bare** — the repo prefers `cargo nextest` (falling back to plain
`cargo test` / `cargo llvm-cov test` when nextest isn't installed), via `runCargoTestBudgeted`
(`🟦️.ts:1804`), doc comment verbatim:

```
🦀️Warm-builds the exact test runner invocation with the opt-in [[buildBudgetMs]], then runs
assertions under the active level's budget and [[nextest.toml]] profile (per-test
`slow-timeout`), appending cumulative `--skip <level>::` filters for every level above it (tests live in
`mod quick`/`mod long`/`mod exhaustive` submodules inside `mod tests`; unscoped tests are `fundamental`).
```

and, on missing nextest: `console.error("[budget] cargo-nextest not installed — falling back to cargo
test (run setup or: cargo install cargo-nextest --locked)")`. It also guarantees a 128 MiB
`RUST_MIN_STACK` floor for assertion threads.

**Where vitest actually runs**: confirmed by a committed policy-test fixture
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/📐️test-layout/🔣️.json`), one of the
canonical "valid layout" vectors is a `vitest.config.ts`:

```ts
import { defineConfig } from "vitest/config";
export default defineConfig({ test: { include: ["../../🧪️tests/**/🟦️.ts"] } });
```

So: **cargo (nextest-first) for Rust**, **`bun test` invoked directly by script.ts for plain TS unit
suites**, **vitest for TS packages that declare a `vitest.config.ts`** — both are valid per-bundle
choices, selected by the bundle's own `📜️script.ts`, never centrally forced. `pytest`/`go test`/`dotnet
test` exist for Python/Go/.NET bundles the same way.

### 2.2 Layer 2 — the cross-language Protocol v2 harness (`🧪️tests/<case>/`)

This is **not** run through `cargo test`/`bun test`/`vitest` at all. It is driven by the repo-test-domain
router, `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts` (in full — 77 lines):

```ts
// 🧪️ Router of the repository testing domain:
//   bun ./📜️script.ts <discover|contract|oracle|subject|parity|run|report|clean|dependency|nx|doctor|acceptance> [args…]
const router = new ScriptRouter(import.meta.dir)
  .register("discover", DiscoverScript)
  .register("contract", ContractScript)
  .register("oracle", OracleScript)
  .register("subject", SubjectScript)
  .register("parity", ParityScript)
  .register("run", RunScript)      // contract, then parity
  .register("test", TestScript)    // routes to schema | command-composition-source | RunScript
  ...
```

`bun ./📜️script.ts run <level> --owner <path> --case <slug>` (or the case's generated Nx `test`
target, see below) is "contract, then parity" — i.e. static checks, then oracle+subject+diff.

**Per-language host materialization** (what actually executes an adapter), from
`🖥️host/🏗️materialization/🟦️.ts`: a Rust host is built with `command: "cargo"` (`env: {
CARGO_TARGET_DIR: cargoTargetDirectory(repoRoot) }` — a generated `cargo run`/`cargo build` of a
synthesized host crate, not `cargo test`), a Python host resolves `.venv/…/python.exe` or falls back to
`python3`, and a .NET host runs `command: "dotnet"`. These generated hosts live only under
`.🧬semio/🦑️repo/⚡️cache/tests/hosts/` (README: "Generated hosts and outputs live only under
`.🧬semio/🦑️repo/⚡️cache/tests/`, carry an ownership marker, and are safe to delete. A committed
generated wrapper is a taxonomy breach.").

### 2.3 How Nx targets get generated per case (no committed `project.json` per test case)

Verbatim from the test domain's own Nx plugin,
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs` (`testCaseProjects`, a `createNodesV2`
implementation that scans every `🥒️.feature` file matching the taxonomy's case-slug pattern and
generates one synthetic Nx project per case):

```js
targets: {
  lint: scoped("lint", `contract ${select}`),
  "test-contract": scoped("test-contract", `contract ${select}`),
  "test-oracle": scoped("test-oracle", `oracle ${select}`),
  "test-subject": scoped("test-subject", `subject ${select}`),
  "test-parity": scoped("test-parity", `parity ${select}`),
  test: scoped("test", `run ${select}`),
  ...Object.fromEntries(LEVELS.map((level) => [`test-${level}`, scoped(`test-${level}`, `run ${level} ${select}`)])),
},
```

where `select = `--owner ${JSON.stringify(ownerRel)} --case ${caseSlug}`` and `target(...)` (same file)
shows the actual generated executor:

```js
function target(domain, command, inputs, cacheable = true, scope) {
  return {
    executor: "nx:run-commands",
    options: { cwd: domain, command: `bun ./📜️script.ts ${command}`, forwardAllArgs: false, env: { SEMIO_TEST_OUTPUT_SCOPE: scope } },
    inputs,
    outputs: ["results", "reports", "diffs"].map((child) => `{workspaceRoot}/.🧬semio/🦑️repo/⚡️cache/tests/tasks/${scope}/${child}`),
    cache: cacheable,
  };
}
```

So **every generated test case gets `test`, `test-quick`, `test-long`, `test-exhaustive`,
`test-contract`, `test-oracle`, `test-subject`, `test-parity`, `lint`** targets automatically, all
routed through the one `bun ./📜️script.ts <phase> [level] --owner … --case …` command at the repo-test
domain root — never a per-case `project.json`. Levels are cumulative: "`test-long` selects every
scenario tagged `fundamental`, `quick` or `long`" (comment, same file).

For the quiz product this means: **do not hand-write `project.json` targets for `🧪️tests/` cases** —
just create the `🧪️tests/<emoji-case>/🥒️.feature` + adapters, and Nx will generate the targets. Only
the *package* (`📦️packages/🦀️rust`, `📦️packages/🟦️typescript`, …) needs its own `project.json`
+ `📜️script.ts` wired for its own ordinary unit-test layer (§2.1).

---

## 3. Oracles: schema, third-party declaration, comparison, and no-oracle decisions

There are **two on-disk shapes** in the wild, both named `🔮️oracles/🔣️.json` — a JSON-Schema-governed
Protocol-v2 shape (subset-owned, rich) and a lighter flat `🔣️oracle.json` shape (product-owned). Both
are read by the same `loadOracleRegistry()` (used in `oracleDecision()`, §4).

### 3.1 Full Protocol v2 shape — `🔮️oracles/🔣️.json` (subset-owned)

Schema: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json` (3740 lines; key `$defs`:
`OracleRegistry` requires `schemaVersion`, `oracles`, `noOracleDecisions`; plus `MutationManifest`,
`NoOracleDecision`, `OracleHostPackage`, `SubjectFeatures`).

Verbatim `NoOracleDecision` schema:

```json
"NoOracleDecision": {
  "type": "object",
  "additionalProperties": false,
  "description": "📇️ A recorded decision that a capability has no credible reference. For a runtime mutation capability it stands only with `coversMutations: true` and a `referenceSurvey`, and even then the gate keeps the gap visible as a medium finding instead of discharging it.",
  "required": ["id", "capabilities", "rationale", "substitutes"],
  "properties": {
    "id": { "$ref": "#/$defs/Slug" },
    "capabilities": { "type": "array", "items": { "type": "string" }, "minItems": 1 },
    "rationale": { "type": "string", "minLength": 20 },
    "substitutes": { "type": "array", "items": { "enum": ["specification-vectors", "metamorphic-laws", "independent-implementations", "second-parser"] }, "minItems": 1 },
    "coversMutations": { "type": "boolean", "description": "✅️ True claims mutation capabilities; the claim needs a referenceSurvey and never silences the gap, it only stops it blocking." },
    "referenceSurvey": { "$ref": "#/$defs/ReferenceSurvey" }
  }
}
```

`Oracle.kind` enum (same schema file): `third-party-library`, `third-party-cli`,
`standards-reference-tool`, `verified-native-second-implementation`, `cross-semio-implementation`
(the last only from a promoted/superseded `cross-semio-implementation` entry, per the writer example's
`"kind": "verified-native-second-implementation"` history in §1.1's registry entry).

Real example fields (writer subset's `🔮️oracles/🔣️.json`, quoted earlier in full for `id:
"writer-python-independent"`): `id`, `ecosystem`, `package` (empty string when the reference needs no
distribution — "this entry contributes nothing to `🔒️dependencies.json`"), `version`, `capabilities`,
`comparisonProfiles`, `license`, `testOnly: true`, `rationale` (long-form prose — this **is** the third-
party survey, kept as an audit trail), `kind`, `engine: {family, implementation, version}`,
`productionReachable: false`, `networkDuringExecution: false`, and for a native second implementation a
`nativeSecondImplementation` block: `{format, noThirdPartySurvey: {ecosystemsSearched,
candidatesConsidered}, subjectImplementationLanguage, secondImplementationLanguage,
specificationSource, fixtureCoverage: {vectors, capabilitiesCovered}}`.

Also present in this shape: `mutationCatalogs` (per-mutation scenario→vector→directoryName mapping) and
`mutationManifests` (per-mutation `payloadSchema`, `outcomes: [applied, no-op, ...]`,
`productionDispatch: {operation, bridgeVersion, variant}`, and `oracleRequirements: [{capability,
qualifyingKind}]` — this is what `bun ./📜️script.ts contract` cross-checks against the owner's real
production mutation bridge, per the README's "Mutations, subsets and external oracles" section).

`oracleHostPackages` — how a library is actually provisioned per ecosystem (README, verbatim):

```json
"oracleHostPackages": [
  { "implementation": "rust",       "package": "semio-s-plugin-stdio-test-oracle", "path": "…/📦️packages/🦀️rust", "features": ["oracles"] },
  { "implementation": "python",     "package": "pypdf",  "version": "6.14.2" },
  { "implementation": "typescript", "package": "semver", "version": "7.8.5" }
]
```

`path` present ⇒ **local in-repo source** (Rust `Cargo.toml` path-dependency; a crates.io coordinate is
refused for it). `path` absent ⇒ **external distribution**: Python gets a cache-local venv under
`.🧬semio/🦑️repo/⚡️cache/tests/hosts/` (layered on the repo's own `.venv` via a `.pth` file so an
already-installed version is reused, never installed into the system interpreter); TypeScript resolves
straight from the repo's own `node_modules` — "one lockfile, one version of every library" — and
**reports the declaration unmet if it doesn't resolve** (no private copy is ever installed for TS).

### 3.2 Flat product-owned shape — `🔣️oracle.json` (e.g. `🧰️framework/🛍️products/📓️print/🔣️oracle.json`)

Verbatim, in full (presentation product):

```json
{
  "$comment": "🔮 Test-only third-party references of the presentation product. Every entry is a devDependency of 📦️packages/🟦️typescript/🎯️targets/⚛️react and is imported only from 🧪️tests/**; no runtime code may depend on one. `@oracle-<id>` tags in the Gherkin features name these ids.",
  "schemaVersion": 1,
  "oracles": [
    {
      "id": "remark",
      "package": "unified",
      "version": "11.0.5",
      "entry": "unified",
      "covers": ["markdown-html-compilation"],
      "reason": "Judges the owned slide markdown compiler against the CommonMark and GFM reading of the JavaScript ecosystem — remark-parse, remark-gfm, remark-rehype and rehype-stringify composed by unified — so a parsing or escaping rule cannot agree with itself."
    }
  ]
}
```

Print product's `🔣️oracle.json` has 5 entries: `ajv` (JSON Schema draft 2020-12, `entry:
"ajv/dist/2020"`), `d3-array`, `d3-scale`, `d3-shape`, `markdown-it` — each with `id, package, version,
[entry], covers: [capability-ids], reason`. This shape is simpler (no `kind`/`engine`/comparisonProfiles
machinery) and appears to be an earlier/product-level convention that coexists with the richer subset-
level `🔮️oracles/🔣️.json`; the print product **also** has its own `🔮️oracles/🔣️.json` (2000+ lines)
with the full Protocol v2 shape (including the `jstat` entry below) — so at product scale both files are
maintained side by side, the flat one apparently the older/coarser declaration and the rich one the
per-capability Protocol v2 registry actually consulted by `oracleDecision()`.

### 3.3 A statistics-oracle precedent already in the repo (relevant to quiz scoring)

`🧰️framework/🛍️products/📓️print/🔮️oracles/🔣️.json` already registers **jstat** (`1.9.6`, MIT, devDependency) as a `third-party-library` oracle for `"viz-scientific-distributions"`:

```json
{
  "id": "jstat", "kind": "third-party-library", "ecosystem": "javascript", "package": "jstat", "version": "1.9.6",
  "capabilities": ["viz-scientific-distributions"],
  "comparisonProfiles": ["viz-probe-v1", "viz-probe-coarse-v1", "viz-probe-exact-v1", "floating-point-v1", "ordered-json-v1"],
  "license": "MIT", "testOnly": true,
  "hostPath": "🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript",
  "engine": { "family": "d3", "implementation": "d3-shape", "version": "3.2.0" },
  "productionReachable": false, "networkDuringExecution": false
}
```

(`engine.family: "d3"` here looks like a copy-paste artifact from the adjacent d3-shape entry — worth
fixing if this entry is reused, but not this session's job to touch.)

### 3.4 How comparison actually works

`@comparison-<profile>` on the feature names a `ComparisonProfile`, resolved from `profileTable(loadOracleRegistry(repoRoot))` (framework-wide profiles + every owner-contributed one), and `evaluateParity`
(from the shared TS package, `⚖️parity/📋️orchestration/🟦️.ts`) walks oracle vs. subject `Outcome`
projections through it — a structural JSON diff by default (`ordered-json-v1`, `floating-point-v1` seen
above — the latter presumably tolerance-based for scores/statistics). Protocol v2 additionally supports
`ComparisonPipeline`: an ordered list of external **probe** stages (`viz-probe-v1` etc.) whose typed JSON
output is checked against declarative `<key>Max/Min/Equal` assertions — for a case comparing more than
one artifact or needing a measured metric instead of a structural diff. This is directly relevant to a
weighted-inversion/Kendall-tau scoring oracle: it can be registered as a `comparisonProfile` (tolerance-
based numeric diff) rather than requiring byte-exact agreement.

### 3.5 Validator that enforces oracle presence

`⚖️parity/📋️orchestration/🟦️.ts`, function `oracleDecision()` (quoted in §4) returns a `problem` string
whenever "feature declares neither an oracle nor a no-oracle decision" or "unknown oracle id" or "native
oracle X names no hostImplementation to drive it from" — these `problem`s are collected into
`runPhases()`'s `problems` array and fail the run (`return summary.failed + summary.errored +
problems.length === 0 ? 0 : 1;`). Additionally (same function, `runPhases`, the final loop): a
`noOracleDecision` claiming the `independent-implementations` substitute is rejected if fewer than two
implementations actually produced cross-subject parity results:

```ts
if (decision.implementation === null && decision.noOracleDecision !== null && crossPairs.length === 0) {
  const substitutes = loadOracleRegistry(repoRoot).noOracleDecisions.find((entry) => entry.id === decision.noOracleDecision)?.substitutes ?? [];
  if (substitutes.includes("independent-implementations")) problems.push(`${discovered.caseDir}: no-oracle decision ${decision.noOracleDecision} claims the independent-implementations substitute but only one implementation ran`);
  else if (!caseResults.every((result) => ["conformance", "property", "round-trip", "error"].includes(planModeOf(repoRoot, discovered, level, result.scenario))))
    problems.push(`${discovered.caseDir}: no-oracle decision ${discovered.caseDir} rests on ${substitutes.join(", ")}, which only discharge a conformance, property, round-trip or error scenario — a differential scenario needs an oracle or a second implementation`);
}
```

i.e.: a `@no-oracle-…` case is only legitimate if either (a) it has ≥2 independently-written language
adapters that must cross-agree (`independent-implementations`), or (b) every one of its scenarios is
tagged `@mode-conformance|property|round-trip|error` (self-discharging — no external comparison needed
by construction, e.g. checking a committed statement against itself, or a round-trip law).

---

## 4. Parity harness — Protocol v2

### 4.1 What a "case" is and how phases run

A case = one discovered `🧪️tests/<emoji-slug>/🥒️.feature` + its sibling adapter files
(`DiscoveredCase`, from `🔍️discovery/🎛️selection/🟦️.ts`: `selectCases`, `selectImplementations`).
`runPhases(repoRoot, segments, phases)` (`⚖️parity/📋️orchestration/🟦️.ts`, quoted at length — this is
the actual engine behind `contract|oracle|subject|parity|run`) does, per discovered case:

1. `oracleDecision()` — resolves which registry entry judges this case (or the recorded no-oracle id),
   verifies the entry's `executables` are on `PATH` (else `oracleUnavailable`, reported not silently
   skipped), and returns a structured `{implementation, hostedByCase, noOracleDecision, comparison,
   problem, unavailable}`.
2. `subjectImplementations()` — the languages that both (a) the repo actually ships a package for this
   owner in, and (b) are not themselves the declared oracle's *own* hosting language (unless that
   language's adapter also separately registers a `subject` handler, per §1's numstat example where TS
   is oracle-only and thus excluded).
3. For each phase requested (`oracle`, `subject`, or both = `parity`), `executeOne(repoRoot, discovered,
   level, role, implementation)` runs the generated per-language host (§2.2) and collects `TestResult`s.
4. If both oracle and subject results exist: either pipeline-verdict comparison (`ComparisonPipeline`) or
   `evaluateParity(comparison, caseResults, profiles)` structural diff, writing a `.diff.txt` per failing
   scenario into `testCacheDir(repoRoot, "diffs")`.
5. **Always**, regardless of oracle presence: `evaluateCrossSubjectParity()` — pairwise agreement between
   every two subject implementations, described in code as "the ONLY parity evidence a recorded
   no-oracle case can offer."
6. A case selected but producing **zero** results is reported as `not-exercised` with a reason (never
   silently counted as a pass) — "A selected case that produced no result at all is not a pass — it is
   an absence of evidence, and the two must never look the same."
7. Run-level output: `📊️summary.json`, `📤️results.jsonl`, `📋️junit.xml`, `📈️metrics.json` under
   `reportsDir(repoRoot)`, plus a `markRunComplete(dir)` marker.

### 4.2 The "harness case" file format

There is **no separate JSON "case file"** distinct from the `.feature` + fixtures already documented in
§1 — "case" in Protocol v2 terminology *is* the `🧪️tests/<slug>/` directory (feature + adapters +
optional local `🧫️fixtures/`). What IS a distinct file format is the **fixture vector directory**
(§1.1's `shared://🧬️mutations/<vector>/{📸️snapshot/⬅️before,🦠️mutation,📸️snapshot/➡️after,🎯️outcome}/🔣️.json`)
and the **oracle/mutation-manifest registry** (§3). The `📡️protocol/🦀️.rs` file (571 lines) defines the
Rust-side wire types the generated hosts speak to the orchestrator (scenario id, role, fixture URIs,
outcome projection + raw bytes) — this is the "Protocol v2" the ticket-name and README refer to, not a
YAML/JSON case-description file authors hand-write.

### 4.3 Does it apply outside the repo product?

Yes — the harness's owner (repo product, `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/`) is the
*infrastructure*; every `🧪️tests/<case>/` anywhere in the monorepo (plugins under `✏️s/🔌️plugins/…`,
the presentation/print/os products, etc.) is *discovered and run by* this one shared harness — confirmed
by both worked examples above living outside the repo product (writer plugin, wfc plugin) and by the
README's explicit statement: "A test is owned by the nearest language-neutral domain owner... never by
a Rust crate, a TypeScript package, a Go module or a .NET project." The one hard exclusion is
`compose/**`, excluded in the discovery library itself (README, "Rules and current enforcement").
**This directly answers the quiz-product question**: quiz scoring/randomization/badge features should
live as `🧪️tests/<case>/` cases under wherever the quiz product's domain owner ends up (e.g. a future
`✏️s/🔌️plugins/🧩️quiz/…` or `🧰️framework/🛍️products/🎓️…` artifact/subset), and they will be picked up
by this same repo-wide harness automatically — no new test infrastructure is needed.

---

## 5. Candidate oracle packages — what's already in `bun.lock` / `package.json`

Checked via `grep` over the root `bun.lock` (workspace-wide single lockfile — "one lockfile, one version
of every library" per §3.1):

| Candidate | In `bun.lock`? | Evidence |
|---|---|---|
| **jstat** (statistics) | **Yes** — `1.9.6`, direct dependency in some package.json, registered oracle in `📓️print/🔮️oracles/🔣️.json` for `viz-scientific-distributions` | Already the precedent for a statistics-library oracle (§3.3) — most direct fit for Kendall-tau/Spearman scoring if it covers rank-correlation, otherwise it establishes the *pattern* to extend |
| **simple-statistics** | **No** — no hits in `bun.lock` | Would be a *new* dependency; per the README's oracle-selection lifecycle ("Search `📇️registry/🔣️.json` first, then the existing test dependencies. Only when no approved library can support the behaviour do you compare new candidates"), jstat should be checked first since it's already vendored and already an approved test-only oracle pattern |
| **seedrandom** | **Indirect only** — pulled transitively by `mathjs@14.0.0` (`"seedrandom": "^3.0.5"` in mathjs's own deps), not a direct workspace dependency | Not currently a first-class devDependency; would need its own explicit registration if used directly as the PRNG oracle |
| **pure-rand** | **No** — no hits | Would be a new dependency |
| **ajv** (JSON Schema validation) | **Yes** — `8.20.0`, already a registered oracle (`📓️print/🔣️oracle.json`, `entry: "ajv/dist/2020"`) validating `🖼️assets/🔣️viz-catalog.json` against a JSON Schema draft 2020-12 | Directly reusable as-is for any quiz-model JSON Schema validation oracle — same `ajv/dist/2020` entry point pattern |
| **better-sqlite3** | **No** — no hits in `bun.lock` | Not present |
| **bun:sqlite** (Bun built-in) | **Already used repo-wide** — real imports found in ≥10 files across the codebase, e.g. `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔒️leases/🟦️.ts` and its own test case `…/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts` | Given AGENTS.md's "MUST NOT create runtime dependencies on external libraries" + "SHOULD use only system libraries provided by the frameworks" rules, and that `bun:sqlite` is already the established choice with zero added dependency, this is the clear pick for the proctor server's event-sourced sqlite store over `better-sqlite3` (which isn't even vendored) |
| **d3-array / d3-scale / d3-shape** | **Yes**, all three, already registered oracles in `📓️print/🔣️oracle.json` and `🔮️oracles/🔣️.json` for binning/quantile/scale/pie-angle geometry | Not directly relevant to quiz scoring, but confirms d3-* family is an approved, already-paid-for oracle family if any quiz visualization (e.g. leaderboard chart) needs one |
| **unified/remark, markdown-it, semver** | **Yes**, all registered oracles for markdown compilation / versioning | Not directly relevant to quiz scoring |

**Recommendation implied by the "prefer existing libraries" rule**: for Kendall tau / Spearman rank-
correlation scoring, evaluate **jstat** first (already vendored, MIT, already precedented as a
statistics oracle with an established `comparisonProfiles` set including `floating-point-v1` for
numeric tolerance) before adding `simple-statistics`. For seeded-shuffle randomization, `seedrandom` is
only one `bun add -D` away from being a direct workspace dependency (it's already resolved and present
in the lockfile transitively via `mathjs`, so adding it directly adds no new package to the dependency
graph, only promotes an existing resolved version to direct); `pure-rand` would be a genuinely new
addition. For JSON Schema validation of the quiz model, reuse the existing `ajv@8.20.0` `third-party-
library` oracle registration pattern verbatim. For the proctor server's sqlite store, use `bun:sqlite`
(already the repo's de facto standard, zero new dependency, consistent with the "no runtime dependency
on external libraries" + "use only system libraries provided by the frameworks" rules in AGENTS.md).

---

## 6. Validators for test presence, fixture naming, and `[DEBUG]` hygiene

### 6.1 Fixture / test-case naming

`🔣️taxonomy.json` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`), verbatim:

```json
"testCaseSlugPattern": "^[a-z0-9]+(?:-[a-z0-9]+)*$",
```

(applies to the kebab-case tail after the case directory's one leading emoji grapheme — enforced by the
Nx plugin's `canonicalCase(vocabulary, ownerRel, caseSlug)` check in `testCaseProjects`, §2.3: a
directory failing this check is simply never discovered/never gets a generated target — silent exclusion
rather than a hard failure message, so a misnamed case just won't show up in `discover`).

### 6.2 Test-presence / feature-tag / contract enforcement

The primary enforcement point is `oracleDecision()`'s `problem` field and the final "unexercised" loop in
`runPhases()`, both in `⚖️parity/📋️orchestration/🟦️.ts` and both quoted in full in §3.5/§4.1 — every
case must declare either `@oracle-<id>` (resolving to a real registry entry with a working
`hostImplementation`/`executables`) or `@no-oracle-<id>` (resolving to a real `noOracleDecisions` entry),
and a `@no-oracle-…` claiming `independent-implementations` is rejected unless ≥2 implementations
actually ran and cross-agreed. The README additionally states (verbatim, "Rules and current
enforcement"): "The legacy backlog is shrink-only. `contract` fails when an area's unmanaged-test count
grows." — i.e. there is a ratchet against *new* untested code, tracked via `🚚️migration.json` (mentioned
in the README's lifecycle step 12: "lower this owner's count in the repository-root `🚚️migration.json`").
I did not find a schema/count validator file for `🚚️migration.json` itself in this pass; it is referenced
but its enforcement code was not located within this session's search budget — worth a follow-up if the
quiz product needs to register/lower a legacy count.

### 6.3 `[DEBUG] ` log-hygiene

No dedicated automated validator/statute was found (searched the test module's own `⚖️policy/🧹️domain/🟦️.ts`
— which only enforces the testing-domain's own folder taxonomy, quoted in full below — and grepped
broadly for `debugPrefix`/`debugLog`/similar names repo-wide with no hits). The convention from AGENTS.md
("You MUST add `[DEBUG] ` prefix to temporary logs so that they can be easily removed later") is followed
in practice but appears to be **honor-system / code-review enforced, not tool-enforced** — e.g. the
literal `console.log("[DEBUG] writer ts ok")` left in the committed writer TS package's `📜️script.ts`
(quoted in §2.1) is a real, currently-committed example of the convention being used (and, arguably, an
example of exactly the kind of leftover the prefix exists to make greppable/removable — `grep -rn
"\[DEBUG\]"` across the repo does surface it and similar lines). For the quiz/proctor work, follow the
same convention but do not expect a CI gate to catch a missing prefix — verbatim policy check, in full,
for context on what "enforcement" actually looks like in this codebase:

```ts
/** 🧹️ Folder policy for this domain: the six generated output roots must never be committed. */
export function policy(): BreachRecord[] {
  const domainRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
  const repoRoot = resolve(domainRoot, "../../../../..");
  const breaches: BreachRecord[] = [];
  for (const name of readdirSync(domainRoot)) {
    if (![ /* ~25 whitelisted child names: 🧱️contract, 🔍️discovery, 🖥️host, ⚖️parity, 🧾️contracts,
             📊️reporting, 🕸️dependencies, 🏭️inventory, 🧾️provenance, 📊️coverage, 🧹️retention,
             🩺️environment, ⚖️policy, 🧬️schema, 📇️registry, 📦️packages, 🧫️fixtures, 🧪️tests,
             📡️protocol, 🏃️runner, 🔮️oracles, 📜️script.ts, 📋️project.json, 🟨️.mjs, 🟦️.ts, 🦀️.rs,
             AGENTS.md, README.md, node_modules */ ].includes(name)) {
      breaches.push({ id: "unknown-domain-child", kind: "testing/taxonomy", scope: `${REPO_TEST_DOMAIN_REL}/${name}`,
        summary: `Unexpected child ${name} in the testing domain root`, priority: "medium",
        reason: "The testing domain root holds its schema, registry, packages, fixtures, self-tests and routers — nothing else.",
        solution: "Move it into the owning child directory, or delete it." });
    }
  }
  /* + a check that no nested .🧬semio cache directory was accidentally committed inside the domain */
  return breaches;
}
```

This is a general `BreachRecord[]`-returning `policy()` convention (repo-wide statute/breach machinery,
referenced generically in the top-level AGENTS.md's formal spec) — a future DEBUG-hygiene statute could
be added the same way (a `policy()` function scanning for `console.log`/`eprintln!`/etc. calls whose
string literal doesn't start with `[DEBUG] `), but none currently exists for that specific rule.

---

## Practical checklist for the quiz product + proctor server

1. **Schema-first**: define the quiz model / scoring / event schema under `🧬️schema/🔣️.json` (JSON
   Schema) before any implementation, per AGENTS.md's "schema-first over code-first" and the harness's
   `contract` phase ("everything provable without executing a test").
2. **One `🧪️tests/<emoji-case>/🥒️.feature`** per scoring algorithm / randomization law / badge rule,
   with `@capability-…`, `@oracle-…` or `@no-oracle-…`, `@comparison-…` on the feature, and `@id-…`
   `@level-…` `@mode-…` per scenario — copy Example A's structure for anything comparable against a
   reference, Example C's for a semio-native rule with no external reference (badge policy, leaderboard
   tie-break rule).
3. **Oracle research order**: check `jstat` (already vendored) for Kendall tau/Spearman before adding
   `simple-statistics`; check `ajv@8.20.0` (already registered) for quiz-model JSON Schema validation;
   promote `seedrandom` (already transitively resolved via `mathjs`) to a direct devDependency for the
   seeded-shuffle oracle rather than adding `pure-rand` fresh, unless `seedrandom`'s algorithm doesn't
   match the production PRNG's needs.
4. **Register every third-party test dependency** in the owner's `🔮️oracles/🔣️.json` (`oracles[]` +
   `oracleHostPackages[]`), never a bare `package.json` devDependency alone — `bun ./📜️script.ts
   dependency` checks oracle purity / production-unreachability against this registry.
5. **`bun:sqlite`**, not `better-sqlite3`, for the proctor server's event-sourced store — zero new
   dependency, already the repo's established choice.
6. **No hand-written `project.json` for test cases** — only for the quiz product's own
   `📦️packages/<language>/` implementation packages; the Nx plugin (`🟨️.mjs`, §2.3) auto-generates
   `test`, `test-quick/long/exhaustive`, `test-contract/oracle/subject/parity` targets for every
   discovered `🧪️tests/<case>/` directory.
7. **CQRS/event-sourcing note** (AGENTS.md, not re-verified this session): the proctor server's event
   store is itself a natural `🧪️tests/<case>/` differential/conformance target — e.g. "replaying a
   committed event log reconstructs the committed projection" as a `@mode-round-trip` or
   `@mode-conformance` scenario, following Example C's committed-statement pattern for the projection
   shape.
