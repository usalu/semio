# Protocol file contract (`.protocol.semio`)

## Location

- `…/🎒️pack/📡️component.protocol.semio` — `.spk` document layout for the artifact projection.
- `…/📡️spr/📡️component.protocol.semio` — per-op CDE layout (`format | ordinal | record body`).

## Header

```
dialect protocol
protocol <id>
magic <bytes-literal>?
version <int>
start <production>
```

## Primitives

`varint`, `u8`, `u16`, `u32`, `leb128`, `tag`, `record`, `field`, `segment`, `framing`, `chain` — aligned with `pack_core` / `protocol_format`.

## Normative role

Byte-level recognizer walks encoded fixture bytes and asserts structural agreement with Rust `encode`/`decode`. Not a codegen source; encoders stay handcrafted.

## Conformance

Per-artifact test: `encode(sample)` then `ProtocolRecognizer::verify(bytes, spec)`.
