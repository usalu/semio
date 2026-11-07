# Python Oracle Query Choice

The three declared Python suites use standard-library path, argument and SQLite oracles, so synced workspace dev/test dependencies are not their prerequisite. The existing producer installs the configured workspace main interpreter under the plan's managed interpreter root. The shared suite helper currently runs uv project execution, which may create an unsynchronized environment and cannot prove preparation. No existing readiness receipt was found in the Python plan.

Use the existing registered uv and workspace plan to discover that installed main interpreter with `uv python find <series> --system --managed-python --no-project --no-python-downloads`; require the reported absolute executable to be inside the planned managed interpreter root before supplying the registered Python locator. The system option skips active virtual environments while managed-only excludes system Python. The query cannot create a project environment or download Python, and fails when a matching managed installation is absent. The strict runner probes the bound executable. No workspace sync or fabricated receipt is implied. The official [uv command reference](https://docs.astral.sh/uv/reference/cli/) describes python find and managed-only/no-download controls.

Acceptance: schema-first vectors cover command arguments and containment on Windows and POSIX; independent Ajv and SQLite checks classify the neutral fixtures; authentic helper wiring must use the plan series and interpreter directory, and not invoke project run. Actual managed query and canonical suites remain behind the shared setup barrier.

The shared helper has two production callers: the caching and build suite wrappers. Their manifests declare Python for storage-compiler, storage-compiler-processes, build-host-capacity, and build-preflight. The preflight test tree invokes three Python oracle scripts; source import inspection confirms only Python standard-library imports, with no workspace packages. No additional production helper callers were found in the library tree.

Executed focused TDD: original helper failed the new contract with missing discovery export (inner/outer 1); the repaired helper passed 2 tests and 14 assertions (inner/outer 0). Ajv validates the neutral schema, SQLite independently classifies the four platform path fixtures, and TypeScript AST checks the actual plan series and managed root wiring. The earlier 6-test binding regression is separate evidence. No operational managed query or canonical suite ran in this check.

Registered uv is not discoverable by a normal clean-shell tools exec on this host, despite the successful native setup. The producer supplies a local pinned PATH only during its own call. The existing read-only `preparedNativeToolV1` verifies native tool receipt identity and binary digests; bind its verified uv binary through the registered uv environment locator before the discovery query. This does not provision or download tools.

The registered-manager contract failed before the receipt-backed selector and then passed. The pinned uv help confirmed all flags. The first actual discovery failed with exit 2, with project environment absent before and after. After a red/green diagnostic-and-cancellation test, captured bounded sanitized stderr identified a conflict between the explicit managed flag and the plan's UV_PYTHON_PREFERENCE. The final per-call environment suppresses that preference without mutating process.env; an executed regression uses the actual toolEnvironmentV1 to prove planned and inherited preferences are absent in the child.

The final genuine helper query passed inner/outer 0 and selected managed CPython 3.14 under the planned tools root. The workspace environment was absent before and after. Full focused test file passed 10 tests and 62 assertions inner/outer 0; no canonical test graph has yet run. Actual check: repo capacity run --memory=768 --cpu=1 --lang=en -- repo tools exec bun -- ticket check-python-discovery.ts. Focused test: same capacity wrapper and repo tools exec bun -- test on the source fixture file. Failure and passing output were captured under generated.

## Source Roster and Handoff

Owned query repair:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🐍️python/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🐍️python/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🧪️tests/📌️python/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🧪️tests/📌️python/🧭️query.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🧪️tests/📌️python/🧭️schema.json`
- Ticket inputs `check-python-discovery.ts`, `check-uv-find-help.ts` and this report.

The earlier binding repair roster remains in nov06-native-gate-plan.md. Eight applicable canonical infrastructure gates remain pending genuine JavaScript Ready and coordinator release; no gate passed or failed in this handoff. Full workspace Python dev/test environment readiness is not claimed. There are no live command handles owned by this agent.

## Windows Environment Case Follow-up

Independent review identified that Windows treats environment names case-insensitively while the prior uppercase-only suppression left mixed-case aliases alive. The actual toolEnvironmentV1 preserves input spelling. Schema-first Windows/Linux fixtures with Ajv and SQLite reproduced the issue: Windows mixed-case preference remained (inner/outer 1). The scoped projection now removes ambient case-insensitive aliases of every planned variable and the registered uv locator on Windows before applying the authoritative plan; Linux/macOS retain case-sensitive behavior. This also prevents an inherited mixed-case UV_PYTHON_INSTALL_DIR from overriding the planned managed root. Parent input remains untouched.

Full focused file passed 11 tests / 73 assertions inner/outer 0, including actual toolEnvironmentV1 projection and independent SQLite classification. The isolated genuine Windows query then passed inner/outer 0 with a synthetic mixed-case inherited preference and wrong mixed-case install directory supplied only to its probe child; it selected the planned managed Python 3.14, and workspace environment existence was false before and after. No fake Python binding, setup or canonical graph was used.

Additional source fixtures: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🧪️tests/📌️python/🌿️cases.json` and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🧪️tests/📌️python/🌿️schema.json`. Ticket input check-python-discovery.ts was updated to exercise the mixed-case scenario.
