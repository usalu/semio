# Native Artifact Genesis Factory

## Current source boundary

The package-owned genesis capability is separate from the generic `ArtifactCodec` table.

- `NativeCodecBinding::new` admits a codec with no creation authority.
- `NativeCodecBinding::with_genesis` admits an exact codec plus one package-owned editor factory.
- The complete 26-codec Stdio closure remains headless and carries no implicit application dependency or genesis authority.
- GIS Map, GIS Terrain, and VCS receipts derive codec and genesis from one private receipt discriminant and consume them together only after receipt validation.
- `VerifiedNativeArtifactCodec` retains the optional capability. Creation selection accepts only one exact Editor open target whose plugin, package, version, component SHA-256, kind, schema, and Pack schema hash match a verified codec carrying that capability.
- An absent factory fails closed; there is no generic/default snapshot fallback.

The framework helper validates the server-minted nonempty document identifier and exact editor dialect, invokes the editor's own `initial_snapshot`, creates the normal document envelope, stamps the selected dialect, verifies zero edits, changes, checkpoints, alternatives, applied edits, redo edits, and cursor checkpoint, and then uses the normal canonical document Pack printer.

Package-owned GIS and VCS receipt laws now consume the codec and genesis together, execute each factory with its exact dialect, parse the resulting Pack/SPR through the matching typed artifact, and assert the minted document identifier, exact dialect, and every zero-history lane. Each law also supplies a substituted dialect and requires refusal. These laws are authored but have not yet run natively.

## Evidence status

`git diff --check` is green for the touched source set. `rustfmt` parsed the source, while repository-existing formatting differences prevent a whole-file `--check` receipt. No native law has yet qualified this source frontier. The long-running browser build started before later catalog/codec changes and remains diagnostic-only even if it completes successfully.

The remaining publication and zero-lineage transaction are owned by the Hub creation boundary. Mid-factory cancellation is presently observable only at the Hub operation checkpoints before and after the package future; the selected GIS/VCS initial snapshots are bounded empty constructors, but no native timing claim is made here.
