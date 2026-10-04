# Natural File Native Boundary Review

This is a read-only coordination review of the in-progress natural-file route. It is not a completed acceptance result.

The mounted `NaturalFileCodec` describes a format identifier, extension, MIME type and binary/text character. Open and Save are paired localized shell actions, and the builder adds their format to the existing import/export lists without replacing other capability declarations. The reserved `artifact:native` export dispatch calls the editor's real encoder and returns an exact binary wire descriptor. Unsupported editors refuse the route explicitly.

The import path validates the format identifier and live app instance before dispatching the import as a document mutation. One current implementation detail needs repair: already-owned file bytes are encoded again as a Semio pack/base64 string in `MediaPayload::Structured.json`. This obscures the type and adds a second materialization. The execution owner has been asked to carry first-party intrinsic bytes or an explicit typed natural-byte payload through the existing mutation admission path instead.

The coverage report correctly distinguishes codec declarations from actual mounting: CSV, PNG and the three DOCX profiles are the present boundary. It also records that native encode/decode and browser file reads still materialize whole buffers. Progress/cancellation around the surrounding transfer does not establish cancellation during those expensive phases. The retained segmented-media follow-on remains necessary before claiming complete large-file interaction.

Relevant source: framework plugin `NaturalFileCodec`, `mount_natural_file_actions`, `VcsArtifactApp::produce_media` and `VcsArtifactApp::consume_media`. The bounded execution lane owns repairs and fresh native/TypeScript receipts; root owns browser acceptance.

The execution owner repaired the import carrier: raw file bytes now move through `MediaPayload::Intrinsic` as first-party `DslValue::Bytes`, with exact schema validation and no pack/base64 intermediate. The neutral octet vector includes0,1,127,128,255; the registered native exact gate passed1/1 (`natural-file-plugin-native-4.log`). Broader codec and browser acceptance remain open.
