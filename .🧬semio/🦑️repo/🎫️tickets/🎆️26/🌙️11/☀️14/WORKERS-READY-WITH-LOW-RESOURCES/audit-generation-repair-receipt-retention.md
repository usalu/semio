# Generation Repair Receipt Retention

The generation retirement pass now removes exact repair receipt files and atomic-write temporaries after the ordinary generation grace period. It retains the quarantine's exact receipt or, for a sealed pointer, its next staged receipt. Retirement checks the expected pointer again immediately before each removal and uses the existing builder's structural custody fence when held. Fresh and foreign names are skipped. The file port refuses non-regular files and hard-linked aliases on these removals, and checks cancellation after its final stat.

The graph closure budget proof enumerates matching receipt paths as control metadata. It requires a bounded same-image regular file with one link before excluding it from closure membership and reclamation. The exact directory and filename grammar is shared with receipt retirement. Oversized, aliased, and directory-shaped receipts fail the proof. Receipt bytes still occupy the original enforcing graph image and count toward its physical usage.

Before the change the retirement fixture retained four aged inactive records and their temporary. The SQLite oracle now verifies that aged inactive files are removed, with the live or staged receipt, younger files, unrelated names, and hardlink aliases retained. The same fixture verifies no receipt removal if the pointer moves or cancellation arrives during enumeration. The budget fixture independently exercises the native census path classification and alias and size checks.

The generation retirement, generation file port, repair receipt, and budget census suites passed 16 tests and 939 assertions. The generation engine remained below the 65,536-byte Source admission ceiling. Only private fixtures and workspace source files were changed.

- [Generation retirement](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs)
- [Receipt retirement helper](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🩹️closure-repair/🟨️.mjs)
- [Bounded file operations](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🗃️file-ports/🟨️.mjs)
- [Native closure budget](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts)
- [Retirement regression](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🩹️closure-repair/🧪️tests/🟦️.ts)
- [Physical metadata regression](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🌀️row-log-spill/🟦️.ts)
