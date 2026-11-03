# r2-dp report: dependency audit of test and oracle imports

Scope: third-party imports this ticket added or touched in tests and oracles, checked against the manifests, the lockfile, and runtime code. The ticket folder has no `T/` subfolder, so this report sits at the ticket root next to `r2-ph-report.md` and `r2-tc-report.md`.

## Manifests and lock

- Teaching Python package: `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/pyproject.toml`, with `uv.lock` beside it. Its `test` group is the place for test and oracle libraries (the staffage runners use `--group test`).
- Staffage TypeScript: `ajv` and `jsdom` are root `devDependencies` (`package.json:233`, `:256`), and `ajv` also sits in `🎓️teaching/package.json` devDependencies. Both are in `bun.lock` (`ajv@8.20.0`, `jsdom@24.1.3`). Nothing to change.

## Import audit

| Library | Where the ticket imports it | Declared before | Status |
|---|---|---|---|
| `scipy` | phase-change oracle `🫕️phase-change/🐍️.py:15-17`, `thermal-transient` oracle imports | `test` group `scipy==1.18.1` | ok, locked (`uv.lock` `scipy 1.18.1`) |
| `CoolProp` | phase-change oracle `🫕️phase-change/🐍️.py:14` (also `thermal-transient`, `respiration`, `condensation`, `leaf-transpiration`, `vapour-pressure` oracles) | nowhere (root pyproject, teaching pyproject, `uv.lock` all lacked it); the oracle used `uv run --with CoolProp` | fixed, see below |
| `jsonschema` | `🧍️staffage/🧪️tests/🐍️.py:311` (Schema vectors) | `test` group `jsonschema==4.26.0` | ok, locked |
| `svgelements` | `🧍️staffage/🧪️tests/🐍️.py:61`, `:97`, `:370` (`:370` new with the tc round) | only transitive, via manim (`uv.lock` `svgelements 1.9.6`) | fixed, see below |
| `xml.etree` | `🧍️staffage/🧪️tests/🐍️.py:340` | stdlib | no action |
| `ajv` | `🧪️tests/🎭️cast-export/🟦️.ts:10` | root and teaching devDependencies, `bun.lock` | ok |
| `jsdom` | `🧪️tests/🎭️cast-export/🟦️.ts:11` | root devDependencies, `bun.lock` | ok |

The phase-change TypeScript test (`🫕️phase-change/🟦️.ts`) imports only `vitest` and local modules. It reads the oracle output from `🧫️fixtures/🫕️phase-change/🔣️.json` as data, so scipy and CoolProp are not needed at test time.

## Changes made

1. `pyproject.toml` `test` group: added `"CoolProp==8.0.0"` (line 20) and `"svgelements==1.9.6"` (line 28), both exact pins in the group's style. The version of svgelements matches the `uv.lock` entry. CoolProp 8.0.0 is the version already in the uv cache. Its wheel tag is `cp312-abi3-win_amd64`, which fits `requires-python = ">=3.12,<3.13"`. Linux and macOS wheels were not checked here.
2. `🫕️phase-change/🐍️.py:7`: the run line in the oracle docstring now reads `uv run --project <teaching python package> --group test python -I 🐍️.py [--write]` instead of `--with scipy --with CoolProp`, so the oracle runs on declared dependencies.

## Lock state

- `uv.lock` is not refreshed. It has no CoolProp entry, and the new `svgelements` pin is a new direct requirement, so the lock is stale until refreshed.
- `repo dependencies lock --help` works (usage printed, no error). The command takes `--all` or `--scope <owner>`. I did not run it, because `--all` rewrites every owner's lock while about 20 agents share these files, and the brief says to refresh once infra is back. To refresh: `repo dependencies lock --scope <teaching python owner>` (or `--all` once quiet).
- Until the refresh, `--group test` runs will try to re-lock.

## Runtime imports

A grep over the non-test code of `🧍️staffage` and `🎨️visuals` found no imports of scipy, CoolProp, jsonschema, svgelements, ajv or jsdom. The only runtime third-party libraries are manim and numpy, declared in `dependencies`. The manim package imports svgelements and scipy itself, under its own dependency tree, which is not this ticket.

## Data committed?

Not yet. The phase-change folders (`🫕️phase-change/` tests and `🫕️phase-change/` fixture), the whole `🧍️staffage/🧪️tests/` folder of the visuals package, and the `🐍️staffage_only.py` and `🔣️staffage_only.json` vectors are untracked (`??` in `git status`). The fixture `🔣️.json` is data written by the oracle with `--write`, and the test reads it without scipy or CoolProp. It is ready to commit, but I did not commit (the brief and AGENTS forbid it).

## Pre-existing gaps, not from this ticket (not changed)

These imports already existed in other test or oracle files and are not declared in the teaching `test` group: `anastruct` (cantilever, simply-supported, two-foot-stance, tied-canopy, centre-of-pressure), `pandas` and `pyet` (penman-monteith), `psychrolib` (vapour-pressure), `ezdxf` (dxf-reopen), `reportlab` (pdf-reopen), `pythermalcomfort` (two-node-gagge), and `pathops` and `PIL` in the semio-style test (probably via manim, but not declared directly). The `CoolProp` pin now covers the oracles that import it.

## Open

- Run `repo dependencies lock` once infra is back, then check that `uv.lock` contains `coolprop 8.0.0` and `svgelements 1.9.6`.
- Decide on the pre-existing gaps above (declare or drop).
- Commit the untracked ticket data when the owner asks.
