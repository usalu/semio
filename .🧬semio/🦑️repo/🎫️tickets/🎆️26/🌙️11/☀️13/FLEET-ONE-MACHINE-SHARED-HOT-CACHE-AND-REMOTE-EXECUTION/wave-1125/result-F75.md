# F75: "descriptor changed" checks sorted into three classes

m12 cause: `smallBytes` in the closure proof (`⚡️caching/…/📏️headroom/🧾️closures/🟦️.ts`) compared the full stamp (mtimeNs/ctimeNs included) of a group's append-only member index against the earlier lstat; a sibling append or metadata touch threw a plain `Error: Closure index descriptor changed`.

## Shared helper
`🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧷️descriptor/🟦️.ts` (+ `🧬️schema/🧷️descriptor.json`, test `🧪️tests/🧷️descriptor/🟦️.ts`, 6/0 incl. Python stat cross-check). `settleDescriptorV1` by class; refusal `descriptorRefusalV1` is a ConcurrentEditErrorV1 with EN/DE text ("<label> file changed while it was open / <Label de> Datei hat sich geändert, während sie offen war", analogous for append-only and renamed), bounded pauses 10/40/160 ms.

## Sites converted
- (a) append-only (identity dev/ino/nlink, size may only grow, no mtime/ctime): closures `smallBytes` for `member-index` rows (adopts fresh row, re-reads, bounded); native journals `🧹️cleanup/🎛️command` and `🧱️fixed` (stamp compare replaced by byte equality of the held prefix).
- (b) immutable, bounded re-observe then typed bilingual refusal: closures `smallBytes` other indexes; closures retirement leaves (orphan/closure leaf x2, group leaf x2, terminal intent); `👷️closure-read-worker` (3 checks, texts and codes unchanged, child test 2/0); `📄️retirement-file` reader (bounded re-read); builder authoring target; vite artifact descriptor; cache charge receipt (layout); dependency receipt observation (2); authorizer module body; cargo execution Source; observation leaf (`🏗️build/👁️observation`).
- (c) replaced by rename, reopen by name bounded: nx graph publication `bytes()` (`🔌️nx/📦️graph/📒️journal`); current maintenance file reader (`📥️opening/📤️publication`).

## Sites kept, with reason
Already churn-classified (concurrentEditV1 under retry layers): `🔏️revision`, `🧊️stable-file`, `📥️reuse`. Structured own diagnostics: deps storage `refuse()`. Identity-only already (dev/ino/size/nlink): leases physical/metadata, cargo product receipt. Tolerant (return undefined/continue): ready input hashing, tools-grace, fleet outage client. Own state under an exclusive lease: native storage journal append (109), receipt creator (162), fixed image identity (133). Embedded worker script string, cannot import: headroom `🟦️.ts:38`. Not reached by a descriptor-vs-stamp check: bun closure scripts, layer-index.

## Ready schema (source-transient)
`🚦️ready/🧬️schema/🔣️.json` +4 patterns: old plain texts (Closure index/proof descriptor changed, retiring leaf, group retirement, terminal intent), the three bilingual helper texts (any label), worker "descriptor stamp mismatch". Test `🧱️source-transient` 4/0 (exact m12 line "reference 92b15f5b … Closure index descriptor changed" among 10 new cases, equal to Python oracle).

## Peer breakage fixed on the way
The closure read child runs under node strip-only; peers' parameter properties in `🧩️component-pass/💾️storage-gate` (NativeBuildStorageShortfallV1) and `🦀️cargo/🗃️unit-cache/🔒️lock` (SlotWaitTimeoutV1) made the child unloadable. Rewritten as plain fields. Peers must avoid `constructor(readonly x)` in that import chain.

## Not verified
Closures main test file (stopped by seal window), storage journal tests, nx journal and publication tests: not run. `repo typecheck`: closures, descriptor, worker, nx journal clean; others only errors in unrelated test files. Dangling imports 0.
