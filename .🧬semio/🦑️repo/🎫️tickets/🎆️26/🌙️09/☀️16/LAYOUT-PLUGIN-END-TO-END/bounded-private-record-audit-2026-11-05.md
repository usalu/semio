# Bounded Private Record Audit — 2026-11-05

Read-only inspection; no source edits or gates.

## Commands Located

Exact declared positive witness retained in fleet-private-native-birth-2026-11-04.md: repo --tool=bazel run @semio-tech/repo-lib:test-private-native-birth. Its owner is testPrivateNativeBirthV1 in 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧫️private-workspace/🧪️tests/🚪️owner/🟦️.ts; its actual OS entry is 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧫️private-workspace/🧪️tests/👷️child/📜️script.ts. It opens the real products store and parent compares PID/deadline/capabilities/retired-root receipt. This remains required positive proof.

Portable funding source: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧫️private-workspace/🧪️tests/🟦️.ts; test name: private native funding follows portable cases and independent SQLite minima. The normal command reconstructed from source and saved PID85435 prefix is repo tools exec bun -- test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧫️private-workspace/🧪️tests/🟦️.ts (optional --test-name-pattern with that exact test name narrows it). This is not a recovered verbatim historical command.

Historical strict PID85436 snapshot records only truncated prefix: repo tools exec bun -- /Users/ueli/Documents/semio/node_modules/typescript/bin/tsc --noEmit --stric…. Logs private-native-phase-strict.log and private-native-funding-law.log contain capacity wait only. No complete private phase strict argv/input file was found in inspected ticket inputs; do not represent a reconstructed invocation as exact historical evidence. Existing final-bootstrap-strict-input.json belongs to other roots. Suggested concrete narrow strict invocation uses repo tools exec bun -- node_modules/typescript/bin/tsc with --noEmit --strict --noUncheckedIndexedAccess --skipLibCheck --resolveJsonModule --esModuleInterop --target ESNext --module preserve --moduleResolution bundler --allowImportingTsExtensions --types bun, with private helper, limits, portable tests, owner and child roots. Normal capacity admission remains mandatory.

## Current Bound Gap

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧫️private-workspace/🟦️.ts receivePrivateNativeTestProcessV1 reads metadata using readFileSync(path) before checking 4096 bytes; .owner is unbounded readFileSync(path,utf8). Neither call pins a regular single-link file. Root device/inode binding and pre-store admission remain present.

🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📄️retirement-file/🟦️.ts exports readNativeImageRetirementFileV1(path,maximumBytes), preserving bytes/stamp API. It already checks ancestor directories, regular file, no symlink, single link, pre-open size, no-follow descriptor where available, and stable descriptor/path stamp before/after. It still calls readFileSync(fd), so concurrent growth can allocate beyond maximumBytes before rejection.

## Smallest Concrete Change

Preserve readNativeImageRetirementFileV1 signature and checks. Validate maximumBytes is a nonnegative safe integer. Replace its whole descriptor read with a Buffer bounded to before.size (already <= maximumBytes), loop readSync with explicit offsets until expected bytes or EOF, then probe one extra byte in a separate one-byte buffer. Reject short read or nonzero extra probe; retain all final identity comparisons and finally close. This bounds returned/retained read storage to maximumBytes plus constant overhead and observes growth without an unbounded allocation. Use existing declared maximum limits; do not raise them.

Receiver should reuse that interface for metadata at protocol.maximumMetadataBytes and owner at a new protocol.maximumOwnerBytes schema constant (16 bytes comfortably bounds decimal supported OS PIDs). Preserve exact bytes comparison against String(process.ppid), metadata validation, and root identity binding before Source/store effects. Keep schema-first authoring and catalog hash updates for the modified private schema. This adds no API or runtime external dependency.

## Minimal Tests Before Change

Add reader-local 🧪️tests/🟦️.ts and language-neutral fixture JSON under its tests directory. Cases: zero/exact bound, bound+1, directory, symlink/hardlink where host supports them, invalid bounds, and unchanged bytes/stamp. Compare stable ordinary read results with independent Bun.file(path).arrayBuffer() after an independently checked stat bound; validate fixture shape with internal validator plus Ajv. Use a deterministic injected system-read test seam only if needed to force growth/short-read/extra-byte race without timing; keep the public API unchanged and avoid mutable global hooks. Tests must prove no whole readFileSync(fd) path remains, not merely assert final rejection.

Extend existing private neutral fixtures/tests with closed owner-byte limit protocol expectation and exact/oversized metadata/owner admission cases in a small shared record reader helper, then invoke that same helper from the receiver. Assert rejection occurs before body/store callback. Preserve existing funding SQLite minima and parent selection cases. Actual OS witness remains the final success proof; portable/type checks alone cannot establish it.
