# WAV and EPW Erased State

WAV and EPW typed SQLite providers already preserve states their ordinary file engines cannot encode. Their current erased Text/Pack paths still invoke those engines. WAV includes arbitrary schema/fourcc strings, independently retained pads and data discriminants, optional empty extensions, every binary32 word and an owned unsigned64 auxiliary chunk index. EPW includes arbitrary lexical strings, multiline/comma fields, arbitrary schema and complete header/record collections. These require owned logical persistence before exposing SQLite import through erased I/O.

WAV currently also refuses an auxiliary chunk index outside the current chunk collection, although its public native index type admits the entire unsigned64 domain. The handcrafted SQL will retain that named index exactly and resolve its optional foreign key only when the target exists. Reconstruction will validate the index/foreign-key agreement, preserving both complete owned state and independently queryable relationships.

New native erased/preflight laws and neutral unsigned64 index vectors are being authored before implementations. No new native gate is passing yet.

## Erased Boundary Evidence

The first new erased probe incorrectly indexed native_codecs()[0] for WAV/EPW, whose native factories intentionally return empty vectors. Those index errors are harness failures, not fidelity evidence. The corrected probes register the actual artifact declaration and retrieve its real document_codec. Actual WAV then failed its native-wire invalid-pad-byte admission; EPW lost/rejected retained comma/newline text. Both strict mandatory guards were absent. These actual reds are preserved in the registered rerun logs.

Complete typed logical codecs and borrowed native admission are authored. EPW fresh registered combined gate passes7native laws plus5source laws/31assertions (7.9s), including both actual erased payload modes and bounded admission. WAV enum composition needs explicit DslField bindings; current compile errors are tracked separately from prior semantic failures. A new neutral unsigned64 index corpus covers0,1,2^53+1,2^63,u64MAX; the handwritten WAV relation stores the exact logical decimal index independently of optional resolved target identity, and rejects disagreement. Its native full-index law now passes; fresh combined/public verification is pending.

Both owners now have permanent authored grammar/protocol and shipped demo equality laws. Their first fresh launches exited before the target with only bootstrap banner output; these are harness failures and are being retried before any format asset pass is claimed.

## Full Owned WAV Schema and Public Evidence

Actual new independent source law rejected the stale WAV JSON boundary and GraphQL fields. Its unrelated provider failures additionally exposed a stale duplicated TypeScript SQL constant after the hand-authored unsigned64 relation changed. The provider now imports the one adjacent handwritten DDL as owned text, as EPW already does; this removes the duplicate rather than inferring a schema. Primary/Text JSON declare the full logical model: arbitrary schema/fourcc, independent pads, unrestricted optional extension length, canonical fullu64 decimal indices and exact Binary32 word objects. GraphQL models all4sample branches as a concrete union, fullu32 fields as named scalars and exactu64 indices as strings; protobuf explicitly distinguishes all4sample branches, optional empty ext and rawfixed32 IEEEwords. No protobuf compilation is claimed on this host.

Fresh source6laws/48assertions pass, and the full public WAV package gate passes8outputs/12exports,6snapshot laws48assertions plus10existing mutation/editor laws34assertions (52.3s). The neutral index fixture now uses canonical decimal strings, with explicit test-only native unsigned parsing and independently normalized JSON output; a briefly concurrent native compile captured the intermediate fixture before its test helper update and is tracked as a harness failure, not a semantic regression.

Both complete native grammar/protocol laws admit fixture/default/native-demo logical documents. Ticket-only temporary emission completed for demo Text/Pack; these branches are removed and the permanent comparison laws will run fresh against their authored retained assets. The emission runs are not counted as shipped-example verification. EPW full public package is independently green8outputs/9exports,5laws31assertions (21.0s). Full native owner regressions remain in progress.
