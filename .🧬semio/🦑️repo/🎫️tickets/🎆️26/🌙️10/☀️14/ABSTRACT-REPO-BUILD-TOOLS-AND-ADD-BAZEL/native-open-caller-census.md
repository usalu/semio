> API decision update: Root selected ONE canonical opener with a concrete persistent receiver. Earlier table recommendations to call a separate prepare API describe the superseded draft. Keep fresh-versus-existing intent distinctions for authority/rollback tests; do not restore prepare wrappers. Canonical open now owns accepted fresh provisioning, while cleanup/recovery callers must still prove existing material before opening to avoid unintended recreation.

# Direct Native Image Open Callers

Read-only authored framework census. Exact direct-call syntax only, including embedded worker source strings; no runtime or image/store access. Definitions/delegation are listed separately. This is a wiring review, not blanket permission to prepare before every open. Line numbers may move during concurrent edits.

|File|Lines|Classification / owner|
|---|---|---|
|`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts`|14|Backend definition/delegation, not a fresh caller. High owns native API.|
|`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📥️issuance/🟦️.ts`|49|Generic issued custody opener; borrowed semantics. Explicit fresh caller must prepare before its own SH; do not silently provision here.|
|`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts`|27|Backend definition/delegation, not a fresh caller. High owns native API.|
|`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧪️tests/🟦️.ts`|60,72,74|Fixture: first mkdtemp/owned-image admission fresh; subsequent shared/negative/reopen consumers borrowed. Keep missing-image refusal tests borrowed.|
|`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧪️tests/📚️checkpoint/🟦️.ts`|12,19,27|First parent fixture fresh; embedded crash/replay child borrowed. High native tests.|
|`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🪟️image/🟦️.ts`|22|Backend definition/delegation, not a fresh caller. High owns native API.|
|`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🐧️image/🟦️.ts`|30|Backend definition/delegation, not a fresh caller. High owns native API.|
|`🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🍎️image.ts`|17|Fixture: first mkdtemp/owned-image admission fresh; subsequent shared/negative/reopen consumers borrowed. Keep missing-image refusal tests borrowed.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🧪️tests/🟦️.ts`|48|Fixture: first mkdtemp/owned-image admission fresh; subsequent shared/negative/reopen consumers borrowed. Keep missing-image refusal tests borrowed.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/📒️ledger.ts`|24,39,50,53,78|Initial fixture admissions fresh; crash child/reopen/cancel readers borrowed. Root graph fixture lane.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/💾️storage/🧪️tests/📥️physical/🟦️.ts`|15,34|Fixture: first mkdtemp/owned-image admission fresh; subsequent shared/negative/reopen consumers borrowed. Keep missing-image refusal tests borrowed.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🟦️.ts`|22|Fixture: first mkdtemp/owned-image admission fresh; subsequent shared/negative/reopen consumers borrowed. Keep missing-image refusal tests borrowed.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🧪️tests/🟦️.ts`|26|Fixture: first mkdtemp/owned-image admission fresh; subsequent shared/negative/reopen consumers borrowed. Keep missing-image refusal tests borrowed.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/🟦️.ts`|26,50,51,64|Fixture: first mkdtemp/owned-image admission fresh; subsequent shared/negative/reopen consumers borrowed. Keep missing-image refusal tests borrowed.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts`|22,37|CRITICAL Root: canonical lease partition first-open and recursive nonlease first-open; can require persistent fresh provisioning before borrowed open.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/🟦️.ts`|38|Fixture: first mkdtemp/owned-image admission fresh; subsequent shared/negative/reopen consumers borrowed. Keep missing-image refusal tests borrowed.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts`|70|Fresh diagnostic consumer fixture creation and later reopening; not all generic store opens. Root owns store integration.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🟦️.ts`|36|Existing-image recovery only, borrowed. DO NOT provision absent image just to perform cleanup.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🟦️.ts`|24,30|Fixture: first mkdtemp/owned-image admission fresh; subsequent shared/negative/reopen consumers borrowed. Keep missing-image refusal tests borrowed.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts`|55,107|CRITICAL Root: pool provision=true and native child admission fresh; provision=false/reopen borrowed. Retain parent custody/upfront funding.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts`|24|Fixture: first mkdtemp/owned-image admission fresh; subsequent shared/negative/reopen consumers borrowed. Keep missing-image refusal tests borrowed.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🎛️operation/🟦️.ts`|28,41,57|Parent fixture fresh; serialized original/cancel child consumers borrowed. Root graph fixture lane.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🔒️bounded/🧪️tests/🍎️image.ts`|50|Fixture: first mkdtemp/owned-image admission fresh; subsequent shared/negative/reopen consumers borrowed. Keep missing-image refusal tests borrowed.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️store/🟦️.ts`|43|Borrowed cleanup/recovery owner; DO NOT prepare here (would recreate absent storage). Root graph fixture lane.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/📜️history/🧪️tests/📦️compact/🟦️.ts`|20|Fixture: first mkdtemp/owned-image admission fresh; subsequent shared/negative/reopen consumers borrowed. Keep missing-image refusal tests borrowed.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/📥️workspace/🟦️.ts`|29|Fresh tools/leases fixture admission plus later cleanup borrowed; Core retention helper only.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📦️generation/🧪️tests/🟦️.ts`|18,20|Private physical test first admissions fresh, subsequent readers borrowed. Core/High coordinate exact file ownership.|
|`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📏️headroom/🧪️tests/📜️script.ts`|15,17,19|Private physical test first admissions fresh, subsequent readers borrowed. Core/High coordinate exact file ownership.|

## Required Wiring Distinctions

- Prepare persistent storage before caller acquires its own imageSH; never upgrade that SH implicitly.
- Fresh fixture parent prepares once; crash, cancellation, read-only, busy, missing-image refusal and cleanup consumers retain borrowed open and expected failure semantics.
- Generic issued image custody remains borrowed unless its API explicitly owns fresh provisioning; do not introduce hidden creation authority.
- Repository budget first-open, Nx nested pool and native backing creation must use actual same Original and physically funded parent lifecycle. All child images and accepted work drain before parent closes.
- Mac-only physical fixtures do not become cross-platform tests by switching their imports. Linux/Windows prepare/open contract must match via actual backend implementation and separate evidence.
- This census excludes indirect wrappers already going through canonical repository partition owner; those inherit its corrected first-open behavior. No Source files were modified.
