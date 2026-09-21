# G15 — deploy/production-readiness re-audit of G12's 13 items against the CURRENT tree

Read-only (Sonnet 5), 2026-09-21, session 7. No builds, no servers, no edits outside this file. Method:
read `📓️g12-deploy-and-onboarding-audit.md` (13 ranked items, 2026-09-20 baseline), `📓️p4-hub-and-mcp-production-readiness.md`,
`📓️d2-docs-and-first-run.md`, `📓️ob1r-trace-and-hub-observability.md`, `📓️au1-hub-auth-sessions-and-rate-limit.md` §1,
`📓️hs1-hub-pool-worker-stack-overflow.md` §4/§6, `📓️jc1-jco-task-return-and-republish.md` §6, then re-read every file
those reports cite (`🌎️hub/🏗️bootstrap/🦀️.rs`, `🌎️hub/🗄️stores/🦀️.rs`, `🌎️hub/README.md`, `🌎️hub/Dockerfile`,
`🌎️hub/compose.yaml`, root `📜️script.ts`, the MCP crate's `📜️script.ts`/`🎚️config/🧱️binary-gate.json`,
`🧰️framework/…/🌉️mcp/README.md`, root `README.md`, `.mcp.json`, `.vscode/launch.json`) directly with `grep`/`cat`, plus
`git log`/`git show --stat` to check whether each report's claimed edit actually landed and survived to the current
tree. `📓️status.md` (session 5–7) and `📓️g16-hub-backend-reaudit.md` (parallel audit, hub *backend* depth, not
deploy — read to avoid duplicating it) fill in the current live-fleet state.

**Headline**: P4/D2/OB1r/HS1 collectively closed 8 of G12's 13 items for real, in source that is in the tree today
(re-verified independently below, not by trusting the reports). Production mode **is now reachable** —
`identity_verifier` is still hard-coded `None`, but `validate_auth_startup` no longer requires it; credential
sign-in is an accepted alternative identity authority, and this is real, unit-tested code, not a plan. One claimed
deliverable — the root `README.md` "Products" getting-started section D2's report describes writing — **is not in
the tree**: no commit since 2026-06-04 touches `README.md` and there is no working-tree diff either. Three items
(#1 destructive capabilities, #7 `s` release build, #8 `--hub` caller) have moved past what G12/P4 described, in
different shapes than the ranked list expected. One brand-new production correctness bug outside G12's original 13
was found and fixed (HS1's pool-worker stack overflow) — it would have `SIGABRT`ed any standalone (non-cargo-launched)
`os-hub`, which is exactly the production topology this ticket just made reachable.

---

## Per-item status (G12's 13, current tree)

### #1 — one real `destructive: true` capability + descriptor regen (ranked S, "unblocks M7")

**DONE, and overdelivered far past "one".** Not M5a/P4's doing alone — a real, non-test builder method
`App::action_destructive(action_id)` exists (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5139`, doc:
*"Marks one already-declared action or command destructive… raises `policy.approval` to `WhenDestructive`"*) and is
called from **~50 real plugin editor sources** (grep `\.action_destructive(` outside `🧪️tests`/tickets — e.g.
`✏️s/🔌️plugins/🕸️dag/…/✏️editor/🦀️.rs:870`, `🔱️trinity`, `📸️remodel`, `🖨️raster`, all 15 `📕️norm` Eurocode
artifacts, `📐️cad`, `🧱️block`×3, `🎬️sequence`, `✒️writer`, `🪐️space`×3, `🌀️procedural`×3, `🌍️gis`, `🀄️wfc`×3,
`📜️imperative`, `🪵️sourcing`, `🗒️note`, `📋️forms`, `🏛️architect`, `🎥️shooting`, `➗️mathematical`, `🧩️puzzle`×3,
`🏗️fem`×2, `🖍️draw`, `💠️lowpoly` — 50 files). Committed descriptor JSON confirms it landed: 25 plugin `🔣️.json`
files now carry `"destructive": true` (`grep -rl` count), and the last commit's diff (`git log -p -1 -- …trinity/🔣️.json`)
shows these as newly added lines, e.g. `deleteSelection` in `🔱️trinity`. A3b's status-line entry (session 5d,
22:25) independently confirms the count moving: *"destructive 25 → 44"*, later *"destructive 44 → 82"* (CE1) — this
kept growing through the fleet, well past G12's "one".
**What "working" means for a user**: unchanged from M4/M7's own work — an agent's destructive call now actually
gets an `ApprovalMode::WhenDestructive` gate instead of sailing through `Never`. Not independently re-verified live
in this pass (that is M7/CE1's job); the source-level wiring is real and committed.

### #2 — `os-hub:publish` / real `publish <slice>` map entries

**DONE**, re-verified directly in `📜️script.ts:15074-15077`: `PublishScript`'s map is
`{"os-hub": "os-hub:publish", "os-mcp": "@semio-tech/framework-os-mcp-rs:publish"}` (was `{}` at G12). Each target
depends on its project's release build and writes `<name>-<version>-<platform>-<arch>.tar.gz` + `.sha256` via
`packageNativeRelease` (`🏗️native-build/🟦️.ts`), which P4 ran end-to-end against a **stand-in** Mach-O binary
(rm+cp+codesign+`codesign --verify` all measured) but never against a real release binary — no real tarball has
ever been produced, because that needs the release compile this whole session's load ceiling forbade.
**What "working" means**: `bun ./📜️script.ts publish os-hub` produces a checksummed local tarball. It does — for a
stand-in binary. Nobody has run it against the real `os-hub:build` output. Nothing uploads anywhere (by design,
documented in the code and the hub README).

### #3 — `semio-os-mcp` `--release` build target, drop the debug-only assertion

**DONE (wiring), unrun (the actual compile).** Re-verified in
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:34,112-121,700-702`: the debug-only
assertion is gone, replaced by `profiles.build === "debug" && profiles["build-release"] === "release"`;
`BuildReleaseScript` mirrors `os-hub`'s `BuildScript` (`--release --package … --bin …` → `dist/build-release`,
re-signed). `🎚️config/🧱️binary-gate.json` now has a `profiles` map, not a single `profile: "debug"`. Launch rows
`📦️build-release🌉️os-mcp` / `🚚️publish🌉️os-mcp` exist in both `.vscode/launch.json` and
`.vscode/🧩️launch.seed.jsonc`. No release `semio-os-mcp` binary has ever been produced as a file (unrun, stated as
such by P4).
**What "working" means**: `bun nx run @semio-tech/framework-os-mcp-rs:build-release` produces
`dist/build-release/semio-os-mcp`, which the MCP README's Claude Desktop config now points at. The command exists
and type-checks; it has never been executed.

### #4 — hub Dockerfile + compose example

**PARTIAL — authored, unbuilt, and now stale against #A's own fix.** `🌎️hub/Dockerfile` and `🌎️hub/compose.yaml`
exist (re-read in full above), both explicitly self-labeled "UNBUILT" in their header comments — no `docker` binary
exists on this machine, so neither has ever been `docker build`-ed or `docker compose config`-ed. Both target the
**development** topology (`bun nx run os-hub:dev`, `OS_HUB_MODE=development`, fd-3 launcher, whole repo copied into
the image) because they were authored *before* the production-mode fix (§+A below) landed. Now that a bare
`os-hub` binary can boot standalone in production mode, this Dockerfile's entire "why the image carries the
repository" rationale (its own longest comment block) is obsolete for the production case — P4's own honest-gaps
§9.5 says so: *"a far smaller binary-only image is possible… superseded for production mode… still correct for
the dev topology."* Nobody has reworked it since.
**What "working" means**: `docker build -f 🌎️hub/Dockerfile -t semio/os-hub .` succeeds and `docker run` gives a
`/readyz`-answering hub. Unverified in both directions (build success and, if it built, whether it boots) — and the
file that exists targets the wrong (dev) topology for what a production Dockerfile should now be.

### #5 — operator-facing hub deploy doc

**DONE, substantially.** `🌎️hub/README.md` exists (556 lines, re-read in full above; did not exist at G12). Covers:
the two-topology gate table (accurate against current `validate_auth_startup` source, re-verified line by line),
all ~30 `OS_HUB_*` env vars with defaults, data-root layout, the `credential set`/`trusted-catalog publish` operator
verbs, `/healthz`/`/readyz`, a backup/restore runbook (`tar` the whole `OS_HUB_DATA`, SIGTERM-drain-then-copy), an
explicit "no cross-version upgrade path" callout quoting the postgres module's own design comment, and a systemd
unit that runs the **release binary directly in production mode** (not the dev launcher — this section was
rewritten by D2 in session 5b after P4 landed the production-mode fix, and it is correct against the current gate).
**What "working" means**: an operator can read one file, top to bottom, and get a real server up behind a proxy.
The doc delivers this. What it cannot promise: nobody has run any of these commands (§+A is the one exception).

### #6 — `SIGTERM`/`SIGINT` → the existing drain

**DONE and live-observed.** Re-verified in `🌎️hub/🏗️bootstrap/🦀️.rs:9926-9960`: `TerminationSignalV1`,
`first_termination_signal`, `axum::serve(…).with_graceful_shutdown(…)` at the serve site (was absent at G12 — 0
`tokio::signal` hits). P4's own live production-runtime observation (§8d, step 5) recorded a real `SIGTERM` against
a real running `os-hub` producing `{"event":"server.readiness","outcome":"cancelled",…}` then **exit status 0**.
That capture file (`🗑️generated/p4-production-runtime.txt`) no longer exists on disk — the user's 02:30 disk clean
(`📓️status.md` session 5e "the USER cleaned the machine") wiped `🗑️generated/` entirely — so this pass could not
re-read the transcript, only confirm the code that would produce it is real and present. Two unit laws exist and
are cited passing in the coordinator's 09:17 nextest run (`a_termination_signal_prefers_the_orchestrators_verdict`,
`a_termination_signal_stops_the_listener_and_then_drains_in_flight_work`).
**What "working" means**: `docker stop`/`systemctl stop` drains in-flight requests instead of hard-killing. Proven
once, live, per P4's report; not independently re-provable from this pass because the transcript is gone; the code
is real and the hub suite is green in session 7 (320/321 at 03:46).

### #7 — one end-to-end `build-s-react-release` run, served and opened

**Still OPEN**, and the ranked slice's plan ("shares S2's root fix") turned out correct in a way nobody has closed
yet. S2 landed a real *cold `dev s`* boot (session 5e, 60/60 wasm components staged and activated, served at
`127.0.0.1:6070`, later S7/S6 reached 35/35 spawnable kinds working inside that same host) — but `dev s` is the
**development**-profile target, not `build-s-react-release` (the release-profile distribution bundle G12 named).
`📓️status.md:206` records explicitly: *"#7 (`build-s-react-release` observed once) waits for S2's cold
activation"* — S2 finished (session 5e), and no later status line records anyone running the release build. `grep`
for `build-s-react-release` across the whole ticket folder returns only G12's own text and this one status
cross-reference. **Nobody has ever run the release bundle target, even after its blocking dependency was fixed.**
**What "working" means**: `bun nx run build-s-react-release` (or its nx-generated equivalent for the `s` variant)
completes, and the static output — hashed chunks, one manifest, `VITE_S_HUB_URL` baked in — is served and opened in
a browser exactly the way S2/S6/S7 already proved the *dev* server does. This is now the single largest remaining
gap for outcome B: the blocking defect is gone, the button has not been pressed.

### #8 — `--hub` credential-delegation caller from an authenticated session

**DONE, in a different (better) shape than the ranked item literally asked for.** G12 imagined "something in the
product spawns `semio-os-mcp --hub …`"; what actually landed (M6, surfaced by D2 §2.2/§8.2) is a **credential-file**
flow instead of a live fd-inheritance spawn, which is the correct shape for an *external* MCP client (Claude
Desktop/Code) that the product does not control the process tree of. Re-verified directly:
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:760,815` implements `--credential-file`/`--credential-fd`
(fd 0/1/2 refused); a real product UI, `🧧️AgentDelegations/🟦️.tsx` (302 lines,
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🤖️AgentDelegations/🟦️.tsx`), lets a
signed-in user create a delegation and download the credential file (`os.hub.agent.download` button,
`POST /auth/agent-delegations` per the MCP README's documented response shape). The MCP README's worked configs
(§ "Binding it to your work") show exactly how a `.mcp.json`/`claude_desktop_config.json` uses that file.
**What "working" means**: a signed-in hub user opens the AgentDelegations panel, downloads a credential file, and
points their MCP client's config at `--hub <url> --space <id> --credential-file <path>`. The pieces are all real
and committed; nobody in this ticket has driven that specific loop end to end with a live client (M7 proved the
`--folder`/local-bridge loop, not the `--hub`/credential-file one).

### #9 — CORS allowlist + `OS_HUB_BIND` documentation

**DONE and tested.** Re-verified in `🌎️hub/🏗️bootstrap/🦀️.rs:7954-8086`: `CrossOriginPolicyV1
{LoopbackDevelopment, Allowlist(Arc<[String]>), Closed}`, `from_environment`, `admits`, wired into
`cors_middleware` as `State`. `OS_HUB_ALLOWED_ORIGINS` is documented in `🌎️hub/README.md`'s "Cross-origin access"
section with the exact precedence table. Three unit laws are cited passing in the coordinator's nextest.
**Honest, and still true today**: because `validate_auth_startup` refused every non-loopback bind at G12 time,
the practical effect *today*, for a loopback dev hub, is only "no longer hands a credentialed grant to an arbitrary
origin" — the `Closed` posture for a naive network bind was a guard for a day that had not arrived. That day
**has** now arrived (§+A), so this item and §+A compose: a network-bound production hub now requires the allowlist.
**What "working" means**: set `OS_HUB_ALLOWED_ORIGINS` before serving `s` from a different origin than the hub.
Works as documented; not run against a live browser from a foreign origin in this pass (P4's unit laws cover the
socket-level behavior over a real TCP listener, not a real browser's CORS preflight).

### #10 — end-user-facing MCP install doc (worked `claude_desktop_config.json`, `--hub`/`--folder` explained)

**DONE**, and this is the single strongest outcome-C artifact in the whole re-audit. Re-read
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md:18-224` in full: a task→tool table in plain user terms, an
honest "no descriptor declares destructive yet" caveat (now stale per item #1's overdelivery — worth a follow-up
correction, see Honest gaps), `--folder` vs `--hub` explained with the real mechanical reason `--hub` needs a
credential file, **two copy-paste-ready worked configs** — Claude Code's `.mcp.json` (bun+cwd form) and Claude
Desktop's `claude_desktop_config.json` (absolute binary path, because Desktop takes no `cwd`) — both pointed at
`/Users/you/Documents/my-semio-space`, not the repo checkout, directly closing G12's "no worked example… outside
this repo" complaint. `grep -c claude_desktop_config` in this file: 3 (was 0 anywhere in the repo at G12).
**What "working" means**: copy the Claude Desktop block, change two paths, build `--release`, restart Desktop.
Genuinely works *as written* modulo item #3 (no release binary has actually been built yet, so the path the config
names does not exist on disk until someone runs that command once).

### #11 — TLS-terminating reverse-proxy example + systemd unit

**DONE.** `🌎️hub/README.md` has both a Caddy and an nginx example (both re-read above), each correctly forwarding
the three WebSocket-upgrade socket route families grepped from the real router
(`/directory/socket/v1`, `/directory/spaces/{space}/documents/{doc}/socket/v1`,
`/spaces/{space}/documents/{id}/socket/v1`), and a systemd unit that — after D2's session-5b rewrite — runs the
**release binary directly** in production mode with `KillSignal=SIGTERM`/`TimeoutStopSec=30s`/`ReadWritePaths`
matching the real drain and data-root story. None of the three (Caddy/nginx/systemd) has been loaded by a real
proxy or init system; they are correct-by-construction against the measured facts, not proven configs.

### #12 — first-run wizard inside `s` for hub sign-in/space creation

**DONE and live-observed**, reusing the existing `IntroductionDefinition` mechanism as required (no second
onboarding subsystem). Confirmed present in the current tree:
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎓️HubFirstRun` exists, last touched by the
same commit (`48b9d63cf6`) as the rest of this ticket's landed work — not orphaned. D2's report documents an
8/8-passing headless-Chromium probe against a real served shell + hub (`🐍️d2-firstrun-probe.mjs`), including a
375px-viewport check, with screenshots. Two of five steps (`space`, `invite`) were never observed live — they need
a signed-in session in a space, which D2's cargo-free slice did not set up.
**Known, real gap D2 flagged and left unfixed**: the tour's own anchor-resolution law only checks the id pattern,
not rendered DOM — this is exactly how the tour's first draft shipped as five centered screen steps pointing at
nothing (12/12 anchors were i18n label keys, not DOM ids) before D2 caught it by *running* the live probe, not by
reading the law. The live-anchor probe is not wired into any CI gate, so a future rename can silently break the
tour again with the unit suite still green.

### #13 — durable-store schema versioning (stamp + refuse-newer)

**DONE, tested, and live-observed** — the strongest-verified item in this whole list. Re-verified in
`🌎️hub/🗄️stores/🦀️.rs:248-320`: `STORE_FORMAT_FILE`, `STORE_FORMAT_VERSION = 1`, `open_store_format` wired into all
four store `open`s. P4's live production-runtime observation (§8d step 8) recorded hand-editing a real
`format.json`'s version to 2 and getting the hub to refuse the open with its own named message — the one item in
this whole audit proven by an adversarial live test, not just a unit law (though, per §6 above, that specific
transcript file is also gone from `🗑️generated/` after the disk clean; the source and its 5 unit laws, cited
passing in `🗑️generated/p4-stores-tests.txt`'s summary and the coordinator's nextest, remain in the tree).

---

## +A — scope addition: is production mode reachable at all today?

**Yes**, and this is the load-bearing finding of the whole re-audit, independently re-verified from source, not
taken on the reports' word:

- `🌎️hub/🏗️bootstrap/🦀️.rs:10007`: `let identity_verifier: Option<Arc<dyn IdentityAssertionVerifier>> = None;` —
  **still hard-coded `None`**, exactly as G12 found. This alone would still make production unreachable under the
  *old* rule.
- But `validate_auth_startup` (`:2617-2654`) is no longer that rule. Re-read verbatim: production requires
  `verifier.is_some() || credential_sign_in_enabled` (line 2629) — **not** `verifier.is_some()` alone. `main`
  (`:10052`) computes `bootstrap_ready` for production as `identity_verifier.is_some() ||
  credential_sign_in.is_enabled()`. So `OS_HUB_CREDENTIAL_SIGN_IN=true` alone satisfies the gate; the external-IdP
  seam stays unimplemented and unnecessary for a self-hosted deployment.
- A **network** (non-loopback) production bind additionally requires `CrossOriginPolicyV1::Allowlist` (i.e.
  `OS_HUB_ALLOWED_ORIGINS` set) and `ForwardedTlsTrustV1::TerminatingProxy` (`OS_HUB_TRUSTED_FORWARDING=proxy`),
  each its own named refusal if missing (`:2635-2641`). `transport_security_middleware` (`:2590`) is layered
  outermost on the router and enforces the proxy's `X-Forwarded-Proto` claim before any handler runs.
- This is real, non-aspirational code, present in the tree, with six new unit laws named in P4's report and cited
  passing (individually, not by a fresh run in this pass) in the coordinator's 09:17 nextest capture.

**What is genuinely still unproven**: P4's §8d claims a live loopback production boot — first user via
`os-hub credential set`, `POST /auth/sessions` 200 over HTTP, `SIGTERM` drain, restart-persists, store-format
refusal — all nine steps observed. That capture (`🗑️generated/p4-production-runtime.txt`) **no longer exists** on
this disk (the user's 2026-09-20 02:30 clean wiped `🗑️generated/` entirely, per `📓️status.md` session 5e). The
current `🌎️hub/README.md` still says, in its own words, *"Production mode has never been run by anyone… Expect to
be the first"* — which was written/left standing **after** P4's own claimed observation, so either (a) the README
line is intentionally conservative because the transcript is not independently re-checkable any more, or (b) it is
genuinely stale. Either way: **today, nobody holds runtime proof that survives on disk.** The code path is real and
unit-tested; the "someone actually ran a production hub" claim rests on one now-unverifiable report.

**Network-bound production has never been attempted by anyone**, live or otherwise — only the loopback posture was
(claimed) observed; the allowlist+proxy posture is unit-tested only (P4 §9 honest gap #3, still accurate).

## +B — a bug outside G12's 13 that would break exactly the topology this ticket just unlocked

HS1 (`📓️hs1-hub-pool-worker-stack-overflow.md`) found and fixed a real crash: every pool worker got Rust's default
2 MiB thread stack when `os-hub` runs as a **standalone process** (not launched through `cargo`, which floors
`RUST_MIN_STACK` at 64 MiB repo-wide via `.cargo/config.toml:65`) — i.e. exactly a systemd/Docker/bare-binary
production launch. A `catch_unwind` on an artifact-engine turn moved a ~529 KB value through a debug-build
coroutine frame that (before the fix) totalled 21.5 MB, aborting on a 2 MiB stack. Re-verified fixed in the current
tree: `🧰️framework/🔨️modules/⏳️async/🦀️.rs:1844` — `pub const WORKER_STACK_BYTES: usize = 64 * 1024 * 1024;` — and
`WorkerPool::new` (`:1893`) explicitly calls `.stack_size(WORKER_STACK_BYTES)`, independent of any environment
variable. This is a genuine, load-bearing production-readiness fix that predates none of the deploy work above:
without it, the first real `systemctl start semio-hub` or `docker run … os-hub` would very likely have `SIGABRT`ed
on its first non-trivial artifact read, with the hub's own test suite staying green the whole time (because the
suite always ran through cargo's 64 MiB floor). Confirmed DONE, live-verified per HS1 §5 (20 client frames + 5 min
socket survival, zero overflow lines, hub suite 318/3 → later sessions report 320/321).

## Root README.md discrepancy — a claimed deliverable that is not in the tree

D2's report (§3, "README 'Products' getting-started") describes adding a Getting-started block under each of the
four product blurbs (sketchpad/studio/cloud/assistant) plus a status note above them. **This is not present in the
current tree.** Re-verified: `README.md`'s "🛍️ Products" section (lines 124-179) is byte-for-byte the original
one-sentence-plus-demo-image content G12 described as "aspirational marketing copy" — no Getting-started blocks, no
status note, no mention of `activate-s-react-dev`/`os-hub:dev-secure-suite`/`npm run setup`. `git log --oneline -- README.md`
shows the last commit touching the file is `48a8c69cdb` (2026-06-04), and `git status --short README.md` /
`git diff HEAD -- README.md` are both empty — no uncommitted edit either. By contrast, the sibling edits from the
*same* D2 slice (`🌎️hub/README.md` and the MCP crate's README) **are** in the tree and were committed in the
ticket's most recent commit (`48b9d63cf6 — 555 + / 251 lines` per `git show --stat`). So this is not a git-history
gap or a stale read on my part: two of D2's three doc edits landed and survived; the third (root `README.md`) was
either never actually written to disk, or was written and then reverted/overwritten by a later pass, and nobody
caught the discrepancy. **Outcome D's "getting started" claim for the root README is currently false on the
tree**, even though the ticket's own bookkeeping (`📓️status.md:206`) credits D2 with it.

## Other checks the task asked for directly

- **Is `compose.yaml` real (real images/volumes)?** Yes, in the sense that it names real, resolvable images
  (`postgres:17-alpine`, the local `Dockerfile` build) and a real named volume (`hub-data`) as the whole backup
  unit, consistent with the README's backup runbook. It has never been run through `docker compose config`, let
  alone started — no independent confirmation it parses.
- **Is the Dockerfile buildable in principle (paths, emoji dirs, cargo features)?** The emoji directory names are
  handled the same way every other build script in this repo handles them (no special-casing needed — `bun`/`cargo`
  invocations pass through the literal paths), and `COPY . .` does not care about filenames. The real risk named in
  the file's own comments and P4's honest gaps is untested, not emoji-related: whether `bun install --frozen-lockfile`
  succeeds in a clean Linux container for this workspace, and whether `bun nx run os-hub-admin:build` needs
  something `.dockerignore` excludes. Nobody has run `docker build` to find out.
- **Is there any CI?** No. `.github/workflows/` is still an empty directory (`ls` → 0 files), confirmed directly.
  `.github/` otherwise holds only `dependabot.yml` and `agents/`/`hooks/` config, unchanged since G12.
- **Is the trusted catalog reusable across hub starts, and how?** Partially, and the boundary is now precisely
  known thanks to JC1 §6. A published trusted-catalog generation lives under
  `{OS_HUB_DATA}/trusted-catalog/{current.json, generations/<id>/}` and `os-hub:dev` skips republishing if one
  already exists — so restarting the **same hub binary** against the **same data root** is instant, no 45-minute
  rebuild. But every browser-actor entry in that catalog is stamped with a `codegenPolicy` string (e.g.
  `"semio.os.browser-jco-1.34.0-jspi.v1"`, `🌎️hub/📦️packages/🦀️rust/📜️script.ts:3881`) baked in at the exact jco
  version the *materializing* binary used. JC1 §6 hit this directly: after the jco bump landed in the Rust twin of
  that policy, "HS1's copied binary and the coordinator's both refuse this catalog" — a hub binary rebuild that
  changes the codegen/actor policy **invalidates every previously published catalog outright**, forcing a fresh
  ~45-minute materialize-and-publish. So: reusable across restarts of the same build; not reusable across a rebuild
  that touches codegen policy, with no migration or partial-republish path for that case either.

## Ranked remaining work (≤10 slices)

| # | slice | outcome(s) | size | why this order |
|---|---|---|---|---|
| 1 | Run `os-hub:build` (release) and `bun nx run @semio-tech/framework-os-mcp-rs:build-release` once, each on a quiet machine, and capture the result | A, C | M (cargo-heavy) | Everything downstream (#2 real tarballs, #4 Dockerfile rework, the MCP README's Claude Desktop path existing on disk) is blocked on this one pair of compiles nobody has run since the wiring landed |
| 2 | Run `build-s-react-release` (or the nx-generated equivalent for the `s` variant) once, end to end, and serve+open the static output in a browser | B | L (cargo-heavy, full plugin materialization) | S2/S6/S7 already proved the *dev* boot works with 35/35 kinds; this is the one remaining button-press for outcome B, and its blocking defect is gone |
| 3 | Re-run P4's production-runtime observation script (`📜️p4-production-runtime.sh`) against a freshly built release `os-hub`, on a real network bind with `OS_HUB_ALLOWED_ORIGINS`+`OS_HUB_TRUSTED_FORWARDING=proxy`, and keep the capture this time (outside `🗑️generated/`, which gets swept) | A | M | The loopback posture's own proof evaporated in the disk clean; the network posture (the one G12 actually cared about — "somewhere real") has *never* been attempted, live or otherwise |
| 4 | Rewrite `🌎️hub/Dockerfile`/`compose.yaml` for the production topology now that a bare binary can boot (drop the repo-copy builder stage's dev rationale; ship a binary-only runtime image; entrypoint is the release binary directly, not `bun nx run os-hub:dev`) | A | M | P4's own honest gaps flag this as superseded; the file that exists targets a topology this ticket's own work made unnecessary |
| 5 | Restore or rewrite the root `README.md` "🛍️ Products" getting-started section — D2's report claims it, the tree does not have it | D | S | Cheapest item on this list (the content already exists, written once, in D2's report); currently the ticket's own bookkeeping is wrong about what shipped |
| 6 | Wire `🐍️d2-anchor-probe.mjs` (or equivalent) into an actual CI/nx gate for `HubFirstRun`'s anchors, and extend the tour's live-probe coverage to the `space`/`invite` steps with a seeded credential | B/D | S | D2 named this itself: the unit suite stayed green while 12/12 anchors silently resolved to nothing; the fix (aliases) is a patch, not a guard, and it already broke silently once |
| 7 | A GitHub Actions (or equivalent) workflow: at minimum, `cargo check`/`nx run-many` on PR, and a release-on-tag job that runs items #1's two release builds and #2's `publish` targets | A/C | M | `.github/workflows/` is still empty; every "canonical root command" this ticket wired (`publish`, `build-release`) has no automated caller |
| 8 | `docker build`/`docker compose config` against the rewritten (item #4) Dockerfile on any machine that has Docker, to close the "authored, never built" gap for both the image and its compose file | A | S (given Docker available) | Trivial once #4 exists; currently zero confidence the file even parses |
| 9 | A worked, live-driven demonstration of the `--hub`/`--credential-file` loop (AgentDelegations UI → downloaded file → a real MCP client config → an agent editing a hub-hosted document a human sees) | C | M | M7 proved the `--folder`/local-bridge loop live; the `--hub` loop's pieces are all real and committed but nobody has run the whole chain once, the way M7 did for the other one |
| 10 | A minimal upgrade/versioning decision beyond "stamp and refuse" — at least a documented, deliberate "no live migration; here is the exact procedure to hand-port a v1 root to a v2 binary" runbook entry, now that store-format stamping is proven live | A | S | Explicitly out of scope by design ("greenfield… no migration framework") and lowest urgency, but the versioning half now exists and an operator will hit the missing other half eventually |

## Honest gaps in this re-audit

1. Every runtime claim carried over from P4/OB1r ("observed live") could only be **re-verified as code that is
   present and consistent with the claim**, not re-run — this pass took no build/server actions per its own
   constraints, and the specific evidence files (`🗑️generated/p4-production-runtime.txt`, `ob1r-observability-probe-2.txt`,
   etc.) were wiped by the user's 2026-09-20 02:30 disk clean before this pass started. The hub source, its unit
   laws, and the fact that the hub suite is 320/321 green as of session 7 (03:46) are what this pass could actually
   check.
2. The root-README discrepancy (above) was caught by `git log`/`git diff`, which is conclusive for *that* file, but
   this pass did not exhaustively re-diff every file every prior report claims to have changed — only the ones
   directly load-bearing for this task's questions (hub README, MCP README, `.mcp.json`, launch.json rows, the
   `HubFirstRun` directory, the `destructive` capability count). A different, unverified claim could exist elsewhere.
3. The `destructive: true` count (25 descriptor files, ~50 real call sites) was traced far enough to confirm it is
   real, non-test, and growing (25 → 44 → 82 per status.md's own tally) — but this pass did not verify the
   *quality* of each declaration (i.e., whether every one is a genuinely irreversible action, versus an
   over-eager blanket application across "delete"/"clear"-named verbs). That is a correctness question for
   whichever slice owns the capability-audit gate, not a deploy-readiness one.
4. Items #7 and #9 in the ranked list above assume the machine load allows a release-profile compile and a full
   plugin materialization respectively — both explicitly deferred throughout this entire ticket for load reasons.
   Their ranking reflects value, not near-term feasibility under the fleet's own stated constraints.
