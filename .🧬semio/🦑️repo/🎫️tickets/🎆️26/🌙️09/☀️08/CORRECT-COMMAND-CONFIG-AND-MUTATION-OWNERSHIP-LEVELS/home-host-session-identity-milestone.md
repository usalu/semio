# Home Host Session Identity Milestone

## Ownership decision

The current authenticated user is native/browser host session state. `ViewModel.sessionIdentity` is the typed invocation and render projection of that live host value. Home and Space configurations no longer persist a second `clientId`/`clientName` authority, and Home no longer exposes `setClient` as an app command or mutation. The framework field remains optional because unauthenticated and non-authenticated apps are valid host states; Home and Space presence consumers reject absence or malformed identity when their behavior requires an authenticated actor.

`HomeConfig.directoryProjection` remains a Home-specific authenticated directory read model. It stores directory records, fold state and receipts, but no user identity. This milestone does not move Studio's separate `SpaceConfig.active_panel_tab` field.

## Exact boundary

The shared schema-first boundary is implemented in:

- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`
- `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`
- `🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧫️fixtures/🪟️resolved-host-context/🔣️.json`
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️view-context-capacity/🦀️.rs`

The browser host refreshes the value from its live identity reference for every resolved render/action view and every directory publication:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📇️directory-bootstrap/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📇️directory-home-bootstrap/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`

The native host's `live_view_state` overwrites any carried stale value for every render, command and action. The directory publisher also snapshots the current host identity immediately before invoking Home:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs`

The Home consumer boundary requires the current identity for editor/viewer render and real commands, routes `createStudio` through the explicit identity parameter, and retains the exact `ViewModel` inside the retained job payload rather than dropping its context:

- `✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️create-studio/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs`
- their adjacent Rust unit tests.

The removed Home mirror spans its Rust config/mutation and all JSON, TypeScript, GraphQL and Proto facets, the retired `set-client` command/module/tests, the authored retained fixture, generated Space manifest, and root source-negation verifier. The Space Studio config mirror is removed across the same five schema facets and Rust config/mutation implementation. Space `presenceHeartbeat` now has an empty command payload, validates the current host identity in direct and retained execution, emits no config mutation, and declares `HostOnly` publication:

- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/👥️presence-heartbeat/`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/🔣️.json`
- root `📜️script.ts`.

## Consumer semantics

- A missing identity aborts Home render and every real Home command before authoring, relay or navigation work.
- A carried stale identity is never a fallback. Browser and native hosts replace it with the current host identity, including replacement with absence after sign-out.
- Home roles are computed from the current `userId`: the owner receives author behavior, another authenticated user receives spectator behavior, and an absent identity is rejected.
- `displayName` is presentation metadata from the same live session record and cannot establish document ownership by itself.
- Space presence validates the same live identity and publishes no app configuration state.
- Directory projection records continue to be authenticated Home data and receipts. They do not own the current user.

## Retained-route boundary at this milestone

The Home retained catalogue contains nine migrated fixed scalar/config routes. Each admitted payload-bearing route is bounded by the shared 4,096-byte public invocation string limit; aggregate branch inputs are measured before the one-step reducer is accepted. `goHome` and `presenceHeartbeat` admit no scalar payload. The retained request now carries its exact current invocation context into `ArtifactRetainedCommandPayload`, so the reducer cannot reconstruct identity from persisted config.

Seven routes remain explicitly `BatchOnlyPendingRewrite`: `applyDirectoryEventPage`, `createStudio`, `bindSpaceFile`, `importSpace`, `deleteVirtualFileSystemNode`, `renameSpace`, and `foldDirectoryEvents`. No completion claim is made for them.

## Validation state

Registered launch order `311.236` r8 passed through the ticket Nx facade with zero cache hits:

- 13 shared resolved-host-context vectors, including strict session identity shape and guest rejection.
- 29 canonical browser directory/bootstrap source-oracle checks.
- 10 of 10 browser directory/Home runtime tests. These observe missing identity refusal before open, live identity A, stale-A-to-live-B replacement, and page-action capture of the current identity.
- One related renderer integration test passed with 107 unrelated tests filtered.
- Nx exited zero in 6.5 seconds. Evidence: `🗑️generated/home-host-panel-owner-oracle-r8.log`.

No native or full Home result after the session-identity changes is claimed. Registered `311.237` is the next native gate and must include the focused host identity law, Space current-identity direct/retained trace, and the complete 90-test Home suite. The previous full result remains 85/90 and is historical pre-identity evidence. The seven retained route rewrites begin only after the independent identity/route review requested at this milestone.
