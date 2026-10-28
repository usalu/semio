# Ticket File Inventory Audit

Reconciled files.md with files-reuse.md, the 20-path complete scale footprint, ventilation.md, architecture.md, caption-audit.md, transform.md and scene-integration.md. Used bounded named-file reads and immediate ticket-root listing; no Git, IDE or workspace traversal.

## Result

- files.md contains 115 unique absolute C:/git/semio paths: 67 application/source/test/schema/documentation files and 48 retained ticket files.
- Missing listed application or input files: 0.
- Maximum absolute path length: 114 UTF-16 code units; all paths are at most 256.
- Explicit portable caption cases, integration tests (including native font-map allocation and failure restoration), scene cases/tests, transform cases/tests, base visual cases/schema/tests, runtime cases/schema/tests and authoring test script are included.
- Source-copy check compared ticket-root executable inputs with the enumerated application files by SHA-256: 0 byte-identical copies. Ticket root contains no copied canonical emoji-named source files or source subtrees; its only directories are important and generated. The existing important directory contains an empty metadata Markdown file.
- The unwanted authoring project 📦️packages/🐍️python/.venv is absent.
- No removed source paths were reported in the reconciled inventories: seven obsolete scene definitions were removed within listed existing source files.

## Pending Owner Cleanup

caption-validated.json is a tool-generated success cache written by caption-regression.py at the ticket root. It is excluded from the retained close inventory; the owner should remove it when final verification no longer needs it. caption-repairs.json is retained authored edit input. No generated directory or pre-existing file was deleted during this audit because rendering and review are active.

This is an inventory reconciliation of reported ownership, not a claim to inspect unrelated concurrent workspace changes. files.md is the refreshed final close list. The briefly created ticketfiles.md was removed immediately after the coordinator clarified the inventory filename.
