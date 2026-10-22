# Current Gzip Snapshot Acceptance Matrix

Read-only complementary fixture proposal for High existing formats home; no duplicate implementation. Actual producer v2manifest/gzip packed-hash format is the authority. Positive matrix preserves empty graph, optional absent/empty external data, actual project/dependency/external records, legitimate graph remainder and duplicate content. Negative matrix distinguishes filename authentication, decompression/JSON, mandatory links, container role, unknown outer fields and actual resource bounds.

- empty-graph — accept: v2 manifest with empty projects/dependencies, absent externalNodes and zero links
- explicit-empty-external — accept: externalNodes empty array stays distinct from absent but has zero records
- project-record — accept: real producer project object gzip with Source reference in value and key
- dependency-record — accept: real producer dependency array gzip with retained Source/lock references
- external-record — accept: real producer external map gzip and exact optional manifest links
- graph-remainder — accept: actual producer graph remainder retains legitimate metadata/references without inventing extra root restrictions
- shared-record-digest — accept: duplicate referenced compressed content deduplicates physical links and unions references
- authenticated-unreferenced-record — accept: known exact record namespace authenticates and conservatively scans even if no manifest points to it
- record-packed-sha — refuse: compressed bytes differ from filename digest
- manifest-sha — refuse: manifest UTF8 bytes differ from filename digest
- malformed-gzip — refuse: name hash matches packed bytes but gzip stream invalid
- malformed-json — refuse: valid gzip with hash-correct invalid JSON or invalid UTF8
- missing-linked-record — refuse: manifest required record coordinate absent from authenticated inventory
- wrong-manifest-link — refuse: hex digest points to wrong or corrupted inventory member
- invalid-link-coordinate — refuse: nonhex/traversal/non64 digest rejected before deriving physical coordinate
- wrong-record-container — refuse: manifest expected project/external object or dependency array conflicts with payload; apply actual producer contract
- manifest-extra-field — refuse: unknown outer manifest member refused; graph remainder follows actual schema
- duplicate-project-name — refuse: ambiguous project-name→record binding refused before proof acceptance
- unknown-gzip-name — refuse: gzip outside exact current snapshot namespace is not admitted
- expanded-boundary — accept: exact remaining/record expanded bound accepted without free reread
- expanded-one-over — refuse: producer-sized record or shared proof budget exceeded refuses before unsafe allocation
- readset-race — refuse: changed manifest/record identity during proof or before unlink refuses; first primary/counters retained

Independent expected output: hash actual packed bytes with Python hashlib; decode using Python gzip or independent fflate, canonicalize JSON, never demand byte-identical compression from different compressors. SQLite enumerates manifest digest lists, joins against authenticated record inventory, detects missing links and unions JSON key/value Source references. Byte/debit expected output comes from actual packed/expanded sizes summed independently. Unknown graph remainder fields must follow actual upstream graph contract, not arbitrary new closure restrictions. Shared digests deduplicate required coordinates but do not excuse bytes actually read.

Existing High plan extends formats schema/corpus/test; Root decides final selected title/budget after actual candidate. This matrix is DATA preparation only, no Source evaluation/Native/current cache reads or pass claims.

## Actual Writer Reconciliation

Producer record corpus graph remainder concretely contains version,namedInputs,unavailable,admission, after removing projects/dependencies/externalNodes. Matrix graph-remainder positive must preserve those legitimate fields and follow actual readBuildGraphV1 admission. Records policyversion2 with projectsPerYield/edgesPerRecord/externalNodesPerRecord controls; per-record maximum applies to raw JSON before gzip. Digest applies to actual compressed bytes. Encoder aggregate maximumBytes counts deduplicated packed records; manifest bytes are added in returned total, not checked by that same record debit. Writer funding counts only missing compressed records plus absent manifest. Closure proof has separate stricter cumulative packed+expanded read budget and must not equate producer allocation accounting with proof work. No executable producer call was made by tooling.
