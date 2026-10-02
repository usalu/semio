# Neutral Compute Interface And Caller Roster

Read-only current audit; no jobs. Companion complete-os-deletion-scope report retains protocol/replication authority distinction.

Neutral destination: general2D/🧮compute/🦀.rs, fixture/schema/test sibling facets. Own complete EngineKey([u8;32]), EngineHandle{key,engine_id}, EngineFault UnknownEngine/Compute/Evicted/InvalidInput, Engine Send+Sync+'static with ENGINE_ID and compute borrowed bytes→owned bytes, EngineHandles Vec+empty, EngineCache new/register/engine_key/derive/read, EngineRep<P>::build. Private DynEngine/CacheEntry/map/LRU helpers stay same owner. First-party hash is sole nonstandard runtime dependency. Explicit public exports, no os_engine or KernelEngineHandle aliases.

Actual higher fields/functions: Flow DrawingHandle hex conversion311–322, StoredNode decoding/encoding294–296, DrawingEngine registration, drawing cache budget, drawing mutation/inference assembly; plugin host EngineHandles injection and lifecycle; OS protocol facade combines neutral replication with command/channel/CLI/history. These remain higher consuming neutral interfaces. 2D geometry itself never needs OS protocol; remove os_spr facade export and OS Cargo dependency, rebind any real protocol callers directly to existing neutral replication contract. No blanket protocol copying.

Original engine six laws: register_and_derive_echoes_input, derive_twice_same_key_is_cache_hit, eviction_when_budget_exceeded, unknown_engine_fault, engine_key_is_stable, engine_rep_build_is_deterministic. Preserve original bytes/assertions/mount after moving owner; original source hashes below. These laws assert same-key/inequality but do not pin digest bytes. Schema-first vectors must add independent first-party hash/third-party BLAKE3 oracle exact bytes, input ID framing, empty/nonASCII/NUL and split-boundary collisions; no silent key change. Current key concatenation ID+NUL+input109 is ambiguous. Separate canonical length-delimited contract change from byte-identical ownership transfer.

Actual cache caveats preserve/own explicitly: output larger than budget currently still admitted after evict-all; same key read ignores engine_id field and read clones output. Registration replacement does not invalidate existing cached keys. New tests must specify these semantics rather than silently alter during ownership move. EngineRep only concrete impl remains unit SumRep; comments elsewhere are not implementations.

Current source SHA256: 9b0b163e1f92438a0855862d0ba785a395459382e99a9b360aaa2d1038192bb6
Current six-law source SHA256: 96ea271d3e9e0e809d325e1f7a33d6d21d1f767129d654009c069b7b358962dc

## Actual caller census

```text
✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs:37:use store::EngineHandles;
✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs:1023:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:12:use store::EngineHandles;
✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:76:        _engines: &EngineHandles,
✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:37:use store::EngineHandles;
✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:823:        _engines: &EngineHandles,
✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:34:use store::EngineHandles;
✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:630:        _engines: &EngineHandles,
✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:15:use store::EngineHandles;
✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:62:use store::EngineHandles;
✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2673:        _engines: &EngineHandles,
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:25:use store::EngineHandles;
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:784:        _engines: &EngineHandles,
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:343:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:119:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs:328:pub use crate::os_engine::*;
✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:30:use store::EngineHandles;
✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:551:        _engines: &EngineHandles,
✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:12:use store::EngineHandles;
✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:112:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🦀️.rs:389:        _engines: &EngineHandles,
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:31:use store::EngineHandles;
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1022:        _engines: &EngineHandles,
🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🦀️.rs:45:use store::EngineHandles;
🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🦀️.rs:1012:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🦀️.rs:573:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/👁️viewer/🦀️.rs:105:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:101:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:33:use store::EngineHandles;
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1770:        _engines: &EngineHandles,
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:243:        _engines: &EngineHandles,
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:47:use store::EngineHandles;
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:925:        _engines: &EngineHandles,
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:100:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🦀️.rs:611:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/👁️viewer/🦀️.rs:108:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs:36:use store::EngineHandles;
✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs:1064:        _engines: &EngineHandles,
✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🦀️.rs:12:use store::EngineHandles;
✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🦀️.rs:69:        _engines: &EngineHandles,
✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:30:use store::EngineHandles;
✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:523:        _engines: &EngineHandles,
✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:17:use store::EngineHandles;
✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:116:        _engines: &EngineHandles,
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:41:use store::EngineHandles;
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1340:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️.rs:2://! Kernels register as Engines; plugins only hold EngineHandles — never their own registries.
🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️.rs:63:pub struct EngineHandles {
🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️.rs:67:impl EngineHandles {
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:11:use store::EngineHandles;
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:84:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/✏️editor/🦀️.rs:21:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/✏️editor/🦀️.rs:340:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/✏️editor/🦀️.rs:549:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/👁️viewer/🦀️.rs:105:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:19:use store::EngineHandles;
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:706:        _engines: &EngineHandles,
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:100:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/✏️editor/🦀️.rs:195:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/✏️editor/🦀️.rs:15:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/✏️editor/🦀️.rs:236:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/✏️editor/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/✏️editor/🦀️.rs:162:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic/✏️editor/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic/✏️editor/🦀️.rs:162:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:30:use store::EngineHandles;
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:634:        _engines: &EngineHandles,
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:110:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/✏️editor/🦀️.rs:17:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/✏️editor/🦀️.rs:166:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🦀️.rs:231:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:44:use store::EngineHandles;
✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:938:        _engines: &EngineHandles,
✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:119:        _engines: &EngineHandles,
✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:31:use store::EngineHandles;
✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:950:        _engines: &EngineHandles,
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:26:use store::EngineHandles;
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:640:        _engines: &EngineHandles,
✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:12:use store::EngineHandles;
✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:12:use store::EngineHandles;
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:107:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/✏️editor/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/✏️editor/🦀️.rs:161:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/✏️editor/🦀️.rs:22:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/✏️editor/🦀️.rs:435:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/👁️viewer/🦀️.rs:101:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🦀️.rs:162:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:26:use store::EngineHandles;
✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:470:        _engines: &EngineHandles,
✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs:504:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:105:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🦀️.rs:12:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🦀️.rs:409:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/✏️editor/🦀️.rs:514:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/👁️viewer/🦀️.rs:106:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/✏️editor/🦀️.rs:138:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/👁️viewer/🦀️.rs:67:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:32:use store::EngineHandles;
✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:881:        _engines: &EngineHandles,
✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:84:        _engines: &EngineHandles,
✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:29:use store::EngineHandles;
✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:937:        _engines: &EngineHandles,
✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:12:use store::EngineHandles;
✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:82:        _engines: &EngineHandles,
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:32:use store::EngineHandles;
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1084:        _engines: &EngineHandles,
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:17:use store::EngineHandles;
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:301:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🦀️.rs:18:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🦀️.rs:163:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/✏️editor/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/✏️editor/🦀️.rs:162:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/✏️editor/🦀️.rs:138:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/👁️viewer/🦀️.rs:67:        _engines: &store::EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️.rs:146:/// 🧠 Content-addressed inference value cache — mirrors `crate::os_engine::EngineCache`'s LRU/byte-budget
✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs:38:use store::EngineHandles;
✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs:1175:        _engines: &EngineHandles,
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:59:use store::EngineHandles;
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2247:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/✏️editor/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/✏️editor/🦀️.rs:162:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:15:use store::EngineHandles;
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:80:        _engines: &EngineHandles,
✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🦀️.rs:12:use store::EngineHandles;
✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🦀️.rs:87:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:20867:    /// `crate::os_engine::ArtifactEngine` never existed as a live trait (see
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/✏️editor/🦀️.rs:236:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs:138:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:29:use store::EngineHandles;
✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1032:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:66:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/✏️editor/🦀️.rs:22:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/✏️editor/🦀️.rs:164:        _engines: &EngineHandles,
✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:15:use store::EngineHandles;
✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:74:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/✏️editor/🦀️.rs:236:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/✏️editor/🦀️.rs:236:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/✏️editor/🦀️.rs:236:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/✏️editor/🦀️.rs:236:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/✏️editor/🦀️.rs:236:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🦀️.rs:22:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🦀️.rs:164:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:138:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/✏️editor/🦀️.rs:236:        _engines: &EngineHandles,
✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:48:use store::EngineHandles;
✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2244:        _engines: &EngineHandles,
✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:336:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/✏️editor/🦀️.rs:236:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:402:        build_history_columns, create_config_envelope, create_document_envelope, ArtifactCommand, ArtifactEnvelope, ArtifactPack, ArtifactStore, ChildDispatch, CompositionCoordinator, ConfigStore, EngineHandles, GroupMeta, HistoryColumn,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:9049:                engines: &store::EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:9186:                &store::EngineHandles::empty(),
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:9217:                ViewerApp::<V>::handle(&V::Command::default(), &doc, &cfg, &interaction, None, &draft, &store::EngineHandles::empty()).await.map(|emit| (emit.artifact_mutations.is_empty(), emit.draft_mutations.is_empty()))
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13802:        /// `engines` is the host-owned {@link EngineHandles} bag (empty until WIT engine-derive/read
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13811:            engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:37166:            engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:37785:            engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:38160:            engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:38476:            engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-label-reload/🦀️.rs:118:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🦀️.rs:10:    ArtifactApp, ArtifactMutationOutcome, ArtifactView, ConfigView, DraftView, EngineHandles, InteractionView, PendingArtifactStorePublication, PendingArtifactStorePublicationRetirement, UiAssemblyResult, WindowConfigMutation, WindowConfigOwner,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🦀️.rs:304:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/✏️editor/🦀️.rs:22:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/✏️editor/🦀️.rs:164:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧾️document-archive-load-legs/🦀️.rs:42:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs:73:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/✏️editor/🦀️.rs:236:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs:9:        AppDefinition, ArtifactDialect, ArtifactEditor, ArtifactKindId, ArtifactPack, ArtifactView, ArtifactViewer, ComponentTree, ConfigView, Dialect, DraftView, Editor, Emit, EngineHandles, Fault, IconName, InteractionView, LocalizedLabel,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs:175:                fn handle(_command: &$command, doc: &ArtifactView<'_, $snapshot>, _cfg: &ConfigView<'_, NoConfig>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _draft: &DraftView<'_, NoDraft>, _engines: &EngineHandles) -> ArtifactMutationOutcome<$mutation> {
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs:200:                fn handle(_command: &$command, _doc: &ArtifactView<'_, $snapshot>, _cfg: &ConfigView<'_, NoConfig>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _engines: &EngineHandles) -> Result<ViewEmit<NoConfigMutation>, Fault> {
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs:20:use store::EngineHandles;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs:400:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs:497:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/✏️editor/🦀️.rs:22:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/✏️editor/🦀️.rs:164:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/✏️editor/🦀️.rs:236:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/✏️editor/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/✏️editor/🦀️.rs:161:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/✏️editor/🦀️.rs:236:        _engines: &EngineHandles,
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:50:use store::EngineHandles;
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:5173:        _engines: &EngineHandles,
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:34:use store::EngineHandles;
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1697:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:111:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/✏️editor/🦀️.rs:22:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/✏️editor/🦀️.rs:164:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/✏️editor/🦀️.rs:22:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/✏️editor/🦀️.rs:164:        _engines: &EngineHandles,
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:11:use store::EngineHandles;
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:80:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:161:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs:20:use store::EngineHandles;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs:379:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/✏️editor/🦀️.rs:22:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/✏️editor/🦀️.rs:164:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🦀️.rs:10:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🦀️.rs:145:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/✏️editor/🦀️.rs:22:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/✏️editor/🦀️.rs:164:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:138:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔬️schema-stamping/🦀️.rs:7:use store::EngineHandles;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔬️schema-stamping/🦀️.rs:46:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔬️schema-stamping/🦀️.rs:82:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧫️fixtures/🧩️component/🦀️.rs:22:    async fn handle(_command:&Self::Command,_doc:&ArtifactView<'_,Self::Snapshot>,_cfg:&ConfigView<'_,Self::Config>,_interaction:&InteractionView<'_>,_view_state:Option<&ViewModel>,_draft:&DraftView<'_,Self::Draft>,_engines:&semio_framework_os_kernel::EngineHandles)->ArtifactMutationOutcome<Self::Mutation,Self::ConfigMutation,Self::DraftMutation> {Ok(Emit::default())}
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/✏️editor/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/✏️editor/🦀️.rs:244:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs:21:use store::{Backbone, BackboneMessage, EngineHandles, MemoryBackbone};
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs:354:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:55:    use store::{ArtifactPack, EngineHandles};
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:1091:            _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:1491:            _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:2080:            engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs:434:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs:10:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs:147:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:513:    async fn handle(command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/✏️editor/🦀️.rs:18:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/✏️editor/🦀️.rs:360:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:138:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:286:use semio_framework_2d::Engine as _;
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:291:impl semio_framework_2d::Engine for DrawingEngine {
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:294:    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, semio_framework_2d::EngineFault> {
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:295:        let node = StoredNode::decode_pack(input).map_err(semio_framework_2d::EngineFault::InvalidInput)?;
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:296:        StoredNode::encode_pack(&node).map_err(semio_framework_2d::EngineFault::InvalidInput)
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:302:fn map_engine_fault(fault: semio_framework_2d::EngineFault) -> semio_framework_2d::DrawingError {
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:304:        semio_framework_2d::EngineFault::Evicted => semio_framework_2d::DrawingError::MissingHandle("engine cache evicted handle".into()),
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:305:        semio_framework_2d::EngineFault::InvalidInput(message) => semio_framework_2d::DrawingError::InvalidInput(message),
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:306:        semio_framework_2d::EngineFault::Compute(message) => semio_framework_2d::DrawingError::Operation(message),
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:307:        semio_framework_2d::EngineFault::UnknownEngine(message) => semio_framework_2d::DrawingError::Operation(message),
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:311:fn drawing_handle_from_key(key: semio_framework_2d::EngineKey) -> DrawingHandle {
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:315:fn kernel_handle_for_drawing(handle: &DrawingHandle) -> Result<semio_framework_2d::KernelEngineHandle, semio_framework_2d::DrawingError> {
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:322:    Ok(semio_framework_2d::KernelEngineHandle { key: semio_framework_2d::EngineKey(key), engine_id: DrawingEngine::ENGINE_ID.to_string() })
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:365:/// boundary (process-local [`semio_framework_2d::EngineCache`]) — flow's own ephemeral node-evaluation kernel,
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:368:    cache: semio_framework_2d::EngineCache,
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:380:        let mut cache = semio_framework_2d::EngineCache::new(ENGINE_CACHE_BUDGET_BYTES);
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:386:    pub fn with_engine_cache(mut cache: semio_framework_2d::EngineCache) -> Self {
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs:392:    pub fn engine_cache_mut(&mut self) -> &mut semio_framework_2d::EngineCache {
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:138:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:38:use store::EngineHandles;
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2318:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🦀️.rs:22:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🦀️.rs:164:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/✏️editor/🦀️.rs:22:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/✏️editor/🦀️.rs:341:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/✏️editor/🦀️.rs:22:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/✏️editor/🦀️.rs:164:        _engines: &EngineHandles,
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:32:use store::EngineHandles;
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:1478:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/✏️editor/🦀️.rs:20:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/✏️editor/🦀️.rs:475:        _engines: &EngineHandles,
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:55:use store::EngineHandles;
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:9220:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:138:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:343:        _engines: &EngineHandles,
✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:59:    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional/✏️editor/🦀️.rs:139:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional/👁️viewer/🦀️.rs:67:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:18:use store::EngineHandles;
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:114:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/👁️viewer/🦀️.rs:101:        _engines: &EngineHandles,
✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:25:use store::EngineHandles;
✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:553:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:21:use store::EngineHandles;
✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:250:    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs:306:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:66:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/✏️editor/🦀️.rs:19:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/✏️editor/🦀️.rs:155:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/✏️editor/🦀️.rs:20:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/✏️editor/🦀️.rs:157:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/👁️viewer/🦀️.rs:69:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:46:use store::EngineHandles;
✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1134:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/✏️editor/🦀️.rs:139:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs:20:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs:212:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:89:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:80:        _engines: &EngineHandles,
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:48:use store::EngineHandles;
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:7438:        _engines: &EngineHandles,
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:12:use store::EngineHandles;
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:102:    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/👁️viewer/🦀️.rs:67:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🦀️.rs:482:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/👁️viewer/🦀️.rs:105:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🦀️.rs:480:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/👁️viewer/🦀️.rs:105:        _engines: &store::EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:39:use store::EngineHandles;
✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1340:        _engines: &EngineHandles,
✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:43:use store::EngineHandles;
✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1273:        _engines: &EngineHandles,
✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:86:        _engines: &EngineHandles,
✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:35:use store::EngineHandles;
✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1949:        _engines: &EngineHandles,
✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:67:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:138:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:42:use store::EngineHandles;
✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:3808:        _engines: &EngineHandles,
✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:68:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:78:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/🦀️.rs:17:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/🦀️.rs:254:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:41:use store::EngineHandles;
✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1241:        _engines: &EngineHandles,
✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:17:use store::EngineHandles;
✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:136:        _engines: &EngineHandles,
🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/🦀️.rs:15:pub use os_engine::{Engine, EngineCache, EngineFault, EngineHandle as KernelEngineHandle, EngineKey};
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:138:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:143:        _engines: &EngineHandles,
✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:26:use store::EngineHandles;
✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:663:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:138:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:12:use store::EngineHandles;
✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:83:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:145:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/🦀️.rs:19:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/🦀️.rs:216:        _engines: &EngineHandles,
✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:31:use store::EngineHandles;
✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:827:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:11:use store::EngineHandles;
✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:66:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:23:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:137:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/✏️editor/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/✏️editor/🦀️.rs:235:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/👁️viewer/🦀️.rs:70:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/✏️editor/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/✏️editor/🦀️.rs:191:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:138:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/✏️editor/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/✏️editor/🦀️.rs:193:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:38:use store::EngineHandles;
✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1733:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/👁️viewer/🦀️.rs:13:use store::EngineHandles;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/👁️viewer/🦀️.rs:60:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:35:use store::EngineHandles;
✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:577:        _engines: &EngineHandles,
✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:14:use store::EngineHandles;
✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:124:        _engines: &EngineHandles,
✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:15:use store::EngineHandles;
✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:83:        _engines: &EngineHandles,
✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:40:use store::EngineHandles;
✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1157:        _engines: &EngineHandles,
✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:79:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:138:        _engines: &EngineHandles,
✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:17:use store::EngineHandles;
✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:78:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:29:use store::{ArtifactPack, EngineHandles};
✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:510:        _engines: &EngineHandles,
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:15:use store::EngineHandles;
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:80:    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:28:use store::EngineHandles;
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1106:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:71:        _engines: &EngineHandles,
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:24:use store::EngineHandles;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:138:        _engines: &EngineHandles,
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:12:use store::EngineHandles;
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:81:        _engines: &EngineHandles,
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:36:use store::EngineHandles;
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1727:        _engines: &EngineHandles,
```

This census includes comments; production compiled sites must be distinguished when rebinding. Real Flow uses semio_framework_2d exports, OS root328 exports os_engine, plugin consumes EngineHandles via store. Keep host/store interfaces strongly typed with neutral owned types, without restoring product dependency through facade exports. Retain original native package feature routes and genuine post-deletion compile/runtime proof; none executed by this audit.
