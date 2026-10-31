# Completion audit 2026-10-30

End-to-end verification of `@teaching/staffage`, `@teaching/staffage-react`, and `@teaching/staffage-site`. Every gate below is green.

## Typecheck

`repo tools exec bun -- x tsc -p tsconfig.json --noEmit` in each package.

| package | result |
|---|---|
| `@teaching/staffage` | exit 0 |
| `@teaching/staffage-react` | exit 0 |
| `@teaching/staffage-site` | exit 0 |

The library typecheck first failed because call sites had drifted from the current contracts. Those sites now match the contracts:

- The package entry re-exports the video types the module actually defines (`CodecSupport`, `EncodeOptions`, `EncodedClip`, `VideoCapabilities`, `VideoFormat`, `VideoFrame`).
- The structural harness builds a complete `ThermalInputs` (`externalWorkW`, `breathing`, `passengers`) plus `respiration`, `cycle`, and `representation`.
- Seven-modes indexes preset representations with a const denominator, then requires `status: "authored"`.
- The empty DXF scene carries `representation: { kind: "none" }`.
- Representation resolution narrows the unavailable 1:500 decision before reading `limitingSizeM`.
- The two-node Gagge cases pass `thermalState(...).thermal`, so `passengers` is present.

The React typecheck first failed because framework components saw two copies of `@types/react` (duplicate `Ref` identities). `tsconfig.json` now maps `react`, `react/jsx-runtime`, `react/jsx-dev-runtime`, `react-dom`, and `react-dom/client` to the package's own `@types` copies. The site typecheck passed on the first run.

## Library tests

From `🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript`: `repo tools exec bun -- x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts`.

**48 files, 845 passed, 1 skipped, 0 failed** (846 tests). The skip is the DWG reopen case that stays skipped when the DWG tool is absent.

The first full run failed one case: condensation saturation humidity ratio was 0.40–0.54 % off the CoolProp fixture (tolerance 0.2 %). `saturationRatio` used Buck vapour pressure without the moist-air enhancement factor `1.00062 + 3.14e-8·P + 5.6e-7·t²`. With that factor every saturation point is within 0.05 % of the fixture, and the mixing, enthalpy, leaf, and transport cases still pass. The full suite was then re-run green.

Earlier notes about thermal-legend, thermal-scenarios, thermal-transient, transport-field, frame-timing, and zip-reopen did not reproduce. Those files passed, including the transport-field case that previously hit the 5 s timeout (about 4 s on this run).

## React tests

From `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/📦️packages/🟦️typescript`: `repo tools exec bun -- /home/deck/semio/node_modules/vitest/vitest.mjs run --config ../../🧪️tests/🎚️config/🟦️.ts --configLoader native`.

**15 files, 99 passed, 0 failed.** jsdom prints that `HTMLCanvasElement.prototype.getContext` is unimplemented while export capabilities are probed. That message did not fail a test.

## Catalog check

Quiet check, 5 samples, physics on, from the staffage root. Exit code 0.

- manifest pass
- **107 / 107** assets pass

The asset list is in `check-v2-results.md`.

## Release site build

From the repository root: `repo tools exec bun -- x vite build --mode release --config 🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🏗️builder/🌐️vite/🟦️.ts`.

Exit code 0. Vite reported `built in 2.50s`. Output is `staffage/site/packages/typescript/dist/release`. Rollup printed one notice that `site` chunk is 1,058.87 kB after minification. The build still exited 0.

Two load failures were fixed before that green run:

1. Node's type stripper rejected parameter properties in the JSON decoder that the bundled config imports (`JsonSyntaxError`, `SegmentedJsonText`, `JsonSyntaxReader`). Those fields are now assigned in the constructor body, which strip-only mode accepts.
2. The config requires `SEMIO_BUILD_MODE` of `prototype` or `release` and `SEMIO_VITE_CACHE_DIRECTORY` inside the products cache partition. `repo tools exec` supplied the build mode from `--mode release`. The cache directory was set to `/home/deck/semio/.cache/b/p/m/vite/staffage-release` for the green run. Without that variable the config throws `Vite config requires its original bounded compiler cache`.

## Cleanup

The empty `generated` folder under this ticket was removed. The 2026-10-30 ticket day has no generated folders.
