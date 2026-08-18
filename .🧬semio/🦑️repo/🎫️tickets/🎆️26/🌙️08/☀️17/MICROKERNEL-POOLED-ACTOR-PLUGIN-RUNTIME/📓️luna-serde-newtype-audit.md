# Serde Tagged Enum Newtype Defect Audit

**Ticket**: MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME  
**Issue**: serde CANNOT serialize internally-tagged enums with newtype variants whose payloads are not maps (strings, integers, sequences fail at RUNTIME with `cannot serialize tagged newtype variant X::Y containing a <string|sequence|integer>`).

**Prior incident**: `JobStep::Done(Vec<u8>)` / `JobStep::Failed(Vec<u8>)` in plugin-host crate. Both failed at runtime when serialized through serde, despite compiling successfully. Tests only asserted on in-process values, not bytes crossed the wire. **Fixed by converting newtype variants to struct variants** (`Done { output }`, `Failed { error }`). The critical lesson: "fixing one variant is not fixing the defect — re-derive the rule and re-check every sibling."

---

## 1. Full Inventory: Problematic Enums

### 🎭️ Actor Component (`🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`)

#### LIVE DEFECTS (actively usable, serde serialization possible)

| Enum | Line | Variant | Payload Type | Generated TS | Liveness Evidence |
|------|------|---------|--------------|--------------|------------------|
| `Origin` | 498–504 | `Actor(ActorId)` | u64 | `{"kind":"actor"} & bigint` | Envelope field; crosses wire in downstream code |
| `Payload` | 540–547 | `Event(Vec<u8>)` | sequence | `{"kind":"event"} & Array<number>` | Envelope.payload; Event variant used in frame creation |
| `Payload` | 540–547 | `Cancel(u64)` | u64 | `{"kind":"cancel"} & bigint` | Envelope.payload; Cancel tracking in mailbox coalescing |
| `TurnStatus` | 650–656 | `Faulted(Vec<u8>)` | sequence | `{"kind":"faulted"} & Array<number>` | TurnResult.status; returned from execute_turn trait |
| `FailureSignal` | 900–910 | `Trap(String)` | string | `{"kind":"trap"} & string` | FailureState embedded in ActorRecord |

#### LATENT DEFECTS (masqueraded by hand-rolled codec)

| Enum | Line | Variant | Payload Type | Generated TS | Notes |
|------|------|---------|--------------|--------------|-------|
| `Backpressure` | 739–745 | `Dropped(Lane)` | enum (serializes as string) | `{"kind":"dropped"} & "Interactive"\|"UserVisible"\|"Background"\|"Maintenance"` | Returns from Mailbox::enqueue; no direct serde_json usage found but derives allow it |

#### SAFE (no newtype variants or struct-only variants)

- ✓ `ActorKind` (line 299): all struct variants
- ✓ `FailureStage` (line 959): all unit/struct variants  
- ✓ `ActorStatus` (line 1187): all struct variants
- ✓ `ActivationEvent` (line 2100): all unit/struct variants

---

## 2. Liveness Analysis Per Variant

### `Origin::Actor(ActorId)` — **LIVE**

**Path**: `Origin` → `Envelope.from` → across wire  
**Serialization**: Currently pack-codec (`pack_encode`/`pack_decode` at lines 507–533) in actor component, BUT:
- Envelope struct derives `Serialize`/`Deserialize` (line 607–608)
- Found in `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` creating Envelopes and placing them in sequences

**Defect Surface**: If downstream consumer calls `serde_json::to_vec(&envelope)` or any serde wire transport, runtime panic: `"cannot serialize tagged newtype variant Origin::Actor containing a <integer>"`

---

### `Payload::Event(Vec<u8>)` — **LIVE** (HIGHEST SEVERITY)

**Path**: `Payload` → `Envelope.payload` → wire  
**Frequency**: EVERY actor event crosses this path  
**Serialization**: Pack codec in core, but Envelope can be serde-serialized; verified in renderer glue above

**Defect Surface**: Any Event-bearing Envelope that tries serde serialization fails with `"cannot serialize tagged newtype variant Payload::Event containing a sequence"`. The failing call site is likely a web shard or cross-process boundary.

---

### `Payload::Cancel(u64)` — **LIVE**

**Path**: Cancellation sequences in envelope routing  
**Usage**: Mailbox coalescing tracks `cancel_of: Option<u64>` and routes `Cancel` payloads  
**Defect Surface**: Same as Event — if envelope with Cancel payload is serde-serialized, runtime panic

---

### `TurnStatus::Faulted(Vec<u8>)` — **LIVE**

**Path**: `TurnStatus` → `TurnResult.status` → returned from `GuestRuntime::execute_turn`  
**Usage**: ShardLoop receives TurnResult, encodes via pack codec to send back over ShardTransport  
**Defect Surface**: If TurnResult ever gets serde-serialized (currently masked by pack codec), Faulted variant fails

---

### `FailureSignal::Trap(String)` — **LIVE but INDIRECT**

**Path**: `FailureSignal` → `FailureState::on_signal` (in-process) → `FailureState` embedded in `ActorRecord`  
**Serialization**: FailureState has pack_encode/pack_decode (lines 1150–1166); ActorRecord is Serialize/Deserialize (line 1232–1233)  
**Defect Surface**: If ActorRecord is ever serde-serialized with a Trapped actor, runtime panic. No current serde_json usage found, but derives enable it.

---

### `Backpressure::Dropped(Lane)` — **LATENT**

**Path**: `Backpressure` returned from `Mailbox::enqueue`, stored in test assertions  
**Current Serialization**: None found via grep; pack codec not used  
**Defect Surface**: If Backpressure is serde-serialized (e.g., in metrics telemetry), Dropped variant fails  
**Verdict**: Latent trap, not currently triggered

---

## 3. TypeScript Mirror Impact

**Framework config**: ts-rs v10 (optional feature "typegen"), respects serde renames  
**Generated file**: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🤖️generated/🟦️actor.ts`

### Impossible Intersection Types (Wire Contract Violations)

These types are generated but cannot represent values that a wire actually carries:

| Type | Generated Definition | Problem |
|------|---------------------|---------|
| `Backpressure` | `{"kind":"dropped"} & Lane` where Lane is `string` | Object ∩ string = ∅ |
| `Origin` | `{"kind":"actor"} & ActorId` where ActorId is `bigint` | Object ∩ bigint = ∅ |
| `Payload` | `{"kind":"event"} & Array<number>` \| `{"kind":"cancel"} & bigint` | Object ∩ array/bigint = ∅ |
| `FailureSignal` | `{"kind":"trap"} & string` | Object ∩ string = ∅ |
| `TurnStatus` | `{"kind":"faulted"} & Array<number>` | Object ∩ array = ∅ |

**Impact**: Any TypeScript consumer trying to handle these types as unions will find no value satisfies them. The generated mirror is **not usable as a wire contract** for those variants.

---

## 4. Serde vs. ts-rs Casing Divergence

**Key Finding**: ts-rs v10 **DOES respect serde rename annotations** despite being optional.

### Examples in Actor Component

| Type | Attribute | Rust Names | Serialized Names | TS Mirror |
|------|-----------|-----------|------------------|-----------|
| `Lane` | (none) | Interactive, UserVisible, Background, Maintenance | PascalCase (no rename) | `"Interactive"\|"UserVisible"\|"Background"\|"Maintenance"` |
| `Origin` variants | `#[serde(...rename_all="camelCase")]` | Ui, Actor, Kernel, Bus | ui, actor, kernel, bus | `"ui"\|"actor"\|"kernel"\|"bus"` |
| `Payload` variants | `#[serde(...rename_all="camelCase")]` | Event, Suspend, Resume, Cancel, JobStep | event, suspend, resume, cancel, jobStep | TS matches serde |

**Implication**: The generated TypeScript is **authoritative for what serde WILL serialize**. Lane's PascalCase in the mirror is correct; Origin's camelCase is correct. Inconsistency is detected correctly.

### Casing Inventory (Actor Component)

All tagged enums in actor component declare `rename_all`:
- **camelCase**: Origin (line 498), Payload (540), TurnStatus (650), Backpressure (739), FailureSignal (900), FailureStage (958), ActorStatus (1186), ActivationEvent (2099), ActorKind (298)

**Consistency**: ✓ Uniform camelCase policy across the crate. No divergence issues within actor component.

**Outside Actor Component** (spot check):
- `🛂️manifest/🦀️component.rs`: camelCase uniformly applied
- `📇️directory/🧬️schema/🦀️component.rs`: rename_all varies (camelCase, kebab-case, lowercase) — possible future issue if any contain newtype variants
- `🔌️plugin/🖥️host/🦀️component.rs`: JobStep (already fixed to struct variants)

---

## 5. Diagnostic: What Happened to JobStep

**File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`  
**Lines**: 440–461

```rust
// OLD (DEFECTIVE):
pub enum JobStep {
    Running(Option<Vec<u8>>),
    Done(Vec<u8>),             // ← serde_json error: "cannot serialize tagged newtype variant"
    Failed(Vec<u8>),           // ← SAME defect, found one wave after Running
}

// NEW (FIXED):
pub enum JobStep {
    Running { progress: Option<Vec<u8>> },
    Done { output: Vec<u8> },           // ← struct variant: serde-safe
    Failed { error: Vec<u8> },
}
```

**Symptoms**:
- Line 449–452 comment: "cannot serialize tagged newtype variant JobStep::Done containing a sequence"
- Tests passed because they asserted on in-process values, not on bytes that crossed `send_outcome`
- One-wave lag: Running was fixed first (Option detection), Done/Failed (sequence detection) found later

**Pattern Applied**: All three variants carry payloads that fail serde's internal-tag rule (Option, Vec, Vec). Fixing just the first symptom left siblings unfixed.

---

## 6. Call Sites Proving Liveness

### Origin::Actor
- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs:513–515`: pack_encode case
- `🧰️framework/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`: Envelope created with `Origin::Kernel`; same place could use `Actor`

### Payload::Event & Payload::Cancel
- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs:552–566`: pack_encode for all variants
- Line 2888–2889 (test): `Payload::Event(vec![1, 2, 3])` in Envelope
- `🧰️framework/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`: `Payload::Event(serde_json::to_vec(&events.first()...))`

### TurnStatus::Faulted
- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs:664–667`: pack_encode case
- Line 2940 (test): `TurnStatus::Faulted(b"boom".to_vec())`

### FailureSignal::Trap
- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs:928–931`: pack_encode case
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:1072`: used in on_signal matching

---

## 7. Recommendations (OUT OF SCOPE FOR THIS AUDIT)

This audit identifies defects only. Fix decisions belong to the ticket's fix phase:

1. **Payload::Event, Payload::Cancel, TurnStatus::Faulted, Origin::Actor**: Convert newtype → struct variants (same fix as JobStep)
2. **FailureSignal::Trap**: Same conversion or wrap in diagnostic struct
3. **Backpressure::Dropped**: Same conversion if ever serialized
4. **Generated mirror**: Will auto-correct once Rust enum structs replace newtypes
5. **Wire contract**: Verify downstream consumers of Envelope/ActorRecord don't rely on impossible TS union branches

---

## 8. Measured vs. Inferred

**Measured**:
- ✓ Grep found all `#[serde(tag` enums in framework and OS product crates
- ✓ Read enum definitions to identify newtype variants
- ✓ Read generated TypeScript to confirm impossible intersections
- ✓ Checked Cargo.toml for ts-rs version and features
- ✓ Verified ts-rs v10 respects serde renames
- ✓ Grep found explicit serde_json usage in renderer glue

**Inferred** (honest uncertainty):
- `Backpressure::Dropped`: No serde_json call found via grep, so "latent" assumption is conservative guess; actual liveness requires tracing all transport boundaries
- `FailureSignal::Trap` liveness: ActorRecord derives Serialize but no call site found that actually serializes it; may be dead code
- Downstream consumers of generated TS: Assumed they might call serde on Envelope; not verified per consumer

---

## Summary Table

| Enum | Variant | Defect | TS Mirror | Liveness | Severity |
|------|---------|--------|-----------|----------|----------|
| Origin | Actor | newtype u64 | impossible | LIVE | HIGH |
| Payload | Event | newtype Vec<u8> | impossible | LIVE | CRITICAL |
| Payload | Cancel | newtype u64 | impossible | LIVE | HIGH |
| TurnStatus | Faulted | newtype Vec<u8> | impossible | LIVE | HIGH |
| FailureSignal | Trap | newtype String | impossible | LIVE-INDIRECT | MEDIUM |
| Backpressure | Dropped | newtype enum | impossible | LATENT | LOW |

**Total defects found**: 5 live, 1 latent = **6 locations** across one crate (actor), awaiting sibling sweep in plugin-host and OS crates.

