# K4: `-C metadata` from the slot-level identity (stable across edits)

UC = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache`

## Finding (concern from K1)

K1 derived `-C metadata` from the portable key of the whole unit (`canonicalMetadataV1(portableKeyV1(keyInput,roots))`, `UC/📜️script.ts` ~:135). That key contains `sourceSha256` and the extern digests, so a body edit of crate A changed A's metadata, hence its StableCrateId and every symbol hash of A.

- (a) Early cutoff: the existing `UC/🧪️tests/🔭️cutoff` chain (K1: 4/0) still served B after a body edit of A, and a consumer compiled against the rebased A/B/C, because the rebase rewrites SVHs and a check consumer does not read def-path hashes of foreign crates. So the cutoff was NOT observably broken at that level. But it relied on metadata that no longer matches: A's rmeta carried a new StableCrateId while B's rmeta (served from the cache, rebased only on SVH) still embedded the old one. That is an unsound latent mismatch (anything reading the StableCrateId of a dependency: incremental sessions, proc-macro/`-Z` features, a later compiler version) and not the Cargo semantics (Cargo's metadata never depends on source bytes).
- (b) Symbol names: confirmed by construction, every edit changed all symbol hashes of the edited crate.

So: refuted as an observable break today, confirmed as a design flaw; fixed by moving to slot-level identity.

## Change

- `UC/🛰️fleet/🪪️identity/🟦️.ts`: new `slotMetadataV1(keyInput, roots, slotParts)` = `canonicalMetadataV1(JSON([toolchain, portable normalized command line with extern NAMES only (digest after `=#` stripped), slotParts.slice(1)]))`. `slotParts` (from `unitBasicsV1(...).slotParts`) = target, profile (opt, debug, da, oc, lto, panic, cgu, inc), `CARGO_PKG_NAME@VERSION` plus the relative member path for path packages, crate name, crate types / emit kinds, features, other cfg. The compiler release digest (slotParts[0]) is left out because the toolchain is already in. Excluded: source digests, extern digests, environment values. Without slot parts the portable key environment rows are used instead (never an empty identity).
- `UC/🔏️key/🟦️.ts`: `canonicalMetadataV1(identity)` (parameter renamed, doc says slot-level identity, `@see` the new function).
- `UC/📜️script.ts`: `basicsEarly=unitBasicsV1(...)` (slot id read from it as before), `metadata=slotMetadataV1(keyInput,roots,basicsEarly?.slotParts)`; import fixed.
- Namespace bump to `semio.cargo.unit-cache/v3` (output bytes change): `UC/🧬️schema/🔣️.json`, `UC/📖️admission/🧪️tests/🔑️key-stability/🔣️.json`, expectation in `UC/🔏️key/🧪️tests/🟦️.ts`. Old v2 entries (local and fleet keys) are never read.
- No build-generation risk: the wrapper is sealed per build (K1 section "Build-generation safety" holds unchanged); only builds that start after the landing see v3.

## Collision analysis

Two units share a metadata only if toolchain, whole normalized command line (incl. source path, edition, all `-C`/`-Z`/`--cfg`/`--check-cfg` flags, extern names), target, profile, package name+version(+member path), crate name, kinds and features are equal. Different crate types or emit sets (check vs build vs rlib vs dylib) differ through `kinds`. Different crates/versions differ through package and crate name. Residual (accepted, documented): same package, same flags and feature set, but different dependency versions/feature sets for the same extern names inside ONE graph (resolver-2 host vs target split of a dependency's features). Cargo's own hash would separate them through the dependency metadata; here they are the same `-C metadata` but different units. They are never loaded into one rustc invocation as the same extern (a proc-macro dylib carries its own copy). A later hardening could mix the producer's slot metadata into the dependents (needs the producers' slot ids from the extern paths, K2 lane).

## Tests (`repo test-file`, one at a time, load 45 to 75)

- `UC/🧪️tests/🆔️metadata/🟦️.ts`: 4/0 (548 s). K1's workspace-independence test (two workspaces, other roots/members/Cargo metadata: equal keys, local hits, own file names, restored rlibs link and run printing 42, plain rustc oracle) still passes. NEW unit test: `slotMetadataV1` is equal for other source/extern digests, other checkout root, other release digest; distinct for 11 variants (version, package, member path, rlib vs lib kind, metadata vs link emit, features, opt-level, target, toolchain, edition, extern name). NEW real-rustc test: check chain A,B through the wrapper with a logging inner compiler: after a body edit of A, A is `miss`, B is `local-hit`, A's `-C metadata` is byte-equal before and after the edit, differs from B's, and a consumer crate compiles against the rebased `.rmeta` files.
- `UC/🧪️tests/🔭️cutoff/🟦️.ts`: first run 1/3 FAILED only through the fixed 240 s per-test timeouts under load (rest cascaded through `process.chdir`), not through logic. Timeouts raised to 900 s; the re-run then stayed 30 minutes in the queue and hit the suite deadline ("Original Script deadline refused", 1798 s) without running, so `🔭️cutoff` is UNVERIFIED after K4. Equivalent coverage exists in the new metadata test (B local-hit after a body edit of A with A's metadata unchanged).
- Test harness notes: a first run died after 524 s with "Explicit finite owned Bun file selection required" (budget stretch above the schema ceiling under load, fixed concurrently by F2 at ~00:30). Metadata test fixture roots must be at least 4 characters (`portableRootsV1` ignores shorter roots): fixed in the unit test.

## Open items

- Not re-run after the final edit: `🔏️key` (`repo test-file` of `UC/🔏️key/🧪️tests/🟦️.ts` failed once at 105 s under load with an error not inspected; it only asserts the namespace string and the Python oracle corpus, neither touched except the namespace), `📖️admission/🔑️key-stability`, `🧳️portable`, `🔭️interface`, `🛰️fleet`. Re-run when the host is quiet.
- Slot-level metadata does not include producers' identities (see residual above).
