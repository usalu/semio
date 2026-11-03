# Capacity Bottleneck (2026-11-02)

`repo capacity status` shows **1 running grant and 17 waiters**, every one held with the verdict `pressure-warn`. The pressure reading is wrong:

- Windows memory counters could not be read ("The operation was aborted").
- The gate treats unknown counters as raised pressure.
- Under raised pressure it admits only one job at a time.
- The host actually has 24.6 of 65.4 GB available and 16 cores.

**Rule for every worker (until the gate is fixed):** run at most **one** queued `repo tools exec bun` process per worker per round. That process runs one ticket-local driver `T/🧰️<id>/all.ts` that does check, consistency and previews for all assigned assets in a single process.

- Never queue one `repo` call per asset or per scale.
- Read the driver's output file when it finishes.
- Fix everything it finds before queueing the next round.

**Follow-up (outside this ticket):** the repo capacity probe on win32 must not treat unreadable counters as pressure-warn when the OS reports large free memory.
