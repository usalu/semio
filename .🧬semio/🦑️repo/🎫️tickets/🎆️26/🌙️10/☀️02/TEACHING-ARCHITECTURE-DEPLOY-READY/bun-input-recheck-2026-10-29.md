# Bun Input Recheck — 2026-10-29

Normal preview68950 authoritatively exited1 before Nx/application startup. The first cause is Original bounded Bun input file required from dependency storage readJavascriptDependencyInputV1, called by recheckJavascriptDependencyInputsV1 after store input replay. Sync reports0 completed binding changes, no active Source directory/intent and no retained capsules. There is no live app/listener.

The recheck validates current existence/type/dev/ino/mode, then quotes its bounded read with the original captured byte length. A same-inode growing peer edit exceeds that quote and raises the generic initial-input bound error before the later byte comparison can issue the existing concurrent-edit classification. A type or inode change between the two lstats can hit the same generic reader guard. The old diagnostic has no path or stat metadata, so the exact file cannot be identified from this log; an actual peer edit is a reproducible causal possibility, not an established filename.

Planned correction: keep the original allocation/read bound and all physical identity checks, classify replay changes at the actual observed descriptor/stat boundary, and attach bounded content-free expected/actual metadata. Tests must exercise actual same-inode growth, shrink, replacement/type races, descriptor closure and the existing concurrent-edit classifier; initial policy violations remain hard refusals. No cap increase, stale-byte adoption, peer edit reversal or blind preview retry is authorized by this diagnosis.

## Final bounded correction and actual proof

The private reader now has an explicit capture/replay phase at its two actual callers. Replay requires the captured byte quote to remain exact before allocation and classifies observed growth/shrink, kind change, descriptor substitution and disappearance through the existing snapshot concurrent-edit protocol. It still refuses all such reads; it never adopts changed bytes or raises the original quote. Capture policy-bound failures remain hard, and native initial ENOENT/churn behavior is preserved. Unrelated IO errors retain their exact identity; there is no catch-all conversion.

The enumerable frozen diagnostic carries only a bounded Unicode-safe path, expected/current byte counts, current kind, original maximum and replay/capture reason. It contains no input contents. Existing8MiB file/64MiB aggregate ceilings and no-follow/stat/read/descriptor-close checks remain.

Actual baseline42098 failed on same-inode growth: expected concurrent bytes, received no classification. A preservation vector then exposed initial disappearance losing native churn; baseline54788 failed and the capture stat behavior was preserved. Final94567 authoritatively exited0:1 test,151 assertions across13 portable physical/fault vectors. Independent Ajv and SQLite classification agree. Tests verify no allocation for changed quotes, closed real descriptors, public exported replay behavior, empty inputs, same-size edits, races and unchanged EIO identity. This is Linux execution, not an invented Windows runtime result.

Exact command from workspace root, with SEMIO_LANGUAGE=en, SEMIO_TICKET_DIR set to this absolute ticket and SEMIO_TEST_ARTIFACT_DIR to its generated directory:

    repo tools exec --light bun -- test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/📥️recheck/🟦️.ts

Logs: generated/bun-input-recheck-baseline.log, generated/bun-input-capture-race-baseline.log and generated/bun-input-recheck-fixed.log. Independent read-only review found no blocking issue. The original68950 log still lacks the exact offending filename; the new diagnostic will identify a future refusal without guessing the earlier path.

## Source ledger

- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts)
- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json)
- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/📥️recheck/🟦️.ts](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/📥️recheck/🟦️.ts)
- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/📥️recheck/🔣️.json](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/📥️recheck/🔣️.json)
- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/📥️recheck/📋️schema.json](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/📥️recheck/📋️schema.json)

## Runtime custody

Normal full prototype preview75700 remains owned and live under the normal governor; no restart, app/listener or readiness claim. Parent semantic12990 subsequently exited1 before TSC on the separately owned missing-nx graph prerequisite. No additional semantic or browser job was launched here. The next test work is the separately requested genuine full fixed parent/disposable-child lifecycle.
