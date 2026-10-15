# Bounded JSON and Baseline Successor Independent Review

Read-only Source review, 2026-10-15. No edits, tests, Originals, Native or SQLite work. Root supplied baseline BOTH92568/95593 and strict14003/94122; not independently rerun. Storage's remaining isolated funding proof is separate. No already-fixed separator/cycle/freeze defect is reissued.

## P2: Proxy objects still execute reflection traps

`🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/📥️json/🟦️.ts` 17 rejects unsupported ordinary prototypes/accessors, but a Proxy around a plain object passes the prototype test. Object.getPrototypeOf, Reflect.ownKeys and Object.getOwnPropertyDescriptor invoke its traps; proxied array length access can invoke get. Thus the promise “without executing accessors or serialization hooks” is true for ordinary own descriptors, but not hostile proxied input. Traps can execute arbitrary work before count/depth refusal and provide changing descriptor views. No Proxy case appears in the reviewed constructed corpus.

Refuse system-recognized Proxy objects before reflective admission (for example Node's built-in util.types.isProxy, with actual Bun/Node qualification), rather than claiming all object-like inputs are passive data. Do not resolve this by invoking toJSON or serializing the original. Sent the exact seam to Storage. No runtime exploit or termination was attempted.

## Ordinary bounded data admission is coherent

The JSON owner checks string type/UTF-8 wire length before JSON.parse (8–9), caps carried wire at262144 bytes, then iteratively captures a private tree. Schema defines depth32, nodes16384, object members4096, array items8192, key256UTF8, individual string65536UTF8 and aggregate/wire262144UTF8. String escaping is included in wire charge at14, including control characters; finite numbers only are admitted.

Iterative task traversal checks depth/count before descending, rejects active cycles while allowing bounded repeated shared JSON data, and creates independent immutable copies. Ordinary objects use null-prototype outputs; defineProperty rather than assignment treats `__proto__` as data. Own symbols, unsupported prototypes, undefined/functions/bigint/NaN/Infinity, sparse/extra-property arrays and own accessors refuse. Own toJSON function is rejected by value type and never called. Objects freeze only after children are processed; the old recursive Object.values deep freeze is removed from staged issuance.

Carried wire byte bounding precedes graph allocation via JSON.parse; deep JSON still parses within that finite byte envelope and is refused by iterative capture before recursive envelope-schema validation. `readScriptProcessEnvelope` now calls bounded capture before validation at process owner13; create captures policy then the complete envelope at28–29. This is serialized data acceptance, not a private birth grant. The separate WeakMap issuer still requires actual issued identities; no public register(structuralObject) was added by these readers.

## Baseline fixes verified in current Source

Baseline owner `🧰️framework/🔨️modules/🏃️process/🔒️leases/📦️namespace/📒️baseline/🟦️.ts` 26 now admits only empty root or finite ASCII nonempty segments, rejects repeated/trailing separators, dot/dot-dot, trailing dot, colon/drive syntax and Windows device names. Capture of actual binding namespaces also goes through that parser at69–71, so unrepresentable old namespaces refuse publication instead of becoming absent/new. The previous alias P1 is addressed.

At69, initialized real claim identity in the canonical claim directory reserves its target hash as unresolved `claim-target`, including a target file not yet materialized. At70, journal/wal/shm-only filenames reserve their base hash. At71, queue-only directories reserve the same hash. INSERT OR IGNORE ensures these reservations cannot overwrite a real initialized target; real SQLite binding publication at69 explicitly replaces a provisional reservation. Unknown/empty/foreign/oversize/unreadable targets remain unresolved rather than migrating to new slots. Final SQL binding count and unresolved count include auxiliary reservations at75, enforcing the global cap after census.

Old key admission preserves exact round-trippable strings within4096UTF16/12288UTF8 (25), while absent new-key classification enforces4096UTF8. Full known key equality remains mandatory, with mismatch unresolved. The constructor owns actual image/control, seals ingress before census, retains generation reader and drains private accepted operations before cleanup. No canonical global routing or durable reopen authority is inferred from receipt text.

Current seal provider refuses non-macOS explicitly. Its macOS image protocol declares case-sensitive HFS+, so this review does not manufacture a Windows case-alias runtime claim for an unavailable provider. Future platform integration must preserve canonical physical namespace equality, not assume this ASCII grammar alone proves it on all filesystems.

## Result and qualification

One new bounded-object Proxy admission gap; no new blocking baseline census defect found in this focused successor. Existing canonical integration, crash/reopen, funding/peak accounting and cancellation-safe unseal boundaries retain their separately recorded status. Supplied current gates do not qualify unimplemented canonical enrollment or unsupported platform execution.
