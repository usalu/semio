# GC Census and Cardinality Independent Static Audit

Read-only review; moving batch repair excluded until final held candidate. No Source evaluation/CLI/Native/cache effects.

No concrete blocker found in inspected census/cardinality admission and aggregate bounds. Main test entry imports groups.ts; groups statically imports census,cardinality,batch homes. Both new closed corpus/schema pairs pass strict Ajv validation. Census has10 platform-service/application boundary vectors; cardinality has5 entry-bound vectors including131073files, observed136474files+23737directories,262144edge and262145refusal.

Removing separate maximumFiles leaves unchanged262144 aggregate directory-member ceiling: per-directory names cap and cumulative names count both remain, including ignored platform entries, directories and files. Rows/directories/pending names bounded by that single census; ordinary leaf bounds/path256/sameimage/singlelink remain. Cardinality memory test measures stub census JS heap delta against1536MiB; it is not measured full-proof peak/RSS or physical Native runtime admission.

Parent retains16777216charged checks,16GiBaggregate raw+decoded budget,512MiBfile/batch,64rows/64KiBstdin request and64MiBresult. Before child launch parent pessimistically reserves remaining shared byte allowance; commits accepted total only after all row/hash/reference/link validation, so failure cannot retry free work. Worker charges raw before Buffer allocation, decoded before parsing, decompression maxOutputLength=min128MiB/remaining; parent small-index reads also charge before allocation. Bounds may honestly refuse costly O(entries×unlinks) fullproof work; no availability guarantee inferred.

Service exemption is only direct Native classifier at actual image root before application path access; nested same names follow ordinary census. Root matching service symlink is delegated without opening it; unknown/control symlinks refuse. Native owner remains responsible for delegated service namespace. Stale-loaded implementation guard binds closure runtime/schema/worker physical hashes outside an actual implementation pin; it does not claim a blanket transitive Source guard.

Current observed pins (batch remains moving):

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/♻️groups.ts`: 9066B / `4324ce5be18d053967b025cf2411f6f14afee5b8b8daba2b9827e79e3c74ce70`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts`: 45834B / `286c3fb5dbc833fd25f8fb64e5aa455377edf98f8a8dd980eb97ba4008d028bb`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧬️schema/🔣️.json`: 15845B / `db21ac19ce122b71eb913c21c0ac75120c463423998f589d4aabc8327e0b27b0`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/👷️read.mjs`: 13269B / `a571881deeacbeae12c2bd34b2a990cf4f070238f5c74be985e7a3735f0fac65`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📁️census/🟦️.ts`: 4627B / `0cd131b05ad9a76a36d9f9b69c22ee4537b7633c793824f34f9361da7a82226b`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📁️census/🧫️cases.json`: 1629B / `13b634eaa4fc61a086da5fe22ab75bfd5cda979c3692137984772e562e40b2fa`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📁️census/🧬️schema/🔣️.json`: 1035B / `f5ce10135fbd7c2904b62e03ee4b93b734aa909bac15237d791a8ee36c232d49`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📏️cardinality/🟦️.ts`: 3982B / `87eb8e1071e00c54f834617dcd5196ffe6941b6fe96e57cac85796de528b3187`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📏️cardinality/🧫️cases.json`: 725B / `0312a2afa25d52991e92d8e1c6546b5dfab154cc3ea507d979eb688edb43d066`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📏️cardinality/🧬️schema/🔣️.json`: 1046B / `8cd2fae3b0f99890681083666d466481a6eb1381906e81750e4fd4ee35402aaf`
