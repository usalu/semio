# Mounted Instance Source Boundary

Native SDK request 64669 was refused before compiler admission because the mounted instance crate exhausted its whole-crate source-byte budget. The composer-api source requested 16,515 bytes with 13,227 bytes remaining. Those numbers describe the remaining crate quota, not a prior size of the individual file.

A read-only census resolves the actual package manifest, uses the pinned Linux x86_64 compiler configuration, and compares source-reader byte totals with independent filesystem stats. Production, artifact-app-testing, and unit-test selections are reported separately. It does not increase compiler bounds or substitute for supported compiler admission.

The next change will follow the actual source structure. The generic source budget remains 262,144 bytes, 32 files and 4,000 active lines.

Before changes, the instance source closure was 267,172 bytes across seventeen files. The two generic fixed-registry close functions now live beside their existing ClosingOwner contract in instance-ladders, using the same real context, grants, removal rules and terminal witnesses. The instance imports those canonical functions; their bodies are unchanged apart from public visibility.

The instance root also groups repeated imports and removes obsolete migration notes attached to private imports. Existing ownership-field documentation and composer API documentation remain. No compiler budget was widened.

After changes, production and testing selections pass the same declared source bounds: instance-app 259,790 bytes across seventeen files; instance-ladders 171,312 bytes across ten files. Three changed Rust files pass parse-only checks. These are independent source/structure results, not a native compilation verdict.

Composer wrapper documentation was then shortened while preserving its custody/refusal contracts. The latest instance-app closure is 257,391 bytes, leaving 4,753 bytes below the same cap. The moved registry helper bodies were compared directly with their original source and differ only by public visibility. Supported native observer-law request 21579 is compiling; no test verdict has returned yet.
