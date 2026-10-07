# Kernel Corpus Closure

Before editing, actual isolated Nx paged-history command failed ENOENT on the retired corpus schema. The six DB owners and VCS history visibility published fixed test envelopes, not runtime schemas. Removed only whole-corpus admission; all SQLite, SHA256, array/deque, structuredClone, byte/grant/lifecycle/native predicates remain. Preserved actual writer operation assertions; removed the orphan schema enum used only by the wrapper, which had no production declaration or codec. Extracted the real generic Edit wire contract into neutral replication/mutation, where Edit<Op> lives; branch vectors validate actual edits with Ajv and native FromValue. The DB artifact record was a test summary rather than the actual DurableOwnedGroupJournalRecordV1 wire contract, so it was not promoted as a fake domain schema.

Exact removed admission statements:

```ts
const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/📸️paged-history-stack/🔣️.json"), "utf8"));
```

```ts
const admit = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
```

```ts
assert(admit(fixture), JSON.stringify(admit.errors));
```

```ts
const validate = ownedExport(this.repoRoot, "db.engine", "HistoryCompletionV1");
```

```ts
assert(validate(fixture), JSON.stringify(validate.errors));
```

```ts
const validate = ownedExport(this.repoRoot, "db.engine", "CatalogReadOwnershipV1");
```

```ts
assert(validate(fixture), JSON.stringify(validate.errors));
```

```ts
const validate = ownedExport(this.repoRoot, "db.engine", "CapabilityCompletionV1");
```

```ts
assert(validate(fixture), JSON.stringify(validate.errors));
```

```ts
const validate = ownedExport(this.repoRoot, "db.storage.writer", "WriterAuthorityV1");
```

```ts
const writerDeferredWakeSchema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔔️deferred-wake/🔣️.json"), "utf8"));
```

```ts
const validateWriterDeferredWake = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(writerDeferredWakeSchema) as SchemaCheck;
```

```ts
const validateRemote = ownedExport(this.repoRoot, "db.storage.writer", "WalWriterFenceV1");
```

```ts
const validateMemory = ownedExport(this.repoRoot, "db.storage", "MemoryBackingV1");
```

```ts
const validatePoolUse = ownedExport(this.repoRoot, "db.storage", "BackendPoolUseV1");
```

```ts
const validateDirectory = ownedExport(this.repoRoot, "db.storage", "DirectoryDurabilityV1");
```

```ts
const validateOpen = ownedExport(this.repoRoot, "db.wal", "OpenRejectionV1");
```

```ts
assert(validate(fixture), JSON.stringify(validate.errors));
```

```ts
assert(validateWriterDeferredWake(writerDeferredWake), JSON.stringify(validateWriterDeferredWake.errors));
```

```ts
assert(validateRemote(remoteFixture), JSON.stringify(validateRemote.errors));
```

```ts
assert(validateMemory(memoryFixture), JSON.stringify(validateMemory.errors));
```

```ts
assert(validatePoolUse(poolUseFixture), JSON.stringify(validatePoolUse.errors));
```

```ts
assert(validateDirectory(directoryFixture), JSON.stringify(validateDirectory.errors));
```

```ts
assert(validateOpen(openFixture), JSON.stringify(validateOpen.errors));
```

```ts
const validate = ownedExport(this.repoRoot, "db.wal", "CommittedTransactionsV1");
```

```ts
const validateFaults = ownedExport(this.repoRoot, "db.wal", "FailStopV1");
```

```ts
const validateDecoder = ownedExport(this.repoRoot, "db.wal", "RetainedDecoderV1");
```

```ts
assert(validate(fixture), JSON.stringify(validate.errors));
```

```ts
assert(validateFaults(faults), JSON.stringify(validateFaults.errors));
```

```ts
assert(validateDecoder(decoder), JSON.stringify(validateDecoder.errors));
```

```ts
const validate = ownedExport(this.repoRoot, "db.compact", "CommittedEffectsV1");
```

```ts
assert(validate(fixture), JSON.stringify(validate.errors));
```

```ts
const validate = ownedExport(this.repoRoot, "db.engine", "ShutdownV1");
```

```ts
assert(validate(fixture), JSON.stringify(validate.errors));
```

```ts
const validate = ownedExport(this.repoRoot, "db.engine", "DocumentMountV1");
```

```ts
const mountedPoolUseSchema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔐️pool-use/🔣️.json"), "utf8"));
```

```ts
const validateMountedPoolUse = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(mountedPoolUseSchema);
```

```ts
assert(validate(fixture), JSON.stringify(validate.errors));
```

```ts
assert(validateMountedPoolUse(mountedPoolUse), JSON.stringify(validateMountedPoolUse.errors));
```

```ts
const validate = ownedExport(this.repoRoot, "db.artifact", "DurableGroupJournalV1");
```

```ts
assert(validate(fixture), JSON.stringify(validate.errors));
```

```ts
const validate = ownedExport(this.repoRoot, "db.wal", "TailOnlyRecoveryV1");
```

```ts
const validateFailStop = ownedExport(this.repoRoot, "db.wal", "FailStopV1");
```

```ts
assert(validate(fixture), JSON.stringify(validate.errors));
```

```ts
assert(validateFailStop(failStop), JSON.stringify(validateFailStop.errors));
```

```ts
const validate = ownedExport(this.repoRoot, "db.wal", "SegmentCapacityV1");
```

```ts
assert(validate(fixture), JSON.stringify(validate.errors));
```

```ts
const schema = JSON.parse(readFileSync(join(root, "🧬️schema/🔣️.json"), "utf8"));
```

```ts
const validate = new Ajv2020({ strict: true, allErrors: true }).compile<RetainedCloneNestedFixtureV1>(schema);
```

```ts
const mapSchema = JSON.parse(readFileSync(join(mapRoot, "🧬️schema/🔣️.json"), "utf8"));
```

```ts
const validateMap = new Ajv2020({ strict: true, allErrors: true }).compile<RetainedOrderedMapPagingFixtureV1>(mapSchema);
```

```ts
const preparationSchema = JSON.parse(readFileSync(join(preparationRoot, "🧬️schema/🔣️.json"), "utf8"));
```

```ts
const validatePreparation = new Ajv2020({ strict: true, allErrors: true }).compile<RetainedClonePreparationFixtureV1>(preparationSchema);
```

```ts
const pagedSchema = JSON.parse(readFileSync(join(pagedRoot, "🧬️schema/🔣️.json"), "utf8"));
```

```ts
const validatePaged = new Ajv2020({ strict: true, allErrors: true }).compile<RetainedPagedListCopyFixtureV1>(pagedSchema);
```

```ts
const pagedOwnerSchema = JSON.parse(readFileSync(join(pagedOwnerRoot, "🧬️schema/🔣️.json"), "utf8"));
```

```ts
const validatePagedOwner = new Ajv2020({ strict: true, allErrors: true }).compile<RetainedPagedOwnerFixtureV1>(pagedOwnerSchema);
```

```ts
assert(validate(fixture), JSON.stringify(validate.errors));
```

```ts
assert(validateMap(mapDescriptor), JSON.stringify(validateMap.errors));
```

```ts
assert(validatePreparation(preparationFixture), JSON.stringify(validatePreparation.errors));
```

```ts
assert(validatePaged(pagedFixture), JSON.stringify(validatePaged.errors));
```

```ts
assert(validatePagedOwner(pagedOwnerFixture), JSON.stringify(validatePagedOwner.errors));
```

```ts
const validator = ownedExport(repoRoot, "directory", "DirectoryEventPageBootstrapTraceV1");
```

```ts
assert(validator(trace), JSON.stringify(validator.errors));
```

```ts
const validate = ownedExport(this.repoRoot, "db.engine", "ThroughputV1");
```

```ts
assert(validate(throughput), `throughput fixture: ${JSON.stringify(validate.errors)}`);
```


Removed 16 stale corpus identity markers/documentation references. Actual isolated Nx paged-history check now passes: three deque vectors and 59 canonical edit/domain observations with actual Ajv/SHA/native-source oracles.


Final actual isolated Nx behavior batch: 15/15 commands passed. Runtime logs confirm CRC/hash-chain recovery prefixes, SQLite ownership/publication interleavings, 6decisioncases+6witnesscases+4recoveries, clone2MiB/cancellation9/map71/growth49/preparation6/list513/text5440/bytes6145/map70, and11bootstraporderingchecks. Before success, corrected real current WALpoll-close/source-test ownership and the static durable journal digest to match canonical declared bytes; all original semantic predicates remained and writer release terminal ownership gained an explicit assertion.

Five-language host parity also completed through correctly project-scoped Nx:25passed/0failed/0errored,50of50paritycomparisons. Initial shared native kernel build failed18canonical I/O import errors; no native kernel success claimed until focused rerun completes.

## Actual Native Kernel Gate

The registered kernel-native task exited0 after42minutes35seconds. The real Cargo-captured executable ran all four exact named Rust laws: portable paged-history traversal, required branch provenance wire, canonical Edit digest chains, and Unicode retirement under single-byte grants. Native receipt records four assertions and the actual captured binary hash14c3b4b72d3d3e1be33ba7a6ad0a5c1e9a8851a15819b0213c944f78cbe80774. Its three neutral vectors also passed independent Array/Ajv comparison. No native result is inferred from source inspection.
