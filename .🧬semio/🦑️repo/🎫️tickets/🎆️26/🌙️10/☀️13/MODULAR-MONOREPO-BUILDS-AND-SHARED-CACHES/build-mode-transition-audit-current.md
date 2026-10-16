# Current Build Mode Transition Audit

Read-only Source inspection on 2026-10-15. Only this report was authored. No tests, builds, compiler/store/image/Original acquisition or modifying git operations ran. Parent owns the existing ticket lifecycle. Retained phase-one report establishes actual prototype graph oracle success: eight cases, 40 assertions, with named-package inheritance and environment-over-config behavior. It establishes neither wasm nor runtime/ABI qualification.

## Concrete Owner Closure

Paths below are repository relative. `P` = `🧰️framework/🔨️modules/🏃️process`; `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`; `O` = `🧰️framework/🛍️products/💻️os/🔨️modules`.

| Owner | Required coherent transition |
| --- | --- |
| `P/🏗️build/🎚️mode/🟦️.ts:20` and nearby neutral corpus | `wasmProfile` currently returns old names. Bind the canonical Cargo name to mode and add closed actual target/derived role projection. Existing new `🦀️profile` remains unimported policy authoring. |
| `P/📦️artifacts/🏗️native-build/🟦️.ts:174–187` | Generic `--config` is admitted; profile admission explicitly permits both native/wasm names and release+host-tool. Replace this with exact projected arguments, refusing arbitrary profile config and conflicting environment. Preserve `--` forwarding boundary. |
| `P/📦️artifacts/🕸️wasm-build/🟦️.ts:87–130` | Profile drives wasm-pack flags, Cargo args, lease args and artifact path. Free `prototypeOptimizedCrates` adds per-package opt3 after owned config. Change all together; wasm-pack release/custom metadata semantics must be requalified when old custom release becomes built-in release. |
| `O/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts:14,22,55` | Live descriptor emitter selects host-tool independently. Derive descriptor role from this owner and parent mode; preserve 16-codegen-unit measured release role constraint without a third name. Artifact paths also use component profile directly. |
| `L/⚡️caching/🦀️cargo/🟦️.ts:69–81`, `P/📦️artifacts/🏗️native-build/🧩️component-pass` | Component plans, schema, jobs and lanes rewrite native/wasm strings; environment is inferred from profile strings. Replace with admitted mode+role, keeping exact dependency/staging semantics. |
| `O/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs}` | Profile is public receipt vocabulary and shipping policy. Handcraft release/prototype schema, implementations and corpus together; preserve release-only shipping, 256 MiB shipped and 512 MiB local bounds. |
| `O/🔌️plugin/{🏗️build/📦️materialization,🏗️build/🧩️driver-stages,🖨️describe/🧩️component-stage,🌐️browser-bundle/🏗️materialization/🚀️commands,📇️registry/🛂️descriptor-verification}` | CLI `--profile`, receipt validation, deliverable lookup, browser vendor/module paths and shipped catalog verification use old vocabulary. Existing output directories are already mode-named in places; do not infer a safe transition by global replacement alone. |
| `O/📇️directory/🧬️schema/🔣️.json:3114`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️.json`, `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🧬️schema/🔣️.json` | Downstream schema admission consumes shipped receipt profile. Include all implementations/fixtures and renderer load/catalog tests; renaming only builder output breaks trust admission. |
| `L/{🟦️.ts,🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts,🏗️build/🕸️graph/📥️inference/🟨️.mjs}`, root `📜️script.ts` | Discovery/driver/task routes also mention wasm-release. Distinguish task names from Cargo profile names explicitly; update public selectors coherently with both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`. |
| `L/🏗️build/🎚️mode/🟨️.mjs:13–17` | Nx merges option env, config env, then forces selected `SEMIO_BUILD_MODE`; fixed-name routes choose defaults. Cache input includes mode only when target cache is enabled. Projection policy bytes and actual backend facts need compiler identity beyond this two-value selector. |
| `O/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts` | Already canonical Cargo names; integrate target/postlink projection rather than applying native release settings to wasm. Error text still mentions wasm-release task. |

Additional authored caller closure: `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, hub norm/stdio composition scripts, play site script/pages/project, OS scale Rust script, dev schema, stdio compiler capture schema. Tests include native/component-pass schemas and generated fixture manifests, mode/native fixtures, wasm builder tests, receipt TS/Rust tests, renderer catalog/load fixtures, registry verification, directory execution-target corpus. Old names remain useful as explicit rejection inputs in the new neutral corpus; a blanket zero-occurrences law would wrongly remove those negatives.

## Configuration And Identity Risks

`L/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🟦️.ts` currently captures actual `cargo config get` with command-line config selectors, then explicitly refuses observation sources rooted at `profile` or `env` (`Immutable Cargo profile or config-environment flag view unavailable`). It pins encoded rustc/rustdoc flags and hashes a JSON view, but execution still consumes original Cargo profile inputs. This owner must gain actual consumed-profile authority before using a projected config as proof. A recorded digest alone is insufficient; no broadening/removal of the current refusal without new custody proof.

The native target recipe at `P/📦️artifacts/🏗️native-build/🟦️.ts:220–224` hashes original manifest/config/toolchain ancestry and Cargo home plus args, selected environment and compiler/toolchain identities. `🎯️recipe/🟦️.ts:15` selects declared environment (its tests explicitly include `CARGO_PROFILE_PROTOTYPE_DEBUG`). This is valuable invalidation evidence, but post-hoc recheck does not eliminate a change-and-restore consumed-manifest/config race. Profile environment values that Cargo applies above config must be refused or exactly admitted into immutable projection; do not silently strip them after key generation. Both `--profile X` and `--profile=X`, repeated `--config`, target env, cwd ancestry and direct caller overrides need coherent admission.

The admitted actual graph proves config-over-manifest and environment-over-config for the fixture opt-level, and named dev package inheritance into prototype. It does not prove full merged profile fields, wildcard/dual-role precedence, explicit cross-target host segregation or panic/assertion contracts. Removing dev declarations without representing measured hot constraints changes effective prototype behavior. Adding a target-wide rustflag can contaminate host units; package overrides cannot express all profile fields, including LTO. Use a closed invocation-owned projection and compare actual units later.

One canonical `release` path now serves distinct target/role settings, so mode/profile alone cannot be a cache or lease recipe. Include effective immutable profile fields, target/role, admitted environment/config bytes, toolchain and post-link policy in applicable identities, while leaving irrelevant role spelling out. Actual observed unit owner `L/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️graph/🟦️.ts:21` already carries observed profileDigest/compilation evidence: extend that genuine observation route, do not fabricate a policy digest as equivalent Cargo evidence. Component receipt/provenance/source epoch and wasm transpile cache recipes must invalidate honestly when compiler semantics change. No old receipt compatibility fallback.

Keep Nextest assertion `--profile` distinct from Cargo `--cargo-profile`. Preserve `.cargo/config.toml` target stacks/memory, panic/overflow/assertions and wasm postlink limits pending real qualification.

## Current Peer Hot Package Declarations

This snapshot supersedes the earlier 7/19-row references. These are declaration facts, not effective Cargo membership proof. All listed wasm prototype rows are opt2 at inspection; root and Framework are equal, hub is a smaller owner cut. Peers can continue changing files: re-read before authoring. The pure schema projection must capture the exact authored package-constraint rows from the real consumed Source under closed owner/evidence admission, then bind those bytes and rows into immutable policy/cache identity. A static implementation list is unsafe even if it matches today: peer Kernel/Pack descendant cuts have already expanded during this ticket. This is finite schema-owned data admission, not restoration of the arbitrary per-call tuning knob. Resolve actual workspace membership before treating every declared row as an active unit constraint.

### `Cargo.toml` — 34 Rows

- `semio-framework-os-flow`
- `semio-framework-os-flow-geometry`
- `semio-framework-os-flow-mesh`
- `semio-framework-os-flow-drawing`
- `semio-framework-os-flow-operators`
- `semio-framework-os-flow-vcs`
- `semio-framework-os-flow-extensions`
- `semio-framework-os-flow-host-source`
- `semio-framework-os-flow-host-core`
- `semio-framework-os-flow-host-read`
- `semio-framework-os-flow-host-session`
- `semio-framework-os-flow-host`
- `semio-framework-os-flow-wasm-domain`
- `semio-framework-os-flow-wasm-actions-a`
- `semio-framework-os-flow-wasm-actions-b`
- `semio-framework-os-flow-wasm`
- `semio-framework-os-kernel-neural-engine`
- `semio-framework-os-kernel-neural-engine-model`
- `semio-framework-os-kernel-neural-engine-registry`
- `semio-framework-os-kernel-neural-engine-budgeted`
- `semio-framework-replication`
- `semio-framework-pack`
- `semio-framework-pack-format`
- `semio-framework-pack-format-retained`
- `semio-framework-pack-format-catalog`
- `semio-framework-pack-record-schema`
- `semio-framework-pack-record-retained`
- `semio-framework-pack-record-encode`
- `semio-framework-pack-record-preflight`
- `semio-framework-pack-record`
- `semio-framework-pack-intrinsic`
- `semio-framework-pack-async`
- `semio-framework-pack-http`
- `semio-framework-pack-io`

### `🧰️framework/Cargo.toml` — 34 Rows

- `semio-framework-os-flow`
- `semio-framework-os-flow-geometry`
- `semio-framework-os-flow-mesh`
- `semio-framework-os-flow-drawing`
- `semio-framework-os-flow-operators`
- `semio-framework-os-flow-vcs`
- `semio-framework-os-flow-extensions`
- `semio-framework-os-flow-host-source`
- `semio-framework-os-flow-host-core`
- `semio-framework-os-flow-host-read`
- `semio-framework-os-flow-host-session`
- `semio-framework-os-flow-host`
- `semio-framework-os-flow-wasm-domain`
- `semio-framework-os-flow-wasm-actions-a`
- `semio-framework-os-flow-wasm-actions-b`
- `semio-framework-os-flow-wasm`
- `semio-framework-os-kernel-neural-engine`
- `semio-framework-os-kernel-neural-engine-model`
- `semio-framework-os-kernel-neural-engine-registry`
- `semio-framework-os-kernel-neural-engine-budgeted`
- `semio-framework-replication`
- `semio-framework-pack`
- `semio-framework-pack-format`
- `semio-framework-pack-format-retained`
- `semio-framework-pack-format-catalog`
- `semio-framework-pack-record-schema`
- `semio-framework-pack-record-retained`
- `semio-framework-pack-record-encode`
- `semio-framework-pack-record-preflight`
- `semio-framework-pack-record`
- `semio-framework-pack-intrinsic`
- `semio-framework-pack-async`
- `semio-framework-pack-http`
- `semio-framework-pack-io`

### `🌎️hub/Cargo.toml` — 22 Rows

- `semio-framework-os-flow`
- `semio-framework-os-flow-geometry`
- `semio-framework-os-flow-mesh`
- `semio-framework-os-flow-drawing`
- `semio-framework-os-flow-operators`
- `semio-framework-os-flow-vcs`
- `semio-framework-os-flow-extensions`
- `semio-framework-os-flow-host-source`
- `semio-framework-os-flow-host-core`
- `semio-framework-os-flow-host-read`
- `semio-framework-os-flow-host-session`
- `semio-framework-os-flow-host`
- `semio-framework-os-flow-wasm-domain`
- `semio-framework-os-flow-wasm-actions-a`
- `semio-framework-os-flow-wasm-actions-b`
- `semio-framework-os-flow-wasm`
- `semio-framework-os-kernel-neural-engine`
- `semio-framework-os-kernel-neural-engine-model`
- `semio-framework-os-kernel-neural-engine-registry`
- `semio-framework-os-kernel-neural-engine-budgeted`
- `semio-framework-replication`
- `semio-framework-pack`

The separate browser constraint owner `🧰️framework/🔨️modules/🖱️ui/🖌️render/🏗️build/🕸️browser/🟦️.ts` declares twelve opt3 caller crates: `ttf-parser`, `rustybuzz`, `usvg`, `fontdb`, `roxmltree`, `svgtypes`, `simplecss`, `strict-num`, `tiny-skia-path`, `kurbo`, `peniko`, `vello_encoding`. Both editor Rust script and NodeGraph Rust script now pass this list. Preserve both consumers when replacing the public tuning knob with a closed browser role policy. Root dev named opt3 also covers Wasmtime/Cranelift dependencies, wasmparser/pulley/gimli/object/wit-parser and repository plugin-interpreter/hash; these remain separate native responsiveness constraints from the wasm opt2 rows.

## Minimal Coherent Ordering

1. While SDK physical revalidation proceeds, author only schema/corpus/policy and exact consumer inventory independently; extend existing neutral laws for target/derived role and environment/config conflicts. No SDK publication/store/image acquisition is needed for this Source work. Do not edit any frozen SDK captured closure during its physical revalidation.
2. Implement immutable consumed-profile custody in the actual configuration receiver and derive exact compiler plans/recipe identity. Keep separate host and target settings and preserve admission caps. This blocks production removal of old profiles.
3. At the coordinated Source boundary, change canonical profile declarations, both generic builders, descriptor owner, component plans/receipts/downstream schema admission and all browser caller constraints together. Handcraft every standalone workspace declaration found by authored manifest census, not just root/Framework/hub. Update fixtures and editor catalogs; no third-profile alias or migration.
4. After SDK physical work settles, obtain a new explicit finite physical operation for both-mode actual profile field/host-target/environment/cwd qualification and runtime/ABI/product bounds. Retain the existing successful native graph result as a limited oracle. Only then coordinate any necessary new SDK Source freeze/publication and cache invalidation qualification.

Blocking semantic risks are immutable consumption, full inherited hot-constraint preservation, host/target isolation, wasm-pack built-in-versus-custom release transformation, public receipt/trust admission closure, and honest backend-sensitive cache identity. Pure Source implementation can progress in parallel; physical qualification and mutable captured SDK closure changes must wait for the parent’s coordination.
