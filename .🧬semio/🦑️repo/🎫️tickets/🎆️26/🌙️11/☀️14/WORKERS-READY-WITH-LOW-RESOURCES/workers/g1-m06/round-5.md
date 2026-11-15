# g1-m06 Round 5

Result: not ready. 10 tries, none reached a setup task, marker absent. Tries 1 and 2 ended with the fleet refusal after partial deliveries, tries 3 to 5 with a hard stop on memory pressure while inferring here, tries 6 to 10 within about a minute with an "unexpected failure" of the graph service: a file of the build input closures is missing (`ENOENT`).

## Machine

| Item | Value |
| --- | --- |
| Model | Mac mini, Apple M4, 10 cores |
| Memory | 16 GB |
| Free disk | 110 GiB of 228 GiB (measured before the round) |
| OS | macOS 26.6.2 (25G83) |
| Replica code digest after the round | `39e3688eb315…` (first 12 characters read; full value before the run below) |

## Before the Run (2026-11-14T17:58:37Z)

- Replica code digest (`.cache/f/health.json`): `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759`.
- `ps` lines (pid, ppid, age, resident KB, command shortened) of `repo show projects` and `repo service host` processes:

```
97767     1    01:11:12  63984 bun …/door/script.ts repo service host run
97781 97767    01:11:11 100544 bun --eval \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=await import('
```

- Last 8 lines of `.cache/f/sidecars/build-agent.log`:

```
2026-11-14T17:51:44.072Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T17:51:44.111Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T17:51:44.115Z worker registered: 7be78220-90eb-47a8-bd04-bd9420144478 cache http://192.168.178.70:6094
2026-11-14T17:53:43.702Z agent code changed, restarting
2026-11-14T17:53:43.704Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-04-lan.fritz.box-26788
2026-11-14T17:53:44.801Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T17:53:44.840Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T17:53:44.844Z worker registered: b69842c7-4642-462f-bb93-45e81a4b8bf4 cache http://192.168.178.70:6094
```

## Tries

| Try | Start (UTC) | End (UTC) | `native_rc` | Wanted | Arrived | Missing | Seconds | How it ended |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | 17:58:38 | 18:25:37 | 75 | 686 | 262 | 424 | 1501 | fleet refusal |
| 2 | 18:27:44 | 18:43:54 | 75 | 424 | 48 | 376 | 909 | fleet refusal |
| 3 | 18:46:18 | 18:54:20 | 1 | 590 | 406 | not stated in the log | not stated | then `refreshing 686 projects`, `owners 68 of 686` at 460 s, hard stop |
| 4 | 18:57:51 | 19:05:53 | 1 | no request | - | - | - | `refreshing 638 projects`, `owners 74 of 638` at 460 s, hard stop |
| 5 | 19:09:23 | 19:16:55 | 1 | no request | - | - | - | `refreshing 590 projects`, `owners 156 of 590` at 430 s, hard stop |
| 6 | 19:20:25 | 19:21:28 | 1 | no request | - | - | - | unexpected failure 48c53713 (`ENOENT`) at `assembly 161 of 703`, 42 s |
| 7 | 19:23:55 | 19:24:59 | 1 | no request | - | - | - | unexpected failure 29bbd28b, 40 s |
| 8 | 19:27:26 | 19:28:26 | 1 | no request | - | - | - | unexpected failure 48c53713 (`ENOENT`) at `assembly 97 of 703`, 40 s |
| 9 | 19:30:56 | 19:31:56 | 1 | no request | - | - | - | unexpected failure 29bbd28b, 38 s |
| 10 | 19:33:57 | 19:34:58 | 1 | no request | - | - | - | unexpected failure 48c53713 (`ENOENT`), 38 s |

- Pauses: 2 minutes after tries 1, 2 and 6 to 9, 3 minutes after the hard stops of tries 3 to 5. My wrapper treated the "unexpected failure" text of tries 6 to 9 as a reason to try again; the round names only `native_rc=75` and a fleet refusal text, so those four extra tries go beyond the letter of the round.
- Failed nx tasks: none; `workspace:setup` never got past the graph step.
- Stale project count over the tries: 686, 686, 686, 638, 590. Tries 4 and 5 each started with 48 fewer than the try before.

## Graph Service Log, Verbatim per Try

Try 1 (`host-6feebb05efe3.log`):

```
graph: 686 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 686
graph: waiting for the fleet: 0 of 686 projects arrived
graph: waiting for the fleet: 121 of 686 projects arrived
graph: waiting for the fleet: 168 of 686 projects arrived
graph: waiting for the fleet: 214 of 686 projects arrived
graph: waiting for the fleet: 262 of 686 projects arrived
graph: nothing to wait for any longer: asked the fleet for 686 stale projects and received 262 within 1501 s, 424 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T18:25:37.436Z graph-wanted-expired {"wanted":"686","arrived":"262","seconds":"1501","missing":"424","local":"192"}
```

Try 2 (`host-a561d8014168.log`, a new service; it serves all later tries):

```
graph: 686 stale projects, 262 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 424
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 48 of 424 projects arrived
graph: nothing to wait for any longer: asked the fleet for 424 stale projects and received 48 within 909 s, 376 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T18:43:54.139Z graph-wanted-expired {"wanted":"424","arrived":"48","seconds":"909","missing":"376","local":"192"}
```

Try 3:

```
graph: 686 stale projects, 96 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 590
graph: waiting for the fleet: 0 of 590 projects arrived
graph: waiting for the fleet: 310 of 590 projects arrived
graph: waiting for the fleet: 358 of 590 projects arrived
graph: waiting for the fleet: 406 of 590 projects arrived
graph: refreshing 686 projects (generation 19)
graph: 372 stale facts (file 215, analysis 110, field 38, readdir 7, digest 2) leave 686 projects stale
graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile
[fleet] projects from the fleet: 0 taken, 0 stored, 0 offered, 0 waited for another machine, 0 computed here | Projekte aus der Flotte: 0 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 0 hier berechnet
```

Try 4:

```
graph: refreshing 638 projects (generation 20)
graph: 365 stale facts (file 215, analysis 110, field 38, digest 2) leave 638 projects stale
graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile
[fleet] projects from the fleet: 0 taken, 0 stored, 0 offered, 0 waited for another machine, 1 computed here | Projekte aus der Flotte: 0 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 1 hier berechnet
```

Try 5:

```
graph: refreshing 590 projects (generation 21)
graph: 364 stale facts (file 215, analysis 109, field 38, digest 2) leave 590 projects stale
graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile
[fleet] projects from the fleet: 0 taken, 0 stored, 0 offered, 0 waited for another machine, 0 computed here | Projekte aus der Flotte: 0 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 0 hier berechnet
[fleet] projects from the fleet: 15 taken, 0 stored, 0 offered, 0 waited for another machine, 0 computed here | Projekte aus der Flotte: 15 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 0 hier berechnet
[fleet] projects from the fleet: 48 taken, 0 stored, 0 offered, 0 waited for another machine, 0 computed here | Projekte aus der Flotte: 48 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 0 hier berechnet
```

Tries 6, 8 and 10 (none of the asked-for lines; the failure):

```
2026-11-14T19:21:28.833Z request-failed 48c53713 Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/b459f26e71ea15d46bbc9ce5cdd6b0e8e0021424c9f87333d84d7b0b6b43b339.json'
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:86:20)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
```

Tries 7 and 9 (same stack, error without a message):

```
graph: 1 derived results asked again after 1 stale facts: 0 unchanged, 0 changed
2026-11-14T19:24:59.055Z request-failed 29bbd28b Error
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
```

- The named file does not exist after the round; its directory `.cache/b/g/m/build-input-closures` holds about 16,300 entries.
- No line with `inferred here` appeared in any try.
- The last 30 service log lines of every try are in `round-5.service-tails.txt`.

What the lines show, as observed:

- Try 2 took the 262 projects of try 1 from the fleet and asked only for the remaining 424. Try 3 counted only 96 as taken and asked for 590 again, although 262 + 48 = 310 had arrived before; its wait then began at 310 of 590.
- After 406 of 590 had arrived, try 3 switched to `refreshing 686 projects`, the full stale set, not the remaining 184.
- Tries 4 and 5 did not ask the fleet at all and refreshed 638 and 590 projects on this machine.

## Hard Stops and Memory

| Try | Level-2 samples | Largest process (resident KB, pid, parent) |
| --- | --- | --- |
| 1 | 0 of 54 | 3,532,560, pid 30038, parent 29958 |
| 2 | 0 of 33 | 2,872,160, pid 43717, parent 43630 |
| 3 | 5 of 17, the last five | 9,711,264, pid 52246, parent 43630 |
| 4 | 5 of 17, the last five | 5,443,056, pid 57825, parent 43630 |
| 5 | 5 of 16, the last five | 9,141,600, pid 63698, parent 43630 |
| 6 | 0 of 3 | 2,024,192, pid 69212, parent 43630 |

- Pid 43630 is the child of the graph service of this host (`repo service host run`, pid 43627, parent launchd, started with try 2).
- In tries 3 and 5 the inferring process went above 8 GB resident (9.7 GB and 9.1 GB). My runner did not stop on that: it checks descendants of the bootstrap only, and this process hangs under the graph service. The stop came from the pressure rule 30 s to 2 minutes later. So the "process of the run above 8 GB" stop was missed by my runner in tries 3 and 5; the memory bound announced for this round was not visible in these two tries.
- Each hard stop was one SIGINT to the `repo run workspace:setup` process after five samples of level 2 in a row; the level was 1 again at the next try.
- No `git` process above 1 GB appeared in any sample.
- launchd left the replica agent down 0 times during the round.

## `[capacity]` Lines (each try; the available figure varies)

```
[capacity] admitted run: 1 job(s), 2048 MB reserved (9814 MB available, 2048 MB floor)
[capacity] admitted run: 1 job(s), 2048 MB reserved (9861 MB available, 2048 MB floor)
[capacity] admitted run: 1 job(s), 2048 MB reserved (9359 MB available, 2048 MB floor)
[capacity] admitted run: 1 job(s), 2048 MB reserved (9185 MB available, 2048 MB floor)
```

These are the `workspace:setup` admissions of tries 7 to 10. The lines of the earlier tries have the same form; I did not extract them one by one.

## Wall Time per Phase

| Try | Graph step | Total |
| --- | --- | --- |
| 1 | 1588 s, all in `flush 1 of 1` | 1619 s |
| 2 | 939 s | 970 s |
| 3 | 460 s | 482 s |
| 4 | 460 s | 482 s |
| 5 | 430 s | 452 s |
| 6 to 10 | 38 to 42 s | 60 to 64 s |

The phases before `workspace:setup` took about 20 to 30 s per try (total minus graph step; not measured separately).

## State After the Round

- `repo service host run` pid 43627 (70 MB) and its child 43630 (326 MB) are still running; I left them.
- Files in this folder: `round-5.md`, `round-5.tail.txt` (last 200 lines of try 10), `round-5.service-tails.txt`, `state.json`.
