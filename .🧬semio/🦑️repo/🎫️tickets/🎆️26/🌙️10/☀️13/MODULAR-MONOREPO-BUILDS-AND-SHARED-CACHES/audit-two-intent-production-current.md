# Production Two-Intent Completeness Audit

2026-10-16. Read-only moving-Source census. No tests/Cargo/Native/SDK/compiler/build/network/lease/image effects and no files modified except this report. CPU manifest/policy migration remains live; findings must be reread against its final freeze. Exact hashes below identify this observation, not aggregate acceptance.

## Current Omissions Outside Canonical Manifest Cleanup

- P1: Hub production actor provenance selector `🌎️hub/🏗️bootstrap/🎭️actors/🟦️.ts:42` requests exact `--profile wasm-release`; first-party production runtime verifier `library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts:253` demands that same argument tuple. Fresh release-profile component evidence will be rejected by these current exact consumers. Migrate both actual selected argument construction and verifier expectations coherently; descriptor argument construction adjacent at actors43 must also be checked against the actual consumed release emitter tuple.
- P1: Hub stdio Script line47 COMPONENT_PROFILE remains wasm-release and lines1036/1045 use it for actual Cargo args and artifact output coordinate. Its capture schema line19 and trusted-catalog schema line395 require wasm-release receipts while current shared component receipts now use release/prototype. This can both fail compiler admission and reject new valid receipt DATA. Update actual producers/consumers/committed fixtures as one authored contract, no legacy acceptance union.
- P2: Play project declaration lines298–375 retains five actual wasm-release lane/profile jobs. It is current authored native-pass job DATA, not a historical fixture. Migrate both lane and profile to release coherently with actual core component-pass and WASM producers; validate resulting mode-specific output contracts. The site binding type and runtime schema were actively corrected to prototype/release during this audit, so their earlier stale occurrences are not reported as current defects.
- P2: Config mutation bridge Cargo.toml line26 retains `[profile.prototype] inherits=dev` in its own workspace root, outside the canonical cleanup roster. Its actual Script routes through repository mutation inventory -> shared Cargo launcher with explicit mode. It is not a third mode, but duplicates the now centrally consumed prototype declaration and prevents claiming all production manifests are declaration-free. Remove via the same canonical policy ownership after including this real workspace owner in census.

All above locations were reread immediately before recording. Root was notified while CPU was still migrating; a later reread may close them. No stale in-flight Source is elevated to final freeze defect.

## Classification and Reviewed Current Routes

Core build-mode schema is exactly prototype/release. Profile planner/rendered configuration consumes explicit mode/producer/target/operation and role policy; settings keys release-native/release-wasm are internal role matrix coordinates, not accepted external profile aliases. Rendered Cargo selectors use profile=mode, prototype inherits dev only as Cargo backend configuration, and target/host/package constraints stay explicit. Native artifact, component, browser and descriptor-emitter requests route through the shared consumed configuration interface in current Source. Their actual compiler behavior still needs current aggregate freeze/runtime qualification; interface presence alone is insufficient.

WASM build Source lines75–81 maps pack and cargo `--profile` from exact BuildMode and builds explicit canonical selector arguments; lines99–125 use mode-specific output directory and forwarded consumed configuration. wasm-pack custom-profile support is not inferred from this Source reading, although existing authored comments name its supported custom-profile version. Component pass handles explicit pass.profile through profileBuildModeV1 and generation-scoped compiler arguments; this audit did not execute either producer.

Package export `./process/native-release` names the existing native component-pass API semantic operation; it is not a selectable third compiler profile. Likewise native-release Nx target naming is not automatically a Cargo mode alias. `.vscode/launch.json:86225` display name wasm-release launches `:wasm --configuration=release` at86228, so this is cosmetic old wording, not an active old selector. The separate native-release target command at60953 is a target name and needs its own target semantics, not blind string replacement.

Historical ticket launch rows and oracle/evidence corpora retain Cargo dev/test profile environment overrides. They were not classified as production entry modes. Hub Script synthetic receipt literals containing wasm-release should be migrated when they model current accepted component receipts; deliberately old receipt rejection/oracle evidence must remain explicitly distinguished. No wholesale fixture rewrite or test pass is claimed here.

## Acceptance Boundary

CPU's canonical239 manifest census/removal and live new32-law/strict collectors are separate Root/CPU evidence. This audit supplies only Source omissions and exact selected hashes. Two-intent completeness requires a final current producer/consumer/receipt/editor/help/schema census after all above callers freeze, aggregate strict and real bounded production native/component/browser/emitter qualification. Current Graph phase BOTH10/78 and strictBOTH0 do not qualify profile consumption or physical compiler execution.

## Exact Scoped Source Roster

- `🧰️framework/🛍️products/💻️os/🎚️config/🏭️bridge/Cargo.toml` — SHA-256 `8d1a2f7ac16b1c6b74c8de8949f75890d61cfcf9f7b92982faae74adb2ec2f76`
- `🏢️semio-tech/🎡️play/📋️project.json` — SHA-256 `3b25d187be6c78b0d4496920d72d8e54eb14ce437df92e68ed2fef25df278511`
- `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages/🟦️.ts` — SHA-256 `89e74c7183931d9707d0596239a193816c5a18d63ebfae77fbba779bd20d994c`
- `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🧬️schema/🔣️.json` — SHA-256 `8ca12f08ccd718fd20de8e865020305e6d8c4aaa3e9995b8663e94f11d8a6d06`
- `🌎️hub/🏗️bootstrap/🎭️actors/🟦️.ts` — SHA-256 `e31e6468a71b209514119a7b08c8d4a300b981e83e9d12fc9db7903f808ce5e4`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts` — SHA-256 `5007bc78629423803ddd7b862850d1e6c1cfaf70defb9680bcd17abbd4fad380`
- `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts` — SHA-256 `f30803df612bb34c03c13a8a7fcb34ff1cc9820fa75b41ad556427fca9b6a3d1`
- `🌎️hub/🧩️compositions/🗄️stdio/🧩️component/🏗️compiler/♻️capture/🧬️schema/🔣️.json` — SHA-256 `75245a835d2b28310e600a8f2d4b7220ceff7836691f9b4bf60a5250b87abc82`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️.json` — SHA-256 `d97b85c6c259946ad5fd79ead2d524a17d9ecfecb25f7efb9ffbad75778f143b`
- `.vscode/launch.json` — SHA-256 `4d80c6582b9543bb5dfe6e2adaae1f727f1733bdc778187e0092787edc09de24`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts` — SHA-256 `1b71854ee2767a39d24185227039ea1ae4da99c3204e9f7059f21e4a21739e4d`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts` — SHA-256 `18f0fdd81edc49097b27c04f392bacd40c050b4d65aed9da04328b2421a4a895`
