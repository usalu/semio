# Funded Retained Key Lookup — 2026-11-06

## Current Owner Update

The unrestricted callback was replaced with RetainedKeyedRow, whose owner must directly borrow one stored String field without allocation/traversal. All actual implementations are direct field borrows (String, media stream, GCP, camera, rig and asset entry). Row events separately fund projection and one byte scan (2 work units); alias/final events fund 1. Tests explicitly deny row-phase budget 1 and validate original source identity even after completion. No callback identity wrapper remains.

The key topic is now its genuine independent semio-framework-replication-retained-key crate; the previous retained package directly reexports that canonical owner. The extraction resolves a measured cfg(test) closure overflow without changing limits: parent 16 files / 245712 bytes; key owner 2 files / 10569 production bytes and 3 files / 19429 cfg(test) bytes. Correct actual key-owner native run 2574 is pending. Previous live parent run 43059 can only establish parent compilation/filtered results after the move, not execution of dependency-owned key tests. The earlier wrong-facade run 73059 was explicitly cancelled before body and is no pass.

Historical implementation notes below are superseded where they mention a runtime function projection or one work unit for a complete row event.

## Implementation

Updated the existing first-party `RetainedKeyLookup` rather than adding another product lookup implementation. Original row/query bindings are checked on every quote and turn, including completed queries. The key projection function is captured before scanning. Each funded scan compares at most one UTF-8 byte; equal-length keys are fully checked, uniqueness scans continue until the second match or the end, and no row or key value is cloned.

The current API includes `maximum_work_units`, `advance_work_demand`, actual work outcomes on advance, and delegated original-owner close work/outcomes. Zero work or denied physical grants leave state untouched. Metadata and byte-copy receipts are bounded by the pre-turn quote. Native tests observe heap births/releases and cover source/projection substitution before and after completion, all fixture outcomes, and cancellation at every successful frontier.

Independent read-only census found no production callers; declarations/reexports remain the same and the only direct native callers were the owner's old tests, updated together. No compatibility wrapper or old signature remains.

## Evidence

- Existing neutral schema/fixture extended to nine cases, including three duplicates, Unicode-distinct text, empty duplicates and a long common prefix.
- Normal repository Bun execution of ticket `🧪️retained-key-oracle.mjs`: exit 0, Ajv validation and existing third-party `lodash.transform` match all nine expected first-two indices.
- Normal repository rustfmt execution: exit 0; this is syntax/format evidence only.
- Initial native submission selected the replication facade package, which cannot run dependency-owned tests. Only that own handle 73059 was cancelled (terminal exit 1 before body). Public capacity/lease snapshots then showed no facade grant or matching holder/waiter. The corrected owner test is live as 43059 (`semio-framework-replication-retained`), with no native body or compiler verdict yet.
- No cold collection lookup or changed authority/quota was introduced. The product's complete retained diff and mutation owners are still unfinished.

## Files

Core `📡️replication/🎮️mutation/🧳️retained/🔎️key`: owner Rust, native law Rust, neutral fixture JSON and schema JSON. Ticket oracle script is preserved; generated logs remain until successful ticket completion.

