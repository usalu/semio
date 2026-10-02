# Owned Native Schema Producer Ports

The mandatory shared metadata port is concrete: RecordSpecProducer carries ordinary/decoding/encoding factories. Controlled metadata owns its field frontier, field keys, keywords and boxed shapes through NativeSchemaControl before allocation; an ordinary schema copy cannot stand in for this operation.

Home and Space literal derived snapshots now pass Self::__dsl_spec_producer() to decode_sqlite_snapshot_record_native. OBJ native_pack::spec wraps its actual derived ObjSnapshot; STL and PLY native_pack::spec wrap their actual owned derived Snapshot records. Each exposes spec_producer() forwarding the corresponding genuine generated __dsl_spec_producer(), and the SQLite hook now passes that mandatory value. No manual runtime schema reflection, ordinary clone, metadata fallback or alternate layout was added.

The prepared, unmounted Wires flat Document is derived and exposes its genuine producer; its intrinsic Octets custom field explicitly implements controlled Bytes64 shape admission with a checkpoint and no allocated schema child. Wires remains on the old native codec until the staged old-codec fidelity/capability laws genuinely run RED.

Six Writer/Wires ordinary mutation/editor variant consumers and the OBJ ordinary mutation consumer now invoke `(spec_fn.ordinary)()` or the explicit `.ordinary` field. These are ordinary declared transport paths, not controlled snapshot bypasses.

IO reports actual shared compiler/public producer readiness: 15 native laws executed,14 GREEN, sole intended recursive Pack hash ordinary-metadata RED. The sole warm Wires baseline retry uses quick level and the existing json-paired-target/json-paired-build cache, with no cold native lane. Its result remains pending; earlier preassert failures are retained as prerequisites.

That sole retry completed before owner assertions on the shared in-flight Pack schema graph reserve_map helper: E0599 at 🎒️pack/🌱️value/🏭️schema/🦀️.rs:68, HashMap::try_reserve missing K:Eq+Hash. IO confirmed the exact generic bounds are already repaired in current source. The sampled compiler draft is a prerequisite, not a Wires fidelity RED; old Wires native codecs remain intact and the prepared logical modules remain unmounted.
