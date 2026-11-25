# Repository CLI closed result pipe

`repo capacity status | head -3` could exit 1 and print a stack trace from `writeResultLineV1` after the reader closed. The front door and direct build script can dispatch capacity diagnostics before `runScriptMain`, which is where the existing runtime installs its standard-stream guard for owned scripts.

The two CLI process entry points now install that guard before dispatch. Both import its implementation directly. An intermediate import through the public runtime entry failed because that entry does not export the guard. The runtime retains its existing policy for EPIPE and ERR_STREAM_DESTROYED and raises other stream errors. Child signal handling, exit statuses and reservation release still follow the existing command lifecycle.

The regression fixture uses Python and an OS pipe with its read end closed. Python first checks that the pipe produces EPIPE, then gives its write descriptor to each real CLI entry point. Both cases failed with exit 1 and a Bun stack trace before the fix and passed with exit 0 and empty stderr afterward. The complete output-contract suite passed all six tests. The existing runtime guard test passed, including its ENOSPC assertion.

On macOS, three live `repo capacity status` pipelines using `head` with limits of 2, 3 and 2 rows reported zero statuses for every stage. Tiny real subprocess checks through `repo tools exec --light bun` completed successfully and propagated an intentional child exit of 7. Verification used no browser, compiler or workspace graph workload.
