# Gzip Snapshot Closure Role Audit

Actual95408 frozen failure is unknown snapshots/<64hex>.gz. Current role30–52 accepts only snapshots/<64hex>.json and therefore refuses producer gzip records before payload validation. Independent second mismatch: worker snapshot branch requires whole graph version1/projects object/namedInputs/externalNodes; genuine current snapshot JSON is version2 record manifest with graph remainder, projects[name,digest] rows, dependency digest array and optional external digest array. Merely adding.gz regex would neither validate nor safely retain this producer format.

Actual producer caching graph owner49–54 calls encodeGraphRecordsV1. Records owner13–30 gzip-compresses each project JSON, dependency chunk and external-node chunk; filename digest is SHA256 of compressed bytes. Manifest filename is SHA256 of manifest UTF8 bytes. Producer writes record.gz before manifest.json under actual graph store writer, fsync/rename. Actual consumer buildgraph441 verifies pinned manifest and decodeGraphRecordsV1 validates compressed digest then bounded gunzip/JSON; policyversion2 records max256MiB, aggregate snapshot512MiB.

Minimal correct repair: schema-first explicit current manifest and gzip-record roles, compressed-byte digest authentication, boundedgzip expansion under shared proof debit, producer-sized JSON validation and conservative closure-reference scanning. Manifest must emit required links to every record and parent must authenticate their inventory presence; unknown/partial/corrupt record or manifest states continue refusing. Do not classify arbitrarygzip or silently ignore unknown publications. Existing standalone record tests already compare independent Python gzip/hash decoding, but current selectedGC format law does not exercise this format. Proposed one literal actualproducer→genuineEXclosure law uses Python gzip/hashlib and SQLite reachability oracle, provisional120000test+60000margin=180000phase; Root chooses actual final authored selection after implementation.

Finite current pins:

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts: 50580/ddd9e8909cc4f052d017b44950b4b72ec68381b9a745292cd4ea59064c666bac
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/👷️read.mjs: 29006/37a28e404c0c25dabcbf54239b7763aacd87f8654aa95ee8bcd592299cf94247
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧬️schema/🔣️.json: 36813/a0c80905f5788e36d873206b8abc5efd8233c656de879d69af4296dc402e776a
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts: 12010/5dd0f53b6d40862f5eefc14048a32fcad59ccdf9b59081fbe0e5ae48fbc4b07d
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🔣️policy.json: 305/cb2e9a9c55e5bf9b6014fedbc4f10052756c0f3eebc41661f4a1e154844add48
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🧬️schema/🔣️.json: 1771/d94cc2534e57d1588ebde9f908f9790eebb6f2fc40157f3c03d408d0f115943e
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📇️records/🟨️.mjs: 5890/09961c96fdc365a44609bd237c37acdb1e411c287fd9dc066cc7650db7f7a024
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📇️records/🧪️tests/🟦️.ts: 5728/5c63c1298fb4b40e7a91a2232fbafe8f9941b1fe37541fae5243d70372ac5ae3
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📇️records/🧪️tests/🔣️.json: 2310/28c0ab5a97a7c4f1194301e4da1831cb537f36a2c80f8ba968b59ebdf8394b47

No cache contents, Source imports/functions, CLI, Native or production mutations were evaluated. Current Source is not historical loaded-byte attribution.
