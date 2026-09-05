# 💠️ Lowpoly end to end — runtime round (status)

Ticket reopened on disk (repo MCP failed to connect this session: `repo (-32602) invalid initialize params`).
Round start commit: `0a908f6c661c75b0bba19fc99334edcfb064171c` (2026-09-04 01:18:32 +0200).
Session date: 2026-09-05.

## What this round is for

The 08/29 round of this ticket closed the *static* surface: 47 commands classified, IO honesty,
schema parity, tests written. It explicitly ended with **every Rust test WRITTEN BUT UNRUN** because
`semio-s-plugin-stdio` (a hard dependency of lowpoly's io layer) had 2196 errors from a peer's
in-flight refactor, so `semio-s-plugin-lowpoly` was never compiled.

This round is the runtime half, matching the definition of done used by the sibling e2e tickets
(`PROCEDURAL-3D-END-TO-END`, `PUZZLE-3D-END-TO-END`, `CAD-END-TO-END`):

1. `cargo check -p semio-s-plugin-lowpoly --lib` green (native)
2. `cargo check -p semio-s-plugin-lowpoly --target wasm32-wasip2` green
3. `cargo clippy -p semio-s-plugin-lowpoly --all-targets -- -D warnings` green
4. `cargo test -p semio-s-plugin-lowpoly --lib` green (baseline was 137 lib tests)
5. `bun ./📜️script.ts test discover` finds the lowpoly cases and they pass
6. **`bun run dev:lowpoly` boots and is observed in a browser**: every window renders non-empty
   content, the default example loads, examples are switchable, and at least one editing command
   is confirmed to dispatch (console-log evidence, not inference)

Nothing in 1-6 is claimed until it is observed. Prior rounds' pass claims are not inherited.

## Machine conditions at round start

Load average 203 (1-min), ~55 MB free RAM, 38 cargo/rustc processes, 45 concurrent agent sessions.
A peer's emoji-uniqueness rename wave (`ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY`) is live
inside the lowpoly tree right now. Per the standing instruction, this round works alongside it.

### Rename-wave false alarm (checked, cleared)

`git status --porcelain` showed what looked like content rotation across io export format dirs
(`ply -> txt`, `txt -> stl`, `stl -> obj`, `obj -> ply` for the `🟦️.ts` serializers). It is not
corruption: all nine export `🟦️.ts` files are byte-identical 1-line stubs (md5
`e2ebd7ddedcadeeadbf819c35985c768`), so git's rename heuristic paired them arbitrarily. The `🦀️.rs`
serializers paired correctly. No repair needed.

Separately worth noting for the io audit: the TypeScript export serializers are all empty stubs; the
real io implementation is Rust only.

## Open

Everything. Build is queued behind a peer's cargo build-directory lock.

## Static picture at this round's start — verified by the coordinator, not inherited

Four read-only explorers ran (`📓️e2e-explore-boot-chain.md`, `-dispatch-factory.md`,
`-examples-render.md`, `-build-blockers.md`). Their raw conclusions are in those files; the
coordinator re-verified the load-bearing claims directly, and **two of them were over-called**.

### Boot chain (confirmed)

`bun run dev:lowpoly` → root `📜️script.ts dev lowpoly` → nx `@semio-tech/framework-os-dev:dev lowpoly`
→ Vite React dev server on **port 6078**. Lowpoly declares ONE app, `s.lowpoly.lowpoly@1/*#editor`
(no app-id argument, unlike `dev:procedural:3d`), plus a viewer app. Modes/windows:

| Mode | Window | SurfaceKind | Share |
|---|---|---|---|
| `edit` | Model | World3d | 100% |
| `paint` | Model | World3d | 60% |
| `paint` | UV | Canvas2d | 40% |

### Dispatch is LIVE (confirmed by coordinator)

Lowpoly does **not** have the gen3d/forms bare-factory defect. Editor `🦀️.rs:1613` declares
`factory_type: LowpolyCommandJobFactory`; `register_tool_job_factories` (`:1665`) and `build_tool_job`
(`:1670`) are overridden; 47 tool proofs (`:2140`); 0 `BatchOnlyPendingRewrite`. The 08/29 round's
app-owned factory work did land. **No dispatch work is needed this round.**

### Examples — explorer over-called it

The explorer reported "no example switching, examples are dead code". The literal fact is right
(`App { definition: create_lowpoly_app(), examples: Vec::new() }`, editor `🦀️.rs:2079`) but the
framing is wrong: `.example(…)`/`.workflow(…)` are DROPPED in **every** plugin in this repo
(trinity, remodel, raster, process3d, all 12 norm artifacts) under the
ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.4. Examples now come from the subset's own
`📚️examples/🎬️demo`. This is the repo-wide design, not a lowpoly regression. Multi-example
switching (as gen3d has) is an app-level opt-in lowpoly has never had; lowpoly ships one example.

### Mesh rendering — explorer over-called it

The explorer concluded a fresh boot shows "only a hardcoded placeholder" because the demo example
carries an empty mesh handle. The example payload does decode to exactly that —
`objects=[[obj-1, "Unit Box", [0,0,0,0,0,0,1,1,1], false, [], []]]`, empty mesh handle and no paint
layers — but that is **by design**, and the other half of the design is wired:

- `LowpolyScratch::default()` (session `🦀️.rs:274`) and `LowpolyTransientState::default()` (`:612`)
  both seed `mesh_workspace` from `schema::default_mesh_workspace()`.
- `schema/🦀️.rs:206` seeds that map with key `"obj-1"` — **the same object id the example declares**.
- Session `🦀️.rs:244-252` states the intent outright: seeded on `Default::default()` "so a freshly
  booted session can immediately reload the mesh `ArtifactApp::initial_snapshot()`
  (`default_snapshot()`) describes".

So the unit box is the intended boot content, not a placeholder fallback. The same doc comment is
honest about the genuine structural limit — the `mesh_workspace` cache can go stale against the
`ArtifactChild` handle across undo/redo of `create-mesh`/`delete-mesh`, and "a real fix needs
child-document resolution, which no WASM-guest plugin in this repo has yet". That is a
framework-wide unimplemented capability affecting every plugin, **out of scope** for this ticket;
`LowpolyDocument::reload_meshes` fails closed rather than computing wrong geometry, which is correct.

### Consequence for this round

The static surface is sound. Nothing above needs a code change. The entire remaining goal is
**compile → boot → observe**, and it is 100% build-gated.

## The build gate (the actual blocker)

`cargo check -p semio-s-plugin-lowpoly --lib` (PID 9392) has been stuck 15+ min on
`Blocking waiting for file lock on build directory`. `lsof target/debug/.cargo-lock` names the holder:
**PID 270, `cargo run -p semio-framework-os-mcp --bin semio-os-mcp -- stdio`** — a repo-MCP server
build. There are **five** such MCP `cargo run` processes queued (270, 1586, 5241, 9075, 22180), which
is also why this session's repo MCP failed to connect. Behind them: two `cargo check --workspace
--keep-going` runs, per-plugin checks for puzzle/block, and a `semio-s-plugin-stdio` wasm-release
build running for **2h42m**.

Machine: load avg peaked 275, 40-270 MB free RAM, 45 agent sessions. A private `CARGO_TARGET_DIR`
would escape the lock but forces a from-scratch dependency rebuild at ~60 MB free — two prior
sessions were OOM-killed doing exactly that. Decision: **stay queued on the shared target**, add no
build load, and spend the build slot once when it arrives.

## Build attempt 1 — `cargo check -p semio-s-plugin-lowpoly --lib` (03:33-04:12)

Queued 18 min on the shared build-directory lock, then compiled for ~20 min. Result:

- `semio-framework-plugin` (lib): **compiled**, 214 warnings, 0 errors.
- `semio-s-plugin-stdio`: **225 errors** — and the check terminates there.
- **Errors attributable to `✏️s/🔌️plugins/💠️lowpoly`: 0.** Every single error path is under
  `✏️s/🔌️plugins/🗄️stdio`. Lowpoly is never reached, so this round still has **no compile signal
  on lowpoly itself** — same wall the 08/29 round hit, at 225 errors instead of 2196.

Two error families in stdio, both signatures of a live peer refactor:

1. `include_str!`/`#[path]` "couldn't read … No such file or directory" — the peer's emoji-rename
   wave renamed subset `✳️model` → `🏛️model`. **These specific errors were STALE**: by the time the
   check reported them the tree had already converged (stdio's Rust now references `🏛️model`; only
   one doc-comment mentions the old name). A lock-blocked cargo check compiles the source as of
   roughly when it acquires the lock, so its errors can describe a tree that no longer exists.
2. `E0277` trait bounds — the repo-wide serde → `ToValue`/`FromValue` migration, still in progress.

Peer liveness, settled by mtime rather than inference: stdio's newest `🦀️.rs` was written at
**04:02:19**, ~90 s before the check was inspected. A peer is actively rewriting stdio right now.
(Note: BSD `find -newermt "20 minutes ago"` silently returns nothing on this platform — it does not
parse relative strings. `stat -f %m` is the reliable way to ask this question here.)

### Can lowpoly be checked without stdio? No.

`📦️packages/🦀️rust/Cargo.toml` declares
`semio-s-plugin-stdio = { … default-features = false }` as an **unconditional** dependency — not
optional, not feature-gated. (`cad_plugin` is the only optional one, behind `cad-fixtures`.) So there
is no build configuration that yields lowpoly compile signal while stdio is red. The Rust half of
this ticket is hard-blocked on a peer, and no amount of lowpoly-side work changes that.

The check was killed at 04:12 once its verdict was established, because it held the shared lock that
this ticket's own dev boot was starved behind. Log: scratchpad `check-lowpoly-lib.txt`.

## Dev boot attempts (`bun run dev:lowpoly`, react renderer, port 6078)

| # | Outcome |
|---|---|
| 1 | Died in `ensurePluginRegistry` → plugin registry generation: `EINTR: interrupted system call, scandir` on a `📕️norm` fixture dir, thrown from `entries()` at `🔍️discovery/🟦️.ts:8754`. Transient — the peer's rename wave was churning the filesystem under the directory walk. Not a lowpoly defect. Worth noting as a robustness gap: that `readdirSync` is not retried on `EINTR`, which is survivable normally but not under 45 concurrent sessions. |
| 2 | Cleared the EINTR on retry — "plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 45 framework packages)", launch.json regenerated. Then queued on the cargo build-directory lock. In flight. |


## GREEN — re-verified this round by running, not inherited

### TypeScript / JS (`📓️e2e-typescript-verification.md`)

- `bun nx run @semio-tech/lowpoly-js:test --skip-nx-cache` → **exit 0**. Real assertions: the
  interactive-job route oracle cross-checks all 47 Rust action registrations, an Ajv schema
  round-trip, and 4 hostile-mutation rejections. It is the only test the package defines — narrow,
  but genuinely green.
- `bun ./📜️script.ts test discover` → **exit 0, 240 cases** repo-wide, with all four lowpoly ids
  present (`io-lowpoly-1`, `command-lowpoly-1`, `mutate-lowpoly-1`, `io-lowpoly-png-1`). The
  silent-zero/taxonomy-drift failure mode did not occur. (Was 172 at the 08/29 round; +68 repo-wide.)
- Typecheck: **no `typecheck`/`tsc` nx target exists anywhere in the repo.** A scoped `tsc -p` over
  lowpoly's TS tree reports **zero errors in lowpoly's own files**, and the check was proven
  non-vacuous by injecting `const x: number = "not-a-number"`, seeing tsc catch it at the exact
  file/line, then reverting (confirmed clean by `git diff`).
- Nothing needed fixing. Real `tsc` errors reachable from lowpoly are all outside it: lowpoly's own
  `📜️script.ts` uses Bun-only `import.meta.dir` with no `bun-types`/`@types/bun` installed anywhere
  (repo-wide tooling gap), plus shared framework files. Reported, not touched.

### Schema (`📓️e2e-schema-validation.md`, helper `🔬️validate-lowpoly-fixtures.ts` kept in this folder)

- **17 mutations, set-equal in both directions** across the Rust enum, the 17 directories on disk,
  and the json / proto / graphql / typescript catalogs — same order, same spelling, no drift.
- **85/85 fixtures with real ajv output** (ajv 8.20.0 from the repo's own node_modules, no new dep):
  68/68 real schema validations (17 mutation-envelope + 17 snapshot-before + 17 snapshot-after +
  17 diff) all pass; the remaining 17 outcome fixtures are valid JSON with no lowpoly-owned schema
  to check against (generic protocol `MutationOutcome`, outside the five-representation scope).
  Re-run twice, green both times.
- Both historical defects confirmed gone: no `meshJson`/`mesh_json` survives as a schema property
  anywhere, and `LowpolySelection` is fully defined in every representation including the diff
  family's `$defs`. A repo-wide scan for the same defect class (any `$ref` to an undefined `$defs`
  entry) across every non-fixture `🔣️.json` under the schema root found **zero**.
- Field parity checked beyond the brief: `LowpolyArtifact` (37 fields), `LowpolySnapshot` (2),
  `LowpolyDiff` (38 + nested delta/patch types) match across all five representations, nullability
  included. **Nothing needed fixing; no lowpoly file was modified.**

### The TS io serializer stubs are NOT a gap — question closed

The coordinator flagged that all nine lowpoly export serializers are byte-identical 1-line stubs.
Checked against cad, raster, remodel, gis (both artifacts) and puzzle (all three artifacts): **every
plugin's per-format TS serializers are `export {};` stubs** (lowpoly/raster/remodel/gis/puzzle
byte-identical `e2ebd7dd…`; cad differs only by a one-line JSDoc). Real serialization lives only in
the Rust siblings, repo-wide. This is the convention, not a lowpoly defect. No serializers written.

## Framework fix landed this round — EINTR in the discovery walk

`bun run dev:lowpoly` died **2/2** in `ensurePluginRegistry` → plugin-registry generation with
`EINTR: interrupted system call, scandir`, thrown from `entries()` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` — a **different directory
each time** (`📕️norm` once, `🧱️block` once), which is what rules out a taxonomy/data cause and
points at load. Signal delivery interrupts `scandir`/`lstat` when this many processes work the tree
at once, and the `node:fs` sync wrappers surface that as a throw instead of retrying. There was **no
EINTR handling anywhere in the library** before this.

Added `retryOnEintr` (`🟦️.ts:8532`) and applied it at the three call sites on that walk:
`readdirSync` in `entries` (`:8771`), `lstatSync` in `kind` (`:8753`), and `readdirSafe` (`:8545`).
Bundles clean (`bun build`, 196 modules).

The `readdirSafe` site is the important one: it caught **every** error and returned `[]`, so under
EINTR it silently truncated discovery instead of failing. A short catalog would have been reported
as a successful generate with a plausible count. That is strictly worse than the crash, and it is
the same silent-zero class of defect this repo has been bitten by before.

Reported to peer semio-f4, whose dev boots were dying in the same stage; they confirmed it is on
their boot path and that they had attributed the flakiness to taxonomy edits landing mid-walk.

**This did not fully unblock the boot**: the next attempt cleared discovery and then hit
`NX EINTR reading node_modules/nx/plugins/package-json.js` — the same environmental condition inside
nx's own reader, which is not ours to patch. A bounded retry loop is now driving the boot.

## Where this round stands at 05:06

### The stdio measurement, cross-examined and upheld

Peer semio-f4 challenged the 225 with three observations reading "0 E0277", then **retracted** it:
a crate that aborts at module expansion never type-checks, so its error count reads clean for the
wrong reason. Their test for telling the two apart is good, so it was applied to this round's own log
rather than assumed:

- stdio **warning** lines in the lowpoly check: **1335**.

rustc therefore genuinely type-checked stdio here. The 225 is a real result (160 E0277 mentions /
82 distinct headers, 33 stale `couldn't read`), not a false clean.

The competing "feature unification" hypothesis — that lowpoly enables stdio features the others do
not, so only lowpoly sees the migrated code — is **dead**:

```
cargo tree -e features -p semio-s-plugin-lowpoly -i semio-s-plugin-stdio
semio-s-plugin-stdio v0.1.0
└── semio-s-plugin-lowpoly v0.1.0
    └── semio-s-plugin-lowpoly feature "default" (command-line)
```

Lowpoly declares `default-features = false` and adds nothing back, so it enables the **minimal**
stdio surface — strictly less than procedural/process/cad, not more. There is no gate to decline.

What survives is the stronger reading: stdio has **two stacked problems** — a self-healing rename
race that aborts compiles early, and a genuinely in-flight serde → `ToValue`/`FromValue` migration
underneath it. You only see the second after surviving the first. Lowpoly is not specially cursed;
it merely got *further*. Peers should expect the same 225-class result one blocker later.

stdio's newest source was written **05:04:08** — still under active edit by its owner
(`26/04/08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY`). Not ours to repair.

### Verification scoreboard

| # | Gate | State |
|---|---|---|
| 1 | `cargo check -p semio-s-plugin-lowpoly --lib` | **BLOCKED** on stdio — 0 lowpoly errors, but lowpoly never compiled |
| 2 | `cargo check … --target wasm32-wasip2` | BLOCKED (same, plus shared `target/wasm32-wasip2` has been wiped) |
| 3 | `cargo clippy … -D warnings` | BLOCKED (same) |
| 4 | `cargo test … --lib` | BLOCKED (same) — every Rust test from the 08/29 round remains **written but unrun** |
| 5 | `test discover` + lowpoly TS suite | **GREEN, re-run this session** (240 cases; `@semio-tech/lowpoly-js:test` exit 0) |
| 5b | schema / ajv fixtures | **GREEN, re-run this session** (17 mutations set-equal, 85/85 fixtures) |
| 6 | `bun run dev:lowpoly` observed in a browser | **NOT ACHIEVED** — see below |

### Boot: honest status

Attempt 3 has run **40 minutes** and is still pre-Vite. It is alive and CPU-bound in JS (8-11% CPU
under load ~126, `sample` shows it running, not blocked on IO), past `ensurePluginRegistry` and
launch.json regeneration, with no `plugin-build-lowpoly.json` lease and no cargo child yet. It is
starved, not wedged.

The decisive structural point, from the dev script's own sequence
(`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1664-1763`):
`acquirePluginBuildLease` → `ensurePluginRegistry` → `buildEngineWasm` → Vite listens →
`buildPluginsStreaming`. The lowpoly plugin wasm is built in that **last** stage, and it links
`semio-s-plugin-stdio`. So even a successful Vite boot would serve a shell whose lowpoly plugin
fails to build. **A browser-verified lowpoly render is not reachable while stdio is red**, no matter
how long the boot is given. Saying otherwise would be inventing a result.

The boot is left running under a bounded retry loop on the chance stdio lands.

## The 225 was cascade, not a real migration — retracted and replaced

The round's own earlier conclusion ("stdio has two stacked problems, the E0277s are a genuine
serde → `ToValue`/`FromValue` migration in flight") is **wrong** and is retracted here.

What settled it, in order:

1. The 225 is not 225 problems. It is **150** × `SemioModelMutation: Mutation<SemioModelSnapshot>`
   + **10** × `<Leaf>: MutationLeaf` (InsertElement, RemoveElement, SetElement, InsertRelation,
   RemoveRelation, SetRelation, InsertSpatialNode, RemoveSpatialNode, SetSpatialNode, SetSnapshot)
   + **33** × `couldn't read`. One shape, not many.
2. Opening one of the ten supposedly-missing impls on disk: `🧱insert-element/🦀️.rs:8` carries
   `#[derive(… dsl::MutationLeaf)]` with `#[mutation_leaf(contract = ::protocol)]`. **The impl was
   never missing.** Module resolution was broken above it.
3. Every error path in that log names `✳️model` — the pre-rename directory, which no longer exists.
4. Peer semio-f4 independently died on `SetSnapshot` — one of the same ten — in a *different*
   artifact (`📐️step`), under the same `🟤️` → `📸️` emoji reassignment.

So: the mutations parent module fails to load a renamed sibling → `super::*` breaks for the leaves →
the `MutationLeaf` derives go unresolved → the aggregate `Mutation` bound then fails once per call
site. All 225 hang off the 33 module-resolution failures.

### The methodological lesson, which is the durable part

Three sessions produced four measurements tonight and **every one failed a different condition**:
- "0 E0277" from a compile that aborted at module expansion → the count meant "no trait bounds were
  ever evaluated", not "none fail". Caught by: **zero stdio warnings**.
- This round's 225, which *did* type-check (1335 stdio warnings) — but type-checking a tree with
  broken module resolution **manufactures** bogus trait errors. So "it type-checked" does **not**
  license "the trait bounds genuinely fail".

The acceptance test now agreed across sessions, all three conditions required:

> zero `couldn't read` in the log **AND** warnings non-zero **AND** *then* read the error count.

### `🔎️scan-dangling-path-refs.py` — answering condition one without compiling

Written this round and kept in this ticket folder (shared with peers on request). It walks every
`.rs` file, resolves each `include_str!("…")` and `#[path = "…"]` target against the filesystem, and
reports the ones that do not exist. **13,911 files in ~90 seconds**, versus a 20-minute lock-blocked
compile. Limits stated honestly: it only sees string literals (a `concat!`/`env!`-built path is
invisible) and it proves the reference resolves, not that the contents are right.

Baseline at 05:32 — **stdio 0, lowpoly 0**; repo-wide 1399, concentrated in 📕️norm 868,
📸️remodel 102, 🏭️process 80, 📏️layout 75, 🌍️gis 70, 🕸️dag 56. Both crates on this ticket's
dependency path had already healed.

### The feature hypothesis, raised by semio-1d and falsified

Proposal: lowpoly's `default-features = false` might *disable* the features carrying the
`MutationLeaf` impls — a one-line manifest fix. Checked: stdio has exactly one feature,
`default = ["plugin-root"]`, and exactly one gate on it, `🦀️.rs:133`
`#[cfg(feature = "plugin-root")] plugin_exports!(plugin, plugin::StdioApps)` — the WASM component
entry point. No schema, no subset, no `MutationLeaf` impl sits behind it. Turning it on would trade
a compile error for a `rust-lld: duplicate symbol` link error, since two crates in one component
would each emit `#[no_mangle] semio_plugin_install_bundle`. **`default-features = false` is
load-bearing and correct.** Hypothesis dead; no manifest fix exists.

## The real machine-wide blocker: a dead sccache server

`sccache --show-stats` reported **all zeros** with **nothing listening** on its socket — the server
was gone. Four sccache client processes sat pinned at **0.0% CPU for 20 minutes** holding the shared
`target/debug/.cargo-lock`, one of them under a **65-minute** `cargo check --workspace`. That
deadlock — not stdio — is what most sessions had actually been queued behind all night.

Escaped it with a private target dir **and** an sccache bypass:

```
CARGO_TARGET_DIR=…/target-lowpoly-e2e RUSTC_WRAPPER= cargo check -p semio-s-plugin-lowpoly --lib -j 4
```

Cost: a cold dependency rebuild. Started at 73 MB free RAM with swap at 53.8G/55.3G; free RAM dipped
to 17 MB twice and it survived, but that was luck, not headroom. semio-1d's framing is the right one
and is recorded here as guidance: **the machine is past the point where starting another isolated
build is a free action.** Anyone else should let a warm target finish instead of copying this.

## Settled-tree measurement (in flight at time of writing)

`target-lowpoly-e2e` is warm (3.2 GB, 245 deps). Re-run at 13:19 with the current tree:

- `semio-framework-os-kernel` — **compiled clean**, so the `DirectorySpaceDetailV1` blocker that
  stopped procedural/puzzle3d/process3d has healed too.
- `semio-s-plugin-stdio` — type-checking for **~55 minutes**, **0 errors, 0 `couldn't read`**,
  warnings flowing. This is the settled-tree measurement no session had managed to obtain.

Confirms the cascade reading: stdio is not 225-errors broken. Lowpoly not yet reached.
