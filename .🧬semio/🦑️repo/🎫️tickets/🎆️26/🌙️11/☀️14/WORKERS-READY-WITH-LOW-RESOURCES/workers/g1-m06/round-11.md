# g1-m06 Round 11

Running. One block per try is appended as each try ends; a summary follows at the end of the round. Machine: Mac mini, 16 GB memory, macOS (Darwin 25.6.0). Replica agent running, `bootVersion` 6, code digest at 00:12Z: `eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc`. My runner stops a try whose graph step shows no new progress line for 15 minutes (30 samples), as this round asks of g1-m06.

## Try 1

- start 2026-11-14T23:56:20Z, end 2026-11-15T00:12:15Z, `native_rc=75`, hard stop: none.
- Pressure levels of the samples (count, level):  32 pressure_level=1;. Peak resident: 1387264 KB.
- Launcher after the try: inode 3631707 mtime 2026-11-14T22:55:40+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=a0f39e389ce8a697c89d7f9357e72d37eb06ab0d ; marker absent.
- EPERM lines in the run log: 0.

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] admitted run: 8 job(s), 1289 MB reserved (9037 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] dependencies: sized at 22% of the default from 15 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 1341 MB reserved (9004 MB available, 2048 MB floor)
[capacity] admitted run: 1 job(s), 2048 MB reserved (8944 MB available, 2048 MB floor)
Reading the project graph slice (933 s): flush 1 of 1.
Reading the project graph slice (935 s): flush 1 of 1.
Planning 1 targets.
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 11 and delivered 0 within 901 s, 11 are still missing, among them the too heavy project(s) @semio-tech/cargo-workspaces (about 4470 MB) (this machine can give about 3512 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-15T00:12:15Z
native_rc=75
```

## Try 2

- start 2026-11-15T00:14:51Z, end 2026-11-15T00:29:55Z, `native_rc=1`, hard stop: memory pressure level 2 in six samples in a row.
- Pressure levels of the samples (count, level):  25 pressure_level=1; 6 pressure_level=2;. Peak resident: 1430752 KB.
- Launcher after the try: inode 3631707 mtime 2026-11-14T22:55:40+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=a0f39e389ce8a697c89d7f9357e72d37eb06ab0d ; marker absent.
- EPERM lines in the run log: 0.

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] admitted run: 8 job(s), 1289 MB reserved (8608 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] dependencies: sized at 22% of the default from 15 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 1341 MB reserved (8604 MB available, 2048 MB floor)
[capacity] admitted run: 1 job(s), 2048 MB reserved (8575 MB available, 2048 MB floor)
Reading the project graph slice (881 s): owners 69 of 659; running (name, kind, seconds, rows): @semio-tech/framework-actor-rs (language:cargo, 133, 0); waiting for memory headroom (8057 of -7777 MB reserved).
Reading the project graph slice (883 s): owners 69 of 659; running (name, kind, seconds, rows): @semio-tech/framework-actor-rs (language:cargo, 133, 0); waiting for memory headroom (8057 of -7777 MB reserved).
Cancelled (SIGINT); leases released.
Internal error: Original Script process interrupted
end 2026-11-15T00:29:55Z
native_rc=1
```

## Try 3 (cut by a restart of the machine)

- start 2026-11-15T00:33:25Z; no end line and no `native_rc`: the Mac restarted at 03:57Z (macOS is now 27.2.1; `last reboot` says 04:57 local). I did not start that restart; whether it was an automatic system update is not measured. The last log line is from 03:51Z.
- First try on this machine that reached the setup tasks: `NX Running target setup for project workspace and 29 tasks it depends on`. Succeeded (14): `ui-rs:generate`, `assets:build`, `ui-styling-tokens:generate`, `framework-schema:generate`, `dsl-derive-rs:generate-schema-index`, `repo-lib:cpp-setup`, `repo:generator-inputs`, `repo-mcp:build`, `plugin-registry:generate`, `repo-mcp:install`, `ui-styling-py:deps`, `repo-test-dotnet:deps`, `energy-oracle-py:deps`, `framework-graph:generate`. The other 15 did not end before the restart.
- Failure seen: `Cargo artifact build failed: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml`. First error line and the 15 lines before it:

```
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🏪️stores/📦️packages/🦀️rust/../../📥️staging/🧱️components/🦀️.rs:7:5
  |
7 | use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `pack::snapshot::ArtifactPack`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🏪️stores/📦️packages/🦀️rust/../../📥️staging/🧱️components/🦀️.rs:8:5
  |
8 | use pack::snapshot::ArtifactPack;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `semio-plugin-instance-stores` (lib) generated 142 warnings (run `cargo fix --lib -p semio-plugin-instance-stores` to apply 97 suggestions)
   Compiling witx v0.9.1
   Compiling fxprof-processed-profile v0.8.1
   Compiling addr2line v0.26.1
error[E0460]: found possibly newer version of crate `hashbrown` which `gimli` depends on
  --> index.crates.io-1949cf8c6b5b557f/addr2line-0.26.1/src/lib.rs:42:1
   |
42 | pub extern crate gimli;
error: could not compile `addr2line` (lib) due to 1 previous error
```

- Standstill: `Another process holds the dependency store; waiting for it` from (0 s s) to `(9807 s of at most 21600 s)` at 03:51Z. Round 14 says to interrupt that after 5 minutes; this try ran under my round 11 runner, which has no such rule, and I did not see rounds 12 to 18 until 03:51Z. The holder is not named in the line; `ps` at 03:51Z showed three `bun --eval` processes aged 2:43 to 3:00 h (pids 61000, 64692, 72456) and an nx process (68772).
- Samples: 389 at pressure level 1, 2 at level 2; no hard stop. Peak resident 1638912 KB (bun of the run). Replica agent kickstarted by my runner 4 times (01:30:44Z, 01:44:09Z, 02:48:54Z, 02:53:46Z).
- Launcher: the run log says "The repo launcher is current, nothing was replaced".

## Summary of Round 11

Not ready, 3 tries: `native_rc=75` (fleet delivered 0 of 11 in 901 s), `native_rc=1` after a hard stop (pressure level 2 six samples in a row), then the try above, cut by the restart. Marker absent. Next tries run under round 18.
