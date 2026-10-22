# Owned Channel Input Bound Causal Audit

Frozen fourteen-law Native log42739/31a4b46cb934d542ad37d4495d860b33cdac1f361fd301c1c982f47f3239a9ad reports the text-specific own-data UTF-8 guard, then channel writer, session and capture body. Current Source is a point-in-time contract audit, not proof of the exact historical loaded bodies.

The current public API is writeOwnedCommandChannelV1(channel,input), with closed input {version:1,text:string} or {version:1,base64:string}. Current session passes correctly positioned {version:1,text:captured.bootstrap}; later credit/request frames use the same DTO. Thus current argument positions and shape are compatible. The observed specific error is downstream of own-object, closed-key and primitive-field checks; it is not evidence of a signature mismatch.

Input Policy permits Text at most65536 UTF-8 bytes and units, with exact UTF-8 roundtrip; Binary permits4194304 decoded bytes and canonical base64. The captured Node builder constructs JSON.stringify(snapshot)+'\n' and admits up to16777216 bytes. For valid snapshot DATA that construction is UTF-8 roundtrippable, but its admitted size is not bounded to65536. This is a concrete producer/consumer bound mismatch. Frozen log does not disclose actual bootstrap length, so it alone cannot prove the failed numeric subcondition or historical payload content.

Minimal repair preserves the input guard: deliver immutable bootstrap bytes as ordered finite canonical Binary DTO chunks within4MiB per frame, enforce existing channel aggregate bytes/frame limits, preserve exact concatenated digest/newline and await actual readiness before credit/request traffic. Alternatively explicit rejection at capture construction is safe but would refuse legitimate existing larger snapshots. Do not widen generic Text limits or pass mutable buffers/controls as DTO authority. Binary splitting avoids dividing UTF-16 surrogate pairs. Node loader accepts stream chunks until final newline; no bytes after that newline may precede readiness.

Finite regression rows should independently compare original bytes/digest with reconstructed ordered chunks for empty/small,65536,65537,multibyteboundary,4MiB,4MiB+1,16MiB boundary and over-cap; enforce canonical base64/decoded per-frame bounds, aggregate frame/byte exhaustion, final newline exactly once, cancellation/first failure during intermediate send, and no credit before readiness. Existing own-object/accessor/proxy/extra-field/invalid UTF8 guards must remain refusing. Expected bytes/base64/hash can be computed independently through Python or third-party codec and SQLite lengths/order; the actual future Native test should exercise real channel+captured loader rather than only a duplicated splitter.

Current pins:

- 🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/📥️input/🟦️.ts — 1944/d8f8975217d8211f4a9d9a675812c2aa64a7491b734d8b0f59709f1f57bf6dbc
- 🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/📥️input/🧬️schema/🔣️.json — 1085/25b51ff632f110a78626c329800896226a48c0f4baea2fbb47580b4af28cab83
- 🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts — 21527/04fcb49dcb6bc4b5b8869df9f450f2faae19e9fd1a420e05d9d3174ccc68abd8
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟦️.ts — 9548/3f3a3cead092b02484e6e2e6f01b61de1d531e91123e80e01521bad6da496159
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🟨️.mjs — 9015/b0653019a93e3e6188f977639edf6a0cf1f33c1d5a2dc281060ec82d32e78dbc
- 🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts — 1981/c96b98b763e4c440bfd6d85868734011650d0d0547c647335ed6f2aff35ebcde

No Source evaluation, commands, tests or edits were performed.
