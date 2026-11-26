# Result F51: `repo fleet hostname apply` registered and proven

## Cause and fix
- The front door refuses subcommands not in the operations catalogue (`closedSubcommands` contains `fleet`). F47 wired `runFleetCliV1` to the topology op but never added `hostname` to `⚙️operations/🔣️catalog.json`. The catalogue is hand-written (no generator): I added the subcommand `fleet hostname apply` (operation synopsis `fleet status|hostname`), en/de summary and an example.
- Help snapshots (`⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json`) regenerated with `T/🧰️scripts/F51/regen-help-snapshot.ts` (only root, fleet, fleet hostname rows, both languages).
- MCP surface: `🔌️mcp/🖼️assets/🏗️build-surface.json` excludes `fleet hostname` (sudo, human decision).
- Plan visible before the question: the topology op now flushes its plan lines through `progress` before calling `confirm` (before, they were only returned after the question). The fleet command passes `progress: write`.
- TTY: confirmation reads stdin via readline only on a TTY; without one it prints the refusal (en+de, text `hostname.tty`) and exits 2, no stack trace. `--yes` does not exist (`repo fleet hostname apply --yes` is a syntax error). Ports `hostname {interactive, answer, run, platform}` and `refuse` allow injected tests.
- Fleet-status tests: 30/0 including a new test (decline, de, no TTY exit 2, --yes refused, runner injected, only after yes runs `sudo scutil --set LocalHostName semio-g1-zz-test`).

## Exact dry run on this Mac (answer n; no sudo ran; LocalHostName still semio-g1-m08)
```
This machine is g1-server. Its host name will become semio-g1-server with: sudo scutil --set LocalHostName semio-g1-server
Rename this machine to semio-g1-server? [y/N] n
Nothing was changed.
```
Without a TTY (</dev/null): the plan line, then the en/de refusal, exit 2.

## repo fleet --help
```
repo fleet - Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.

Usage:
  repo [--tool=nx|bazel|semio] fleet status|hostname [options]

Subcommands:
  fleet status [--json] [--limit=<n>]  Print the fleet picture as compact text, or with --json as one document whose schema is validated before it is printed.
  fleet hostname apply                 Give this Mac or Linux machine its semantic host name semio-<origin>, derived from the origin id in .cache/f/machine.json: prints the plan and the exact sudo command, asks on the terminal and runs it only after a yes (no --yes; refused without a terminal; nothing to do when the name is already right).

Options:
  --json       Print lists as JSON instead of one entry per line.
  --limit=<n>  fleet status: how many runs the text output lists (default: 20, the JSON output always lists all).

Global options:
  --tool=<nx|bazel|semio>  Select the build implementation; otherwise SEMIO_BUILD_TOOL, then the policy default (nx).
  --workers=<on|off>       Run actions in persistent workers (on) or in one process each (off); otherwise SEMIO_BUILD_WORKERS, then the worker policy of the build policy.
  -h, --help               Print the usage of the command and stop.
  --lang=<en|de>           Language of the texts; otherwise SEMIO_LANGUAGE, then LANGUAGE and the locale variables (LC_ALL, LC_MESSAGES, LANG), then the operating system language. No language is assumed: the command refuses when none resolves.

Examples:
  repo fleet status
      See at a glance who runs what, who waits for what and for how long, and what the disk and memory budgets look like.
  repo fleet hostname apply
      Rename this machine to semio-<origin> after you confirm the printed sudo command; every machine runs the same command, no name is typed.
  repo fleet status --json
      The same picture as one machine-readable document for an agent.

Notes:
  - The command answers from the existing status readers of the leases, the capacity gate, the Bazel slots, the cache ledger, the sealed implementation store and the graph generation pointer in parallel; it takes no lease, loads no graph, starts no build and answers within a second on a loaded machine. A reader that fails is named under the sources and leaves only its own section out.
  - Waiting-time estimates add the time left of the holder (its own progress counter, else earlier holds of the same kind) to the typical hold of everyone ahead; a memory waiter is admitted once the memory of the running grants, released after the typical duration of their operation, covers its need. No estimate is printed where nothing is known.
```

## Tests
fleet-status 30/0, operations 15/0, door 9/0, build-surface 6/0, addresses 20/0, topology 74/0, caller door 3/0. Landing recorded in T/landings.md.

## Not verified
The y path with real sudo (deliberately not run). Windows prints the name only (as F47).

## User
`cd ~/Documents/semio && repo fleet hostname apply` in an interactive terminal on each Mac.
