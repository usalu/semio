# Ticket Document Production Pilot

## Committed Result

The exact owner `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️01/☀️29/FIX-CODEBASE-SECTIONS-AND-TICKETS` is normalized:

- `ticket.md` moved to the same owner's `📝️.md`.
- Its closed-ticket, zero-byte `📌️important.md` marker was removed under the manifest-status contract. No content was discarded; the retained plan records its empty SHA-256, mode, and source identity, so the marker can be recreated if required.
- `🎫️ticket.json` was unchanged.

The production CLI committed attempt `000001`, journal revision 13. No staging or backup directories remain. Its exact approved source-parent pruning set is empty.

## Frozen Authority and Verification

Baseline: `9f449b10659b95148c8bcb3f91ce583bf7446973`.

Plan: `🧾️ticket-document-production/🔣️.json`, digest `22acd5429721c8f785fee55069c90cd9d78d6bd9274641c6d1260b08360ff7a0`. It contains one move, one empty-marker removal, no other operations, and no unresolved findings. The fresh production plan matches the earlier read-only plan byte identity.

The destination document has SHA-256 `2d51ef0ee0595c01edcd5790227acdbe9193c26cf990df188b26a795df952964`, 318 bytes, mode `0644`, inode `70408713`. These content, size, and mode values equal the frozen source preimage.

The unchanged manifest has SHA-256 `a80f5b76525ecec01c1038e01885de43785f74154e5e051346eef5e7b72752b7`, 675 bytes, mode `0644`.

Journal: `🧾️taxonomy-transaction/🔖️22acd5429721c8f785fee55069c90cd9d78d6bd9274641c6d1260b08360ff7a0/🔂️attempts/🔢️000001/🔣️.json`.

The identical owner scope was inventoried and replanned after commit: zero moves, roots, relocations, symlink edits, removals, reference edits, regenerations, or unresolved findings. Retained replan: `🧾️ticket-document-production/🔁️replan/🔣️.json`, digest `cfeaeb59f4ab5700ef5a664bb8bdc238c253f594ea49446a31d4349aa6b37af4`.

## Test and Invocation Accounting

Immediately before applying, the document-owner and journal-bound pruning packets passed together: 10 tests, zero failures, 154 assertions, 53.32 seconds. Scoped walker tests passed separately: 3 tests, zero failures, 12 assertions. Existing full/scoped census and deterministic ordering regressions passed: 4 tests, zero failures, 10 assertions, 19.40 seconds.

One attempted apply invocation incorrectly included `--workers`; CLI validation rejected that flag before mutation. The corrected apply invocation omitted it and committed successfully. No real Git state was changed. Actual Compose trees were not used or restored.

This is a bounded document-owner pilot, not full repository convergence. The remaining ticket census, package migrations, generated consumers, all-repository zero-plan gate, and complete timing gate remain active work.
