# Native Regression Coverage

Read-only review found a concrete omission: the declared repo-graph-maintenance:test dispatcher and runtimeSourceEntries listed storage tests, optional-ledger tests and graph-headroom tests, but none imported the closure-root suite. A bounded static import inventory of those three roots reached15 test modules, including the9-source/19-state custody regression, but omitted generation42, batch13 and member-admission13 cases. Broad freshness inputs do not execute omitted test files.

After coordinator authorization, the existing MaintenanceVerification schema and neutral fixture were changed first from three to four roots. The existing AST test executes the actual TestScript class with a controlled runner, compares the dispatched paths with the actual manifest/neutral fixture, uses Ajv and SQLite for execution/input-role parity, and confirms every dispatched file exists. Production then appended the exact existing closure-root test file as fourth Bun argument and fourth runtimeSourceEntry. No new target or extra Native invocation is needed. The existing60000ms command budget, Source/freshness metadata, Original signal and all other fields remain unchanged.

The same pending repo run repo-graph-maintenance:test now statically registers25 test modules. Actual chains:

- generation (42 neutral cases/states): 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/📜️script.ts → 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🟦️.ts → 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📇️formats/🟦️.ts → 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🕰️generation/🟦️.ts
- batch (13 neutral cases/states): 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/📜️script.ts → 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🟦️.ts → 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/♻️groups.ts → 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔢️batch/🟦️.ts
- member-admission (13 neutral cases/states): 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/📜️script.ts → 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🟦️.ts → 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📇️formats/🟦️.ts
- source-custody (19 neutral cases/states): 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🟦️.ts → 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🔏️closure/🟦️.ts

The closure root also imports the15 receipt-format and8 coordinate cases. This is source coverage evidence only: no test assertion was executed and no pass is claimed. The focused Source run remains separate and the application unit/catalog/canonical chain is held until actual snapshot/backend admission.

Changed paths:

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/📋️project.json — SHA256 2ceb3728f72785108b90e66efe87a0cad9843f831be250005e61f709459413cd; 1876 bytes; 2026-10-21T14:34:55.230Z
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/📜️script.ts — SHA256 0df238c0552f5f3c7b5e0e943fb7d1749b992bfff19442ff2f06523e06be35c2; 1480 bytes; 2026-10-21T14:34:55.245Z
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json — SHA256 0d2811f1018518cd965bbfd80180e309997059aee2c73038e41f925c57f6479b; 38322 bytes; 2026-10-21T14:34:35.350Z
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🔣️.json — SHA256 41401c345ff98e265eb02fbc441df8fe415bafad76334a9271c4436b46a4112b; 45115 bytes; 2026-10-21T14:34:35.351Z

Generated boundary and coverage arrays preserve exact file identities/import paths for later verification.
