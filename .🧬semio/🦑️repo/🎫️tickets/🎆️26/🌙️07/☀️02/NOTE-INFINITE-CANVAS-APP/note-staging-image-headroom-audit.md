# Staging Compiler Image Headroom Audit

Read-only source and CLI help review; no cleanup was invoked. Attempt 47’s generated log reported 6.0 GiB requested, 9.8 GiB free, 7.3 GiB still expected from three running commands, and 1.2 GiB kept free. The implied shortfall at that observation was about 4.7 GiB. Values change while peers write and their forecasts decay.

The admission implementation is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🏦️headroom/🟦️.ts`. `decideHeadroomV1` admits when measured free less the floor, outstanding running estimates, and earlier queued reservations covers the request. Outstanding estimates decrease over each command’s measured ramp. When no active peer or earlier waiter remains, it can admit after reclamation even if the estimate does not fit; active includes a 120-second stranded grace beyond a peer’s ramp. The policy is in the sibling `🧬️schema/🔣️.json`.

`admitImageHeadroomV1` already invokes single-flight reclamation on demand while waiting, described in source as idle lanes, incremental sessions and cold units, never content held by a running command. The admission wait is declared to the repo watchdog so its own deadline does not run while queued. The safe supported option identified is to keep the normal repo-owned admission wait alive and allow its automatic admission logic to progress; ensure any outer wrapper does not terminate the queue prematurely. This automatic reclaim is part of normal admission and is not a separate manual cleanup.

`repo cache gc --help` says Cargo-owned areas are reported rather than collected by the general collector; it therefore does not supply an independent compiler image cleanup. Its idle-image host block reclamation does not increase the fixed image’s internal free space. The capacity schema describes a fixed image envelope; host free space alone does not enlarge it. No supported non-cleanup bypass or live image expansion was found.

A `repo cache status --json` request unexpectedly started source-admission work through the wrapper; it was immediately cancelled and its leases were released. No cache status result was obtained.
