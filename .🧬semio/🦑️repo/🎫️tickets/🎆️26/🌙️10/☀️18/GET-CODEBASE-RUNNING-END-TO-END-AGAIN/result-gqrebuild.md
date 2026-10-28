# Lane gqrebuild (10-28 01:4x): all 7 germanqa batches rebuilt from the G50 live

FOR coordinator: 7 staged batches in `C/germanqa-tools/staged/` (151 files: process 23, libbuild 74, libcache 22, libother 7, mcp 4, modules 14, docs 7), ALL `base` on live at 01:41 (rebuilt 01:35, `apply_all.sh check` prints no non-base row). Land as ONE group, after G50 (E1 + docs3 + langbudget + schemafirst are in the base).
- rebuild if live drifts: `sh C/germanqa-tools/rebuild_all.sh` (re-reads live, runs the edit law; no missing anchor today)
- check: `sh C/germanqa-tools/apply_all.sh /Users/ueli/Documents/semio check`
- apply: `sh C/germanqa-tools/apply_all.sh /Users/ueli/Documents/semio apply`
- law after apply (GREEN): `python3 -I C/germanqa-tools/law_germanqa.py /Users/ueli/Documents/semio` and `python3 -I C/germanqa-tools/law_edits.py C/germanqa-tools/staged/*`
- then (the committed `💻️client/🔌️mcp/mcp` binary embeds `RepositoryInstructions`): `repo run repo-mcp:install`
- revert: `sh C/germanqa-tools/apply_all.sh /Users/ueli/Documents/semio revert`
- proof after landing (cheap): `SEMIO_LANGUAGE=de bun repo preflight`, `SEMIO_LANGUAGE=de bun repo doctor`, `bun repo preflight` (captured command; the argv law below); expect one graph re-inference (catalogs are inputs), as with the first germanqa plan.

## What was different from the 10-27 cut
- Live grew German text since: schemafirst `const: {en:{…}, de:{…}}` description tables (`LimitsDescriptions` / `PolicyDescriptions`, 28 files; the existing pair reader already walks them) carry ASCII fallbacks (`Eigentuemer`, `Bloecke`, `Zaehler`, `Praefixe` …). The law's corpus detector found 73 words the map lacked. Fix in the lane folder only:
  - `translit_map.json` 103 -> 176 words, re-derived with the macOS German spell checker as oracle (new `C/germanqa-tools/spell.swift`: `swiftc -O spell.swift -o <out>`, reads words on stdin, prints the rejected ones; command: `python3 -I translit_map.py <root> <out.json> <spell binary>`, then union, old entries win, `Daemon`/`Daemons` dropped: the oracle prefers `Dämon`, but this is the Nx daemon).
  - `rules.py`: 4 phrase rules for new texts (`Eingabe-Hülle` -> `Eingabe-Abschluss`, `Hüllen-Stempel`/`Hüllen-Block` -> `Abschluss-…`, `Der Beleg der blockierten Beförderung` -> `Der Eintrag …` (EN: record, not receipt), `Die Zeile des Abbildbuchs` -> `Die Zeile der Schnappschuss-Bilanz` (EN: retained snapshot ledger), `das (exklusive) Start-Lease` -> `die … Start-Lease` in the build guide).
  - `runtests.py` takes `GQ_MIRROR` and `GQ_GEN` (defaults unchanged).
- Row counts vs 10-27: libbuild 62 -> 74 files, docs guide re-cut on the post-docs3 bytes (docs3 blocks kept: the batch reads live), process `policy.json` on the post-E1 bytes (E1 and langpick `copies` overlap resolved by the rebuild, not by a merge).

## Laws (mirror `gqr` at `/private/tmp/m/gqr`, dropped)
- `law_germanqa.py` on live: RED 1442 findings; on the mirror with all 7 applied: GREEN 0 findings. `--selftest` ok (13 red fixtures fire, 0 on green). `law_edits.py` GREEN (English leaves byte-identical, only German leaves changed). All 7 `stage.py` keep file modes (`xplat5-tools/tests/test_stage_mode.py` OK x7).
- langbudget argv law (`🏷️loader-identity` + `🧭️sites` suites): 8 pass / 0 fail before and after. Stream loader 26,377 chars, file loader 30,361, identical on live and on the mirror with all batches. The authorizer sites schema (`🔏️authorizers/🧭️sites/🧬️schema/🔣️.json`) grows 8,475 -> 8,510 bytes (+35, German leaves only, in libcache); the loader no longer inlines it, so the command line is unaffected.
- `bun repo preflight` through the official door on the mirror: complete before and after (4.4 s / 4.9 s; the mirror needs `✏️s/🧑‍💻dev/🧩️catalog/📣️publication` + `package.json` + `tsconfig.json` copied in, as in langbudget).
- Test directories (the 89 directories next to the changed files, `runtests.py base|after`, sealed `mirror.py run`, `bun test --timeout 60000`): base 1346 pass / 133 fail, after 1346 pass / 133 fail; failure sets identical in 89 of 89 directories (36 directories fail on base already: envelope-only suites, stale fixtures, load). Result JSON: `C/🗑️generated/gqrebuild/tests-base.json|tests-after.json`.
- Not run: Go/Rust tests (cargo/go budget; edits are string literals), `bun repo` product builds (coordinator).

## Notes
- The first mirror was rebuilt once: the post-apply edit of the rules needed a re-apply, and the base run was repeated on a fresh mirror so base and after share the same non-batch bytes (stale peer Cargo.lock diff in `mirror.py diff` is live churn, not a batch row).
- Whole-repo law still excludes the product README German section (316 KB, unread), as before.
- Files: tools `C/germanqa-tools/` (`rules.py`, `translit_map.json`, `spell.swift`, `runtests.py` changed), logs `C/🗑️generated/gqrebuild/[DEBUG] *`, old staged copy `C/🗑️generated/gqrebuild/staged.old` (10-27 cut; delete at ticket end).
