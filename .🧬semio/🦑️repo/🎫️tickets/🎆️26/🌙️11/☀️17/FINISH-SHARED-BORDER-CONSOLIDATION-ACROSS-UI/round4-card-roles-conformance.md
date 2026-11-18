# Round 4: conformance and accessibility fixtures for `cardStack` and `card`

Paths under `🧰️framework/🔨️modules/🖱️ui/🧬️contract/` (`FX` = `🧪️tests/🧫️fixtures`).

## Fixtures added
- `FX/🧪️conformance/🖥️composite/🗂️card-stack/` (`📸️snapshot.json`, `🎯️expect.json`): a `cardStack` of two `card` containers; card "Alpha" carries a footer chip (`description` "3 items"), card "Beta" none; each has one text body. 5 nodes, no actions.
- `FX/🧪️conformance/📇️catalog.json`: composite case `card-stack` registered (corpus count 77 -> 78 in the Rust `🛡️limits/🧪️tests/🧬️conformance` and the TS `🧪️tests/🔬️conformance-corpus` assertions).
- `FX/♿️accessibility-projection.json`: three role rows (`cardStack` group, `card` group, pressable `card` button) and three document nodes 12-14 (stack, Alpha, Beta) with their pre-order projection rows (role `group`, card label from the container label, no description, not focusable, not announced).

## Root-cause fix
- TS `🖼️presentation/♿️accessibility/🟦️.ts` `uiAccessibilityProjectionNodeV1` took a container label only for `section`/`group`; Rust also takes it for `sheet` and `card`. TS now matches Rust (`sheet`, `card`), so a card announces its title in both languages. The Rust role mapping already named `cardStack`/`card` as `group`.

## Consumers and schemas
- Runners of the corpus: Rust contract harness (`🛡️limits/🧪️tests/🧬️conformance`), Rust accessibility laws (`🧪️tests/♿️accessibility`, run by `🧪️conformance` crate), TS `conformanceCorpusSelfTests` and `accessibilityProjectionSelfTests` (invoked by the contract package script), and the wgpu companion `🧪️tests/🧪️conformance-corpus` which mounts every catalog accept case (so the new case is also mounted natively). TUI does not iterate the catalog.
- Schemas: the `ContainerRole` TS schema projection row, the Rust `ContainerRole` enum, kernel json copy and typed wire already list `cardStack` and `card`; no fixture JSON schema enumerates roles (the catalog is validated by the serde catalog type and the filesystem-ownership checks). The glyph `🔣️taxonomy.json` is a generated registry that already lacks several composite cases, left alone.

## Verification
- TS (ticket-local `vitest-card-roles.config.ts` + `card-roles-selftests.test.ts`, since the self-tests are plain functions called from the package script): 2/2 pass (catalog 78, accessibility projection twin).
- Rust `semio-ui-contract-conformance` (`--lib -- accessibility`): 12/12 pass, including the shared-fixture role rows and the document projection with the new card nodes.
- Rust `semio-ui-contract-limits` (`--lib -- conformance`): 6/6 pass; the corpus has 78 cases and every component variant is covered.
- Rust wgpu companion `semio-ui-wgpu-laws --lib -- conformance_corpus`: the mount test's case count assertion (`🧪️tests/🧪️conformance-corpus/🦀️.rs`, 55 -> 56) is updated and the test now passes that assertion, but all 12 corpus tests in this companion abort afterwards on an environment/infrastructure fault that is independent of the new case: "controlled retirement abandoned before terminal-empty" (`🌱️value/♻️retirement/🎮️controlled`) when a mounted `Ui` is dropped, and in two tests "the tree paints" / hover reason (no painted output in this environment). The companion never opens a `UiArenaFixture`; fixing that means giving the companion a `semio-ui-contract-values` testkit dev-dependency and a fixture per test, left open. The `--test original_clipboard_ingress` target of that package also fails to compile (`WorkerJobSessionPreparation` import), unrelated.

## Root-cause test fixes made on the way
- The contract fixtures deserialize components whose text lives in the finite UI arena, which only works under a registered lifetime. `every_component_implies_the_role_the_shared_fixture_declares` and the five `🛡️limits/🧪️tests/🧬️conformance` tests now hold `UiArenaFixture::new()`; `semio-ui-contract-limits` gained the dev-dependency `semio-ui-contract-values` (feature `testkit`), the same pattern as the hosting fix in round 3.
- Other failures in `semio-ui-contract` lib and in the conformance crate (`refresh`, `root_reservation`, `typed_descendants`, `whole_patch`, `patch_pages`) have the identical missing-arena cause; not touched here.

## Files
- `🧪️conformance/🖥️composite/🗂️card-stack/📸️snapshot.json`, `🎯️expect.json`, `📇️catalog.json`, `♿️accessibility-projection.json`
- `🖼️presentation/♿️accessibility/🟦️.ts`, `🛡️limits/🧪️tests/🧬️conformance/🦀️.rs`, `🛡️limits/📦️packages/🦀️rust/Cargo.toml`, `🧪️tests/♿️accessibility/🦀️.rs`, `🧪️tests/🔬️conformance-corpus/🟦️.ts`
- `🧪️tests/🧪️conformance-corpus/🦀️.rs` (ui module)
- Ticket inputs: `card-roles-selftests.test.ts`, `vitest-card-roles.config.ts`. Run logs were kept in the session scratchpad and deleted.
