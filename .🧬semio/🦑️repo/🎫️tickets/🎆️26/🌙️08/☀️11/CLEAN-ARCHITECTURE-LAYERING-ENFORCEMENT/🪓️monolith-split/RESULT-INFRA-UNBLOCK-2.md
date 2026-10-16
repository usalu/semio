LAUNCHER REACHES CARGO: yes (2026-10-16 05:04)

# Infra Unblock 2 Result (2026-10-16)

Status: the first blocker (`Complete observed Cargo compiler profile required`) is fixed and tested. The launcher now gets past graph capture and stops at a second, different blocker: two orphan lease claims in the old-format ticket layout wedge the capacity-lease sweep. That one is a lease-custody policy decision, so I stopped there. No gate was bypassed. No cache material was deleted.

Logs: `🪓️monolith-split/🗑️generated/iu2-probe-1.log`, `iu2-probe-2.log`, `iu2-probe-3.log`, `iu2-claims.ts` (read-only claim inventory).

## 1. Cause of the profile refusal

The validator is not at fault. `validateJsonSchemaSubset` supports `oneOf`, `const`, `enum` with null, type arrays and `additionalProperties:false` correctly. The real Cargo profile contains a field the schema did not admit.

Observed through the sanctioned launcher (a temporary `[DEBUG]` log inside `readCargoEffectiveProfileV1`, now removed):

```
{"name":"prototype","opt_level":"0","lto":"false","codegen_backend":null,"codegen_units":256,"debuginfo":1,"split_debuginfo":"unpacked","debug_assertions":true,"overflow_checks":true,"rpath":false,"incremental":false,"panic":"unwind","strip":{"resolved":"None"},"trim_paths":["object"]}
errors ["/trim_paths is not allowed"]
```

- Reason: both `Cargo.toml` and `🧰️framework/Cargo.toml` start with `cargo-features = ["trim-paths"]`. With that feature enabled, nightly-2026-07-20 `cargo --unit-graph` emits an extra `trim_paths` array on every profile. The schema had `additionalProperties:false` and no such property.
- The peer's 18-observation corpus has no `trim_paths` (producer manifests without the feature), so `trim_paths` must stay optional. Making it required would invalidate the corpus.
- I did not observe the shape for `trim-paths = "none"` or `"all"`. Only `["object"]` was observed. The schema admits an array of unique values from `diagnostics|macro|object` (max 3) and refuses anything else, so an unseen shape fails loudly instead of being guessed.

## 2. Fix (test first)

- RED: new test appended to `📐️profile/🧪️tests/🟦️.ts` ("neutral real nightly-2026-07-20 profile with enabled trim-paths ..."). It failed with `must NOT have additional properties` / `trim_paths`.
- Schema: added optional `trim_paths` (`type:array, uniqueItems, maxItems:3, items.enum:[diagnostics,macro,object]`) to the profile schema. All required fields stay required, `additionalProperties:false` stays.
- The existing mirror test `graphSchema...compilerProfile toEqual(schema)` and `unitSchema.properties.profileData toEqual(schema)` forces identical copies, so the same 12-line addition went into all four copies:
  - `🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🧬️schema/🔣️.json`
  - `🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🧬️schema/🔣️.json` (graph `compilerProfile`)
  - `🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧬️schema/🔣️.json` (`profileData`)
  - `🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧭️context/🧬️schema/🔣️.json`
- `📐️profile/🟦️.ts`: added `readonly trim_paths?:readonly(...)[]` to `CargoEffectiveProfileV1`, and `projectCargoRlibProfileV1` now refuses a non-empty `trim_paths`. The direct rlib adapter emits no `--remap-path-prefix` / `-Zremap-path-scope` flags, so an unobserved remap default must stay unavailable. `materializeCargoRustUnitsV1` catches that refusal and marks such units `effective-profile` unsupported (graceful, not a crash).
- Test file added: 1 line (the new test). The git index already holds these changes (a peer auto-commit swept them in), so `git diff` and `git diff --stat` are empty for these paths. The edit size is +12 lines in each of the 4 schemas, +1 test line, 2 small edits in `📐️profile/🟦️.ts`.

## 3. Tests (real results)

`cd …/🕸️units && bun test ./📐️profile/🧪️tests/🟦️.ts`: 6 pass / 2 fail.
- Pass: the new `trim-paths` test (Ajv oracle, `readCargoEffectiveProfileV1`, projection refuses, `trim_paths:[]` still projects, unknown/duplicate/scalar/null/misspelled-sibling refused, every required field removal refused), both pre-existing profile tests, the schema mirror test, and the others.
- Fail (same before my change, environment only): `neutral real issued control materializes full profile DATA ...` and one `(unnamed)`, both `Original parent Script process invocation required`. These are `registerBuildTestLawV1` tests that need the launcher's process invocation environment (`SEMIO_TEST_ARTIFACT_DIR`, mode).
- `bun test ./🧪️tests/🟦️.ts` in the same folder: 6 pass / 5 fail, the 5 failing are the same `Original parent Script process invocation` environment tests (actual Cargo plan tests). I did not run them under the launcher.

## 4. Reproduce, after the fix

Command as given. `iu2-probe-2.log` and `iu2-probe-3.log`: the run passes the headroom gate and the dependency fetch, passes graph capture and profile validation (no `Complete observed Cargo compiler profile required`), then fails before cargo compiles anything:

```
error: Original queue claim path identity required
      at readResourceQueueClaimIdentityV1 (🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts:241)
error: script "repo" exited with code 1
```

## 5. Next blocker: orphan old-format queue claims in the capacity-lease directory

- Where: the cargo launcher's `capacityLeaseDirectory` is `.cache/framework/leases` (`…/🦀️cargo/🚪️launcher/🔣️policy.json`). Every queued acquire first runs `retireResourceQueueClaimsV1` (`🔒️leases/🟦️.ts:266`), which reads each file in `claims/`.
- Observed through a temporary `[DEBUG]` log in `readResourceQueueClaimIdentityV1` (removed): the claim resource
  `queued-original:be78523a…87e9:01a140b2944d-0000037808-BP1nUB_DG6SU-dHZMvLlmg` does not match the canonical ticket pattern `^[0-9a-z]{10}-[0-9a-z]{7}-…` (`identity:false`).
  - The ticket uses a superseded layout: 12 hex arrival digits plus 10 decimal pid digits. The current protocol is 10 base-36 digits plus 7 base-36 digits (`🧬️schema/📜️ticket.json`).
- Inventory (read-only, `.cache/framework/leases/claims/`): exactly 2 files, both written 2026-10-15 19:53 by pid 37808 (not running):
  - `3771ec42689d57b6686569aaf7e45560b9f561526eae3922ab2af80356867371.sqlite` (resource under queue `be78523a…`)
  - `7310e8ea7d303b49d6426b7bf08ae5f8cc21566c16428f53483f6eed86ba4350.sqlite` (resource under queue `d505e367…`, ticket `01a140b2946e-0000037808-…`)
  - Their `*.queue` directories exist and are empty; nothing holds a descriptor on `claims/`.
- The integrity check is intended (a claim whose identity does not match is not silently skipped, unlike a foreign `application_id`). The data is leftover from an earlier writer revision. Two sanctioned ways out, both the peer fleet's decision, so I did not take either:
  1. Data: authorize removing exactly those 2 orphan claim files and their 2 empty `.queue` directories (dead pid 37808, superseded format, nothing queued).
  2. Code: change `retireResourceQueueClaimsV1` to treat a non-canonical ticket as foreign/unretirable and skip it. That loosens lease custody and would need the lease owner's sign-off. It is also a legacy-format allowance, against the greenfield rule.
- Recommendation: option 1. After it the sweep examines the remaining canonical claims normally. I expect no further format drift, but the directory holds 2423 entries, and I have not verified anything beyond `claims/`.

## Files changed

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧭️context/🧬️schema/🔣️.json`

Both temporary `[DEBUG]` logs (profile, lease claim) are removed; `🔒️leases/🟦️.ts` is unchanged by me.

## 6. Outcome after the coordinator removed the orphan claims (rerun, `iu2-probe-4.log`)

The coordinator moved the 2 orphan claim files to `🗑️generated/coord-orphan-claims-backup/` and removed their 2 empty `.queue` dirs; `claims/` was empty. I re-ran the reproduce command unchanged. No further launcher defect and no further stale data from a dead writer appeared. The launcher reached cargo, compiled the dependency closure, and checked both crates with exit 0.

- `semio-framework-canvas-assets`: Checking ... no errors, no warnings reported.
- `semio-framework-ui-wgpu-device`: 2 warnings, 0 errors:
  - `🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎮️execution/🔌️device/📦️packages/🦀️rust/../../🗺️conversion/🦀️.rs:20:15: warning: function `map_mode` is never used`
  - `…/🗺️conversion/🦀️.rs:21:15: warning: function `poll_type` is never used`
- Warnings in dependencies (not fixed): `semio-framework-value` (3), `semio-framework-pack-json` (4), `semio-framework-dsl` (1).

Tail of the log:

```
    Checking semio-framework-canvas-assets v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️canvas/🔌️runtime-assets/📦️packages/🦀️rust)
    Checking semio-framework-ui-wgpu-device v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎮️execution/🔌️device/📦️packages/🦀️rust)
🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎮️execution/🔌️device/📦️packages/🦀️rust/../../🗺️conversion/🦀️.rs:20:15: warning: function `map_mode` is never used
🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎮️execution/🔌️device/📦️packages/🦀️rust/../../🗺️conversion/🦀️.rs:21:15: warning: function `poll_type` is never used
warning: `semio-framework-ui-wgpu-device` (lib) generated 2 warnings
    Finished `prototype` profile [unoptimized + debuginfo] target(s) in 39.33s
exit=0

```
