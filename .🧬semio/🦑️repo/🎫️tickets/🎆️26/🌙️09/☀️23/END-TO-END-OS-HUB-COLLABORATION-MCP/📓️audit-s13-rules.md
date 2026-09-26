# Session 13 Rules Audit (A13-rules)

Read-only AGENTS.md compliance pass over code changed in the last 4 days (`git log --since=2026-09-22
--name-only --format=` over the whole repo, excluding `.🧬semio/🦑️repo/🎫️tickets/**`: 18,088 paths).
Filtered to the runtime paths of the four outcomes — os `s` frontend + plugins (`✏️s/`,
`🧰️framework/🛍️products/💻️os/`, `🧰️framework/🔨️modules/🖱️ui/`), hub backend (`🌎️hub/`,
`🧰️framework/🛍️products/🖥️server/`), collaboration (`🧰️framework/🔨️modules/📡️replication/` +
presence-tagged paths inside the above), semio MCP (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/`) —
excluding `🧫️fixtures/`, `🧪️tests/`, `node_modules/`, `dist/`, `target/`, `📖️stories/`, `🤖️generated/`.
5,029 existing source files (`.rs .ts .tsx .js .mjs .py .sh`) form the audited set (`code-files-exist.txt`).

**Methodology caveat:** `git log --name-only` only proves a file was *touched* in the window, not that
every line flagged below was *added* in it — some hits (esp. #1 and #4) are pre-existing content in
files a session-13 agent also edited elsewhere. Counts are a proxy from grep, not an AST pass; verify
before mass-editing.

## Ranked findings

### 1. `/// @emoji <emoji> …` literal-placeholder bug in ~3,930 docstrings across 163 files — spans all four outcomes — NEW (repo-wide debt, no single slice owns it)

The docstring-emoji rule (AGENTS.md: "docstrings MUST start with a unique and fitting emoji") is violated
at scale by a mechanical artifact: docstrings literally start with the 7-byte token `@emoji` followed by
the real emoji, instead of the emoji alone. Confirmed identical pattern in Rust `///` and TS `/** */`:

```
🌎️hub/🏗️bootstrap/🦀️.rs:127:  /// @emoji 🧯️ Top-level startup error — the only fallible paths outside a document/WS session are
🌎️hub/🏗️bootstrap/🦀️.rs:218:  /// @emoji 📦️ Axum JSON boundary for first-party `ToValue`/`FromValue` directory contracts.
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2029: /// @emoji 🧹️ Explicit store-owner cursor used only when a domain installs it through its
🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx:14:  /** @emoji 🪁️ Label pair resolved by the active driver's `labelTier` axis. */
```

Per-file hit counts (`grep -c '@emoji'`): `🏪️store/🦀️.rs` 420, `⚛️react/🟦️.tsx` 403,
`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` 192, `🔌️plugin/🦀️.rs` 185, `🌎️hub/🏗️bootstrap/🦀️.rs` **160**,
`🏪️store/🔄️sync/🦀️.rs` 154, `🛢️db/🗄️storage/🦀️.rs` 129, `🕸️dag/➕️normal/🦀️.rs` 117,
`📡️replication/📡️wire/🦀️.rs` 69 (collaboration). Full list: `chk4-literal-at-emoji.txt`. This hits the
hub's own bootstrap error taxonomy, the kernel store (used by every outcome), the wgpu/React shell, the
plugin runtime, and the replication wire codec — i.e. the core of all four outcomes simultaneously. High
fix cost if done by hand (163 files) but mechanically trivial (strip literal `@emoji ` before the emoji);
given the memory note on codemod risk on emoji-heavy files (`project-codex-rename-plan-codemod-incident`),
a scripted sweep needs a dry-run diff review, not a blind regex commit.

### 2. Runnable commands missing from `.vscode/launch.json` at massive scale — AGENTS.md rule "All devs … never use the cli … MUST register all executable commands there" — H11 / G11 / S16 / T13 / W3 / WG9/WG10

Cross-checked every `nx` target in the 19 `📋️project.json` files touched in the last 4 days inside outcome
scope against `bun nx run <project>:<target>` strings in `.vscode/launch.json` +
`.vscode/🧩️launch.seed.jsonc` (both currently valid JSONC — a working-tree brace-shuffle in `launch.json`
around line 4674 turned out to be a peer's in-progress, currently-consistent edit, not corruption).
Registered/total per project:

| project (owner) | registered/total |
|---|---|
| `os-hub` (H11, hub backend) | **20/110** |
| `@semio-tech/framework-os-kernel` (LA-LD landing) | **1/48** |
| `@semio-tech/framework-os-dev` (S16/W3 dev tooling) | 8/36 |
| `@semio-tech/framework-renderer-wgpu` (WG9/WG10) | 13/34 |
| `@semio-tech/framework-os-mcp-rs` (G11, semio MCP) | 8/25 |
| `@semio-tech/space-plugin` (S16/T13) | **0/15** |
| `@semio-tech/stdio-plugin` (S16/T13) | 1/14 |
| `@semio-tech/plugin-registry` (W3) | 4/14 |
| `@semio-tech/norm-plugin` (S16/T13, new plugin) | **0/13** |
| `@semio-tech/gis-plugin` (S16) | 1/13 |
| `@semio-tech/framework-os-mcp` (G11, TS side) | 1/7 |
| `@semio-tech/framework-plugin-host` (S16) | **0/7** |
| `os-hub-ts` (H11) | 3/7 |
| `@semio-tech/framework-plugin-web` (S16) | **0/2** |

372 targets checked, 300 unregistered (81%). Sample missing entries: `run os-hub:dev-secure-mcp`,
`run os-hub:trusted-catalog-bootstrap`, `run @semio-tech/framework-os-kernel:wal-recovery-check`,
`run @semio-tech/space-plugin:test` (every target in that project.json, all 15, are missing — see
`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📋️project.json`). This is systemic, not isolated: nx target
authoring has outrun launch.json curation across the whole fleet's last 4 days of work.

### 3. Comments inside definitions — AGENTS.md "MUST NOT comment inside definitions" — S16/T13 (plugin), WG9/WG10 (wgpu shell), G11 (MCP), LA-LD (store)

Proxy (`^\s{8,}//[^/!]`, i.e. an inline `//` comment nested ≥2 levels deep — a fn-body body-block, not an
item-level annotation) over 4,571 changed `.rs` files: **6,324 hits across 266 files**. Verified real
(not annotation-before-fn) samples:

```
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:1008-1010: (inside `composer_entry_of`'s body)
        // 🌉️ bridged via `resolve_ready` — see `composer_entry_of`'s doc: no real
        // suspension exists in a codec body, and this fn-pointer thunk feeds the
        // Send-bounded `ComposeFuture` erasure table (R1), which plain AFIT can't satisfy.
```

Top files: `🔌️plugin/🦀️.rs` 910, `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` 852,
`🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` 229, `🌉️mcp/🏠️workspace/🦀️.rs` **210** (MCP outcome),
`🏪️store/🦀️.rs` 189, `♾️infinite/🌍️world/🦀️.rs` 189. Full list: `chk3-deep-comments-counts.txt`.

### 4. Legacy/fallback/shim code paths — mixed: mostly compliant self-documentation, but real live exceptions exist — S16/T13 (plugin runtime), G11 (MCP protocol, self-justified)

705 matching lines (`fallback` 499, `legacy` 96, `shim` 79, `deprecated` 15, `compat` 2) in
`chk1-legacy.txt`. Two categories:

- **Justified, not a violation:** `🧰️framework/…/🌉️mcp/🧭️protocol/🦀️.rs:5-11` implements `ProtocolEra::Legacy`
  for MCP spec version negotiation with an explicit docstring: *"not a compatibility shim: CLAUDE.md's
  'no legacy support' rule does not apply to an external protocol we do not own."* Most `legacy` hits in
  `🔌️plugin/🦀️.rs` (e.g. lines 595, 900, 910, 40562, 40897) are docstrings **describing already-deleted**
  legacy code ("deleted with the legacy enum in the deletion wave"), which is the repo's own compliant
  convention for narrating removals.
- **Real, live exceptions worth a second look:**
  `🔌️plugin/🦀️.rs:6071` — `/// … those apps keep their own fallback wrapper around this shared core for
  now.` (a live per-app fallback, "for now").
  `🔌️plugin/🦀️.rs:41651,41890` — `ensure_plugin_initialized`'s `component-guest`-gated **weak-linkage
  shim**, and a `#[linkage = "weak"] pub extern "C" fn semio_extension_bundle_installer_link_shim()` at
  the matching call site — a real, named "shim" still active in the plugin/extension bundle-install path
  (may be a legitimate wasm weak-linkage technique mislabeled, or genuine compat code; needs an owner
  read, not a blind removal).

### 5. `[DEBUG]` / debug-print residue — H11/DB1 (hub), G11 (MCP), WG9/WG10 (wgpu shell), S16 (dev tooling)

341 `[DEBUG]` tags, 475 `console.log` (415 of them inside permanent `📜️script.ts` CLI output — legitimate,
not residue), 416 Rust `println!/eprintln!` (123 inside `⌨️cli/` dirs — legitimate CLI output). No `dbg!`
macro found anywhere (good). Real signal after excluding legitimate CLI output:

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` has **26** `console.log("[DEBUG] …")` lines (e.g. line 3666,
  4163, 4758, 8373, 8469, 8533, 8825) reporting structured check results ("pinned-tables=3 deep-equal=…")
  — these read as permanent test-report output that was `[DEBUG]`-tagged and never promoted/cleaned; per
  AGENTS.md `[DEBUG]` marks *temporary* logs meant for removal, so these are either overdue for cleanup
  or wrongly tagged.
- `🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` has 117 `[DEBUG]`
  hits via a named `Self::debug_log(...)` helper (line 6748, 6760, 6860) — looks like a permanent
  instrumentation facility reusing the `[DEBUG]` string, not ad-hoc leftovers; lower confidence finding.
- 293 `println!/eprintln!` outside CLI dirs concentrate in `🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs`
  (20), `🔌️plugin/🦀️.rs` (19), `🧊️wgpu/🧊️renderer/🦀️.rs` (14), `🌉️mcp/🏗️bootstrap/🦀️.rs` (10, e.g. line
  275 `eprintln!("[semio-os-mcp compile-component] {message}")` — plausibly intentional stderr
  diagnostics for a server that has no `tracing`/`log` crate wired in, still worth an owner look).
- 60 `console.log` outside `📜️script.ts`, concentrated in `🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts` (11) and
  `🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts` (5).

### 6. CRUD-shaped verbs on the hub's write side — AGENTS.md "MUST NOT use CRUDs" — H11/C11

Hub routing (`🌎️hub/🏗️bootstrap/🦀️.rs:10589-10616`) is mostly command/event-shaped (`POST
/directory/commands`, `/directory/events`, `/directory/event-page/v1`, per-job `/cancel` and `/approval`
endpoints for inference and artifact-creation — good CQRS shape). Two exceptions use the literal HTTP
DELETE verb on a REST resource rather than a command:

```
🌎️hub/🏗️bootstrap/🦀️.rs:10606:  .route("/auth/sessions/me", get(get_session_me).delete(delete_session_me))
🌎️hub/🏗️bootstrap/🦀️.rs:10608:  .route("/auth/agent-delegations/{id}", axum::routing::delete(delete_agent_delegation))
```

### 7. Scripts outside `📜️script.ts` — AGENTS.md "MUST NOT create any other script files" — T13/S16 (plugin oracles), NEW (repo tooling, out of the four outcomes)

Six `.py` files under `✏️s/🔌️plugins/📕️norm/…/🔮️oracles/` (`en1991`, `iso16757`, `din4108`, `en1995`,
`en1999`, `en1998`) — these are third-party-comparison oracle implementations (AGENTS.md itself requires
"the same output … with at least one third-party library"), so likely intended as fixtures/oracle code
rather than the forbidden "other script files", but the letter of the rule ("You MUST NOT create any
other script files other than `📜️script.ts`") does not carve out an oracle exception — worth an explicit
call from the dev. Outside the four outcomes (repo tooling, lower priority): `.devcontainer/post-start.sh`,
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`, `…/🔩️native/🥾️bootstrap/🐚️.sh`,
`…/🧪️test/🖥️host/🐍️.py`.

### 8. External runtime dependencies — mostly clean

`git log -p` over `🌎️hub/📦️packages/🦀️rust/Cargo.toml` and
`…/🌉️mcp/📦️packages/🦀️rust/Cargo.toml` for the last 4 days shows **no new third-party crate deps** —
only internal workspace deps (`semio-framework-dispatch-macros`, `semio-framework-plugin`) and new test
binaries (`binding_cancellation_law`, `compile_cancellation_law`). 102 `Cargo.toml`/`package.json` files
touched in outcome scope, mostly per-plugin `🏭️bridge/Cargo.toml` (wasm bridge boilerplate) — not spot-
checked individually past the hub/MCP manifests given the volume; a full third-party-type-leakage sweep
of `pub fn` signatures was not run (budget) and should be a follow-up if this rule is a current focus.

### 9. Language-agnostic test + third-party oracle — convention present, not exhaustively sampled

The taxonomy already carries paired `-check`/`-native-check` targets and named `-oracle` targets
(`gis-inference-ledger-oracle`, `hub-live-catalog-oracle`, `canonical-checkpoint-resource-oracle`,
`inference-discovery-oracle` — all in `os-hub`/`framework-os-mcp-rs` project.json, §2 table) plus the
norm-plugin Python oracles (§7), suggesting the oracle-per-feature convention is generally followed. Not
exhaustively verified against every feature landed in the last 4 days; sample only.

### 10. User-facing strings without en+de — no violation found in sample; structurally enforced

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx:205-206` defines `UiLabelValue`'s locale record as
`{ readonly en: UiLabelValue; readonly de: UiLabelValue }` — TypeScript requires both fields, so a missing
translation is a compile error, not a runtime gap. Grepped `👥️PresenceBar`, `💬️AgentChatPanel`,
`🖱️ContextMenu`, `🤖️AgentApprovals` for raw hardcoded `aria-label="…"`/`title="…"`/JSX text — zero hits.
Sample only (43 changed `.tsx` files total, ~8 checked).

### 11. Expensive operations without progress + cancellation — mixed signal, process-level gap on the biggest chains — W3/DB1

Hub inference and artifact-creation jobs have explicit `/cancel` and `/reconcile`/`/approval` routes
(§6), and cancellation-law tests were added in the last 4 days
(`🌎️hub/…/wp-db1`-adjacent Cargo.toml additions `binding_cancellation_law`,
`compile_cancellation_law`) — positive signal for the MCP/hub inference path. But the fleet's own
`📓️session-13-preamble.md` rule 7 routes every chain longer than ~45 min (rebuild-all, `--packages all`
publish, hub 7800 restart) through a **coordinator-launched detached process + manual log polling**
(`w2-detach.py`), not a first-class progress+cancellation API — i.e. the single most expensive operation
class in this session has no built-in cancellation, only kill-by-pid. Not itself new code to cite by
line, but a standing gap in the exact area the rule targets; owner is W3 (build/publish) and DB1 (write
throughput chains).

## Top-5 by impact on the four outcomes

1. §1 `@emoji` literal-placeholder docstring bug — 3,930 hits, 163 files, all four outcomes' core runtime.
2. §2 launch.json under-registration — 300/372 sampled targets missing, worst in `os-hub` (90 missing) and `framework-os-kernel` (47 missing).
3. §3 comments inside definitions — 6,324 proxy hits, worst in plugin runtime (910) and wgpu shell (852), 210 inside MCP's own workspace module.
4. §4 live fallback/shim exceptions in `🔌️plugin/🦀️.rs` (lines 6071, 41651, 41890) — small in count but exactly the pattern AGENTS.md forbids, needs an owner call (weak-linkage technique vs. real shim).
5. §6 CRUD DELETE verbs on hub auth routes (`🏗️bootstrap/🦀️.rs:10606,10608`) — small, concrete, cheap to convert to a command endpoint.
