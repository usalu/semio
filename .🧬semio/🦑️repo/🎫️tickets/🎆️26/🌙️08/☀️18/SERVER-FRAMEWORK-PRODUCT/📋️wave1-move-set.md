# Wave 1 — validated move set

The approved plan's move set was based on an agent's dependency claim that proved **partly wrong**.
Verified by grepping real code vs doc comments. Corrections below are authoritative.

## Claims that were WRONG in the plan

| Plan claim | Reality |
|---|---|
| `🧵️channel` moves | **Stays in os.** It is `AppFrame`/`AppCommand` (UI/plugin tier) and has a real dep on `crate::os_store::pack_rt`. Neither db nor hub reference it at all. |
| `🎮️command` moves whole | **Must be split.** Real code `pub use crate::os_vcs::{…}` (Collection region, l.669) plus Inference/Semantics/DiffKit/Descriptor/Composite are os authoring-tier. Only the mutation contract moves. |
| move set is dependency-free | `📡️wire` has a real `impl crate::os_store::ArtifactPack for InteractionState` (l.1776-1785) that must relocate to the store side. |

## Claims that were RIGHT (verified doc-comment-only, safe)

- `🧾️wire` → `os_pack::value`, `os_dsl::op_rt` — doc comments only (l.418, l.423).
- `📐️format` → `os_spr::history` — doc comment only (l.942).
- `🔗️causal` → `os_spr::history` — doc comment only (l.184).
- `📡️spr/🆔️ids`, `🎒️pack/🆔️ids`, `🗣️dsl/📍️span` — zero `crate::` deps.

## Final move set

Into new `🧰️framework/🔨️modules/📡️replication/` (crate `semio-framework-replication`, `[lib] name = "protocol"`):

| target | source | note |
|---|---|---|
| `🆔️ids/` | `📡️spr/🆔️ids` | clean |
| `🔢️scalar/` | `📡️spr/🔢️scalar` | needs codec |
| `📖️dictionary/` | `📡️spr/📖️dictionary` | needs ProtocolError |
| `🔐️crypto/` | `📡️spr/🔐️crypto` | needs wire |
| `🎮️mutation/` | **split** of `📡️spr/🎮️command` | regions Mutation, Message, OpBinary, Meta + `MutationOrigin` (from Composite) |
| `🔗️causal/` | `📡️spr/🔗️causal` | needs mutation contract |
| `⚔️conflict/` | `📡️spr/⚔️conflict` | needs MutationMessage, MergePolicy |
| `🧾️wire/` | `📡️spr/🧾️wire` | codec prims, REC_* vocab, MergePolicy |
| `📡️wire/` | `📡️spr/📡️wire` | minus the ArtifactPack impl |
| `📐️format/` | `📡️spr/📐️format` | needs codec + DeflateCodec |
| `⚙️codec/` | `🎒️pack/🧾️codec` + `🎒️pack/🆔️ids` + DeflateCodec (pack `📐️format` l.910-960) | folded: CodecId/ContentHash join codec |
| `🚰️source/` | `🎒️pack/🚰️source` | needs codec |
| `🦀️component.rs` | spr root 🔖️Sync region | extract_range/verify_slice/slice_content_chain |
| `🧫️fixtures/wire/` | the 20 `.bin` (moved, never regenerated) | single canonical copy |

Into new `🧰️framework/🔨️modules/⚠️diagnostic/` (crate-less, mounted once by replication glue):
`🗣️dsl/⚠️diagnostic` + `🗣️dsl/📍️span`.

**Stays in os:** `🧵️channel`, `📜️history`, `💎️materialize`, spr `🔌️io`/`⌨️cli`/`🧪️testkit`,
the os authoring half of `🎮️command`, pack `📐️format` (minus DeflateCodec), pack `value`/`http`/`io`/`testkit`/`async`.

## Name collision found

`🎮️command` already defines `pub struct CommandOutcome<Diff>` (region 🔖️Outcome, l.939).
The server contract's `CommandOutcome` (Accepted/Transformed/Rejected/Pending) is a **different**
concept. Server contract keeps its name inside `server_contract`; never glob-import both.

## db's actual protocol surface (measured)

ArtifactId 92, ActorId 71, MutationId 66, MutationEnvelope 57, HybridLogicalTimestamp 31, SchemaId 27,
ProtocolError 19, MergePolicy 19, ServerFrame 17, ArtifactDiff 17, InverseMutation 16, MutationMessage 13,
protocol_format::recover 12, RuntimeFrontierSummary 11, Bootstrap 10, SprWriter 8, Severity 8,
RecoveryMode 8, ProtocolLimits 8, ConflictKind 7, Mutation 5, encode_envelope 4, Signer 4,
MutationOutcome 4, MutationDiff 4, decode_envelope 3, SignatureVerifier 3, FrameCursor 3,
ConflictResolution 2, ClientFrame 2, worst_level 1. All covered by the move set above.
