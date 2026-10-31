# Native Warm Reuse Next Acceptance

Provenance correction: native-receipt-audit.md compares a saved partial selected census, not an authoritative canonical Go input manifest. Its71drift facts do not establish exact ELF contributor coverage. Existing Receipt has no Source digest; full admitted runner/Go projection provenance remains the separate gap in native-source-witness-next.md. The physical warm reuse plan below does not close that gap.

Read-only current Go Source review, 2026-10-29. No Go, compiler, native command, container, lease or process operation ran. The complete build contributor receipt is stale as recorded in native-receipt-audit.md; do not query the old installed candidate to establish current Source acceptance.

## Current Reuse Law

Paths below are relative to 🧰️framework/🛍️products/🦑️repo/🔨️modules/.

| Current Source | Exact Law |
| --- | --- |
| 🌳️tree/📦️packages/🐹️go/🎊️query-cache/🐹️.go:60–134 | Composite identity includes HEAD, tracked porcelain, submodule identities and a lexical current authored-source byte digest. Source edits invalidate despite unchanged porcelain. |
| Same file:320–384 | Current projection digest is combined with composite identity. Under shared generation lease, warm index opens only with equal fingerprint/scope/includeSections and no pending .next/.previous generation. Open failure refuses rather than rebuilding corruption. |
| Same file:385–452 | After exclusive admission/recovery, identity is checked again. Only a mismatch builds staged index, streams documents, verifies it and commits a generation swap. |
| Same file:576–601 | Projection identity walks ordered tree documents, skips category documents, marshals the same treeSearchDocument used for indexing and hashes each JSON document with its byte length. |
| 🌳️tree/📦️packages/🐹️go/🔍️search/🐹️.go:245–263 | Indexed node and child Data text now call shared search.FieldText. Child sequence order remains part of the document. |
| 🔎️search/📦️packages/🐹️go/🏛️core/🐹️.go:788–824 | FieldText sorts map keys lexically, preserves sequence order and normalizes typed nil slices/maps/pointers to absent text, matching JSON null after snapshot decoding. |
| 🔎️search/📦️packages/🐹️go/🌲️snapshot/🐹️.go:69–126 | Codebase snapshot reuse requires matching fingerprint and shape; the domain projection remains fresh per request. |
| 🔎️search/📦️packages/🐹️go/🧵️shard/🐹️.go:390–466 | Warm open reads a closed bounded manifest, validates ordered unique shard identities, replays/checks each ordinary shard and verifies complete document totals. ShardCount=64 at line22 is a maximum; generic sparse indexes may use fewer shards. |

Duplicated client query-cache has the same combined projection identity and warm branch (client/⌨️cli/🎊️query-cache/🐹️.go:327–381, projection helper574); its indexed document builder also calls shared FieldText (client/⌨️cli/🔍️tree-search/🐹️.go:153/157).

## Known Historical Bug Versus Current Source

Historical f54d3e… completed cold97.631s/warm76.774s with identical JSON and physical codebase snapshot reuse, while rebuilding257913 indexed documents. The proven cause was unordered Go map iteration in indexed FieldText, followed by a typed-nil/JSON-null parity edge. Current Source includes both corrections. No remaining concrete occurrence of those known defects was found in the reviewed paths. This is not proof of actual current native reuse, nor a claim about every possible future arbitrary nested Data type.

Warm reuse still legitimately computes the full current authored-source fingerprint, projects current domain records, fingerprints documents and reads/verifies shard events. An expensive warm request alone does not establish a rebuild. Physical generation identity is the acceptance witness; absence/presence of throttled progress phases is supporting evidence only. Concurrent Source/domain changes may legitimately invalidate the index, so a Source-only freeze does not imply equal domain projection.

## Existing Focused Laws

- tree/🧪️tests/🔦️search/🐹️.go:1000 TestSearchProjectionStableAcrossMapReadsAndSnapshotRoundTrip uses neutral projectionText JSON, repeated map rendering, aliased sequences and typed nil slice/map/pointer before/after JSON roundtrip.
- Same file:1043 TestSearchIndexReusesEquivalentSnapshotProjection builds once, JSON-roundtrips the projection, then repeats eight requests, requiring no indexed event and identical meta.json file identity.
- Same file:521 and694 distinguish section choice and scope, so reuse cannot ignore those dimensions.
- search/🧪️tests/🧵️shards/🐹️.go:566/610/641/657/735/750 cover neutral snapshot lifecycle/Node gzip, cancellation/no publication, corrupt reuse refusal, cancelled lease wait, symlink ancestry and trailing metadata.

native-index-stability.md records owner-reported final765pass/97skip and Windows/macOS compilation plus9independent oracle tests/70assertions for the normalized candidate. These reported focused/unit/platform results do not supply the missing actual installed all-shard witness. native-final-validation.md describes earlier candidate layers; it must not override the newer installed/index history.

## Next Concrete Success Witness

After the authorized canonical rebuild/install, bind fresh binary digest/Receipt to the saved complete contributor manifest, require zero host/private-overlay drift before and after, and coordinate a quiet interval for both authored Source and relevant fresh domain records. Keep protected caches intact and record actual starting state; do not call a cache hit a newly generated cold index.

Use the existing native-layout-acceptance.cjs through the existing owned normal-admission container route only when Root authorizes it. It first proves positive resolving-file-identities SIGINT cancellation (130, no OS signal/timeout, empty stdout), with snapshot metadata/payload unchanged. Then require both complete search invocations to terminate0 within their explicit120000ms/16MiB limits, produce structured stdout, and return equal parsed JSON.

Capture after-cold and after-warm ordinary-file metadata for tree/snapshot/meta.json and payload.json.gz: same inode, bytes, mtimeNs and SHA256. Capture index.search/meta.json and manifest.json with same inode/bytes/mtimeNs/SHA256, plus every manifest-listed sNN/events.jsonl with same inode/bytes/mtimeNs. Preserve the exact ordered manifest identities and hashes. For this real monorepo all64 occupied shards must be accounted for if the captured generation has64; do not sample a subset. The generic helper currently enforces at most64, not exactly64, so record actual count and require its complete equality across observations. Its warm OpenSharded replay independently validates each listed shard hash/count.

Require same index generation fingerprint/manifest snapshot and no newly committed replacement. A warm indexed/committed event is a useful rebuild diagnostic but its absence alone is insufficient because progress is throttled. Record elapsed times without inferring failure solely from warm/cold ratio. Any changed contributor/domain projection, forced timeout, JSON mismatch, snapshot replacement or shard replacement leaves acceptance unproved; diagnose the exact mismatch before another authorized attempt. Scoped/tree/only-file and MCP controls remain separate required surfaces in native-rebuild-route.md.
