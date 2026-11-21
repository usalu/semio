# Graph Closure Shard Transport

The archived-size regression models 5,101 fragments, 20,671 typed closure references and 73,950 owner links. The former exchange policy carried at most 20,000 closure files. Its flat ActionResult grew by another path and digest for every closure.

The new ActionResult keeps graph documents in OutputFiles and addresses canonical REAPI Trees for the closure inventory. The schema allows 32,768 closure leaves, 1,024 files per Tree, and 160 KiB per Tree. The existing authenticated 4 MiB ActionResult ceiling still applies. The CAS server qualifies the Trees and their leaves before admission. If a Tree or leaf is evicted during publication, the exchange requalifies the addressed content and retries the same ActionResult once. Cache collection resolves live Tree roots transitively.

Using the production protobuf encoder with 5,104 document entries, 20,671 closure entries, maximum fixture payload sizes and a 512-byte note yielded 3,897,644 bytes for the flat ActionResult and 778,045 bytes for the Tree ActionResult. The 21 Trees occupied 3,080,063 CAS bytes in total. The largest Tree occupied 152,580 bytes.

Strict restore reads each shard by its exact digest and size and compares its inventory with the closure tokens in the verified fragments. A missing or omitted shard and a missing leaf all leave the local generation pointer absent. A non-strict partial head retains its existing stale-owner reporting.

The publisher keeps its bounded, checksum-validated fragment memo. Repeated publication of the fixture reopens zero unchanged fragments. The mixed-memo test reads only the new fragment and checks that the same SHA-256 can appear in both closure namespaces. Readers verify the fragments independently of the publisher memo.

The private archive-sized test uses the existing authenticated protocol validator and SQLite as an independent inventory oracle. A separate HTTP fixture evicts a Tree before admission and observes one refused write followed by acceptance of the identical head.

The first full exchange run passed 46 of 47 tests. The existing serving-size shadow-margin fixture exceeded Bun's default five-second timeout under concurrent load. The rerun passed all 47 tests across nine files with a 20-second runner default. The shadow-margin fixture completed in 1.97 seconds. The separate three-wave fixture passed in both runs and retained its authored 30-second deadline. Both focused cache retention tests also passed.

A later shared-source check found the memo schema declared a different shape from the runtime, which caused the import to fail on startup. Restoring the runtime's required version, checksum and entries fields preserved the stricter array bounds and advanced the schema digest from `32af123a7d51cbc42349096ebe6587e7b3277a56d2d93cd24e427cc5c4e09f54` to `54d0500e4ced3fec1028a7ab61414352bca8e1395872e9ee5e91330c967b3cf5`. A direct native import probe and all 16 scoped closure exchange tests passed after the correction.
