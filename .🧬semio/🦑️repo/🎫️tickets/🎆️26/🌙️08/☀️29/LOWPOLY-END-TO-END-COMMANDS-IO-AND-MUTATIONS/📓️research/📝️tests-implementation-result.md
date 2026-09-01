# Lowpoly Tests Implementation Result

Ticket `26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS`. Scope: add `io-lowpoly-1` and
`command-lowpoly-1` test cases, discharge (or honestly fail to discharge, with reasons) the
third-party-oracle requirement, without touching any file outside this pass's exclusive ownership.

## 1. Verified discovery convention

Confirmed by reading `discoverTestCases`/`testFeatureFilename`/`testAdapterFilenames`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts:125-172,518-565`)
against `🔣️taxonomy.json`: a case is discovered at `<owner>/🧪️tests/<slug>/` iff it holds
`🥒️.feature` (`gherkin-feature` kind, emoji `🥒️` + `.feature`) — adapters (`🦀️.rs`, `🐍️.py`, …) are
optional but at least one is required by the CONTRACT phase, not discovery itself. `owner` is
literally "the directory that holds `🧪️tests`" — for lowpoly that is the ARTIFACT root
(`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly`), matching `mutate-lowpoly-1`'s existing location.
This matched the existing files on disk exactly (`🧪️tests/mutate-lowpoly-1/{🥒️.feature,🦀️.rs,🐍️.py}`)
— no filename drift.

**Before**: `bun ./📜️script.ts test discover` → `[discover] 169 test case(s)`, one lowpoly row
(`mutate-lowpoly-1`).
**After**: same command → `[discover] 171 test case(s)`, three lowpoly rows:
```
✏️s/…/🧪️tests/command-lowpoly-1  [rust]
✏️s/…/🧪️tests/io-lowpoly-1       [rust]
✏️s/…/🧪️tests/mutate-lowpoly-1   [rust,python]
```
`bun ./📜️script.ts test discover --json` cross-checked `owner`, `adapters`, `localFixtureDir` (only
`io-lowpoly-1` has one, correctly) for both new cases. `--filter` is not a real flag (checked
`readSelectors`/`selectCases` in the same file) — the real selectors are `--owner`/`--case`/
`--project`; `test discover` itself takes no selector at all (prints everything). This is stated
because the ticket brief's suggested command differs from what the CLI actually implements.

## 2. New scenarios added

### `io-lowpoly-1` (9 scenarios: `roundtrip-{dwg,gltf,json,las,obj,ply,png,stl,txt}`)
Round-trips ONE committed `LowpolySnapshot` fixture (handcrafted,
`🧪️tests/io-lowpoly-1/🧫️fixtures/lowpoly-snapshot.json`, two objects — one with a mesh child handle
and a paint layer, one bare) through `serialize_bytes`/`deserialize_bytes` for every
`stdio.*` format `import_stdio_kinds()`/`export_stdio_kinds()` declares
(`✳️any/🚪️io/🦀️component.rs`), asserting the re-imported document equals the original member for
member.

**Real state discovered while writing this (not what the ticket brief assumed):** reading every
export/import leaf under `✳️any/🚪️io/📤️export|📥️import/**` found **5 of 9 formats genuinely work**
(`obj`, `ply`, `json`, `png`, and — as of partway through this pass — `txt`, finished by a concurrent
agent mid-session) and **4 are honest, permanent stubs** (`dwg`, `gltf`, `las`, `stl`) that
unconditionally `Err(...)`, each doc comment naming the SAME root cause: `LowpolyObject.mesh` is a
content-addressed HANDLE, never embedded geometry, so no synchronous `&LowpolySnapshot -> …` function
can produce real vertices — an architecture limit, not a stub someone forgot. The working formats are
not real interop either: `obj`/`ply`/`png`/`txt`/`json` all carry the FULL document losslessly via
lowpoly's own `.lowpoly` DSL text (hex-embedded in an OBJ unknown-statement / PLY comment / PNG `tEXt`
chunk, or verbatim as the TXT body, or as plain JSON) — geometry-empty by design. This is stated
explicitly in the feature file so nobody mistakes 5/9 green for "IO works."

### `command-lowpoly-1` (14 scenarios: `command-{13 groups}` + `catalog-size`)
See §4 — scope was reduced from "dispatch and assert the mutation" to "construct the representative
payload and assert `command_id()`/`TOOL_JOB_IDS` membership" for a structural reason discovered mid-pass,
documented in full in the feature file's own "REDUCED SCOPE" section and in `noOracleDecisions` in
oracle.json. The 13 representative commands (one per group, exact payload mirrors the crate's own
`every_command()` test): `patchObject`, `addPrimitive`, `setSunAzimuth`, `setCamera`,
`toggleShowEdges`, `engagementInput`, `setFixtureJson`, `toggleSmooth`, `addPaintLayer`,
`setActiveObject`, `setUtilityParam`, `transformEnd`, `unwrapActive`. Also corrected an off-by-one in
this ticket's own `📝️editor-commands.md` (headline "48 commands" vs. the macro's actual 47 rows,
matching the crate's own `command_ids_are_unique` assertion of 47).

## 3. Third-party oracle outcome

**Rust side (io-lowpoly-1): structurally blocked, not merely absent — recorded, not silently
skipped.** `tobj`/`ply-rs`/`stl_io` already exist, vendored and wired, in the SIBLING `🗄️stdio`
plugin's `🧪️oracle/📦️packages/🦀️rust` crate (`semio-s-plugin-stdio-test-oracle`, `oracles` feature).
Reaching them needs an `oracleHostPackages` entry (`OracleHostPackage` in the test framework's schema)
declared in a contribution file at an ANCESTOR path of this case's owner —
`oracleHostPackagesFor(registry, owner, implementation)` matches by exact owner or ancestor-prefix
only (`📦️index.ts:928-931`), and this pass's owned oracle file (`✳️any/🧪️oracle/🔣️.json`) is a
DESCENDANT of the owner (`🗿️artifacts/💠️lowpoly`), the wrong direction. A new file at
`✏️s/🔌️plugins/💠️lowpoly/🧪️oracle/🔣️.json` (plugin root) or
`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🧪️oracle/🔣️.json` (artifact root) would fix it — outside
this pass's granted ownership, so NOT created; recorded as a handoff (§5). Confirmed by grep: no test
case anywhere in `✏️s` currently links a foreign-plugin oracle crate this way except within `🗄️stdio`
itself, so this is a real, narrow gap, not a pattern this pass should improvise around.
Even with that fixed, `tobj`/`stl_io`/`ply-rs` would only prove the container is well-formed GRAMMAR
(0 vertices, 0 faces, by design) — never real geometry fidelity, because there is none to check.

**Python side: a genuine finding, found late.** `pyproject.toml`'s dependency groups do NOT list
`trimesh` or `Pillow`. But the ACTUAL resolved environment does carry `Pillow` (12.2.0) — confirmed
`python3 -c "from PIL import Image"` succeeds — almost certainly a transitive dependency of
`matplotlib` (already declared in the `dev` group). `numpy` (2.5.0 declared / 2.0.2 resolved) is
similarly present. `trimesh` is genuinely absent (`ModuleNotFoundError`). This means a REAL,
zero-new-dependency third-party PNG oracle (Pillow decoding the PNG bytes our own `png` exporter
writes, independent of our own `png` crate usage) is achievable without touching `pyproject.toml` at
all. **Not wired into a passing scenario in this pass**: doing so correctly needs splitting `png` out
of the single no-oracle-tagged `io-lowpoly-1` feature into its own `@oracle-`-tagged scenario (the
oracle/no-oracle tag is FEATURE-level, not scenario-level, so the two cannot coexist in one feature),
verified against the SAME contract gate that is currently unreachable (cargo-blocked, see §4) — under
the coordinator's explicit direction to stop blocking and report, this was left as a concrete,
ready-to-implement follow-up rather than risking an unverified, possibly contract-breaking change.
Recorded as a handoff item (§5) with the exact Pillow finding so the next pass does not have to
re-derive it.

`oracle.json` (`✳️any/🧪️oracle/🔣️.json`) now records BOTH new capabilities honestly:
- `lowpoly-io-native-round-trip` (no-oracle, substitute `metamorphic-laws`) — full rationale above,
  written in the file itself.
- `lowpoly-command-catalog-shape` (no-oracle, substitute `specification-vectors`) — see §4.
The existing `lowpoly-python-independent` oracle entry for `mutate-lowpoly-1` (17 mutation kinds) was
**not touched** — still `cross-semio-implementation`, still short of `third-party-library`, exactly as
it was found.

## 4. The command-dispatch blocker (why command-lowpoly-1's scope shrank)

Confirmed structurally, not assumed: `semio_framework_plugin::app_commands!` generates
`LowpolyCommand::dispatch(&self, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_,
LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<..>, Fault>`, and every per-command
`handle()` takes the identical `ArtifactView`/`ConfigView` pair. Both types live in
`semio_framework_plugin`. `materializeRustHost` (`🧰️framework/…/🧪️test/📜️script.ts:415-470`) links
exactly THREE things into a generated test host: `semio-repo-test-host` (dependency-free by design),
the case owner's own `sut` crate (found by walking UP for `📦️packages/🦀️rust/Cargo.toml` —
`semio-s-plugin-lowpoly` for every lowpoly case), and `contributedOraclePackages` from an
ancestor-scoped `oracleHostPackages` array — lowpoly declares none, and (§3) I cannot add one from
inside my ownership. Ground-truthed against an ALREADY-BUILT generated host on disk
(`.🧬semio/🦑️repo/⚡️cache/tests/hosts/…create-and-round-trip-png-oracle-rust/Cargo.toml`): exactly
`semio-repo-test-host` + `semio-s-plugin-stdio-test-oracle` (stdio's OWN plugin-root oracle
contribution) + `semio-s-plugin-stdio` (sut). No `semio-framework-plugin` anywhere, for any plugin —
confirmed repo-wide with `grep -rl "ArtifactView\|ConfigView" --include="🦀️.rs" ✏️s | grep 🧪️tests/`
returning nothing. `protocol::Mutation::diff/apply` (needed to observe a mutation's effect) is gated
behind the same kind of missing crate (`protocol`). Rust's trait-method resolution requires the trait
in scope, and neither crate is nameable from a generated host — this is not fixable by writing
different Rust, only by adding the missing linkage (outside this pass's files) or by lowpoly's own
crate re-exporting the types publicly (also outside this pass's files — `✏️editor/**` is off limits).
What IS reachable without any extra crate: `app_commands!` also emits `TOOL_JOB_IDS`/`command_id()` as
plain INHERENT items (not trait methods), and every payload struct is public — hence the reduced-scope
test in §2.

## 5. Verification — what ran, what didn't, and why (read before trusting any PASS claim)

**Environment at verification time** (reported by the coordinator, independently confirmed):
`uptime` showed load average 69/55/57 on this machine, `ps aux | grep cargo` counted ~69 concurrent
cargo processes, and the root cargo workspace is RED for reasons entirely outside this ticket — a
separate concurrent session is mid-refactor on `semio-s-plugin-stdio` (~329 errors), which lowpoly
depends on. Two `bun ./📜️script.ts test contract --case …` runs and one plain `cargo check -p
semio-s-plugin-lowpoly --lib` were started and were still running or were killed by the environment
after 5+ minutes without producing output; a separate `bun ./📜️script.ts test run --case
create-and-round-trip-obj --implementation rust` probe (run earlier, unrelated to my case, to
ground-truth the host-generation mechanism in §4) failed with `EINTR`/`scandir` from
`surveyUnmanagedTests`'s full-repo walk — a transient OS-level interrupted-syscall error from
concurrent filesystem contention, not a defect in anything I wrote.

**What DID run and pass, with real output:**
- `bun ./📜️script.ts test discover` (no cargo, no repo-wide contract walk) — 169 → 171, confirmed
  twice, exact rows shown in §1.
- `bun ./📜️script.ts test discover --json` — cross-checked owner/adapters/fixture-dir shape for both
  new cases, exact output in §1.
- `python3 -c "from PIL import Image"` / `import numpy` / `import trimesh` — exit codes and tracebacks
  captured directly, §3.
- Static, line-by-line reads of every `serialize_bytes`/`deserialize_bytes` leaf under
  `✳️any/🚪️io/**`, the `app_commands!` macro definition, `ArtifactView`/`ConfigView`/`Emit`
  definitions, and `materializeRustHost`/`oracleHostPackagesFor` — this is how §3/§4's findings were
  derived; they are read from the actual source, not inferred.

**What did NOT run, honestly, because the cargo workspace is unreachable right now:**
- `bun ./📜️script.ts test contract --case io-lowpoly-1` — started, no output after 5+ min, killed.
- `bun ./📜️script.ts test contract --case command-lowpoly-1` — same.
- `bun ./📜️script.ts test --case io-lowpoly-1` / `--case command-lowpoly-1` / `--case mutate-lowpoly-1`
  — not attempted after the two contract runs stalled; would hit the same `semio-s-plugin-stdio`
  compile wall (`io-lowpoly-1` and `mutate-lowpoly-1`'s subject role both transitively need
  `semio-s-plugin-lowpoly`, which depends on `semio-s-plugin-stdio`).
- `export DEVELOPER_DIR=/Library/Developer/CommandLineTools && cargo test -p semio-s-plugin-lowpoly
  --lib` — not run for the same reason (`cargo check -p semio-s-plugin-lowpoly --lib` alone was
  started and killed without output).
- Consequently: `mutate-lowpoly-1` was NOT re-verified to still fully pass in this pass. I did not
  touch any file it owns or depends on (only added two new sibling test-case directories and appended
  to `noOracleDecisions` in the shared `🔣️.json`, never touching its own `oracles` entry or the 85
  mutation fixtures), so there is no code-level reason for a regression, but that is an argument from
  non-interference, not a runtime confirmation — flagged per this ticket's own "must not say a test is
  passing when you didn't run it" rule.
- Every one of io-lowpoly-1's 9 scenarios and command-lowpoly-1's 14 scenarios is therefore
  **written and contract-plausible by hand-verification of the taxonomy rules (§1), but UNRUN** —
  none has been executed end to end. The 4/9 io-lowpoly-1 rows expected to fail (`dwg`/`gltf`/`las`/
  `stl`, §2) are expected to fail on architecture grounds proven by reading the stub source directly,
  not by having watched them fail.

## 6. Handoff items

1. **Rust command-dispatch testability (repo-wide, not lowpoly-specific).** No plugin in this
   repository can currently unit-test `handle(payload, doc, cfg, ctx)` or `LowpolyCommand::dispatch`
   from a generated test host, because `semio_framework_plugin`/`protocol` are not linkable. Fix
   either by (a) the owning editor agent adding `pub use semio_framework_plugin::{ArtifactView,
   ConfigView, Emit, Fault, HistoryView};` to `✏️editor/🦀️component.rs` (a ~5-line, additive,
   non-breaking change — `✏️editor/**` is off limits to this pass), or (b) registering
   `semio-framework-plugin` and `protocol` as `oracleHostPackages` in a new ancestor-scoped
   `🧪️oracle/🔣️.json` (plugin or artifact root — outside this pass's owned files). Either unblocks a
   real `command-lowpoly-1` dispatch test AND fixes the same gap for every other plugin.
2. **Rust third-party mesh oracle for `obj`/`ply`/`stl`.** `tobj`/`ply-rs`/`stl_io` already exist in
   `semio-s-plugin-stdio-test-oracle`. A new ancestor-scoped `oracleHostPackages` entry (same file as
   #1, cheaper) would let `io-lowpoly-1` link it — though it can only validate container well-formedness,
   never geometry fidelity (§3).
3. **Python third-party PNG oracle, ready to write.** `Pillow` is genuinely resolvable in this repo's
   Python environment today (transitively via `matplotlib`), no `pyproject.toml` change needed. Split
   `roundtrip-png` out of `io-lowpoly-1` into its own `@oracle-`-tagged feature/scenario and add a
   `🐍️.py` adapter using `PIL.Image.open` to independently decode our exported PNG bytes.
4. **This whole pass's Rust-side scenarios are unrun.** Re-run once `semio-s-plugin-stdio` compiles
   again: `bun ./📜️script.ts test contract --case io-lowpoly-1`, same for `command-lowpoly-1`, `bun
   ./📜️script.ts test run --case io-lowpoly-1 --implementation rust` / `--case command-lowpoly-1`, `bun
   ./📜️script.ts test run --case mutate-lowpoly-1` (regression), and `cargo test -p
   semio-s-plugin-lowpoly --lib`.
5. **`📝️editor-commands.md`'s command count (48) is off by one** against the macro's own 47 rows and
   the crate's own `command_ids_are_unique` assertion — corrected in `command-lowpoly-1`'s feature file
   but the research report itself was not edited (outside this pass's remit).

## 7. Files touched (all inside this pass's exclusive ownership)

- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🧪️tests/io-lowpoly-1/🥒️.feature` (new)
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🧪️tests/io-lowpoly-1/🦀️.rs` (new)
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🧪️tests/io-lowpoly-1/🧫️fixtures/lowpoly-snapshot.json` (new)
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🧪️tests/command-lowpoly-1/🥒️.feature` (new)
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🧪️tests/command-lowpoly-1/🦀️.rs` (new)
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` (edited
  — appended two `noOracleDecisions` entries; existing `oracles`/`mutationCatalogs` content unchanged)

`mutate-lowpoly-1` was not modified in any way.
