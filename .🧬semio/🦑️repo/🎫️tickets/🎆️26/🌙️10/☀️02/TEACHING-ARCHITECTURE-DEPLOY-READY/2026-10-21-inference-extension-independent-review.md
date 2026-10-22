# Inference Extension Independent Review

Current bounded review finds two integration blockers; Root notified before resealing. No Source or Native execution.

1. The actual exported verifyInferencePublicationV1 now consumes a third ResourceLeaseControl for its real captured child, but main 20973/d2245ecb still calls it with only publication and root. It must receive authentic current owned control at this call.
2. The extracted current writeClosureReadRefusalV1 uses writeSync(2,bytes,offset,length). The test extraction omits writeSync and injects process.stderr.write instead. Its guarded catch swallows the resulting ReferenceError, leaving packets empty; the new positive transport assertion must fail. Supply the exact FD write port and test partial writes/failure through that interface rather than an unused process port.

Closed PayloadRefusalFixture validates under strict Ajv: eleven decoder mutations and seven compressed-role rows. Current closed metadata specifies seven allowed compressed roles, bounded path, packed/expanded byte bounds, exact check enumeration and maximumOutputBytes; bounded transport is 1024 bytes including newline. Metadata is best effort and the actual owner catches annotation/output faults while retaining the primary. The new compressed tests require exact real ERR_BUFFER_TOO_LARGE identity and context, so unrelated generic throws cannot pass those checks.

There are now two selected literal inference tests, both explicit60000ms:

- `inference closure refusal context is closed and preserves the exact primary value`
- `all compressed closure roles retain real inflation refusal and bounded transport context`

Existing phase120000 now equals both ceilings with no margin. Preserve the existing phase convention by admitting the second title and increasing its enclosure to180000 if maintaining60000 margin. The real child in verifyInferencePublicationV1 is under its genuine existing main inference scenario; it has a requested10000ms child control, not a new independent test title.

New direct imports materially used by this extension: actual process capture facade `@semio-tech/process/execution`, requested child-control facade `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`, schema validator facade `🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts`; lease facade is type-only. Compare these against final fresh roster rather than claiming an absent path from an old seal. Existing actual capture implementation/control issuer may already be roster witnesses.

## Observed Pins

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📥️inference/🟦️.ts`: 11596 / `c2433bff8de9604724da269a0aad3a4da64aa86deb75e89c11d7c8097964439e`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📥️inference/🔣️.json`: 2928 / `8145906f2503220d4b1dd46808bfc276f8afa37b3d1499d0ea096ae53b8bd6de`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧬️schema/🔣️.json`: 21057 / `235b02ee5eeae6a5eb6c4f9c30fff72cf256c1f0af592838e1925a0c1d13133d`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/👷️read.mjs`: 17364 / `9ccff028ebe0ea47b945d0bff46c3028bbdf2fac2ff8fb0e1f7e3cc56d555f9d`.

Current main d224 integration remains as described. Final clearance requires repaired call/port and fresh pins.
## Concurrent Repair Observation

The initial semantic review above used test11361/252992bcc55dd67633acc980a41992f8dfba5267a952b26f59c177c7366a0ed5 and schema21057/84ce052153b1d35a8de976f925fae10e05d7fff1130b76b811ac47d1d5b2f309. The listed observed pins are later concurrent reads, not attribution of those initial defects to unchanged bytes. Test11596/c2433bff now correctly supplies filesystem.writeSync and exact partial-write/failure ports. However it introduces nodes.join("\\n"), whose AST string value is literal backslash+n rather than a newline; the generated Function body cannot parse. Restore newline separation. Main was reread and still calls with two arguments. Final current closure schema changed concurrently and requires held-pin validation.
