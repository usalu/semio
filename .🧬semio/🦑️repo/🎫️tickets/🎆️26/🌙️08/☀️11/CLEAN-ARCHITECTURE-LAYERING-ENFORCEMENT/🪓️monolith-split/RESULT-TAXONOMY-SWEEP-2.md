# Taxonomy Sweep 2: folders born since 2026-10-15

Slug: TAXONOMY-SWEEP-2, prefix `tx2`. Executor: Sonnet. Edited file: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (JSON only, +8 lines, one new member row). No folder was renamed.

## 1. Verdict

Nearly everything wave 5 created was already resolved. `RESULT-TAXONOMY-SLUG-PATTERN.md` does not exist, but the slug-pattern rule itself has landed. `memberSlugPattern` is implemented in both loaders (`🔍️discovery/🟦️.ts` and `🧹️normalization/🔣️taxonomy/🟦️.ts`) and in `matchDirectoryKind`. The live taxonomy already used it for `members-of-tests` (kebab) and `crate-module-children` (kebab or PascalCase, down from 778 hand-listed names to 73).

Of 5414 directories born since 2026-10-15 (or absent from HEAD), 5412 resolved before my change. The remaining 2 were `📦️publication` and `📋️contract` under `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages`. Both sit below older, also unresolved parents. I added one pattern row, `semio-tech-module-children`, owned by `semio-tech`. After it, 0 born-since-10-15 directories have a finding, and 0 ambiguities exist.

## 2. Method

- Directory list: `git ls-files --cached --others --exclude-standard` (115067 directories at the end). HEAD directory list: `git ls-tree -r HEAD`.
- Born since 10-15 means `st_birthtime >= 2026-10-15` or absent from HEAD, restricted to non-dot paths. `.🧬semio` ticket folders are excluded, as in sweep 1 (free-form names).
- New simulator `tx2-tools/sim.ts`. It no longer re-implements the matcher. It calls the real `loadTaxonomy` and the real `matchDirectoryKind` (so `memberSlugPattern` is honoured exactly) over the directory list, with the same parent and ancestor-kind walk the inventory uses. It runs in about 2 s and accepts any candidate taxonomy file.
- Real inventory: `tx2-tools/inv.ts` (calls `inventoryTaxonomy` on the whole repo, about 8 min each). It ran before and after my install.
- Fidelity: the real run reported 3657 `directory-kind-unresolved`, 1156 of them outside dot paths. The simulator reported 1182 outside dot paths (it overcounts fixed-contract folders, 26 here). It had 0 born-since-10-15 findings beyond the same 2 that the real run found. The only real findings the simulator lacked were 2 ticket folders created after the directory list was taken.

## 3. Before and after

| Measure | Before | After |
|---|---|---|
| Real inventory, `directory-kind-unresolved`, whole repo | 3657 | 3642 |
| Same, outside dot paths | 1156 | 1141 |
| Real, born since 10-15 or not in HEAD | 2 | 0 |
| Real, `directory-kind-ambiguous` | 0 | 0 |
| Simulator, all directories unresolved (incl. dot paths) | 7578 | 7563 |
| Simulator, born since 10-15 (5414 directories) | 2 | 0 |
| Simulator, born since 10-16 (1552 directories) | 0 | 0 |
| Simulator, previously resolved directories whose kind id changed | n/a | 0 (107489 compared, 0 lost) |
| `validateTaxonomy` problems | 0 | 0 |
| `loadTaxonomy` | ok, 758 directory kinds | ok, 758 directory kinds |

Real before/after diff by `(code, path)`: exactly 15 `directory-kind-unresolved` rows removed (the 15 `semio-tech` folders listed in section 4). The only added rows are `semantic-stem-unresolved` and `kind-only-basename` for new files in peer ticket folders (`audit5-tools`, `RESULT-SDK-BOUNDARY-2.md` and others, created between the two runs). Both codes are outside this task.

How the 5414 born-since-10-15 directories resolve (simulator, before my change):

| Resolution | Directories |
|---|---|
| Structural directory kind | 2813 |
| `crate-module-children`, by slug pattern | 1460 |
| `members-of-tests`, by slug pattern | 343 |
| Exact member name in an existing row (`members-of-tests` 198, `crate-module-children` 159, `members-of-members-of-elements` 104, `members-of-wgpu-target` 95, `members-of-members-of-modules` 70, `members-of-modules` 50, others) | 797 |
| Unresolved | 2 |

By area, the 1552 directories born on 10-16 are `🧰️framework/🛍️products/💻️os` 681, `✏️s/🔌️plugins` 297, `🧰️framework/🛍️products/🦑️repo` 204, `🧰️framework/📦️packages/🟦️typescript` 167, `🔨️modules/🏃️process` 59, `📓️print` 43, `🎠️kernel` 34, `⏪️time-travel` 17 and about 50 elsewhere. All of them resolve. That includes the root-script split (`🎛️commands`, `⚖️laws`), the kernel engine, SQLite-host and sync crates, the plugin-host producers, the renderer Shell and frame-core crates, the TS splits, the misplaced e2e tests, the SDK facades and `🫧️transient`.

## 4. The change

One new row in `semanticDirectoryMemberKinds`, after `crate-module-children`:

```json
"semio-tech-module-children": {
  "ownerKindIds": ["semio-tech"],
  "memberNames": [],
  "memberSlugPattern": "^[a-z0-9]+(?:-[a-z0-9]+)*$",
  "source": "registry"
}
```

Why this mechanism and owner:
- A pattern row needs no per-name list, which is the preferred mechanism now that the rule exists.
- `semio-tech` is the farthest ancestor kind for everything below `🏢️semio-tech`. Exact member names and structural kinds win first, and every other pattern row matches nearer or not at all. So the row can only affect directories that currently have no kind. The simulator confirmed 0 kind-id changes over all 115067 directories.
- No other pattern row has owner `semio-tech`, so no two pattern rows match at the same nearest distance (0 ambiguities in the simulator and the real run). The validator's identical-pattern-per-owner check passes.
- The row admits any RGI emoji plus a kebab slug. All 15 names below fit it.

Directories resolved (all kind `semio-tech-module-children`), under `🏢️semio-tech/🎡️play/🔨️modules/`:

- Born since 10-15 (the 2 target findings): `📦️site/📄pages/📦️publication`, `📦️site/📄pages/📦️publication/📋️contract`.
- Older parents that had to resolve first, and their siblings: `📦️site`, `📦️site/🆕️fresh-build`, `📦️site/📄pages`, `📦️site/🗺️map-tiles`, `📦️site/🗺️tile-serve-mode`, `🧩️runtime`, `🧩️runtime/♻️activation`, `🧩️runtime/♻️activation/🌐️vite`, `🧩️runtime/📦️assets`, `🧩️runtime/🧪️e2e`, `🧪️e2e`, `🧪️e2e/📦️release`, `🧪️e2e/📦️release/🎭️fixture`.

Install: the live file's SHA-256 was `6710cb0c…b4abe4b78e` when I started. Right before installing I re-hashed it, rebuilt the candidate from the live file with `tx2-tools/build.py` (a JSON round trip that was byte-identical first), ran `validateTaxonomy` and `loadTaxonomy` on the candidate (0 problems, ok), re-checked the hash and then `mv`d the candidate over the file (same filesystem, atomic). New hash: `6979fd79…27bcf8a2d`. `git diff` against HEAD shows only the 8 added lines.

## 5. Renames

None. The real inventory shows no `path-emoji-*` finding for any of the 5414 born-since-10-15 directories, and a per-path `pathEmojiStatuteFindings` scan of all of them returned nothing. I did not touch any `#[path]` or import reference.

## 6. Open points (not changed)

- `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages` uses `📄` without U+FE0F, and `📄` is on the repo's generic file emoji list. The inventory does not flag it, and it is older than 10-15 (born 2026-09-24), so I left it. Renaming it means updating at least 7 importers (`🏗️builder/🌐️vite/🟦️.ts`, `📦️site/📜️script.ts`, `🧪️e2e/📜️script.ts`, `🧪️e2e/📦️release/🟦️.ts`, `🧪️tests/🧪️playrelease/🟦️.ts` and others).
- `🏢️semio-tech/🎡️play/public` (no emoji) stays unresolved. It is older debt and probably a Vite convention name.
- The remaining 1141 non-dot unresolved folders are older debt, 1001 of them under `♻️mit-bestand/📋️bericht`, plus `✏️s/🔌️plugins` 70, `🧰️framework/🛍️products/🦑️repo` 26 and others. None was born since 10-15. I did not sweep them.
- The 2451 unresolved dot-path folders under `.🧬semio` tickets are free-form ticket folders and stay out of scope.
- The sibling-duplicate-emoji debt from sweep 1 is unchanged.
- I ran no taxonomy unit tests. The `memberSlugPattern` tests live in `🧪️tests/🧷️member-slug-pattern/` and were not part of this task.

## 7. Commands run

- `git ls-files --cached --others --exclude-standard` and `git ls-tree -r HEAD` to build the directory lists.
- `bun tx2-tools/sim.ts <taxonomy> <dirs> <out>` on the before taxonomy (7576, then 7578 unresolved after a list refresh), on candidate 1 (7561) and on the installed file (7563). 0 ambiguities each time.
- `bun tx2-tools/inv.ts` before (3657 unresolved, 62216 violations in total) and after (3642, 62217).
- `bun tx2-tools/validate.ts` on the candidate and on the installed file: `validateTaxonomy` problems=0, `loadTaxonomy` ok with 758 kinds.
- `bun tx2-tools/emoji.ts` over the 5414 born directories: no findings.

## 8. Artifacts (all in `🗑️generated/`, not deleted)

- Tools: `tx2-tools/` (`sim.ts`, `inv.ts`, `validate.ts`, `emoji.ts`, `build.py`, `born.py`, `bornstats.py`, `cmp.py`, `diffsim.py`, `dirs.py`).
- Data: `tx2-taxonomy-before.json`, `tx2-inv-before.json`, `tx2-inv-after.json`, `tx2-sim-before.json`, `tx2-sim-after.json`, `tx2-born-all.txt`, the directory lists `tx2-dirs*.txt`, `tx2-head*.txt` and `tx2-now*.txt`, and the logs `tx2-inv-before.log` and `tx2-inv-after.log`.
- To repeat: refresh the list with `dirs.py`, run `sim.ts` on the live taxonomy, then `born.py`. A new finding that fits an existing owner family needs only a pattern row. Run `build.py`-style rebuild from the live file and `validate.ts`, then install with a hash-guarded `mv`.
