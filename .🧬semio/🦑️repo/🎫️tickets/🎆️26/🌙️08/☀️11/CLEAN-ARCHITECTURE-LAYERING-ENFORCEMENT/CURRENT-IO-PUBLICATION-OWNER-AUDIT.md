# IO Publication Owner Audit

Observed 2026-10-03 in the actual general IO Rust source. This is an admission audit and a proposed ownership cut; no IO Rust mutation or runtime claim is made by this report.

The general IO module directly binds the OS Store implementation. The all-registry plan at line 1861 contains Store document codecs and dialect migrations, and its error at line 1880 embeds three concrete Store errors. The native document/snapshot registration functions at lines 1904 and 1911 acquire the Store assembly gate. The commit at line 1919 takes Store guards, then composer, subset, format, and native-snapshot write locks. It preflights all proposed rows before its first registry mutation.

Store coupling extends beyond that plan. Composer and subset/format registration accept concrete Store transaction references. Native snapshot registration and its map use Store codecs. The snapshot import/export and serializer constructors bind ArtifactSqliteSnapshot, ArtifactPack, and ArtifactDsl traits through Store. Moving the assembly plan alone would leave these real dependencies behind.

Canonical ownership: general IO owns its registries, format vocabulary, composer contracts, and a publication guard borrowing the general Async transaction. Store owns document codecs, migrations, and Store registry guards. The OS assembly composition owns the plan that combines these capabilities, selects a fixed lock order, and commits them together. Snapshot capabilities need an owned lower interface or an explicit upper implementation; copying Store behavior into IO would preserve the dependency in another form.

The current gate is blocking and registry preflight performs work under write locks. The controlled replacement must prove cancellation, finite work and ancestry bounds, contention reporting, guard lifetimes, and no mutation on refusal through the actual callers. The existing Async publication transaction is private and does not expose a final cancellation checkpoint. Any required extension needs a schema and a failing law before implementation; a copied ancestor walker or implicit control defaults are not an acceptable substitute.

The complete source and its SHA-256 are retained in io-publication-inputs/before.json. The current Value/Record controlled floor and native source epochs take precedence over mounting this later Rust extraction.
