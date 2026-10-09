# PDF Semantic Body Ownership

The source gate was run before implementation and rejected encoded image codecs and font-program bytes in semantic bodies (session 65272, exit 1). Current migration is in progress; no runtime or compiler success is claimed.

`PdfImageBody` is either explicit logical component samples (`Vec<u32>`, 1/2/4/8/16 bit value range) or a canonical `ArtifactRef`. Font programs carry an explicit canonical reference, with the font kind semantic. Native image row packing, row padding and foreign font metadata remain in actual IO. The resource port owns original native stream representations independently and admits identities derived from body/filter encoding; native lowering explicitly refuses unresolved references.

The shared native port is `base/io/foreign_artifacts`: `PdfArtifactResourcePort::admit(kind, PdfObject)`, `resolve(&ArtifactRef)`, and `NativePdfArtifactResources::from_objects`. Detached builders retain native custody through `admit_pdf_artifact` in the snapshot's native graph. The role dictionary stores reference/sample bodies, not cached encoded representations.

Schema-first facets changed together: Rust, JSON Schema, TypeScript, GraphQL and Protobuf image/font bodies. Native lifter now records admission errors instead of silently producing empty logical samples. Native lowering and font codecs use explicit resource resolution. SQL and consumer migration are being completed.

Peer owns ICC, mesh, indexed palette, output profile and role persistence; this worker owns image/font program, inline image, and logical CID glyph ids. Common native build currently fails in independent framework retirement consumers before PDF, so native validation remains pending.

## Current Validation

- RED source gate 65272 preceded semantic body edits.
- GREEN current source gate 2058: `[DEBUG] PDF image/font body ownership admits logical samples or typed foreign artifacts only`.
- Earlier 37820 and 98652 were command setup failures before subject execution; they are not body outcomes. The exact registered command uses `nx exec --projects=workspace --excludeTaskDependencies`.
- Native semantic body law 97273 is running through the repository Bun/Nx package owner; at last receipt only Nx dependencies had executed. It is not a runtime pass. The two laws cover three neutral image row cases (1/4/16 bits), malformed sample refusal, independent lopdf native stream decoding, and unadmitted foreign custody refusal.
- PDF peer owns fresh full strict TypeScript, independent SQLite Source and stream role native outcomes. Its prior strict diagnostics were fixed in current font witness code.

## Exact Source Manifest For Handoff

Paths below are relative to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base` unless stated otherwise. Shared facet edits were targeted and coordinated with the PDF graph worker.

- `🧬️schema/📸️snapshot/🦀️.rs`
- `🧬️schema/📸️snapshot/🔣️.json`
- `🧬️schema/📸️snapshot/🟦️.ts`
- `🧬️schema/📸️snapshot/🔗️.graphql`
- `🧬️schema/📸️snapshot/🛰️.proto`
- `🧬️schema/🔎️graph-projection/🦀️.rs`
- `🧬️schema/🧬️mutations/🏞️set-image/🟦️.ts`
- `🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/🦀️.rs`
- `🚪️io/📦️foreign-artifacts/🦀️.rs`
- `🚪️io/🧪️tests/🔬️unit/🦀️.rs`
- `🔨️modules/⬇️lift/🦀️.rs`
- `🔨️modules/⬆️lower/🦀️.rs`
- `🔨️modules/🔤️fonts/🦀️.rs`
- `🔨️modules/🔤️fonts/🧪️tests/🔬️unit/🦀️.rs`
- `🔨️modules/🖼️images/🦀️.rs`
- `🔨️modules/🖼️images/🧪️tests/🔬️unit/🦀️.rs`
- `🔨️modules/🖼️images/🧪️tests/🔬️unit/🧫️fixtures/🔣️.json`
- `🔨️modules/🖋️content/🦀️.rs`
- `🔮️oracles/🦀️.rs`
- `✏️editor/🦀️.rs`
- `✏️editor/🖼️page/🦀️.rs`
- `✏️editor/🖼️page/🧪️tests/🔬️unit/🦀️.rs`
- `🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql`
- `🚪️io/🪶️sqlite/📸️snapshot/🔤️font/🦀️.rs`
- `🚪️io/🪶️sqlite/📸️snapshot/🔤️font/🟦️.ts`
- `🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🔤️font/🦀️.rs`
- `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔤️font/🟦️.ts`
- `🚪️io/🪶️sqlite/📸️snapshot/🔤️font/🧫️fixtures/🔣️.json`
- `🚪️io/📝️text/📸️snapshot/🪪️native-json/🔤️font/🟦️.ts`
- `🚪️io/🪶️sqlite/📸️snapshot/🖼️resource/🦀️.rs`
- `🚪️io/🪶️sqlite/📸️snapshot/🖼️resource/🟦️.ts`
- `🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🖼️resource/🦀️.rs`
- `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🖼️resource/🟦️.ts`
- `🚪️io/🪶️sqlite/📸️snapshot/🖼️resource/🧫️fixtures/🔣️.json`
- `🚪️io/📝️text/📸️snapshot/🪪️native-json/🖼️resource/🟦️.ts`
- `🚪️io/🪶️sqlite/📸️snapshot/🖋️content/🦀️.rs`
- `🚪️io/🪶️sqlite/📸️snapshot/🖋️content/🟦️.ts`
- `🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🖋️content/🦀️.rs`
- `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🖋️content/🟦️.ts`
- `🚪️io/🪶️sqlite/📸️snapshot/🖋️content/🧫️fixtures/🔣️.json`
- `🚪️io/📝️text/📸️snapshot/🪪️native-json/🖋️content/🟦️.ts`
- `🚪️io/📝️text/📸️snapshot/🪪️native-json/🧬️schema/🟦️.ts`
- `🚪️io/📝️text/📸️snapshot/🪪️native-json/🖼️resource/🧫️fixtures/🔣️.json`

Additional package/runner edits: PDF `📦️packages/🦀️rust/📜️script.ts` adds `test semantic-body`; both `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json` register the exact new law. The ticket's `construct-geometry/📜️script.ts` owns the current body source gate. Existing authored fixture image/font/inline/CID bodies were replaced directly; no migration script or runtime adapter was added.

Pending native receipt belongs to the peer after handoff. Retain the native body filter `pdf_semantic_body_`, do not claim passed until both tests and DEBUG lines execute. Flow/Lowpoly native outcomes remain blocked before their own tests by common framework compiler consumers; root owns those repairs and requests holding duplicate native reruns.
