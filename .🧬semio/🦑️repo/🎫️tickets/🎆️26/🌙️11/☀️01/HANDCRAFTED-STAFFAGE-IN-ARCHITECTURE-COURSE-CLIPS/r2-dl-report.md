# r2-dl report: zero-touch test dependencies

## Verified imports (all under `🎓️teaching/🏛️architecture`, none declared before)
Test/oracle files under `🧍️staffage/🧪️tests/<case>/🐍️.py` import: `anastruct` (cantilever, simply-supported, two-foot-stance, tied-canopy, centre-of-pressure), `pandas` + `pyet` (penman-monteith), `psychrolib` (vapour-pressure), `ezdxf` (dxf-reopen), `reportlab` (pdf-reopen), `pythermalcomfort` (two-node-gagge). `PIL` and `pathops` are imported in `⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🎨️semio-style/🐍️.py:53,106` and in clip code (`PIL`); both were only transitive via manim.

## Declared (teaching python `pyproject.toml`, `test` group, exact pins)
`anastruct==1.7.1`, `ezdxf==1.4.5`, `pandas==2.3.3`, `pillow==13.0.0`, `psychrolib==2.5.0`, `pyet==1.5.0`, `pythermalcomfort==4.8.0`, `reportlab==5.1.0`, `skia-pathops==0.9.2` (import name `pathops`), plus dp's `CoolProp==8.0.0` and `svgelements==1.9.6`. I first wrote `pandas==3.1.0` (latest on PyPI); the lock resolution settled on `pandas==2.3.3` (someone/the tool changed the pin, kept as is). All are python-3.12 compatible; the lock carries wheels for win/mac/linux of the resolved versions (CoolProp has cp312-abi3 wheels for all three).

## Lock
`uv.lock` now contains all pins in `[package.metadata.requires-dev]` (coolprop, svgelements and the nine new ones; resolved entries for each). It was regenerated through repo tooling only (`repo dependencies lock ...`, owner scope probed with `--scope`), never `uv lock`. One of my `repo dependencies lock` invocations hit the 110 s shell timeout (exit 143, `OwnedCommandFailure ... failed (143)`, from my own `timeout`), but the lock file on disk is complete and consistent with `pyproject.toml` (timestamps 15:44:36 / 15:44:39). No infra refusal was seen. Note: I did not find the exact owner name; the lock refresh seems to have been completed by a concurrent invocation (mine or another agent's).

## Phase-change oracle
`T/m9_tests.sh` is unittest-only and finds no tests in the oracle (it is a script, "Ran 0 tests"). Ran the oracle itself in verify mode through the `test` group: `uv run --project <python pkg> --group test python -I 🧍️staffage/🧪️tests/🫕️phase-change/🐍️.py` exits 0 and prints kettle-water, ice-plate and boiling point/latent heat (CoolProp + scipy resolved from the locked group).

## Open
- Other oracle docstrings still say `uv run --with ...` (thermal-transient, condensation, parcel-jet, blackbody-exchange, leaf-transpiration, penman-monteith, vapour-pressure, export-performance, two-node-gagge, respiration); they should read `--project <python pkg> --group test`.
- Linux/macOS runs not executed here (only Windows).
