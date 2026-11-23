# Record Field Match: Flatten Keys and Ordinals

## Root cause

The Ambiguous result was not a name overlap. `DrawingGroupBody` has no name shared between `DrawingLayerBase` and its own fields (`isolation`, `children`), and serde_json decodes the JSON fine.

The Drawing pack is a DSL record source. Its keys are `Key::Ordinal(field id)`, and the DSL derive numbers each struct's own fields by index. `base` is a nested `#[dsl(block)]` record at id 0, `isolation` is 1, `children` is 2.

The schema walk expanded a `Flatten` node for every key kind and compared ordinals against the flattened struct's own indices (id 0, name 1, visible 2 ...). Ordinal 1 (`isolation`) therefore matched both `name` inside `base` and `isolation`. A `[DEBUG]` trace confirmed this: `key="#1" leaf=isolation root=1 matched=Field(0)`.

serde semantics: `flatten` splices field names and only exists for self-describing, name-keyed formats. An id-keyed (non-self-describing) format addresses the declared field itself and never splices.

## Fix

- `NativeRecordSchema::Flatten` (struct `#[value(flatten)]`) is spliced by name only. An ordinal key addresses the flatten field itself as a leaf at its own index (`🧭walk.rs`, `apply_frame_into`).
- New `NativeRecordSchema::Splice` keeps the old inline behaviour for enum variant carriers (the enum derive emits it).
- `NativeRecordMatcher::ordinal_key()` tells the derived receiver which case matched.
- Derived struct receiver (`✨️derive/🛬️receive/🦀️.rs`): a flatten field matched by ordinal records its own position and is projected with `record_child(position)` as a nested record. A flatten field matched by name keeps `usize::MAX` (whole-record subset), as before. A duplicate ordinal for a flatten field is refused.

## Name overlap, serde parity

serde gives an own field precedence over a flattened one with the same name. The native matcher answers Ambiguous for that case (refusal, stricter than serde). That is deliberate: a single native match cannot be dispatched to two receivers, and own-wins needs an own/flatten origin bit in the resident matcher state, which would change its metered accounting. The law asserts serde's owner is the own field wherever the matcher says Ambiguous.

## Law

- `🔎️schema/🎟️work/📐️.json` and `🔣️.json`: ordinal 0 and 2 now resolve to fields 0 and 2 (previously Ambiguous through the collision).
- `🎟️work/🧪️tests/🦀️.rs`: every name case is cross-checked against `serde_json` through a serde `#[serde(flatten)]` mirror struct (`serde_owner`).
- `🎟️work/🧪️tests/🟦️.ts`: TS twin validates the fixture against its schema and re-derives every outcome independently.

## Record TS twin

`🫴️record/🧪️tests/🟦️.ts`: the floor law now asserts the turn copy is above one receiver (the Rust law asserts only receiver < copy), and the text was renamed. `bun test` passes.

## Results

- `semio-framework-value-native-owned --test record_matcher_driver`: 2 passed (flatten walk + serde oracle).
- Record open tests: the Group receive now gets past the flatten match and descends through `DrawingLayerNode` -> `DrawingPathBody` -> `DrawingLayerBase` -> `DrawingAttributes` -> `FillStyle` (trace in an intermediate run). Later runs, with other agents editing the job context, the Drawing schema and the variant receiver concurrently, fail before the first receive turn with "pending decoder requires original recovery" (`🧵️job/⏱️context/🛬️owned`), which is outside the matcher. An intermediate run also showed the `FillStyle` tagged-enum selector (`🔀️variant`, phase 2 `tag` missing) refusing the DSL source. Neither is resolved here.
- Framework crates: see the summary in the chat.
