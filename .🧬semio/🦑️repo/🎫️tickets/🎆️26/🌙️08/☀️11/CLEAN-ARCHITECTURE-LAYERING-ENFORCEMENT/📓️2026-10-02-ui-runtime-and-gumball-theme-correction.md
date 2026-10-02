# UI Runtime and Gumball Theme Correction

The original serial registered UI replay2 terminated exit1 in1m48s Nx after current normal workspace preparation3 (26.5s) and mandatory locked root metadata2 exit0. Both target feature strings and all original native assertions remained unchanged.

| Original target | Compilation | Actual runtime |
| --- | --- | --- |
| `@semio-tech/ui-rs:test` | 36.83s, successful | 297 selected/run;296passed,1failed,0skipped;2.053s |
| `@semio-tech/ui-rs:test-wgpu-engine` | 30.84s, successful | 767selected;313run,312passed,1failed,0skipped;454not run because of original fail-fast;9.629s |

The complete output is `🗑️generated/goal-stdio/current-ui-native-original-targets-2.log`. Default Nextest ID `a1eafa03-0eb0-404b-81d1-6429ad36287c`, retained trace `semio-nextest-7sspGm`. Engine ID `d32e456e-4c84-424b-a3d7-43b2997a5a99`, trace `semio-nextest-dOUbq9`. Successful compilation proves the prior32 binding/feature errors are no longer the current blockers; neither target has a complete passing runtime outcome.

The unchanged default law `wgpu::theme::tests::no_wgpu_target_paints_a_hand_written_colour_literal` reports three actual Canvas2dHost gumball paints: white pivot disc, slate boundary, and amber active gesture. The unchanged engine law `wgpu::events::control_commit_tests::every_retained_control_commits_its_own_guest_action` refuses the `ring-reads-its-own-circle` discrete control because it emitted press keys `main/1:5=false,true`. The UI owner investigates that genuine behavior independently; no expected fixture, selector, count, feature or deadline was weakened.

## Appearance-Owned Gumball Paint

The existing styling schema/palette already owns foreground, muted foreground and warning roles, with appearance-specific generated native Theme fields and matching React CSS variables. Gumball paint now receives the actual caller's Theme: pivot uses `theme.text`, boundary `theme.text_muted`, and pending gesture/ghost `theme.warning.with_alpha(0.9)`. Both Scenes callers pass `ctx.theme`. The original paint unit supplies explicit light appearance; an additional native law requires light and dark foreground/boundary roles to reach actual vector paint. Geometry, gestures, actions, hit testing and original assertions are retained.

The React twin binds `--foreground`, `--muted-foreground` and `--warning-border`, preserving ghost/marker0.9 opacity. The root styling CSS defines those variables; native Theme reads matching generated chrome/outcome palettes. No channel literals were relocated into another constant or schema, no fallback color or source exclusion was introduced, and the original no-literal native law remains byte-identical.

Full before bytes are retained at `ui-gumball-theme-before.json`; exact inverse witnesses at `ui-gumball-theme-preservation.json`. These source changes have not yet been compiled or executed through the affected renderer suite.

| Actual source | Before SHA256 | After SHA256 |
| --- | --- | --- |
| Canvas2dHost WGPU | `037bb074f43e9bcb6efa15c1e2b7afebd5fe3adce1ba7addfc6f47d5a0b3841e` | `40a473d6ee9f3984f641f2e9c56036af9ed58f43dd00f2c960093fe2ff4a5fd8` |
| Canvas2dHost React overlay | `70e844ac0932db109117066dec68ce9926e2719632e8feaf2c8584c2ec371de0` | `eebf09e21fc00346af2ca53f06af4a75afb49549973b834e784208092e867675` |
| Scenes WGPU callers | `7f26767d14d30710f35180a2bead8d16c58a9d0edd839d80cdd24d25086b07d9` | `ee69357455423f5cebadd8792f242df6c8881b3fa094390e11d46b5a25a8d398` |
| Original Scenes canvas2d unit plus additional role law | `6e8206a31d6b2e0add390449039d3845e459845256cf2d60b0d904224aa7f420` | `f31bc4ff4f56604742f0637419a018997913026059fb7c97f14bfdfafc88cda7` |
| Original lower no-literal law | `11113e5a6efddc30fed19ed6245cf2713f5892979db53349d41e1b51e3992e8b` | unchanged |

All five exact inverse projections match original full files. Additional role-law text is excluded only from the declared inverse reconstruction, not native execution. This is source preservation evidence, not a physical-deletion or globally coherent native epoch.

## Original Default UI Replay Three

The unchanged complete registered `@semio-tech/ui-rs:test --skip-nx-cache` terminated exit0 in10.6s Nx (10.3s critical path), cache skipped, after the real palette correction. Current native compilation0.21s; original long Nextest selected and executed297laws,297passed,0skipped in0.628s. ID `014037fa-338e-466d-a664-9cba08c1eeec`, trace `semio-nextest-XjLkkc`, output `current-ui-native-default-3.log`. The original no-literal law now genuinely passes. The engine767-law route remains pending the independent Ring contract resolution; this default pass does not compile the higher Renderer paint API or execute its new appearance-law.

Actual retained Nextest binary metadata identifies executed library binary `semio_framework_ui-b440762e95ad17b1` under the caller-owned compiler directory. Every one of its310 rustc-owned checksum-bearing input rows in the corresponding `.d` independently matches current physical input length and neutral TypeScript BLAKE3. Verification output `ui-default-replay3-current-compiled-source-origins.json` contains zero mismatches. This proves the executed default package's current own inputs; dependency fingerprints and higher Renderer native closure remain separate proofs.

## Original Renderer Full Compiler Refusal

The unchanged registered `@semio-tech/framework-renderer-wgpu:test-wgpu-unit --skip-nx-cache` terminated exit1 in4m58s Nx (4m57s critical path;4m51s native owner task), with no runtime admission. Full output `current-renderer-wgpu-original-full-1.log` contains212compiler diagnostics:159 E0599,46 E0308,6 E0277,1 E0614. The dominant actual refusal is the production and test `WorkerCell<Ui>` calling `with/state` whose implementation still requires `T:Default`; lower Ui now requires explicit Locale/Terminology authorities. The existing cell lazily calls `T::default()` under both production OnceLock and isolated test storage. The type/borrow cascades remain recorded as compiler outcomes rather than independently diagnosed causes.

Exact physical owner: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`, lines32–97. Captured original full bytes SHA256 `e28354c3a8f13a67875a80052e8ed9d30a807bae8a04eba63a93768e58eb2a1e`; all212 diagnostic blocks and that source are retained in `current-renderer-worker-cell-compiler-red.json`. First observed failure is the unchanged `wgpu-ui-command-wiring` unit1545 calling `UI_ENGINE.with`; production Interpreter1207/1220 and other callers expose the same unmet initialization bound.

Root owns the actual explicit cell initialization correction; no Ui Default, implicit locale/terminology, feature or selector restoration is authorized. The sole compiler queue is drained and held until that source handoff. This full renderer refusal does not invalidate the separate completed lower default297-law runtime; higher paint API/light-dark witness runtime remains unproved.

## Original Engine Replay Three

After the independently captured Ring fixture contradiction and strict continuous-contract correction, the unchanged complete registered `@semio-tech/ui-rs:test-wgpu-engine --skip-nx-cache` terminated exit0. Actual compiler2m01s; long Nextest767selected/run,767passed,0skipped in92.389s, original features/deadlines/full scope retained. Nextest ID `36ff54fd-30c8-4cc9-84ad-3fcec4c173c7`, trace `semio-nextest-jnGjAw`. Output `current-ui-native-engine-3.log`, Nx6m20s uncached (native owner task5m37s; graph/preparation separate).

This current bounded runtime closes all original lower UI engine laws, including the actual Ring press lifecycle. See [Ring authority and exact preservation](./📓️2026-10-02-ui-ring-press-contract-runtime-refusal.md). Root's lower Ui setter and higher worker admission correction begins after this terminal epoch; subsequent affected lower/higher suites must be distinguished from this receipt. The original complete lower Value target is queued independently while those source changes proceed.

The actual retained engine binary `semio_framework_ui-20aa67021ca3ec13` independently matches all440 rustc-owned checksum input lengths and neutral TypeScript BLAKE3 values, zero mismatches. Its current source-origin observation was completed after the terminal and before the later cell/UI epoch: `ui-engine-replay3-current-compiled-source-origins.json`. This includes real engine test/corpus inputs, not guessed current names or earlier binary metadata.
