# CPython 3.14.4 reported as not provisioned

Cause: `.cache/b/t/m/python/interpreters/cpython-3.14.4-macos-aarch64-none` was a legacy (uv-era) installation without `.toolchain.json`. `repo tools setup` located interpreters by `bin/python3` alone and said "already matches its pin", while `repo dependencies ensure` (`prepareArchiveV1`) requires the receipt and raised `missing` for an existing directory without one. The directory had never had a receipt (mtime 13 Nov); the 3.12 directory got one on 14 Nov.

Fix:
- `tool-setup/🟦️.ts` `pythonInterpreter`: a managed interpreter counts only with its `.toolchain.json` (macOS, Linux, Windows `python.exe`), so setup and lookup agree.
- `tools/🧰️native/🟦️.ts` `prepareArchiveV1`: an existing directory without a receipt is not ours; it is re-provisioned from the checksum-pinned archive and swapped in by rename (the old one goes to the temporary directory and is removed).
- Tests: tool-setup test for the receipt-less interpreter, native test for replacing an unreceipted installation.

Result: `repo dependencies ensure` completes; 3.14.4 now has a receipt.
