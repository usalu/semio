# Taxonomy Sweep 3: folders born since 2026-10-15

Slug: TAXONOMY-SWEEP-3, prefix `tx3`. Executor: Sonnet. No file was edited: no taxonomy row, no folder rename, no reference update was needed.

## 1. Verdict

Zero born-since-10-15 directories have a `directory-kind-unresolved` or ambiguous finding, in the simulator and in a real inventory run. The unregistered folders that the wave 6/7 RESULT files list were registered in the meantime. Two mechanisms did it: the `memberSlugPattern` rows (`crate-module-children`, `members-of-tests`, `semio-tech-module-children`, which already cover kebab and PascalCase names) and the names that peer slices added to the head of `members-of-tests` and `crate-module-children` (TS-SPLITS-3 added 52 names). The least-specific correct change is therefore no change. I installed nothing, so the hash-guarded install step did not apply. The live `📚️library/🔣️taxonomy.json` stayed at SHA-256 `4bed0f2c…d5908` from my first copy to my last check, and it still validates.

## 2. Method

- Directory lists: `git ls-files --cached --others --exclude-standard` (existing directories only; 115885 at the first run, 115919 at the second) and `git ls-tree -r HEAD` for the HEAD directories.
- Born since 10-15 means `st_birthtime >= 2026-10-15` or absent from HEAD, restricted to non-dot paths (`.🧬semio` ticket folders are free-form and out of scope, as in sweeps 1 and 2). That gave 6204 directories at the first run and 6235 at the second, 2389 of them born on 10-16.
- Simulator `tx3-tools/sim.ts` (the sweep 2 tool): it calls the real `loadTaxonomy` and the real `matchDirectoryKind` with the same parent and ancestor-kind walk as the inventory. I ran it twice, on the snapshot of 14:52 and on the live file at about 15:10. Both runs gave the same result.
- Real inventory `tx3-tools/inv.ts` (`inventoryTaxonomy` over the whole repo, about 11 min under load 76) on the live taxonomy.
- Fidelity: the real run has 0 `directory-kind-unresolved` findings that the simulator lacks (real-not-sim = 0). The simulator overcounts by 26 non-dot directories, fixed-contract folders that the inventory accepts. That is the same effect as in sweep 2.

## 3. Counts

| Measure | Sweep 2 after | Sweep 3 before = after |
|---|---|---|
| Real inventory, `directory-kind-unresolved`, whole repo | 3642 | 3704 |
| Same, outside dot paths | 1141 | 1141 |
| Same, dot paths (ticket folders) | 2501 | 2563 |
| Real, born since 10-15 or not in HEAD (non-dot) | 0 | 0 |
| Real, `directory-kind-ambiguous` | 0 | 0 |
| Real, `fixed-directory-contract-ambiguous` | 0 | 0 |
| Simulator, unresolved over all directories | 7563 | 7627 (7625 at the first run) |
| Simulator, unresolved outside dot paths | 1182 | 1167 |
| Simulator, born since 10-15 (6235 directories) | 0 | 0 unresolved, 0 ambiguous |
| Simulator, born on 10-16 (2389 directories) | 0 | 0 |
| Kind ids changed for previously resolved directories | 0 | 0 (no edit made) |
| `validateTaxonomy` problems | 0 | 0 |
| `loadTaxonomy` | ok, 758 directory kinds | ok, 758 directory kinds |

The whole-repo growth from 3642 to 3704 is only dot-path ticket folders (+62, free-form peer ticket folders); the non-dot count is identical. Real violations that touch a born directory path itself: `projection-catalog-coverage` 1, `projection-authority-invalid` 1, `mutation-payload-schema-authority-invalid` 9. None of them is a directory-kind finding, so none belongs to this task. Other violation codes in the inventory (`path-too-long` 15738, `semantic-stem-unresolved` 25776, `taxonomy/kind-only-basename` 11386) are a different class and none of them is a finding on a born directory.

How the 6235 born directories resolve (kind id, simulator): `crate-module-children` 2113, `rust-language` 1044, `packages` 1042, `members-of-tests` 686, `tests` 391, `schema` 248, `members-of-members-of-elements` 104, `members-of-wgpu-target` 95, `members-of-members-of-modules` 75, `members-of-modules` 57, `fixtures` 38, others fewer.

## 4. The listed RESULT files

I took every backticked folder name from each named RESULT file and matched it against the basenames of the born directories. Every matching directory has a kind.

| RESULT file | Names listed | Names found as born directories | Directories matched | Unresolved |
|---|---|---|---|---|
| REPO-CRATE-ROOT-SPLITS | 60 | 42 | 44 | 0 |
| SCHEMA-VALUE-MATH-SPLITS | 33 | 25 | 1707 | 0 |
| TOOL-GRAPH-MACHINE-SPLITS | 50 | 38 | 54 | 0 |
| OS-RUN-VCS-IO-SPLITS | 88 | 34 | 42 | 0 |
| OS-TS-CYCLES | 50 | 18 | 61 | 0 |
| ROOT-SCRIPT-SPLIT | 21 | 6 | 12 | 0 |
| TS-SPLITS-3 | 72 | 54 | 59 | 0 |

The names not found are mostly file names, crate names and prose tokens, not folders. Spot checks with kind: `📚️library/🎛️commands` and `🎛️commands/🏭️generate` are `crate-module-children`; `📚️library/⚖️laws` (born 2026-09-15) and `⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape` are `crate-module-children`; `🧹️normalization/🔣️taxonomy/🏷️filenames` is `crate-module-children`.

## 5. Renames

None. `tx3-tools/emoji.ts` (`pathEmojiStatuteFindings` per directory) reports no finding for any of the 6235 born directories, and the real run has no `path-emoji-*` finding on them.

## 6. Open points (not changed)

- The 1141 non-dot unresolved directories are older debt (born before 10-15 and present in HEAD): `♻️mit-bestand` bericht, presentation and demonstrator folders about 1020, `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts` 51, `🧰️framework/🛍️products/🦑️repo/🔨️modules` 26, and a few others. None was born since 10-15.
- The 2563 dot-path unresolved folders are ticket folders and stay out of scope.
- A peer slice may add folders after this run. The tools repeat the check in about 20 s (simulator) once the directory list is refreshed; see section 8.

## 7. Commands run

- `git ls-files --cached --others --exclude-standard` and `git ls-tree -r --name-only HEAD`, then `tx3-tools/dirs.py`.
- `bun tx3-tools/sim.ts <taxonomy> <dirs> <out>`: snapshot run (115885 directories, 7625 unresolved, 0 ambiguous) and live run (115919 directories, 7627 unresolved, 0 ambiguous).
- `python3 tx3-tools/born.py` and an inline classification script: 0 born-or-new unresolved or ambiguous.
- `bun tx3-tools/inv.ts` once: `entries=280829 violations=63814`; `python3 tx3-tools/cmp.py`: real total 3704, non-dot 1141, born/new 0, real-not-sim 0.
- `bun tx3-tools/validate.ts` on the live taxonomy: `validateTaxonomy` problems=0, `loadTaxonomy` ok, 758 kinds.
- `bun tx3-tools/emoji.ts` over the born directories: no findings.

## 8. Artifacts (`🗑️generated/`, not deleted)

- Tools: `tx3-tools/` (copied from `tx2-tools/`; `born.py` and `cmp.py` point to `tx3-headdirs.txt`).
- Data: `tx3-taxonomy-before.json`, `tx3-taxonomy-live2.json`, `tx3-sim-before.json`, `tx3-sim-live2.json`, `tx3-inv-before.json`, `tx3-inv-before.log`, `tx3-born-all.txt` (6235 paths), the directory lists `tx3-dirs*.txt`, `tx3-headdirs.txt` and `tx3-files-*.txt`.
