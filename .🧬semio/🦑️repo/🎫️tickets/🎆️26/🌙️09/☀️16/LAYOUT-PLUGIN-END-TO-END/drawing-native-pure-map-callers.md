# Pure Native Map Callers

Hand-migrated SchemaId, PayloadHash and LocalizedLabel receiver initializers to canonical pure NativeReceive::receiver. Their existing mappings move retained String/array owners allocation-free; no sorting, cloning, constructor work or authority is hidden behind the initializer. Existing owning laws remain unmodified and require their normal API migration before execution. No runtime or compilation was run. DAG expanded paths and Neural Atom remain separate concrete funded-body blockers documented in the provider census.
