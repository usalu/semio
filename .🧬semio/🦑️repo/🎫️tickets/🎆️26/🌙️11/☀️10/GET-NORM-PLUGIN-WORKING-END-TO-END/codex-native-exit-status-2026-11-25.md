# Native build exit status

The discovery summary expanded `$?` after running `$(date +%H:%M)` in the same Bash command. Bash then expanded the successful date status. A shell control reproduced the false zero after a failed command. The discovery script now saves each build's status immediately, records it beside its compiler diagnostics, and returns the first failure after completing the census.

The real graph-free native wrapper acquired 2,048 MB and ran the build script with the intentionally invalid name `invalid-norm-diagnostic`. The script reported Unknown Standards component and the repository launcher returned status 1 to the shell. The component-name check runs before compiler dispatch. This confirms that the launcher and native helper propagate the exception and the discovery summary was the source of the false zero.

The mock suite ran the production summary shell with statuses `0, 17, 1, 0` and then `0, 0, 0, 0`. It verified the recorded per-component codes, the failed crate text and aggregate statuses 17 and 0. No compiler or browser was started.
