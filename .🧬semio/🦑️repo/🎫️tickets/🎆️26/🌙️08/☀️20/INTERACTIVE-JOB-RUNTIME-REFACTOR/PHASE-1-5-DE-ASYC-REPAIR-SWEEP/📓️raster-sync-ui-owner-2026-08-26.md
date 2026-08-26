# Raster Synchronous UI Owner — 2026-08-26

## Scope

- Actual package path: `✏️s/🔌️plugins/🖨️raster`.
- Excluded pre-existing staged oracle JSON and binary mutation component.
- Preserved the binary component's two staged retained `JobFault` payload conversions; its unstaged diff remained empty.
- Kept the genuinely suspending Wasm `RasterArtifactVcs::new` constructor asynchronous.

## Static Contract

The PHASE-1-5 ticket `📜️script.ts` now exposes a read-only, UTF-8 byte-preserving production/test classifier:

```sh
bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYC-REPAIR-SWEEP/📜️script.ts report-immediate <path-prefix> [excepted-file...]
```

It excludes attributed test functions while deliberately including immediate helpers inside test modules; it classifies a body containing `.await` as genuinely suspending. Its language-neutral fixture assertions cover UTF-8 spans, `cfg(test)` block recognition, attributed-test exclusion, immediate test-helper inclusion, and production inclusion.

Raster baseline with the protected binary file excepted was `production=427 test=164 suspending-or-excepted=3`: 411 production declarations plus 16 immediate test helpers. The migration was emitted as apply-patch documents and applied without source-side writers. Its exact post-state is `production=0 test=164 suspending-or-excepted=3 files=0`; raw parity is `async_fn=167`, `async_test=164`, `.await=1`. The three excepted/suspending declarations are the two protected binary functions and the Wasm constructor.

The same ticket script now exposes `audit-raster-envelope-contract`. It loads the exact production source bundle used by the root interactivity verifier, instruments every conjunct in `toolJobRasterEnvelopeCallerRetainedExact`, and reports each rejected predicate without changing the verifier.

## Applied Production Boundary

All non-test, non-excepted Raster declarations are synchronous. The sole `.await` in the Rust tree is the Wasm constructor's `VcsArtifactApp::new(...).await`.

The four Raster editor panels return `UiAssemblyResult<BuiltNode>`. Artifact and Masks retain semantic admission through `UiFixedList`, `UiText::try_from_str`, and `try_push`. Inspection and Catalogue now build rows through the shared fixed-capacity `ui_node_list` helper and assemble them with `PanelTreeBuilder`; their legacy `UiNode`, `ui_stack_vertical`, `ui_declarative_sections_to_tree`, and `vec!` render boundaries are removed.

## Checks

- `bun .../📜️script.ts self-test`: `17/17 passed`, including positive and negative exact-contract instrumentation fixtures.
- `audit-immediate`: passed with the exact post-state above.
- `audit-fixed-ui <inspection> <catalogue>`: passed for both files.
- The first exact Raster envelope audit was `177/183`. The only rejected predicates were six stale verifier anchors requiring `async fn` for the already-migrated immediate snapshot text/binary helpers. No retained decoder, recursive retirement, fixed-page ingress, initializer, cancellation, acknowledgement, close, or handback predicate was missing.
- The verifier contract and its canonical Raster fixture now require the six synchronous signatures. Four substring-sensitive private-function checks also explicitly reject their `async fn` forms, so this is a stricter signature contract rather than a relaxed string match. Six hostile sync-to-async mutations were added; the self-test census increased from 424 to 430.
- `audit-raster-envelope-contract`: `187/187 predicates accepted` against the live Store, Raster codec/domain, Raster editor, Raster Wasm bridge, mounted serializers, and shared plugin sources.
- `bun ./📜️script.ts verify interactivity tool-jobs --self-test`: `self-tests=430 clean`.
- Full JSON coverage wrote `📝️raster-tool-jobs-full-coverage-2026-08-26.json` before returning exit 1 for unrelated repository work. The report has `169` failures and zero failure strings containing `Raster`; the former `Raster .spr/.ops envelope caller` failure is absent. Its exact census is `production-hosts=50`, `production-invocations=50`, `production-rows=776`, `literal-registrations=648`, `fixture-hosts=1`, `fixture-invocations=2`, `fixture-rows=4`, `admitted=162`, `unique=774`, `factories=17`, `registrations=0`, `dispatches=3`, `aliases=4`, `remaining=723`.
- `cargo fmt --manifest-path .../raster/Cargo.toml -- --check`: parsed the complete Raster source tree, then returned a formatting diff that includes the protected binary file and unrelated pre-existing formatting; no formatter write was run.
- Fresh isolated native `CARGO_INCREMENTAL=0 cargo check --locked -p semio-s-plugin-raster --lib --message-format short`: crossed the previous `bitflags`/wit-bindgen failure and `semio-framework-plugin`, then failed before Raster in `semio-s-plugin-stdio` with exit 101: `could not compile semio-s-plugin-stdio (lib) due to 2299 previous errors; 105 warnings emitted`. First emitted errors include stdio E0728 stale awaits and E0277 non-Future awaits.
- Independent component-target `CARGO_INCREMENTAL=0 cargo check --locked -p semio-s-plugin-raster --lib --target wasm32-wasip2 --message-format short`: crossed bitflags/wit-bindgen and `semio-framework-plugin`, then failed before Raster in `semio-s-plugin-stdio` with the same 2299 errors and 106 warnings, exit 101.
- Browser/runtime and timing were not run because neither native nor Wasm compilation reached Raster.

## Remaining Work

The Raster source/verifier envelope failure is closed with exact static and hostile-fixture evidence. Native Rust semantic acceptance, Wasm compilation, browser runtime, and timing remain externally blocked by the upstream stdio compilation failure. Test async declarations remain intentionally untouched.
