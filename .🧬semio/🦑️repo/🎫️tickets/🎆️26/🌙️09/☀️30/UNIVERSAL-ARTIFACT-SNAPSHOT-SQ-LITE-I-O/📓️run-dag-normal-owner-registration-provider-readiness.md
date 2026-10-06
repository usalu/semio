# Run and DAG Normal Owner Registration Provider Readiness

Source-only, unmounted proposal. No compiler or gate was executed; no runtime success is claimed.

The held `📥️inputs/run-dag-normal-owner-registration-providers-held.json` has two unique exact impl-header insertion guards. Existing owner codecs and schemas remain intact.

Store ArtifactPack at line 11040 takes `Option<(crate::os_io::Dialect, ArtifactCodec)>`. The already exercised first-party `store::io_schema::Dialect` is the exact alias (IO reexport line 21). NativeSnapshotRegistration at IO line 2177 is an assembly-plan entry, not this hook return type. Store bare at line 11564 constructs the actual owner capability with real mutation bounds. IO registration line 1905 atomically publishes document/native entries.

Run Snapshot ArtifactPack line 372 uses actual RunArtifact, crate::RunMutation, S_RUN_SCHEMA=os.run and os.run@1/*. Its existing public test already instantiates this exact bare owner/mutation codec and dialect. Existing Before: `📥️inputs/run-normal-owner-public-registration-demand-held.json`; selected top-level count 11→12. No named RunStore constructor was found. Generic ArtifactStore::new and hydration are confirmed consumers; do not invent a Run lifecycle wrapper.

DAG VCS ArtifactPack line 515 uses actual DagSnapshot, DagMutation, DAG_DOCUMENT_SCHEMA=dag.host_snapshot and dag.host_snapshot@1/*. Existing Before: `📥️inputs/dag-normal-owner-public-registration-demand-held.json`; selected top-level count 8→9. Existing create_dag_store at VCS line 479 calls DagStore::new(create_document_envelope(...)), then installs exact member owners, directly reaching the hook.

Store constructor line 17770 and persisted hydration line 296 consume the existing hook. Bare codec DSL/native construction uses current interfaces. Expected Before is hook assertion refusal; After is exact owner coordinate/schema/capability admission followed by both public file forms, neutral raw-word/full-owner equality, independent SQL edit and bounded retirement. These are two concrete owners, not a blanket hook for the 126 lexical Snapshot census. Source presence grants no runtime credit.
