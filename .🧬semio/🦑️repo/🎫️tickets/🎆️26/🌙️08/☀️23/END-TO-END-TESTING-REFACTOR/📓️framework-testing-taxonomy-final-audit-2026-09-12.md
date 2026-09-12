# Framework Testing Taxonomy Final Audit — 2026-09-12

## Scope and method

Read-only final audit of `🧰️framework`, after the guard census and the synthetic-input relocation. I read the guard report and its retained physical census, compared each framework record with the live filesystem, inspected the cited Rust and TypeScript consumers, and searched current Rust test declarations and test-source inclusion consumers. No source was changed and no build was run by this audit.

The retained physical census is a concurrent snapshot, not a live verdict. Its framework records were reclassified below. The relocation report supplies the runtime evidence for its own move: JCO S1–S4 and the scale native twelve-test suite passed after relocation.

## Current category state

There are no live framework directories named `🪨️tests`, `🪞️fixtures`, `🧪️fixtures`, `🔮️oracle`, `testkit`, `test-support`, `test-helper`, or `test-harness`. In particular, the six framework legacy-fixture/oracle findings in the retained census were all **empty**, and were removed with the empty-directory pass:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🪞️fixtures` and its `🔄️lifecycle`, `🔮️oracle`, and `🫴️owners` children
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️fixtures/🧪️generator-preview`
- the now-empty `🧪️fixtures` parent above

No populated framework legacy fixture directory remains. The one name match still present,
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️test-support-scratch-directory`, is a single populated **test case**, not a support category; it must remain a direct test leaf.

The synthetic JCO guest, JCO browser host/shims, JCO browser bundles, and scale actor are now in the correct owner-level fixture roots:

- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale`

The remaining OS assertions are direct test leaves, including `🧪️tests/🧩️jco-callback/🟨️.mjs`, `🧪️tests/🧩️jcoprobe-callback/🟦️.ts`, and `🧪️tests/⚖️scale-profile/🦀️.rs`. The former nested `🧪️tests/⚖️scale/...` and `🧪️tests/🧩️jcoprobe-callback/...` implementation-depth/data records are therefore gone, rather than actionable live defects.

## Genuine source defects

### Six inline Rust test bodies and one inline test module

These are the only live `#[test]` declarations outside a `🧪️tests` collection:

| Owner and current source | Contents | Narrow correction |
| --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:1632-1779` | `wire_effect_laws` module, its two tests, and their wire-fixture builder | Move the complete test-only module to a direct turn test leaf and replace it with a `#[cfg(test)] #[path = "🧪️tests/<case>/🦀️.rs"] mod …;` declaration in `turn/🦀️.rs`. The private `decode_wire_effect` access remains available through `super`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19478` and `:19495` | app-module interaction-selection and leftover-overlay laws | Put each law in an external direct test implementation wired from the owning `app` module, so it can retain private-module access without an inline test body. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30573` | diagnostics-off law in `plugin_runtime` | Externalize as a `plugin_runtime`-owned direct test file with a local `#[path]` module declaration. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:33704` | self-waking-ready law in `plugin_runtime` | Externalize beside the preceding runtime law, using the same local module wiring. |

`#[cfg(test)]` helper declarations alone were not counted as test bodies. They need their own colocation cleanup below.

### Fixture embedded inside a test case

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🧫️fixtures/🔣️.json` is populated recursive-composition input data. It is a true fixture-in-case violation, not a harmless directory name. Move it to the plug-in semantic owner, for example `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧩️composition/🔣️.json`, then update both current consumers:

- `🧪️tests/🧩️composition/🟦️.ts:5`
- `🧪️tests/🧩️composition/🦀️.rs:322`

### Test-only support code left in production modules

These functions are all explicitly `#[cfg(test)]`, so they do **not** create a production runtime dependency on fixture data. They are nevertheless case-specific helper implementations that parse fixtures from production files, contrary to the required test-support colocation:

| Current helper and fixture | Current consumer | Narrow correction |
| --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:709` → `🧫️fixtures/🚪️raw-allocation-close.json` | `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:1946` | Move the helper body to the consuming direct case, or an external test module wired inside the retained-command owner. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12620` → `🧫️fixtures/🔬️tool-factory-proof.json` | `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:1931` | Move it with its test-only generic fixtures; retain the JSON under owner `🧫️fixtures`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:17344` → `🧫️fixtures/🔗️tool-latest-wins-integration.json` | `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:1404,1449` | Move it with the case-specific integration assertions. |

The physical-census `production-fixture-dependency` rule did not report a framework record because these accesses are test-gated. Its source rule should distinguish that fact while still reporting test-support code hosted outside a canonical test implementation.

### Stale JCO schema authority

`🧰️framework/🛍️products/💻️os/🧬️schema/🔣️.json:1733-1751` still declares all JCO package paths under removed `🧪️testkit/🧩️jcoprobe/...`. These seven constants must name the live owner `🧫️fixtures/🧩️jcoprobe/👽️guest/...`. This is the immediate metadata blocker for workspace-contract/discovery validation.

`🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🧪️destination-cases.json` is different: it is an intentionally invalid contract corpus. Its `historical-owner` and `stacked-adapter` values are negative vectors and should remain test fixture data.

## Invalid wiring not enforced by the physical census

The guard says external Rust test modules require `#[path]`; current source permits `include!(…/🧪️tests/…)`, which textually injects a test source into a different module and bypasses that contract. The live framework has **32 Rust source consumers** of this form, including **20 under OS**. Four are in the plug-in area:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-native-aggregate-backing/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`

The last file has seven cross-case injections at lines `36`, `380`, `2582`, `2584`, `2586`, and `3111`; `plugin-runtime-native-aggregate-backing/🦀️.rs:2` has the seventh. `:2586` injects the physical `⚛️reactor/🔄️turn/🧪️tests/📄️command-page-authority/🦀️.rs`, which explains the census's four apparent inline-test findings and the malformed test-case-name finding there. The file contains real tests but is not independently module-wired; it is wrongly scoped support for the builder-contract case.

Replace each inclusion with a direct, owner-correct test implementation connected with `#[path]`, or fold a case-specific injected source into the case that consumes it. Do not exempt `include!` in the guard: add a dedicated invalid-wiring finding for `include!` paths containing `🧪️tests`, then migrate the 32 consumers deliberately. The cross-case plug-in injections should be the first partition because they cause both false classification and hidden ownership coupling.

## Guard false positives and narrow rule changes

| Census finding | Live classification | Required guard change |
| --- | --- | --- |
| `🌐️browser-bundle/🧪️tests/🌊️actor-import/📋️project.json` and `📜️script.ts` | Valid Nx test-project contract and its required runner. The script calls the canonical `🟦️.ts`; neither is fixture input. Launch consumers target this project. | Permit a case-root `📋️project.json` plus `📜️script.ts` only when the project command invokes that script and the script delegates to the direct canonical test implementation. Keep arbitrary JSON test input disallowed. |
| `📇️directory/🔌️client/🧪️tests/🔬️unit/🧱️transport.rs` | Valid case-specific `FakeTransport` support; `🦀️.rs:6-8` explicitly wires it through `#[path]`. | Permit a sibling Rust support source explicitly `#[path]`-referenced by the direct test implementation; still reject unreferenced executable children and test declarations in support files. |
| `⚛️reactor/🔄️turn/🧪️tests/📄️command-page-authority/🦀️.rs` | Not an inline test location. Its misleading census findings originate from cross-case `include!` wiring. | Report the injection as invalid wiring rather than treating `#[test]` in the physical test source as inline. |

## Handoff priorities

1. Correct the seven JCO schema constants before discovery/workspace-contract validation.
2. Move the one populated composition fixture and externalize the six direct inline test bodies plus the `wire_effect_laws` module.
3. Make `include!(…/🧪️tests/…)` visible to the guard; its 32 consumers are an existing framework-wide migration partition, with seven cross-case plug-in consumers immediately affecting the present audit.
4. Preserve the current JCO/scale fixture topology. Do not restore deleted empty legacy categories or move negative corpus data such as `destination-cases.json` out of `🧫️fixtures`.
