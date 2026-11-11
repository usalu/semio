# Source Budget Refusal for semio-framework-job-context

## Where the budget lives

- Declared per crate in its `Cargo.toml` under `[package.metadata.semio.rust-source-boundary]` (`maxSourceBytes`, `maxSourceFiles`, `maxActiveLines`), rendered by the crate-tree renderer from the `sourceBoundary` policy in `the repo library caching policy.json` (262144 bytes, 32 files, 4000 active lines). Read in `cargo/units/descriptor`, enforced in `native-build/source/admission`: each read gets `maximumBytes = crate budget - bytes already read`.
- It is a deliberate hard limit, not a learned or stale extent. 378 crates use 262144; smaller crates use 16384 to 65536.

## Overage

- `semio-framework-job-context` (`job/context/packages/rust/Cargo.toml`): the lib closure (root `rs` 70002 bytes plus about 30 modules, including path-included continuation and retirement-context sources) is about 260 KB. `owner/rs` (3702 bytes) is the file that crossed the remaining 1960. Migration growth of about 23 lines in sibling files consumed the last headroom. The limit was not raised.
- Proposed split: move the largest cohesive slices into their own crates, in this order: `source/` (44905 plus 16849 close, 15943 child, 14133 return-origin, 7905 admission), then `decode/` (32305) with `owned/` (16658), then `encoding/` (31648). Any one of the first two frees more than 60 KB.

## Fixes made

- Typed refusal in the physical source read: `source.read-bytes-exceeded`, `source.read-files-exhausted`, `source.read-extent-mismatched` (en and de) in the process texts, carrying file, observed and maximum bytes. The cause keeps the closed refusal facts.
- Captured kernel entry (`command/entry`) now reports uncaught errors through `reportBoundaryErrorV1` (localized, exit code, no stack or source excerpt) instead of rethrowing. This covers `cargo:build-entry`.
- Tests: read fixtures got a `key` per refusal case (schema-extended); progress boundary fixture row for the typed byte refusal (en, de, exit 1); texts usage test passes; dependent tests updated to the new key.

## Verification

`repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-job-context` (log `🗑️generated/source-budget-verify.log`): source admission of job-context completed (unit 122, 134 files, 1911468 bytes read in the graph) and the build moved on to compiling. The root `rs` shrank by 63 bytes concurrently (another agent), which restored the headroom; the crate is still within about 2 KB of its limit, so the split above remains needed before further growth.
