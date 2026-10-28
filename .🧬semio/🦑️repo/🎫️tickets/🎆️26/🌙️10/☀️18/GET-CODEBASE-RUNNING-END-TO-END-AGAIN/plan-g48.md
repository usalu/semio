# Landing plan G48–G51 (coordinator, 2026-10-27 21:3x)

Source: `result-audit14.md` §6 plus the audit13 leftovers (`result-mwrebase.md`, `result-lrrebase.md`) and germanqa (`result-germanqa.md`). One product verdict chain per wave (course, course:test, dash, print, hot ×2, repo-mcp, edit probes).

| Wave | Content | Pre-step |
|---|---|---|
| G48 | nxdeadline release · quiet16 hold → lastrun · builderlock multiwriter · Group A: audit11 reapi (NOT nolegacy/reapi), benchci acceptance (.github part stays a user decision), edit30 nxbuild, cbudget shard + clock drift | stage.py check per batch; clock rebuild.py |
| G49 | Group B compliance ×8 (rebuild_all.sh → apply_all.sh) · Group C isolate helper → rest → fence · Group D nolegacy lease → lang + nobaseline (same pass) | rebuilds after G48 (multiwriter touches generations/claims, which compliance libgraph and schemafirst generations drifted on) |
| G50 | Group E schemafirst (rebuild_all.sh, order.txt) · Group F witpub (edit.py) + audit11 warmderive (rebuild.py; edit30 warmderive superseded) · Group G langpick (build.py → land.sh) · germanqa (rebuild_all.sh → apply_all.sh → law GREEN → repo-mcp:install) | rebuilds after G49 |
| G51 | Group H incfacts (make_batch.py re-cut; one-time full re-inference) | after G50 verdict |

Done already: edit30 freshmtime, edit30 prodclock (live).
Not landed by the gate: `.github` changes (user decision).

## Update 00:1x (10/28)
- G49 as landed: compliance ×8, isolate ×11, nolegacy lease. nolegacy lang + nobaseline REVERTED (loader > 32767) → lane langbudget re-cut; lang6 `rest` lands after lang.
- G50a (after chain g49r): schemafirst ×15 (sfrebuild, order.txt) → witpub (cbrebase) · cbudget shard · hotreg failuredir · E1 via mergeland `C/backlog-tools/land.py` core → nx-patch → nxlock (+ docs3 core/nxlock) — audit15: stage.py refuses, mergeland merges clean, nxlock never before core.
- G50b: langbudget loader + nobaseline + lang → lang6 rest (build.py first) · audit11 warmderive (after memo) · witpub (cbrebase, after bazelscript) · cbudget clock → shard · isov fixes · handshake · diskproof closurefix/futurestamp/acceptjournal/lawregex/mirrordrop · agents3 narrow (gate decision: starvation-free beats makespan per target T9).
- G50c: langpick (build.py, re-cut after E1) · germanqa (re-cut after E1/docs3) · then repo-mcp:install.
- G51: incfacts.
- G50c sequence (after g50 GREEN): v50triage fix (make.py first) · runner tsdebt · nodecast · innerclock · forceclock · devslot · audit11 warmderive · diskproof futurestamp · germanqa (apply_all.sh, law GREEN) · `LP_GO=<go 1.26.9> python3 -I C/langpick-tools/build.py` → `sh C/langpick-tools/land.sh apply` (core copies imports dashboard routing) → launcher → reinstall launcher (install 🐚️.sh) + `repo run repo-mcp:install` → ONLY THEN launcher2 (old binary refuses every command after launcher2). Smoke after each: loaderlen + `bun repo preflight`.
- G50d (after g50c GREEN): langpick core..launcher (launcher2 held on installer decision) · lintglob globproj → bazelglob → buildscript (rebuild.py first) · handshake/isov/osreds/hubreds/dashnoop/mem3 fixes as they report · then G51 incfacts.
- G50d part 2 (after g50d verdict): lintglob globproj → bazelglob → buildscript (rebuild.py first) · mem3 releaseturn · handshake (build.py rebase first) · ownerbytes (build.py first; smoke loaderlen — revision ceiling 64→256 KiB) · then sealed flip (sealprep flip.sh + chainlaw, self-g post-flip, promote.sh) · installown/ownerbytes/handshake/isov/osreds/hubreds as they report · G51 incfacts last.
- open (ownerbytes heads-up): ResetOwner schema requires only graph+nx reset targets; cache-reset-rust/products routes have no authored target in ⚡️caching owner → nothing validates them. Route to a small lane after G50d.
