# Native Test Layout Migration

All eight authored Go tests and all active Python legacy test paths from the native audit now use `<semantic-owner>/🧪️tests/<named-case>/<canonical-implementation>`. Go uses `🐹️.go`; Python uses `🐍️.py`. No authored C or C++ tests exist. The only authored .NET test found was already canonical at `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🖥️host-protocol-parity/🔷️.cs`.

Go discovery is implemented with ephemeral standard-toolchain overlays. The runner preserves package-private test access, chooses only semantic owner packages, and avoids treating canonical case directories as standalone packages. Root, CLI, MCP, and coordinator test routers call it. A language-neutral vector and independent Go toolchain oracle validate the plan.

Python discovery now uses `python_files = ["🐍️.py"]` with Pytest importlib mode, so repeated canonical filenames collect independently. The obsolete compose-only root `conftest.py`, absent compose paths, and all legacy Pytest filename patterns were removed. Moved files calculate paths from `__file__`; the netz source catalog no longer contains Windows drive paths. The gebaeude unfolder is production source and its canonical test imports it, so production no longer imports a test module.

## Runtime evidence

- Canonical Go discovery Bun test: 1 passed, 6 expectations; both private-function Go oracle tests passed.
- Public Nx MCP Go target: passed, standard Go package result `ok`.
- Public Nx focused CLI quick target: passed in 12.1 seconds; the selected canonical case ran and both internal canonical-owner packages compiled.
- Python syntax compilation: every migrated canonical test and extracted unfolder source compiled under a ticket-local bytecode cache.
- Final native filename scan: zero `*_test.go`, `test_*.py`, or `*_test.py` sources under the repository product and active research tree; targeted `git diff --check` passed.
- Pytest canonical discovery: 70 selected tests collected from nine repeated `🐍️.py` files; the pilot case independently collected and ran 14 more tests.
- Stable Python subset: 23 passed across final-logo-audit, missing-information, current-image-render, strict-review, and gebaeude-unfolder.
- Netz semantic cases: 11 passed; one existing derived-data digest assertion failed because `beziehungsprofil_final.json` records a stale classification hash.
- Pilot image case: 12 passed and 3 subtests passed; one committed asset-safety assertion reported 11 existing radial-zone violations and the PDF check lacks the optional `fitz` module.
- Full image collection: 35 passed; four existing research artifact/data assertions failed (provisional flag, undersized candidate, FR:M07 digest mismatch, and visual rejection).

The intake graph harnesses, Figma bridge, and graph-query diagnostic were syntax checked but not executed because they install packages, access external services, write research results, or query Neo4j. Those limits are unrelated to the path migration and are recorded rather than concealed.

The exact path manifest is `📓️test-layout-native-files-2026-09-08.md`.
