# M5br — semio MCP protocol conformance (takeover of the killed M5b)

Successor to `📓️m5b-mcp-protocol-conformance.md`. That report's §1–§6 describe the code M5b
landed before it was cut at ~01:45 on 2026-09-20; its §7 ("Tests and transcript") was never
filled because the worker died inside its first `cargo` run (`🗑️generated/m5b-cargo-1.txt` is
0 bytes). **M5br's job is to make those claims measured**: compile the crate, run the tests,
drive the staged stdio binary from a scripted client, and fix whatever is wrong.

Crate: `semio-framework-os-mcp` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp`).
Live siblings in the same crate while this ran: R2 (reactor chain), M5a (catalog/audience/
destructive), M6 (hub principal, `🤖️agent-credential/`).

## 1. Inherited state at 02:28 — the crate was RED (lib test), lib itself green

`cargo check -p semio-framework-os-mcp --all-targets --keep-going`
(`🗑️generated/m5br-check-1.txt`): **lib compiled with warnings only; `lib test` failed with 12
errors.** Whose:

| error | count | owner |
|---|---|---|
| `ArtifactChannels` imported from the private `crate::actions` re-export (`📣️notify/🧪️tests/🔬️quick/🦀️.rs:2`) | 1 | **M5b's** |
| `GisMapInferenceSubmitRequestV1::new` called with 2 args after M5b gave it a `service_id` first parameter (`💡️inference/🧪️tests/🔬️inference-jobs/🦀️.rs:141,346`) | 2 | **M5b's** |
| `CapabilityOwner::Plugin` missing the new `label` field (policy/artifact/inference `🔬️quick` fixtures) | 3 | a peer's (catalog `label`, `🗂️catalog/🦀️.rs:56`, landed 01:36, fixtures never updated) |
| `AppDefinition` missing the new app-wide `actions` field (`🧪️tests/🧱️source-builders/🦀️.rs` ×4) | 4 | a peer's (DS1 descriptor dedup, `🛂️manifest/🦀️.rs:3693`) |

M5b's three were fixed first. The peers' seven were each a **single additive field** in a
test-only fixture (`label: None`, `actions: Vec::new()` — both semantically identical to the
pre-change behaviour) and they blocked every test in the crate, so M5br closed them rather than
wait; recorded here so their owners know.

All 12 were closed in source by 02:45. The compile that would prove it never got the build-dir
lock — see §2, which states exactly that rather than implying greenness.

**Session note (rule 23):** the account limit cut this worker at ~03:00 while its first
`cargo test` had been queued on the shared build-dir lock since 02:30 (killed at 06:12 by the
coordinator with the other 33). Re-run started 06:14.

## 2. Unit tests — NOT OBTAINED: build-dir lock starvation (honest)

`cargo test -p semio-framework-os-mcp --lib` was started three times and **never got the shared
build-dir lock**:

| run | started | outcome |
|---|---|---|
| `🗑️generated/m5br-test-1.txt` | 02:30 | queued behind the 4 h fleet deadlock; killed in the coordinator's 06:12 sweep (`exit=137`) |
| `🗑️generated/m5br-test-2.txt` | 06:14 | `Blocking waiting for file lock on artifact directory` for 88 min; killed by pid (10677) at 07:42 to requeue against the current tree |
| `🗑️generated/m5br-test-3.txt` | 07:43 | same line, still blocked at 08:36 (53 min), left running |

This is **not** the rule-23 deadlock signature: real `rustc` processes ran throughout (4–53 at
every check, with live rustc children under `cargo test -p semio-s-plugin-process`,
`cargo rustc -p semio-s-plugin-stdio` and two more), so the build dir was busy, not wedged — my
job simply never won the lock. ~15 cargos were queued on it at 08:36, including M5a's
`--lib m5a` and M6's `--lib agent_credential` on this same crate.

What that leaves proven and unproven:
- **Proven:** the whole protocol surface, at runtime, against the real staged binary — §3, 23/23.
- **Proven at 02:28:** the crate's **lib** compiles (`🗑️generated/m5br-check-1.txt`, warnings only);
  the 12 errors were all in `lib test`.
- **Unproven:** that the 12 fixes compile, and that M5b's 21 new Rust oracles in
  `📣️notify/🧪️tests/🔬️quick/🦀️.rs` pass. Each fix is a mechanical one-liner and all seven
  peer-owned ones were verified by diff (`actions: Vec::new()` at lines 179/291/363/503 of
  `🧪️tests/🧱️source-builders/🦀️.rs` — each app's window kinds keep their own rosters, e.g.
  `draw_app`'s `actions: draw_actions()` at line 494 — and `label: None` in the three
  `CapabilityOwner::Plugin` fixtures), but **a diff is not a compiler** and this report does not
  claim otherwise. The next worker on this crate should read `🗑️generated/m5br-test-3.txt`
  first: if it ever completed, its result is the missing row.

## 3. Measured: stdio conformance run against the staged binary — 23/23 green

Harness: `🐍️m5br-conformance-probe.ts` (ticket folder). It spawns the `semio` server with the
**literal `command`/`args` out of `.mcp.json`** (`bun ./📜️script.ts dev mcp stdio os --folder .
--scopes workspace.read,artifact.write,inference.execute,ui.observe,ui.control`) plus the two
Claude-Code harness variables, and drives newline-delimited JSON-RPC over stdio exactly as
Claude Desktop / Claude Code do. It collects **server-initiated notifications** (the envelopes
with no `id`), which is what makes the push assertions possible at all. It does not touch the Go
`repo` server, so a repo-MCP outage cannot mask or fail this gate — that is the one deliberate
difference from `🌉️mcp/🟦️.ts`'s `client-e2e`, whose `runMcpClientEndToEnd` runs both servers.

Run at 06:27 against the staged `dist/build/semio-os-mcp` (built 02:06 from this tree) —
`🗑️generated/m5br-probe-3.txt`, `exit=0`, **23/23 rows green**:

| conformance item | measured |
|---|---|
| protocol version negotiation | `initialize` returns the exact version asked for all three of `2026-07-28` / `2025-11-25` / `2025-06-18`; an unsupported `1999-01-01` is answered with `2026-07-28` (the spec's "offer one you do support"), not an error |
| advertised capabilities | `{"prompts":{"listChanged":true},"resources":{"listChanged":true,"subscribe":true},"tools":{"listChanged":true}}` — and `subscribe` is now backed (below) |
| `tools/list` pagination | walked to exhaustion: 1 page, 27 tools, order identical to the unpaged list |
| foreign cursor | `tools/list` with a cursor this server never minted → JSON-RPC `-32602 unknown cursor: someone-elses-cursor` |
| `resources/list` | 1 page, 9 resources, **0 duplicate URIs** (G7 §6 P2.11 closed and now guarded) |
| `resources/templates/list` / `prompts/list` | walked: 5 templates, 5 prompts |
| `outputSchema` / `inputSchema` | all **27** tools declare both |
| JSON-RPC error vs `isError` | unknown tool → JSON-RPC `-32602`; a real tool's own failure (`artifact_snapshot` on a missing id) → HTTP-200-equivalent result with `isError:true` and the `GatewayError` payload in `structuredContent`; a successful call carries `structuredContent` |
| `resources/subscribe` validation | `semio://there-is-no-such-resource` → `-32602 cannot subscribe to unknown resource` (was an unconditional `Ok(())` forever-wait) |
| `resources/subscribe` → `notifications/resources/updated` | subscribed to `semio://artifact/m5br-mu9bfgwr`, then drove `artifact_validate`/`artifact_snapshot`/`artifact_open`; exactly **1** `notifications/resources/updated` arrived, naming that exact URI, with no poll |
| `notifications/cancelled` | accepted, answered with nothing, server still answers `ping` |
| `notifications/progress` | `tools/call inference_run` with `_meta.progressToken`: **3 rows pushed MID-CALL**, `progress=0→0.05→0.35`, each carrying the client's token — the difference between a pushed stream and polling `job_get`, proven on the wire |

Two caveats about that transcript, stated rather than smoothed:
- `inference_run` ended `isError:true` (the GIS guest refuses the run without a bound document).
  The progress assertion is unaffected: the rows are pushed while the call is in flight, before
  its outcome exists. An earlier 02:48 run of the same probe had `inference_run` not return
  within 240 s at all under the deadlocked fleet, and the rerun's mid-call rows are what that
  hang was missing evidence for.
- `artifact_validate` answers `isError:true` on a freshly created probe artifact. Out of this
  slice (it is the generic-probe-document limit G7 §6 P1.5 names), recorded because the
  transcript shows it.

## 4. Fixes landed by M5br

| fix | file:line | why |
|---|---|---|
| `ArtifactChannels` imported from its owning module instead of `🔀️dispatch`'s private re-export | `📣️notify/🧪️tests/🔬️quick/🦀️.rs:2` | M5b's new test module did not compile at all, so none of its 21 tests could run |
| `GisMapInferenceSubmitRequestV1::new` call sites carry the resolved `service_id` | `💡️inference/🧪️tests/🔬️inference-jobs/🦀️.rs:141,346` | M5b gave `new` a leading `service_id` (descriptor-routed quartet, its §5) and left the two hub-job tests on the old 2-argument form |
| `label: None` added to three `CapabilityOwner::Plugin` test fixtures | `🛡️policy/🧪️tests/🔬️quick/🦀️.rs:10`, `🗿️artifact/🧪️tests/🔬️quick/🦀️.rs:20`, `💡️inference/🧪️tests/🔬️quick/🦀️.rs:20` | a peer's new `label` field (`🗂️catalog/🦀️.rs:56`) left the crate's whole test target red; `None` is exactly the pre-change behaviour |
| `actions: Vec::new()` added to four `AppDefinition` literals | `🧪️tests/🧱️source-builders/🦀️.rs:179,291,363,503` | DS1's app-wide action roster (`🛂️manifest/🦀️.rs:3693`); the window kinds in these fixtures keep their own rosters, so an empty app-level roster is behaviour-identical |
| the conformance probe itself | `🐍️m5br-conformance-probe.ts` (ticket folder) | new — §3 |

The seven peer-owned fixture fields are flagged in §1 for their owners. Nothing in production
code was touched by M5br: M5b's protocol/notify/inference work is what §3 measures, unchanged.

## 5. Honest gaps

M5b's §8 gaps stand except where §3 narrows them:

1. **stdio cancellation is still serialized** (M5b §8.1) — unchanged, and now visible in the
   transcript: the 02:48 run's `inference_run` blocked the connection for the whole 240 s budget,
   so a `notifications/cancelled` sent during a blocking call is only acted on when that call
   returns. What §3 *does* prove is that `notifications/progress` reaches the client **mid-call**
   (rows at 0.05 and 0.35 arrived while the call was still in flight), so the notification lane
   itself is not blocked by the request lane — only inbound dispatch is.
2. **Six tools' `outputSchema` is the generic object envelope** (M5b §8.2) — the probe proves all
   27 declare one and that successful results carry `structuredContent`; it does not prove those
   six are typed. Unchanged follow-up.
3. **Resource updates are published from the tool-result table, not from the store**
   (M5b §8.3) — unchanged. §3 proves the tool-call path end to end; a peer's edit arriving
   through the workspace still publishes nothing.
4. **The hub inference routing table has one row** (M5b §8.4) — unchanged; `inference_list`
   answers 6 declared services but only the GIS Map quartet has a hub transport.
5. **The e2e in `🌉️mcp/🟦️.ts` was not run.** M5b extended `runOsMcpClientJourney` with the same
   four probes; `client-e2e` runs it together with the Go `repo` server's journey, which this
   fleet's own memory records as frequently unreachable. §3 measures the identical assertions
   against the same binary without that dependency. The `🟦️.ts` additions are therefore **written
   but not executed** — said plainly rather than implied by the probe's greenness.
6. The probe ran against `dist/build/semio-os-mcp` staged at 02:06, which contains M5b's
   protocol work but predates nothing of it. It was not rebuilt afterwards (the build-dir lock);
   §2's `cargo test` compiles the same sources.
7. Everything here is in the working tree, not merged (the standing INF-2 risk).

## 6. Files changed by M5br

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📣️notify/🧪️tests/🔬️quick/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🧪️tests/🔬️inference-jobs/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🧪️tests/🔬️quick/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗿️artifact/🧪️tests/🔬️quick/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🛡️policy/🧪️tests/🔬️quick/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🧱️source-builders/🦀️.rs`
- `.🧬semio/…/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️m5br-conformance-probe.ts` (new)
- captures: `🗑️generated/m5br-check-1.txt`, `m5br-probe-1.txt`, `m5br-probe-3.txt`, `m5br-test-2.txt`

M5b's own production files (§9 of `📓️m5b-mcp-protocol-conformance.md`) are unchanged by M5br.
