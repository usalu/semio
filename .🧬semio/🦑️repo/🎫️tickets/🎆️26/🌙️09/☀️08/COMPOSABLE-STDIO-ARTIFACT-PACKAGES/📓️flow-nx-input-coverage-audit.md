# Flow Editor Fixture Nx Input-Coverage Audit

Captured: 2026-09-09T16:06:47+02:00. Read-only inspection of the existing private graph and a static minimatch receipt; no Nx graph rebuild, native build, or test task was run.


## Captured graph

`🗑️generated/nx-root-flow-recovery-8/project-graph.json` records `@semio-tech/flow-plugin` with project root `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust` and graph timestamp `1788962695167`.

Its project-level `default` named input contains both of these positive workspace patterns:

- `{workspaceRoot}/✏️s/🔌️plugins/🌊️flow/**/*.{json,semio,wit,wgsl,glsl,h,c,cpp,ts,tsx,js,mjs,cjs}`
- `{workspaceRoot}/✏️s/🔌️plugins/🌊️flow/**/*.rs`

It also contains `{projectRoot}/**/*`. The later exclusion patterns concern generated/build/cache locations and did not match any audited path.

## Effective target coverage

| Target | Captured declaration | Result |
| --- | --- | --- |
| `test-source` | `cache: true`, inputs `default`, `^default`, and transitive dependent outputs | Includes all current Flow JSON/TS/Rust sources through `default`; dependency inputs add coverage and do not remove it. |
| `child-identity-check` | `cache: true`, `inputs: null` | Uses the effective named default, which includes every audited Flow path. |
| `child-edit-check` | `cache: true`, `inputs: null` | Uses the effective named default, which includes every audited Flow path. |
| `add-widget-retained-check` | `cache: true`, `inputs: null` | This is the captured retained target; it uses the effective named default and includes every audited Flow path. |

The package root is intentionally narrower than the editor tree, but the two Flow-wide positive patterns bridge that topology. There is no fixture, schema, source-contract, or Rust omission caused by `{projectRoot}` being `📦️packages/🦀️rust`.

The static minimatch receipt at `🗑️generated/flow-nx-input-minimatch-receipt.txt` verified positive inclusion and zero matching exclusion patterns for:

- `📦️packages/🦀️rust/📜️script.ts`;
- `✏️editor/🧪️tests/🔬️source-contract/🟦️.ts`;
- `✏️editor/🧫️fixtures/🧬️schema/🔣️.json`;
- `✏️editor/🧫️fixtures/🧫️grant-frontier/🔣️.json`; and
- a retained editor Rust recipe path.

The current router reads the fixture schema in `add-widget-retained-check`; the source-contract TS reads the same module and the GrantFrontier fixture. The default-input coverage therefore invalidates cached source and custom child/retained tasks when any of those inputs changes.

No input-coverage defect was found. This is cache-key/source-topology evidence only; it does not constitute source-task or native-law acceptance.
