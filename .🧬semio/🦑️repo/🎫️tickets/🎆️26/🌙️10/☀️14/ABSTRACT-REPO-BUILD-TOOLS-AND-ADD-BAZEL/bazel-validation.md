# Bazel Backend Validation

The pinned distribution schema and pure task-planning tests ran through the installed Nx task runner in an isolated ticket-owned test workspace. The first focused run passed three tests and 26 assertions. SQLite independently checked distribution lookup and dependency-edge selection, while AJV admitted the language-neutral schemas.

An actual Bazel 8.4.2 integration run passed three tests and 17 assertions in 42 seconds. It ran a two-node task graph, observed `[DEBUG]` producer execution, deliberately deleted the declared product, restored that product from the Bazel action cache without incrementing the producer count, and rebuilt after changing the source. The real Bazel action times were approximately 0.58 seconds, 0.17 seconds, and 0.31 seconds.

Further validation is in progress for nested orchestration, explicit argument precedence, excluded dependency projects, uncached tasks, continuous roots, and continuous service dependencies. The nested orchestration already completed and read a cached inner task without deadlocking. A stable final run is needed before reporting the expanded suite as passing.

Native Windows, native Linux, and devcontainer runtime validation remains outside the execution host used for this ticket. Nx remains the configured default.
