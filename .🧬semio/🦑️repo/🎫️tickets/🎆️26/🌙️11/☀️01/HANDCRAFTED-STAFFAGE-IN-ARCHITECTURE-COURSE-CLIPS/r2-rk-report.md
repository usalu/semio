# r2-rk: link, path and symbol check of S/V markdown and docstring links

Scope: `S/README.md` (the only markdown under S and V), and every `.py`/`.ts`/`.tsx` file listed in `🧰️cf3/files.json` (138 files). Checker: `🧰️rk/links.py` (kept in the ticket).

## Fixed (docs and docstrings only, no code logic)
Broken relative `@see` / markdown links, all resolved against the file's own folder:
- `V/🐍️.py:227` `@see` BIM base: four `../` to three.
- `V/🐍️.py:291` `@see` cast API in `fitted_staffage`: `../🧍️staffage` to `./🧍️staffage`.
- `V/🧲️physics/.../🧲️fundamentals/🐍️.py:439` external-venetian-blind cast: five `../` to four.
- `V/🎯️optimization/.../🏠️room/🐍️.py:1336, 1349` staffage README link: five `../` to four.
- `V/🏗️bim/.../📊️information/🐍️.py:211` staffage README link: four `../` to three.
- `S/.../🌐️i18n/🔠️english/🎛️viewer.ts`, `📖️detail.ts` and `S/.../🔡️german/🎛️viewer.ts`, `📖️detail.ts`: `@see ../../` pointing to `modules/` to `../` (sibling locale).
- `S/.../🖨️representation/📐️measure/🟦️.ts:443` `@see` to `representation/🟦️.ts`: `../../../` to `../`.
- `S/README.md` Guard section: `🧪️tests/🐍️staffage_only.py` resolved to nothing under S. Now the full repo path from the teaching root, with `🔣️staffage_only.json` named as its sibling.

## Verified, no change needed
- Every `README.md` backtick path under S resolves (schema, catalog, modules, packages, site, tests, fixtures, catalog devices and environments).
- Package names `@teaching/staffage`, `@teaching/staffage-site`, `@teaching/staffage-react` exist in their `package.json`.
- Exporter flags `--manifest --ids --out --catalog --parent` match `parseArguments`; `--lang` is refused by the cast exporter (`FLAGS`). The README's "there is no `--lang`" is correct.
- Exit 130, SIGBREAK/SIGHUP handling, `[pid]`, `[cast i/n]`, `[done/total]` every 50 frames, 600-frame thinning, `DEFAULT_FPS` 15, scale 50 all match the code.
- Cast schema `KEYS` (16 fields) match the README cast row table; `transportAgeS` requires `transport` and `clip`, as the README says.
- Index keys `boundsM`, `figureBoundsM`, `heightM`, `loop`, `fps` exist in the exporter.
- Python API: `staffage`, `staffage_motion`, `staffage_height`, `staffage_parts`, `ensure_casts`, and `fitted_staffage(cast, *, width, height, color, rate, loop, window)` in `visuals` match the README table.
- Cache root `.🧬semio/🎓️teaching/⚡️cache/staffage`, `SEMIO_TEACHING_CACHE_ROOT`, `.tmp-` prefix and 24 h prune match `_cache_root`, `_prune`, `_PRUNE_AGE_S`.
- Job object / process group handling exists (`_job` and the process-group flags in `V/🧍️staffage/🐍️.py`).
- Thermal: `--environment` refused unless `--view thermal`; `referenceEnvironment` present in the solar collector catalog (`summer-heatwave-calm`); `phaseChange`-related fields exist.
- Guard test names `thermal_span`, `rule_a`, `rule_b`, `rule_c` exist in `🐍️staffage_only.py` and `🔣️staffage_only.json`.
- Catalog devices `gas-meter`, `electricity-meter`, `hot-water-cylinder`, `solar-thermal-collector` and `energy_tank` symbol exist.
- React `StaffageFigure` / `staffage-figure` exist under `🎯️targets/⚛️react`.

## Still open / not verified
- `S/README.md` lines 138-140 use `repo run ... --lang=en`. I could not confirm that `repo run` accepts `--lang` (the brief says `repo` takes `language=en`). Not changed.
- Symbol-level grep of every docstring in the 138 files was not done. Only links and paths were checked there.
- `V/🧪️tests/.../🟦️.ts:11` template literal `../../🗂️catalog/${path}/🔣️.json` is code, not a link. Left as is.
- Other agents edit these files concurrently, so line numbers may shift.
