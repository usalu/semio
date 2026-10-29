# taxaudit: taxonomy / artifact-name registry audit (PARTIAL, stopped at usage checkpoint 22:55)

Status: YELLOW. Static audit done, authoritative gate run started but not finished, NO fixes applied (no file changed).

## Method
- New (untracked) non-ticket paths today: 158 of 298 (`git status -z`, saved only in scratch). Novel = dir/file name not present anywhere in `git ls-files`.
- Gate: `SEMIO_LANGUAGE=en repo ./📜️script.ts verify taxonomy report` (needs the mode argument `report|enforce`; `--scopes-from <file>` restricts it). Started detached; log `C/🗑️generated/1028/taxaudit-verify2.log`, exit code lands in `taxaudit-verify2.rc`. At stop time it had finished inventory (355427 paths) and was in `plan/references 9356/155549`. Findings appear after that phase: `grep -v progress` the log, then fix per finding.

## Novel emoji directories (23): all carry U+FE0F like their siblings; none registered literally in `🔣️taxonomy.json`
Literal registration is only required where a `semanticDirectoryMemberKinds.<id>.memberNames` allowlist governs the parent (artifacts, options, fonts, ...). The mutation-facet parents below are governed by `mutationDirectoryPattern` (`^.+️[a-z][a-z0-9]*(?:-[a-z0-9]+)+$`), not by an allowlist, so "absent" is expected there; whether the gate wants more is decided by the verify run.

| New dir | VS16 | Literal in taxonomy | Location class | Pattern check (`mutationDirectoryPattern`) |
|---|---|---|---|---|
| 🧳️native-lifecycle, 🧳️native-singletons | yes | no | energy `🧬️mutations/` children | both match; NOTE same emoji `🧳️` on two siblings: `pathEmojiPolicy` says identity is a single-emoji grapheme in the sibling namespace of files and directories, check the gate result for a duplicate-identity finding |
| 🌋️ground, 🍰️fractions, 🏖️holidays, 🔀️nodes, 🔆️emission, 🔎️decisions, 🔭️visible, 🛠️properties, 🧭️design, 🧮️simulation | yes | no | energy/layout mutation sub-facets | hyphen-less single slugs: do NOT match `mutationDirectoryPattern` (needs at least one hyphen) unless they are below a mutation dir as organizational children; verify against the gate |
| 🧱️construction, 🧱️bulk, 🧾️services, 📥️pack, 📥️receive, 📝️dsl, 📏️store-bounds, 🌳️checkpoint-graph | yes | no | framework/os module dirs | module dirs, governed by the module taxonomy (no allowlist found); gate decides |
| 🧬️dsl-native-snapshot, 📜️definition-roster, 🔬️definition-roster | yes | no | test/fixture case dirs | `testCaseSlugPattern` governs |

## Novel leaf filenames (10)
| File | Verdict |
|---|---|
| `authority.json`, `payload.json` (os store replay tests) | emoji-less leaf names: suspicious; siblings use `🔣️.json` / `📐️.json`. Candidates to rename to taxonomy leaf names unless they are fixture payloads (`testFixtureSchemes`) |
| `behavior.test.ts`, `portable-oracles.{d.mts,json,mjs}` (layout `🧪️tests/🔬️typescript-package-commands/`) | ad-hoc names, not taxonomy leaves; candidates to rename (`🟦️.ts`, `🔣️.json`) |
| `checkpoint-order.rs` (store history replay), `tests.rs` (energy native-singletons outputs) | emoji-less Rust leaves: expected `🦀️.rs` or `🧪️tests` form |
| `👁️source.rs`, `🧪️cases.rs`, `🧰️context.rs` | emoji-prefixed with a stem: allowed only if `fileKinds`/`taxonomyLeafParentDirs` permit stemmed leaves in that dir; check gate |
| `schema.json`, `staged-form` (os flow fixtures, plugin declaration) | stemmed, emoji-less: check gate |

## Next step (exact)
1. `cd /Users/ueli/Documents/semio; G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1028"; cat $G/taxaudit-verify2.rc; grep -v 'progress\]' $G/taxaudit-verify2.log | grep -v '"stage"'`
2. Treat every finding as a rename or registry row; registry source of truth is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (`semanticDirectoryMemberKinds.*.memberNames`, `fileKinds`, `taxonomyLeafParentDirs`); regenerate generated registries with their generators, never by hand.
3. Re-run with `--scopes-from <file of new dirs>` for a fast confirmation.
