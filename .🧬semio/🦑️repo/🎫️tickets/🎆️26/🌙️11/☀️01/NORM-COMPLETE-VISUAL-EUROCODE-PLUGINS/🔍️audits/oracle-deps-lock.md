
# 🔒️ Oracle Dependencies Lock Outcome

Follow-up to `oracle-harness.md` section 6 (lock and sync of the oracle libraries).

## Result

- The six oracle pins are now locked in `uv.lock`: `structuralcodes 0.7.3`, `sectionproperties 3.10.2`, `sympy 1.14.0` (plus `concreteproperties`, `blue-prints`, `groundhog`), resolved by `repo tools exec uv -- lock` (exit 0, "Resolved 113 packages").
- The Python environment is installed: `repo tools exec uv -- sync --locked --group test` (exit 0, 44 packages installed).
- Import check `/home/deck/semio/.venv/bin/python -I -c "import structuralcodes, concreteproperties, sectionproperties, sympy; print('ok')"` prints `ok`. `.venv` is a symlink to `/usr/bin/python3`, and uv installed into it.
- `repo dependencies lock --all` and `sync --all` both exit 0 but cover only the Bun/JS owner (`repository:build`). They do not touch `uv.lock`.

## Not done: ratchet baseline

- `🔒️dependencies.json` (379 entries) does not contain the six oracle packages. Not written.
- The baseline write (`verify dependencies write-baseline`) is only reachable through the full native `repo verify` command, which is not installed here. The launcher says to install it with `repo run repo-mcp:install`. That install configures MCP client files, so I did not run it.
- Open item: install the native `repo` verify command, then write the baseline and confirm the ratchet passes.

## Method notes

- uv was run through `repo tools exec`, not directly. The brief says never to invoke uv directly, and this is the documented repo route (`README.md` "Package manager" and "tools exec").
- The `dependencies` command has no Python lock step: the uv manager (`🔌️managers/🐍️uv`) reads `uv.lock` but does not regenerate it.

## Platform note (unchanged)

- `sectionproperties`, `concreteproperties` and `blue-prints` need `cytriangle`, which has no linux-arm64 wheel.

## Cleanup

- `🗑️generated/deps/` (lock, sync and uv logs) removed.
- Changed: `uv.lock` (lock by repo tool), `.venv` (uv sync). No git writes.
