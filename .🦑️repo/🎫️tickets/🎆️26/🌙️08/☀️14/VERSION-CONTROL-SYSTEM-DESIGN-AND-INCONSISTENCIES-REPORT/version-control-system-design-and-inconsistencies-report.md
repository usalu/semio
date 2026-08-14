# 🧾 Framework Version Control System Design & Inconsistencies Report

## 📌 Executive Summary

This report presents an in-depth architectural analysis of the Version Control System (VCS) designed and implemented strictly within the **Framework** (`🧰️framework`), covering application data, document versioning, spaces, and composable artifacts. External systems (such as Git integration and the legacy `compose` subsystem) are excluded per specification.

The framework's VCS architecture is centered on a generic, content-addressed version-graph algebra (`💻️os/🌿️vcs`) integrated with a local-first artifact store (`💻️os/🏪️store`). It handles document state, history graphs, composite artifact pinning, alternative branching, and CQRS mutation diffing across volatile `Draft` and persistent `Asset` artifacts.

---

## 🏛️ Framework VCS Architecture & Design

```mermaid
graph TD
    subgraph Framework OS Level ["🧰️framework/🛍️products/💻️os"]
        Space[Space Container 🪐] --> Artifact[Artifact / Document 🗿]
        Artifact --> Draft[Draft (Volatile) 📝]
        Artifact --> Asset[Asset (Persisted) 💾]
        
        Engine[Engine ⚙️] --> PackBuffer[Pack Buffer 🎒]
        PackBuffer --> Operations[Command / Cde / Op 🎛️]

        Artifact --> ArtifactVcs["ArtifactVcs<P, Mutation> 🌿️"]
    end

    subgraph VCS Algebra Module ["🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs"]
        ArtifactVcs --> Edits["edits: Vec<Edit<Mutation>> ✏️"]
        ArtifactVcs --> Changes["changes: Vec<Change> 📦️"]
        ArtifactVcs --> Checkpoints["checkpoints: Vec<Checkpoint> 🚩"]
        ArtifactVcs --> Alternatives["alternatives: Vec<Alternative> 🌿️"]

        Checkpoints --> CompPins["composition_pins: Vec<CompositionPin> 🧩️"]
    end

    subgraph Artifact Store Module ["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"]
        Store[ArtifactStore 🏪] --> Envelope[ArtifactEnvelope ✉️]
        Envelope --> Graph[CompositionGraph 🕸️]
        Graph --> Coordinator[CompositionCoordinator 🎛️]
    end
```

### 1. Domain Entities & Taxonomy
Defined in [`🧰️framework/🛍️products/💻️os/AGENTS.md`](file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/AGENTS.md):
- **`Space`**: The version-controlled container for all artifacts within an application domain.
- **`Artifact`**: Data model for an application, specialized into:
  - **`Draft`**: Volatile, ephemeral artifact state.
  - **`Asset`**: Persisted artifact with optional Time-To-Live (TTL).
- **`Engine`**: Stateful computation unit holding a binary **`Pack`** buffer. Alternative versions of an artifact are materialized dynamically by applying patches to pack buffers.
- **`Command` / `Cde` / `Cmd`**: Commands targeting an engine (`Cde` = native binary protocol representation; `Cmd` = text format for logging/LLMs).
- **`Operation` / `Patch` / `Op`**: Structural mutations applied to an artifact.

### 2. Document Version-Graph Algebra (`vcs`)
Defined in [`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs`](file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%8C%BF%EF%B8%8Fvcs/%F0%9F%A6%80%EF%B8%8Fcomponent.rs):
- **`ArtifactVcs<P, Mutation>`**:
  - `initial_snapshot: P` (base payload snapshot)
  - `edits: Vec<Edit<Mutation>>` (fine-grained edit stream)
  - `changes: Vec<Change>` (grouped edits saved by actors)
  - `checkpoints: Vec<Checkpoint>` (immutable checkpoints in the version DAG)
  - `alternatives: Vec<Alternative>` (named alternative branches/variants)
- **`CompositionPin`**: `{ child_ref: ArtifactRef, checkpoint_id: String }`:
  - Pins exact child artifact checkpoints onto parent checkpoints, supporting composite multi-artifact documents without hardcoding child state.

### 3. Content-Addressed Hashing Engine
The framework uses **BLAKE3 content hashing** (`content_addressed_entity_id`) formatted as `{prefix}-{hex16}`:
- `mint_edit_id`: Content-addressed edit ID from actor + sequence + forward fingerprint.
- `mint_change_id`: Content-addressed change ID from ordered edit IDs + description.
- `mint_alternative_id`: Content-addressed alternative ID from name + ordered checkpoint IDs.
- `mint_mutation_id`: Content-addressed operation ID from binary fingerprint bytes.
- `content_addressed_checkpoint_id`: Content-addressed checkpoint ID (`ck-{hex16}`) hashing parent ID, ordered change hashes, message, authors, timestamp, and sorted composition pins. Pins are pre-sorted by `child_ref.to_uri()` to ensure deterministic convergence across peers.

### 4. CQRS & Collection Mutation Engine
- **`CollectionMutation<TId, TItem, TPatch>`**: `Add`, `Remove`, `Move`, `Patch`.
- **Pure Transformations**:
  - `apply_collection_mutation`: In-place mutation application.
  - `inverse_collection_mutation`: Derives exact mechanical inverse operations from pre-state.
  - `collection_diff_from_mutation`: Projects collection operations to sparse `CollectionDiff`.
  - `apply_mutation`: Transforms a document snapshot through forward diff application.

---

## 🔬 In-Depth Inconsistency & Friction Point Analysis

### 1. Schema Property Name Divergence (`CollectionMutation` vs `spr`)
- **Location**: [`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs`](file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%8C%BF%EF%B8%8Fvcs/%F0%9F%A6%80%EF%B8%8Fcomponent.rs#L228-L230)
- **Issue**: `ItemPatch` in `vcs` is field-identical to `crate::os_spr::command::ItemPatch`, but duplicated as a local struct because `vcs::CollectionMutation` uses `index` / `to_index` whereas `spr` uses `at`.
- **Impact**: Duplicated schema definitions and conversion overhead across the `vcs` and `spr` protocol boundaries.

### 2. Leaky Abstraction of `CollectionMutation`
- **Issue**: `CollectionMutation` was designed strictly as an internal diff/inverse engine for mutation leaves. However, `CollectionMutation<..>` is wrapped directly by several public `pub enum *Mutation` variants across plugin crates.
- **Impact**: Violates the framework's semantic mutation rule (`policySemanticVocabularyBreaches`), erases domain-specific mutation verbs (`Add`/`Remove` instead of domain actions like `RemoveStakeholder`), and degrades LLM reasoning clarity.

### 3. Collision Risk in Legacy Prefix Minting
- **Location**: [`vcs/🦀️component.rs`](file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%8C%BF%EF%B8%8Fvcs/%F0%9F%A6%80%EF%B8%8Fcomponent.rs#L69-L71)
- **Issue**: `create_document_vcs_id(prefix)` hashes the prefix string itself rather than a payload (`content_addressed_entity_id(prefix, prefix.as_bytes())`).
- **Impact**: Identical prefixes produce identical IDs regardless of invocation context. While documented as deprecated, calls still exist in legacy helpers.

### 4. Compensation Failure & State Asymmetry in Composite Rollbacks
- **Location**: [`vcs/🦀️component.rs`](file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%8C%BF%EF%B8%8Fvcs/%F0%9F%A6%80%EF%B8%8Fcomponent.rs#L214-L220) (`VcsError::CompensationFailed`) and `store::CompositionCoordinator::dispatch_group`.
- **Issue**: Multi-member group dispatches mutate parent and child artifacts sequentially. If phase 2 fails mid-stream and the reverse-order `Undo` compensation pass also fails, the system returns `CompensationFailed`.
- **Impact**: Parent `CompositionPin` references point to child checkpoint IDs that failed to commit, causing composite graph state asymmetry.

### 5. In-Memory Pack Buffer vs Store VCS Synchronization
- **Issue**: The engine maintains a binary `Pack` buffer in memory for high-frequency operations, while `ArtifactStore` maintains `ArtifactEnvelope` with `ArtifactVcs` history.
- **Impact**: Transient edits (in `Draft` status) are buffered in memory and only periodically flushed to `Change` entries in persistent `Asset` envelopes, creating a window where ephemeral draft history can become detached from the persisted VCS checkpoint graph during unexpected process interruptions.

---

## 🎯 Targeted Refactoring Recommendations

1. **Standardize Property Names Across `vcs` and `spr`**:
   - Align `CollectionMutation` field naming (`index` / `to_index` vs `at`) between `vcs` and `os_spr` to eliminate duplicated `ItemPatch` definitions.

2. **Enforce Semantic Mutation Vocabulary**:
   - Enforce `policySemanticVocabularyBreaches` in `📜️script.ts` to prevent public `Mutation` enums from exposing `CollectionMutation` directly, ensuring domain-specific verbs are preserved.

3. **Eliminate Legacy ID Minting**:
   - Replace all usages of `create_document_vcs_id` with content-addressed helpers (`mint_edit_id`, `mint_change_id`, etc.).

4. **Transactional Composite Rollbacks**:
   - Upgrade `CompositionCoordinator` to take transactional pack snapshots prior to multi-member dispatch, enabling atomic rollback on failure without risking `CompensationFailed` asymmetry.

---
*Report compiled inside ticket folder `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️14/VERSION-CONTROL-SYSTEM-DESIGN-AND-INCONSISTENCIES-REPORT` on 2026-08-14.*
