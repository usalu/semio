# Retained Evidence Loss Audit

## Result

The Git index for the original `ASSETS-FIXTURES-SEPARATION` ticket contains 79 ticket files: 45 Markdown reports, 29 `📜️script.ts` inputs, and five JSON files. Comparing each indexed relative path with the live renamed ticket finds two absent indexed paths, but only one is a recovery-required retention loss:

| Missing retained file | Indexed bytes | Recovery significance |
| --- | ---: | --- |
| `📓️plugins-authored-file-manifest-2026-09-09.md` | 13,622,727 | This is the known oversized plugin path ledger. It exceeds the repository MCP's 5 MiB file limit and contributed 29,848 unique paths to the complete authored manifest. Its exact blob remains available in the Git index; the plugin recovery lane owns restoration. |
| `📋️plugin-followup-authored-paths-2026-09-09.json` | 8,398 | This is not a retention loss. It was deliberately moved into generated output and deleted before the first close under the generated-output cleanup requirement. Its 13 updated paths, 13 moves, and two retained records flatten to 41 unique paths, all retained inline in the `## Authored Paths` array of `📓️plugin-followup-fixture-fixes-2026-09-09.md`. It must not be restored. |

No other indexed artifact requires recovery. All 29 indexed scripts resolve in the live ticket. Of the 42 report inputs named by `📓️authored-manifest-2026-09-09.md`, 41 exist; the only absent input report is the known plugin ledger above.

## Closure artifacts outside the indexed baseline

The required `🧑‍💻coordination/📋️ticket-close/🔣️input.json` is also absent. This is the already-known 7,726,134-byte closure request containing 32,011 paths; the retained manifest records SHA-256 `217e1a6f7f87e324e63cc29f07a8068b2357355c163a4f258414085cf930085c`. It was created after the indexed baseline and exceeds 5 MiB, so it requires the protected close implementation or a lossless split whose individual files and containing subtrees remain below the cleanup limits.

`📓️ticket-lifecycle-completion-2026-09-09.md` is referenced by the completion report but is currently absent. The retained close script creates this report only after it verifies a closed ticket. Because the ticket is open during recovery and the report is not in the indexed baseline, this is expected lifecycle output rather than a recovery gap; it will be generated after the final successful close.

The only recovery actions are restoration of the oversized plugin Markdown ledger and regeneration of the full closure input under the protected close implementation.

## Framework lane

The framework evidence chain is intact. The indexed and live copies of `📓️framework-fixture-separation-2026-09-09.md`, `📓️framework-computed-reader-fixes-2026-09-09.md`, `📓️framework-store-corpus-followup-2026-09-09.md`, `📓️remaining-vector-fixture-audit-2026-09-09.md`, `📓️final-framework-fixture-audit-2026-09-09.md`, and `📓️final-layout-repair-followup-2026-09-09.md` all exist. Their exact authored-path arrays remain embedded in Markdown, including the final DSL boxed-fields move. The expected removal of `🗑️generated` does not remove those retained path ledgers or conclusions.

## Limits

This audit compares only the original ticket's indexed retained tree and explicit closure/report references. It intentionally excludes generated logs and does not infer an unreferenced, unindexed temporary file. No repository source file was changed.
