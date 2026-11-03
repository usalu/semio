# Nx "Owned Nx invocation failed (1)" with no diagnostics (2026-11-02 09:15)

## Verdict
Not a code defect in the norm plugin or our crates. The host is out of memory (15.4 of 15.8 GiB RAM used, swap 8.9 of 8.9 GiB full, load 40) because about 10 fleet builds each run bun/Nx graph processes of 0.5 to 2.6 GiB at once. The kernel OOM killer / earlyoom kills the `bun` process of the build, and the owned wrapper then reports only exit code 1.

Evidence (`journalctl --no-pager | grep -i 'out of memory'`, 42 kills in total, all `bun`, anon-rss 1.0 to 2.7 GiB):
- 08:38:11 kill of pid 2785334 (1.6 GiB) vs `build-en1998/7.txt` ending 08:38:31 (graph received, `Nx Daemon was not able to compute the project graph`, `daemon-error.log` = "Daemon process terminated and closed the connection", then silent exit 1).
- 08:34:19 kill of the daemon (2.6 GiB), 08:19:19 and 08:09:57 same (daemons of other graph namespaces).
- 08:58:23 kill of pid 2800479 (2.2 GiB) vs `build-en1992/10.txt` ending 08:58:35: last line is `nx:received-graph ... native:existing-journal:replay`, no task started, silent exit 1.
- 09:04:23 and 09:09:19 more kills after that.

## Two different symptoms
1. en1992, en1998 (and likely en1999, ...): the failure happens before any Nx task runs, during graph construction / hashing. The Nx daemon or the Nx client bun is OOM-killed. Nothing is printed because the killed process cannot print; `NX_VERBOSE_LOGGING`/`--verbose` would not help. Signature in the log: `NX   Nx Daemon was not able to compute the project graph` or no `Running target build` line at all.
2. en1993 `build-en1993/8.txt`: the task did run (`nx run @semio-tech/norm-en1993-rs:build:prototype`, after 33 min) and failed with a visible reason at about line 4640: `error: Insufficient physical compiler initial headroom: 1.6 GiB free of the 4.0 GiB a compile needs after 1803 s` (`admitCompilerHeadroomV1`, compiler capacity owner). The terminal only said `Output of 3 successful tasks were not shown` because the failing task's output is in the middle of the log, between the `> nx run ...:build:prototype` line and `NX Running target ... failed`. It is the same memory starvation, refused after the 30 min wait ceiling (`pace.maximumMilliseconds`). The compile never started, so there is no rustc diagnostic.

No rustc, schema-index, twin, lock or source-admission error is hidden anywhere; the three generate tasks (`ui-styling-tokens:generate`, `dsl-derive-rs:generate-schema-index`, `framework-schema:generate`) succeeded (checkmarks in `build-en1993/8.txt`).

## Where to see task errors
- Whole build log: search for `Failed tasks:` and the `> nx run <project>:build:prototype` marker; the task output (rustc, headroom refusal) is between that marker and `NX   Running target ... failed`. Strip colors: `sed 's/\x1b\[[0-9;]*m//g' LOG | grep -v '^\[script\]\|still running\|still working'`.
- Flag: `repo run <project>:build --output=static` (or `--verbose`) prints successful task outputs too. Without it Nx says "Output of N successful tasks were not shown". Task failures are always printed.
- Daemon: `.cache/b/o/m/nx/graphs/<hash>/<graph-id>/daemon-error.log` and `daemon.log` (path is printed in the "Nx Daemon was not able to compute the project graph" block). "Daemon process terminated and closed the connection" means the daemon was killed (OOM).
- Silent exit 1 with last log line `nx:received-graph` or `Verifying graph facts`: check `journalctl --no-pager | grep -i 'out of memory' | tail` for a `bun` kill at the log's mtime, and `free -m` / `uptime`.
- Source code: child output is relayed in `runWithPreparedToolingStore` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts` (about lines 489 to 515); `SEMIO_NX_OUTPUT_LOG=<file>` spools the Nx output to a file and prints a summary.

## What to do
- Serialize builds: at most 2 `-rs:build` runs at once on this 16 GiB host (each peaks at 2 to 3 GiB for the graph and 4 GiB for the compile image). Builders should check `free -m` first and start only when available memory is above 4 GiB; an exit 1 with no diagnostic is a signal to wait and re-run, not to edit code.
- Re-run after memory is free. Do not leave the 5400000 ms capacity wait running in parallel for all 10 artifacts; the framework's own admission (`[capacity] waiting`) measures free memory only when the run starts, not for the later graph/daemon phase, which is why admitted runs still get killed.
- Framework follow-up (integrator, not done here): the wrapper should surface "child killed by signal / exit 1 without output" with a hint to check OOM, and capacity admission should reserve the graph-phase memory.
