# Audit S14-Tree — Working-Tree Health vs the Relaunched Chain

Auditor A14-tree, session 14, read-only (no builds/servers/edits; git status/diff/log/show only).
Scope: every `git status --porcelain --untracked-files=all` / `git diff HEAD` entry vs HEAD (`6b8089dcb21`), as of
2026-09-27 ~19:0x. Total diff: **593 files, +71622/-6676 lines** (`git --no-pager diff HEAD --stat`). Of these, **271
files are inside the ticket folder** (`.🧬semio/🦑️repo/🎫️tickets/…/END-TO-END-OS-HUB-COLLABORATION-MCP/wp-*` — prepared
scripts, payloads, throwaway check-crates; NOT guest-linked, NOT chain-linked) and **322 are in the real source tree**
(241 `✏️s/🔌️plugins/**`, 56 `🧰️framework/**`, 24 `🌎️hub/**`, 1 `.vscode/launch.json`). All counts below are the
real-tree 322 unless stated otherwise. Method note: git status codes are almost all fully **staged** (`M `/`A `/`R `),
so `git diff` alone (unstaged only) showed just 30 files — the real comparison is `git diff HEAD`.

---

## 1. Ranked chain-risk list (read this first)

### #1 — HIGH, already owned (W4 item 1, in progress) — layout mutation leaves lack authority
`✏️s/🔌️plugins/📏️layout/**`: the untracked/staged leaves `🧬️mutations/📐update-grid` and `🧬️mutations/🔒set-frame-flags`
(mtime 2026-09-27 14:20–14:21, matches preamble's "a peer's 14:21 work") have `🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs` under
`🧬️schema/🧬️mutations/` but are **missing their own `🔣️.json`** (every sibling leaf, e.g. `📏resize-frame`,
`➕create-frame`, has one) **and the whole `🧬️schema/` subdirectory that JSON would normally sit next to**. This is the
confirmed root cause of `error[E0277]: the trait bound ‹update_grid::component::UpdateGrid›/‹set_frame_flags::component::SetFrameFlags›: MutationLeaf is not satisfied`
(`.🧬semio/🌐hub/s13-w3-logs/final-rebuild-all.txt:68956` onward, ~18 repeats). New `✏️editor/🎮️commands/📄️patch-document/🦀️.rs`
(untracked) is a **separate, complete, unrelated** command (rename/print-target/data-fields only) — not part of this risk.
**Status: W4's item 1, not yet touched (fix scripts staged in `wp-w4/` at 18:35–18:37, not yet applied).** No action
needed from this audit beyond confirming scope — nothing else in the tree shares this pattern (checked every `A `
entry under `🧬️mutations/` repo-wide: only these two leaves).

### #2 — HIGH, NOT yet owned — robotic + flow-text failures have no captured diagnostic; log corruption
The same failed run also failed `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/📦️packages/🦀️rust/Cargo.toml` (log
line 25137) and `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/📦️packages/🦀️rust/Cargo.toml` (line 40597) — W4 lists this
as item 2, **open**, no root cause yet. Findings:
- **Neither extension has ANY working-tree change** (confirmed: not in `git status`, not in `git diff HEAD`). This is a
  pre-existing HEAD-committed condition, unrelated to anyone's in-flight edit — it is not "waiting on a peer to
  finish," it will very likely **reproduce identically on relaunch**.
- **Neither extension defines any `🧬️mutations/` leaves** (checked), so this is *not* the same missing-`🔣️.json`
  pattern as layout.
- **The log has zero `error[E…]` lines anywhere near either failure** (checked both windows and the whole 99963-line
  log: only layout's 18 `E0277`/1 `E0283` show up). At both failure points the log shows the exact same corruption
  signature: a warning block cuts off mid-line into a garbled `notewarningerror` token, immediately followed by the JS
  wrapper's generic `error: Cargo artifact build failed: <path>` (thrown from `buildCargoArtifacts` on any non-zero
  exit — it never captured the real rustc/cargo diagnostic). This reads as **nx's parallel multi-task stdout
  interleaving corrupting the log at the exact moment of failure** for both crates, not as evidence of "no error was
  printed." No `SIGSEGV`/`Killed`/`panicked` signal evidence either.
- **Recommendation for W4 before relaunching the full chain:** rebuild each crate **in isolation** (e.g.
  `cargo build -p <robotic/flow-primitive-family crate> --manifest-path …/Cargo.toml` outside nx's parallel runner, or
  `nx run <project>:component-dev --output-style=static` for just that one target) to capture the real diagnostic.
  Relaunching the full 11-step chain without this will likely burn another full cycle only to reproduce the same
  unexplained failure.

### #3 — MEDIUM — `.vscode/launch.json` working tree has 159 fewer configs than HEAD, unverified
Working tree: **1244** configurations; HEAD: **1403** (`git show HEAD:.vscode/launch.json`, JSONC-parsed). The diff is
`-3590/+103` lines. The delta (159) matches almost exactly what dropping the 158 now-non-shipped stdio library-fleet
apps (176 − 18 = 158, per landing row 61 "`lb-p3-stdio-shipped-fleet.py`" / row 67 "`lb-p4` … `SHIPPED_APP_CEILING`
… ≤24") would remove from a per-app-subset launch generator — i.e. this is very plausibly the **expected, correct**
side effect of already-landed work, not corruption. However: (a) this file is normally frozen during the guest
freeze and every dev uses it exclusively (never the CLI, per AGENTS.md), (b) I cannot re-run the generator myself
(read-only), so this is **unverified**, not cleared. **Recommend:** a cheap `launch generator check`/byte-diff pass
(no rebuild) before or right after the relaunch, so a genuinely stale/partial regeneration isn't shipped silently. No
other frozen file touched: root `nx.json`, `🔣️taxonomy.json`, root `Cargo.lock`/`Cargo.toml`, `.cargo/config.toml`,
and every `📋️project.json` are **byte-clean vs HEAD** (checked directly) — the freeze is otherwise intact.

### #4 — LOW, generated/expected — extension descriptor hash churn
`✏️s/🔌️plugins/{🌊️flow,📜️imperative,📐️cad,🪵️sourcing,🏭️process}/🧩️extensions/**/{🔣️.json,🛂️.descriptor.semio}` (30
files) only change `wasmSha256`/`coreWasmSha256`/`descriptorSha256` — outputs of a prior `generate` step (matches
landing rows 63/64, the coordinator's repeated standalone `generate`/`check --skip-nx-cache` runs). The chain
regenerates these itself; no hand content changed. Safe to relaunch over.

### #5 — LOW — norm renames/`#[path]` updates already landed and green
`✏️s/🔌️plugins/📕️norm/**` (66 files, mostly `R` renames + 6 `#[path]`-bearing roots) = the coordinator's
`wp-coord/norm-example-slugs.py` (landing row 62, 12:4x 09-27): fixed the previous chain failure's norm example-slug
emoji/VS16 drift. Native check green, "taxonomy violations 0" recorded. Two pre-existing test reds noted there
(`🧪️tests/🔬️surface`, `🚦️compliance-gate`) are unrelated stale-fixture issues already routed to N1, not new.

### #6 — LOW — wfc clock-injection wrapper, additive
`✏️s/🔌️plugins/🀄️wfc/**` (25 files): each `🧬️schema/💡️inferences/🦀️.rs` gains a `solve_with_clock(snapshot, now_us)`
sibling of the existing `solve_with_job` (T13's prepared "wfc solve-law clock fix", landing row 163) — additive,
backward compatible, no removed symbol. Low risk.

### Hub / os-mcp (must stay compile-green through the post-publish os-hub/os-mcp build)
All 24 `🌎️hub/**` changes and the 56 `🧰️framework/**` changes map cleanly onto already-landed, native-green session-13
work with no unclaimed content found (see §2 for the row-by-row mapping): H11's `CHANNEL_VERSION`/`artifact_authority`
fixes, LD's envelope `observed`/`target` wire + vigilant law, G11's channel-version-pin generator + agent rate-limit +
empty-body directory grant, LB's `linked-codec-ownership` fence, C11's `access-changed` stream, H12's codec table +
puzzle cost fixes, H9-C creation-progress, S16's preference-lane auth-schema enum, WG9's lost-Ack/echo-suppression,
WG10's canonical-pair + AccessKit + transport-deadline, R9's SDK-shim removal + auth DELETE→POST routes. Landing rows
report `semio-hub --bins --tests` green as of session 13 ~11:2x. **No new/unclaimed hub or framework edit found in
this pass.** Risk is therefore inherited entirely from #1/#2 above (the descriptors step gates everything after it),
not from hub/framework content itself.

---

## 2. Inventory by area → claim → verification status

| Area | Files | Claim (landing row / wp report) | Verified after last edit? |
|---|---|---|---|
| `✏️s/🔌️plugins/📏️layout/**` | 31 (+3 untracked) | **W4** item 1 (this session, in progress) | No — fix not yet applied (§1 #1) |
| `✏️s/🔌️plugins/📕️norm/**` | 66 | landing row 62 (coordinator, norm-example-slugs) + row 75 (N1, test-only) | Yes — native green, taxonomy 0 violations |
| `✏️s/🔌️plugins/🀄️wfc/**` | 25 | landing row 163 (T13, "wfc-clock prepared") | Additive only; not independently re-checked by me, low risk |
| `✏️s/🔌️plugins/🌊️flow/**` | 18 (8 descriptors + robotic-adjacent 📝️text untouched) | descriptor hashes → row 63/64 generate churn; **📝️text extension itself has 0 tree changes** | 📝️text failure is pre-existing, see §1 #2 |
| `✏️s/🔌️plugins/📜️imperative/**` | 12 | descriptor hash churn (row 63/64); `📝️text` here compiled fine in the failing run (log line 23800) | n/a (descriptors only) |
| `✏️s/🔌️plugins/📐️cad/**`, `🪵️sourcing/**`, `🏭️process/**` | 10+8+8 | descriptor hash churn (row 63/64); **`🏭️process/🧩️extensions/🤖️robotic` itself has 0 tree changes** | robotic failure pre-existing, see §1 #2 |
| `✏️s/🔌️plugins/🗄️stdio/**` | 7 | LB-P3/LB-P4 shipped-fleet split + guard (rows 61, 67) | Yes — native green 27 15:06 |
| `✏️s/🔌️plugins/📖️playbook, 🔱️trinity, 🌿️vcs, 🌀️procedural, ✒️writer` | 4/3/3/3/3 | small test/law diffs matching S17/LC/G11 landed rows | Not individually re-verified by me; no compile-risk pattern found |
| `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`, `🧵️job/**` | 2+2 | R9 ActivationRegistry/router fixes (row 55/71 area), wfc-clock's `default_now_us`/`logical_now_us` (T13) | tsc/native green per rows |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/**` | 10 | G11 carrier/rate-limit/fault-map (row 56), G11 directory-grant-empty-body (row 65) | native+laws green per rows |
| `.../🏪️store/**` | 6 (+2 new fixture/schema) | LD envelope `observed`/`target` (row 58/60/70), rebuild-welcome fixture (row 70) | native+wasm32+laws green per rows |
| `.../📇️directory/🔌️client/**` | 2 | G11 empty-body grant (row 65) | native green |
| `.../📺️renderer/**` (ShellHost, wgpu Shell, TextEditor, engine-contract) | 8 | WG10 AccessKit/canonical-pair (row 44/46), C11 access-changed, S16 Home viewer (row 53) | native+wasm32+laws green per rows |
| `.../🔌️plugin/**` (host, registry, tool-run) | 5 | H12 codec-table (row 54), R9 shim removal (row 55), W3 item 4 (row 29) | native+wasm32 green per rows |
| `.../🧑‍💻dev/**` (fixtures/tests) | 7 | V1 program-matrix, S16 matrix rows, F2 perf | test fixtures only, no compile risk |
| `🌎️hub/**` | 24 | H11 (bin/CHANNEL_VERSION), LD (vigilant law), G11 (rate-limit/pin), LB (linked-codec-ownership), C11, H12, H9-C, S16 | `--bins --tests` green ~11:2x per rows |
| `.vscode/launch.json` | 1 | R9 launch generator (row 157) | **Unverified count delta, §1 #3** |
| `.🎫️ticket.json` | 1 ( M, unstaged) | session bookkeeping (not code) | n/a — do not touch per rules |

Nothing under `✏️s/**` or `🧰️framework/**` is **unclaimed**: every touched crate/area maps to a named landing row or a
named slice's active scope. No file is claimed-but-stale by a landing timestamp earlier than its mtime, other than
layout (§1 #1, explicitly still open) and the two untouched-but-failing extensions (§1 #2, which were never "claimed"
at all — they simply weren't investigated).

### Half-finished-work checks (all repo-wide, not just flagged areas)
- **Untracked/new leaves wired but missing authority:** only layout's two leaves (checked every `A ` entry under any
  `🧬️mutations/` path repo-wide).
- **`todo!()`/`unimplemented!()` in touched non-test files:** **0 hits** (checked every file this diff touches).
- **`[DEBUG]` literals actually introduced by this diff:** grepping whole files is misleading here — the wgpu Shell
  and plugin-host files carry **thousands of pre-existing, permanent `debug_log`/`debug_runtime_line` calls tagged
  `[DEBUG]`** by deliberate, documented convention (e.g. `🧰️framework/…/🔌️plugin/🦀️.rs:31168` "🐞️ `[DEBUG]` last
  maintenance stage entered — temporary, ticket 26/09/02/PUZZLE-3D-END-TO-END", `…/wgpu/🦀️.rs:3768` "🐛️ Last
  `[DEBUG]` dock-plan line, so the trace prints on CHANGE rather than once per frame") — **none of those lines are in
  this diff** (`git diff HEAD -- <file> | grep '^[+-].*\[DEBUG\]'` = 0 hits for both). The diff's only real additions
  are 2 new `debug_log` calls in the wgpu Shell file following the same existing convention (rebootstrap-reseed
  logging) — consistent, not a violation. Two genuine **removals** landed cleanly (`println!("[DEBUG] stdio plugin
  assembles…")` — LB-P4; `console.debug("[DEBUG] document archive restore"…)` — S16 item 4).
  Separately, **pre-existing** (not diff-introduced) `eprintln!("[DEBUG] …")` leftovers remain in 5 test files
  (`procedural/generation3d`, `layout`, `norm/compliance-gate`, `trinity/jack`, `stdio/editor-catalog` unit tests) —
  these predate this session's edits and are a repo-hygiene item, not a chain risk.
- **Dangling `#[path]`:** one absolute `#[path = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay/…"]` exists,
  but it is inside AV1's own throwaway verification crate
  `wp-av1/rs-check/src/lib.rs` (ticket folder, narrow-proof-over-an-overlay per session-13 rule 37) — not shipped, not
  a real-tree risk. No other absolute or otherwise-dangling `#[path]` found in the real tree.

---

## 3. AGENTS.md violations introduced in this diff

- **`@emoji` tag instead of a leading emoji glyph, in NEW docstring lines:** `🌎️hub/🏗️bootstrap/🦀️.rs`,
  `🌎️hub/📇️directory/🦀️.rs` (11 instances — role/ceiling/principal doc comments), `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs`,
  `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`, `.../🏪️store/🔄️sync/🦀️.rs`,
  `.../📺️renderer/…/TextEditor/🟦️.tsx` (×2). AGENTS.md requires docstrings to *start with* an emoji; `/// @emoji 🎚️ …`
  starts with the literal ASCII tag, not the glyph. This is **not a new pattern** — it is the same repo-wide `@emoji`
  drift session 13 already found and explicitly deferred (fleet-13-agents.md 19:4x / 21:2x: "residue since 2026-04…
  codemod + law AFTER publish", R9 item 5) — these new lines simply continue the existing (already-decided-to-defer)
  convention rather than introduce a fresh one. Not blocking, but flag so it isn't miscounted as new debt.
- **No comments-inside-definitions found** in the sampled new/changed function bodies across the touched real-tree
  files (spot-checked the largest hunks: layout mutations, hub bootstrap/directory, store/sync, wgpu Shell additions).
- **No CRUD, no CRDT, no external runtime dependency** additions found in this diff (root `Cargo.lock`/`Cargo.toml`
  untouched; no new `[dependencies]` lines in any touched `Cargo.toml`; the only `Cargo.lock` in the diff is AV1's
  own throwaway ticket-folder check crate).
- **No compatibility shim / fallback / deprecation markers** found in the diff (`grep` for `deprecat|legacy|fallback|
  compat` across touched real-tree files: only doc-comment prose describing *removed* shims, e.g. R9's SDK
  weak-linkage shim removal — that's a deletion of a shim, not an addition).

---

## Summary for the coordinator

Layout (#1) is W4's known item, untouched so far. **Robotic and flow-text (#2) are the one genuinely new,
un-owned finding**: zero working-tree changes there, so they are not "someone's unfinished edit" — they are a
pre-existing failure whose real diagnostic never made it into the log (nx parallel-output corruption at the exact
failure point, confirmed on both). Relaunching the full chain without first getting a clean isolated build capture
for those two crates risks reproducing an unexplained failure. `.vscode/launch.json`'s 159-config shrink (#3) is very
likely the correct effect of the already-landed stdio shipped-fleet fix, but is unverified by me and cheap to check.
Every other real-tree change (hub, framework, the rest of the plugins) maps to already-landed, native-green
session-13 work with no unclaimed or stale-claimed content found.
