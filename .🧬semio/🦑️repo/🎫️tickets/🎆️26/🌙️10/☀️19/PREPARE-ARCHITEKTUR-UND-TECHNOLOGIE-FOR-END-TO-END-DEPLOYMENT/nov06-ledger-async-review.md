# Async Installed Ledger Review

Reviewed the current incremental ledger core, storage adapter, schema, and progress fixture and tests. No concrete defect was found in the inspected paths.

- The verifier serially awaits stat and any required relist. Its control checks surround each await and the post-await check runs outside the filesystem-error-to-mismatch catch. Consequently cancellation observed during an operation is thrown after it settles instead of being converted into a cache miss.
- A progress tick follows every 32 fully matched rows, including any required relist and digest; a check follows the tick. The schema specifies checkWork 1 and progressWork 32. Identity, device, full stamps, relist limit, digest comparisons and mismatch classifications remain present.
- The adapter uses promise-based no-follow lstat with bigint stamps and promise-based readdir with directory entries; ENOENT on stat maps to missing, while other failures reach the verifier.
- The focused fixture checks progress boundaries against SQLite. Tests cover delayed stat and relist cancellation, IO errors, and cancellation after progress yield.

The focused tests do not directly exercise the real adapter's ENOENT behavior or prove that the outer actual read custody remains held across pending IO. The outer call chain and old limits beyond the inspected code were not independently integration-tested. No commands or tests were run for this review; the implementation was still being edited concurrently.

## Immediate Integration Review

The core awaits both installed ports, the adapter awaits ledger verification, and the synchronization gate awaits the adapter. The production incremental path awaits verification, scan, and ledger sealing. Resume and donor callbacks return the verification promise to their consumers. No missing await was found in the inspected immediate files. The main test at lines 106 and 123 awaits the verifier; its counted listing wrapper forwards the adapter Promise. These two temporary-filesystem and Python-oracle ledger tests do not construct a forged Original. The Store writer test at line 249 and extracted production scan test at line 281 are separate custody-sensitive integration paths. Callback consumers beyond the specified immediate files were not independently audited.

The implementation owner subsequently reported that the actual Promise adapter directory, listing and ENOENT test passed in the focused suite (four tests, 24 assertions); managed custody and full integration remained pending at that point.
