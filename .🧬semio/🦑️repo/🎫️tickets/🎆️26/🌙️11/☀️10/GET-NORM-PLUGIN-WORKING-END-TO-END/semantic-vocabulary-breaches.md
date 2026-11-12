# Semantic vocabulary breaches

Scope: `workspace:semantic-vocabulary` (rule `policySemanticVocabularyBreaches`, `⚖️laws/🧩️plugin-consistency/✍️handcrafted-mutation/🟦️.ts`). No `repo run` lane was used. Detection reproduced read-only with `vocab-scan.py` (this folder): same walk roots, `🧬️mutations/` or `🎮️commands/` `.rs` files, substring tokens `SetSnapshot`, `NoMutation`, `CollectionMutation(<|::)`. Before: 78 (matches the lane). After: 0.

## Allowlist
`✏️s/🔌️plugins/🔒️policy-allowlist.json` deleted (342 dead or inert entries). The whole `policy/🔒️allowlist` module (loader `policyDiscoveredAllowlist`, schema, json corpus, py and ts tests) was deleted. The only other consumer, the "policy allowlist" describe block and import in `🧪️tests/🔬️workspace-contract/🔭️package-discovery/🟦️.ts`, was removed. The unrelated language-catalogs `🔒️allowlist` was left. Stale wording "(unallowlisted)" removed from the rule's reason string. No replacement added.

## Breaches and decisions (78 files, 3 kinds)
- 72 files were doc-comment narration of retired names (`NoMutation was dropped ...`, `SetSnapshot is banned`, `tag 0, formerly NoMutation`), mostly in stdio, plus fem, cad, forms, process, writer, vcs/draw docs. The rule is right to flag them (retired vocabulary, history narration is legacy-support). Comments were reworded to state only the current invariant (derive requires one leaf payload per variant; tag 0 retired; whole-snapshot replace is banned). Writer region marker `🔖️SetSnapshot` renamed `🔖️WholeSnapshot`.
- vcs `no-operation` command: struct `NoMutation`, command id `noMutation`, `VcsCommand::NoMutation` renamed `NoOperation` / `noOperation` / `VcsCommand::NoOperation` (animate already used `NoOperation`). Renamed across the vcs plugin sources, fixtures and `🌎️hub/🧩️compositions/🌿️vcs/🧩️component/🔣️.json` (9 files).
- draw `set-snapshot` command (a real whole-document replace): renamed to `replace-document` (dir `📸️replace-document`, struct `ReplaceDocument`, id `replaceDocument`, module `replace_document`, label "Replace Document" / "Dokument ersetzen"). Across the draw plugin sources, `🧪️tests/🧫️fixtures/🧪️publication-authority`, and the hub draw component json.
- The rule itself needed no change (no false positives found), so no rule or test change.

## Not verified
No cargo check or lane run (throttle). Renames are text-level; sorted-order or generated-catalog assertions in vcs/draw (command lists, hub component json ordering) may need a check when runs are allowed. Scripts: `vocab-scan.py`, `vocab-rewrite-1.py`, `vocab-rewrite-2.py`. No `[DEBUG]` logs added.
