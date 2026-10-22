# Consumed Maintenance Receiver Adapter Review

Read-only actual Native/source review; no helper/test/native execution.

Native fixed backend storage/🧱️fixed/🟦️.ts98 closes maintenance readers, clears guard, moves gap, then ends receiver closed on failed nonwaiting acquisition or after successful EX callback. Persistent macOS storage/🧹️cleanup/🎛️command/🟦️.ts142 does the same: ready→gap, deferral closes, successful maintenance terminal closes owner/receiver. Thus withExclusiveMaintenance consumes its receiver on both outcomes; post-return recheck/statfs is invalid authority. The private body fixture image has a distinct lifetime wrapper, but ordinary persistent adapters must follow this actual contract.

Current scoped adapter samples availability only inside acquired EX callback, then returns consumed:true/acquired:true plus counts/sample; false branch returns consumed:true/acquired:false/zero counts with no availability. Closed schema oneOf matches. It performs no post-consumption image recheck/statfs. This semantics is coherent and can be adopted independently of authorship.

Current Nx graph prepare73 awaits scoped adapter, then marks recover without resampling consumed receiver, closes it, and reopens real coarse recovery. Selected path preceding scoped is its own live operation; no false optional result can establish a physical floor. Storage opportunistic maintain checks control only after maintenance and closes receiver; prepare wrapper uses callback-returned DATA rather than post-return physical access. Opening adapter's accepted-operation enrollment retains Original lifecycle without adding native SH.

Required integration: exact declaration and extracted caller/adapter fixtures must use consumed union and zero/noavailability false branch; tests should prove success and deferral both close actual receiver and future operations refuse. Scope/helper metadata pins must refresh to current observed coherent bytes. No final whole-cohort or runtime pass claim; peer delta preserved.

## Exact Direct Witness Observations

 - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts` — 97789B / `498a7014e9b13fd3b789593d6032e84bd5e08942d4033ec336df441981fd32cd`; ordinary single-link stable FD/name epoch=true
 - `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts` — 129922B / `7023b11722183fd65409c9bebf1c137568b4ad1ef8db8ac561e4b758571d184f`; ordinary single-link stable FD/name epoch=true
 - `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts` — 128538B / `334e30775e4ce3d6f61aadc4e434cd99e549c7233ac2218b0d102fadb0166c34`; ordinary single-link stable FD/name epoch=true

## Integrated Fixture Blocker

SourceHigh identified concrete same-fixture reuse defect: issuer exposes one image, withReader reuses it after withMaintenance consumes it. Actual Native contract above makes later writer/consumer invalid. Integrated acceptance is blocked pending private owner reopen of same funded child under same diagnostics Original, full old receiver drain/close and exact current-image mapping. Old custody/image must reject; no new allocation/backing/caller options. Earlier GC own11 static disposition does not establish integrated fixture execution. Root/High notified; no Native dispatch authorized by this audit.
