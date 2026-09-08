# Development Coordinator Verification

The actual root Nx coordinator class passed an isolated real Nx workspace. It registered source watching before starting the server graph, rebuilt the transitive dependency after an emoji-path edit, served the changed artifact over HTTP, ignored generated reports, and stopped the server plus an ignoring descendant and released its port on SIGTERM. The observed parent exit was 143. This validates the coordinator on macOS; complete product preparation remains separately tracked.

## Malformed process snapshot fallback

The new language-neutral cancellation probe first reproduced an exception in the SIGTERM handler for truncated Windows process JSON. The root coordinator now ignores unusable individual process rows and catches failed snapshot commands/parsing before continuing to stop its owned launch processes. Four simulation vectors pass: malformed Windows JSON, missing Windows row, malformed POSIX row and a snapshot-command exception. These vectors test the failure fallback and signal exit status; Windows process-tree completeness remains unqualified when operating-system enumeration fails.

After this change, the real isolated Nx watcher/server regression passed on macOS with plugin isolation enabled: dependency edits rebuilt, ignored diagnostics did not rebuild, HTTP served the changed output, cancellation stopped both the server and its stubborn descendant, released the port, returned 143 and preserved the fixture's shared daemon. The fixture daemon was stopped during its own cleanup.
