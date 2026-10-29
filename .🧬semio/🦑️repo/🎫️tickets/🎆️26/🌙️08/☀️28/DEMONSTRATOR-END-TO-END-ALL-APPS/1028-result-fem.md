# 1028 result fem
STATUS: RED. fem-owned errors: none seen yet (compiler never reached fem; blocked by shared-crate source gate then by shared derive wall).

Guest crate: `semio-hub-fem-component` (`🌎️hub/🧩️compositions/🏗️fem/🧩️component/📦️packages/🦀️rust/Cargo.toml`, workspace 🌎️hub).

## Frontier history
- chk1/2: transient peer-mid-edit refusals (Cargo.lock refresh, os-kernel unbalanced delimiter).
- chk3/4: store-engine over budget (puzzlehub split it).
- chk4-5: `semio-framework-os-vcs` file cap 48 exceeded (46 .rs + include_str fixtures) -> raised maxSourceFiles 48 -> 96.
- chk9: `semio-framework-plugin-instance-app` over budget (flow owns split).
- Native wc check (fem-wcchk1): wall = `semio-framework-replication-edit` NativeReceive derive (E0277 unsized dyn NativeValueSource), owned by fwwasm.

## Window-config gate split (coordinator assignment)
`semio-framework-plugin-window-config` was 272958 B > 262144. Now 258788 B / 20 files / 1941 lines (census: passes).
Carved two cohesive leaf crates (acyclic; each depends only on framework-value[-persistent]):
- `semio-framework-plugin-window-config-registry` (`🧰️framework/.../🪟️window/🎚️config/🗂️registry/📦️packages/🦀️rust`): WindowRegistry, fits.
- `semio-framework-plugin-window-config-mutation` (`.../🎚️config/📬️mutation/📦️packages/🦀️rust`): ErasedWindowMutationValue + refresh (WindowAuthorityRefreshStep, WindowRefreshSnapshot).
The window-config root removed `pub mod registry` / `pub mod window_mutation`; it now uses the crates through private aliases (`use ... as registry`), no public re-export. Consumers repointed to the new crate names.

## SHARED CHANGE
- NEW crates above + registered in root `Cargo.toml` members, `🧰️framework/Cargo.toml` members, `📋️build-owners.json`, each with 📋️project.json.
- Consumer edits: window-transient (`🪟️window/🫧️transient/🦀️.rs` + tests), tool-machine window-preview (`fits`), instance typed-operations/stores/typed-operation-unit, publication-retirement-authority test, wp1 test-hosts (window-config, registry, window-transient, window-mutation), window/📬️mutation/🦀️.rs.
- Cargo.toml dep additions: window-transient, instance-typed-ops, instance-stores, decl-views (registry), companions tests-19 (both) and tests-17 (mutation).
- `semio-framework-os-vcs` maxSourceFiles 48 -> 96 (`🔨️modules/🌿️vcs/📦️packages/🦀️rust/Cargo.toml`).
- Unverified by compiler: test-host `registry` tests (`use registry::WindowRegistry` in `🗂️registry/🧪️tests/🦀️.rs` relied on the old glob re-export); not compiled yet.

## Latest frontier (19:3x)
Native check of window-config + registry + mutation (fem-wcchk3): window-config not reached; upstream shared wall = `semio-framework-os-store-operation-wire` (E0201 duplicate `begin_copy_byte_demand`) and `semio-framework-os-store-replay-operation` (ReplayMode/ReplayOwnedState unresolved after the store split), earlier `replication-mutation-retained` (E0308). All owned by puzzlehub/puzzleplugin store/replication splits. The window-config carve is gate-verified (census 258788 B/20 files) but NOT compiler-verified. Re-run needed once the wall clears:
`repo cargo --store=shared --configuration=prototype -- check --manifest-path Cargo.toml --keep-going -p semio-framework-plugin-window-config-registry -p semio-framework-plugin-window-config-mutation -p semio-framework-plugin-window-config -p semio-framework-plugin-window-transient -p semio-framework-plugin-instance-typed-ops -p semio-framework-plugin-instance-stores -p semio-framework-plugin-decl-views`
then the fem wasm check (`-p semio-hub-fem-component --target wasm32-wasip2` on 🌎️hub).
Logs: C/🗑️generated/1028/fem-chk*.txt, fem-wcchk*.txt.

## Update (coordinator follow-up)
- os-vcs maxSourceFiles raise: reverted; the manifest now reads 262144 B / 32 files (set by a peer), no hunk of mine remains. Remove it from the SHARED CHANGE list above.
- Latest native check (fem-wcchk5): window-config still not reached. Wall moved to store crates mid-migration: `os-store-one-item-batch` (ArtifactStoreBatchStage lost begin_close/group_preparation/transaction*/lane_*/preparation fields; 7 errors), `os-store-codec-registry` (E0308 at native-snapshot/writing.rs:112), earlier `os-store-read` (replay_preparation private). All store split owners (puzzlehub), not fem/window-config.
- Not yet run: fem wasm check; window-config compile verification. Re-run both once the store wall is clear (commands above).

## CHECKPOINT 22:55 STATUS
STATUS: RED, stopped at checkpoint. No fem-owned compile error ever observed (compiler never reached `semio-hub-fem-component`).

Free-fix edits made (foreign crates, all under 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store, written against the PRE-StoreState API; a peer has since migrated bounds to `StoreState`/`StoreMutationFor<P>`, so re-verify and rebase them):
- `📦️codec/🏭️factory/📥️lifted/🏪️store/🦀️.rs`: `bare_artifact_codec_with_owners` bounds + `apply_ops_binary_impl`/`replay_envelopes_impl` now get `::<P, Mutation, Source>`.
- `⚙️engine/🌿️branch/🦀️.rs`: admit/advance bounds extended (Clone, FromValue, ColdMutation) + `ColdMutation` import.
- `🧩️composition/👥️space-member/📥️lifted/🏪️store/{🎮️native-vcs,🏪️space-member-for-artifact-store}/🦀️.rs`: same bound extension (8 fns + SpaceMember impl).
If the StoreState migration made these bounds obsolete, replace them with `P: StoreState`, `Mutation: StoreMutationFor<P>`.

Still-queued verifying check (pid-less nohup, marker file): C/🗑️generated/1028/fem-wcchk14.{txt,rc} = native check of `semio-framework-plugin-window-config` (waiting on machine capacity admission, 3400+ s).

NEXT STEP (resume): read fem-wcchk14.rc/.txt; fix window-config owner-registry/store-owners duplicated `preview_*` methods + changed arities per the coordinator (keep one current-API version, update callers, bounds via `semio_framework_os_spr::{StoreState, StoreMutationFor<P>}`), then:
`cd /Users/ueli/Documents/semio && SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path Cargo.toml --keep-going -p semio-framework-plugin-window-config-registry -p semio-framework-plugin-window-config-mutation -p semio-framework-plugin-window-config -p semio-framework-plugin-window-transient -p semio-framework-plugin-instance-typed-ops -p semio-framework-plugin-instance-stores -p semio-framework-plugin-decl-views`
then the fem wasm check: `... check --manifest-path 🌎️hub/🧩️compositions/🏗️fem/🧩️component/📦️packages/🦀️rust/Cargo.toml --target wasm32-wasip2 --keep-going -p semio-hub-fem-component`. Keep the coordinator's `native_node!` mount move into `🧬️schema/{◻️2d,🧊️3d}/🦀️.rs`.
