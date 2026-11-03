# r2-or report: oracle run lines and test-group pins

## Coordinator priority: teaching env resolution
- `uv lock --check` in `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python`: exit 0 ("Resolved 70 packages"). The lock is consistent with `pyproject.toml`.
- `bash T/m9_tests.sh <staffage_only module>` (module `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️staffage_only`): exit 0, `Ran 20 tests`, `OK`. uv resolved and ran the `test` group on this machine.
- No pin was changed for resolution. The pandas pin is already `2.3.3` in `pyproject.toml` (no `3.1.0` left). The failure bp saw is not reproducible now; it may have hit a stale lock state during the concurrent regeneration.

## Task 1: docstring run lines
Ten oracle docstrings under `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/<case>/🐍️.py` had `uv run --with ...`. Each now reads the phase-change form:
`uv run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --group test python -I 🐍️.py [--write]`
- `⏳️thermal-transient`, `☁️condensation`, `🌀️parcel-jet`, `🌡️blackbody-exchange`, `🍃️leaf-transpiration`, `🍃️penman-monteith`, `💧️vapour-pressure`, `🚀️export-performance`, `🫀️two-node-gagge`, `🫁️respiration`.
- Grep for `uv run --with` under `🎓️teaching/🏛️architecture`: 0 hits. 11 oracle files now carry `group test python -I` (the 10 plus phase-change).

## Task 2: pins vs uv.lock
All 18 exact pins in the `test` group match the locked versions (CoolProp 8.0.0, anastruct 1.7.1, ezdxf 1.4.5, fonttools 4.63.0, formulas 1.3.4, jsonschema 4.26.0, openpyxl 3.1.5, pandas 2.3.3, pillow 13.0.0, psychrolib 2.5.0, pyet 1.5.0, pythermalcomfort 4.8.0, reportlab 5.1.0, rhino3dm 8.35.0, scipy 1.18.1, shapely 2.1.2, skia-pathops 0.9.2, svgelements 1.9.6). No pyproject edits were needed.

Platform coverage (from lock entries, Windows / macOS / Linux):
- Compiled, cp312 wheels for win, macOS and manylinux/musllinux: CoolProp (abi3), ezdxf, fonttools, pandas, pillow, scipy, shapely, skia-pathops (abi3), rhino3dm (win_amd64 and macOS universal2 and manylinux; no win for x86 in the win32 list, but win_amd64 is present), anastruct (win_amd64, macOS, manylinux).
- Pure Python wheels (`py3-none-any` or `py2.py3-none-any`): formulas, jsonschema, openpyxl, pyet, pythermalcomfort, reportlab, svgelements.
- Sdist only: psychrolib 2.5.0 (pure Python `PsychroLib-2.5.0.zip`, no compiled code, so it builds on all three OSes). This is the only package without a wheel. It is an acceptable gap but worth knowing.
- Linux/macOS runs were not executed here (Windows only).

## Open
- psychrolib is sdist-only; a wheel does not exist on PyPI for 2.5.0.
