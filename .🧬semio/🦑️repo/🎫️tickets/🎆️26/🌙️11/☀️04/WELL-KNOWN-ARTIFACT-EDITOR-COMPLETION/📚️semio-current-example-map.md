# Current Semio Examples and SDK Codec Plan

Bounded read-only source/content audit. No actual Rust decoding/build was run; schema/header consistency is distinguished from valid current codec execution. Most typed editors still publish shared demo and parse its old umbrella hex scaffold with unwrap_or_default. Exact existing typed bindings are below. Paths are relative to stdio Semio v1/subsets.

| App subset | Example ID | Exact source | Asset header |
| --- | --- | --- | --- |
| audio | tone | ✉️base/📚️examples/🎵️tone/🦀️.rs | semio stdio.semio.audio.dsl v1 |
| animation | walk | ✉️base/📚️examples/🚶️walk/🦀️.rs | semio s.stdio.semio.animation.dsl v1 |
| brep | solid | ✉️base/📚️examples/🧊️solid/🦀️.rs | semio stdio.semio.brep.dsl v1 |
| cad | drawing | ✉️base/📚️examples/📐️drawing/🦀️.rs | semio stdio.semio.cad.dsl v1 |
| document | memo | ✉️base/📚️examples/🗒️memo/🦀️.rs | semio s.stdio.semio.document.dsl v1 |
| drawing | sketch | ✉️base/📚️examples/🖍️sketch/🦀️.rs | semio stdio.semio.drawing.dsl v1 |
| image | swatch | ✉️base/📚️examples/🖼️swatch/🦀️.rs | semio s.stdio.semio.image.dsl v1 |
| text | note | ✉️base/📚️examples/📃️note/🦀️.rs | semio s.stdio.semio.text.dsl v1 |
| value | graph | ✉️base/📚️examples/🕸️graph/🦀️.rs | semio stdio.semio.value.dsl v1 |
| mesh | cube | 🔺️mesh/📚️examples/🧊️cube/🦀️.rs | semio stdio.semio.mesh.dsl v1 |
| video | clip | ✉️base/📚️examples/🎥️clip/🦀️.rs | semio stdio.semio.video.dsl v1 |
| * | envelope | ✉️base/📚️examples/🌐️envelope/🦀️.rs | semio stdio.semio.dsl v1 |
| presentation | deck | ✉️base/📚️examples/📽️deck/🦀️.rs | semio s.stdio.semio.presentation.dsl v1 |
| flow | pipeline | ✉️base/📚️examples/🌊️pipeline/🦀️.rs | semio stdio.semio.flow.dsl v1 |
| model | building | ✉️base/📚️examples/🏢️building/🦀️.rs | semio stdio.semio.model.dsl v1 |
| graph | wires | 🕸️graph/📚️examples/🕸️wires/🦀️.rs | semio s.stdio.semio.graph.dsl v1 |
| object | crate | 📦️object/📚️examples/📦️crate/🦀️.rs | semio stdio.semio.object.dsl v1 |
| kit | furniture | 🧰️kit/📚️examples/🪑️furniture/🦀️.rs | semio stdio.semio.kit.dsl v1 |
| table | sheet | 📊️table/📚️examples/📃️sheet/🦀️.rs | semio s.stdio.semio.table.dsl v1 |

Base shared examples exports building/deck/drawing/graph/note/sketch/swatch/walk directly and clip/demo/envelope/memo/pipeline/solid/tone through examples_2. The graph module is VALUE, not graph-domain; graph-domain must bind its local wires source. Mesh/object/kit/table also have exact subset-local source modules.

## Canonicality Findings

The s.stdio prefix remains in current animation/table/document/presentation/graph/text/image DSL headers and embedded schema strings, while source DOCUMENT_SCHEMA constants are stdio.semio.<subset>. Such assets cannot be byte-identical to current print_dsl output with current schema. Assets bearing current header remain candidates only until actual typed decode and full reprint equality run. Pack counterparts must be compared against actual current encode_pack; existence or filename is no validity proof.

Text parse_dsl strips any preamble and body parser simply retains decoded schema without checking the current constant. Thus stale note can parse nonempty but retain wrong schema. Tests must assert snapshot.schema==E::DOCUMENT_SCHEMA, full populated data and canonical regenerated text/pack, not just parse success. Shared demo is old hex umbrella scaffold and is wrong for every typed subset.

## Clean Binding

Each editor examples() and example_snapshot must reference the same exact example ID/source above. Preserve existing ExampleSource localized label and icon (en/de already authored). Parse strictly into that editor Snapshot; refuse unknown IDs and invalid source with typed faults. Remove both unknown-ID default and parse-error unwrap_or_default. Do not publish a fabricated empty default as a selected example. Handcraft current assets together from their matching populated typed demos; no compatibility readers or migrations.

## Minimal Actual SDK Target

Add a standalone existing Semio root Rust test target with19explicit source ArtifactEditor calls and model/snapshot testkit enabled. For each exact populated demo, assert nonempty discriminator, schema identity, source-example ID/label consistency, current DSL/Pack assets equal actual print/encode, register natural_file_codec then invoke actual SDK encode/decode and complete typed equality. Reject each wrong-schema neighbor and unknown example ID. The neutral roster plus independent serde/Ajv content oracle is separate from physical native application lifecycle and browser proof. No source-wide fallback to default is allowed.
