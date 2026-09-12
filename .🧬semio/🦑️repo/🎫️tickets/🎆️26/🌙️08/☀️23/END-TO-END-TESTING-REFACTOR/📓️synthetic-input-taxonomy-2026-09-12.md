# Synthetic Input Testing Taxonomy — 2026-09-12

The JCO guest and host stand-ins and the scale actor are synthetic programs supplied to runtime tests. They belong to OS fixtures. Assertions remain direct leaves under OS tests. The scale component build still writes to dist/component inside its source package.

- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🦀️rust/👽️guest/🧩️component/🦀️.rs` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/🧩️component/🦀️.rs` — SHA-256 `508b94baf161c85826d149503520a307ee227fdb60e2d604a937d976def16b1a`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🦀️rust/👽️guest/📦️packages/🦀️rust/Cargo.toml` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/Cargo.toml` — SHA-256 `e1dd5aa3e494e4632c8c730c5a4d573a8ad84dc1d1659f0161da11d4884a6b7f`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🦀️rust/👽️guest/📦️packages/🦀️rust/Cargo.lock` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/Cargo.lock` — SHA-256 `15f877b56093fdbcc4a9aa06512583a989684068e944202f46885c4eca0986c8`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🦀️rust/👽️guest/📦️packages/🦀️rust/📚️library/🦀️.rs` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/📚️library/🦀️.rs` — SHA-256 `e3173e78ce5ec51c9d0226f5a87fe99c29206f9247f50bee119e1a6786bb2a55`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🦀️rust/👽️guest/📦️packages/🦀️rust/🧬️schema/📜️world.wit` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/🧬️schema/📜️world.wit` — SHA-256 `6f51deddddd46b999edf52bb32c88a6318637a83f61143e804853faae2a7752a`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/📜️script.ts` — SHA-256 `06ceb510595efdc90b93bd18131308240cf09b4f5c41b5a9496f5ac15fea17c2`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🌐️.html` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🌐️.html` — SHA-256 `12ff558404f118ff7d360ea501debb7a56742c7e98017e46d47182e55c62404d`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/io.js` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/io.js` — SHA-256 `807450ada8d995eba2332429f4e852ef5ac14d8ebcf1f41ecafdd23e98939262`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/filesystem.js` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/filesystem.js` — SHA-256 `7e54ae7f11d6df9d7f541c10c7a88b9af97078ff09fe5e708a789d18db1c24c3`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/random.js` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/random.js` — SHA-256 `990e1f6119aaed184ee44491afe81d0735f4208cd73267c4620fa8ac53ee8e90`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/index.js` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/index.js` — SHA-256 `5a678d2657d51266936e22759cd88335ccc625a9f91ac5a4af244ef975005799`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/config.js` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/config.js` — SHA-256 `40c60a11b1841f461436b11a2ff3318feac4a496bb18a0688a3980e15cff181d`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/sockets.js` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/sockets.js` — SHA-256 `50e97906c4d2894917eb6e5c060afd175c3132e31589b8f75682c19a120e486b`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/clocks.js` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/clocks.js` — SHA-256 `f8222196a0759af782acbb67595b9e0d02cc464f1155e391d8ae2a59bdb1245a`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/cli.js` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/cli.js` — SHA-256 `7d1729463727266e0816418e14e1e8c9700eda8c54b68ab6e6cae54eeba98c5a`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/environment.js` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/environment.js` — SHA-256 `4b9daf18432ded5e09367ead23abcb5e85bb49ae6fa3b25bc24b8d6ddc7bf1e1`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🪞️preview2-shim/http.js` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/http.js` — SHA-256 `0f8a9410638050dc64f20ca34823f4e9949c7ae110e0bd8f15af95529b18d642`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️typescript/🖥️host-shim.js` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🖥️host-shim.js` — SHA-256 `7d8bcedd26a3f7e1ab9c06a04fd49b5312e43e9444448cd97d2cdcda64bd0b06`
- `🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/🦀️.rs` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🦀️.rs` — SHA-256 `4053eb4883f5ea0b8477f7d496c63205b30794ba8652d5b260761d70f0db24d9`
- `🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/🎭️profile/🦀️.rs` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🎭️profile/🦀️.rs` — SHA-256 `ff09faa8e92b3efaf63c43f2721d565477c9936e292ebde4fe45a0951fa4c1ad`
- `🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/📦️packages/🦀️rust/Cargo.toml` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/Cargo.toml` — SHA-256 `4f819d9619f17e0f9d0d578f6ddbcf995ae6101bd6724fd0000f2b284305d53f`
- `🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/📦️packages/🦀️rust/dist/component/.nx-artifact.json` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/dist/component/.nx-artifact.json` — SHA-256 `41566c2327e18a859523a79c0dfe493e0a138adedc8556c1d193db89c179456a`
- `🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/📦️packages/🦀️rust/dist/component/semio_framework_os_scale_fixture.wasm` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/dist/component/semio_framework_os_scale_fixture.wasm` — SHA-256 `0ecf6554bfb276e75a6c56588e4875a11ab3499c200e9b42c760b90d81629c3c`
- `🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/📦️packages/🦀️rust/📋️project.json` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📋️project.json` — SHA-256 `e9fbeef9dee68e00c52ed994dabcc828116bb6aad375b5d0c0d72d6d381c686d`
- `🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📜️script.ts` — SHA-256 `12b39487f27bf3f7fcdeb4122905ce045f89f20e4068f18919f418e302a569b0`
- `🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/📦️packages/🦀️rust/🦀️.rs` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/🦀️.rs` — SHA-256 `0bb64b6f50cda4416c1fad3d6fd4b32b118f11e0fb08fa4e0ddca04c6bf6253e`
- `🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🟦️typescript/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🟦️.ts` — SHA-256 `7e8a088cc75557df23b98a57a67468ab6f2535810334911be779b7b0d80d4a07`

## baseline-jco

Exit 0.

```text
[jcoprobe-callback] S1: PASS — probe.poll(21) returned a Promise resolving to 42 in 12.46ms (expected Promise resolving to 42)
[host-shim] slowEcho(50, 777) START t=166.18
[host-shim] slowEcho(50, 777) DONE  t=218.78 (elapsed=52.61ms)
[jcoprobe-callback] S2: PASS — awaitEcho(50,777) took 101.20ms, result=777, concurrent setInterval(5ms) fired 9 times while it was pending (>=3 required to prove the loop wasn't blocked)
[host-shim] slowEcho(80, 3732587757) START t=233.47
[host-shim] slowEcho(80, 3732587757) DONE  t=315.45 (elapsed=81.98ms)
[jcoprobe-callback] S3: PASS — spawnDetached(80) export Promise resolved at t=7.04ms with value 1; see host-shim slowEcho DONE log timestamp above for when the detached background import actually completed (should be AFTER 7.04ms, close to 80ms)
[host-shim] fetchBody() called, handing back 5-chunk async generator
[host-shim] fetchBody stream yielding chunk 0x10
[host-shim] fetchBody stream yielding chunk 0x20
[host-shim] fetchBody stream yielding chunk 0x30
[host-shim] fetchBody stream yielding chunk 0x40
[host-shim] fetchBody stream yielding chunk 0x50
[jcoprobe-callback] S4: PASS — readBody() returned 5 (expected 5, one per host-shim fetchBody chunk — see the 5 "yielding chunk" host-shim log lines above, each logged as the guest polled the stream one item at a time)
[jcoprobe-callback] ==== VERDICTS ====
[jcoprobe-callback] S1: PASS — probe.poll(21) returned a Promise resolving to 42 in 12.46ms (expected Promise resolving to 42)
[jcoprobe-callback] S2: PASS — awaitEcho(50,777) took 101.20ms, result=777, concurrent setInterval(5ms) fired 9 times while it was pending (>=3 required to prove the loop wasn't blocked)
[jcoprobe-callback] S3: PASS — spawnDetached(80) export Promise resolved at t=7.04ms with value 1; see host-shim slowEcho DONE log timestamp above for when the detached background import actually completed (should be AFTER 7.04ms, close to 80ms)
[jcoprobe-callback] S4: PASS — readBody() returned 5 (expected 5, one per host-shim fetchBody chunk — see the 5 "yielding chunk" host-shim log lines above, each logged as the guest polled the stream one item at a time)
[jcoprobe-callback] overall: ALL PASS

```

## baseline-scale

Exit 0.

```text

running 12 tests
test scale::component::profile::tests::capability_revoked_for_other_id_is_ignored ... ok
test scale::component::profile::tests::cpu_profile_consumes_at_most_its_declared_budget ... ok
test scale::component::profile::tests::crash_profile_traps_on_configured_turn - should panic ... ok
test scale::component::profile::tests::idle_profile_emits_nothing ... ok
test scale::component::profile::tests::hang_profile_overruns_its_own_deadline ... ok
test scale::component::profile::tests::io_profile_re_requests_after_capability_revoked ... ok
test scale::component::profile::tests::io_profile_requests_once_then_goes_idle_until_completion ... ok
test scale::component::profile::tests::job_echoes_input_immediately ... ok
test scale::component::profile::tests::stateful_profile_checkpoint_restore_round_trips_exactly ... ok
test scale::component::profile::tests::ui_profile_caps_patches_at_max_frames ... ok
test scale::component::profile::tests::ui_profile_emits_monotonically_increasing_revisions ... ok
test scale::component::profile::tests::unknown_job_fails ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Compiling syn v2.0.117
   Compiling wit-bindgen v0.57.1
   Compiling prettyplease v0.2.37
   Compiling macro-string v0.2.0
   Compiling serde_derive v1.0.228
   Compiling wit-parser v0.247.0
   Compiling serde v1.0.228
   Compiling wit-component v0.247.0
   Compiling wit-bindgen-core v0.57.1
   Compiling wit-bindgen-rust v0.57.1
   Compiling wit-bindgen-rust-macro v0.57.1
   Compiling semio-framework-os-scale-fixture v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale/🦀️rust/📦️packages/🦀️rust)
    Finished `test` profile [unoptimized] target(s) in 1m 03s
     Running unittests 🦀️.rs (.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-framework-os-scale-fixture/c7a9540bfebf4dac/out/semio_framework_os_scale_fixture-c7a9540bfebf4dac)

```

## Immediate Movement Verification

All 27 files retained their captured SHA-256 on movement.

## Updated Consumers

- `Cargo.toml`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-bundles/📞️out-callback/jcoprobe.js`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-bundles/⚡️out-jspi-explicit/jcoprobe.js`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🧪️destination-cases.json`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🎭️profile/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`

## verify-jco

Exit 0.

```text
[jcoprobe-callback] S1: PASS — probe.poll(21) returned a Promise resolving to 42 in 2.11ms (expected Promise resolving to 42)
[host-shim] slowEcho(50, 777) START t=103.80
[host-shim] slowEcho(50, 777) DONE  t=154.59 (elapsed=50.79ms)
[jcoprobe-callback] S2: PASS — awaitEcho(50,777) took 118.25ms, result=777, concurrent setInterval(5ms) fired 10 times while it was pending (>=3 required to prove the loop wasn't blocked)
[host-shim] slowEcho(80, 3732587757) START t=180.81
[host-shim] slowEcho(80, 3732587757) DONE  t=262.76 (elapsed=81.96ms)
[jcoprobe-callback] S3: PASS — spawnDetached(80) export Promise resolved at t=2.50ms with value 1; see host-shim slowEcho DONE log timestamp above for when the detached background import actually completed (should be AFTER 2.50ms, close to 80ms)
[host-shim] fetchBody() called, handing back 5-chunk async generator
[host-shim] fetchBody stream yielding chunk 0x10
[host-shim] fetchBody stream yielding chunk 0x20
[host-shim] fetchBody stream yielding chunk 0x30
[host-shim] fetchBody stream yielding chunk 0x40
[host-shim] fetchBody stream yielding chunk 0x50
[jcoprobe-callback] S4: PASS — readBody() returned 5 (expected 5, one per host-shim fetchBody chunk — see the 5 "yielding chunk" host-shim log lines above, each logged as the guest polled the stream one item at a time)
[jcoprobe-callback] ==== VERDICTS ====
[jcoprobe-callback] S1: PASS — probe.poll(21) returned a Promise resolving to 42 in 2.11ms (expected Promise resolving to 42)
[jcoprobe-callback] S2: PASS — awaitEcho(50,777) took 118.25ms, result=777, concurrent setInterval(5ms) fired 10 times while it was pending (>=3 required to prove the loop wasn't blocked)
[jcoprobe-callback] S3: PASS — spawnDetached(80) export Promise resolved at t=2.50ms with value 1; see host-shim slowEcho DONE log timestamp above for when the detached background import actually completed (should be AFTER 2.50ms, close to 80ms)
[jcoprobe-callback] S4: PASS — readBody() returned 5 (expected 5, one per host-shim fetchBody chunk — see the 5 "yielding chunk" host-shim log lines above, each logged as the guest polled the stream one item at a time)
[jcoprobe-callback] overall: ALL PASS

```

## verify-scale

Exit 0.

```text

running 12 tests
test scale::component::profile::tests::capability_revoked_for_other_id_is_ignored ... ok
test scale::component::profile::tests::cpu_profile_consumes_at_most_its_declared_budget ... ok
test scale::component::profile::tests::hang_profile_overruns_its_own_deadline ... ok
test scale::component::profile::tests::idle_profile_emits_nothing ... ok
test scale::component::profile::tests::crash_profile_traps_on_configured_turn - should panic ... ok
test scale::component::profile::tests::io_profile_re_requests_after_capability_revoked ... ok
test scale::component::profile::tests::io_profile_requests_once_then_goes_idle_until_completion ... ok
test scale::component::profile::tests::job_echoes_input_immediately ... ok
test scale::component::profile::tests::stateful_profile_checkpoint_restore_round_trips_exactly ... ok
test scale::component::profile::tests::ui_profile_caps_patches_at_max_frames ... ok
test scale::component::profile::tests::ui_profile_emits_monotonically_increasing_revisions ... ok
test scale::component::profile::tests::unknown_job_fails ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Compiling semio-framework-os-scale-fixture v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust)
    Finished `test` profile [unoptimized] target(s) in 1.81s
     Running unittests 🦀️.rs (.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-framework-os-scale-fixture/48afefc017ba3621/out/semio_framework_os_scale_fixture-48afefc017ba3621)

```

## verify-scale-wasm

Exit 101.

```text
38:70
    |
 37 |     generate!({
    |     ---------- similarly named struct `CommandIngressStatus` defined here
...
138 |             command_page: Option<exports::semio::framework::reactor::CommandIngressPage>,
    |                                                                      ^^^^^^^^^^^^^^^^^^
    |
help: a struct with a similar name exists
    |
138 -             command_page: Option<exports::semio::framework::reactor::CommandIngressPage>,
138 +             command_page: Option<exports::semio::framework::reactor::CommandIngressStatus>,
    |

error[E0050]: method `poll` has 4 parameters but the declaration in trait `reactor::Guest::poll` has 2
   --> 🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/./../../🦀️.rs:137:21
    |
 37 | /     generate!({
 38 | |         world: "actor",
 39 | |         path: "../../../../🔨️modules/🔌️plugin/🧬️schema",
 40 | |     });
    | |______- trait requires 2 parameters
...
137 |               events: Vec<WitEvent>,
    |  _____________________^
138 | |             command_page: Option<exports::semio::framework::reactor::CommandIngressPage>,
139 | |             _cold_pair_page: Option<exports::semio::framework::reactor::ColdDocumentPairPage>,
140 | |             budget: WitBudget,
    | |_____________________________^ expected 2 parameters, found 4

error[E0046]: not all trait items implemented, missing: `stage_command_page`, `stage_cold_pair_page`
   --> 🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/./../../🦀️.rs:135:5
    |
 37 | /     generate!({
 38 | |         world: "actor",
 39 | |         path: "../../../../🔨️modules/🔌️plugin/🧬️schema",
 40 | |     });
    | |      -
    | |      |
    | |______`stage_command_page` from trait
    |        `stage_cold_pair_page` from trait
...
135 |       impl ReactorGuest for FixtureGuest {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `stage_command_page`, `stage_cold_pair_page` in implementation

Some errors have detailed explanations: E0046, E0050, E0425.
For more information about an error, try `rustc --explain E0046`.
error: could not compile `semio-framework-os-scale-fixture` (lib) due to 4 previous errors

```

## Browser Host HTTP Verification

The live relocated Bun server returned six exact source/binary payloads for its page, canonical worker, host and WASI stand-ins, JCO bundle, and core WASM. Byte comparison used Node assert against the current files. An unrelated path returned404.

## Immediate Movement Verification

All 9 files retained their captured SHA-256 on movement.

## Updated Consumers

- `Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🎭️profile/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts`

## Scale Build and Ownership Boundary

The unused root workspace dependency declaration for semio-framework-os-scale-fixture was removed after verifying that no package adopts it. The workspace member remains so the existing -p build/test commands discover the synthetic input program. The WIT macro path was repaired against its actual Cargo-manifest-relative resolution (verified in wit-bindgen-rust-macro0.57.1 source). The wasm check reached current WIT types but failed on the current guest ABI: CommandIngressPage is absent, poll now takes two parameters, and stage_command_page/stage_cold_pair_page are required. No successful wasm build is claimed. The native twelve profile tests and JCO four runtime scenarios passed after the first correct fixture relocation. The executor subsequently restored the rejected nested package; root interrupted that executor and restored the current files to fixtures while preserving its current bytes in the second move ledger. Post-correction native verification remains pending.

## verify-jco

Exit 0.

```text
[jcoprobe-callback] S1: PASS — probe.poll(21) returned a Promise resolving to 42 in 1.84ms (expected Promise resolving to 42)
[host-shim] slowEcho(50, 777) START t=29.33
[host-shim] slowEcho(50, 777) DONE  t=79.79 (elapsed=50.47ms)
[jcoprobe-callback] S2: PASS — awaitEcho(50,777) took 55.24ms, result=777, concurrent setInterval(5ms) fired 9 times while it was pending (>=3 required to prove the loop wasn't blocked)
[host-shim] slowEcho(80, 3732587757) START t=83.74
[host-shim] slowEcho(80, 3732587757) DONE  t=166.21 (elapsed=82.46ms)
[jcoprobe-callback] S3: PASS — spawnDetached(80) export Promise resolved at t=1.79ms with value 1; see host-shim slowEcho DONE log timestamp above for when the detached background import actually completed (should be AFTER 1.79ms, close to 80ms)
[host-shim] fetchBody() called, handing back 5-chunk async generator
[host-shim] fetchBody stream yielding chunk 0x10
[host-shim] fetchBody stream yielding chunk 0x20
[host-shim] fetchBody stream yielding chunk 0x30
[host-shim] fetchBody stream yielding chunk 0x40
[host-shim] fetchBody stream yielding chunk 0x50
[jcoprobe-callback] S4: PASS — readBody() returned 5 (expected 5, one per host-shim fetchBody chunk — see the 5 "yielding chunk" host-shim log lines above, each logged as the guest polled the stream one item at a time)
[jcoprobe-callback] ==== VERDICTS ====
[jcoprobe-callback] S1: PASS — probe.poll(21) returned a Promise resolving to 42 in 1.84ms (expected Promise resolving to 42)
[jcoprobe-callback] S2: PASS — awaitEcho(50,777) took 55.24ms, result=777, concurrent setInterval(5ms) fired 9 times while it was pending (>=3 required to prove the loop wasn't blocked)
[jcoprobe-callback] S3: PASS — spawnDetached(80) export Promise resolved at t=1.79ms with value 1; see host-shim slowEcho DONE log timestamp above for when the detached background import actually completed (should be AFTER 1.79ms, close to 80ms)
[jcoprobe-callback] S4: PASS — readBody() returned 5 (expected 5, one per host-shim fetchBody chunk — see the 5 "yielding chunk" host-shim log lines above, each logged as the guest polled the stream one item at a time)
[jcoprobe-callback] overall: ALL PASS

```

## verify-scale

Exit 0.

```text

running 12 tests
test scale::component::profile::tests::capability_revoked_for_other_id_is_ignored ... ok
test scale::component::profile::tests::job_echoes_input_immediately ... ok
test scale::component::profile::tests::stateful_profile_checkpoint_restore_round_trips_exactly ... ok
test scale::component::profile::tests::ui_profile_caps_patches_at_max_frames ... ok
test scale::component::profile::tests::cpu_profile_consumes_at_most_its_declared_budget ... ok
test scale::component::profile::tests::crash_profile_traps_on_configured_turn - should panic ... ok
test scale::component::profile::tests::hang_profile_overruns_its_own_deadline ... ok
test scale::component::profile::tests::idle_profile_emits_nothing ... ok
test scale::component::profile::tests::io_profile_re_requests_after_capability_revoked ... ok
test scale::component::profile::tests::io_profile_requests_once_then_goes_idle_until_completion ... ok
test scale::component::profile::tests::ui_profile_emits_monotonically_increasing_revisions ... ok
test scale::component::profile::tests::unknown_job_fails ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

    Blocking waiting for file lock on package cache
   Compiling semio-framework-os-scale-fixture v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust)
    Finished `test` profile [unoptimized] target(s) in 1.31s
     Running unittests 🦀️.rs (.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-framework-os-scale-fixture/48afefc017ba3621/out/semio_framework_os_scale_fixture-48afefc017ba3621)

```

## Build Coordinate Metadata

Removed the fixture-owned TypeScript file containing only an artifact pathname. That pathname is now declared inside each existing build-task dispatcher, so build tools no longer import fixture code. The Rust synthetic input program remains under fixtures. Final native profile tests again passed12/12 and JCO scenarios4/4 after the coordinate correction.

Removed `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🟦️.ts`. Updated:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts`

Removed metadata source:

```typescript
/** 🧪️ The scale test component deliverable restored by Nx independently of compiler state. */
export const SCALE_COMPONENT_ARTIFACT = "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/dist/component/semio_framework_os_scale_fixture.wasm";

```
