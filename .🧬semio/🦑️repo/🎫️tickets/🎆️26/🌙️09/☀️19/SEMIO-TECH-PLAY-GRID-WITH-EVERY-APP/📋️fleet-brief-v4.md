# 📋️ Fleet Brief v4 (session 5, 2026-09-21 13:55) — addendum to `📋️fleet-brief-v2.md` + `📋️fleet-brief-v3.md`

Read `AGENTS.md`, then `📋️fleet-brief-v2.md` (ALL hard rules, fixing rules and stale-test buckets apply verbatim),
then `📋️fleet-brief-v3.md` (machine + dev-server rules), then this file. `$T` = this ticket folder.

## What happened since v3
- The whole `$T/🗑️generated/` folder was SWEPT between 2026-09-20 02:35 and 2026-09-21 13:45 (every topic STATUS.md,
  baseline log and audit result is gone). Only tracked files survived: `📓️status.md`, `📓️app-boot-defects.md`,
  `📓️audit-artifacts.md`, the briefs and the `🧪️*` scripts. Play itself is committed (last commit 2026-09-20 23:20).
- Play unit suite now 58/62; the four reds are all the missing `stdio` pane (no pane, no activation lane, nine
  `s.stdio.*#editor` apps unreachable). The catalog has 60 panes in 8 groups incl. `home`, `space`, `imperative`,
  `playbook`; 27 activation lanes.
- Peers (two other Claude sessions + a Codex fleet) are sweeping `fn label(&self) -> protocol::LocalizedLabel`
  across every plugin's mutation files (~3 400 uncommitted files) and rewriting `🧰️framework/🛍️products/💻️os`.
  Never revert or "fix" their in-flight edits; if a peer edit breaks your crate's compile, wait a few minutes and
  re-run before touching it, and only then fix minimally.
- An auto-committer commits the tree periodically; you never run git write commands.

## Durability rules (new)
- Keep `$T/🗑️generated/<topic>/STATUS.md` current as before, AND at the end write your final report as a TRACKED
  file `$T/📓️<topic>.md` (tracked files survive sweeps). Never sweep, delete or `git clean` anything.
- The coordinator runs the only full plugin baseline (`$T/🗑️generated/baseline/summary.tsv`, one cargo at a time
  with the shared build dir). Fix agents re-run ONLY their own crates, batched into one cargo invocation, with
  `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 RUST_MIN_STACK=33554432 DEVELOPER_DIR=/Library/Developer/CommandLineTools`.
  Disk is at ~59 GB free: no private `CARGO_TARGET_DIR`, no `--release`, prune only stale incremental dirs (v2 rule).
- Dev server `:6033` = `$T/🔁️serve-supervisor.sh` (coordinator-owned, FROZEN Vite; recycle via
  `touch "$T/🗑️generated/serve-restart.request"`, see v3). Ports 6033/6040/6056/6063/6076/6080 are owned by others.
- Do not spawn sub-agents; run builds in the foreground of one Bash call (timeout up to 600000 ms), re-attach by
  tailing your log.

## Definition of done (unchanged from v3)
Play shows EVERY plugin (every editor app of every plugin descriptor incl. stdio, space, imperative/flow extensions in
the activation union); every pane boots to `data-shell-ready` with its curated default example and VISIBLE correct
content; no page errors, console errors or refused inputs; play unit suite + strict acceptance suite
(`bun nx run @semio-tech/semio-tech-play:test-e2e`) green; every plugin crate's native test suite green.

## Peer constraints (14:20, from the `End-to-end repo completion` session, ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END)
- EVERY wasm32 cargo command (build, check, activate, describe — incl. `cargo check --target wasm32-wasip2`) must run
  through the fleet mutex, or the shared build dir deadlocks:
  `zsh "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/📜️wasm-build-mutex.sh" <your-topic> -- <command …>`
  (FIFO queue; a long stdio wasm-release build holds it until ~15:00 — batch your wasm32 checks into one command).
- Do NOT edit for the next hours: `🌎️hub/**`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `🌐️browser-bundle/**`,
  `🏪️store/👷️worker`, `🏛️ShellHost` document-mount/presence paths, `🌉️mcp/**`, `🧰️framework/🔨️modules/🛂️manifest`
  `ExampleDefinition`/`ExampleSource`, and the GUEST code of `✏️s/🔌️plugins/{🪐️space,🌊️flow,🎬️sequence,🌀️procedural,📕️norm,📖️playbook}`
  (peer slice S10). Never `cargo test/build -p semio-hub` or `-p semio-framework-plugin --lib`. For those six plugins:
  diagnose, fix test-side/framework-general defects, and put any guest-code change as a PROPOSED DIFF in your report
  (the coordinator hands it to the peer). Ports 7611–7671, 6071, 6081, 6190, 6191 are theirs.
- Activation lanes: all lanes stage into one `🔌️plugin-modules` root and play's merge rejects lanes disagreeing on a
  component sha; the `demonstrator` lane covers cad, flow, gis, procedural, process, puzzle, sourcing (+extensions).
  Do NOT run `activate-*-react-dev` yourselves — ask the coordinator (`touch "$T/🗑️generated/activate.request/<lane>"`)
  and the coordinator runs the consistent set through the mutex.
