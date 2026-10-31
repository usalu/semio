# Final Native Witness Input Plan

Provenance correction: historical container-native-build-source.json is a partial selected census despite its filename. Its1257rows/800Go/71changed facts are listed-row comparisons, not complete canonical compiler input certification. No current owner-persisted combined exact Go overlay/dependency/embed/runner manifest has been established; see native-source-witness-next.md. Physical artifact and warm reuse evidence below must be labelled separately from that remaining Source provenance gap.

Read-only current ticket inputs and search protocol review, 2026-10-29. No input script or Source was edited; no test, native query, container, process or lease operation ran.

## Minimal Workload-Specific Count Change

Keep the production ShardManifest schema and generic helper permissive for valid sparse indexes. StorageProtocol.shards=64 is partition capacity; ShardManifest.shards has maxItems64, not minItems64. A one-document/one-shard mini-domain fixture remains valid.

The real-repository acceptance input already hardcodes root /workspaces/semio and query linux. Add an explicit expected occupied count only to that workload's calls: `indexState(64)` after cold and warm. Give indexState an optional expected count; validate the generic closed manifest first, then require exact count only when supplied. For expected64 require the sorted names to equal Array.from({length:64},(_,i)=>'s'+String(i).padStart(2,'0')); this simultaneously establishes complete s00–s63 coverage, no duplicate and no missing partition. Do not infer expected count from documents>64 or change production schema minItems. A genuinely smaller future workload must use its own explicitly declared expected count, not silently weaken this full-repository gate.

Current helper's `length>64` and /^s[0-9]{2}$/ checks alone accept duplicates, unordered entries and s64–s99. Replace those generic checks with actual ShardManifest schema validation plus strict lexical uniqueness and summed document count. The native reader already enforces these semantics; the evidence reader should refuse invalid metadata independently before constructing paths. A failed exact64 condition is an acceptance mismatch to diagnose, not permission to reset a cache or invent shards.

## Authoritative Paths And Fields

Source schema: 🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/🧬️schema/🔣️.json, $defs.StorageProtocol, $defs.ShardManifest, $defs.Shard and $defs.SnapshotCacheProtocol. Use existing Ajv behind the ticket verifier; no runtime dependency change is needed. ShardManifest schema is semio.search.shards/1, closed keys schema/snapshot/documents/shards; each shard has name/bytes/sha256/documents/events. Max manifest65536B; shard268435456B,250000documents,500000events; snapshot identity1–256chars. Protocol Go implementation is search/📦️packages/🐹️go/🧵️shard/🐹️.go:22–45 and390–466.

Physical cache root is /workspaces/semio/.🧬semio/🦑️repo/⚡️cache/<SHA256 UTF8('/workspaces/semio')>. Root identity must match the authenticated actual repository coordinate; do not hash host /home/deck/semio for the container witness.

Capture these exact ordinary files:

- tree/snapshot/meta.json and tree/snapshot/payload.json.gz.
- index.search/meta.json and index.search/manifest.json.
- index.search/s00/events.jsonl through index.search/s63/events.jsonl, derived exclusively from the validated full-workload manifest.

Snapshot metadata is implemented by search/📦️packages/🐹️go/🌲️snapshot/🐹️.go (closed metadata decode and bounded compressed payload); its protocol schema semio.search.snapshot/1 caps metadata65536B, payload268435456B and gzip67108864B. Index cache meta is tree/📦️packages/🐹️go/🎊️query-cache/🐹️.go:39–50, including schema/fingerprint/scope/section dimensions. Record parsed manifest snapshot/documents and every ordered shard metadata row, not only file names. The current implementation's warm open independently validates actual event bytes/hash/count through replay.

## Stable Read And Compare Witness

For each file record relative name, dev, ino, mode, nlink, bytes, mtimeNs, ctimeNs and SHA256. Use bounded streaming256KiB reads and before/after fd/file identity checks; reject symlink/nonregular/multilink files and changed metadata while hashing. Bound manifest/meta before parsing; use protocol byte limits for snapshot and shard files, plus an explicit declared total audit-byte budget if a stronger operational bound is needed. Verify each shard actual size/hash equals its manifest row, and total shard documents equals manifest.documents. This adds actual byte identity beyond the current helper's shard size/inode/mtime-only rows.

Hashing and count enforcement are separate: the count-only minimal improvement is `indexState(64)` plus exact ordered names; full byte witnessing additionally streams every shard. Avoid treating unchanged inode/mtime as an independent byte-hash proof. Capture/hash all64 once per observation if full actual hashes are required; do not repeatedly reopen shards in additional diagnostics.

Use exact before/after cancellation snapshot comparisons and after-cold/after-warm snapshot/index comparisons. Require no snapshot publication after positive identity-stage cancellation, terminal130/no OS signal/empty stdout/no forced budget; then terminal0 cold/warm within120000ms and16MiB output bounds, equal parsed JSON and unchanged captured physical generation. Compare dev/ino/size/mtime/hash for retained files; mode/nlink/ctime changes also signal an unexpected mutation requiring explanation. Validate full64 count after both completions. Preserve protected caches and label initial state accurately.

The capture must settle without concurrent Source/domain updates invalidating identity. Record Source manifest/binary digest and exact timing around the two requests. A missing throttled progress phase is not a reuse failure or proof; physical identity is decisive. No file deletion or cache reset is part of this witness.

## Wrapper And Source Receipt Limits

Current run-container-command.cjs validates the log label, creates only ticket generated output, and uses registered normal Docker with SEMIO_TOOL_DOCKER=/usr/bin/podman. Its final condition is `if(result.status!==0) process.exitCode=Number.isInteger(result.status)&&result.status>0?result.status:1`. Thus spawn error or signal yielding null status fails1; the prior false-success possibility is corrected. The debug record logs status/error but not signal; retaining signal as diagnostic metadata would improve failure interpretation without changing the now-correct exit result.

Current capture-container-source.cjs correctly labels its coverage as all present nonignored Go sources and Go manifests plus changed/untracked implementation/configuration files, Source payloads excluded. It excludes ticket contents, skips missing/nonordinary files and checks per-file stability. This is a selected census, not all authored Source, a complete admitted SDK closure, an atomic tree freeze, generated-asset freshness or live container identity. Preserve it as a selected build-time receipt and separately verify the actual complete build/installer/resource closure. It does not establish any bytes for omitted unchanged non-Go imports/resources. Its internal Git calls are read-only, but directly spawn `git`; an execution under the current all-commands-through-repo constraint should route those through registered repo Git rather than treat the ticket script as an exemption. This audit did not execute it.

Installed receipt verifier remains native-layout-installed.cjs, using the current installation schema at client/🔌️mcp/📥️installation/🧬️schema/🔣️.json. Bind its fresh digest/ELF/receipt checks to the preserved selected census plus actual authenticated closure before authorizing this final witness. Historical installed3f61eed0… and 71non-Go drift are insufficient for current acceptance.

Root applied the minimal evidence-input correction: actual central D07 ShardManifest+Shard validation, strict ordered unique names, summed document count, and indexState(64) at both full-workload observations. Generic helper without expectedCount still permits valid sparse data. Production schemas and cache content were not modified. Full independent streaming hashes for every shard remain an additional witness enhancement; current helper retains per-shard inode/size/mtime comparisons and the actual Native reader's mandatory byte/hash verification. No Native query was run.

## Streaming Evidence Implemented

The retained input now hashes every snapshot file, index metadata/manifest and all64 admitted shard files with256KiB streaming reads. It validates exact single-link ordinary-file identity before open, from the live descriptor during progress, and again from descriptor and leaf after reading (dev/ino/mode/nlink/size/mtimeNs/ctimeNs). Each shard's actual byte length and SHA256 must equal its admitted manifest row. Metadata capture is capped at64KiB; payload/shard caps come from the current Source protocols. No protected cache was read or changed in these focused tests.

Actual Node24 tests:9passed,0failed. Language-neutral cases cover empty/small/multiple-block files, byte-limit refusal, symlink and hardlink refusal, mutation during a read, cancellation and bounded metadata capture. Existing Ajv validates the corpus; independent existing @noble/hashes SHA256 agrees for each positive output. The initial helper test first failed because the new module was absent; bounded capture additionally had a genuine behavioral red8passed/1failed before implementation. Input syntax check passed. Temporary fixture directories were removed by finally blocks; retained test/schema inputs remain in the ticket.

Snapshot/index observations are async, report progress every16MiB, and accept cooperative SIGINT during observations without altering the existing Native query's cancellation owner or limits. This is verification-input readiness only. No real Native cold/warm/cancellation query or new installed-artifact success is claimed.

Changed inputs: native-evidence.cjs, native-evidence.test.cjs, native-evidence-vectors.json and native-layout-acceptance.cjs. Existing physical identity comparisons now include hashes for all shards, rather than relying on Native replay alone. Per-file stability does not create an atomic multi-file tree snapshot: concurrent generation changes must still cause the acceptance run to be rejected.

## Actual Shard Observation Integration

A further9tests passed for the actual retained indexState/evidence/observe function bodies, selected by the existing third-party TypeScript AST parser without copying Source files. The language-neutral corpus covers valid sparse and full64-shard manifests, incorrect hash/length, duplicate/unsorted/out-of-range shard names, incomplete full-workload coverage and document-count mismatch. An independent SQLite relation computes acceptance; independent @noble/hashes supplies actual event digests. All18focused evidence tests are green; no Native query was executed. Temporary fixture files were generated under the ticket and removed after each test.
