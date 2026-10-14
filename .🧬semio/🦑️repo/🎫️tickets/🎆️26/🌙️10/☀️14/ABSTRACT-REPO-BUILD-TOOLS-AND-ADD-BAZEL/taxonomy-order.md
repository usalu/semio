# Current Taxonomy Ordering Refusal

Bounded read-only inspection found complete valid JSON with identical SHA256 on two reads: `c77555ce2a83cae587497730fbe194d193151c7db859917fc89ec583e6b9cd9e`. The observed failure is stable ordering at this snapshot, not evidence of a torn JSON write. No product or peer was modified.

The generatorContracts.wgpu-frame-worker.packageGeneration.browserProfile.sourceModulePaths array has 171 entries and no duplicates. Its sole adjacent UTF-8 byte-order inversion is zero-based index 31: `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🫀️liveness/🟦️.ts` follows index 30 `🧰️framework/🔨️modules/🎭️actor/🧬️codec/🟦️.ts`. The codec entry occurs too early relative to the subsequent actor paths.

The same contract's inputPatterns array has 193 entries and no duplicates. Its sole adjacent lexical inversion is zero-based index 30: `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` follows index 29 `🧰️framework/🔨️modules/🎭️actor/🧬️codec/🟦️.ts`. The codec entry again occurs too early.

The actual source validator in discovery/🟦️.ts line 5506 requires exact eligible module paths and strictly increasing projectionByteCompare order for browser inputs. Line 5113 independently requires inputPatterns uniqueness and JavaScript default lexical order. The identified pairs violate the respective ordering rules and suffice to explain both reported admission messages. This audit did not run full taxonomy validation or assert that unrelated semantic fields are valid. Root/core received exact findings; correction ownership remains with them.

## Independent Post-Correction Runtime Check

The corrected exact SHA256 is `6e44ed8075907527a2c314b61883e5593ab3cb01093439dfad9e7fb274355109`. Retained `taxonomy-proof/📜️script.ts` independently checks this digest, unchanged 171/193 counts, uniqueness, JavaScript Buffer byte/default lexical ordering and third-party lodash sorting (hex-key byte oracle and direct string oracle). All ordering assertions passed. The actual strict loadCatalogTaxonomy validator also passed, confirming the reported package-profile/order admission is corrected.

The actual full loadTaxonomy validator then failed a separate current physical workspace check: `generatorContracts["graph-catalog"].nativeConsumers has no Cargo project at "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust".` This is not an ordering failure. The proof process deliberately exits nonzero on that full-loader error; full taxonomy/workspace validation is not claimed green. Root was informed. Generated `taxonomy-proof.log` retains explicit ordering/catalog success followed by the exact workspace failure. No source, taxonomy, Cargo file or peer was modified by this reviewer.
