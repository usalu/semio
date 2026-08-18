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

## ⏱️ Agent turn discipline — PUT THIS AT THE TOP OF EVERY AGENT BRIEF

**Three agents lost work to this before it was written down. It must appear in the brief itself, not only here —
an agent launched before this file was updated never sees it, and a mid-flight `SendMessage` usually arrives too
late.** Paste this verbatim into every future dispatch:

> Do not end your turn waiting on a Monitor or a background build — your background children are killed when your
> turn ends, so the build dies instead of completing. Block inside a single Bash call with `timeout: 600000` (the
> 10-minute max). If it times out, re-issue the identical command; cargo caches, so it resumes rather than
> restarts. Repeat until it returns. `Blocking waiting for file lock on build directory` is expected while sibling
> agents run — keep re-issuing, do not treat it as failure. Stay in ONE continuous turn until your report is
> written. If you run out of budget with edits done but verification incomplete, write the report anyway with
> honest partial numbers, clearly labelled — never claim a test passed that you did not observe pass.

## ⏱️ Agent turn discipline (learned the hard way, three times)

**Never end your turn waiting on a background build.** When an agent's turn ends, its background children are
killed and the work is lost. The stdio repair agent did exactly this — kicked off a baseline `cargo check` in the
background, armed a monitor, and ended its turn "waiting to be resumed". Nothing was waiting; the build died.

Instead, **block inside a single Bash call** and pass a long `timeout` (up to 600000 ms):

```
cargo check -p <crate> --all-targets --keep-going 2>&1 | tee /tmp/base.txt | grep -cE '^error'
```

If it exceeds the timeout, re-issue the same command — cargo caches, so the second run resumes rather than
restarting. Keep working in one continuous turn until done.

Under heavy fan-out the machine carries 90+ concurrent cargo processes; `Blocking waiting for file lock` is normal
and expected, not a failure. Be patient rather than retrying in a tight loop.

## 🚦 Concurrency cap — a shared `CARGO_TARGET_DIR` SERIALIZES every agent

**Do not run more than ~3 build-heavy agents at once on this ticket.**

Every agent is told to use the same `CARGO_TARGET_DIR=<ticket>/🎯️target`. Cargo takes an **exclusive lock on the
build directory**, so N concurrent agents do not build in parallel — they queue. Each agent waits behind the others
for *every one* of its many builds, so wall-clock per agent grows with N while total throughput stays flat.

**Evidence (corrected).** The honest evidence is: repeated `Blocking waiting for file lock on build directory`
messages in agent output, and single `cargo check`/`nextest` invocations taking 30–60 minutes that normally take
2–3, plus 11 agents dispatched yielding 2 reports. An earlier note in this file claimed "133 cargo processes" —
**that number was wrong**: `ps aux | grep '[c]argo'` also matches the `/bin/zsh -c source …` wrapper of every shell
call, which inflated it by ~42. The real figure at the same moment was ~67 cargo+rustc processes across a handful
of genuinely-running builds. The conclusion stands on the lock messages and the wall-clock, not on the process
count.

The original plan noted "cargo flock serializes; `Blocking waiting for file lock` is normal". That is true and
harmless at 2–3 agents. At 11 it is pathological, and it was a coordinator error to read "disjoint file boundaries"
as "safe to run 11 at once" — **the build directory is a shared resource the ownership table never modelled**, the
same blind spot as workspace membership.

Options, in order of preference:
1. **Cap concurrency at 3.** Keeps cache reuse (the reason to share a target dir at all) with tolerable waiting.
2. Per-agent `CARGO_TARGET_DIR` — genuinely parallel, but each agent rebuilds the whole dependency tree
   (~10 min and tens of GB each). Only worth it for a long-running agent on a big crate, e.g. the stdio repair.
3. Serialize deliberately: one agent at a time, which is slower in agent-count terms but has the shortest
   wall-clock per plugin.

Do NOT kill running agents to enforce this — they lose their work. Let the queue drain, then dispatch at the cap.
