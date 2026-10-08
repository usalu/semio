# Prepared Adjacent Cargo Custody

The owned normal WGPU dev run 58744 was intentionally cancelled through its owned Nx process. Its authoritative terminal was Nx 130 after 15m19s; workspace:deps-cargo occupied 15m18s. This is not an inferred timeout. Compiler/Trunk/staged/mounted proof was not produced. Existing prepared outputs and foreign jobs, queues, caches and locks were preserved.

The current normal dependency route prepares every workspace twice: generic runTool prepares before update --workspace, then prepares again before fetch --locked. The shared per-native and orchestration budgets are zero unless an explicit supported environment budget is set; the cancelled run had no such override.

The replacement is one adjacent dependency transaction under the existing exclusive preparation lease. It prepares once, retains precise source/input/presence/output and current member observations, executes the same update --workspace argv, rechecks custody, then executes the same fetch --locked argv. It has no caller skip flag or persistent historical receipt reuse. Unknown executable preparation observations refuse adjacency. Three existing composition producers publish exact observations from their existing physical resource operations: Remodeling capability links, Semio conversion definition, and Hub Stdio composition. Cargo.lock is the declared update output and is checked separately at the locked fetch boundary.

Neutral cases are plain examples. Actual Cargo and an independent TOML/parser and source inventory oracle must verify the authored contract and command behavior before production resumes.
