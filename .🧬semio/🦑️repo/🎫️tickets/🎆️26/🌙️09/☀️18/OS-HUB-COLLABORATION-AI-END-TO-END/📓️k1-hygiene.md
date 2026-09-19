# K1 — hygiene slice

Slice K1 of ticket 26/09/18 OS-HUB-COLLABORATION-AI-END-TO-END, run 2026-09-19.
Scope: the small unowned items from `📓️g1-goal-gap-audit.md` §2 (#9, #11, #12, #17) and
`📓️g2-hub-depth-audit.md` §12 (#5, #6, #8, #9, #10). Every item was verified against the live tree
before any edit; three of the nine audit claims did not survive that verification and are corrected
below rather than "fixed".

Captures: `🗑️generated/k1-*.txt`. Scratch codemod: `🐍️k1-strip-energy-debug.py`.

---

## 1. G1 #9 — missing launch entry for `🌎️hub/🧪️tests/🧱️foundation-source`

**Verified state.** Confirmed missing, and worse than the audit said. The route contract is owned by
`🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json`'s `route` block, which demands
`launchName: "⚖️gate🧱️hub-foundations📐️source"`, `launchCommand: "bun nx run os-hub:foundation-source-check"`,
`launchGroup: "4_gate"`, `launchOrder: 411.10755`, and its test
(`🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts:379-386`) asserts **exactly one** matching row in BOTH
`.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`. The nx target itself already existed
(`🌎️hub/📦️packages/🦀️rust/📋️project.json:541`, `📜️script.ts:16585`) — only the launch rows were absent.

**Fix.** `.vscode/🧩️launch.seed.jsonc:2266-2275` — one new configuration appended after the last
`4_build` row. `.vscode/launch.json:4036-4045` — produced by the repo's own generator, never
hand-edited: `bun nx run @semio-tech/plugin-registry:generate`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts` copies the seed skeleton
verbatim and substitutes only `@generated:<variant>:<renderer>` placeholders).

**Proof.** Generator exit 0 (`k1-registry-generate.txt`). The gate's own assertion, replayed
standalone against the fixture with `jsonc-parser` (`k1-launch-row-assert.txt`):

```
.vscode/🧩️launch.seed.jsonc matches = 1 {"name":"⚖️gate🧱️hub-foundations📐️source",…,"presentation":{"group":"4_gate","order":411.10755}}
.vscode/launch.json         matches = 1 {"name":"⚖️gate🧱️hub-foundations📐️source",…,"presentation":{"group":"4_gate","order":411.10755}}
```

**The full `foundation-source-check` gate does not pass, for a reason that is not this row.** Running
`bun test ./🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts -t "launch registrations"` fails **earlier in the
same test**, at `:371`, on a peer's in-flight edit: `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts` now
exports `HUB_DEV_BINARY_TARGET`, which is not yet listed in that owner's `rootImports` in
`🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json` (`Expected -0 / Received +1`). That fixture line belongs
to whoever added the export; I did not touch it. My assertions are at `:379-386`, after that failure
point, which is why they are proven standalone above. Capture: `k1-foundation-launch-test.txt`.

**Wider finding (not fixed, out of one slice).** The `4_gate` presentation group has **zero** rows in
either file at HEAD (`git show HEAD:.vscode/launch.json | grep -c 4_gate` → 0), yet at least eight
schemas and tests under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/**` require `⚖️gate…` rows
in that group (`🧬️schema/🤝️package-language-kind-handoff/**`, `🧪️tests/🛟️transaction-recovery-authority`,
`🧪️tests/🔬️workspace-contract:5198`, `🧪️tests/🎟️reference-coverage-selection:130`,
`🧪️tests/🥤️rust-finite-target-consumption:214`, `🧪️tests/✍️rust-writable-path-authority:58`,
`↪️rust-divergence-callback:159`, `🖼️assets/🗺️testing-readme-coordinates`). `📜️script.ts:6856` also
refers to "`.vscode/launch.json`'s `⚖️gate…` entries" as if they exist. My row is now the **only**
`4_gate` row in the repo. The whole `4_gate` family is missing and is a separate, larger slice.

---

## 2. G1 #11 — stray `[DEBUG]` ignores and logs

**Verified state — the audit's specific claim is stale.** There is **no** `#[ignore = "[DEBUG] W3-1b probe"]`
anywhere in `✏️s/🔌️plugins/🔋️energy` (or anywhere in `✏️s`/`🌎️hub`/`🧰️framework`): a repo-wide
`rg '#\[ignore'` returns zero `[DEBUG]`-reasoned ignores. P1's Task 4 removed both. The one remaining
energy `#[ignore]` (`🔨️modules/⚡️simulation/⚙️engine/🏛️bestest/🧪️tests/🔬️unit/🦀️.rs:268`,
"full-year comparison against the committed EnergyPlus references; run explicitly") is deliberate and
documented three lines above it. **Nothing to un-ignore.**

**What the grep did find instead.** 2180 `[DEBUG]` occurrences across `✏️s`+`🌎️hub`+`🧰️framework`.
The overwhelming majority are inside `🧪️tests/**` and are the repo's *established* test-receipt idiom
(`eprintln!("[DEBUG] PCG dense CSR: …")`), not leftovers — removing them is neither this slice's call
nor a hygiene win. I removed only the ones that are genuinely temporary probes in **production** paths
and that no peer slice owns:

| file | what | action |
|---|---|---|
| `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs` | 27 probes, 23 of them `{ eprintln!("[DEBUG] begin_fault line 3282 stage {:?}", …); return self.begin_fault(); }` with **stale** line numbers — the W3-1b instrumentation G1 #11 was pointing at | removed |
| same file, `fn fault(error: &Error)` | the `&Error` argument existed *only* to feed the removed `eprintln!`; the returned `JobFault` has always been `RetainedJobPayload::empty(Fault)`, which is the repo-wide shape (`🏗️fem/⚙️engine/**` constructs it with no error at all) | parameter and its six `&Error::severe("…")` construction sites removed |
| `🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1423` | a `#[cfg(test)]`-gated `eprintln!("[DEBUG] GIS fixed-three assembly terminal failure=…")` | removed |
| `🌎️hub/🏗️bootstrap/🦀️.rs:4749, 4761, 5017` | three error-path diagnostics on artifact-creation recovery/execution | `[DEBUG] ` prefix dropped, message kept |

The last row is a deliberate judgement: those three are the **only** diagnostic on a silent recovery
failure path in a hub that G2 §9 measured as having zero `tracing::` call sites. Deleting them would
swallow the error outright; the root fix is real structured logging (G2 §12 item 3, a different slice),
so I removed the "temporary" marker and left the operator signal. Flagging this explicitly rather than
claiming a clean sweep.

**Proof.** `cargo check -p semio-s-artifact-energy-model` exit 0, 1 pre-existing
`unnecessary qualification` warning from a dependency, none from energy (`k1-energy-check.txt`) — the
warnings prove the crate really expanded and type-checked. `cargo test -p semio-hub --lib --no-run`
exit 0 (`k1-hub-lib-testbuild.txt`) proves the `cfg(test)` removal compiles.
`git diff -U0` of the energy file, with `[DEBUG]` lines filtered out, shows **only** the `Self::fault()`
signature change and `{ return self.begin_fault(); }` bodies — no logic moved.

**Honest gap.** `cargo test -p semio-s-artifact-energy-model --lib sim::` is **32 passed, 3 failed**
(`k1-energy-sim-test.txt`): `p7c1_weather_owner_is_exactly_admitted_never_grows_and_retries_maximum_plus_one`,
`p7c2_preview_typed_view_is_derived_from_canonical_wire_with_live_facility_total`,
`p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology`. I did **not** run them before my edit,
so I cannot claim a before/after measurement. The evidence that they are pre-existing is strong but
circumstantial: (a) the removed probes were committed at HEAD and did nothing but print which
`begin_fault` arm fired — i.e. someone was already debugging exactly these three failures; (b) the
diff is behaviour-preserving (`fault()` still returns `StepOutcome::Fault`, every `begin_fault` arm is
untouched); (c) the failing assertion is `matches!(outcome, StepOutcome::Fault(_))` at
`🧪️sim/🧪️tests/🔬️unit/🦀️.rs:716`, a path my edit cannot reach. **The energy simulation W3-1b failures
need an owner; they are not closed by this slice.**

Also unresolved and deliberately untouched: ~60 production-path `[DEBUG]` lines in plugins owned by
peer slices right now (`🧩️puzzle`, `✒️writer`, `🪐️space`, `🔱️trinity`, `🌀️procedural`, `💡️reasoning`,
`📐️cad`, `🌊️flow`, …), plus 384 in `🧰️framework` and 27 in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
(the latter are gate receipts, not leftovers). Per-owner counts are reproducible with the census
command recorded in §8.

---

## 3. G1 #12 — `🌉️mcp/README.md` tool count / documentation drift

**Verified state — the count was already correct.** `GATEWAY_TOOL_NAMES` is
`[&str; 27]` at `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:250-278`, and the README already read
"Twenty-seven stable tools" (a peer fixed this after G1 was written). I recounted the README's own
bullet list against that array: 3 discovery + 6 authoring + 2 history + 5 artifact + 7 inference +
4 jobs/UI = 27. **Correct, no change needed.**

**Real drift found and fixed instead.**

1. `README.md:41-48` — the resources sentence was wrong in six places: it claimed artifact
   sub-resources `snapshot`, `selection` and `diff` (none exist) and top-level `plugin`, `extension`,
   `transaction` and `audit` resources (zero hits for `semio://plugin|extension|transaction` anywhere;
   `semio://audit` survives only in a doc comment at `🧭️protocol/🦀️.rs:629`). Rewritten from
   `🧠️context/🦀️.rs:176-240` (`WorkspaceResourceRegistry::list`/`templates`),
   `🖥️ui/🦀️.rs:625-670`, `💡️inference/🦀️.rs:381` and `🏠️workspace/🦀️.rs:2101-2119, 2203-2216`, and it
   now also names the hub-origin `PLUGIN_UNAVAILABLE` behaviour and the per-document
   `descriptor`/`checkpoint` scope resources.
2. `README.md` Layout table — the row `🎬️actions` names a facet that **does not exist**; the mutation
   protocol lives in `🔀️dispatch` (whose module doc still opens with the 🎬️ emoji and the words "The
   mutation protocol"). Row corrected, and six real facets that the table silently omitted were added
   (`🗿️artifact`/`🖥️ui`, `💡️inference`, `📇️registry`/`💬️prompts`, `🧪️conformance`/`⚠️errors`, `🏗️bootstrap`).
3. The same stale `🎬️actions` name appeared in four Rust docstrings — corrected at
   `🌉️mcp/🏠️workspace/🦀️.rs:590` and `🌉️mcp/🛡️policy/🦀️.rs:4, 8, 193`.

Verified-still-true README claims left alone: `.mcp.json` really exposes exactly `repo` and `semio`;
the HTTP port really defaults to 6300 (`🚚️transport/🦀️.rs:164`).

**Proof.** Source recount above (file:line). No build was needed for the README; the four docstring
edits are inside `//!`/`///` comments and are covered by the `cargo check -p semio-hub` runs only
indirectly — they are comment-only and cannot change compilation.

**Note.** A peer expanded this README (approval lanes, `🛰️rendezvous`) while I was editing; my rows
merged cleanly and are present in the current file at `README.md:41-48` and `:138`.

---

## 4. G1 #17 — dead `ui.chat.*` translation keys

**Verified state.** Confirmed dead. The `ui.chat` group (`readyFor`, `localOnly`, `instructions`,
`placeholder`, `savedLocally`, `send`) existed in exactly three places and nowhere else in the repo:
the type at `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx:426-433` and the German and English
tables at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:2901` and `:3743`. A repo-wide
`rg 'savedLocally'` (all file types, generated `.d.ts` included) returns those two files only.

**Not removed, and why.** Two same-named things are still live and were deliberately kept:
`ui.panelToggle.chat` (`📚️I18n/🟦️.tsx:169`, the chat-panel toggle button) and
`childElementId("ui.chat", "send")` (`🆔️ElementId/🟦️.tsx:50`, `🛂️manifest/🦀️.rs:1748` and their tests)
— the latter is an **element id** example, not a translation key, and M2's live `💬️AgentChatPanel`
reads a different namespace entirely (`agentUiLabel("os.agent.chat.placeholder")`).

**Fix.** Type group removed from `📚️I18n/🟦️.tsx`; both label tables removed from `🎯️targets/⚛️react/🟦️.tsx`
(brace-matched removal, asserted to hit exactly 2 blocks, each asserted to contain `readyFor` and
`savedLocally` before deletion).

**Proof.** `rg 'savedLocally|readyFor'` over both files → no matches. Both files transpile clean
through `Bun.Transpiler({loader:"tsx"})`.

**Honest gap.** I did **not** run a TypeScript typecheck over the consumers — that surface is T3/T4's
slice and is at ~865 pre-existing renderer errors, so a `tsc` run proves nothing about my three-block
deletion. The grep is exhaustive (all extensions, generated output included) and the type and the two
tables were changed together, so a dangling reference would have to be a dynamic string lookup; I found
none.

---

## 5. G2 #5 / #9 / #10 — dead-route and orphan trace

All three audit suspicions were **refuted**. Nothing was deleted, because nothing was dead.

### G2 #9 — `/execution-target/component` and `/execution-target/browser-actor` are NOT dead
Both have real production callers. The audit missed them because it grepped only the Rust directory
client (`📇️directory/🔌️client/🦀️.rs`, which indeed only issues `manifest` at `:989` and `descriptor`
at `:1015`). The other two are issued from the **browser store worker**, in TypeScript:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts:1454` — `browserExecutionTargetAssetRequest(binding, …, "component", intent, options)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts:1165` — the same call with `"browser-actor"`

both through `browserExecutionTargetAssetRequest` (`:1231-1242`), which POSTs
`/_semio/hub/spaces/{s}/documents/{d}/execution-target/{asset}` with a `DocumentOpenIntentV1` body and
an explicit four-asset allowlist regex. `DocumentExecutionTargetAssetV1` (`:914`) is literally
`"manifest" | "component" | "descriptor" | "browser-actor"`. **Refuted — document, do not delete.**

### G2 #10 — `🚀️local-relay/🧭️routing/🟦️.ts` is NOT orphaned
It is the admission allowlist of a real dev relay. `localRelayUpstreamPath` is imported and called in
production at `🌎️hub/📦️packages/🦀️rust/📜️script.ts:101` (import) and `:474` (the relay request handler,
behind the `x-semio-local-relay` shared-secret check at `:447`), and it is the single source both the
schema-drift test (`🌎️hub/🧪️tests/🧱️socket-grant-command-source/🟦️.ts:7`) and the live-route proof
(`📜️script.ts:3381`, `:3435-3440`) hold the hub against. The file is 68 lines because it is an
allowlist, not because it is a stub. **Refuted — document, do not delete.**

### G2 #5 — `POST …/checkpoint-publications` has no *client* caller, but is not dead code
There is genuinely no caller in the os product. There **is** a live end-to-end gate: the
"checkpoint publication process" proof at `🌎️hub/📦️packages/🦀️rust/📜️script.ts:908-935` uploads both
blobs, POSTs a real `semio.hub.checkpoint-publication-command/v1`, asserts the exact
`semio.hub.checkpoint-publication-receipt/v1` fields back, and then **replays the identical request**
and asserts a byte-identical durable receipt. So the route is the server-authoritative publication
write path with an idempotency contract and a passing proof — not orphaned code that a greenfield
sweep should delete. The honest statement, which the audit could not reach statically, is: *the route
is exercised and correct; the os product simply publishes over the document WebSocket today and has
not yet grown an HTTP publication client.* **I did not delete it, and I did not wire it** — wiring a
client is a feature decision, not hygiene. It belongs to whoever owns H2 §B.5 item 9's module split.

**Hub router edits.** None of the above required a router change, so the only router edit in this
slice is §6's one line — re-read immediately before writing, as H1/W3b/AU1 are live in that file.

---

## 6. G2 #8 — `/healthz` liveness route

**Verified state.** Confirmed: `/readyz` only (`🏗️bootstrap/🦀️.rs`, `get_readyz`), no liveness split.

**Fix.**
- `🌎️hub/🏗️bootstrap/🦀️.rs` — new `HubLivenessV1 { schema, status, run_id, uptime_ms }` (camelCase
  serde, schema `"semio.hub.liveness/v1"`), a `HUB_PROCESS_START: LazyLock<Instant>`, and
  `get_healthz`, placed immediately above `get_readyz`.
- same file, `fn router` — `LazyLock::force(&HUB_PROCESS_START)` so uptime is measured from boot and
  not from the first request, and `.route("/healthz", get(get_healthz))` ahead of `/readyz`.

The handler deliberately reads **nothing** but the run id and the process clock: an orchestrator must
restart a wedged hub and must never restart one that is merely still warming up, so a not-ready hub is
`503` on `/readyz` and `200 live` on `/healthz` at the same instant.

**Fix (test).** `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — `healthz_reports_liveness_on_a_process_whose_readyz_still_refuses`,
inserted after the `raw_http_get` helper. It spawns the default `test_state()` (whose
`artifact_authority_ready` is false, so `/readyz` is `503`), asserts `/readyz` 503 **and** `/healthz`
200 on the same address, checks the four body fields, asserts the body carries **no** readiness
subsystem keys, and asserts `uptimeMs` is monotonic across two calls.

**Proof — runtime, not static.**
- `cargo check -p semio-hub --bin os-hub` → exit 0, 7 warnings (`k1-hub-check-default.txt`).
- `cargo test -p semio-hub --bin os-hub healthz_reports_liveness -- --nocapture` → exit 0,
  `test tests::healthz_reports_liveness_on_a_process_whose_readyz_still_refuses ... ok`,
  `1 passed; 0 failed; 95 filtered out` (`k1-hub-healthz-test.txt`).

**Note.** The first attempt to run this test failed to compile on two **pre-existing** `E0382`
borrow-after-partial-move errors at `🔬️bin-unit/🦀️.rs:6557` and `:6559` (unrelated socket asserts). A
peer fixed both with `ref` bindings while I was reading them, so I did not edit that hunk; the rerun
is the capture above.

---

## 7. G2 #6 — `semio-hub` postgres / neo4j feature compilation

Both run foreground, one at a time, `-p` scoped, default features off.

| command | exit | `^error` lines | evidence the drivers really compiled |
|---|---|---|---|
| `cargo check -p semio-hub --no-default-features --features postgres` | **0** | 0 | `Checking sqlx-core v0.8.6`, `Checking sqlx-postgres v0.8.6`, `Checking sqlx v0.8.6`, then `semio-framework-os-kernel-db` and `semio-hub` re-checked; 44 crates checked, finished in 49.24s |
| `cargo check -p semio-hub --no-default-features --features neo4j` | **0** | 0 | `Checking neo4rs v0.8.0`, then `semio-framework-os-kernel-db` and `semio-hub` re-checked; finished in 54.75s |

Captures: `k1-hub-check-postgres.txt`, `k1-hub-check-neo4j.txt`. Both emit the same 7 `semio-hub`
warnings as the default build, which proves the crate really expanded rather than being served from
cache. **G2 §12 item 6 is closed: both backends compile clean today. Nothing to fix.**

Not proven by this (and not claimed): neither backend was *run* against a real Postgres or Neo4j
server — this closes the compilation question the audit asked and nothing beyond it.

---

## 8. Files changed

Source:
- `.vscode/🧩️launch.seed.jsonc` — new `⚖️gate🧱️hub-foundations📐️source` configuration
- `.vscode/launch.json` — regenerated by `bun nx run @semio-tech/plugin-registry:generate` (never hand-edited)
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs` — 27 `[DEBUG]` probes removed, dead `&Error` parameter and its 6 construction sites removed
- `🌎️hub/🏗️bootstrap/🦀️.rs` — `HubLivenessV1` + `HUB_PROCESS_START` + `get_healthz` + `/healthz` route; three `[DEBUG] ` prefixes dropped
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — `healthz_reports_liveness_on_a_process_whose_readyz_still_refuses`
- `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` — `cfg(test)` `[DEBUG]` probe removed
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md` — resources paragraph rewritten from source; Layout table corrected and completed
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`, `…/🛡️policy/🦀️.rs` — stale `🎬️actions` facet name → `🔀️dispatch`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx` — dead `ui.chat` type group removed
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` — dead `ui.chat` de + en label tables removed

Ticket folder:
- `📓️k1-hygiene.md` (this report), `🐍️k1-strip-energy-debug.py`
- `🗑️generated/k1-registry-generate.txt`, `k1-hub-check-default.txt`, `k1-hub-healthz-test.txt`,
  `k1-hub-check-postgres.txt`, `k1-hub-check-neo4j.txt`, `k1-hub-lib-testbuild.txt`,
  `k1-energy-check.txt`, `k1-energy-sim-test.txt`, `k1-foundation-source-check.txt`

`[DEBUG]` census command (reproducible):
`rg -n --glob '!node_modules' --glob '!*/🎫️tickets/*' --glob '!dist' -e '\[DEBUG\]' ✏️s 🌎️hub 🧰️framework | grep -v '🧪️tests'`

---

## 9. Honest gaps

1. **Three energy simulation tests fail** (`sim::tests::p7c1_weather_owner_…`, `p7c2_preview_typed_view_…`,
   `p7c2_restored_commit_bytes_…`). Evidence they predate me is strong (§2) but I never measured the
   before-state. **Needs an owner.**
2. **No TypeScript typecheck** behind the `ui.chat` deletion (§4) — grep + transpile only.
3. **The `4_gate` launch group is missing wholesale** (§1); I added the one row my slice names and left
   the other ~8 families that schemas and tests require. **Needs a slice.**
4. **`checkpoint-publications` has no os-product client** (§5) — confirmed, not resolved. Wiring one is
   a feature decision, not hygiene.
5. **~470 production-path `[DEBUG]` lines remain** outside energy and the hub (§2), almost all inside
   plugins that B1a/B2b/B3a–d hold right now. Not touched on purpose.
6. **`/healthz` was proven by an in-process axum test**, not against a booted `os-hub` binary over a
   real socket; `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts:222` still polls `/readyz` only, which is
   correct (it wants readiness), so no caller was changed.
7. **The `foundation-source-check` gate is red on a peer's fixture drift, not on my row** (§1): the
   fixture's `rootImports` for `🚀️local-bootstrap/🏃️execution/🟦️.ts` is missing the newly exported
   `HUB_DEV_BINARY_TARGET`. One fixture line, owned by whoever added that export. My two launch rows
   are proven standalone by the gate's own assertion (`k1-launch-row-assert.txt`). The long-running nx
   invocation of that gate was stopped by pid once the direct assertion had answered the question;
   `k1-foundation-source-check.txt` therefore holds only nx graph chatter.
