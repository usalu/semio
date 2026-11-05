# Verification Walk Audit

Read-only source audit, 2026-11-05. No builds, tests, process inspection, cancellation, source edits, or dependency changes were performed.

## Finding

98304 is the 24th progress checkpoint (24 × 4096), not a verifier fuel ceiling. The verifier initializes work=0 and increments it for bindings, ready receipts, input sources, manifests, membership rows (including vouched/skipped rows), then provider observations. It publishes only every 4096 increments. It is an aggregate work counter, not a byte count, completion percentage, or fixed stage boundary.

Evidence: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🟦️.ts:184-186,198-239; 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/💾️storage/🟦️.ts:74-76. Capture also uses the same published resource and its own work=0 counter (generation module lines 132-154), so the progress label alone does not distinguish capture from replay verification.

The actual schema declares checkWork=256, progressWork=4096, maximumRows=524288, maximumEvidenceBytes=67108864, maximumRelist=4096 and maximumScan=65536 (🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧬️schema/🔣️.json:15-24). No 98304/96×1024 limit appears in these defining verifier/schema files. A 98304 match elsewhere includes output capture stdoutBytes in the MCP build-surface asset; it is unrelated to generation work.

## Can Work Stay CPU Active Without A New Checkpoint?

Yes, the current progress number can remain unchanged during synchronous work between increments: evidence hash/decode/JSON parse (generation module lines 114-121,219,238), project listing and hashing (234-236), recursive scan (222-225), or supplied vouch/ready/control continuation operations. scan has a global 65536 cap and returns conservative true on exceeding it; relist cap returns mismatch. Neither exhaustion loops or resets work in this function. The main verifier loops traverse finite arrays and return match or mismatch; its catch returns a typed mismatch except cancellation (248-250). No exhausted-fuel busy loop is proven here. CPU liveness plus identical last publication cannot identify the live instruction or prove an infinite loop.

There is an observability gap: synchronous scan/decode/project work does not publish its internal work or check cancellation per row. A prolonged interval can therefore retain the last checkpoint even while CPU active. This is a source-level possibility, not proof that the foreign processes are there.

## Source Changes And Restart

Current generation receipts are deliberately keyed by walk protocol, generation data, runtime and configuration, not implementation source bytes (schema description line 5 and protocol tier lines 90-95). Current tests explicitly forbid captureBuildSourceClosureV1/implementation-file keying in generation verifier/storage (generation tests lines 327-338). The README description of implementation closure invalidation is stale relative to this defining code. Editing an arbitrary verifier source file therefore does not itself invalidate a dependency receipt. Changing walkStamp does (verifier line 189); changing recorded input/manifest/topology/provider observations returns the corresponding mismatch, causing the owner to perform its full walk.

The retained loader is defined by 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts:73-81, delegating to 🍞️bun/🟦️.ts. It executes captured immutable module images; its entry proof recheck iterates images (Bun loader line 123), while captured file reads select retained images (115-116). Live source edits are not automatically hot-loaded into an already running retained child. Captured command authentication additionally verifies loader identity (closure/🚪️command/🟦️.ts:17). No process eval body or environment was printed.

Format-stamp changes are a separate mechanism: registry compares declared loaded stamps with live schema values (🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs:75-88,125-133). Its exclusive admission guard checks bound stamps; stale errors or holder yield requests lead to abort/restart exit 75 (118-122,134-136). The restart schema caps reexecutions at 6 (format-stamps/🧬️schema/🔣️.json:73). Sealed trampoline considers whether this start is a restart when selecting snapshot refresh/promotion (🏛️sealed/🚦️trampoline/🟦️.ts:18-22,47). Thus source change, generation invalidation and outer restart are distinct events; a plain source save cannot be assumed to wake existing shared verification holders.

## Conclusion And Next Check

High confidence on checkpoint meaning, bounded verifier structure and absence of a 98304 fuel ceiling in the owner. Medium confidence on retained-source/restart implications because no live child snapshot was read. No concrete infinite-loop bug is proven. Smallest useful next read-only check: inspect receipt metadata counts (bindings, ready, input sources, manifest texts, frontier.rows, execution.rows), without dumping receipt contents or credentials, to map 98304 to its possible tier and determine total expected counted work. If counts place the boundary before execution decode, inspect that exact content-addressed evidence size/row metadata next. A code change or budget increase is not justified by the checkpoint alone.
