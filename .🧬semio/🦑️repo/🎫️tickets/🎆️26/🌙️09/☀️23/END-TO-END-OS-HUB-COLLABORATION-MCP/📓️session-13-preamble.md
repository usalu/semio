# Session 13 Preamble

Session 13 of the repo goal (2026-09-26 19:00, Claude Code fleet). Coordinator = main Claude Code chat (Opus 5.5, session
"End-to-end repo completion"). Executors = Opus 5.5 agents, one slice each. Auditors = Sonnet 5 agents, read-only. Ticket
folder (ASCII entry `/Users/ueli/Documents/semio/.tmp-ticket/`). Canonical history `/Users/ueli/Documents/semio/.tmp-ticket-0918/`.
Session-12 rules (`📓️session-12-preamble.md`) apply unless overridden here. Fleet handles: `📓️fleet-13-agents.md`.

## The Goal (four outcomes, all end to end)

1. Working os `s` frontend with all plugins and artifacts.
2. Working hub server backend (db, presence, auth, observability).
3. Working collaboration between users over the hub (React shell and wgpu shell).
4. Working AI integration for users over the **semio** MCP (`semio-framework-os-mcp`, `mcp__semio__*`), never the repo MCP.

## Situation at 19:00

- The session-12 coordinator and all its agents are gone (their agent ids cannot be resumed). Every slice report
  `📓️wp-<slice>.md` is the handover; the new agent that takes a slice continues that report.
- **Publish 4 (`--packages all`) FAILED 18:45** after 152 min (`.🧬semio/🌐hub/s12-w2-logs/publish-w2-catalog-all-2.txt`,
  samples `publish-all-4-samples.txt`): 15 packages complete, then the 16th (imperative, last wasm rustc
  `semio_s_plugin_imperative`) failed in `buildClosedBrowserActorArtifactOwned` (`🔌️plugin/🌐️browser-bundle/📜️script.ts:317`):
  `browser actor artifact: unsupported import interface` (jco `generate` returned an import outside the admitted list).
- Hub **7800 is READY on catalog B2** (9 packages, 16 kinds, binary from 02:31, hold pid 28673 / hub 54029). It stays until
  the all-package publish replaces it. Hub 8050 (hold 78966 / hub 79010) and serves 6552/6553 (21493/50393) are WG7's lineage,
  serve 6590 (35215) is WG8's; they belong to WG9/WG10 now.
- **Decision (coordinator, 19:0x): the landing window opens NOW.** Publishing the frozen tree again and then republishing after
  the landing costs two 3–4 h chains; the landing carries the collaboration blockers (guest re-announce, late joiner), the
  hub creation speedups (Q1, codec-app resolution) and the MCP/plugin fixes. Sequence: landing (all prepared sets, compile-atomic)
  → imperative codegen fix → ONE consolidated `rebuild-all` (describe → materialize → generate → check → activate-s → verify-s)
  → ONE `--packages all` publish → hub 7800 onto it on the current-tree `os-hub` → live verification of every outcome.
- A **Codex peer** (ChatGPT app, codex app-server pid 96882) works on the wgpu renderer (`test-wgpu-unit`, `serve-puzzle3d-wgpu-dev`,
  trunk builds). Never kill its processes; share the machine.
- Machine: M1 Max, 10 cores, 32 GiB RAM, 133 GiB free. Builds are the bottleneck.

## Rules (override session 12)

1. **Rules 20, 21 and 24 of session 12 are LIFTED** (no guest freeze, no build-quiet). Rule 18 (`nice -n 15` for everyone but
   W2) is replaced by rule 3 below. The coordinator's renice watch is stopped.
2. **Landing is compile-atomic.** Re-run a prepared set's dry run on the current tree first; re-derive any hunk that no longer
   applies (never force, never overwrite a peer's newer code). Apply → immediately `cargo check -p <every touched crate>`
   (`--lib --tests`), plus `--target wasm32-wasip2` (guest/framework crates) or `wasm32-unknown-unknown` (renderer, kernel
   `sync`) where cfg(wasm) or guest code changed → fix or revert your own hunks. Record every landed set as one row in
   `📓️landing.md` under `# Session 13 Landing Window` (slice, set, crates checked native/wasm32, result + capture, time).
   Landing agents add `crate + reason` for every guest crate whose code changed to `.tmp-ticket/wp-w3/requests/<slice>.txt`.
3. **Priority:** landing slices (LA, LB, LC, LD, LE) and W3 run builds at normal priority; every other slice prefixes every
   cargo/nx/bun build or test command with `nice -n 10`. The critical path is landing → rebuild → publish.
4. **wasm32 builds/checks** go through `zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm <slice> -- <cmd…>`
   (FIFO, one at a time, ≤ 25 min per hold; split larger work). Lock order wasm → hub; never hold hub around a wasm build.
   From the moment W3 announces "REBUILD START" (in `📓️wp-w3.md` and by the coordinator) until "PUBLISH DONE": nobody
   except W3's coordinator-launched chain runs wasm32 builds, and nobody edits guest-linked crates, the kernel derive inputs
   (`📚️library/🔣️taxonomy.json`, root `nx.json`, `📋️project.json` files, root `Cargo.toml`/`Cargo.lock`, `.cargo/config.toml`).
   Before that announcement guest edits are allowed (and wanted: land them now).
5. **Cargo:** `-p <crate>` only; `CARGO_INCREMENTAL=0`; one cargo from you at a time; binary-producing commands (test, build,
   run) with `CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-<slice>/target` (build-dir stays shared; never change
   it); `--no-fail-fast` before `--`. Above 14 running rustc (`ps -axo command | /usr/bin/grep -c '^[^ ]*rustc '`) wait in ONE
   blocking loop (`until …; do sleep 30; done`, single call, `timeout: 600000`), never many short polls.
6. **Foreground, no helpers** (session 12 rule 5): never `run_in_background`, never `Monitor`, never sub-agents (no Agent tool),
   never worktrees. Anything longer than 10 min: `setopt no_bg_nice; nohup … > <capture> 2>&1 & disown` (record the pid), then
   wait on the capture in one blocking call per ≤ 10 min. Never end your turn while an undetached build of yours runs.
7. **Chains longer than ~45 min** (rebuild-all, catalog publish, hub 7800 restart) are launched by the COORDINATOR with
   `python3 .tmp-ticket/wp-w2/w2-detach.py <log> <cmd…>`; the owning agent prepares the exact command, sends it to `main`, then
   monitors the log.
8. **Kills:** only pids you started (verify with `ps -o pid,ppid,command`). Never `pkill`/`killall`/pattern kills.
9. **Ports (session 13):** 7800 = W3 only (never bind or default to it: pass your slice port explicitly). Hubs / serves:
   W3 8000–8009 / 6500–6509; H11 8010–8019 / 6510–6519; C11 8020–8029 / 6520–6529; G11 8030–8039 / 6530–6539;
   S16 8040–8049 / 6540–6549; WG9 8050–8059 / 6550–6559 (keeps 8050, 6552, 6553); T13 8060–8069 / 6560–6569;
   LA 8070–8079 / 6570–6579; F2 8080–8089 / 6580–6589; WG10 8090–8099 / 6590–6599 (keeps 6590); Z3 8100–8109 / 6600–6609;
   LB 8110–8119 / 6610–6619; LC 8120–8129 / 6620–6629; LD 8130–8139 / 6630–6639; DB1 8140–8149 / 6640–6649;
   LE 8150–8159 / 6650–6659; H12 8160–8169 / 6660–6669; R9 8170–8179 / 6670–6679; V1 8180–8189 / 6680–6689;
   S17 8190–8199 / 6690–6699; N1 8200–8209 / 6700–6709. `lsof -nP -iTCP:<port> -sTCP:LISTEN` before binding.
10. **Durable data** under `.🧬semio/🌐hub/s13-<slice>-<name>/` (gitignored, outside every sweep); `wp-<slice>/generated/` holds
    only expendable captures (≤ 1 MB each). Never create data in tracked paths (`git check-ignore -q <path>/x` first).
11. **Report:** continue (or create) `📓️wp-<slice>.md`: add `## Session 13` right under the title block — a short status table
    of this session's items FIRST, then a timestamped log. Update after every landed item. Measured results only; "written,
    not run" is honest; never claim a pass you did not run. Chat answer ≤ 10 lines pointing at the report.
12. **AGENTS.md applies** (session 12 rule 10 verbatim): schema-first; no legacy/compat/shims/fallbacks/deprecations; no
    migration scripts in the repo (one-off codemods live in `wp-<slice>/`); emoji-first docstrings; no comments inside
    definitions; no `[DEBUG]` leftovers; bun + nx; permanent scripts only in `📜️script.ts`; new runnable commands in
    `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`; en + de for every user-facing string; language-agnostic test +
    third-party oracle for every feature; progress + cancellation for expensive operations.
13. **Git:** never any modifying git command. Read-only git is fine.
14. **Peers edit the same files** (other Claude slices, a Codex peer, human devs): re-read before editing, keep to your scope,
    diff before assuming a vanished symbol is yours to restore. Fix a peer's compile break only when it blocks you and the fix is
    one obvious line; note it in your report.
15. **Grep:** `/usr/bin/grep` (the shell `grep` alias is ugrep and misses matches on big emoji files). Quote every emoji path;
    `cd` explicitly in every Bash call. macOS has no `timeout` command. zsh: `setopt no_bg_nice` before `nohup … &`.
16. **Do not** close/reopen the ticket, edit `🎫️ticket.json`, touch `📌️important.md`, or delete `🗑️generated`/other slices' files.
17. **Blocked on another slice?** Write the exact blocker into your report, `SendMessage` to `main` in ≤ 5 lines, continue with
    your next unblocked item. Never wait idle, never poll.
18. **Usage cuts:** the coordinator resumes you by SendMessage; write the report early and often.
19. **Disk:** the coordinator's lock-aware disk guard (pid 717) runs. Below 80 GiB free: stop starting builds, tell `main`.
20. **Current-tree hub without a build (19:4x):** hub 7800's 02:31 binary answers `/directory/spaces` in ~84 s for users with
    many spaces. Live testing uses C11's "Current-Tree Hub Recipe" (top of `📓️wp-c11.md`: copy the 19:07 os-hub binary + a B2
    catalog clone onto your own port) until W3 moves 7800 to the all-package catalog on a current-tree binary.
21. **New harnesses are permanent (19:5x):** any harness you write or extend for acceptance goes into a `📜️script.ts` verb + nx
    target + launch row (V1 productizes the existing ticket-local ones; coordinate with V1 so you don't fork them).
22. **Memory budget (20:0x, swap 24.3/25.6 GB):** stop every server, hub, serve and browser you are not using THIS hour (record
    the stop in your report; restart later from your recipe); at most ONE headless browser per slice at a time; non-landing
    slices wait until fewer than 10 rustc run (`ps -axo command | /usr/bin/grep -c '^[^ ]*rustc '`) before starting a cargo
    command, landing slices + W3 keep the 14 bound; no parallel cargo/nx from one slice; prefer `cargo check` over `cargo test`
    builds until your code compiles; the Docker VM (pg + neo4j, 8 GB) is shared — never start a second pg/neo4j, reuse
    `semio-hub-backend-{postgres,neo4j}`; no Docker image builds until the coordinator lifts this rule (Z3).
23. **Envelope wire change → fresh data roots (20:1x, LD item 2):** `MutationEnvelope` gains `observed` (advisory newest foreign
    op the author had applied) + `fields` (declared field keys); record + batch codecs change (Rust + TS twin + fixtures). Greenfield:
    no replay of older WALs, no compatibility path. After W3's consolidated rebuild every hub (7800 and every slice hub) runs a
    current-tree binary on a FRESH data root; C11's 19:07 recipe binary is valid only with pre-rebuild guests and old roots.
24. **Poisoned fingerprints (20:14, Z3):** 351 native debug fingerprints (157 crates incl. replication + kernel deps) had dep-info
    pointing into session 12's P8 scratch clone, so cargo reported them Fresh against the real tree (false greens possible since
    ~16:00). Z3 deleted those `dep-*` files at 20:14 and is scanning every other target. **Every green `cargo check`/test you
    recorded before 20:14 today is suspect: re-run it and update your landing row.** Never build a scratch clone of the repo
    with the shared build-dir (give it its own `CARGO_BUILD_BUILD_DIR` and `CARGO_TARGET_DIR`).
25. **Lock convoys (20:3x):** the shared build-dir's fine-grain locks make every cargo hold SHARED locks on all units it uses for
    its whole run, and a cargo that must rebuild an edited unit waits for EXCLUSIVE until all of them end (12 of our cargos waited
    20 min behind a 45-min `cargo test` build of a Codex peer). Until REBUILD START: non-landing slices run only `cargo check`
    (no `cargo test`/`nextest`/`build` of crates that depend on kernel/replication/plugin/framework) unless the coordinator
    approves; landing slices keep test builds short (`-p <one crate> --lib`, filtered). A cargo of yours that waits > 15 min
    with no rustc child: stop it (it's yours) and retry later rather than stacking more waiters.
26. **Second build-dir for non-landing slices (21:0x, until REBUILD START):** ~25 cargos convoy on the shared build-dir and the
    landing slices (critical path) wait 15–20 min per check. Until REBUILD START every slice EXCEPT W3, LA, LB, LC, LD prefixes
    every cargo command with
    `CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b`
    (gitignored; same sources, so results are equally valid; first build there is cold). Keep `nice -n 10`, one cargo at a time,
    and your private `CARGO_TARGET_DIR`. The coordinator deletes `build-fleet-b` at REBUILD START (then build-quiet applies) —
    never store anything else there. W3/LA/LB/LC/LD stay on the default build-dir. Slices doing guest landings (WG9, WG10, G11,
    S16, S17, T13, N1, R9, Z3) use build-fleet-b too; their wasm32 checks still go through the wasm mutex.
27. **Landing build-dir (21:1x, until REBUILD START):** the default build-dir is jammed by a Codex peer's 8 cargos (many long
    `cargo test --no-run` builds) plus orphans. W3 (native checks), LA, LB, LC, LD now prefix every cargo with
    `CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-landing` (cold first build; gitignored;
    deleted by the coordinator at REBUILD START). Everyone else stays on `build-fleet-b` (rule 26). The default build-dir is
    left to the Codex peer until W3's coordinator-launched rebuild chain, which uses the default. The coordinator stopped our
    stuck cargos in the default build-dir at 21:1x (0 % CPU, no rustc, 6–34 min): just re-run yours in your new build-dir.
