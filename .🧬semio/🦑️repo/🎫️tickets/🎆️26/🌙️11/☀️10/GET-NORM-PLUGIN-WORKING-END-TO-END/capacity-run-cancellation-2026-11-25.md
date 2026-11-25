# Capacity Run Cancellation and Grant Ownership

`repo capacity run` forwarded interrupt and termination signals to its immediate child and released the memory grant when that child exited. A grandchild in its own POSIX process group could survive and keep using memory after that release. The SIGINT regression reproduced the missing grandchild signal with the previous code.

The command now forwards the original signal through the shared owned-process-tree termination helper. It keeps the grant until the helper observes the tree exit or sends its bounded escalation. It preserves direct child exit codes and the interrupt status when SIGINT escalates. The termination helper still uses the existing Windows tree termination path.

The POSIX integration test uses a bounded, idle grandchild in a detached group. It independently verifies whether the child saw the requested signal, whether the grant still exists while the grandchild waits, and the wrapper exit status after release. Both SIGINT and SIGTERM cases use the same language-neutral JSON vector.

Both cases passed after the change on macOS, alongside the existing arbitrary command, grant propagation, and ordinary exit status test. The runtime test did not execute on Windows. No compiler or production build was started.
