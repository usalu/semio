# g1-m06 Round 10

Result: not ready, 1 try, `native_rc=75`. One project is too heavy for this machine: `@semio-tech/cargo-workspaces`. `ROUND.md` said `round: 11` when the try ended, so the next try runs as round 11. I never saw `round: 9` (round 8 try 2 ran from 22:06Z to 23:45Z; see `round-8.md`).

- Machine: Mac mini, 16 GB memory, macOS (Darwin 25.6.0).
- Replica agent: running at the start (`runs = 45`), not kickstarted in this round. Code digest in `.cache/f/health.json` at 23:46Z: `5f634aa0499efcb1813d95aae72f33dd759a85b248d9bbfdc8d58943672db4e1` (3 s old). `bootVersion` 6.
- Start 23:46:45Z, end 23:54:19Z. No hard stop. 15 samples, all pressure level 1. Peak resident: 1446464 KB.
- Launcher after the try: link inode 3631708, target inode 3631707, mtime 22:55:40+0100, identifier `tech.semio.repo`, CDHash `a0f39e389ce8a697c89d7f9357e72d37eb06ab0d`: unchanged since "after try 1 of round 8" (third bootstrap run on this launcher). No consent text, no "runs under the app's access" line, no `fstatat failed (EPERM)` in the run log (0 matches).
- Failed tasks: none ran. Marker absent.

## Refusal (Verbatim)

```
[repo] still working after 350 s; last phase started: door.script
[repo] still working after 380 s; last phase started: door.script
[repo] still working after 410 s; last phase started: door.script
The project graph could not be read on this machine yet: @semio-tech/cargo-workspaces (about 3552 MB) need(s) more memory than this machine can give one project (1756 MB). The inference was stopped before the host ran short of memory and the project(s) are marked too heavy here. Nothing is wrong with your tree; run the command again, the fleet is asked for them first.
end 2026-11-14T23:54:19Z
native_rc=75
```

## `graph:` and `[capacity]` Lines

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] admitted run: 8 job(s), 1289 MB reserved (7813 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] dependencies: sized at 22% of the default from 15 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 1341 MB reserved (9395 MB available, 2048 MB floor)
[capacity] admitted run: 1 job(s), 2048 MB reserved (9352 MB available, 2048 MB floor)
```

Last graph step lines before the refusal:

```
Reading the project graph slice (418 s): owners 42 of 659.
Reading the project graph slice (420 s): owners 44 of 659.
Reading the project graph slice (422 s): owners 44 of 659.
Reading the project graph slice (424 s): owners 45 of 659; running (name, kind, seconds, rows): @semio-tech/dsl-derive-rs (language:cargo, 3, 0).
Reading the project graph slice (426 s): owners 47 of 659.
Reading the project graph slice (428 s): owners 47 of 659; running (name, kind, seconds, rows): @semio-tech/cargo-workspaces (language:typescript, 3, 0).
```
