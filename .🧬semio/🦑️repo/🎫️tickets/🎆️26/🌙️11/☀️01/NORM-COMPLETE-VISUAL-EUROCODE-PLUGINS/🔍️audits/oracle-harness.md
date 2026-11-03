# Oracle Harness

Status of the shared independent-oracle infrastructure (strategy: `📐️specs/oracle-strategy.md` sections 4-6).

## 1. Dependencies

Pinned in the root `pyproject.toml` test group: `structuralcodes==0.7.3`, `concreteproperties==0.8.0`, `sectionproperties==3.10.2`, `blue-prints==0.8.0`, `sympy==1.14.0`, `groundhog==0.15.0` (GPL-3.0, permitted: test-only, never linked into shipped code, same basis as `anastruct`).
Each is claimed by an entry in `✏️s/🔌️plugins/📕️norm/🔮️oracles/🔣️.json` (`norm-eurocode-*`: pinned version, SPDX license, `testOnly`, rationale, linked packages). The five tolerance profiles are registered there as `comparisonProfiles`.

Wheels (PyPI, checked 2026-11-01): the six libraries are pure `py3-none-any`; compiled transitive deps have cp314 wheels: `meshpy` (linux x64/arm64, macOS x64/arm64, win amd64), `cytriangle` (linux x64, macOS x64/arm64, win amd64; no linux arm64 wheel, so sectionproperties/concreteproperties/blue-prints cannot be installed on linux-arm64).
Skipped: `eurocodepy` (hard dependencies on mkdocs*, marimo, plotly, pyzmq, pandas: unacceptable weight for a test oracle, and it disclaims fitness; revisit with a vendored data-only extraction), `openseespy` (cp314 wheels and licence unverified).
Lock/sync state: see section 6.

## 2. Layout (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️harness`)

- `🐍️.py` entry: `--json` (stdin `{"cases":[...]}` -> `{"checks":[{"id","quantities","failures","utilization"}]}`), `--cases FILE...` (verify), `--cases FILE --bless` (rewrite expected from the first reference after a spread check), `--list`.
- `🧬️schema/🔣️.json` JSON Schema 2020-12 for case files (SI inputs, `citation{standard,edition,clause,annex,table,sha256}`, `expected{value,unit,profile,tolerance}`, `references[{oracle,adapter,check,kind}]`).
- `📏️tolerance/🐍️.py` the five profiles: `closed-form-v1` 1e-9, `eurocode-quantities-v1` 1e-4, `numerical-v1` 5e-3, `method-spread-v1` 3e-2, `table-exact-v1` exact.
- `🔌️adapters/<name>/🐍️.py` one module per library exporting `CHECKS: {check_id: fn(inputs) -> {quantity: float}}`. Present: `numerical` (scipy: parabola-rectangle flexure by quadrature, Ayrton-Perry chi by brentq, Euler N_cr by FE eigenproblem, beta from p_f).
- `🔍️coverage/🐍️.py OUTPUT.md` library-coverage introspection (`🔍️audits/oracle-library-coverage.md`).
- `🧪️tests/🐍️.py` pytest laws; shared smoke cases in `✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/🔮️oracle-cases/numerical.json`.
- Missing adapter, missing check or missing library raises; nothing is skipped. References disagreeing beyond the profile are reported as "oracle disagreement" and `--bless` refuses nothing silently (it fails the post-bless judge).

## 3. Python from Rust

`semio_s_plugin_norm_compliance::python_oracle` (`⚖️compliance/🧪️tests/🔬️python-oracle/🦀️.rs`, feature `compliance-testing`, already a dev-dependency of every artifact schema/inferences/io crate): `interpreter()` = `SEMIO_PYTHON` (set by the repo command layer, `🧰️framework/.../🎛️command/⚙️configuration`), else nearest ancestor `.venv` (`Scripts/python.exe` on Windows, `bin/python` elsewhere), else panic with `repo dependencies ensure` hint; `command()` adds `-I`; `run(script,&[args],stdin)` / `run_ok(...)` capture output. The twelve `Command::new("python3")` spawns in en1990-en1999 compliance/inference/io tests were replaced by `python_oracle::command()` (one-token edits).

## 4. Add a case + adapter (instructions for artifact agents)

1. Case: create `.../✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases/<topic>.json`, `"$schema"` = relative path to the harness schema, `artifact` = `enXXXX`, one case per quantity group with id `enXXXX.<clause>.<slug>`, SI inputs only (NA-DE parameters such as `alpha_cc`, `gamma_c` are inputs, never read from a library), `citation` (standard, edition, clause, table/annex), `expected` with placeholder `value: 0`, unit and a profile, and `references` naming adapters.
2. Adapter (only if no existing one has your check): create `🔌️adapters/<name>/🐍️.py` with a docstring starting with an emoji, `LIBRARY`, functions `fn(inputs) -> {quantity: float}` and `CHECKS = {"enXXXX.<clause>.<check>": fn}`. Import the library at module top: an uninstalled library must fail the run. Never import Rust-derived tables.
3. Bless: `<python> -I 🐍️.py --cases <file> --bless` (the repo `.venv` interpreter, see the dependency state), review the diff, then `--cases <file>` must print only `ok`.
4. Wire into Rust (optional, recommended): feed the case file to the Rust evaluator as data and compare `expected.*.value` with `eurocode-quantities-v1`; for live comparison use `python_oracle::run_ok(harness, &["--json"], Some(stdin))`.
5. Register: add the capability to the `enXXXX-1-compliance-independent` entry or a new `norm-eurocode-*` entry in the norm root registry; independent formulations are labelled `kind: independent-formulation` in the case, never third-party.
6. Add a pytest law if the adapter has a new numerical method (see `🧪️tests/🐍️.py`).

## 5. Registry

Every `enXXXX` `🔮️oracles/🔣️.json` got `enXXXX-1-compliance-independent` (`cross-semio-implementation`, capability `enXXXX-1-evaluate`, all five profiles): honest labelling of the Python compliance scripts as independent formulation; the third-party claims live in the norm root registry.

## 6. Status and open items

Done and verified: harness core, schema, tolerance profiles, `numerical` adapter, shared smoke cases (blessed; values match the closed forms: M_Rd 207.4 kNm, chi 0.597, N_cr 4.933 MN = pi^2 EI/L^2, beta 3.800), 6 pytest laws green (`python -m pytest ...🏃️harness/🧪️tests/🐍️.py`), registry entries, requests.md announcement.

NOT done (blocked by host capacity queue):
- `repo dependencies lock --all` is queued (running in background since this session started, log `🗑️generated/oracles/lock.log`, behind other agents' 6.4 GB `show projects` jobs; the MCP variant aborted after 1800 s). When it exits 0: run `repo dependencies sync --all`, then `<.venv python> -I ✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️harness/🔍️coverage/🐍️.py $TICKET/🔍️audits/oracle-library-coverage.md` and write the library adapters (`structuralcodes`, `concreteproperties`, `sectionproperties`, `blueprints`, `groundhog`, `sympy`) against the installed APIs, then `repo dependencies` baseline (`verify dependencies write-baseline`) so the ratchet sees the new `test-oracle` entries. The `pyproject.toml` pins and registry claims are already in place. `🔒️dependencies.json` baseline is intentionally unmodified.
- The Rust helper and the twelve call-site edits were not compiled (no `check` slot obtained); the edits are mechanical (`Command::new("python3")` -> `semio_s_plugin_norm_compliance::python_oracle::command()`); first `repo run @semio-tech/norm-enXXXX-rs:check` will confirm.
- `eurocodepy` skipped (dependency weight), `openseespy` deferred.
