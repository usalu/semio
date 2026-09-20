# G12 — Deploy and onboarding audit: can a non-developer get the four outcomes running?

Slice G12, read-only (Sonnet), 2026-09-20. Scope: static tree reading only (grep/find/Read), no
builds/servers/cargo. Question nobody in this ticket has asked yet: from a clean machine — not this
repo's devcontainer, not someone who already knows the codebase — can a real person reach (A) a
running hub somewhere real, (B) the `s` frontend as a production build, (C) an installed end-user
`semio-os-mcp` registered in an MCP client, (D) documentation/first-run experience that gets them
there? Builds on `📓️g2-hub-depth-audit.md` (hub backend depth: auth/db/presence/observability),
`📓️g4-zero-touch-and-run-paths.md` (dev golden paths, devcontainer zero-touch),
`📓️g7-mcp-agent-and-collaboration-audit.md` (MCP tool/bridge depth), `📓️h1-hub-build-and-boot.md`
(hub build/boot/observability), `📓️au3-live-sign-in-integration.md` (live auth, first-user
bootstrap), `📓️m4-mcp-bridge-approval-binding.md` / `📓️m7-live-agent-bridge-loop.md` (MCP bridge to
a live shell). None of those asked the deploy/onboarding question; this report cites their findings
rather than re-measuring what they already measured, and adds what a person outside the repo would
hit.

**Headline finding**: every one of the four outcomes has a real, working *developer* path (verified
live by AU3/M4/M7/H1 inside this repo's own dev loop) and **zero** production/distribution path. There
is no Dockerfile, no CI/CD, no release automation, and the root `publish` command — the one README's
CI/CD section names as canonical — is a stub with an empty target map. A non-developer cannot reach
any of the four outcomes without first becoming a developer of this repo.

---

## A. Hub in production

**Release build exists, everything around it does not.**

- **Build target**: `os-hub:build` → `bun ./📜️script.ts build` → `BuildScript.run`
  (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:12064-12069`) runs
  `cargo build --release --bin os-hub` into `dist/build`. Per H1 §7 item 3, this target has **never
  been executed** in this ticket ("a release build of `semio-hub` into a fresh `CARGO_TARGET_DIR`
  costs more than the fleet had to spare") — only `build-dev` (dev profile) has run, and only through
  `os-hub:dev`'s own dependency chain. So even the one release artifact this repo can produce is
  compile-target-only, not observed to exist as a file.
- **Container image / Dockerfile**: **none for the hub.** The only `Dockerfile` in the repo
  (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/Dockerfile`) packages the
  unrelated internal Next.js repo-tooling coordinator server (`node:22-alpine`, port 8787 — a
  coincidental port collision with the hub's own dev default, unrelated products). `find . -iname
  "docker-compose*"` finds only `.devcontainer/docker-compose.yml` (the dev container, not a
  deployment topology). No `.dockerignore`/multi-stage build for `os-hub` exists anywhere.
- **No deployment descriptors of any kind**: `find . -iname "*.service" -o -iname "*.plist"` returns
  only an unrelated macOS document-association plist and vendored Playwright Chromium plists — no
  systemd unit. No k8s/helm manifests, no `fly.toml`, `railway.json`, `Procfile`. `.github/workflows/`
  is an **empty directory** (`ls .github/workflows` → 0 files) and `.github/agents/`/`hooks/`/
  `dependabot.yml` are the only populated things under `.github` — there is no CI pipeline of any
  kind, let alone a release-on-tag workflow.
- **The root `publish` command is a stub.** README's CI/CD section (line 744) names `publish` as one
  of the "canonical root commands… controlled through" the automation surface. Its implementation
  (`📜️script.ts:14882-14898`, `PublishScript.run`) is:
  ```ts
  const map: Record<string, string> = {
  };
  if (!slice) { console.error(`[publish] usage: …`); process.exit(1); }
  ```
  The slice map is **empty**. Every invocation of `bun ./📜️script.ts publish <anything>` prints a
  usage error and exits 1, regardless of argument. There is no working publish path for the hub
  binary, the `s` distribution, the `semio-os-mcp` binary, or any npm/cargo package — the "canonical
  root command" README and AGENTS.md point to does nothing today.
- **Configuration surface**: real but entirely undocumented outside source. G2 §9 counted 14
  `OS_HUB_*` env vars read directly via `std::env::var` in `🏗️bootstrap/🦀️.rs` (`ADMIN_DIR`,
  `BIND`, `DATA`, `DATABASE_URL`, `DB_SQLITE`, `DIRECTORY_BACKEND`, `DIRECTORY_DATABASE_URL`, `PORT`,
  `STORAGE_BACKEND`, …) — no schema/validation layer, no `.env` file support. `grep -n "OS_HUB"
  README.md` → **0 hits**: none of these 14 variables is documented anywhere a person would read
  before running the binary. `OS_HUB_BIND` defaults to `0.0.0.0` (`🏗️bootstrap/🦀️.rs:8621`) — i.e. a
  naive `os-hub` launch already listens on every interface, not just loopback, with no TLS and no
  warning.
- **TLS / reverse-proxy**: **none in-process.** `grep -n "rustls|native-tls|TlsAcceptor|tls" 🏗️bootstrap/🦀️.rs`
  → 0 hits. The hub is plain HTTP only; any real deployment needs an external TLS-terminating reverse
  proxy, and nothing in the repo documents how to front it with one (no example nginx/Caddy config).
- **CORS/origin rules**: real but permissive by construction, not by policy choice a deployer can see.
  AU3 §4.4 measured the hub reflecting the request's own `Origin` header with
  `access-control-allow-credentials` (`🏗️bootstrap/🦀️.rs:6733`) — i.e. **any** origin is admitted for
  credentialed requests; there is no configurable allowlist. This is the right behavior for the
  dev/local-multi-port topology AU3 built it for, but it is also the CORS posture a naive production
  deploy would inherit unchanged, and nothing surfaces this as a decision to make before going public.
- **Database / persistence**: real (G2 §1-2, re-confirmed by H1 §11): WAL + snapshot + compaction
  under `db::Database`, selected via `OS_HUB_STORAGE_BACKEND` (`fs` default / `sqlite` / `postgres` /
  `neo4j`); directory (identity/tenancy) via `OS_HUB_DIRECTORY_BACKEND` (`sqlite` default, postgres/
  neo4j opt-in). H1 §11 confirms postgres/neo4j now compile clean under their features
  (`au3-hub-postgres-check.txt`/`au3-hub-neo4j-check.txt`) and AU3 §3.4 implemented and tested their
  credential paths — this closes G2's P1 #6 stub. **Sqlite file location**: `{OS_HUB_DATA}/directory.db`
  for the directory, `{OS_HUB_DATA}/db` for the filesystem document store (H1 §6.2, observed on a
  real boot). No documentation states this path convention for an operator planning disk layout or
  backups.
- **Schema/event-store bootstrap on first start**: real and observed. H1 §6.2's fresh-data-root boot
  shows the directory and document stores self-create and report ready; `artifactAuthority` starts
  `false` until a trusted catalog is published into that data root (§6.2-6.3) — this is the step that
  needs `os-hub:build`/`build-dev` plus the stdio+GIS materialization, which is itself gated on a
  peer's plugin build and the shared Cargo lock (H1 §6.3 Findings C-D). A cold-start-to-ready path
  exists and was measured, but it costs a native compile, not a database migration.
- **First-user bootstrap**: real, and the one part of "A" that is genuinely production-shaped. AU3 §3.3
  implemented an **operator CLI verb** on the binary itself: `OS_HUB_DATA=/abs/path os-hub credential
  set --email a@example.com [--display-name "Ada"] < password` (`🌎️hub/🔐️auth/📤️command/🦀️.rs`,
  dispatched in `main` before any service opens, `🏗️bootstrap/🦀️.rs:8556`). Password read from stdin
  (never argv/env), requires read/write on the server-owned data root, idempotent, revokes existing
  sessions on a credential change. This is the right shape for a real deployment's first-user story
  and it is live-tested (AU3 §5, 38 checks). **Gap**: nothing documents this verb outside the AU3
  report and the source itself — it does not appear in README, and `os-hub --help`/no-args behavior
  was not checked in this pass.
- **Backup/restore**: **no operator tooling.** `grep -rn "backup" 🌎️hub` finds only a test-name string
  (`📜️script.ts:6203`) and unrelated internal WAL-recovery language — no `os-hub backup`/`restore`
  verb, no documented "stop the process and copy `OS_HUB_DATA`" runbook, despite the durable WAL/
  snapshot layer underneath being real (G2 §1). An operator today would have to infer the backup unit
  (the whole `OS_HUB_DATA` tree, hub stopped) from source, not from any doc.
- **Upgrade across versions of an event-sourced store**: explicitly, deliberately absent, and the
  codebase says so out loud: `🌎️hub/📇️directory/🐘️postgres/🦀️.rs:39-40` — *"no migration framework
  (greenfield: there are no users yet, so schema changes are edited in place, not migrated)"*. This is
  an honest, load-bearing design comment, not an oversight, but it means the product today has no
  upgrade story at all for anyone who has already written data into a hub.
- **Log/trace output**: **absent**, confirmed independently by G2 §9 and H1 §13.8: 0 `tracing::` call
  sites anywhere under `🌎️hub`, no `tracing`/`prometheus`/`opentelemetry` dependency. H1b (§12) raised
  the floor from "silent" to "the closed gate names itself" — `/healthz` (K1), `/readyz`'s
  `blocked_by`/`reason` fields, and a startup line that says `[WARN] … closed gates: …` instead of a
  bare "ready" claim — genuine, tested (`a_not_ready_hub_names_every_closed_gate_and_its_reason…`,
  `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:96-129`) improvement to *readiness* diagnosis. But there is still
  no structured request/WS tracing and no metrics endpoint for a production operator to scrape.
- **Graceful shutdown**: partial, and not orchestrator-shaped. `shutdown_with_deadline`
  (`🏗️bootstrap/🦀️.rs:4848`) is a real internal drain path for the artifact-creation actor (H1 §5.3
  fixed a `Send`-future bug in it), but `grep -n "signal::ctrl_c|signal::unix|tokio::signal" 🌎️hub` →
  **0 hits** — the process installs no `SIGTERM`/`SIGINT` handler at all. The only "clean stop" path
  measured is the **dev-only** local-bootstrap pipe close (H1 §6.2: closing the pipe is classified as
  `local bootstrap endpoint closed` and the process exits 1 even on a launcher-initiated stop). A
  `docker stop`/`systemctl stop`/k8s `SIGTERM` against a bare `os-hub` binary today gets the OS
  default (immediate termination), not a drain.

## B. `s` frontend as a production build

- **A real static-bundle pipeline exists and is contract-tested**, contrary to what a first read of
  G4 might suggest. `@semio-tech/framework-os-dev:build` (`🧰️framework/…/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`)
  `dependsOn: ["build-s-react-release"]` and runs `bun ../../🚚️distribution/🏁completion/📜️script.ts
  complete`. The `🚚️distribution` module (`🧰️framework/…/🧑‍💻dev/🚚️distribution/**`) defines a closed,
  validated `DistributionLayout` (`🟦️.ts:1-45`): one entry HTML, one manifest JSON, hashed chunk/asset
  filenames (`-[hash].{js,mjs,css,wasm,woff2,ttf}`), sibling-emoji-collision checks — a real Vite-style
  production bundle contract, not aspirational. `generate-distribution`/`preview-distribution`/
  `check-distribution` nx targets exist and exercise it. `build-<variant>-react-release` (dynamically
  generated per playground, `🧰️framework/…/📚️library/🟨️.mjs:932,1056`) materializes every selected
  plugin component at `release` profile (not `dev`), depending on `workspace:deps-wasm-opt`.
- **Which nx target, concretely**: for the `s` variant, `build-s-react-release`
  (`pluginSiteTargetsForCrate`/`playgroundPreparationTargets`, `🟨️.mjs:1056,1066`). This is the
  release counterpart of the (broken, per G4 §2a) dev launch rows — G4 already found the **dev**
  "s ⚛️react" launch rows silently serve wgpu instead of React (`.vscode/🧩️launch.seed.jsonc:3151-3182`);
  this pass did not find any evidence that gap also affects the release target's own command shape
  (it is a distinct, dynamically-generated target, not the same `@semio-tech/framework-os-dev:dev`
  path G4's bug lives in) — but nobody in this ticket has ever run `build-s-react-release` end to end,
  so whether it actually produces a *working* served bundle for the full `s` variant (all ~60 plugins)
  is unverified in either direction. G9 (`📓️g9-s-product-space-audit.md`, cited by fleet-4 progress at
  ~23:30) is the report that already established nobody has ever observed `dev s` finish a cold boot
  at all (stops on 🧱️block's missing staged module) — the release path shares that same plugin-
  materialization dependency chain and has not been separately proven to avoid it.
- **~60 plugin wasm components — publication and caching**: real machinery, incompletely wired for
  production per G9/S2 (cited, not re-audited here): components are compiled per-plugin (`wasm32-wasip2`),
  a `plugin-registry:generate` catalog (`🤖️generated/🔌️plugins.json`) resolves them, and G4 §1 already
  found this generated catalog is **gitignored** and outside `setup`'s dependency graph — a fresh clone
  has no catalog until someone runs `bun nx run @semio-tech/plugin-registry:generate` by hand. For a
  *production* distribution the release-profile materialization (`materialize-release`,
  `🟨️.mjs:1052`) is the analogous step, and it inherits the same "never proven end to end" status as
  the dev path.
- **How the shell learns the hub URL**: real and load-bearing, found by AU3 not G9/G4. `ShellHost`
  used to hardcode `bootstrapOrigin: globalThis.location.origin` (the UI's own origin) — AU3 §4.4
  fixed this with `hubBootstrapOriginV1()` reading the **build-time** Vite define `VITE_S_HUB_URL`,
  which is exactly the config surface a production static bundle needs (bake the hub URL in at build
  time, since a static bundle has no server-side config injection point). This is the one piece of
  "B"'s config story that is real, tested, and live-proven (AU3 §5.3's two-browser transcript uses it).
  It has no counterpart for a deployer who wants to build once and point at different hubs per
  environment (no runtime config file, no `/config.json` fetched at load) — the URL is compiled in.
- **Service worker / offline**: **absent.** `grep -rli "serviceWorker|service-worker|workbox"` across
  `🧰️framework/🛍️products/💻️os` → 0 hits. No offline capability, no cache-first asset strategy beyond
  whatever the browser's ordinary HTTP cache does with the hashed filenames.
- **Bundle size**: not measured in this pass (would require running the build); the wasm-per-plugin
  architecture (~60 components, `component-release` per `🟨️.mjs:873-883`) means the theoretical total
  is large, but lazy-loading is architecturally the point (G9's finding that `dev s` fans out and
  "lazy cross-plugin install is real") — whether that laziness survives into the static release bundle
  (versus eagerly bundling every plugin) was not traced here.
- **React vs wgpu renderer for production**: the wgpu renderer is the one that actually boots and has
  been live-collaboration-proven in this ticket (AU3 §5.5, M7); the React renderer is the one the
  collaboration E2E harness and the distribution/build machinery above are built around. Concretely:
  wgpu has **no hub sign-in/spaces/workspace UI at all** (G9's finding, WG6 queued to build it) while
  React has all of it (AU2/AU3) but has never been observed completing a cold `s` boot. So today
  neither renderer has an end-to-end-proven path from "production build" to "a stranger opens a URL
  and sees a working, hub-connected `s`."

## C. End-user semio MCP

**This is the least production-ready of the four outcomes.** Everything that exists is dev-loop-bound;
nothing exists for a person who is not checked out into this repo.

- **No release binary target.** `os-mcp`'s own `build` target
  (`🧰️framework/…/🌉️mcp/📦️packages/🦀️rust/📋️project.json:"build"`) runs `bun ./📜️script.ts build` →
  `buildCargoArtifacts(…, ["--package", MCP_CARGO_PACKAGE, "--bin", MCP_BINARY_NAME], …)`
  (`…/📜️script.ts:89-94`) — **no `--release` flag**, unlike `os-hub`'s own `BuildScript`
  (`🌎️hub/…/📜️script.ts:12064-12069`), which explicitly passes `["--release", "--bin", "os-hub"]`.
  Worse: the crate's own path-contract fixture *asserts* this — `…/📜️script.ts:27`:
  `if (binaryContract.profile !== "debug") throw new Error("semio-os-mcp binary fixture disagrees
  with the shared path contract")`. The end-user-facing AI-integration binary is, by contract, built
  in **debug** profile only. There is no cargo-release/optimized build target for `semio-os-mcp`
  anywhere in the repo.
- **No distribution channel.** No npm package wraps the binary (no `npx semio-os-mcp`), no Homebrew
  formula, no GitHub Releases workflow (`.github/workflows/` is empty), nothing under `dist/` is ever
  staged for anything but this repo's own `.mcp.json`/IDE configs to `exec`.
- **No codesigning/notarization.** `grep -rn "codesign|notariz|Developer ID|hardened runtime|entitlements"`
  across the whole tree (excluding vendored/tooling directories) → 0 hits. The only macOS-signing-
  adjacent code found anywhere (project memory `project-xcode-license-breaks-native-link-use-clt.md`,
  `🧑‍💻dev/⚙️engine/📤️publication/🟦️.ts:3-12`) is about picking the right local Xcode Command Line
  Tools so `cargo`/`cc` can *link* during development — unrelated to signing a binary for distribution
  to someone else's Mac, which would need Gatekeeper-acceptable signing/notarization that does not
  exist in this codebase at all.
- **`.mcp.json`'s "install" is "check out the monorepo and run bun."** The actual entry
  (`.mcp.json`): `{"command": "bun", "args": ["./📜️script.ts", "dev", "mcp", "stdio", "os",
  "--folder", ".", "--scopes", "…"]}`. `--folder .` binds the MCP workspace to **the repo checkout
  itself** as a generic probe artifact (G7 §1's `HeadlessWorkspace` finding) — this is a dev-loop
  self-test binding, not "point the MCP server at a document a real user owns." Every other client
  config the repo ships (`.cursor/mcp.json`, `.vscode/mcp.json`, `.windsurf/mcp.json`,
  `.kiro/settings/mcp.json`, `.codex/config.toml`, per M4 item 24) does the same thing. There is no
  worked example anywhere of registering `semio-os-mcp` against an arbitrary user's own space/folder
  outside this repo.
- **No mention of Claude Desktop (or any standalone client) anywhere.** `grep -rn "claude_desktop_config"`
  across the tree → 0 hits. There is no documented `claude_desktop_config.json` snippet, no
  installer, no guidance for a Claude Desktop / Claude Code (outside this repo) / other MCP client user
  on how they would even discover this server exists, let alone install and register it.
- **The rendezvous-with-a-running-shell mechanism is real and live-proven, but assumes a developer's
  machine.** M4 built `~/.semio/agent/bridge/{sessions,offers}` pid-liveness rendezvous (§1.2); M7
  proved a real stdio gateway dialing a real React `dev` session end to end (8/11 steps passed). This
  is the strongest piece of "C" that exists — but it is inherently "one already-running `dev s`
  session on the same machine as the MCP client," which is exactly the developer topology, not an end
  user's. `--hub` mode (which would let an MCP client attach to a **remote** hub-hosted space instead
  of a local dev session) is real code (`HeadlessWorkspace::open_hub`,
  `🏠️workspace/🦀️.rs:1474-1494`) with **zero production call sites** — G7 §3, M4 §5 item 5, and M7
  all independently confirm nothing in the product spawns `semio-os-mcp --hub` from an authenticated
  session. This is the single blocking gap between "a developer can watch an agent work in their own
  `dev s`" and "an end user can point an MCP client at their own hub space."
- **The approval gate that would make destructive AI actions safe for an end user cannot fire at all
  today.** M7 §5.4, measured against the live catalog: `"destructive": true` appears **zero** times
  across every plugin descriptor in the repo, and `ActionDefinition::destructive()`
  (`🛂️manifest/🦀️.rs:1037,1693`) has **zero** call sites anywhere. `ApprovalMode::WhenDestructive` —
  declared on 56 of `note`'s own 121 capabilities — is dead code at runtime as a result. M4 built a
  real three-lane approval chain (`--auto-approve`, MCP elicitation, live-shell dialog) and it is unit-
  tested, but it has never once been exercised live because nothing ever asks it to decide anything.
  For an end user this means: nothing an AI agent does through this MCP server can ever be gated by an
  approval dialog today, regardless of how destructive the action manifest claims it is.
- **README exists at the crate level and is honest, but is a developer README, not install docs.**
  `🧰️framework/…/🌉️mcp/README.md` (170 lines, read in full for this pass) is well-written and
  accurate about protocol/architecture/safety design — but its only "Run it" section (`README.md:8-14`)
  is `bun ./📜️script.ts dev mcp stdio os …`, i.e. requires the full monorepo, bun, and this repo's own
  build tooling. There is no separate end-user-facing doc anywhere.

## D. Documentation and first-run experience

- **README.md is 100% developer/contributor-facing.** Its table of contents (`grep -n "^#" README.md`)
  covers: Overview, Products (marketing copy — see below), Repo/principles/technologies, Git/GitKraken/
  Discord, Release/Tag/Branch/Commit conventions, **Development** (devcontainer, Windows setup, AI
  tooling for Claude Code/Codex/Cursor/Copilot/Windsurf, **CI/CD**), Examples, Brand, License, Security,
  Contributors, Stats. There is no "Getting started as a user," "Deploying," "Self-hosting," or
  "Installing the MCP server" section anywhere in it.
- **The "🛍️ Products" section (README.md:124-179) is aspirational marketing copy, not instructions.**
  `sketchpad` ("a simple-to-use, accessible and browser-based user interface for compose"), `studio`
  ("a synchronous collaboration environment for teams"), `cloud` ("use any file-hosting platform as an
  asynchronous Common-Data-Environment"), `assistant` ("helps you on every step in the design process")
  each get one sentence and a demo GIF/screenshot — these read as exactly the four outcomes this
  ticket is chasing (frontend, hub collaboration, storage, AI), described as if shipped, with zero
  links to install, run, or even a live demo URL.
- **The external docs site (`docs.semio-tech.com`, linked at README.md:2,11,42,154,954) documents a
  different, older product generation, not the current tree.** The only in-repo trace of that site's
  source is `temp/compose/client/lib/sketchpad/js/page/**/*.mdx` — and `temp/` is git-ignored with
  **zero tracked files** (`git ls-files temp | wc -l` → 0; confirmed via `.gitignore:6`). This reads as
  a scratch local clone of the pre-rewrite "compose" product's docs site, left on this machine by a
  previous session, not a maintained doc source for the current `s`/hub/MCP stack this ticket is about.
  Static reading cannot confirm whether the live `docs.semio-tech.com` covers any of the current
  React/wgpu/hub/MCP work — but nothing in this repository builds or feeds that site today.
- **CI/CD section is a stub pointing at a YouTube channel.** README.md:735-746: a `<details>` listing
  "TechWorld with Nana" (a devops YouTube channel) as the sole "Channel," then a paragraph describing
  local nx commands. As found in §A above, `.github/workflows/` is empty and the `publish` command
  this section names is an empty stub — so "CI/CD" in this README is entirely aspirational text over
  infrastructure that does not exist.
- **In-app onboarding: real, but scoped to per-plugin tutorials, not first-run/hub/MCP onboarding.**
  A genuine `IntroductionDefinition`/`IntroductionStepDefinition` state machine exists in the shell
  reducer (`🧰️framework/…/💻️os/🔨️modules/🖥️shell/🟦️.ts:59-61,139-141,419-427`:
  `autoStartIntroduction`/`setIntroductionStep`/`completeIntroductionInteraction`), with a
  command-palette entry `os.introduceApp` (`🛠️ShellHelpers/🟦️.tsx:4562`, icon `graduation-cap`,
  gated by `hasIntroduction`) and full en+de label resolution
  (`resolveIntroductionDefinition`, `🛠️ShellHelpers/🟦️.tsx:2929`). This is a real, working guided-tour
  mechanism **per plugin** (teaching a specific tool), not a first-run wizard for signing into a hub,
  understanding spaces, or connecting an MCP client — none of those three flows has any in-app
  onboarding surface found in this pass.
- **en+de bilingual coverage is a genuine, enforced product constraint, not a partial effort.**
  `["en", "de"]` locale lists recur as hard-coded, schema-validated constants across at least eight
  independent modules (`📓️print/🧬️schema/🟦️.ts:16`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:4224,4336`,
  `💻️os/🎚️config/🧬️schema/🟦️.ts:73`, `🛂️manifest/🟦️.ts:984`, `📜️script.ts:14464`,
  `🌎️hub/…/📜️script.ts:11451`, `🌎️hub/🔨️modules/🛡️admin/🧱️elements/📚️I18n/🟦️.tsx:236`, plugin
  authoring type constraints e.g. `✏️s/🔌️plugins/🧱️block/…/📜️script.ts:10`) — new UI strings that
  skip either locale fail a build check (M4 item 25's approval-actions label, U1's connection
  indicator, AU3's `os.openHub` command all had to add both). Strong point for D, but scoped to
  exactly two languages with no evidence of a broader localization roadmap.
- **The hub's own env-var/config surface has zero end-user-facing documentation** (cross-referenced
  from §A): 14 `OS_HUB_*` variables, `grep -n "OS_HUB" README.md` → 0 hits.
- **No `claude_desktop_config.json` or any standalone-MCP-client registration doc exists** (cross-
  referenced from §C).

---

## Ranked slice list

Ordered by value toward "a non-developer can reach one of the four outcomes." `cargo-heavy` marks a
slice that needs at least one foreground release-profile Rust compile.

| # | slice | outcome | size | cargo-heavy | why this order |
|---|---|---|---|---|---|
| 1 | Author `--auto-approve`/one real `destructive: true` capability + descriptor regen so the approval chain M4 already built can fire at least once live | C | S | yes (descriptor regen) | Single builder call (`ActionDefinition::destructive()`) unblocks the one thing M7 could not prove; makes "AI agent doing something irreversible is gated" real instead of unreachable |
| 2 | Wire `os-hub:publish`/a real `publish <slice>` map entry for at least `os-hub` (release binary → a versioned tarball or GH Release asset) | A | M | yes | `PublishScript`'s map is empty; this is the one missing link between "a release binary compiles" and "an operator can get it onto a server" |
| 3 | Give `semio-os-mcp` a real `--release` build target (mirror `os-hub`'s `BuildScript`) and drop the "profile must be debug" fixture assertion | C | S | yes | Currently *contractually* debug-only; trivial fix, blocks any real-world binary distribution |
| 4 | A minimal `os-hub` Dockerfile + one documented `docker run`/compose example with `OS_HUB_*` vars listed | A | M | yes (one release build inside the image) | Closes the biggest single "how would I even run this on a server" gap; can reuse the existing sqlite/postgres/neo4j feature flags already proven to compile |
| 5 | Write the operator-facing hub deploy doc: env-var table, sqlite file location, the `os-hub credential set` first-user verb, and an explicit "backup = stop + copy `OS_HUB_DATA`" runbook | A/D | S | no | Almost everything needed already exists in code (AU3's credential verb, G2's env-var census, H1's data-root layout) — this is pure writing, no engineering |
| 6 | Add a `SIGTERM`/`SIGINT` handler that calls the existing `shutdown_with_deadline` drain | A | S | yes | Small, but the current behavior (immediate kill under any orchestrator) is a real production correctness gap for the durable stores G2 already verified are real |
| 7 | One end-to-end run of `build-s-react-release` (or whatever completes the `s` distribution) with the result actually served and opened in a browser | B | L | yes (full plugin materialization) | Nobody has ever observed this path complete; G9/S2 already found `dev s` itself never finishes a cold boot for the same underlying reason (🧱️block staging) — this slice and S2 likely share the same root fix |
| 8 | `--hub` credential-delegation caller: something in the product that spawns `semio-os-mcp --hub …` from an authenticated session | C | M | no (mostly TS/Rust wiring, some cargo) | The single blocker between "developer watches an agent in their own dev session" and "end user points an MCP client at their own hub space" |
| 9 | Fix the CORS reflect-all-origins-with-credentials default to a configurable allowlist, document the loopback-vs-network `OS_HUB_BIND` distinction | A | S | yes | Cheap, but currently silent — a naive non-loopback deploy inherits an unreviewed security posture |
| 10 | An end-user-facing MCP install doc: a worked `claude_desktop_config.json` example, `--hub`/`--folder` explained for someone who is not this repo's own tooling | C/D | S | no | Pure documentation once #3 and #8 exist; premature before then since there is nothing installable yet |
| 11 | TLS-terminating reverse-proxy example (Caddy/nginx) + a documented systemd unit for `os-hub` | A | S | no | Standard ops boilerplate; low engineering risk, currently entirely absent |
| 12 | A first-run wizard inside `s` for hub sign-in/space creation, reusing the existing `IntroductionDefinition` mechanism | B/D | M | no | The scaffolding (per-plugin intro tours) already exists and is en+de-enforced; extending it to the hub-workspace flow is additive, not a new subsystem |
| 13 | A minimal upgrade/versioning story for the event-sourced stores (even just "schema_version stamped, refuse to open a newer one") | A | M | yes | Explicitly out of scope by design today ("greenfield… no migration framework") — lowest urgency until there is a real deployed user base, but worth a decision recorded rather than silence |

## Honest gaps

1. This is a static-only audit: no builds, no servers, no cargo were run, per slice rules. Every claim
   above is a source citation; "never observed to complete," "never executed," and "0 hits" are grep/
   read results, not runtime negatives proven by trying and failing.
2. This report does not re-verify any number from G2/G4/G7/H1/AU3/M4/M7 — it cites them and adds the
   deploy/onboarding lens those reports did not apply. If any of those numbers have moved since their
   own measurement (the tree is under continuous peer edit per the preamble), this report inherits
   their staleness.
3. `temp/compose/...` and `docs.semio-tech.com`'s actual live content were not independently fetched
   (no network access assumed for this slice) — the claim that the live docs site covers an older
   product generation is inferred from the local scratch clone being the only in-repo trace of it and
   from the "compose" branding predating this ticket's "os"/"s"/hub rename, not confirmed by reading
   the live site itself.
4. §B's claim that `build-s-react-release` is architecturally distinct from the broken dev launch rows
   (G4 §2a) is a static read of the generator (`🟨️.mjs:932-1066`); it was not run, so "distinct" means
   "a different code path," not "proven to work."
5. Did not check `os-hub --help`/CLI argument parsing exhaustively, nor whether any config file format
   (TOML/YAML) is accepted anywhere alongside env vars — the "env vars only" claim rests on the
   `std::env::var` grep in `🏗️bootstrap/🦀️.rs`, not an exhaustive walk of the whole crate.
6. Bundle size for a full `s` production build was not measured (would require actually running the
   build) — flagged as unmeasured, not asserted to be a problem.
