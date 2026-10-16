# Native Process Tree Custody Primary Evidence

2026-10-15. Root design research only, no provider implementation or runtime qualification. The CPU marker regression is independently actual and remains open; see cpu-capacity-execution-current.md. A broker that holds a real paid lease must retain it through actual owned work death. A process snapshot or direct-child close alone does not establish arbitrary descendant death.

## Linux Candidate

The current Linux man-pages description of [PR_SET_CHILD_SUBREAPER](https://man7.org/linux/man-pages/man2/PR_SET_CHILD_SUBREAPER.2const.html) states that orphan descendants are reparented to the nearest living ancestor subreaper, which can receive SIGCHLD and wait for their termination. The attribute survives exec and is not inherited on fork. A dedicated custodian with kernel wait/reap ownership is a candidate; correct cancellation, nested subreapers, accepted-root lifetime, reaping and final ECHILD conditions still need authored proofs. No Linux runtime was run here.

## Windows Candidate

[Microsoft Job Objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects) documents default child association, nested jobs, breakaway controls, termination and accounting. It explicitly warns that ordinary completion-port notifications are not guaranteed. Absence of a notification cannot establish an empty job. A suspended-create, actual job assignment, resume and native accounting/handle-based death protocol is a candidate requiring actual Windows tests; no Windows runtime result is claimed.

## macOS Candidate Rejected

The actual installed SDK header [/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/sys/event.h:357](/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/sys/event.h:357) explicitly marks NOTE_TRACK, NOTE_TRACKERR and NOTE_CHILD unsupported since macOS10.5. Root's preliminary suggestion to use fork tracking through these flags is withdrawn. Existence of numeric compatibility constants is not provider support. No such provider was implemented. General descendant custody on macOS remains an explicit design frontier; the CPU lane may qualify only direct-child foundation until a sound native tree ownership primitive is established.

The existing canonical Original, lease namespace and actual process capture remain unchanged. No PID/heartbeat/environment record, global registrar or structural control is promoted to authority by this research.
