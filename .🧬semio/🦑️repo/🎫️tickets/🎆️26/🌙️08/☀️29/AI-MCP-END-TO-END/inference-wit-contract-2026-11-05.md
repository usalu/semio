# Artifact Inference WIT Contract

Read 19 tracked WIT paths. Definitions below are repository-owned schema evidence; compiler/runtime behavior remains separately tested.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️types.wit:77

```wit
record artifact-inference-request {
    wire-version: u32,
    owner: string,
    artifact-kind: string,
    artifact-schema: string,
    artifact-schema-version: u32,
    document-schema: string,
    document-schema-version: u32,
    inference-schema: string,
    inference-schema-version: u32,
    algorithm-version: u32,
    policy-version: u32,
    revision: u64,
    generation: u64,
    source-dialect: string,
    policy: list<u8>,
    budgets: inference-budget,
    retained: retained-clone-grant,
    cancellation-id: string,
    previous-state: option<list<u8>>,
    requested-cache-mode: inference-cache-mode,
    canonical-payload: list<u8>,
    dependencies: list<tuple<string, list<u8>>>,
  }
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️types.wit:109

```wit
record artifact-inference-result {
    retirement-progress: retained-clone-progress,
    wire-version: u32,
    owner: string,
    artifact-kind: string,
    artifact-schema: string,
    artifact-schema-version: u32,
    document-schema: string,
    document-schema-version: u32,
    inference-schema: string,
    inference-schema-version: u32,
    algorithm-version: u32,
    policy-version: u32,
    revision: u64,
    generation: u64,
    source-dialect: string,
    policy: list<u8>,
    budgets: inference-budget,
    retained: retained-clone-grant,
    previous-state: option<list<u8>>,
    requested-cache-mode: inference-cache-mode,
    canonical-payload: list<u8>,
    dependencies: list<tuple<string, list<u8>>>,
    diagnostics: list<inference-diagnostic>,
    provenance: inference-provenance,
    validity: string,
    quality: string,
    complete: bool,
    actual-cache-mode: inference-cache-mode,
    cancellation-id: string,
  }
```
