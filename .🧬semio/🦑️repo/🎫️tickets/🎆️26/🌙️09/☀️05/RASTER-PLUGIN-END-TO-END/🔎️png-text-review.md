# PNG Text Cursor Review

The decoder owns source and metadata cursor together, with source and phase private. CRC validation precedes metadata birth, and the cursor borrows the same immutable source each turn. The cursor scans ordered text chunks, validates incremental UTF-8, checks zlib header and Adler, accounts allocation receipts including retained inflater history, and exposes last work and progress even after a failed inner step. This is source review, not an executed law.

The initial take_result permitted image handoff while text work was pending, changing the decoder phase and making all text operations fail the check thereafter. Controls reports adding a guard which returns pending until the metadata owner transfers its result, with a native assertion in progress. This correction was not independently compiled.

After companion relocation, retained-command feature plus unit is 250,391 bytes / 23 files / 2,431 active lines; companion is 11,913 / 5 / 83; both are within 262,144 / 32 / 4,000. Companion metadata laws refer to semio_os_kernel, semio_ui_locale, semio_framework and protocol alias without those direct dev-dependencies and alias in the observed manifest/root; controls was notified. The checkpoint law uses the publicly exposed checkpoint_cursor and both fixture paths resolved in source census. No native build was run.
