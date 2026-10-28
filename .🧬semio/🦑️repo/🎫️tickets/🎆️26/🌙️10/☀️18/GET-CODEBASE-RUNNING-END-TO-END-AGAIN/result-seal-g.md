# result-seal-g (wave 1026g): the sealed flip as a one-shot landing

`C` = this ticket. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `B` = `L/🏗️build`. Tools `C/seal-g-tools/` (+ the fixed `C/self-g-tools/{flip.py,post-flip.sh}`, originals kept as `*.pre-seal-g`). Mirrors `seal-a`, `seal-c` dropped (DMGs detached first). No live edit, no official run, no library batch staged (nothing in the library needed fixing for the flip), ticket not closed.

FOR gate-g: GO for the sealed flip, land it LAST. Recipe below. No batch to apply from this lane.
FOR chain-g: a sealed LIGHT command costs +1.1-2.6 s and +240 MB over live (measured below). The cost is the loader start: `Buffer.from(base64)` + round-trip `toString('base64')` + sha256 of every one of 1556 files + JSON.parse of 19 MB + sha256 of the whole text, on every start, although the whole-text sha256 against the digest already authenticates the snapshot (`C/seal-g-tools/profile-light.py`: spawn -> exit 1.05-1.66 s for `repo --help`). Lazy per-file verification would remove most of it; the loader digest is part of the identity, so it is a deliberate edit, not a tonight edit.
FOR host-capacity owner: the startup law row `leases-entry` is red WITHOUT the flip (same on live, on a mirror, with all sealed-touching batches applied): `🔒️leases/⌨️command` -> `🌡️host-capacity` -> `process/📦️packages/🟦️typescript/🪪️agent-sessions` -> `process/🧪️testing/🏖️sandbox` (group `test-harness`, 8 modules) and `🌡️host-capacity/🔼️upgrade` -> `🧭️routing/📥️invocation/🏃️process/🐕️watchdog` (group `routing`, 4 modules).
FOR mirror-g: (1) a mirror that runs the REAL tool needs `--only library process launcher "✏️s/🧑‍💻dev/🧩️catalog/📣️publication"` (two data files of that directory are in the sealed closure; without them the promotion fails with `ENOENT lstat <root>/✏️s`); (2) a real-tool run in a mirror provisions two DMG images (`.cache/b/l`, `.cache/b/o`) mounted below the mirror: `hdiutil detach` them before `drop`; (3) the macOS seal denies `ps` (use `--no-seal` for process-tree work; the guard still runs).

## GO / NO-GO: GO (land last, with the post-flip check and the rollback ready)

Why: with the flip applied on a mirror, all five self-g law files and the door law are green, the real tool runs sealed through the Go launcher and the pinned bun, the rollback is one byte-identical JSON line, and the isolation property holds on a real process. What it costs: light commands are slower (below), the first promotion after any closure edit takes 30-60 s (single-flight), and Windows/Linux are untested (the same as self-g said).

## (1) Flip + post-flip + laws on a mirror (`seal-a`: live tree + flip; `seal-c`: live tree + setup-g + oracle2-g + conv-g + obs-g fleet + flip)

| check | seal-a | seal-c (4 batches + flip) |
|---|---|---|
| `🏛️sealed/🧪️tests/🗂️store` | 12 pass 0 fail | 12/0 |
| `🏛️sealed/🧪️tests/🚦️trampoline` | 5/0 | 5/0 |
| `🏛️sealed/🧪️tests/👥️processes` | 3/0 | 3/0 |
| `🏛️sealed/🧪️tests/🔀️interleavings` | 3/0 | 3/0 |
| `🚪️door/🧪️tests` | 7/0 | 7/0 (one load flake, see F2) |
| `🚪️door/🧪️tests/🚦️startup` | 4 pass 1 fail | 4/1 (same row as on live) |
| real tool, sealed: `repo --help`, `leases status`, `repo cache status` | exit 0, promotion 1546 files 41.9 s | exit 0, promotion 1556 files 60.9 s (19.2 MB snapshot; limits 2048 files / 16 MiB source / 24 MiB snapshot: source ~14.4 MB, headroom ~2.3 MB) |
| 3 heavy commands at once on a drifted tree | 1 promotion (32 s), 2 waited ("Waiting for the promotion that process N runs"), all exit 0, 47 s | |

Failures and causes (all of them):
- F1 `leases-entry reached heavy groups` (startup law): not caused by the flip, red on the unflipped live tree too; chain in the FOR line above. Left alone (outside this lane; library edits only through gate-g).
- F2 `the real front door prints the same help…` timed out once at the 5 s default of `bun test` (load 45-55): 3 of 7 runs; with `--timeout 120000` green every time. `post-flip.sh` now passes the timeout.
- F3 the original `post-flip.sh` could not run anywhere but the live tree (hard-coded `cd /Users/ueli/Documents/semio`), had no promote step and no exit status. Rewritten: root = argument or cwd, `PB` override, `--timeout`, `promote` before the real-tool lines, `POSTFLIP exit N`.
- F4 the original `flip.py` was cwd-relative without root choice, idempotence or status; my first rewrite defaulted to the script's own repository, which on a mirror hit the LIVE tree (the macOS seal stopped it: EPERM, nothing written; this is why the final root rule is cwd first, script second). Now: `flip.py [apply|revert|status] [--root DIR] [--dry]`, whole-line anchor, JSON re-parsed before and after, atomic write, exit 3 when already in the wanted state, `status` exit 0 = sealed / 1 = live.
- F5 one unexplained `repo --help` exit 2 with no output (1 of 13 sealed runs that were frozen with SIGSTOP right after spawn and thawed 0.5 s later); not reproduced in 11 more. It happened while the host was at the file-table limit (`ENFILE` seen twice in this session at 19:5x/20:0x). Not proven to be the flip.

## (2) Is the sealed snapshot taken from the live tree at flip time?

YES for every promotion, with one trap. A promotion (`B/🏛️sealed/🔁️promoter`, from the trampoline) captures the LIVE tree when (a) the mode is `promote`, (b) no pointer/receipt exists, or (c) the command is not light and the stat fingerprint of the live closure drifted. Proof: in `seal-c` the batch files overlaid after the mirror was cloned were in the promoted snapshot (1546 -> 1556 files).
The trap: LIGHT commands (`repo --help`, `leases status`, door policy `light`) never promote and run whatever the pointer holds. The real store `.🧬semio/🦑️repo/⚡️cache/implementation/f4f42a1b35aea802/` points at a snapshot promoted 10:35 (10 h old, 1520 files). Measured on a mirror (`stale.py`): an edited help catalog is seen by live (True), not by a sealed `--help` before a promotion (False), seen after `SEMIO_REPO_IMPLEMENTATION=promote` (True; 30.4 s). So the recipe promotes explicitly right after the flip (`post-flip.sh` does) instead of letting the first agents run a 10-hour-old implementation against current on-disk protocols.

Staged batches vs the flip (scan of every `C/*-tools` for `🏛️sealed`, `🚪️door`, `defaultMode`, `SEMIO_REPO_IMPLEMENTATION`, then base-hash and diff checks):
- conv-g (`conv-g-tools/staged/files`): rewrites 8 sealed files, among them `🏛️sealed/🔣️.json` WHOLE (adds `maximumTextCharacters/PathCharacters/CommandLineCharacters`); its base hash equals the live file now (7eeebd99dc61), the `"defaultMode": "live"` line is byte-identical in its version. Consequence: conv-g MUST land before the flip (after the flip its base check fails or, with a plain copy, it would silently put `live` back). Lands after oracle2-g.
- obs-g fleet: edits `🚪️door/🟦️.ts`, `🚪️door/🔣️.json`, `🚪️door/🧪️tests/🚦️startup/🔣️.json` (+ 12 new modules the door imports). Not the flip anchor.
- route2-g: test-only change in `🏛️sealed/🧪️tests/🚦️trampoline/🟦️.ts` (clears LANG/LC_* in the child env) and suite-table rows. setup-g: zero-touch closure table row for the door script. xplat4 platforms: suite rows. Not anchors. (zt-g is landed.)
- No staged batch edits the flip tooling, `flip.py`'s anchor line, or the trampoline/promoter/store modules except conv-g's docstring and policy-constant edits.
- Combined rehearsal (`rehearse.sh`: setup-g apply, oracle2-g 14 + conv-g 65 + obs-g 26 files by base-checked copy, then flip) is green: row (seal-c) above. route2-g could not be rehearsed (its apply tool hard-codes the live root).

## (3) Rollback proof (`seal-a`)
`flip.py apply` -> policy sha `27e7f89fbe08…`; `flip.py revert` -> `7eeebd99dc61…` == the live file, `cmp` BYTE-IDENTICAL; `mirror.py diff seal-a` afterwards lists no library file except two files a peer edited after the mirror was cloned (`🎫️tickets/🧪️tests/🧹️purge-generated/🦀️.rs`, `🏠️workspace/…/🚧️sandboxed-run/🐹️.go`) and runtime artefacts (`.cache`, the mirror's own store). After the revert `bun repo leases status` prints no `[implementation]` line (live door). Per process, no edit: `SEMIO_REPO_IMPLEMENTATION=live`. Running sealed processes finish unaffected. The store stays on disk (about 56-80 MB, own GC, budget 512 MiB).

## (4) What the flip buys and costs (mirror `seal-a`, `bun repo …` through the launcher, interleaved live/sealed, load 30-45, `C/seal-g-tools/bench.py`, raw `C/🗑️generated/seal-g/bench{1,2}.txt`)

| command | live median wall | sealed median wall | live peak process RSS | sealed peak process RSS |
|---|---|---|---|---|
| `repo --help` (n=5) | 0.16 s (0.10-0.44) | 1.76 s (1.37-4.33) | 33 MB | 274 MB |
| `repo leases status` (n=5) | 0.24 s (0.17-0.25) | 2.69 s (1.20-5.19) | 57 MB | 297 MB |
| `repo cache status` (n=3, heavy) | 15.1 s (8.8-20.6) | 14.0 s (10.6-45.1; the 45 s run promoted) | 417 MB | 473 MB |

Reading: light commands pay about +1.5 to +2.5 s and +240 MB each (FOR chain-g); heavy commands are a wash when the snapshot is fresh (self-g measured 35.9 s live vs 7.8 s sealed on the busy real tree; on the small mirror the live capture is fast) and pay one single-flight promotion (32-61 s) per closure edit. The benefit is not speed but isolation and a stable implementation digest.
Isolation demonstration (`isolation.py`, process group frozen with SIGSTOP right after spawn / when the sealed loader child exists, help catalog in the closure edited, thawed): live run shows the edit (True, twice); sealed run shows the OLD text (False) at both freeze points (13 runs, 12 x False; the 13th is the F5 exit-2 run with no output); the edit reaches sealed runs only after a promotion. File restored byte-identically after every run.

## Landing recipe for gate-g (after ALL other batches, incl. conv-g and obs-g, and their product builds)
```sh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
python3 -I $C/self-g-tools/flip.py status           # expect: live (exit 1)
python3 -I $C/self-g-tools/flip.py apply --dry      # expect: would set defaultMode sealed
python3 -I $C/self-g-tools/flip.py apply            # one line of one file, atomic
sh $C/self-g-tools/post-flip.sh                     # 3-5 min under load: 5 law files 0 fail, startup 4/1 [known red], one promotion, tool lines, "POSTFLIP exit 0"
python3 -I $C/self-g-tools/flip.py status           # expect: sealed (exit 0) -- run it again after the LAST batch of the night
# rollback: python3 -I $C/self-g-tools/flip.py revert   (or SEMIO_REPO_IMPLEMENTATION=live per process)
```
Rules: flip is the LAST landing (conv-g's full-file copy of the policy would undo it; gate-g re-runs `flip.py status` after any later batch). Announce in coordination before and after. Lanes that edit the build tool afterwards use `SEMIO_REPO_IMPLEMENTATION=live` (see the sealed README). The first heavy command after any closure edit waits for one 30-60 s promotion.

## Tools (all in `C/seal-g-tools/`): `flip.py`, `post-flip.sh` (both also copied to `self-g-tools/`), `rehearse.sh` + `rehearse.py` (mirror batch: setup-g, oracle2-g, conv-g, obs-g, flip), `isolation.py`, `stale.py`, `concurrent.py`, `bench.py`, `profile-light.py`. Mirror commands used: `mirror.py create seal-a --only library process launcher "✏️s/🧑‍💻dev/🧩️catalog/📣️publication" --batch "python3 -I …/flip.py"`, `run seal-a [--no-seal] -- …`, `diff`, `drop`. Every guard check clean (live tree unchanged). `leases status` showed 1 waiting when I started; this lane started no official or graph-loading run.
