# Native Schema Ownership Boundary

The existing `Shape::Record` and `Shape::Table` carry a bare `fn() -> RecordSpec`. Invoking that function constructs metadata Strings, vectors and boxes before a caller can admit them. A physical encoder may invoke the same producer for each nested value. Generated typed output and borrowed sorting do not cover those allocations.

The clean producer is one explicit `RecordSpecProducer` containing ordinary and genuinely controlled construction functions. `Shape::Record` and `Shape::Table` should own that producer rather than a bare function pointer. The ordinary parser remains a legitimate separate consumer; a controlled codec must invoke only the controlled function, with no ordinary fallback. A generated `__dsl_spec_controlled` uses literal known field/variant counts, before-allocation vector/box admission and controlled copies of each authored metadata string. Custom schema producers must author their own controlled construction.

Metadata needs one neutral allocation port shared by NativeDecodeControl and NativeEncodeControl. That port exposes admission, known work and cancellation; it does not infer schema or inspect snapshots. Nested schema references remain lazy, so recursive native types do not recursively materialize their complete metadata graph. The caller's cumulative ownership charges persist through schema construction, typed fields, physical bytes and envelope ownership.

Canonical schema hashes continue to describe semantic fields and shapes, without hashing callback addresses or encoding-control machinery. Existing composed Pack neutral goldens must remain unchanged when only the metadata ownership producer changes. Required runtime tests cover tiny pre-allocation refusal, large literal field-name interior cancellation, nested lazy schemas and exact schema/hash parity against independent Blake3 and current declared protocol fixtures.

This is a planned producer change. Current source still has bare RecordSpec functions, and current output work must not claim that metadata allocation is already controlled.

## Settled Constructor Interface

The generated owner method is `fn __dsl_spec_controlled<C: dsl::NativeSchemaControl>(control: &mut C) -> Result<RecordSpec, String>`. The schema port is statically implemented for actual NativeDecodeControl and NativeEncodeControl, preserving the same cumulative ownership charges, nested known stages, before-allocation admission and cancellation. RecordSpecProducer carries the ordinary constructor and separately monomorphized controlled input/output pointers to this one authored generic implementation. A lazy nested reference owns the producer; it does not recursively construct a full schema tree.

This interface has been communicated to the parent owner. It is not yet mounted or dispatched. Existing plain schema constructors still allocate outside their caller budget. Required neutral runtime proof must include known vector frontier refusal before allocation; complete literal label/keyword fidelity against independent schema/hash oracles; cancellation during actual metadata ownership; exact ordinary/controlled schema parity; and composed graph/Pack goldens remaining unchanged. No implicit ordinary factory fallback is permitted.


## 2026-10-02: Explicit producer admission baseline

The schema-owned `🏭️producer` module now declares `NativeSchemaControl`, implemented by the canonical input/output controls, and `RecordSpecProducer` with three mandatory owner factory pointers. Its input/output methods deliberately refuse execution until the neutral runtime baseline is observed. The public kernel admission integration mounts three producer laws: literal key/shape/optional identity with Ajv+BunSQLite and independent BLAKE3; refusal before a 1,024-field frontier plus interior long-keyword copy cancellation; exact 1,024-field workload cancellation. The portable fixture and strict schema are colocated under this module. These are authored tests, not executed successes.

This stage does not change `Shape::Record/Table/Statements`, invoke controlled metadata from the physical readers/writers, or activate output factory dispatch. Those remain required integration work; ordinary lazy factories currently still own their metadata without the caller admission control.

The current focused reference/Text invocation (session 42608) reached compilation but stopped before assertions on six newly mounted metadata test factory lifetime coercions. A generic function specialized with `Native*Control<'_>` is not a sufficiently general late-bound factory; each mandatory producer pointer now receives a zero-capture wrapper that delegates to the same generic owner constructor. This was an owned test prerequisite failure, not a producer runtime RED or reference/Text regression. The follow-up run includes the explicit producer baseline.

## Runtime producer RED and reference/Text evidence

Registered public run33995/Nextest64bb1b3f-10e3-4579-b63d-f00bac60dce9 executed all13 selected laws:8passed,5failed,31filtered,1.359s assertions/3m12s Nx. All four Child controlled-output laws, both Ref controlled-output laws, derived typed projection and long physical Text ownership/interior cancellation passed. Four metadata laws genuinely failed the explicit missing producer dispatch, and the Text boundary corpus found a controlled lexer rejection of nonhex `nan64_`/`nan32_` identifiers. The shared lexer now recognizes only full-width hexadecimal words, matching the ordinary lexer, and each producer dispatch invokes only its declared controlled factory inside an isolated known stage/depth bound. Neither change has yet been rerun. Lazy physical factories still require integration.

## Mandatory Lazy Producer Propagation

The recursive Text law executed RED with ten ordinary metadata factory calls (measurement plus emission), despite controlled native field construction. This exposed a metadata allocation gap. `Shape::Record` and `Shape::Table` now carry `RecordSpecProducer`; statement labels carry that same explicit producer. Each producer declares ordinary, controlled input, and controlled output constructors. There is no callable compatibility alias and no controlled fallback to the ordinary constructor.

Derived records expose `__dsl_spec_producer()` and generic `__dsl_spec_controlled<C: NativeSchemaControl>`. Literal owner constructors admit the exact field and label frontiers before allocation, copy authored keys under the same cumulative limit, and retain lazy recursive edges. Typed physical Text resolves those declared controlled factories. Pack field materialization resolves controlled factories through its existing materialization authority. Top-level `decode_sqlite_snapshot_record_native` now also requires the producer value, admitting its metadata before parsing.

The first mandatory-producer public quick attempt stopped before assertions at seven propagation errors: five viewport module qualifications and two parenthesized operation variant calls. Those exact prerequisites are repaired. The fresh same-selector quick run is pending. A new physical Pack recursive law remains intentionally capable of exposing ordinary schema-hash expansion; Text evidence does not establish Pack hash or output compliance.

Owned paths in this step: DSL root/schema/producer/derive/physical readers and emitter, schema producer neutral tests, Pack value materialization, viewport metadata, Store OwnerRef/LinkPin/ArtifactLink metadata and ordinary operation variant callers, shared native record decoding helper and public buffer fixtures, tool-run literal nested metadata, plugin empty config and retained-window producer consumers. Retained-window config currently refuses controlled metadata explicitly when its owner has no producer; this config boundary is not counted as artifact snapshot coverage. Root owns ArtifactChild and WAV/AVI/BCF manual metadata. Rust worker owns Flow/Process and collection manual metadata. TS worker owns its geometry/Wires manual roots.

## Public Mandatory Producer Evidence

The current public quick lane executed 15 selected laws (31 filtered) with Nextest ID `14315807-237b-4b23-ba94-0cf9cddbd958`: fourteen passed and one failed. The recursive physical Text law now records zero ordinary schema calls. Generated four-field metadata, literal references, child metadata, actual child controlled output, typed projection, and long Text passed. The new recursive physical Pack law genuinely failed because manifest schema hashing invoked an ordinary metadata producer. Runtime assertions took 0.572 seconds; the registered uncached task took 24.2 seconds.

The Pack repair is authored but pending runtime proof. It loads nested schemas only through the active input/output metadata port, admits graph frontiers before ownership, sorts in place with cancellation, preserves ordinary stable source ordering for tied field/statement labels, refines structural classes using incremental fingerprints plus exact controlled equality on collisions, and moves the selected owned fields into the minimal graph. Incremental hashing writes the established canonical byte protocol directly into the first-party BLAKE3 state in bounded chunks. It does not materialize a canonical-byte carrier, and both control directions are exercised against the independent BLAKE3 oracle. Literal key hashing has an interior cancellation law; graph frontier refusal and known-work cancellation are also authored.

This remains separate from genuine controlled physical Pack output, container compression/output ownership, envelope output, and the strict SQLite import factory dispatch. Those are unfinished; no earlier preflight/ordinary encoder result proves interior output control.
# Current Public Runtime Proof

The IO agent reports the registered uncached public metadata/hash route executed all sixteen selected tests and passed all sixteen, Nextest `bd382ee4-32d3-4ce2-ac9c-95d9f2c66117`, 0.566 seconds of assertions and 2 minutes 33 seconds Nx. Recursive physical Text and Pack decoding used the declared controlled metadata with zero ordinary-factory invocation; known-frontier and incremental schema-hash cancellation passed. This proves those selected shared decoding/metadata laws. Physical Pack output and factory output activation remain unfinished and are not covered by this success.

### Current Controlled Metadata and Structural Hash Runtime

The registered public quick admission route executed all 16 selected laws successfully (Nextest `bd382ee4-32d3-4ce2-ac9c-95d9f2c66117`; 0.566 seconds assertions, 2m33s uncached Nx). The retained output is `🗑️generated/native-output-controlled-schema-hash-proof.log`. Recursive Text and Pack admission use their declared controlled metadata producers exclusively. New keyed-frontier and streamed canonical hash cancellation laws run in both input/output controls, with independent Blake3/Ajv/Bun SQLite fixture oracles. The genuine previous Pack hash bypass RED is repaired.

Physical Pack output and erased output dispatch remain unfinished and are not inferred from these metadata/input laws. Terminal Pack field emission is being implemented separately; controlled Deflate output is delegated to the Rust codec agent with an explicit cumulative-allocation and actual probe/emission contract.
