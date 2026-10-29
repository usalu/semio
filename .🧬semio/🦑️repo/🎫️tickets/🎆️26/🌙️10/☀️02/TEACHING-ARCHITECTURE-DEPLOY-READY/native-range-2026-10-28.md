# Outside-Wire Original Source Range Completion

Parent assigned all remaining outside-wire OriginalWireSourceIdentity constructors while backend owns retained codec semantic range binding. The actual compile30710 is backend-owned; no Cargo process was launched here.

The whole-source fixtures read byte_at(index) directly from complete byte buffers or full original page tables; decoder subranges are separate (start,end) arguments. Their identity base is therefore zero even when the parser selects a nonzero inner interval. The five tool-run tick fixture sources likewise index complete bodies from zero. The installed ToolRun adapter forwards RetainedJobPayload original_byte_at(index); its original_wire_identity method describes the entire retained payload, so its base is zero. The decompressed history payload owns a new PagedList indexed from zero; the compressed original identity/range remain separately retained and are not reused as the decompressed base.

Existing member history and snapshot builders remain unchanged: they retain full backing address and range_start:start alongside extent, and read pages at start+index. Struct update initializers continue to inherit the actual range base, including foreign-address/extent/authority negative cases. Pack value source fixtures already had range_start:0 and a changed-range rejection case; no duplicate edit was needed.

Updated constructors:

- ./🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/📊️progress/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/🪪️identity/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/🔢️entities/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/📝️step/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/🏗️native/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🪪️identity/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🔀️transition/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🔀️origin/🔣️json/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/⚔️conflict/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/📨️message/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🧵️edit/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/📇️identity-records/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🧾️operation/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🧾️metadata/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/📚️frames/🧪️tests/🦀️.rs (2)
- ./🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/🧪️tests/🦀️.rs (2)
- ./🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/🗜️deflate/🦀️.rs (1)
- ./🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/🗜️deflate/🧪️tests/🦀️.rs (1)
- ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/👷️job/🎟️receiving/🦀️.rs (1)

No blanket source identity trait redesign, legacy adapter, grant bypass or new compiler job was introduced. Runtime verification remains with the sole coordinated compiler and later owning fixture suites. Backend wire source-semantic tests and independent range oracle are separate work.

Final full-repository Rust scan inspected 49 reference files and 71 identity declaration/literal starts: zero complete literals omit range_start; the two member nonzero builders still use range_start:start. This is a source-completeness check, not a compile/runtime pass.

## Actual Next DSL Projection Failure

Compiler30710 terminated after wire core compiled, with sole E0599 in DSL schema receive sequence_child: FieldValue::Bytes64 called nonexistent PagedBytes.get. Inspected the canonical owner: retained_bytes returns a shared reference to the existing PagedList, whose get returns the actual retained octet. The targeted implementation now projects retained_bytes().get(index) as the borrowed dyn NativeValueSource. It performs no owner conversion or cloned scalar.

Before the source fix, added a language-neutral closed byte fixture/schema for [0,127,128,255] plus a Rust law in the existing receive test module. It checks actual sequence length, original octet trait-object pointer equality, numeric values, out-of-range refusal, zero physical query allocations/releases and funded terminal retirement. The retained dsl-bytes-oracle.mjs independently validates the schema and expected sequence through Ajv2020, Lodash and Buffer; managed Bun passed `[DEBUG] {"octets":4,"expected":[0,127,128,255]}`.

No new Cargo job was launched. Backend was notified ready for the sole coordinated wire-core/DSL-model owning suite. Actual Rust execution of this newly added law remains pending that suite. Files: schema/receive source, existing receive Rust test, tests/fixtures/bytes JSON and tests/schema/bytes JSON (actual repository emoji paths retained on disk).

Backend launched sole coordinated release suite88804 for replication-wire-core+dsl-record-model --lib after both source repairs were ready. Actual named direct text ordinal and byte projection results are pending; this agent did not launch or own a competing job.

Suite88804 actual DSL result: direct text ordinal and borrowed paged-octet laws passed; 14/15 DSL tests passed. Sole failure concerned selected closure semantics in a distinct intrinsic receiver law; diagnosis and canonical selected-action repair are recorded in intrinsic-close-2026-10-28.md. Wire test execution was skipped by Cargo fail-fast, so no wire pass is claimed.

Authoritative coordinated86170 terminated0: DSL16passed and wire49passed, no failed/filtered. Actual direct text ordinal and paged-octet pointer laws plus wire full-identity/nonzero-offset refusal passed. Outside-wire fixture completion still relies on each relevant owning suite being mounted; the whole-repository source completeness scan is not represented as compiling every OS/tool-run test. Next sole backend build81996 is live.
