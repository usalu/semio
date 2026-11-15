# Norm end-to-end chain audit (read-only, 2026-11-15)

Scope: targets named in steps 2-10 of `e2e-proof-chain.md`, the project.json that declares each today, the s dev catalog generator's view of the norm plugin, and browser journey coverage of norm in the s app. No builds or runs were executed.

## 1. Target resolution

| Named target | Declared in (absolute path) | Status |
|---|---|---|
| `@semio-tech/framework-os-dev:channel-version-descriptors-check` | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` | present |
| `@semio-tech/framework-os-dev:cold-boot-check-s` | same file | present; `dependsOn: ["activate-s-react-prototype"]` |
| `@semio-tech/framework-os-dev:activate-s-react-prototype` | not a literal key in any project.json | synthesized (see 1a) |
| `@semio-tech/framework-os-dev:serve-s-react-prototype` | not a literal key in any project.json | synthesized (see 1a) |
| `@semio-tech/s-dev-composition:generate` | `/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/📋️project.json` | present; `dependsOn: ["generate-stdio-format-claims", "@semio-tech/framework-os-dev:channel-version-descriptors-check"]` |
| `@semio-tech/s-dev-composition:catalog-check` | same file | present |
| `@semio-tech/s-dev-composition:verify-catalog` | same file | present |
| `@semio-tech/s-dev-composition:test-catalog-publication` | same file | present |
| `@semio-tech/s-dev-composition:check-generated` | same file | present |
| `@semio-tech/s-norm-composition-tests:verify-norm-document-contract` | `/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/📕️norm/📦️packages/🟦️typescript/📋️project.json` | present |
| `@semio-tech/s-norm-composition-tests:verify-norm-results-window-ownership-native` | same file | present (also `verify-norm-results-window-ownership`, `-oracle`) |
| `@semio-tech/norm-js:test` | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/📋️project.json` | present |
| `@semio-tech/norm-js:test-wire-twins` | same file | present |
| `@semio-tech/norm-plugin:test` | `/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust/📋️project.json` | present |
| `@semio-tech/norm-plugin:surface-render-test` | same file | present |
| `@semio-tech/norm-plugin:results-window-config-test` | same file | present |
| `@semio-tech/norm-plugin:component-budget-check` | same file | present |
| `workspace:components` (step 3) | `/Users/ueli/Documents/semio/📋️project.json` | present (root project, 56 targets) |

Result: no named target is missing except the two prototype targets, which are generated rather than declared. The root project.json does not declare any of the step 2-10 targets except `components`; the rest live in the owner projects above.

### 1a. activate / serve prototype targets

- `activate-<variant>-<renderer>-<profile>` is built in `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🎯️target/🟦️.ts` (`playgroundActivationTargetV1`) and requires the variant to exist in the generated playground catalog (`/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/📚️catalog/🟦️.ts`).
- `serve-<variant>-react-<profile>` and `dev-<variant>-react-prototype` are created by the playground preparation generator (`playgroundPreparationTargets`), as used in `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`.
- So `@semio-tech/framework-os-dev:activate-s-react-prototype` and `serve-s-react-prototype` resolve only if the generated playground catalog contains variant `s`. Not verified at runtime (no run was allowed).
- Ticket-level references to these names remain in `README.md` (lines 146, 704, 707) and `/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🤝️dev-collaboration/🧭️scope-and-dev-servers/🟦️.ts` (line 54).

### 1b. Path discrepancy in the proof chain

The proof chain's pin path `✏️s/🧑‍💻dev/🔖️channel-version/📌️pin/🔣️.json` does not exist. The real pin is at `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/📌️pin/🔣️.json`.

## 2. Channel values (current)

- Norm descriptor `/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/📕️norm/🧩️component/🔣️.json`: `executionProtocol.appChannelVersion = 24` (packageId `semio:norm`).
- Pin `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/📌️pin/🔣️.json`: `channelVersion = 25`.
- Host constant `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧾️app-channel/🔢️request-sequence/🟦️.ts`: `APP_CHANNEL_VERSION = 25`.
- Registry contract const `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🧬️schema/🔣️.json`: `appChannelVersion.const = 25`.

Mismatch confirmed: descriptor 24 vs host and pin 25. Re-describing the norm component is the only way to change the descriptor value.

## 3. (a) Would the s dev catalog generator include norm after the channel matches?

No, not from the channel alone. Two independent gates:

1. Channel gate: the descriptor-to-host check (`REGISTRY_HOST_APP_CHANNEL_VERSION`, stale-channel diagnostics in `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts`). The s `generate` target runs `GenerateScript`, which calls `renderCatalogFiles(..., "exclude")` (`.../📽️projection/🟦️.ts` line ~379), so a stale descriptor is withheld with a diagnostic rather than failing generation. However, `generate` depends on `channel-version-descriptors-check`, which is red today, so the target fails before generation.

2. Publication scope gate: `/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🔣️.json` has contributions with `packageRoots: ["✏️s/🔌️plugins"]` only. `publicationIncludesPackageV1` (`.../📣️publication/🟦️.ts`) keeps a manifest only if its path starts with one of those roots. The norm component that declares `deployment-directory = "📕️norm"` and `component-kind = "plugin"` is `/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/📕️norm/🧩️component/📦️packages/🦀️rust/Cargo.toml`, which is under `🌎️hub/`, outside `✏️s/🔌️plugins`. It is therefore filtered out before discovery. The s-side norm tree under `✏️s/🔌️plugins/📕️norm` (1028 Cargo.toml files) has no `deployment-directory` declaration for norm.

Consequences: the s catalog would omit norm even after channel 24 is re-described to 25, unless the publication gets a contribution covering the hub norm component (or the component moves under `✏️s/🔌️plugins`). Also noted: the generated registry file `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json` is `[]` (3 bytes, modified Nov 14 04:38), and the s catalog's `🤖️generated` directory does not exist yet. The hub component Cargo.toml points its playground `compositionConfigPath` at `/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️catalog/🧊️wgpu/🟦️.ts`, which does not mention norm or din4108.

## 4. (b) Playwright / browser journeys exercising norm in the s app

None found. 179 TypeScript files import Playwright (grep for `@playwright/test`, `playwright`, `playwright-core`, `chromium.launch`). Narrowing by norm tokens (`📕️norm`, `s.norm`, `din4108`, `@semio-tech/norm`, `semio:norm`) leaves two files, both false for browser norm coverage:
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts` uses norm artifact fixtures under Node, not a browser.
- `/Users/ueli/Documents/semio/🌎️hub/🧪️tests/👤️browser-actor-checks/🟦️.ts` matches only `normal` (actor module), a false positive.

Browser journeys that do exist for the s app (none reference norm):
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🧪️tests/🌐️browser/🧭️journey`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/💾️persistence-journey` and `.../🌄️drag-journey`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️testing/🚶️journey/🟦️.ts` (generic harness)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🧭️journeys/🟦️.ts` (generic two-human journey)
- `/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🤝️mcp-client-e2e` and `/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🤝️dev-collaboration`

Limits: this was a text search. A journey that loads a catalog-wide plugin list without naming norm would not show up here, so the generic harnesses above were checked only at the level of their names and a grep for plugin enumeration.

## 5. Summary of gaps for the chain

- Step 2 gate is red because the descriptor is 24 and the pin is 25.
- Steps 5-6 cannot pass with norm included while the publication excludes the hub component.
- Steps 7-9 depend on generated prototype targets; their existence is unverified.
- Step 10 norm-only proofs all exist as declared targets; none is browser-based.
- No browser journey covers norm in the s app.
