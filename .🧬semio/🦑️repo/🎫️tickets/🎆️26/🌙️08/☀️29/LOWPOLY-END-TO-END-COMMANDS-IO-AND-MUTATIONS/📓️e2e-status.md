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

