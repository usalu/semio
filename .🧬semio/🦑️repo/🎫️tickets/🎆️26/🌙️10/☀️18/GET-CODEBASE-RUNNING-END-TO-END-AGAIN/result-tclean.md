# Result tclean (ticket-folder source copies and tool output cleanup)

## Outcome
- Scope: this ticket folder and `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/`.
- Deleted 10121 entries, 6,201,498,265 bytes (about 5.8 GiB apparent; `df` showed 6.08 GB more free), plus 666 directories left empty. Directory sizes: this ticket 3.0 GB -> 365 MB (333 MB of that is `🗑️generated`, mostly kept because it is protected or recent); `monolith-split` 3.2 GB -> 60 MB.
- Over-length paths (Windows checkout, `C:\Users\username\source\repos\semio`, limit 260, ticket scope): 416 -> 0. Whole repo with the `C:\git\semio` root (limit 259, scope `all`): 0.

| Category | Entries | Bytes |
|---|---|---|
| A. source copies in `orig`, `orig-*`, `*-orig`, `backup`, `pre-*`, `before`, `snap`, `fmt-tmp` directories (e.g. `uicarve-tools/orig`, `frd-tools/orig`, `tc3-tools/orig`, `hdr3/4-tools/orig`, `r5b/r7c/r9a-tools/orig*`, `nxpart-tools/backup`, `c6*-tools/orig`) | 5902 files | 264,415,640 |
| A. copy files by name (`*.orig`, `*-orig*`, `orig-*`, `*-original*`, `*-before*`, `*-base.rs`, `.bak`, `.prev`, `gocomp/orig.go.txt`, `gocomp/component-test-snapshot.go.txt`, `gocomptest/orig.go.txt`, `hdc-tools/orig-hub-rs.tar`, `harden2-tools/*-base.rs`, `met-tools/*-original.ts.txt`) | 39 | 16,689,841 |
| B. output extensions (`.log`, `.list`) | 6 | 78,956 |
| B. `.txt` dumps with no reading script (`tsp-tools/all-s.txt` 17.5 MB, `tst2-tools/list-*.txt`, `rhu2/3-tools/work/*`, `touched*.txt`, `newdirs.txt`, ...) | 260 | 22,291,252 |
| B. generated JSON (`cg2-tools/dump1/2.json`, `*-report.json`, `touched*.json`) | 11 | 2,817,043 |
| C. top-level entries of both `🗑️generated` folders older than 30 min (`*.err` test/lib logs, `*-run`, `bzl-closure`, `scc`, ...) | 3903 | 5,895,205,533 |

## Kept on purpose
- Skipped as instructed: `repoapi-tools/`, `cc-tools/`; `🗑️generated` entries named `cc-*`, `repoapi*`, `lw*` and 15 entries touched in the last 30 minutes (e.g. `lay12-run`, `seed-run`, `repoapi`, `scc`; 257 MB).
- Also protected (extra caution): every `lw*-tools/` folder (all files, not only `exports-needed.json`), `stale-publication-intents/`, any file or `specs/` input (`exports-needed.json`, `exports-final.json`, `dependencies-needed.json`, `files.tsv`), anything modified in the last 30 minutes.
- All `.md` reports, all scripts (`.py .ts .mjs .cjs .sh .js .go .tsx .mts`), `.toml` outside copy directories, `.tsv`/`.json` tool inputs, `.txt` files that a kept script reads.
- Left unclassified (not deleted): `.rs` files outside copy dirs (21, e.g. `pfx-tools/stub`, `bd12-tools/stage`), `lay9-tools/vt-stubs/*.js` stubs.

## Verification
- Tool: `tclean-tools/clean.py` (input script; dry run first, then `--apply`). Copy rule = path under a copy-named directory or copy-named file; the three non-source files inside copy dirs were handled by extension (the one `.md` report in `bd8-tools/orig` was kept).
- `python3 tclean-tools/overlong.py` replicates the test's rule (`git ls-files` tracked + untracked-not-ignored, `win32.join(root,path)` UTF-16 length > limit, existing on disk): before 416 over (r5b 141, r7c 115, hdr4 67, hdr3 59, hdr 25, r9a 9, all in `monolith-split`), after 0 of 67097 paths.
- `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🎫️ticket-protection/🟦️.ts -t "Windows checkout ticket paths"` still FAILS, but not on paths: its own 15 s timeout fires because `git ls-files --others --exclude-standard` takes 30-38 s on this tree (three such calls per run), so `spawnSync` returns status null ("Unhandled error between tests", 19 expects passed up to the first git call). Open: speed up the untracked scan (large untracked/ignored directory walk) or raise the test timeout; the path assertion itself holds (0 over-length).

## Open
- Remaining long paths: none. 
- `bd12-tools/locks` still contains only the two `.ts` scripts (the `bd12-locks-before.tgz`, `gotix-orig`, `c6ts-orig` backups named in the brief were already gone).
