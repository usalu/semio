# 🔁️ Wave 5 Brief: Flip, Recover, Compile

Wave 5 starts 2026-10-16 05:00. It follows the wave-4 carves and the WP0–WP6 staging of the three big partitions. The previous coordinator session ended at about 04:40 with several executors cut mid-edit.

## State at the start (measured by the coordinator)

- Census (`🗑️generated/census.ts`, 04:50): 712 crates, 413 compilable, 6 oversized: `semio-framework-plugin` (206 files / 55,667 lines), `semio-framework-os-renderer-wgpu` (336 / 53,368), `semio-framework-os-kernel` (56 / 26,162, a carvable leaf), `semio-framework-plugin-host` (32 / 14,532), `semio-framework-plugin-host-test-component` (only through its dependencies), and `semio-framework-os-store-sync` (2 files / 4,736 lines, a carvable leaf).
- Image headroom is admitted again (4,325,515,264 of 4,294,967,296 bytes). The framework `Cargo.lock` was refreshed offline at 04:49, adding the peer's 10 ui-contract crates.
- The launcher now fails one step later, with `Complete observed Cargo compiler profile required` (a peer schema added 2026-10-15 22:57). `RESULT-INFRA-UNBLOCK-2.md` is being written by the unblock executor. **Once that file says the launcher reaches cargo, every flip must be proven by a real `bun repo cargo … check`** (lib, and `--tests` where the crate has tests) of the flipped crate and its direct dependents. Until then use the static evidence of `📓️RESOLUTION-ORACLE.md` and re-check the result file every 30 minutes or so.
- An orphan loop (`🗑️generated/lk-run.log`) is checking `cargo metadata --locked` over every workspace and updating stale locks. Leave it alone.

## Read first

`/Users/ueli/Documents/semio/AGENTS.md`, `📓️EXECUTOR-RULES.md` (all of it, including SPEED, no scratch compiles, and sandbox-only writes), `📓️CARVING-RULES.md`, `📓️RESOLUTION-ORACLE.md`, `📓️WAVE4-BRIEF.md` (including the god-object rule), and the PLAN-* and RESULT-* files your slice names.

## Rules added in wave 5

1. **Recovery first for resumed slices.** If your slice continues the work of a cut executor, start by establishing what it left: read its tool folder (state files, `touched.txt`, pre-edit snapshots, apply-state), compare the live files with the snapshots, and run the lost-item audit or the equivalent. Write a short "found state" section in your RESULT file before you change anything. Finish or roll back its half-applied batch. Never roll back anything that is not from that batch.
2. **Real compile once available.** Canonical command:
   `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p <crate>… --lib --keep-going --message-format=short [--manifest-path 🧰️framework/Cargo.toml]`
   - Framework-workspace members need `--manifest-path 🧰️framework/Cargo.toml` after `--`, or their dev-dependencies do not resolve (RESULT-COMPILE-PASS-1.md §1).
   - Run at most one launcher call at a time per executor. `Original selected owner registry changed` means a peer edited `📋️build-owners.json` while you ran: wait 60 s and retry.
   - A stale lock (`cannot update the lock file … --locked`) needs `-- update --workspace --offline [--manifest-path …]` first.
   - Attribute every error. Fix only errors your slice owns or that your flip exposed in your crates; for any other error, report it in your RESULT file under "foreign errors".
3. **Flip bottom-up, compile-atomic.** Flip one crate (or one rank of crates) at a time, check it, and only then flip the next. Never leave the tree in a state where a lower flip is pending under a higher flipped crate.
4. **Disjoint ownership.** Each slice below lists what it owns. Edits outside that set are limited to one-line consumer re-points; record each of them in your RESULT file.
5. **Deliverable.** `RESULT-<SLICE>.md` in this folder: verdict, found state (for resumed slices), what changed, census rows, every command run with its real result, foreign errors, hand-offs. Keep it current while you work: rewrite the verdict after each milestone, so a cut leaves a usable record.
6. Work in the foreground, spawn no sub-agents, call no ticket tool, and run no modifying git command.
   - Logs go only to `🗑️generated/<prefix>-*`.
   - Tools go in `<prefix>-tools/`.
   - Never delete anything in any `🗑️generated` folder.

## Slices

| Slice | Prefix | Owns |
| --- | --- | --- |
| INFRA-UNBLOCK-2 | `iu2` | the `🕸️units/📐️profile` schema/validator defect and later launcher defects (no gate bypass) |
| OS-KERNEL-ENGINE (resume of WP5/WP6, `okwp56-tools`) | `okeng` | `S/🦀️.rs`, the engine and above-engine crates of plan §10.1 WP5/WP6, `S/🏗️initialization/**`, `S/🧩️composition/**`, `S/🧾️document`, `S/🎚️config` |
| OS-KERNEL-SQLITE-HOST (resume, `oksh-tools`) and then WP7 sync | `oksql` | `S/🚪️io`, `S/📦️codec/**`, `M/🚪️io/**`, `S/📜️space-history/**`, `S/🔄️sync/**`, `F/🏃️process/🧭️routing/🏠️workspace`; only the matching mount lines of `S/🦀️.rs` |
| PLUGIN-INTEGRATION-1 (flip WP3, then WP4, then WP5) | `pi1` | the plugin hubs `🪝️mounts/**`, the WP3/WP4/WP5 crate roots, manifests and staging trees, the facade's `pub use` lines for them |
| PLUGIN-WP7 (instance tier 2/3, builder, testing) | `pwp7` | `🧵️instance/**`, `🪜️*`, `⏪️time-travel/🧵️instance`, `⏯️tool-run/🧵️instance`, `🏗️builder/🦀️.rs`, `🧪️testing`, `⚖️authoring-law`, the WP6 tier-1 staging |
| RENDERER-INTEGRATION-2 (Shell tier-1 flips, frame-core, WP5 flip) | `ri2` | `🐚️Shell/**`, `🏛️ShellHost/**`, the frame-core trees of plan §10.1 WP3, the renderer hubs |
| PLUGIN-HOST-CARVE | `phc` | `semio-framework-plugin-host` (`🔌️plugin/🖥️host/**`) and its test component |
| COMPILE-PASS-2 (after INFRA-UNBLOCK-2) | `cp2` | the compile-and-fix loop over compilable crates outside the slices above |
