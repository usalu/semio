# Coordinator Atomic UI Handback Review

## Actual Runs Read

Four pre-repair held-mutex laws each executed0PASS/1FAIL with111skipped: document(.293s), ordinary value(.243s), strict unstarted(.230s), strict claimed(.222s). Logs are `🧪️member-ui-{document-drop,value-drop,value-unstarted-guard,value-claimed-guard}-red-r3-native-2026-08-27.txt`.

After the atomic repair, the coordinator read the actual focused log `🧪️member-ui-drop-handback-green-r4-native-2026-08-27.txt`: **7PASS,108skipped,.181s**, Nx success. These cover the four actual arena-contention regressions plus fixed-word boundary fairness, delayed ready-bit reuse, and racing producers.

The complete suite then executed **16PASS/1FAIL,98notrun,115discovered,.247s** in `🧪️member-ui-full-green-r7-native-2026-08-27.txt`. Its filename is not a pass claim. The failure is `instance_lifetime_ui_value_retirement_nested_shared_alias_keeps_external_payload`: actual(3items,10bytes), expected(2items,10bytes). No new full-suite pass is claimed.

## Source Review

Read the complete fixed handback module and tests, exact UiValue retirement, document owner/maintenance integration and builder/lease Drop. Producers record per-slot release counts or a returned-claim obligation with atomic operations, then set a fixed ready bit. They do not acquire an arena mutex or grow a queue. The consumer holds the sole arena guard, clears the ready bit before taking one obligation, restores readiness if more work exists, and re-records an obligation if its application rejects. The four64-bit words cover256 value slots; one word covers8 document slots. Normal exact/global retirement uses try_lock and preserves zero-grant behavior.

A delayed producer ready-bit can leave an empty marker, but cannot consume a reused owner because the marker does not carry a release obligation. Slot reuse remains prevented by admitted aliases/claims until their obligations are applied. The three tests exercise boundary/fairness, delayed-bit and concurrent producer behavior; they do not prove every possible platform schedule or8ms outer callback timing.

The full-suite mismatch follows a serializer oracle read immediately before exact external close. That read creates a cursor alias whose Drop is now deferred. This is a candidate explanation for the extra charged release, not permission to suppress actual work or blindly update expected counts. The owner lane is checking the exact alias count and a neutral queued-reader law; nonblocking handback and original strict guards must remain.

## Current Limits

No full UI115 pass, Plugin/native host/Wasm close pass, browser interactivity or hard latency certificate is inferred. Ordinary cold alias/read/builder entry points still use their existing arena access; this packet specifically replaces Drop handback and retirement waiting.

Read-only filesystem check now shows14GiB free on the926GiB data volume. No cleanup/deletion or broad native build was performed. Source-only work and small existing-target regression gates continue.

