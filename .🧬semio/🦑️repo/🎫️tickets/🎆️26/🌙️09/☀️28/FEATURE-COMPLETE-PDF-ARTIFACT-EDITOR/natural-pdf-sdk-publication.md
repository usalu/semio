# Natural PDF SDK and Store Publication

The staged SDK path takes the authenticated kind-241 source into the actual editor reserved importer as `ArtifactReservedToolInput::NaturalPaged`. It retains the exact `ControlledRetirement<PagedBytes<…>>` from the source receiver. The `PagedBytes` const generic controls total bytes. The natural receiver type has a 512 MiB structural ceiling and its backing is independently 4 KiB per page. The editor advertises its smaller format-specific maximum at launch.

## App registration and import

An `ArtifactEditor` enables natural paged input by implementing `admit_paged_natural_file_decode_owner` and supplying a nonzero `PAGED_NATURAL_SOURCE_MAXIMUM_BYTES`. The hook takes `&mut Option<semio_plugin::NaturalFilePagedSource>`, the existing tool contract, the decoder recipient and the actual step context. The `EditorApp` forwards only the app's author opt-in, and the mounted PluginApp projects a capability only for a binary codec. Formats with the default maximum of zero remain disabled. The PDF-specific hook and maximum belong to the separate decoder overlay.

The mounted RenderCtx validates the paid source and exact `artifact:native` port, moves the owner through existing reserved preparation, and uses an empty generic tool raw wire. That keeps the existing assembled command maximum while admitting the original source under the separate natural format extent. The framework natural import worker requires the paged hook to consume the source only when it returns a controlled decoder. Pending source bytes retire through their controlled owner if work ends before decoder admission. After decode, the normal paid Pack export, verified genesis admission, and typed Store replacement path still own publication.

## Terminal Store receipt

The per-instance publication carrier retains local actor metadata, exact Original Source binding and mounted operation identity. It takes the sealed source under the existing capability and grants. The typed genesis route funds and records a natural publication witness only after the replacement frontier agrees with the actual mounted Store generation and revision. The app waits until the mounted typed operation, original preparation, undonated bytes and retained metadata settle before handing the result to the reactor. Complete requires a successful witness and terminal app holder. Cancel requests cancellation of the same pending mounted operation before retiring the holder and allows the receiver to acknowledge only after that cleanup. Native and Browser archive persistence remains outside the Store Complete acknowledgment.

## Verification

- The combined patch passed read-only applicability checks.
- The focused wasm32-wasip2 check compiled fresh generated WIT, the natural reactor format dispatcher, exact staged receive/lifetime functions, and the actual staged publication carrier against cached production dependencies. The integration's outer registry uses a narrow metadata harness. The full editor crate, its Store run, and host archive publication remain unverified in this stage.
- Ajv and SQLite agreed on all eight SDK boundary cases. The same eight passed in Rust using the admission predicate extracted directly from the staged natural import worker. The corpus tests the 9,437,877-byte PDF, exact source maximum, over-limit source, missing paged registration, nonbinary format, empty source, foreign port and duplicated generic wire body.

The staging entry point generates `natural-source-sdk-publication.patch` within the ticket's generated output directory. It includes the earlier reactor/wire and receiver changes. Generate it from the current workspace before combining with the separate PDF decoder, Browser and native overlays.

- [Consolidated publication generator](natural-pdf-publication-stage.py)
- [SDK and reserved worker generator](natural-pdf-sdk-stage.py)
- [App publication carrier](natural-pdf-publication-owner.rs)
- [Mounted app publication driver](natural-pdf-appointed-publication.rs)
- [Reserved RenderCtx intake](natural-pdf-reserved-render.rs)
- [SDK fixture and independent oracle](natural-pdf-sdk-law.mjs)
