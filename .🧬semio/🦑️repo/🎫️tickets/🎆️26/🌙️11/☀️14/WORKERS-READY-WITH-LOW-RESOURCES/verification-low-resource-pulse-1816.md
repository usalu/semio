# Low-Resource Build Verification at 18:16 UTC

## Changes verified

The native dependency stage on g1-m10 exited at 18:00:04 UTC after an exact relative directory observation changed. No Source binding transition was unresolved. The dependency producer now retries the exact descriptor refusal under its existing three-attempt policy, taking a fresh observation and keeping the cancellation and custody checks. A private native directory race reproduced the refusal, then succeeded on a new observation without entering the stale descriptor body. Four focused tests passed. The broader light-wrapper suite had 17 passes and two failures, a Bun hardlink control and a scan test requiring an Original Script envelope. [Dependency retry evidence](audit-dependency-descriptor-retry-1800.md)

Graph preparation now performs the mandatory tool preflight before planning and leaves closure priming to the actual owner wave and its cancellation signal. This removes one extra detached launch per preparation. It also gives up speculative overlap with planning. The physical logs do not separately time the two earlier launches. The focused host and child, loader and carried-closure, and adaptive-wave and repair suites passed 144 tests with 660 assertions. [Graph preparation evidence](audit-graph-owner-wave-prime.md)

The coordinator again had nine fresh worker attestations for the exact stage `7d8c0b059bcaed05be881279fe59b4fa8305d514` at 18:16:42 UTC, after g1-m02 resumed reporting. All eleven files changed in the dependency and graph work had exact PUT records. Seven fresh worker cursors covered all eleven records at first. At 18:17:57 UTC all nine fresh worker cursors covered all eleven. These are journal observations. The running workers’ graph and dependency processes can still be executing earlier code.

## Physical acceptance

| Requirement | Evidence at this check |
| --- | --- |
| Current native Ready on all enrolled 16 GiB workers | Zero of twelve. Nine workers reported the exact stage, two remained stale, and one had no attestation. The two fresh dependency stages and seven workspace stages still reported running. |
| Isolated all-stage run at 8,192 MiB and four cores | Standalone and owner-on nested APFS controls passed real native attach and non-forced detach. The private devices and files cleared. The full 41 GiB launch remained below its 60,129,542,144-byte gate. The fresh preflight measured 36,041,158,656 available bytes, and the later check measured 29,177,397,248. |
| Complete authenticated graph and physical adoption by a compatible worker | The coordinator still had no current whole-workspace seal. Its live whole-read process, PID 24613, remained queued for 14,336 MiB under normal capacity admission. The signed verifier returned `receipt-missing` for all twelve worker destinations. |

[Native storage controls](audit-cold-lab-physical-preflight-1812.md) · [Previous stage propagation verification](verification-current-stage-and-readiness-2026-11-21.md)
