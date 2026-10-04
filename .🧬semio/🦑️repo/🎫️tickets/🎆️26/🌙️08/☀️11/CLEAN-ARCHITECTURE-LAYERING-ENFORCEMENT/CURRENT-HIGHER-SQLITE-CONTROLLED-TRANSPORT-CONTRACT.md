# Higher SQLite Controlled Transport Contract

The actual OS `ArtifactSqliteSnapshot` trait is defined in `🏪️store/🦀️.rs`. Its relational projection, reconstruction and native preflight methods currently return String. Native encode/decode and their helpers also erase TextError, PackError and SemioError to text. The erased codec export/import entry points use IoResult and therefore require the owned IoError cause floor before they can close the chain.

Pure relational methods will retain the canonical ValueError from the now-runtime-proven lower SQLite controller, Projection/Reconstruction and NativeEncodingBound. Native physical methods must preserve TextError source diagnostics and PackError typed causes through IoError. Text projection requires an actual admitted NativeEncodeControl even when the originating operation decoded input; its duplicated diagnostic ownership must settle into the same cumulative SQLite allocation ledger. This is a declared prerequisite, not a silently introduced ValueError-to-text conversion. Existing callback bridges currently use is_ok and discard the originating refusal object; the higher binding must retain that object and return it after releasing the native controller borrow.

The current whole source captures include the actual Store trait, both native physical helpers and both owned lower interfaces. All native production changes remain unmounted. No higher consumer success is claimed. Current Pack non-Value/Text variants still lack producer-authored refusal identities and cannot be assigned universally to invalidValue or classified from text.

The initial typed test boundary will use the full registered OS owner after its independent lower dependencies are coherent; incidental upstream diagnostics will be recorded separately from any reached new feature law. The earlier Semio attempt stopped at ordinary Nx prepare infrastructure RED, with no Cargo/compiler/new-law reached.

Full before sources and inverses: `🗑️generated/value-refusal/higher-sqlite-transport-owner-current-before-1.json`.
