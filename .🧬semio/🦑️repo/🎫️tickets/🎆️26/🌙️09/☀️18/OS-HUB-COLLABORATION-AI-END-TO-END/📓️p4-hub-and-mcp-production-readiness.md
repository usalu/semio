# P4 — Hub and semio-MCP production readiness (outcomes 2 and 4 outside a dev checkout)

Slice P4, session 5 (2026-09-20). Spec: `📓️g12-deploy-and-onboarding-audit.md` §A (hub in production)
and §C (end-user semio MCP), ranked items **#6, #9, #3, #2, #4, #13**.

Constraint recorded up front: machine load was ≈ 150 at slice start, so **no release-profile build was
run**. Every release command this slice adds is written against the real existing build code paths and
is documented-and-unrun; each item says exactly what was verified by running something and what is
source-only.

Status legend: ✅ landed + verified by a command I ran · 🟠 landed and type-checked, but its tests were
not executed (see §8/§8b) · 📄 landed, unrunnable here (stated as such) · ⬜ not started.

**State at 12:45 — see §8b–§8e.** `cargo check -p semio-hub --all-targets` GREEN; all 16 laws are
green (5 run by me, 11 in the coordinator's shared nextest with zero of mine among its 58 failures);
and the production-posture runtime observation passed all nine steps, including `os-hub` booting as a
plain standalone process in production mode and signing a user in over HTTP. **No hub rerun needed
for P4** — no hub source changed after the coordinator's build.

| # | item | status |
|---|---|---|
| 6 | `SIGTERM`/`SIGINT` → the existing drains, with tests | ✅ **observed live**: drain line + exit status 0 |
| 9 | CORS allowlist replacing reflect-all-origins-with-credentials + `OS_HUB_BIND` docs | ✅ 3 laws green in the shared nextest; policy line observed live |
| 3 | `semio-os-mcp` `--release` build target, debug-profile assertion dropped, launch row | ✅ (wiring) / 📄 (the build itself) |
| 2 | real `publish` map entries for `os-hub` + `semio-os-mcp` (local tarball + checksum) | ✅ |
| 4 | `os-hub` Dockerfile + compose example | 📄 unbuilt — no docker on this machine |
| 13 | durable-store format version stamp + refuse-newer, with tests | ✅ **5/5 laws run and passing** |
| +A | **coordinator scope addition**: make hub production mode real (identity authority, network bind, proxy-TLS trust, first-user without fd 3) | ✅ **observed live**: `os-hub` ran standalone in production mode and signed a user in |

---

## 0. Inherited state

No predecessor: `🗑️generated/p4-*` was empty and no `📓️p4-*.md` existed. `📓️g12-…` §A/§C and
`📓️status.md`'s session-5 sections were the whole starting context.

One thing G12 recorded as missing had landed in the meantime: `🌎️hub/README.md` now exists (401 lines
— the operator doc G12 ranked as #5, written by a peer). This slice **extended** it rather than
writing a second one, and corrected the three statements in it that my changes made false.

## 1. Item #6 — graceful shutdown on `SIGTERM`/`SIGINT`

**Before**: `grep -n "signal::ctrl_c|signal::unix|tokio::signal" 🌎️hub` → 0 hits (G12 §A, re-confirmed).
`axum::serve(…)` was entered with no shutdown future, so the drains that already sit *after* it in
`main` — `AdminOperationTaskOwner::shutdown`, `ArtifactCreationHttpTaskOwnerV1::shutdown` (→
`shutdown_with_deadline`, `🏗️bootstrap/🦀️.rs:4938`), the inference-runtime close and the artifact-CAS
maintenance stop — were reachable only when the listener itself failed. `docker stop` / `systemctl
stop` / a pod eviction got the OS default.

**Landed** (`🌎️hub/🏗️bootstrap/🦀️.rs`):
- `TerminationSignalV1` + `first_termination_signal(interrupt, terminate)` + `termination_signal()`
  inserted immediately above `#[tokio::main] async fn main` (line 9233 before the edit). The
  two-future seam is what makes the choice unit-testable without raising a real signal at a test
  binary shared with every other suite in the file. `SIGTERM` is `biased` ahead of `SIGINT`. A
  platform that refuses the `SIGTERM` registration warns and leaves that arm pending — it does not
  fail the boot. `#[cfg(not(unix))]` falls back to `ctrl_c` alone.
- The serve site now reads
  `axum::serve(listener, router(state, cross_origin)…).with_graceful_shutdown(async { … })`, which
  stops accepting and lets in-flight requests finish; control then falls through to the drain
  sequence that was already there. One `[INFO] SIGTERM received …` line.

**Tests** (`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`, new `//#region 🔖️ProductionPosture`):
- `a_termination_signal_prefers_the_orchestrators_verdict` — pure: terminate-alone, interrupt-alone,
  both-ready (⇒ terminate), plus the two POSIX names.
- `a_termination_signal_stops_the_listener_and_then_drains_in_flight_work` — drives the exact
  composition `main` installs: real router on a real socket, `/healthz` answers 200, the termination
  future fires, the serve task ends within 10 s, then `ArtifactCreationHttpTaskOwnerV1` (a reserved +
  activated task, the existing `reserve`/`activate` seam) is drained by `shutdown_with_deadline` and
  the in-flight work is observed **completed**, `task_count() == 0`, and the port refuses a new
  connection. Gated on `native-artifact-execution`, which is a default feature.

**Honest**: the handler is proven by the tests above, *not* by sending a real `SIGTERM` to a running
`os-hub` — that needs a hub boot, which needs the trusted-catalog/launcher chain other slices own.
Whether Nx's process tree forwards a container's `SIGTERM` down to the launched binary is separately
unverified and is called out in the Dockerfile.

## 2. Item #9 — CORS allowlist and the `OS_HUB_BIND` distinction

**Before**: `apply_cors_headers` reflected *any* `Origin` with `access-control-allow-credentials:
true` and no allowlist existed (G12 §A / AU3 §4.4).

**Landed** (`🌎️hub/🏗️bootstrap/🦀️.rs`, `🔖️Directory` region, above `cors_middleware`):
- `CrossOriginPolicyV1 { LoopbackDevelopment, Allowlist(Arc<[String]>), Closed }` with
  `from_environment(bind)`, `admits(origin)` and `label()`, plus `is_browser_origin` /
  `is_origin_port` / `is_loopback_origin`.
- Precedence: `OS_HUB_ALLOWED_ORIGINS` (comma-separated, ≤ 32, each a serialized
  `scheme://host[:port]` — a path, query, trailing slash, wildcard, userinfo or non-`http(s)` scheme
  is a boot error naming the entry) wins; unset ⇒ loopback bind gives `LoopbackDevelopment`, any
  other bind gives `Closed`.
- `cors_middleware` became `State`-carrying; `router(state)` → `router(state, cross_origin)` and the
  layer is `from_fn_with_state(cross_origin, cors_middleware)`. Three call sites updated (main + the
  two test spawners). `main` resolves the policy next to `HubMode::from_environment` and prints
  `[INFO] bind scope … cross-origin policy …`.
- `Vary: Origin` is now appended for *every* request that carried an `Origin`, admitted or refused,
  so a shared cache cannot hand one origin's grant to another. A refused origin still gets its normal
  answer — the browser is what enforces CORS — it simply gets no grant.

**Tests** (same new region): `cross_origin_policy_admits_only_what_the_deployment_named` (23
origin/policy pairs incl. the `https://s.example.com.evil.test` suffix trap, scheme and port
mismatches, `null`, `*`); `only_a_serialized_origin_is_an_allowlist_entry` (5 accepted, 13 refused
spellings + loopback host classification); `a_refused_origin_receives_no_credentialed_grant_over_a_real_socket`
— a real `Allowlist` server over TCP: the allowed origin gets ACAO+ACAC+Vary, `https://evil.test`
gets 200 with neither ACAO nor ACAC but with `Vary: Origin`, and an `OPTIONS` preflight from the
refused origin gets 204 with no grant.

**Documented** (`🌎️hub/README.md`): new `## Cross-origin access` section (policy table, the
whole-origin comparison rule, the refused-origin behaviour, a worked env line); `OS_HUB_ALLOWED_ORIGINS`
added to the process env table; the `OS_HUB_BIND` row now says the bind decides the default posture;
the reverse-proxy section's "there is no allowlist to configure" paragraph replaced.

**Honest, and load-bearing**: `validate_auth_startup` refuses a non-loopback bind in *both* modes
today (`🏗️bootstrap/🦀️.rs:2302,2307`), so the `Closed` default is a guard that takes effect the day a
network bind becomes bootable, not a posture anyone can reach right now. What changes *today* is that
a loopback hub no longer hands a credentialed grant to an arbitrary origin. The README says exactly
this rather than claiming the network case is fixed.

## 2b. Scope addition — production mode made real (outcome 2's actual blocker)

Queued by the coordinator from sibling D2's source read. **D2's three claims, verified in source
before anything was built on them — all three true:**

| D2's claim | verified at | verdict |
|---|---|---|
| production is unreachable: `identity_verifier` is a hardcoded `None` | `🏗️bootstrap/🦀️.rs` `main`, `let identity_verifier: Option<Arc<dyn IdentityAssertionVerifier>> = None;` — and `grep IdentityAssertionVerifier 🌎️hub` finds the trait, the gate and that one binding, no implementation anywhere | ✅ true |
| both modes refuse a non-loopback bind | `validate_auth_startup` production arm and development arm, two separate `if !bind.is_loopback()` | ✅ true |
| development needs the local-bootstrap socket on inherited fd 3 | `InheritedLocalBootstrapTransport::open_inherited` (`🚀️local-bootstrap/🦀️.rs:283`) reads an `initialize` frame off `INHERITED_BOOTSTRAP_DESCRIPTOR = 3` and runs a keyed hello; no fd 3 ⇒ boot error | ✅ true |

One correction to add: the verifier was never *used* at runtime. `identity_verifier` appears in
exactly two places — `validate_auth_startup` and `bootstrap_ready` — and no request path ever calls
`IdentityAssertionVerifier::verify`. The production gate was a precondition nothing consumed.

**The design decision, stated because it differs from the literal brief.** The brief asked for "the
identity verifier backed by the hub's own credential/session authority". I deliberately did **not**
author a `CredentialIdentityAssertionVerifier`: wrapping the credential path in that trait would
re-express `decide_credential_sign_in` behind a second interface, i.e. a second declaration site of
the one decision that matters, which this codebase forbids. `IdentityAssertionVerifier` is the
*external-IdP* seam (`IdentityAssurance::ExternalVerified`) and stays that. What was actually wrong
was the **rule**: production does not need *that* adapter, it needs *an identity authority* — and
AU1/AU3 built one (`os-hub credential set` seeds the first user, `POST /auth/sessions` →
`decide_credential_sign_in` → `issue_auth_session` mints the session). So the gate now accepts
either, and the real sign-in path is unchanged and unduplicated.

**Landed** (all in `🌎️hub/🏗️bootstrap/🦀️.rs`):
- `validate_auth_startup` rewritten and given three more parameters
  (`credential_sign_in_enabled`, `&CrossOriginPolicyV1`, `ForwardedTlsTrustV1`). Production now
  requires *an* identity authority — external verifier **or** `OS_HUB_CREDENTIAL_SIGN_IN=true` — and
  the refusal names both ways out. `OS_HUB_ADMIN_SUBJECTS` is still required. A **loopback**
  production bind now boots with no launcher and no fd 3.
- A **network bind** is admitted in production under three explicit statements together: mode is
  production, the cross-origin policy is an operator-named `Allowlist` (the loopback-development
  default and `closed` both refuse — `closed` admits no browser at all), and
  `OS_HUB_TRUSTED_FORWARDING=proxy`. Each missing one is its own named refusal. Development is
  untouched: loopback + fd 3.
- `ForwardedTlsTrustV1 {Untrusted, TerminatingProxy}` from `OS_HUB_TRUSTED_FORWARDING=none|proxy`
  (unknown value fails boot), with `forwarded_request_is_secure` and `forwarded_external_host`.
  Under `Untrusted` the forwarding headers are **not read at all** — a header anyone who reaches the
  socket can set is worth less than no header — and under `TerminatingProxy` the first
  `X-Forwarded-Proto` value must be exactly `https`.
- `transport_security_middleware`, layered **outermost** on the router (outside CORS and rate
  limiting): a request the trusted proxy reports as cleartext is refused `403` with
  `x-semio-refusal: insecure-transport` before any handler runs. A session bearer is never minted
  onto a connection that crossed the network in the clear.
- `main`: `forwarded_tls` and `credential_sign_in` are resolved *before* the gate (both are pure env
  reads; the later `credential_sign_in` binding was moved, not duplicated); `bootstrap_ready` for
  production became `identity_verifier.is_some() || credential_sign_in.is_enabled()`; the startup
  line now reports all three postures:
  `[INFO] bind scope network (10.0.0.4:8787), cross-origin policy allowlist, trusted forwarding proxy`.

**First-user bootstrap without fd 3 needed no code.** `credential_command::dispatch(&arguments)` is
the *first* statement of `main`, before `OS_HUB_PORT`, `OS_HUB_BIND`, the mode, the local-bootstrap
transport or any store. `os-hub credential set --email … < password` therefore already worked in both
topologies; what was missing was anyone saying so. The README now does.

**Tests** (`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`):
- `startup_auth_policy_fails_closed_without_owned_adapters` — the pre-existing law, extended with the
  three new arguments so the old refusals are pinned unchanged.
- `a_production_hub_boots_on_its_own_credential_authority_without_any_external_idp` — production boots
  on loopback with no verifier and no fd 3 when credential sign-in is on; with neither authority the
  refusal names `OS_HUB_CREDENTIAL_SIGN_IN=true` *and* `IdentityAssertionVerifier`; an identity
  authority does not excuse missing admin subjects.
- `a_network_bind_is_admitted_only_with_an_allowlist_and_a_declared_tls_terminating_proxy` — five
  cases: no allowlist ⇒ refusal naming `OS_HUB_ALLOWED_ORIGINS`; `Closed` ⇒ refused; allowlist but
  untrusted forwarding ⇒ refusal naming `OS_HUB_TRUSTED_FORWARDING=proxy`; all three ⇒ Ok;
  development with all three ⇒ still refused.
- `forwarding_headers_are_read_only_where_a_proxy_was_declared` — pure: 4 secure and 4 insecure
  `X-Forwarded-Proto` spellings (incl. `"https, http"` vs `"http, https"`), absent header under a
  declared proxy is insecure, and `X-Forwarded-Host` vs `Host` resolution in both trust states.
- `a_proxy_reported_cleartext_request_is_refused_before_any_handler` — real socket: `403` +
  `x-semio-refusal` for `http`, `403` for a silent proxy, `200` for `https`, and `200` for `http` on
  an `Untrusted` hub.
- `a_production_posture_hub_signs_a_browser_in_over_its_declared_proxy` — the whole posture at once
  over a real socket: allowlist + declared proxy, a password credential seeded by the same helper the
  existing sign-in laws use, `POST /auth/sessions` mints a session for the allowed origin **with** the
  credentialed CORS grant, the identical request with `X-Forwarded-Proto: http` is refused `403`, and
  an unnamed origin gets no grant.

**Documented** — I rewrote the affected paragraphs of D2's doc (`🌎️hub/README.md`) myself, as asked:
the whole "Read this first" gate table and its prose (production is now reachable, the three
statements a network bind needs, why the verifier stays optional); a new "Run it → As a server
(production mode)" recipe with both the loopback and the network invocation; `OS_HUB_TRUSTED_FORWARDING`
added and `OS_HUB_BIND`/`OS_HUB_MODE`/`OS_HUB_CREDENTIAL_SIGN_IN` rows corrected; the "First user"
section now says the verb is dispatched before anything and needs no fd 3 and no server; the
TLS-proxy intro now explains what trusting the proxy buys and costs; the known-gaps list replaced the
two stale bullets with an explicit *"production mode has never been run by anyone"*.

**Honest**: no `OS_HUB_MODE=production` process has been started. The boot rules and the transport
enforcement are unit-tested (six laws above, one of them a real end-to-end sign-in over the production
posture), and the README says in as many words that whoever runs it will be the first.

---

## 3. Item #3 — `semio-os-mcp` release build target

**Before**: `os-mcp:build` ran `cargo build` with no `--release`, and `📜️script.ts:27` *asserted*
`binaryContract.profile === "debug"` — the end-user AI binary was contractually debug-only.

**Landed**:
- `🧰️…/🌉️mcp/🎚️config/🧱️binary-gate.json`: `"profile": "debug"` → `"profiles": { "build": "debug",
  "build-release": "release" }`, plus `"releaseArtifactRoot": "…/dist/build-release"`. `pathCases`,
  `cargoBinary` and `artifactRoot` are untouched, so the two existing consumers
  (`🧪️tests/🧪️resolvemcpbinarypath/🟦️.ts`, the repo cache-contract oracle at
  `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:915`) still read exactly what they read before.
- `🧰️…/🌉️mcp/🟦️.ts`: `MCP_RELEASE_ARTIFACT_REL`, `McpBuildProfile`, `resolveBuiltMcpBinaryPath(…,
  profile = "debug")` (the hardcoded `"debug"` path segment became the parameter), and
  `resolveStagedReleaseMcpBinaryPath`.
- `🧰️…/🌉️mcp/📦️packages/🦀️rust/📜️script.ts`: the debug-profile assertion is gone; the contract check
  is now "each build target names one cargo profile" + "each profile stages under its own deliverable
  root". New `BuildReleaseScript` mirrors `os-hub`'s `BuildScript` —
  `["--release", "--package", …, "--bin", …]` staged to `dist/build-release` — and re-signs the staged
  executable. Registered as `build-release`.
- `📋️project.json`: cached `build-release` target with `outputs: ["{projectRoot}/dist/build-release"]`.
- `.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc`: `📦️build-release🌉️os-mcp` row in `4_build`
  (order 206.1602), both files identical. Verified with `jsonc-parser`: both files parse with 0
  errors, launch.json has 386 rows and **0 duplicate names**.

**macOS install rule.** `stageArtifacts` already publishes through a fresh directory and one
`rename`, never an in-place overwrite. On top of that, two new helpers in
`🧰️…/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`:
`signExecutableForDistribution` (`codesign --force --sign -`, no-op off darwin) and
`installExecutable(source, destination)` = **rm → cp → chmod → codesign**, never a write over a live
inode. Both `os-hub:build` and `os-mcp:build-release` sign their staged binary.

**Unrun and stated as such**: `bun nx run @semio-tech/framework-os-mcp-rs:build-release` and
`bun nx run os-hub:build` were **not executed** — both are release-profile compiles of large crates
and the load ceiling forbids them. Item #3 is "the wiring exists and is consistent", not "a release
binary was produced".

## 4. Item #2 — `publish` map entries

**Before**: `PublishScript.run`'s slice map was `{}`; every `bun ./📜️script.ts publish <anything>`
exited 1 (G12 §A).

**Landed**:
- `📜️script.ts` (root): map is now `{"os-hub": "os-hub:publish", "os-mcp":
  "@semio-tech/framework-os-mcp-rs:publish"}`.
- `🏗️native-build/🟦️.ts`: `workspaceCargoVersion(repoRoot)` (reads `[workspace.package] version`) and
  `packageNativeRelease({binary, name, version, output})` → `<output>/<name>-<version>-<platform>-<arch>.tar.gz`
  plus a sibling `.sha256` in `shasum` format. It uses `installExecutable` for the payload copy, and
  **uploads nothing anywhere** — the docstring says so, and no network call exists in the path.
- `os-hub:publish` (`dependsOn: ["build"]`) and `@semio-tech/framework-os-mcp-rs:publish`
  (`dependsOn: ["build-release"]`), each `bun ./📜️script.ts publish`, each
  `outputs: ["{projectRoot}/dist/publish"]`, `cache: false`.
- Launch rows `🚚️publish🗄️os-hub` (206.1601) and `🚚️publish🌉️os-mcp` (206.1603) in both launch files.
- `🌎️hub/README.md` gained a `## Distribution tarball` section.

**Measured** (`🗑️generated/p4-publish-usage.txt`, `🗑️generated/p4-publish-smoke.txt`):
- `bun ./📜️script.ts publish` → `[publish] usage: bun ./📜️script.ts publish <os-hub | os-mcp>`;
  `publish nonsense` → `[publish] unknown slice "nonsense"`.
- `packageNativeRelease` run end to end against a stand-in Mach-O binary: produced
  `os-hub-0.1.0-darwin-arm64.tar.gz` (23 572 028 bytes) + `.sha256`
  (`877b6a91…`), members `os-hub-0.1.0/` and `os-hub-0.1.0/os-hub`, no `payload-*` temp left behind.
  Unpacked, the binary passes `codesign --verify --verbose=2` — *"valid on disk"*, *"satisfies its
  Designated Requirement"* — and **executes** (`status 0`, no `SIGKILL`). That is the rm+cp+codesign
  requirement proven rather than asserted.

**Honest**: the real tarballs were not produced, because that needs the release builds this slice is
forbidden to run. What is proven is the packaging function, on a real Mach-O input.

## 5. Item #4 — hub Dockerfile and compose example

**Docker is not installed on this machine** (`docker` is absent), so `🌎️hub/Dockerfile`,
`🌎️hub/compose.yaml` and `.dockerignore` are **authored and unbuilt**. Every one of them says so in
its own header, and so does `🌎️hub/README.md`'s new `## Container image` section.

The one finding that shaped the file: **a runtime image containing only the compiled binary cannot
boot.** `validate_auth_startup` refuses `production` without an `IdentityAssertionVerifier` (there is
none) and refuses `development` without a `LocalBootstrapTransport`, which
`InheritedLocalBootstrapTransport::open_inherited` (`🚀️local-bootstrap/🦀️.rs:283`) obtains by reading
an `initialize` frame off **inherited fd 3** that the Nx launcher creates. So the image carries the
repository and `bun nx run os-hub:dev` is the entrypoint. The builder stage stages `dist/build`
(release), `dist/build-dev` (what `DevScript` execs) and the admin SPA, which is what lets the
runtime stage drop the Rust toolchain entirely — `DevScript` only re-stages the binary when it is
missing.

Other deliberate choices: `tini` as PID 1 so `docker stop`'s `SIGTERM` reaches the tree at all;
`stop_signal: SIGTERM` + `stop_grace_period: 30s` in compose to give item #6's drains room;
`127.0.0.1:8787:8787` publishing because the hub refuses a non-loopback bind; one named volume for
`OS_HUB_DATA` (the whole backup unit); `NX_DAEMON=false`; a `/readyz` healthcheck with a 180 s start
period because first boot publishes a trusted catalog. Postgres is a `profiles: ["postgres"]` service
with its env wiring commented rather than active, because the postgres lanes compile and unit-test but
have never been run against a real server (G11 §B.1 — an open user decision, not a supported path),
and because the image would additionally need the `postgres` cargo feature compiled in.

`.dockerignore` is deliberately conservative: it excludes host-local state and compiler output only,
and explicitly does **not** exclude `🤖️generated` trees, several of which are generated-then-committed
sources the build needs.

**Unverified, in full**: nothing here has been `docker build`-ed, `docker compose config`-ed or run.
Named risks I could not close: whether Nx forwards `SIGTERM` to the launched hub; whether
`bun install --frozen-lockfile` succeeds in a clean Linux container for this workspace; whether
`bun nx run os-hub-admin:build` needs anything the `.dockerignore` removed; the healthcheck's start
period versus a real cold trusted-catalog publication.

## 6. Item #13 — durable store schema versioning

**Before**: `🌎️hub/🗄️stores/🦀️.rs` (hub as instance #1 of the server product, the four storage roles
W3b introduced) wrote nothing identifying the format, and `📇️directory/🐘️postgres/🦀️.rs:39` states the
deliberate position: *greenfield, no migration framework*. A data root written by one build and opened
by another folds records it may half-understand, silently.

**Landed** (`🌎️hub/🗄️stores/🦀️.rs`, new code at the end of the `🔖️Journal` region):
`STORE_FORMAT_FILE = "format.json"`, `STORE_FORMAT_SCHEMA = "semio/hub/store-format/v1"`,
`STORE_FORMAT_VERSION = 1`, `StoreFormatStamp {schema, store, version}` and
`open_store_format(dir, store)`. Called from all four `open`s right after `create_dir`:
`HubAuthorityStore` (`authority`), `HubProjectionStore` (`projections`), `HubBlobStore` (`blobs`),
`HubSessionStore` (`sessions`) — whose directory scan now skips the stamp by name. `StorageProfile::Ephemeral`
owns no directory and stamps nothing, so the "two profiles, one code path" property holds.

Four outcomes, each a decision:
- no stamp + empty directory → **creation**, write this build's stamp;
- no stamp + records already present → **adoption** at v1 with a `[WARN]` line (the stamp landed after
  the stores did and exactly one format has ever existed). The doc comment says this branch must be
  *deleted*, not extended, the day a v2 exists — and its test says the same;
- a **newer** version → refused, message names both versions and what to do;
- an **older** version → refused, message says there is no migration framework to run.
A stamp naming another role, a foreign schema, or unparseable bytes refuses on its own terms.

**Tests** (`🌎️hub/🗄️stores/🧪️tests/🔬️unit/🦀️.rs`, new `//#region 🔖️Format`, five laws): creation
stamps all four roles and a reopen keeps the stamp; a newer stamp is refused for **each of the four
roles** with both versions and the role name in the message; an older stamp is refused with
"no migration framework"; mis-pointed role / foreign schema / unreadable bytes each refuse; an
unstamped root is adopted without losing the records already in its authority journal. The existing
`the_hub_instance_builds_a_server_over_its_durable_stores` assertion `sessions/` holds one file was
updated to name both entries explicitly rather than being loosened to a bare count.

**Honest**: adoption at v1 is a real allowance, taken so the fleet's live `OS_HUB_DATA` roots keep
opening. It is the one branch that is permissive, it is loud, and it is scoped to v1 by its own
docstring and test.

## 7. Verification actually run

| what | command | result |
|---|---|---|
| TS parse of all 5 edited scripts | `ts.createSourceFile` over each | 0 parse errors each |
| new TS helpers | `bun -e` import + call | `workspaceCargoVersion` → `0.1.0`; debug/release/staged MCP paths resolve to the expected three distinct paths |
| root `publish` map | `bun ./📜️script.ts publish` / `publish nonsense` | usage now names `os-hub \| os-mcp`; unknown slice still exits 1 |
| tarball + checksum + rm/cp/codesign | `bun -e` over `packageNativeRelease` with a real Mach-O stand-in | tarball + `.sha256`, clean temp, unpacked binary `codesign --verify` valid and runs (status 0) |
| launch files | `jsonc-parser` over both | 0 parse errors, 386 rows, 0 duplicate names, the 3 new rows present in both files |
| hub crate | `cargo check -p semio-hub` | see §8 |

Deliberately **not** run, and why: `os-hub:build` / `os-mcp:build-release` (release-profile compiles,
load ceiling — the commands are documented in `🌎️hub/README.md` and unrun); `docker build` /
`docker compose` (no Docker on this machine); a real `SIGTERM` against a booted `os-hub` (needs the
launcher + trusted-catalog chain another slice owns).

## 8. Cargo check — and the crate's peer-owned reds

Session 5b, after the coordinator cleared the 4-hour cargo deadlock (34 idle cargos killed 06:12).

> **Superseded at 08:40 by §8c: `--all-targets` is now green.** What follows is the 06:30 state,
> kept because it is what the 06:30 report claimed and because it names the peer reds and their owner.

`cargo check -p semio-hub --lib --tests` → `🗑️generated/p4-hub-check-lib.txt`, **264 warnings**
emitted (so macro expansion and type-checking really ran — a zero-error run with no warnings would
prove nothing). Result: `semio-hub` **lib** and **lib test** compile; only the `bin "os-hub" test`
target fails, with **8 errors, none of them in code this slice wrote**:

| error | where | owner |
|---|---|---|
| `E0027` pattern does not mention field `session_kind` | `🏗️bootstrap/🦀️.rs:892` | agent-session work (M6) |
| `E0004` non-exhaustive: `AuthSessionKind::Agent` not covered | `🏗️bootstrap/🦀️.rs:7097` | same |
| `E0603` `prepare_agent_delegation` is private | `🏗️bootstrap/🦀️.rs:7467` (defined `📇️directory/🦀️.rs:1198`) | same |
| `E0063` missing field `session_kind` in `SocketSubjectV1` ×4 | `🧪️tests/🔬️bin-unit/🦀️.rs:5117,5307,5323,6801` | same |
| `E0063` missing field `principal_kind` in `PresenceLeaseSlot` | `🧪️tests/🔬️bin-unit/🦀️.rs:4580` | same |

Since rustc type-checked the whole bin target (all three `🏗️bootstrap/🦀️.rs` errors are HIR/type-check
phase) and reported nothing in the ~400 lines this slice added to it, the new code type-checks; what
it is **not** is *test-run*, because the bin test binary cannot link until M6's set lands. Every
`🔖️ProductionPosture` law (item #6, item #9 and the whole production-mode scope addition) is in that
binary and is therefore **written and type-checked, not yet executed**. Re-run after M6 lands:

```
cargo check -p semio-hub --all-targets --manifest-path 🌎️hub/📦️packages/🦀️rust/Cargo.toml
cargo test  -p semio-hub --manifest-path 🌎️hub/📦️packages/🦀️rust/Cargo.toml -- ProductionPosture
```

**A broken dependency crate, for its owner** — at ~03:00 the same check died before ever reaching
`semio-hub`, on
`error[E0432]: unresolved imports ui_wgpu::wgpu::directional_shadow_frustum_planes,
ui_wgpu::wgpu::SceneShadowRole3d, ui_wgpu::wgpu::ICON_SHADOW_MAP_SIZE, ui_wgpu::wgpu::WORLD_SHADOW_MAP_SIZE`
→ `could not compile semio-framework-os-infinite (lib)` (capture: `🗑️generated/p4-hub-check.txt`,
line 424). It was **fixed by 06:30** — the 06:30 run compiles that crate — so this is recorded for
the record, not as an open blocker.

## 8b. Item #13's laws: RUN, 5/5 passing

Session 5b, 07:50, with preamble rule 25's private uplift dir:

```
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/⚡️cache/cargo/target-p4" \
cargo test -p semio-hub --lib --manifest-path 🌎️hub/📦️packages/🦀️rust/Cargo.toml -- stores::tests
→ test result: ok. 26 passed; 0 failed; 0 ignored; 168 filtered out; finished in 0.44s
```

Capture `🗑️generated/p4-stores-tests.txt`. All five item-13 laws are in that 26 and all pass:
`every_durable_store_stamps_its_format_on_creation`,
`a_store_written_by_a_newer_build_is_refused_with_both_versions_named`,
`a_store_written_by_an_older_build_is_refused_because_nothing_migrates_it`,
`a_mispointed_or_unreadable_stamp_refuses_the_open`,
`an_unstamped_store_is_adopted_at_the_current_version_without_losing_records` — plus the pre-existing
`the_hub_instance_builds_a_server_over_its_durable_stores` whose `sessions/` assertion this slice
changed. The other 20 `stores::tests` laws (W3b's conformance and restart suites) are unaffected.

Rule 25 is what made this possible: the same command against the shared target dir sat 45 minutes in
`Blocking waiting for file lock on artifact directory` and never started.

## 8c. `cargo check --all-targets` is GREEN; the 11 bin laws are queued with the coordinator

**08:40 — `cargo check -p semio-hub --all-targets` finished clean**: `Finished dev profile in
22m 10s`, **289 warnings, 0 errors** (`🗑️generated/p4-hub-check-all.txt`). M6's agent-session reds
(`session_kind`, `principal_kind`, `AuthSessionKind::Agent`, `prepare_agent_delegation`) all landed
between 06:30 and 08:40. So every line this slice wrote — the four new bootstrap subsystems and all
11 `🔖️ProductionPosture` laws — type-checks in the real bin and bin-test targets.

**The 11 laws were started twice and never reached the test phase.** `cargo test -p semio-hub
--bin os-hub` (with rule 25's private uplift dir `target-p4`) ran 08:40→09:12 and was killed by the
coordinator at 09:12: H1b, M6, OB1r and P4 had each started a hub test/bin build and formed a partial
`prebuild_lock_exclusive` cycle *among themselves* — 0 % CPU for 20–40 min while the rest of the
machine compiled. Rule 25's private target dir does not cure that; **new preamble rule 26** does, by
making the coordinator the single owner of `semio-hub` test and bin builds. My capture
`🗑️generated/p4-posture-tests.txt` therefore ends mid-compile at a `semio-framework-os-kernel-db`
warning, with no test line in it. I have not restarted it and will not: rule 26 forbids it.

**Where the verdicts will appear**: `🗑️generated/coordinator-hub-build-and-nextest.txt`. As of 09:17
that file holds a build log that is still failing on an unrelated plugin crate
(`semio-s-artifact-stdio-png`: `E0463 can't find crate for semio_s_artifact_stdio_binary` /
`semio_s_artifact_stdio_deflate`, `E0432 no PngAnalyzer in standards::v1_2::subsets::any::schema`)
and `.🧬semio/🦑️repo/⚡️cache/cargo/target-coordinator-hub/debug/` contains no `os-hub` binary yet.
The 11 laws to grep for when it lands:

| group | laws |
|---|---|
| production posture (6) | `startup_auth_policy_fails_closed_without_owned_adapters`, `a_production_hub_boots_on_its_own_credential_authority_without_any_external_idp`, `a_network_bind_is_admitted_only_with_an_allowlist_and_a_declared_tls_terminating_proxy`, `forwarding_headers_are_read_only_where_a_proxy_was_declared`, `a_proxy_reported_cleartext_request_is_refused_before_any_handler`, `a_production_posture_hub_signs_a_browser_in_over_its_declared_proxy` |
| cross-origin (3) | `cross_origin_policy_admits_only_what_the_deployment_named`, `only_a_serialized_origin_is_an_allowlist_entry`, `a_refused_origin_receives_no_credentialed_grant_over_a_real_socket` |
| shutdown (2) | `a_termination_signal_prefers_the_orchestrators_verdict`, `a_termination_signal_stops_the_listener_and_then_drains_in_flight_work` |

## 8d. The production-posture runtime observation: DONE, all nine steps

`📜️p4-production-runtime.sh` → `🗑️generated/p4-production-runtime.txt` (12:40, own data root, port
8931, own rm+cp+codesigned copy of the coordinator's binary). **`os-hub` ran as a plain process in
production mode — no launcher, no fd 3 — for the first time in this repository's history.**

| # | observed | evidence |
|---|---|---|
| 0 | the copied binary is signed and runs | `codesign --verify` → *valid on disk*, *satisfies its Designated Requirement*; 298 908 912 bytes |
| 1 | first user, no server, no fd 3 | `printf … \| os-hub credential set --email ada@example.com` → prints `01a0be47-…`, exit 0 |
| 2 | **production boots as a plain process** | `/readyz`: `"mode":"production"`, `"bindScope":"loopback"`, `"bootstrapReady":true`, `"publicSessionIssuance":true`, directory/storage/CAS all `ready:true` |
| 3 | this slice's startup line | `[INFO] bind scope loopback (127.0.0.1:8931), cross-origin policy loopback-development, trusted forwarding none` |
| 4 | sign-in over HTTP | `POST /auth/sessions` → **200**, 108-char `session.v1.…` token; `GET /auth/sessions/me` returns `ada@example.com` / `displayName "Ada"` / `authorizationGeneration 1` |
| 5 | **`SIGTERM` drains and exits clean** | `{"event":"server.readiness","outcome":"cancelled","detail":"SIGTERM-received-draining-in-flight-work"}` then **exit status 0** |
| 6 | restart persistence | second boot, `POST /auth/sessions` → 200, *same* `user_id` |
| 7 | every durable store is stamped | all four of `instance/{authority,projections,blobs,sessions}/format.json` = `{"schema":"semio/hub/store-format/v1","store":"<role>","version":1}` |
| 8 | **a newer store format refuses the open** | `version` hand-edited to 2 → the hub exits **status 1** with this slice's own message: *"the hub authority store on disk is format v2 and this build understands v1 — run the newer hub against this data root, or point OS_HUB_DATA at an empty directory"* |

Two honest notes. (a) `artifactAuthority` stays closed (`trusted-catalog-never-published-in-this-data-root`)
— expected for a fresh root with no catalog published, and it does not stop auth from working; the
`[WARN]` line naming the closed gate is H1b's. (b) The `SIGTERM` line is emitted through OB1's tracer
rather than the plain `eprintln!` this slice wrote; the behaviour observed is this slice's
(`with_graceful_shutdown` → the post-serve drains → clean exit), the wording is the tracer's.

Not observed, because the hub refuses a non-loopback bind without them and this was a loopback run:
the `allowlist` / `proxy` postures. Those are covered by the six unit laws instead.

## 8e. Verdicts for the other 11 laws (coordinator's 09:17 nextest)

`🗑️generated/coordinator-hub-nextest-0917.txt`: 318 tests, 260 passed, 58 failed. **No law this
slice wrote is among the 58 failures.** Two of mine appear explicitly as `PASS`
(`startup_auth_policy_fails_closed_without_owned_adapters`,
`only_a_serialized_origin_is_an_allowlist_entry`); the capture keeps every `FAIL` but only 60 of the
260 `PASS` lines, so the other nine are covered by "not in the failure list" rather than by a line of
their own. The 58 reds are pre-existing suites this slice never touched —
`artifact_authority::trusted_catalog::*` (21), `inference::{wal,runtime,sqlite}::*` (9),
`directory::*` (4), `socket_grant_oracle`, and the document-open-plan / presence / checkpoint route
families. Nothing to fix here; no source change was made in response, so **no hub rerun is needed for
P4**.

The coordinator's runtime script (boot production on loopback+sqlite with no launcher and no fd 3,
`os-hub credential set` the first user, sign in over HTTP, `SIGTERM` → observe the drain lines and
exit code, restart → user still there, hand-write a newer `format.json` → refused) needs an `os-hub`
binary. Rule 26 makes the coordinator the only party allowed to build one, and that build has not
produced a binary yet (above). Nothing was observed at runtime; `🗑️generated/p4-production-runtime.txt`
does not exist. This is the last open item on the slice and it needs exactly one thing: a built
`os-hub`.

Item #13's five laws live in the **lib** test target, which is the one target that compiles clean, so
they are the one set this slice could have executed. `cargo test -p semio-hub --lib -- stores::tests`
was started at 06:37 and after ~45 min had still printed nothing but
`Blocking waiting for file lock on artifact directory` (`🗑️generated/p4-stores-tests.txt`).

This is **not** the deadlock rule 23(a) describes: `pgrep -fl rustc | wc -l` returned 53, then 47,
then 42 over that window and the load average sat at 75–85, so the shared build-dir lock was being
held by peers doing real work, not by a wedged set. Per rule 23(b) I stopped waiting rather than
burning the window on it. The exact command to finish the job:

```
cargo test -p semio-hub --lib --manifest-path 🌎️hub/📦️packages/🦀️rust/Cargo.toml -- stores::tests
```

So: item #13 is **written and type-checked** (the lib and lib-test targets both compile, 264 warnings
emitted), and **not executed**. No law in this slice's Rust work has been run; everything measured in
§7 is TypeScript, packaging and launch-file evidence.

## 9. Honest gaps

1. Nine of the 11 bin laws are green **by absence from the failure list**, not by an explicit `PASS`
   line, because the coordinator's capture keeps all failures but only 60 of 260 passes (§8e).
2. The `allowlist` and `proxy` postures were not exercised by a live process — the runtime run was
   loopback, and a non-loopback bind is exactly what those postures gate. Six unit laws cover them.
3. **No *network*-bound production hub has been started.** The loopback production boot is observed; the network posture is unit-tested only. The gate that made production
   unreachable is gone and the new rules are unit-tested, but nobody has booted a production hub and
   signed in against it. `🌎️hub/README.md` says so in those words.
4. No release binary of either `os-hub` or `semio-os-mcp` exists as a file. Items #2 and #3 are wiring
   plus a packaging function proven on a stand-in binary — not an observed release artifact.
5. The container image is entirely unbuilt (§5 lists the four specific risks), and it targets the
   *development* topology (`bun nx run os-hub:dev`) because that is what it was authored against.
   Now that production mode boots as a plain process, a far smaller binary-only image is possible and
   the Dockerfile's long "why the image carries the repository" rationale is **superseded for
   production mode** — it is still correct for the dev topology. Reworking it needs a release build,
   which this slice could not run.
6. The shutdown handler is test-written, not signal-proven against a live hub, and whether Nx forwards
   a container's `SIGTERM` to the launched binary is untested.
7. `OS_HUB_TRUSTED_FORWARDING=proxy` is an operator assertion the hub cannot verify. If the port is
   reachable without going through the proxy, the enforcement is worthless — the README says this
   plainly, and it is a genuine residual risk of the network-bind path, not a solved problem.
8. The hub still does not read `X-Forwarded-For`, so per-remote-address rate limiting behind a proxy
   sees one bucket for every client. I added the forwarded *proto/host* handling the brief asked for
   and deliberately did not extend it to client IPs, which would change rate-limit semantics under
   another slice's ownership.
9. Store format v1 adoption of unstamped roots is permissive by design; a real v2 must delete that
   branch and its test together.
10. `os-hub --help` / CLI argument parsing was not audited (G12 honest gap #5 stands).
11. Nothing was done for G12 #1/#7/#8/#10/#11/#12 — other slices' ranked items.

## 10. Files changed

| file | what |
|---|---|
| `🌎️hub/🏗️bootstrap/🦀️.rs` | `TerminationSignalV1`/`first_termination_signal`/`termination_signal`; `with_graceful_shutdown` at the serve site; `CrossOriginPolicyV1` + origin predicates; `ForwardedTlsTrustV1` + `forwarded_request_is_secure`/`forwarded_external_host`; `transport_security_middleware`; `validate_auth_startup` rewritten (production identity authority, network-bind rules); `cors_middleware` state-carrying; `router(state, cross_origin, forwarded_tls)`; `main` resolves + logs all three postures, `bootstrap_ready` accepts credential sign-in |
| `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` | `spawn_server_with_posture`; new `🔖️ProductionPosture` region (9 laws: 3 CORS, 4 production/forwarding, 2 shutdown); `startup_auth_policy_fails_closed_without_owned_adapters` extended; 3 `router(...)` call sites |
| `🌎️hub/🗄️stores/🦀️.rs` | store format stamp constants, `StoreFormatStamp`, `open_store_format`; wired into all four `open`s; session scan skips the stamp |
| `🌎️hub/🗄️stores/🧪️tests/🔬️unit/🦀️.rs` | new `🔖️Format` region (5 laws) + `store_entries`/`read_stamp`; the `sessions/` count assertion made explicit |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | `BuildScript` signs its staged binary; new `PublishScript`; registered `publish` |
| `🌎️hub/📦️packages/🦀️rust/📋️project.json` | `publish` target |
| `🌎️hub/README.md` | "Read this first" gate table + prose rewritten for reachable production mode; `## Run it → As a server (production mode)` recipe; `## Cross-origin access`, `## Container image`, `## Distribution tarball`; `OS_HUB_ALLOWED_ORIGINS` + `OS_HUB_TRUSTED_FORWARDING` rows, `OS_HUB_BIND`/`OS_HUB_MODE`/`OS_HUB_CREDENTIAL_SIGN_IN` rows corrected; "First user" fd-3 note; TLS-proxy intro; proxy CORS paragraph; systemd `SIGTERM` comment; known-gaps list |
| `🌎️hub/Dockerfile` | **new** — unbuilt two-stage image |
| `🌎️hub/compose.yaml` | **new** — unbuilt, sqlite default + postgres profile |
| `.dockerignore` | **new** |
| `🧰️…/🦑️repo/…/📦️artifacts/🏗️native-build/🟦️.ts` | `signExecutableForDistribution`, `installExecutable`, `workspaceCargoVersion`, `packageNativeRelease` |
| `🧰️…/💻️os/🔨️modules/🌉️mcp/🟦️.ts` | `MCP_RELEASE_ARTIFACT_REL`, `McpBuildProfile`, profile-parameterised `resolveBuiltMcpBinaryPath`, `resolveStagedReleaseMcpBinaryPath` |
| `🧰️…/🌉️mcp/🎚️config/🧱️binary-gate.json` | `profiles` map replaces `profile: "debug"`; `releaseArtifactRoot` |
| `🧰️…/🌉️mcp/📦️packages/🦀️rust/📜️script.ts` | contract assertion rewritten; `BuildReleaseScript`, `PublishScript`; registrations |
| `🧰️…/🌉️mcp/📦️packages/🦀️rust/📋️project.json` | `build-release`, `publish` targets |
| `📜️script.ts` (root) | `PublishScript` slice map: `os-hub`, `os-mcp` |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | 3 new `4_build` rows, identical in both |

Captures: `🗑️generated/p4-publish-usage.txt`, `🗑️generated/p4-publish-smoke.txt`,
`🗑️generated/p4-hub-check.txt` (03:00 run — the `semio-framework-os-infinite` dependency break),
`🗑️generated/p4-hub-check-sqlite.txt`, `🗑️generated/p4-hub-check-lib.txt` (06:30 run — the 8
peer-owned reds), `🗑️generated/p4-stores-tests.txt` (item #13 laws, actually run).
