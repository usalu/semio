# Inference Runtime Shape Census

Read 894 tracked plugin/manifest Rust files. This is code/schema inspection, not compiler evidence. The WIT request includes document-schema fields that the existing Host/MCP request DTOs do not carry; actual job-reader behavior must decide the canonical change.

## WireArtifactInferenceRequest

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🔌️program/🛬️lifted/💡️inference/🦀️.rs:133`

```rust
pub struct WireArtifactInferenceRequest {
    pub wire_version: u32,
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub revision: u64,
    pub generation: u64,
    pub source_dialect: String,
    pub policy: Vec<u8>,
    pub budgets: WireArtifactInferenceBudget,
    pub retained: RetainedCloneGrant,
    pub cancellation_id: String,
    pub previous_state: Option<Vec<u8>>,
    pub requested_cache_mode: WireArtifactInferenceCacheMode,
    pub canonical_payload: Vec<u8>,
    pub dependencies: Vec<(String, Vec<u8>)>,
}
```

## WireArtifactInferenceResult

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🔌️program/🛬️lifted/💡️inference/🦀️.rs:169`

```rust
pub struct WireArtifactInferenceResult {
    pub retirement_progress: RetainedCloneProgress,
    pub wire_version: u32,
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub revision: u64,
    pub generation: u64,
    pub source_dialect: String,
    pub policy: Vec<u8>,
    pub budgets: WireArtifactInferenceBudget,
    pub retained: RetainedCloneGrant,
    pub previous_state: Option<Vec<u8>>,
    pub requested_cache_mode: WireArtifactInferenceCacheMode,
    pub canonical_payload: Option<Vec<u8>>,
    pub dependencies: Vec<(String, Vec<u8>)>,
    pub diagnostics: Vec<WireArtifactInferenceDiagnostic>,
    pub provenance: WireArtifactInferenceProvenance,
    pub validity: String,
    pub quality: String,
    pub complete: bool,
    pub actual_cache_mode: WireArtifactInferenceCacheMode,
    pub cancellation_id: String,
}
```

## WireArtifactInferenceBudget

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🛬️lifted/💡️inference/🦀️.rs:282`

```rust
pub struct WireArtifactInferenceBudget {
    pub allocation_bytes: u64,
    pub work_units: u64,
    pub recursion_depth: u32,
}
```

## WireArtifactInferenceCacheMode

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🛬️lifted/💡️inference/🦀️.rs:291`

```rust
pub enum WireArtifactInferenceCacheMode {
    Cold,
    Incremental,
    Bypass,
}
```

## InferenceRouteRequest

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧭️routers/💡️inference-router/🦀️.rs:47`

```rust
pub(super) struct InferenceRouteRequest {
    pub(super) wire_version: u32,
    pub(super) owner: String,
    pub(super) artifact_kind: String,
    pub(super) artifact_schema: String,
    pub(super) artifact_schema_version: u32,
    pub(super) inference_schema: String,
    pub(super) inference_schema_version: u32,
    pub(super) algorithm_version: u32,
    pub(super) policy_version: u32,
    pub(super) revision: u64,
    pub(super) generation: u64,
    pub(super) source_dialect: String,
    pub(super) policy: Vec<u8>,
    pub(super) budgets: InferenceRouteBudget,
    pub(super) retained: semio_framework_value::retained_clone::RetainedCloneGrant,
    pub(super) cancellation_id: String,
    pub(super) previous_state: Option<Vec<u8>>,
    pub(super) requested_cache_mode: InferenceRouteCacheMode,
    pub(super) canonical_payload: Vec<u8>,
    pub(super) dependencies: Vec<(String, Vec<u8>)>,
}
```

## InferenceRouteBudget

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧭️routers/💡️inference-router/🦀️.rs:73`

```rust
pub(super) struct InferenceRouteBudget {
    pub(super) allocation_bytes: u64,
    pub(super) work_units: u64,
    pub(super) recursion_depth: u32,
}
```

## InferenceRouteCacheMode

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧭️routers/💡️inference-router/🦀️.rs:82`

```rust
pub(super) enum InferenceRouteCacheMode {
    Cold,
    Incremental,
    Bypass,
}
```

## InferenceRouteResult

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧭️routers/💡️inference-router/🦀️.rs:91`

```rust
pub(super) struct InferenceRouteResult {
    pub(super) wire_version: u32,
    pub(super) owner: String,
    pub(super) artifact_kind: String,
    pub(super) artifact_schema: String,
    pub(super) artifact_schema_version: u32,
    pub(super) inference_schema: String,
    pub(super) inference_schema_version: u32,
    pub(super) algorithm_version: u32,
    pub(super) policy_version: u32,
    pub(super) revision: u64,
    pub(super) generation: u64,
    pub(super) source_dialect: String,
    pub(super) policy: Vec<u8>,
    pub(super) budgets: InferenceRouteBudget,
    pub(super) retained: semio_framework_value::retained_clone::RetainedCloneGrant,
    pub(super) cancellation_id: String,
    pub(super) previous_state: Option<Vec<u8>>,
    pub(super) requested_cache_mode: InferenceRouteCacheMode,
    pub(super) canonical_payload: Vec<u8>,
    pub(super) dependencies: Vec<(String, Vec<u8>)>,
    pub(super) complete: bool,
    pub(super) actual_cache_mode: InferenceRouteCacheMode,
}
```

### Pending Payload Contract
Authored language-neutral cases before the runtime fix: pending null, pending candidate, complete empty payload, and rejected complete null. AJV and SQLite provide independent acceptance oracles; the native router law compares the first-party and serde decoders. Native validation remains pending.

Applied the host optional-payload result and completion validation. Removed duplicated artifact-schema echo comparisons. Aligned WIT inference records with the actual guest Rust wire: no phantom document-schema fields, and an optional result payload. Portable contract oracle: 1 test / 9 assertions passed. Native router laws are authored and not yet compiled.
