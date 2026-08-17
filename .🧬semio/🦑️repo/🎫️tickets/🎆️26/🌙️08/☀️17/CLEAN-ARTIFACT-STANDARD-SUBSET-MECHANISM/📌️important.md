# 📌️ BINDING RULES FOR EVERY AGENT ON THIS TICKET

Ticket path (use this exact path for `ticket_reopen`/`ticket_close`):
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`

> ⚠️ `ticket_open` wrote a duplicate at `.🦑️repo/🎫️tickets/…` (MCP cwd bug, a peer session hit it too). The canonical copy above is authoritative. Do not write into `.🦑️repo/`.

## Never
- ❌ **Never** run a modifying git command (`commit`, `stash`, `checkout`, `reset`, `worktree`, `rm`). Others are live in this tree.
- ❌ **Never** `isolation: "worktree"` on Agent/Workflow.
- ❌ **Never** close or reopen this ticket. Only the coordinator closes it.
- ❌ **Never** run bare `cargo check` / `cargo build` / `cargo test` — always `-p <crate>` with
  `CARGO_TARGET_DIR=<ticket>/🎯️target`. A red workspace is not a red crate.
- ❌ **Never** claim a test passes without running it, or a feature works without runtime evidence.
- ❌ **Never** edit a file outside your boundary. Write a patch to `🔧️patches/<slice>-<file>.txt` and list it under
  `## sharedFileRequests` in your report; the W5 serializer applies it.
- ❌ **Never** use `.log` for scratch (repo-wide gitignored) — use `.txt` inside this folder.

## Always
- ✅ Read `📓️design.md` before touching anything. It is the contract.
- ✅ Every research/summary output is a `📓️*.md` **in this folder**, not a chat message.
- ✅ Prefix temporary logs with `[DEBUG] ` so they can be swept.
- ✅ Report ends with `## verification` (exact commands + exact numbers) and `## sharedFileRequests`.
- ✅ If a file you need is being rewritten by a peer session right now, report `blocked-peer` with proof — do not
  chase a moving target. Ticket start commit: **`101a6b4ea83acc82d6fbdc0607e6ae5d876825ae`** (2026-08-17 15:59:36
  +0200). Check both `git log --date=iso -- <path>` (newer commit = peer landed) **and** `git status --porcelain
  -- <path>` (` M` = peer editing right now, uncommitted — the more dangerous case; it has bitten this ticket twice).
- ✅ Verify the **test** target, not just `cargo check`. A green `cargo check -p <crate>` says nothing about tests,
  and this repo's peer collisions land there. For `semio-framework-os` you must also pass `--features os-host-full`
  or it reports "0 tests run" and looks healthy while running nothing.

## Hot files — single writer each (claim in `📓️status.md` before touching)

| file | owner |
|---|---|
| `🧰️framework/🔨️modules/🚪️io/**` | W1 framework agent |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` | W1 framework agent |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` | W1 framework agent |
| `…/🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit` | W1 framework agent |
| `…/🔌️plugin/🖥️host/🦀️component.rs` | W1 framework agent |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` | W1 framework agent |
| `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`, os-kernel `📦️glue.rs` | W1 framework agent |
| `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` | W1b OS-host agent |
| `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`, `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts` | W1b OS-host agent |
| repo root `📜️script.ts`, `🔣️taxonomy.json` | W1 (allow-half) then W6 ratchet agent |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` + `🗄️stdio/🦀️component.rs` | coordinator only (shards send patches) |
| `✏️s/🔌️plugins/<p>/📦️packages/🦀️rust/📦️glue.rs` + `<p>/🦀️component.rs` | that plugin's single fan-out agent |
| `🌎️hub/**` | read-only for everyone until W6 |

## Debt register (must be empty before close)

| # | debt | opened | owner | closed |
|---|---|---|---|---|
| D1 | old registration (`derive_artifact_facets!`, `ArtifactDefinition` rows, `.editor/.viewer/.document_app/.artifact_kind`) coexists with the new declaration tree | W1 | W6 | ☐ |
| D2 | old io (`ComposerEntry`/`IoKey`/`io_dispatch`/`io_compose_via`/`SubsetValidator`/`FormatCatalog`) coexists with `IoEntry`/`io_route` | W1 | W6 | ☐ |
| D3 | old WIT names (`artifact-compose`, `list-artifact-dialects`, `io-compose`, `io-dialects`, `migrate-artifact`) coexist with new io-* names | W1 | W6 | ☐ |
| D4 | new policies in report mode | W1 | W6 | ☐ |
| D5 | builder method is `.declare_artifact()` because the old `ArtifactDeclaration` still owns the name `.artifact()` | W1-C | W6 (rename) | ☐ |
| D6 | stdio `📇️registry`'s rigid 36-artifact factory map is the only path `plugin()` reaches artifacts by — blocks per-artifact cutover | W2-P | W2 (delete `📇️registry`, cut all 36 at once) | ☐ |
| D7 | `NativeCodecs.{snapshot,diff,mutations}` land as `LanguagePair{None,None}` for binary/txt — the 5 `dsl::LanguageSpec` roles the old `register_pilot_languages()` registered are not yet wired | W2-P | W2 | ☐ |
| D8 | ~~`os_reachable_export_dialects` / `os_reachable_import_dialects` shipped with no caller~~ — **DELETED by the coordinator.** They were dead *and* subtly wrong: one-hop filters over `io_entries()`, so a shell using them would under-report what is actually exportable (real reachability is `io_route`, which already exists). Whoever builds "Export as…" should query `io_route`/`io_entries` directly. ⚠️ Removal verified by zero-reference grep across `🧰️framework` + `✏️s`; **compile verification was blocked** — a peer's uncommitted edit to `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` (new `ArgFormat` type, serde bounds unsatisfied) reds `semio-framework` and everything downstream. **The breakage is provably not the deletion**: it is in `semio-framework`, which is *upstream* of `semio-framework-os` where the deletion happened — a downstream removal cannot break an upstream crate. Re-confirmed twice ~30 min apart: identical `ArgFormat` errors, file still ` M` (uncommitted). Re-run `cargo nextest -p semio-framework-os --features os-host-full` once that clears; expected **110 / 103 / 7**. | W1b | ✅️ coordinator | ☑️ |
| D9 | `IoPayload` crosses the WIT as JSON, so `Binary(Vec<u8>)` serializes as a JSON array of numbers — fine for small payloads, a real blowup for large binary artifacts. Needs a `DslValue::Bytes` variant or a binary framing. | W1-D | post-W6 | ☐ |
| D10 | WIT wire params are `source`/`target`, not `from`/`into` — `from` is a reserved WIT keyword. Rust/TS internals still say `from`/`into`. Not a defect; recorded so nobody "fixes" it back. | W1-D | — | n/a |

## ⚠️ Rejected approaches (do not re-propose)

- **Additive dual registration** (register an artifact through BOTH the old `assembly()`/`📇️registry` channel and the
  new declaration tree). Proposed by W2-P as "Option A". **Rejected**: that is a compatibility layer, which CLAUDE.md
  forbids outright. The cutover is per-plugin and atomic — for stdio that means all 36 artifacts plus deleting
  `📇️registry` in one pass.
- **Mirroring the native codec under both `📥️import` and `📤️export`.** Not implementable (one trait impl per type).
  See the CORRECTION block in `📓️design.md` §1.
