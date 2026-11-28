# 🐍️ Geometry contract Python lane audit (2026-11-27)

Read-only audit (Haiku) of why `📐️example-geometry-3d-1/🐍️.py` fails with `No module named 'numpy'`.

- The repo test host picks its interpreter in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🏗️materialization/🟦️.ts` (`oracleHostPython`, lines 180-186). It provisions a cache venv from `contributedOraclePackages` (`provisionPythonInterpreter`, lines 218-265).
- The subset oracle manifest `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json` declares `example-geometry-scipy` with scipy and nested numpy `>=2.5.0`, but its `oracleHostPackages` is `[]` (line 918).
- `oracleHostPackagesFor` (`🤝️contributions/🟦️.ts:211`) reads only `oracleHostPackages`, so nothing is provisioned. `oracleLinkedPackages` reads `packages` only to classify and probe imports.
- Root `pyproject.toml` has numpy and scipy only in the `dev` group.
- `/usr/bin/python3 -I` ignores every venv, so the earlier baseline run could never succeed.
- No TypeScript lane is expected in the contract folder. The TypeScript twin is per example under `📚️examples/<example>/🧪️tests/🧩️example/🟦️.ts`.
