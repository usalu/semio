# O2: linked units offload (G4, G5, G8, G11)

## G11 reproduction
- `offload.jsonl`: 203 worker-policy refusals. 185 `CARGO_PKG_DESCRIPTION` (last 2026-11-14 13:41) were already fixed in the validator (`MANIFEST_TEXT`); the 18 `--check-cfg` (last 11-21 18:02, `objc2_metal`) were NOT: Cargo writes `cfg(feature, values("a", ...))` with every feature, 2281 chars for objc2-metal 0.3.2 (web-sys: 37957), the validator capped it at 1024.
- Fix: `--check-cfg` allowed up to `limits.maximumCheckConfigurationBytes` (1 MiB, schema), feature names with any letter/digit; `--cfg` keeps 1024. Tests use the real objc2-metal feature list (105 names), a 3000-feature list, real CARGO_PKG_* texts from the registry.

## Policy (schema `policy.link`)
- Crate types: native `cdylib`/`dylib`/`staticlib` offload; wasm targets of the policy only `cdylib`; other wasm* targets (emscripten) refused.
- `-l`: static any plain name; dylib/default kind only names in `systemLibraries`, `framework=` only `frameworks` (worker resolves against its own SDK); no rename except static, no paths.
- `-L native|all|framework=`: only shipped virtual dirs `.semio-offload/n<k>` (libraries of the dir: .a .dylib .so .lib .tbd, framework files; never objects/logs). SDK/system dirs dropped from the command; other system-prefixed dirs (`/opt/homebrew/lib`) decline `not-portable`; missing dirs dropped.
- Link args: native `-Wl,-undefined,dynamic_lookup` (Node addon), wasm `-Wl,--export=<name>` added to `--max-memory`/`-zstack-size` (the only link args in `.cargo/config.toml` and live rustc lines). Linux `-C linker-features`, `link-self-contained`, `-Z unstable-options` accepted. Requester declines unknown `-C link-arg` up front (no 3 s worker-policy memo storm).
- Install name: `cdylib`/`dylib` on macOS now get `-install_name @rpath/lib<name><extra>.dylib` (LC_ID_DYLIB held the absolute output path: two local builds differed).

## Biggest finding
`rustcBypassV1` bypassed every linking unit with `-l`/`-L native` (reason `native-link`): never cached, never offloaded, regardless of the validator. Now script.ts keys such units by the digests of the native libraries (`nativeKeyInputsV1`, `native:<kind>:<path>` pseudo-externs) and no longer bypasses when the inputs are keyable; unkeyable ones keep the bypass. The unit key module itself is untouched.

## Tests (repo test-file)
actions 10/0, offload 39/0, offload cluster 12/0, native-link 10/0 (vectors JSON incl. real command lines: native cdylib with libs, wasm cdylib with stack/max-memory/export, build-script bin; worker/requester bytes identical at two checkout paths and two input roots), native-link cluster 1/0 (two-worker local cluster: native cdylib with a static lib offloaded via the wrapper, `offload-run`, bytes equal a local rustc build), wasm-cdylib 8/0, determinism 6/0, action 3/0, failure 2/0, gate 1/0, remap 7/0. Unit-cache key/script tests: see bottom.

## Open items
- Live fleet: the 11 workers run old validator code until restarted; no live offload of a linked unit was run (only the local two-worker cluster). Needs worker restart.
- SDK identity is in the unit key (`SDKROOT`) and linker identity (darwin major), not a placement property: a worker with another SDK could link differently; add `semio-sdk` worker property if SDKs differ.
- `dylib` path-dependence beyond install name and oso_prefix not exercised; staticlib not exercised end-to-end.

## Unit-cache tests after the script.ts change
key 8/0, portable 35/0, remap 7/0. `unit-cache/🧪️tests/🟦️.ts` 22 pass / 6 fail: the vectors failure is the K1 content-derived `-C metadata` change (expected `metadata=abc` args dropped, not native-link); the other five are cargo/jobserver/sealing tests (`module set exceeds 2000 files`, build-script run), not touching native units. Not verified against a clean baseline.
- Process note: I ran `bun test ./🟦️.ts` (with `./`, one file, from its own folder) three times before the 21:55 addendum; none after.
