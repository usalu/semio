# Independent Neutral Value Type Source Audit

Read-only source/provenance audit; no Cargo, rustc or tests were run by this worker. The source cohort is being edited concurrently by Root. Findings refer to the observed hashes below.

## Exact original capture verification

All15 full authored input blocks independently reconstitute their declared byte counts and SHA-256 values, totaling484767 bytes. This validates the durable original input witness, not runtime.

| Input | Bytes | Original SHA-256 | Observed current SHA-256 | Full bytes unchanged |
|---|---:|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs` | 128775 | `d960ab80d8b545649f24ec23cfb60b061249c3b032a139c473991efc9d737e4b` | `0e1fb38c6685e490c11253bd4fa72b0808097abe52a8cd3c106c1035e48260eb` | false |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs` | 13914 | `9aad5bf0d548a03b678c597623ec8d631a47b58b782c93088b2854014601748c` | `9aad5bf0d548a03b678c597623ec8d631a47b58b782c93088b2854014601748c` | true |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust/Cargo.toml` | 1187 | `26da977ee82c0ad87eab27f3935f048fba826cce9f8fdcc2e9c684fba4ee34d0` | `26da977ee82c0ad87eab27f3935f048fba826cce9f8fdcc2e9c684fba4ee34d0` | true |
| `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml` | 1981 | `ba2b9dc265c46784098448e55c3f6fbadc357e820288938bff75052650863cd2` | `0e6c08ace00b2333aaa97b2b887d638e164764661719067d753bfdea1111803d` | false |
| `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/🦀️.rs` | 671 | `7215a515fd2d8fdbbd9ca50a6590b56b522d57a87fc7b00015a141cb56d59c7e` | `9116f2cfbbd1d441df1411c705420a80abc9669df267535d5b6dbe2ada86593d` | false |
| `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📜️script.ts` | 2450 | `671eb4fa2443d9c8f08e8ece07be127833cb158d1c873a1947b343f729f1014b` | `39d179e6ed7ec78da76f5bf953fca4438c943fadd71bb4245ddd8f908daa5449` | false |
| `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📋️project.json` | 1592 | `1c80a474365ea37e0259c3b034ca5e8434efb41ce17bcc8c09160609e8c95515` | `ae94cbe08e09c9d95a269a1605a50f2729aba2a3937d7a8d5c7462edd4f00397` | false |
| `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/package.json` | 202 | `0ffdc52dd3fadc49e05fa87d5d99f163542eead6137a7a742582bc845f5d7f24` | `b978b61b6fd11e91845c32bb721e65c24ee7bc0fd3019e77010a47ebf94c2b5f` | false |
| `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs` | 29577 | `cbaf1577e77e9661ccc8b6d724b084841cfd13c095856d5755f2d7b269b76f87` | `4e6d71b5fd5faa5914d2c05f556f285d6fab83d6e7382a1b1528ce321e10d565` | false |
| `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs` | 89591 | `64552f21a7ed7f1f9fdf654646874a19007b4350f62586c5eb16c1148737f484` | `6745112636fd365746c1c8d585ba4ea242956dbc8b26ee81a79d61e5d381cd03` | false |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️.rs` | 30037 | `f48b9eb010e7da598eeef60c8dd663bfc08af66d0eaf2df58050cad0f2fbba2d` | `dcb25aef64a0c1f6baf4ebf645965e983068fd51f66152e95165bcd91b73b128` | false |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/🦀️.rs` | 9703 | `484003e849bdfae3f62237095b2ba1c947d02da0ed22285e99b631de4ac07d6b` | `feb32240bc31dd7709655ae4ac2f743f9496adc9e78cfb691f55e808e7eb1333` | false |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🦀️.rs` | 25828 | `8291e22a93d197dba63591b6b30117e307e337b238736528abeef14ca6998558` | `1790e8da674eb208ee31ddb10717a52fb329ba7267f26d65a817c4f38e5fb603` | false |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs` | 40183 | `ef5a898caa3c5ed33b64365cbdc7cc06ae92925e8510a5b6cd7fdf6785063b77` | `33572058ca2a673b7abd49a373d67abfd1d24888b36b6b954f4d91162f7e6712` | false |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs` | 109076 | `a266704d641a8628cb83b80bdb9be9968b7ccc25461a6ae351d2d92cd5a4eb81` | `aea47bfdbe6f49d3be54b8fb1f2469bc94633049189a467545f18478676e7808` | false |

## Concrete classification defect

The original Neural ValueType::Decimal called Atom::as_f64().is_some(). The original Atom::as_f64 includes Boolean(true/false) as1.0/0.0, Integer and Decimal. Initial neutral Rust and TypeScript classifiers accepted only Integer/Decimal, and the independent AJV schema was authored with the same omitted Boolean case. This was a real preserved-semantics gap, not a compiler limitation. It also affects Graph Bool→Decimal through Graph's existing concrete-value projection. Root was notified immediately; no production source was changed by this auditor.

Observed63-row fixture mismatches against the original source semantic truth table: []. The remaining pairs match the old borrowed classification truth table. This comparison evaluates the finite source-defined model; native execution is separately required.

## Borrowed classifiers and Graph-specific behavior

Neural Value::kind borrows Dictionary::schema() and does not construct strings/dictionaries/carriers. Dictionary::schema continues borrowing a string-valued $schema, preserving missing/nonstring schema→None. Integer remains Integer; Graph Number remains Decimal even when numerically integral. Graph Array/Object map to Null after preserving its existing special cases.

Graph's original Any early return must continue accepting Null/Array/Object. Its original Object+Schema early return accepts any Object for any Schema identifier; List does not accept Array or Object. Those branches were observed retained. Recommend an independently authored7types×6PropertyValue variants native table through the actual Graph matcher, including Boolean→Decimal, Any→Null/Array/Object and Schema→Object positives and Integer→Number/List→Array negatives. The generic63 ValueKind corpus alone does not prove this Graph projection.

## Test-only Serde boundary

The initially identified cfg(test) cross-crate Serde concern is addressed in current source by private Neural value_type_reference with independent tagged ReferenceType, explicit conversions and FieldSpec's cfg(test) serde(with). It retains the existing Serde oracle without a public Serde API on neutral ValueType. The reference type is private, the containing module is cfg(test), and only its test module signatures mention Serde traits. It is not a runtime forwarding API.

## API and dependency observations

The neutral ValueType/ValueKind API uses only first-party DslValue/ToValue/FromValue/ValueError plus standard String/Box/borrowedstr. No product import, foreign runtime API type, public Graph/Neural forwarding declaration or new runtime dependency is present in the observed canonical owner. The graph manifest now has a private direct semio_framework_value import and its Neural Cargo edge is removed; its separate OS provider edge remains and prevents whole Graph product-deletion proof. Full mounted Cargo participation checks and native selected consumer builds belong to Root/Native and are not inferred from these source reads.

## Strict wire correction boundary

Old ValueType::FromValue ignored unknown/extra object fields; the new canonical codec explicitly rejects unknown keys, wrong cardinality and duplicate fields. This is an intentional stricter wire contract and must be recorded separately from classification/body preservation. Graph still owns its actually authored bare-string/schema-object manifest dialect. Native duplicate-field tests and independent AJV cases exist but have not been run by this auditor.

The original specific implementation/law bodies should remain unchanged except actual type provider imports, owner projection and the private test-only Serde hook. Exact current source comparison follows after Root signals the final classifier correction. No whole native or deletion closure is claimed.

## Exact fresh byte transformation check

The following actual current sources independently reverse to their full saved original bytes after only the stated owner-binding/classifier replacement. This proves source-body preservation, not native compilation.

| Source | Exact full-byte projection |
|---|---|
| `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs` | true |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️.rs` | true |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/🦀️.rs` | true |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🦀️.rs` | true |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs` | true |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs` | true |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs` | true |
| `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs` | true |

For the six specific direct consumers, only the ValueType import/provider path changes. For Neural, the inverse removes the new direct import, borrowed classifier and private Serde hook, restores the extracted original type block, and restores the one call-site classifier argument. For Graph, only its old product imports and the two classifier/projection functions are replaced. All other original bytes, including evaluation, registry, retirement, manifest dialect conversion, Any and Schema+Object branches, remain identical. Neural retirement source itself remains byte-identical to its captured input.

## Fresh owner/provider and Graph witness observations

Actual Graph native child invokes the real private Graph matcher for independently authored graphCases, retaining PropertyValue conversion. Native execution remains pending. Current Graph vectors: 63; values: 9.

The six selected specific manifests each name the exact neutral Value package through an ordinary direct dependency. This is a selected physical roster, not a whole multiply-mounted graph census.

| Authored manifest | Canonical ordinary dependency |
|---|---|
| `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/Cargo.toml` | semio-framework-value; true |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust/Cargo.toml` | semio-framework-value; true |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/📦️packages/🦀️rust/Cargo.toml` | semio-framework-value; true |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust/Cargo.toml` | semio-framework-value; true |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/Cargo.toml` | semio-framework-value; true |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/Cargo.toml` | semio-framework-value; true |

Selected physical mounts: Graph package root mounts its manifest child; Brep extension mounts mesh; Jack artifact source mounts its standards wire-runtime; Session package declares its library and three explicit integration test paths. These observations do not replace complete per-context graph participation.

## Host-object strict-reader finding

Initial TS readValueType allowed an inherited/non-enumerable kind plus one own enumerable of key to pass primitive keys.length===1. Public unknown input requires the exact own enumerable kind/of shape independent of JSON-only fixtures. Root received the concrete counterexample. This is a source-branch finding; no test/runtime reproduction was run by this auditor.

Fresh independently recomputed Graph original-admission table: all63 authored rows match, including Boolean→Decimal, Any→Null, any Object→Schema and integral Number→Integer refusal. Observed source-model mismatches: []. Native execution remains pending.

## Corrected reader and source-only audit closure

Fresh observed TS reader now requires an own enumerable data descriptor for kind and of, rejects concealed/symbol keys via own-key cardinality, does not invoke kind/of accessors, and receives a first-party caller checkpoint interface for iterative reading/construction. Runtime hostile vectors include inherited/concealed/accessor/cycle inputs and cancellation. This resolves the specific earlier source-branch counterexample. No new external runtime API or default control was introduced. Root owns actual test execution.

The actual original Neural Decimal/Boolean acceptance is now restored in both implementations and the independent AJV expected rule. Both independently recomputed source tables have zero mismatches:63 neutral/Neural pairs and63 real Graph pairs. Original15input witness integrity and all eight full source inverse checks remain valid. The source audit has no remaining concrete established defect in this bounded cut.

Strict-wire correction is explicit: original emitted eight shapes are retained; current closed JSON refusal vectors number13. Recursive ordinary Rust FromValue and complete controlled native codec/performance proof remain outside this read-only result and are explicitly unproved in Root's ledger. No whole product-absence, compiler or native pass is inferred.

| Observed source | SHA-256 |
|---|---|
| `🧰️framework/🔨️modules/🌱️value/🏷️type/🦀️.rs` | `b7c55b65b35982a8633e109df1021171109f6af5384bdde80b2c0a1f03fe2746` |
| `🧰️framework/🔨️modules/🌱️value/🏷️type/🟦️.ts` | `e6e4b63b09a0983ca5f6b0f80eaacece51109d8a79422b09ea191a86b6d78c14` |
| `🧰️framework/🔨️modules/🌱️value/🏷️type/🧫️fixtures/🔣️.json` | `7907552c5d1232813cf047d6625794ff2f2a68499a28c45aca6b4da37e98a398` |
| `🧰️framework/🔨️modules/🌱️value/🏷️type/🧬️schema/🔣️.json` | `6221dfc8b8b4f1fcb205e0e9f3453b644cf7f75841c02874e322befe2e438465` |
| `🧰️framework/🔨️modules/🌱️value/🏷️type/🧪️tests/🟦️.ts` | `1a6640cde955db9bb831ab30189abd4bfebf6d065fe2c162b82a73d7ac258f81` |
| `🧰️framework/🔨️modules/🌱️value/🏷️type/🧪️tests/🦀️.rs` | `f252bd1d967c2165510f0e81ced7799e60edba52dadca034bba52286bf9c0e48` |
| `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧪️tests/🏷️type/🦀️.rs` | `61dbcd79c8a923c9a124b474537da372b50b48e2ce41a78c47ed74522325a666` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧪️tests/🏷️type/🦀️.rs` | `03cf3310e340357e84dca4f392db842c3f299127447c8b5996a65061fc6d0522` |

One attempted read-only manifest analysis used unavailable Python tomllib and terminated before publishing a provider result; existing Bun/@iarna/TOML then independently resolved all six selected providers. No dependency was installed and no production source was changed.

## Final additive native mount epoch

Fresh Graph full-byte inverse remains exact after also removing the one additive cfg(test) type_tests mount. Its original tests mount and every original Graph implementation/body byte remain intact outside the private import and matcher/projection extraction. Current Graph SHA-256: `556a965dac9ff37b261287a6dc68946f8a4207ef03e84ad5b3a42051b19e65b2`. Actual additive native body invokes the original matcher through7×9Graph values; it neither weakens the old registered target nor substitutes a test-only fake implementation.

Root reports final registered portable GREEN5/265/313msBun/2.9sNx. This audit did not execute or independently infer that result; Root owns its retained log. Required original/native suites remain queued behind normal preparation. The mandatory control type and source-level own-data-descriptor guards are first-party; no remaining concrete source defect is established by this bounded review.
