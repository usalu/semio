# P0b — Forbidden-Call Audit & Dependency Freeze Guardrails

Work packet P0b of the Interactive Job Runtime Refactor (master ticket
`26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`, phase ticket
`PHASE-0-INTERACTIVITY-OBSERVABILITY-AND-DEPENDENCY-FREEZE`). Baseline commit `95b8688ee2f62f4056b6403c282bf0c76172c37c`.

Two enforcement mechanisms wired into `📜️script.ts`'s `VerifyScript` (`🧰️framework/🛍️products/💻️os` is
untouched — both live in the root `📜️script.ts`, regions `🔖️InteractivityAudit` and
`🔖️DependencyFreeze`, right after `🔖️VerifyScript`).

## 1. Forbidden-call audit — `verify interactivity`

### How to run

```
bun ./📜️script.ts verify interactivity
bun nx run workspace:verify-interactivity          # same thing, via nx
```

Launch.json: **📦️verify⏱️interactivity🚦️audit** (`.vscode/launch.json`, group `4_build`, order `209.16`,
right after `📦️verify📦️package🚦️purity`).

### What it scans

Two independently-scoped rule groups, both regex-based over masked-literal, test-mod-excluded Rust
source (reuses the repo's existing `policyMaskLiterals`/`policyTestModSpans` machinery — a
`#[cfg(test)] mod tests { … }` body is never scanned, matching this repo's own R4-clause-5 precedent
that a test module is a sanctioned executor/blocking entry point):

- **`blocking-bridge` / `sync-fs` / `sync-net` / `sync-clipboard` / `sync-process` / `sync-db`** —
  scoped to `INTERACTIVITY_AUDIT_UI_ROOTS` only:
  - `🧰️framework/🔨️modules/🖱️ui/`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/`
  - `🧰️framework/🛍️products/💻️os/🖥️host/`
  - `✏️s/🔌️plugins/`
  Patterns: `block_on(`, `run_blocking(`, `std::fs::`, `std::net::`, `reqwest::blocking`,
  `arboard::` (the repo's actual sync clipboard crate — `BrowserClipboard::read_text_async`, the
  sanctioned async exception, uses a different symbol and is never matched), `std::process::Command`,
  `rusqlite::`, `sqlx::`.
- **`thread-pool`** (`std::thread::spawn(`, `rayon::ThreadPoolBuilder`,
  `tokio::runtime::Builder`) — scoped **repo-wide** (minus `compose/`), minus
  `INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS`:
  - `🧰️framework/🔨️modules/⏳️async/` (the async primitives crate — `ThreadPlan`/`block_on`/`ManualRuntime`)
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/` (`TokioHostRuntime` — the crate that actually
    calls `tokio::runtime::Builder` today)

  This category is deliberately **not** scoped to the UI-reachable roots like the others — the task's
  own framing ("outside the single sanctioned runtime module... becomes the 'no nested pools' gate in
  Phase 1") targets the whole repo, and the real known thread-creation sites (shard executor threads,
  `TokioHostRuntime`, `os/store/sync`'s ad-hoc runtimes) sit outside the four UI roots entirely. This
  is a documented interpretation call, not literal task text — flag it to the coordinator if Phase 1
  wants a different scope.

### Config location

Everything lives in `📜️script.ts`, region `🔖️InteractivityAudit` (search for that string):
- `INTERACTIVITY_AUDIT_UI_ROOTS` — the UI-reachable prefix list (widen here for later phases).
- `INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS` — the thread-pool rule's exemption list.
- `INTERACTIVITY_AUDIT_SEVERITY` — `"warn" | "deny"`.
- `INTERACTIVITY_AUDIT_PATTERNS` — the regex table (category, pattern, scope).
- `INTERACTIVITY_AUDIT_ALLOWLIST` — the 5 known `block_on`/`run_blocking` sites, each with a
  `reason`, a `phase`, an `inScope` flag, and an optional `expectedNeverToMatch` flag.

### How to flip WARN → DENY

One line: change `INTERACTIVITY_AUDIT_SEVERITY` in `📜️script.ts` from `"warn"` to `"deny"`. In DENY
mode the command throws (non-zero exit) if there is any:
- `blocking-bridge` finding not covered by an in-scope allowlist entry,
- allowlist entry that no longer matches any finding (stale — shrink the list),
- finding in any other category (`sync-fs`/`sync-net`/`sync-clipboard`/`sync-process`/`sync-db`/`thread-pool`)
  at all — those categories have no allowlist mechanism, so DENY mode requires them to be exactly zero.

Per the master plan this flips at Phase 3 packet P3c ("forbidden-call audit turns from warn to deny").

### Current violation counts (this run — see `📝️p0b-audit-baseline.txt` for full output)

180 findings total:

| category | count |
| --- | --- |
| blocking-bridge (`block_on`/`run_blocking`) | 122 |
| sync-fs (`std::fs::`) | 36 |
| sync-clipboard (`arboard::`) | 6 |
| sync-process (`std::process::Command`) | 6 |
| thread-pool (`std::thread::spawn`/rayon/tokio Builder) | 10 |
| sync-net, sync-db | 0 |

121 of the 122 `blocking-bridge` findings are **not** covered by the allowlist (real surface for
Phase 1+ to work through) — mostly plugin geometry-kernel bridges (`✏️s/🔌️plugins/🌊️flow/…/📐️brep`,
`🖍️draw`, `📐️cad`, `🏭️process`) calling `block_on(kernel.…)` per-operation, plus `pollster::block_on`
call sites in the OS renderer's wgpu glue (`📺️renderer/…/📦️glue.rs`, `…/Shell/🧊️component.rs`). 1
finding (`activation.rs`) is covered by the allowlist.

`sync-fs`'s 36 hits and `thread-pool`'s 10 hits are the next largest categories — not enumerated here,
see `📝️p0b-audit-baseline.txt` for the full per-line list.

### Allowlist (5 entries, all reconciled against the actual repo — 3 needed a correction)

| file | pattern | phase | note |
| --- | --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs` (~255) | block_on | PERMANENT | CLI root — approved process entry point. **Out of today's scan scope** (`🏃️run/` isn't a UI root) — pre-declared for when scope widens. |
| `🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation.rs` (114) | block_on | Phase 1 (P1c) — removed | Shard forwarder poll loop. In scope, matches today. |
| `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` (~1985) | block_on | PERMANENT (test-only) | **Corrected**: this is a `#[cfg(test)] mod tests` local block_on helper, not production code as the master ticket described — already excluded by the test-mod filter. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs` (605) | run_blocking | Phase 1 (P1b) | **Corrected path** — the master ticket's anchor had an extra `📦️packages/🦀️rust/` segment that doesn't exist; real path has none. `ComputeScheduler::run_blocking`. **Out of today's scan scope** (`🛎️services/` isn't a UI root — it IS one of the two `thread-pool` sanctioned roots). |
| `🧰️framework/🔨️modules/⏳️async/✨️macros/🦀️component.rs` (~58) | block_on | PERMANENT (test-only) | **Corrected path** — same extra-segment issue as above. `#[async_test]` expands a hygienically-named `__semio_async_test_block_on` via `syn`/`quote!` — never appears as literal `block_on(` text in any file, so no static scanner (this one included) can see it regardless of scope. |

Stale-entry bookkeeping in the run: 1 entry `expectedNeverToMatch` (host/component.rs, correctly
never fires), 3 entries pre-declared out-of-scope (bin.rs, services, async-macros — correctly not
counted as stale), 1 entry actually matched (activation.rs). **0 real stale entries.**

## 2. Dependency freeze — `verify dependencies`

### How to run

```
bun ./📜️script.ts verify dependencies write-baseline   # (re)generate the baseline from the current tree
bun ./📜️script.ts verify dependencies                  # check current tree against the committed baseline
bun nx run workspace:verify-dependencies-freeze-write-baseline
bun nx run workspace:verify-dependencies-freeze
```

Launch.json: **📦️verify🔒️dependencies🚦️freeze** (check, order `209.17`) and
**📦️verify🔒️dependencies📸️write-baseline** (regenerate, order `209.18`), both group `4_build`,
right after the interactivity audit entry.

### What it covers

Every `Cargo.toml` in the workspace (`policyDiscoverCargoTomlFiles`, filtered to drop `compose/`) and
every `package.json` repo-wide (new walker `dependencyDiscoverPackageJsonFiles`, same
`POLICY_SKIP_DIRS` skip-list plus an explicit `compose` exclusion). For Rust: `[dependencies]`
(runtime), `[dev-dependencies]` (test), `[build-dependencies]` (build), including
`[target.'cfg(...)'.…]` variants and both `name = { workspace = true }` and the dotted
`name.workspace = true` shorthand (resolved against the root `Cargo.toml`'s
`[workspace.dependencies]` table). For JS: `dependencies`/`peerDependencies`/`optionalDependencies`
(runtime), `devDependencies` (tooling).

Internal/first-party detection:
- Rust: has a `path = "…"` key (primary signal, including through `workspace = true` resolution), OR
  name starts with `semio-`/`semio_`, OR name is exactly `ports`, OR name matches `db`/`db_*`.
- JS: name starts with `@semio-tech/`, OR version starts with `workspace:`/`file:`/`link:`, OR the
  name matches any `"name"` field found in a repo package.json (covers internal packages some
  consumer references by bare semver range instead of a `workspace:` protocol).

Verified clean: 0 `semio-*`/`ports`/`db*`/`@semio-tech/*` names leaked into the baseline.

### Baseline location

`🔒️dependencies.json` at the **repo root**, alongside `📋️project.json`/`📜️script.ts`/`🧪️vitest.config.ts`.
No existing convention fit a repo-wide, hand-ratcheted, *committed* generated inventory: the
`🤖️generated/` folders that already exist next to owning modules (e.g. the plugin registry's
`🔣️plugins.json`) are build-regenerated on every run and are `.gitignore`d
(`**/🤖️generated/` in `.gitignore`) — the opposite of what a freeze baseline needs (it must be
committed and must NOT silently regenerate itself back to matching whatever is currently on disk).
Root-level, single-emoji-prefixed, alongside the other root tool files, was the closest fit.

Shape:
```json
{
  "generatedAt": "ISO 8601",
  "commit": "<git sha the baseline was cut at>",
  "entries": [
    { "ecosystem": "rust" | "js", "name": "...", "version": "...", "kinds": ["runtime"|"build"|"test"|"tooling", ...], "users": ["<repo-relative manifest path>", ...] }
  ]
}
```
Only **third-party** entries are stored — internal/path/workspace deps are never written, so the
baseline can't accidentally start gating on a crate rename or crate merge.

### Ratchet semantics

Identity is `${ecosystem}:${name}` — **version is excluded from identity** (recorded for information
only), so a routine patch/minor bump never trips the gate; only a genuinely new third-party name does.
Verified by direct test: removing `tokio` from a scratch copy of the baseline and re-running the check
correctly threw `1 new third-party dependenc(y/ies)`; restoring it passed clean again. Removed
dependencies are reported but never fail — the check only ever tightens.

### How to approve a new dependency deliberately

Add it, then run `bun ./📜️script.ts verify dependencies write-baseline` to fold it into the committed
baseline, then commit `🔒️dependencies.json`.

### Current baseline (this run — see `📝️p0b-freeze-baseline.txt` for the write-baseline + check transcript)

**238 third-party dependencies** at commit `95b8688ee2f62f4056b6403c282bf0c76172c37c`: **104 Rust
crates**, **134 JS packages**. By kind (a dependency can count in more than one): 162 runtime, 69
tooling, 18 test, 2 build. Spot-checked against the master ticket's dependency-surface notes: `tokio`
(12 users), `serde`/`serde_json`/`thiserror`/`wasm-bindgen`/`ts-rs`/`wgpu`/`winit`/`rusqlite`/`sqlx`/
`rayon`/`wasmtime` all present with plausible user counts.

## Files touched

- `/Users/ueli/Documents/semio/📜️script.ts` — new regions `🔖️InteractivityAudit` and
  `🔖️DependencyFreeze` (after `🔖️VerifyScript`); `VerifyScript.run()` dispatches `interactivity` and
  `dependencies` subcommands; two new private methods `runInteractivityAudit`/`runDependencyFreeze`.
- `/Users/ueli/Documents/semio/📋️project.json` — 3 new nx targets: `verify-interactivity`,
  `verify-dependencies-freeze`, `verify-dependencies-freeze-write-baseline`.
- `/Users/ueli/Documents/semio/.vscode/launch.json` — 4 new entries in group `4_build`
  (orders `209.16`–`209.18`, right after `📦️verify📦️package🚦️purity`, before `📦️verify🏛️workspace🚦️gate`).
- `/Users/ueli/Documents/semio/🔒️dependencies.json` — new committed baseline file (238 entries).
- This folder: `📝️p0b-audit-baseline.txt`, `📝️p0b-freeze-baseline.txt`, `📓️p0b-guardrails.md`.

## Validation performed

- `bun ./📜️script.ts verify interactivity` and `bun nx run workspace:verify-interactivity` both run
  clean, exit 0 (WARN mode).
- `bun ./📜️script.ts verify dependencies write-baseline` then `verify dependencies` (and the nx
  targets) both run clean.
- Ratchet failure path exercised directly: removed `tokio` from a scratch copy of the baseline,
  confirmed the check throws with the correct message, restored the real baseline, confirmed clean
  again.
- `bunx tsc --noEmit` — zero new type errors introduced (pre-existing repo-wide `Dirent<NonSharedBuffer>`
  errors are present in the same pattern in code I didn't touch; the one real issue in my new code — a
  bad `as typeof currentSection` cast — was found and fixed).
- `bun ./📜️script.ts policy check` (full repo policy aggregator, includes the `no-raw-spawn` rule that
  forbids raw `execSync`/`spawn` in `script.ts` files) — zero new breaches; confirmed by grepping the
  full output for any of my new identifiers or `🔒️dependencies.json`, none found. (Caught and fixed a
  real violation during development: an initial `require("node:child_process").execSync` for reading
  the git commit sha was replaced with the budgeted `runProbe` helper.)

## Known follow-ups for the coordinator

- The `thread-pool` category's repo-wide (rather than UI-root-scoped) interpretation is a judgment
  call — see the "What it scans" section above.
- Three of the five allowlist entries needed a location or classification correction relative to the
  master ticket's text (see the allowlist table) — the corrections are recorded in each entry's
  `reason` field in `📜️script.ts` so they don't get silently re-introduced.
- 121 unallowlisted `block_on`/`run_blocking` sites and 36 `sync-fs` sites are real Phase 1+ work —
  not something P0b fixes, just surfaces.
