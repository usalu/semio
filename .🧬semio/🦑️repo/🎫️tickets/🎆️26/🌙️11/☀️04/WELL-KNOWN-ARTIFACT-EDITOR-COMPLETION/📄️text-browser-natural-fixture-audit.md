# Text Natural-file Browser Fixture Audit

The actual acceptance owner is stdio/🧪️tests/🎯️artifact-acceptance. Its 🧫️fixtures/🔣️.json Semio row (batch H) selects plugin stdio-semio, exact app s.stdio.semio@v1/text#editor and natural .semio input. It pins the Text mutation fixture at Semio v1/subsets/🔤️text/🧪️tests/🧫️fixtures/🔤️mutate-semio-text/🎒️.pack.semio. The sample manifest sibling 📇️samples.json lists that exact pin. candidateSamples requires row.sample be manifest-listed and never falls back to Kit/Mesh.

Direct binary inspection proves the pinned 7380-byte file outer envelope is s.stdio.semio.text.pack v1. Current E::DOCUMENT_SCHEMA is stdio.semio.text and natural macro mounts its actual ArtifactDsl envelope. No actual native decoding was performed in this audit. The acceptance oracle checks only Semio magic (or UTF-8), so its approval does not establish current typed codec/schema validity.

Four Text Pack files retain that old outer envelope, relative to Semio artifact root:

- 📚️examples/📃️note/🖼️assets/🎒️.pack.semio
- 🏅️standards/🔖️v1/🪆️subsets/✉️base/📚️examples/📃️note/🖼️assets/🎒️.pack.semio
- 🏅️standards/🔖️v1/🪆️subsets/🔤️text/🖼️assets/📃️note/🎒️.pack.semio
- 🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧪️tests/🧫️fixtures/🔤️mutate-semio-text/🎒️.pack.semio

The first three are byte-identical 332-byte Pack copies, and their corresponding 203-byte DSL copies are byte-identical with old header and embedded hex schema s.stdio.semio.text. The mutation Pack is unique, not a duplicate of the example. The named large Text mutation DSL fixture also carries old header/schema. Source modules exist both at artifact-root examples and base subset examples; canonical asset work must account for both, not just the subset tree.

Minimum repair after actual SDK proof: capture genuine populated Text print_dsl/encode_pack and E::encode_natural_file output, assert decoded snapshot.schema==E::DOCUMENT_SCHEMA, native roundtrip exact snapshot and canonical reprint/reencode. Handcraft all matching authored copies from that exact typed data. Either retain and canonically repair the mutation pin with its real content, or pin the canonical Text-local note Pack after adding it explicitly to the sample manifest. Do not change only the binary header: the embedded payload schema is a separate proof obligation. Keep /runs/0/content nonempty and different from Matrix before the browser edit. No asset edits performed.

Browser source 🏃️browser/🟦️.ts requires actual file-opened notice plus kept UUID, exact owner, rendered Details scalar /runs/0/content=Matrix, edits witness, edited export different from baseline, Undo export equal baseline, Redo export equal edited, Savefile download plus file-saved notice, local persisted marker for same UUID, close/reopen exact export, then page reload and same UUID reopen/export. Invalid deadbeef must refuse without content/window change. Semio comparison is exact bytes, with no timestamp normalization. This journey remains unexecuted until the real shell is available.

Select only the canonical Text Semio row with --only semio (not --only text, which names no artifact row); run both explicit --locale en and --locale de. Entry source accepts run --serve URL --only semio --locale en|de --tag TAG --out TICKET/🗑️generated. A repository tool invocation is:

```sh
SEMIO_LANGUAGE=en repo tools exec bun -- ./✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🏃️browser/🟦️.ts run --serve http://127.0.0.1:6070 --only semio --locale en --tag semio-en --out .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITOR-COMPLETION/🗑️generated
```

Use the actual served URL from runtime agent, and repeat with locale/tag de. No browser command was launched by this audit.
