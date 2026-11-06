# Reconnect integration review

Source review: only A receives the relay proxy; B opens the specific artifact ID created by A from the same Space. The outage callback uses browser UI helpers and restores transport in finally; outer custody closes the relay. Actual browser journey and artifact convergence remain unobserved.

At review time reconnect policy listed only four required journey names, while the four setup checks were guarded only by a minimum total of eight; unrelated checks could substitute for a missing setup check. The existing window-focus check swallowed click errors and only inspected faults, which cannot prove its stated focus/command behavior. Engine was asked to require all eight exact unique names and make that setup check honest. Source review is not a runtime pass.
