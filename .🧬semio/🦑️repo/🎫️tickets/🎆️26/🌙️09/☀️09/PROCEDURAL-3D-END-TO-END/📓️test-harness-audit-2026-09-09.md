# 🧪️ Test-Harness Audit — `s.procedural.generation3d@1` (2026-09-09)

Scope: every NON-cargo test lane touching `✏️s/🔌️plugins/🌀️procedural` (with emphasis on
`🗿️artifacts/🧊️generation3d`), each one actually **run** (foreground, no cargo, no dev servers).
Full command outputs are saved under `🗑️generated/` in this ticket folder. Sibling generation2d is
used throughout as the freshness/shape baseline per the task brief.

## 1. TypeScript/bun lanes

### 1a. `bun nx run @semio-tech/procedural-js:test`

This is the launch.json target `🧪️test🌀️procedural📚️examples` (`.vscode/launch.json:10340`,
`"command": "bun nx run @semio-tech/procedural-js:test"`).

Command (run twice — once as-is, once with `--skip-nx-cache` to rule out a stale cache hit per the
"never assume a cached green is a real green" rule):

```
bun nx run @semio-tech/procedural-js:test
bun nx run @semio-tech/procedural-js:test --skip-nx-cache
```

Tail of the fresh (`--skip-nx-cache`) run — full output in `🗑️generated/lane1-procedural-js-test-fresh.txt`:

```
> bun ./📜️script.ts test
bun test v1.3.14 (0d9b296a)

 11 pass
 0 fail
 11 expect() calls
Ran 11 tests across 11 files. [379.00ms]

 NX   Successfully ran target test for project @semio-tech/procedural-js
  Cache:             Skipped (--skip-nx-cache)
```

**Discovered = 11, passed = 11, failed = 0.** The 11 files are hardcoded in
`✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts`'s `TestScript.run()`, not
glob-discovered:

- All **8** of generation3d's `📚️examples/*` (`box-shell-preview`, `face-sweep-extrude`,
  `hexagonal-mushroom-column`, `sphere-cut-with-torus`, `sphere-box-fuse`,
  `rectangle-wire-preview`, `rectangle-extrude-volume`, `box-fillet-preview`) — matches the ticket
  goal's "all eight examples load" exactly.
- generation3d's `✏️editor/📚️examples/🎬️demo-session`.
- generation2d's `📚️examples/🎬️demo` and `✏️editor/📚️examples/🎬️demo-session`.

Each runs through `bun test` directly (not vitest) against the shared shim at
`🧪️tests/🗿️artifact-runner/🟦️.ts` (adapts `vitest`'s `describe/it/expect` for `bun test`). There is
**no `vitest.config.ts` that globs `🌀️procedural`** anywhere in the repo — the root
`vitest.config.ts` deliberately collects nothing (ticket `26/08/23/END-TO-END-TESTING-REFACTOR`);
test discovery for this plugin is this explicit file list, not a glob.

**Finding (not fixed, out of `$T` scope):** `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/package.json`
is a stale copy of the CAD plugin's — its `description` reads "📐️ CAD plugin TS: ..." and its own
`scripts.test`/`scripts.generate`/`scripts.fixture` all read `"bun nx run @semio-tech/cad-js:test"`
etc. (wrong target). Harmless today because nx routes through `📋️project.json`'s own `targets.test`
(`bun ./📜️script.ts test`, package.json is not consulted), but it is copy-paste damage worth a
follow-up.

### 1b. Language-agnostic self-test discovery in root `📜️script.ts`

- `"procedural/3d/op"` and `"procedural/2d/op"` (line ~18655-18656): entries in
  `POLICY_DIFF_COMPLETENESS_ALLOWLIST` — a **policy exemption**, not a test to run. Means
  generation3d/2d's diff types don't yet carry a real `DiffCodec` impl and are allowlisted rather
  than failing the diff-completeness policy.
- `"procedural/generation2d/standards#1-subsets-any-analyzer-component"` and
  `"procedural/generation3d/standards#1-subsets-any-analyzer-component"` (line ~28160-28161):
  entries in `POLICY_SNIFF_REALITY_ALLOWLIST` — both subsets' format-sniff functions are
  allowlisted as not-yet-real (constant-confidence sniff), symmetrically for 2d and 3d.
- `proceduralGenerationRootSelfTests()` — a real, runnable, cargo-free TS self-test at
  `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🧪️tests/🔬️procedural-generation-root/🟦️.ts`.
  It validates the shared `GenerationPlayState`/`GenerationRootV1` fixture against its JSON Schema
  (via `ajv`, with 3 hostile-input rejections), asserts a >16KB independent-JSON-oracle round-trip,
  and then does **text-level exactness checks** on `🧬️schema/📸️snapshot/🦀️.rs` for BOTH
  generation3d and generation2d (`modelPath.replace("🧊️generation3d", "🌀️generation2d")`) plus 7
  hostile-source-mutation rejections. It is aggregated into `toolJobCoverageSelfTests()`
  (`🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts`), which is executed by:

  ```
  bun ./📜️script.ts verify interactivity tool-jobs
  ```

  Full output: `🗑️generated/lane1b-verify-interactivity-tool-jobs.txt`. **Result: FAILS (exit 1)
  before `proceduralGenerationRootSelfTests()` is ever reached.** `toolJobCoverageSelfTests()`
  computes `const scalarConfig = toolJobScalarConfigCohortSelfTests();` *before* the return-line
  that calls `proceduralGenerationRootSelfTests()`, and that earlier call throws first:

  ```
  error: scalar Config route disposition generation2d/addGeneration
      at toolJobScalarConfigCohortSelfTests (…/🔬️tool-job-scalar-config-cohort/🟦️.ts:27:102)
      at toolJobCoverageSelfTests (…/🔬️tool-job-coverage/🟦️.ts:1852:24)
  ```

  i.e. a route-disposition mismatch between `🧵️retained-command/🧫️fixtures/🎚️scalar-config-cohort.json`
  and generation2d's editor config source for the `addGeneration` route — **in generation2d, not
  generation3d**, but it aborts the whole `toolJobCoverageSelfTests()` aggregate before
  `proceduralGenerationRootSelfTests` runs, so **generation3d's root self-test could not be
  exercised via this gate at all**. `git log --date=iso` on generation2d's
  `✏️editor/🎚️config` shows its most recent touch at `599a5d8450` (2026-09-09 07:58:58, same day,
  pre-HEAD) — consistent with a concurrent session mid-edit on that route (this repo has other
  live sessions per the ticket's own Fleet note); this audit does not assert root cause, only the
  observed failure and its effect on reachability of the 3d self-test.

Neither `"procedural/3d/op"` nor `"procedural/generation3d/standards#1-subsets-any-analyzer-component"`
is itself invoked as a standalone verb — they are string keys inside allowlists read by
`policyDiffCompletenessBreaches`/`policySniffRealityBreaches`, both folded into the general
`policy` gate, which is repo-wide and was not run in full here (would need to scope-filter output,
and is a general policy gate, not a generation3d-specific test lane).

## 2. Python second implementation

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧊️mutate-procedural-3d-1/`
holds exactly one case: `🐍️.py` (383 lines) + `🥒️.feature` + `🦀️.rs`.

- `🐍️.py` is an **independent second implementation** of `s.procedural.generation3d` and all 14
  typed mutations (`create-widget`, `update-widget`, `delete-widget`, `connect-synapse`,
  `update-synapse`, `disconnect-synapse`, `move-widget`, `delete-widget-position`, `update-camera`,
  `change-schema`, `create-generation`, `delete-generation`, `rename-generation`,
  `change-generation-value`), registered as an `Adapter("python")` oracle (never as a subject) for
  `mutate-<kind>`, `inverse-<kind>` and `identity-round-trip` scenario ids.
- `🥒️.feature` is the Gherkin driving it: 3 scenario groups × the 14 kinds (`@id-mutate`,
  `@id-inverse`) plus one `@id-identity-round-trip` scenario — 29 scenario instances total, tagged
  `@oracle-procedural-3d-python-independent`.
- Ran: `uv run python3 -c "import semio_repo_test"` →
  **`ModuleNotFoundError: No module named 'semio_repo_test'`** (full output
  `🗑️generated/lane2-python-import-check.txt`). `semio_repo_test` is not a pip-installed nor a
  vendored pure-Python package — it is exposed by the Rust crate
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust` (builds
  `libsemio_repo_test_host` under `target/debug/`, a PyO3 extension). **This lane is BLOCKED on a
  cargo build that is out of this agent's scope** (sibling agent owns cargo) — it cannot be
  exercised end-to-end from here.
- Ran as a fallback: `uv run python3 -m py_compile 🐍️.py` → **exit 0, clean** (full output
  `🗑️generated/lane2-python-syntax-check.txt`). So the Python source itself is syntactically valid;
  only the harness binding is unavailable non-cargo.
- The `🥒️.feature`'s own doc-comment is explicit that `parity` (subject-vs-oracle differential) was
  **not** measured in the most recent pass either, killed by the runner's 900s per-case budget on
  cargo target-dir lock contention (`spawnSync cargo ETIMEDOUT`) — a pre-existing, already-documented
  gap, not something introduced by this audit.

## 3. Mutation fixtures — `🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/*`

All **14** mutation folders present, each with exactly **1** case, each case complete
(`🦠️mutation/🔣️.json`, `📸️snapshot/⬅️before+➡️after/🔣️.json`, `🔺️diff/🔣️.json`,
`🎯️outcome/🔣️.json` — 5 files × 14 = **70/70 files present**, verified file-by-file, not just
counted):

`change-schema`, `disconnect-synapse`, `update-widget`, `connect-synapse`, `move-widget`,
`rename-generation`, `delete-generation`, `delete-widget`, `update-synapse`, `update-camera`,
`create-generation`, `change-generation-value`, `create-widget`, `delete-widget-position`.

Cross-checked against `🧬️schema/🧬️mutations/*` (the schema side): same 14 directories (schema dir
also contains `💾️binary`/`📝️text`, which are snapshot serialization-format specs, not mutations —
excluded from the count) and against `🔮️oracle/🔣️.json`'s `mutationCatalogs`/`mutationManifests`,
which list the same 14 `kinds` 1:1 with `oracleRequirements.qualifyingKind: "verified-native-second-implementation"`
on every one. **No missing, no orphaned, no stale-count mutation.**

**Naming drift vs. `🌀️generation2d` — confirmed intentional, not a bug.** 2d's 14 mutations are
named `disconnect-synapse, create-generation, delete-generation, create-widget, set-camera,
rename-generation, move-widget, replace-widget, replace-synapse, connect-synapse,
change-generation-value, change-schema, delete-widget, clear-widget-layout` — three verbs differ by
name only: 3d's `update-widget`/`update-synapse`/`delete-widget-position` vs. 2d's
`replace-widget`/`replace-synapse`/`clear-widget-layout`. This is documented, not accidental: both
the `🥒️.feature` doc-comment and the `🐍️.py` module docstring state the two subsets are "ONE
implementation instantiated twice ... differ only in three kind names and four argument names," and
name two real semantic divergences the pairing surfaced (3d's `delete-widget` raises no
`mutation.cascade` where 2d's does; 2d's `question_id` is the one snake_case field in either model).

**Staleness check:** `git log -1 --date=iso` on both `🧊️generation3d/…/🧫️fixtures/🧬️mutations` and
`🌀️generation2d/…/🧫️fixtures/🧬️mutations` returns the **same** commit (`9b605a4550`,
2026-09-09 12:10:01, i.e. HEAD) — both directories were last touched together. Content-level
freshness (i.e., whether 3d's committed vectors match what the current Rust codec would actually
produce) cannot be verified without compiling the subject crate, which is out of this agent's scope
(cargo). No mtime-based evidence of 3d fixtures lagging 2d.

## 4. Third-party oracle registration

`🔮️oracle/🔣️.json` for generation3d registers **one** oracle:

- `id: "procedural-3d-python-independent"`, `ecosystem: "python"`, `package: ""` (deliberately
  empty — no distribution needed), `kind: "verified-native-second-implementation"`,
  `testOnly: true`, `productionReachable: false`, `networkDuringExecution: false`. The rationale
  field (verbatim in the JSON) documents a declined third-party survey: "no node-graph format
  models a graph whose layout is a SPARSE side table keyed by node id and whose document's second
  half is an unrelated parameter history, and none of them reads this carrier" — `ecosystemsSearched:
  ["python/pypi"]`, one candidate considered and rejected (`"SPARSE"`).
- `p8yz-b-third-party-oracle-laws.json` fixture: registers `oracle.library: "serde_json"`,
  `scope: "test-only-existing-dependency"`, `runtimeDependency: false` for a small feature-level
  law (`move-one-mounted-generation3d-widget`) — i.e. `serde_json` is used as a lightweight
  structural JSON oracle for one law-test, not as the mutation-vocabulary oracle (that's the Python
  second implementation above).
- `p8yz-b-owner-catalog-laws.json` / `p8yz-b-retained-mounted-laws.json`: not oracle registrations —
  ownership/retained-job law fixtures (`snapshotOwners`, `mutationOwners`, retained-job
  lifecycle/publication vocabulary), scoped `P3D3` discriminator, `forbiddenDiscriminators: ["P2D2"]`.

Ran `bun ./📜️script.ts verify dependencies literal-external` (the exact gate named
`⚖️gate📦️dependencies0️⃣` in the root script's own gate list) — **full output
`🗑️generated/lane4-verify-dependencies.txt`. FAILS repo-wide**:

```
error: [verify dependencies literal-external] target=0, current=193, oracle-conflicts=23,
toolchain-owner-conflicts=2, toolchain-failures=0.
```

`rust:serde_json` is one of the 23 `oracle-conflict` entries, declared by **~80+** Cargo.toml files
across the whole repo, generation3d's and generation2d's `📦️packages/🦀️rust/Cargo.toml` among
them (both listed side by side) — this is a **repo-wide** oracle-registration conflict on a
near-ubiquitous crate, not a procedural-specific or 3d-vs-2d-specific failure. No `rust:` conflict
entry names a procedural-only or generation3d-only crate.

## 5. E2E/browser harness

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/*` and root `🧪️tests/*`: grepped for
  `procedural`/`generation3d`. **One** hit —
  `🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts` references
  `"semio-s-plugin-procedural"` only as a test-fixture *string* inside a generic
  `pluginCargoArgs(...)` unit test (verifying the CLI-args builder shape for wasm-release builds
  across several plugin names) — **not** a procedural3d-specific or runtime-behavior test.
  `🧪️tests/🗿️artifact-runner/🟦️.ts` is the generic `describe/it/expect` shim consumed by the
  example tests in §1 — no procedural-specific content of its own.
- No playwright config or `.spec.ts` anywhere in the repo references `procedural` or `generation3d`.
- **No dedicated procedural3d runtime/browser verification probe exists today.**
- Template studied: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔍️verify-wave1-puzzle3d-oracle.ts`
  (and its 4 siblings in that ticket folder). These are plain `bun run <file>.ts` scripts — **not**
  actual app/browser drivers — that read the owning app's committed Rust source as **text**
  (`Bun.file(...).text()`), then regex-extract retained-tool-id arrays, `.action_interactive_job(...)`
  pairs, and `ArtifactToolPublicationContract{...}` blocks, and diff them exactly against a JSON
  fixture manifest (`🧪️publication-authority/🔣️.json`) describing the expected owner/routes/lanes.
  A generation3d-equivalent probe would need: (a) the owning app/window source for generation3d's
  editor (`✏️editor/…/🎮️commands/*`, `🎭️modes/*`, `📌️panels/*`) as the text-scan target, (b) a
  `🔣️.json` fixture enumerating expected retained tool-job routes/publication lanes for
  generation3d's windows (preview, flow, form, generations), and (c) the same exact-match regex
  approach used by `verify-wave1-puzzle3d-oracle.ts` — none of this currently exists for procedural.
- The ticket goal ("boots, examples switch, hover/selection, editor actions dispatch at runtime")
  is a *live-app* claim that none of the lanes audited here actually exercise — every lane in §1-§4
  is static (schema/text/JSON), never launches react/wasm/wgpu. This is a real coverage gap for the
  ticket's stated goal, not a lane failure — flagged, not run (dev servers are out of this agent's
  scope).

## 6. Summary table

| Lane | Command | Discovered | Passed | Failed | Blocked-by |
|---|---|---|---|---|---|
| TS/bun examples | `bun nx run @semio-tech/procedural-js:test --skip-nx-cache` | 11 (8×3d examples + 1×3d editor demo + 2×2d) | 11 | 0 | — |
| Root self-test (procedural generation root) | `bun ./📜️script.ts verify interactivity tool-jobs` | 1 self-test fn (`proceduralGenerationRootSelfTests`, folded into `toolJobCoverageSelfTests`) | 0 | not reached | unrelated earlier self-test fails first: `generation2d/addGeneration` scalar-config route-disposition mismatch aborts the whole aggregate before 3d's self-test runs |
| Policy allowlists (diff-completeness, sniff-reality) | n/a — static allowlist entries, not a runnable lane | 2 entries (3d) / 2 (2d, symmetric) | n/a | n/a | not independently runnable; folded into general `policy` gate (not run, repo-wide) |
| Python second implementation | `uv run python3 -c "import semio_repo_test"` | 1 module, 14-kind coverage, 29 Gherkin scenarios | 0 | 1 (import) | **cargo** (`semio_repo_test_host` PyO3 ext not built) |
| Python syntax-only | `uv run python3 -m py_compile 🐍️.py` | 1 file | 1 | 0 | — |
| Mutation fixtures | file audit (no test runner) | 14 mutations × 5 files = 70 | 70 | 0 | — |
| Oracle registration audit | file audit + `bun ./📜️script.ts verify dependencies literal-external` | 1 procedural-3d oracle + 1 small-feature law | n/a (audit) | gate itself fails | repo-wide (193 literal-external, 23 oracle-conflicts, incl. shared `serde_json` — not 3d-specific) |
| E2E/browser | grep across `🧪️tests/`, dev tests, playwright/vitest configs | 0 procedural3d-specific | — | — | does not exist; puzzle3d ticket's static text-scan probes are the template, none written for generation3d |

Report: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📓️test-harness-audit-2026-09-09.md`
