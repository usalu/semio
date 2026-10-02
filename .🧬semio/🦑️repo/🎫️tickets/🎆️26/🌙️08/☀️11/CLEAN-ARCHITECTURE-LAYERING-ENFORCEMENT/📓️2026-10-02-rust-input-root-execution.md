# Standalone Rust Input Root Execution

The public inspectRustSourceInputs helper checks authored target components with lstat, but its ordinary non-dot branch joins them below the caller root without first checking that root and its ancestors. The higher-level policy bootstrap already checks its root; that guard does not protect independent callers of this public helper. A regular leaf below a linked root or ancestor can therefore appear admitted.

The schema-first contract requires fresh raw root spelling and physical no-follow ancestry before target inspection, including an empty target list. Valid relative native roots remain resolvable; raw dot/dotdot, NUL, empty and drive-relative spellings refuse before normalization. Root and every ancestor must exist as real directories. Target failure shapes and the existing executor bootstrap guard remain unchanged.

Closed portable cases and an independent Node filesystem oracle are being prepared. A tiny rustc witness will distinguish actual OS-followed linked paths from permitted authored ownership after Root releases its current compiler window. No root provider change or execution pass is claimed yet.

## Actual producer and owned correction

The closed contract contains twelve raw-root rows, a root-replacement/cancellation record and two native byte witnesses. Independent Node `fs.lstatSync` checks every raw root ancestor; independent Node `fs.readFileSync` also selects the actual marker reached by each host filesystem. Two rustc include binaries print that exact marker. The cancellation-prefix witness permits only the declared/foreign authored marker set and requires exact Node/Rust parity, avoiding assumptions about Windows junction/dotdot traversal.

Before provider changes, the actual owned script returned RED: one schema/oracle law passed, one producer law failed, thirty assertions in 1.97s. All Node classifications and both native binaries completed before the ordinary linked-root producer wrongly returned admission. An earlier schema preparation did not claim a provider pass.

The public helper now validates raw spelling before resolving a native root and freshly lstats the complete absolute ancestor chain before any target, including an empty list. Every ancestor must exist as a nonlinked directory. Component inspection, existing failure records, cancellation and its bounded yield policy remain intact; no cross-invocation authority cache was added.

The current normal registered Nx target `@semio-tech/repo-lib:test-rust-source-roots` is GREEN: three laws, fifty-five assertions, Bun 2.16s, Nx 3.1s. Original 45s runtime budget remains unchanged. Coverage includes every twelve-row ordinary/empty target pair, fresh replacement of an admitted root with a link, and cancellation before target admission. Own canonical script/project/package route, semantic member and authored JSONC seed 900.05775 are registered; generated launch bytes were not changed. This does not claim the full Rust direction suite or integrated typecheck; Root owns those receipts.

Normal full current `lint-rust-source-direction` physical census is queued for actual present-source edge/problem evidence. No prior whole gate result is reused as current.

The complete registered whole-tree census finished RED9m31s: 86 strict edges/12 problems, 24135 Rust files/61363 references. Exact groups/owning paths are retained in 📓️2026-10-02-current-full-rust-source-census.md. No deadline/authority waiver or Cargo job was used.
