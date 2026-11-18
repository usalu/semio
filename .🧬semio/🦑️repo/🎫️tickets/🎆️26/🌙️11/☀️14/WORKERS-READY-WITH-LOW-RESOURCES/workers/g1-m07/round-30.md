# g1-m07 round 30 report (tries 1-4, 19:48Z-21:30Z; written 21:45Z)

Machine: Mac mini, 16 GiB, macOS 27.2.1. Ready: **no** (`.cache/f/native-bootstrap.done` absent). `native_rc` was 0 in no try.

**Tries 3 and 4 ran after round 31 was published (20:55Z).** The driver had been started at 19:40Z for four tries and I read `ROUND.md` again only when it ended (21:30Z). Both competed with the `ready` child of this machine (its `workspace` stage: attempt 1 20:25:54Z-20:40:20Z exit 75, attempt 2 21:10:29Z-21:11:44Z exit 1). No try runs now and none will be started (round 31).

## Tries
| try | start (UTC) | end | native_rc | highest step | slice worker grew to (per `cut smaller` line) | peak resident (30 s samples) | hard stop |
|---|---|---|---|---|---|---|---|
| 1 | 19:48:26 | 19:52:57 | 7 (after SIGINT) | none printed | none printed | 8.24 GiB (8640144 KiB) | yes: process of the run above 8 GB |
| 2 | 19:59:02 | 20:30:03 | 75 | step 6 of 6 | 10030, 10448, 10631, 10552, 9954 MB | 7.95 GiB (8333744 KiB) | none |
| 3 | 20:35:15 | 21:08:25 | 7 (after SIGINT) | step 4 of 6 | 9959, 10744, 10051 MB | 8.48 GiB (8888432 KiB) | yes |
| 4 | 21:14:29 | 21:29:02 | 7 (after SIGINT) | none printed | none printed | 8.16 GiB (8556272 KiB) | yes |

`waiting for the fleet: x of N`: no such line in any of the four client logs (largest x: not measured, none printed).

## Try 2: the refusal after step 6 of 6 (asked for by round 30)
Every step line reads `Reading the project graph slice (0 s): the slice worker grew to N MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step k of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.` with N and k: 10030 (step 2), 10448 (step 3), 10631 (step 4), 10552 (step 5), 9954 (step 6). No step 1 line.
Then: "The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 9921 MB, more than the 6553 MB this machine gives a plan, and only 6984 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine."
The worker reaches about 10 GB at every step, also with one project at a time: cutting the work smaller does not lower its size on this machine. Largest resident size seen in the 30 s samples of this try: 8333744 KiB.

## Tries 1, 3 and 4: stopped by the round 23 rule
The graph child passed 8 GB resident in a 30 s sample (8640144, 8888432, 8556272 KiB); my monitor sent one SIGINT to the process group of the try, which ended within a minute with `native_rc=7`. As said in the addendum of `round-29.md`, this rule and the step-down of round 30 cannot both hold on a 16 GB Mac: the step-down only begins above 6553 MB and was seen to act near 10 GB. Try 2 was not stopped only because no sample caught it above 8388608 KiB.
Try 4, last capacity lines before the stop: `[capacity] run: work has to run, waiting 27s to grow the reservation from 512 MB to 14336 MB: not enough free memory (4664 MB spare above the 2048 MB floor; 3 running, 0 ahead); behind graph-plan (running, 1 job(s), 3352 MB), graph-plan (running, 1 job(s), 1235 MB), workspace:setup (quiet, 1 job(s), 50 MB)`: two graph plans ran at once (the try and the `ready` child).

## Launcher
`round-30.launcher.txt`: before and after every try. Receipts at 21:40Z: `client.json` ok, `service.json` ok (route direct), `launcher.json` carries no waiting state.

Files: `round-30.tries.txt`, `round-30.tail.txt`, `round-30*.monitor.txt`, `round-30*.samples.txt`, `round-30*.p12-samples.txt`, logs `~/native-bootstrap-round-30[-try-k].log`.
