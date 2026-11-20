# Describe Tool Restart Diagnostics

The active warm generation is `.cache/b/r/m/components/runs/wMT7y8n`. The outer `🗑️generated/native/site-build.log` reports three `job:describe-tool` exit-75 restarts. The driver collects both child streams in `l/job-describe-tool.log`, but `boundedLog` opens that same pathname with `"w"` on each attempt. Each new attempt therefore truncates the preceding attempt's diagnostics. The outer restart notice does not include the captured tail. The first three exact causes and whether they repeated the same source or stamp cannot be established from these surviving logs.

At inspection the fourth attempt's current log had completed source admission and was emitting compiler artifacts, with no churn or format-stamp failure diagnostic observed. The log is live and will itself be overwritten if another retry begins. Exit 75 alone is not evidence of a specific edit or self-churn. No processes were interrupted and no build was started.

## Retry Log Repair

A generation now retains the first attempt at `job-<id>.log` and later attempts at `job-<id>.log.2`, `job-<id>.log.3` and so on. Each nonzero exit also writes a `.tail` companion containing the bounded late output from both streams. The restart notice lists the attempt and both paths and prints the most recent 40 lines. Terminal errors identify the same files. Case-only job IDs receive distinct log names on every platform.

Per attempted job, the shared restart protocol permits at most seven log files. Each has the existing 128 MiB byte ceiling. Each failure tail is capped at 120 lines of 400 UTF-16 code units. The driver validates the longest possible path before starting a generation and rechecks storage before each child begins. The process watcher still observes every line and both streams settle before the attempt log closes. Cancellation during backoff rejects with the original reason.

Three isolated regression cases passed. They cover distinct logs and notices across two retries, byte bounds and late diagnostics when cancellation prevents a retry, and all attempt and sidecar paths on a case-insensitive filesystem. Two existing driver structure tests also passed. The attempted older integration fixture reached a real Cargo configuration probe before the stub command, and compiler ingress refused the probe before compilation. No further build or probe was started. The running release process was left in place.

## Live release verification on November 20

Run `wCsXqmg` retained `job-describe-tool.log` and `job-describe-tool.log.tail` after exit 75 and started its second attempt. The first tail identifies the changed input as `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`. Its filesystem modification time was 01:02:21 CET, after the 00:51:52 site build start.

The second attempt recorded a change in the store document history hydration fold and exited 75. The third attempt recorded `stage: complete` at 01:52 CET. The driver then advanced to the Flow WebAssembly prerequisite.
