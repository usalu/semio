# Plugin Publication Script Extraction

Date: 2026-09-12  
Executor: GPT-5.6 Sol, extra-high reasoning  
Ticket: KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE

## Outcome

The 862-line Rust descriptor tool entry and the 3,561-line registry entry no longer own publication behavior. Their package/root `📜️script.ts` files are 6 and 11 lines respectively and only import command classes, register literal commands, and invoke the shared router. Descriptor production, fresh source epochs, registry discovery, playground selection/session staging, catalog projection/viewing, taxonomy checks, scaffolding, descriptor verification, and trusted catalog publication now live in 15 neutral semantic owners with anonymous TypeScript leaves.

The extraction preserves the existing Rust package and registry target identities. No compatibility exports, runtime dependencies, migration scripts, or alternate entry paths were added.

## Source identity and behavior preservation

The live inputs were copied to ticket scratch before edits:

| Source | Lines | Bytes | SHA-256 |
| --- | ---: | ---: | --- |
| descriptor package `📜️script.ts` | 862 | 50,800 | `dc775413da6d1600d763854b815e9d5492ef3d400ce7d9ce47204241b6c38b0b` |
| registry `📜️script.ts` | 3,561 | 220,610 | `40896165f65d5251a179e7dbde10dd20d4b83c577739f58e5947c71510ad8f05` |

A TypeScript AST statement audit after the domain split and before the final registration-only router cleanup found 291 original declarations, 286 declarations in semantic owners, and 278 byte-identical declaration statements. The eight purposeful changes were path-relative fixture rebases, direct registry-filter direction, session/catalog-view rebases, and native-oracle fixture rebases. The five removed declarations were entry composition or superseded duplicate command glue. The final cleanup moved the remaining build/test/describe/session command wrappers into their existing owners and changed native file construction from URL `.pathname` to `join(import.meta.dir, ...)`; all portable, bundle, policy, and native checks below were rerun after that cleanup.

The current bindgen authority was checked directly. `world actor` imports only `pure` and `host-async`; `return-page` is not imported by that world. The obsolete empty `actor_bindings::semio::framework::byte_page::Host` implementation was removed instead of adding a compatibility binding. The native Rust suite then compiled and passed.

## Semantic owners

| Owner | Responsibility | Lines | Bytes | SHA-256 |
| --- | --- | ---: | ---: | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧾️source-epoch/🟦️.ts` | Stable input capture, source epochs, leases, ordered receipts | 286 | 17048 | `fc7790490c5f89783d922d70c6f2f244f7a624b653af85899be14b4bab4ac5f1` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts` | Cargo/JCO component construction and build/test command adapters | 76 | 4861 | `887136a12d957bb2c5c4b4c81ed5b4f938ba55b9d46a14e501cb1f7e63792f46` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🟦️.ts` | Bounded descriptor decoding, verification, atomic publication, and describe command | 140 | 7766 | `475d338ac3ab51e1bcef00ab643369c95a0acfec41b0505efa22506ae210b71f` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts` | Fresh plugin/extension orchestration with progress and cancellation | 341 | 22301 | `5a5700a206e889d2599b436876c17fadbc316f95f892db302a6ecdeb576e4b6e` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts` | Descriptor/Cargo discovery, plugin identities, dependency closure | 499 | 26845 | `51cd01aec3a143cccf9bc00fa6b0554bc5bd24afd5bc884b2c085b510b1ccbd4` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts` | Playground rows and host/plugin selection | 208 | 11802 | `f40dbae29ce14effd3d74b5a1bc3da3446095c0ea178fb4624e72b6694f2f165` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts` | Session projection, staging, publication receipt, and session command | 131 | 6777 | `67da6a2854c17e4228f0918db31c565777e60fa279b0decf56ed10376febae07` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧰️framework-catalog/🟦️.ts` | Framework package catalog derivation | 65 | 2950 | `56f2840a854cb56f59629b9fe619c565ca970b4263b010ac3ed497c997e684b9` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts` | Catalog rendering, Rust host/artifact emission, generation commands | 510 | 32518 | `51f22e03f95191733a8accbbc367abb001d644bdcacb8886b55ec8736b707736` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts` | Lower generated-catalog read/filter view shared by session and projection | 36 | 2591 | `d757453fb05f66ada0cd35df4e18cb5c6c515c857d8e96cdbbcdbb3a560079ff` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts` | Rust mounts and plugin-root taxonomy checks | 733 | 43044 | `6ed8e8e496ed181a453063e59a0aa18191c6797ef8fde02b5b987507bba88195` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌳️surface-scaffold/🟦️.ts` | New plugin/extension surface scaffolding | 250 | 13419 | `e151af7e0c73ccb16952046814d159a1ac721df14b2afa25be2c171bde1afba3` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🟦️.ts` | Published descriptor validation | 153 | 9924 | `3d44b0b54892e4ed5e256cb8ae12873ada69c5033f8a6545786f3ddf0f08895c` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts` | Catalog completeness, descriptor pair verification, native selection and test command | 1055 | 65918 | `affeea2c90025548ca6c632ef1747dce626c0700adabdc8b35432bb4d2264a2e` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️trusted-stdio-catalog/🟦️.ts` | Trusted stdio catalog construction and atomic publication | 221 | 11672 | `bc5cf1337135efe3669f7114cf5a4592e5c5816d3982813b822e12c45002daa0` |

The generator/registry command classes remain explicit native command adapters inside their matching owners. The two command entry files export no API and contain no domain computation.

## Dependency direction

Registry discovery owns Cargo/descriptor parsing, plugin identities, dependency closure, and direct plugin filtering. Playground discovery consumes it and owns host/playground selection. `📖️catalog-view` is the lower generated-row reader/filter consumed by both projection and session. Projection owns native Rust host/artifact emission and composes the session output. This removes the actual local runtime cycle that previously existed between session and projection.

Required edges:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts`

Forbidden reverse/upward edges:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts` ⇏ `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts` ⇏ `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts` ⇏ `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts` ⇏ `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🟦️.ts` ⇏ `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts` ⇏ `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🟦️.ts`

The portable test walks every reachable local static import, re-export, and dynamic import from both routers. It reports a cycle path if any local runtime cycle is present; the final graph is acyclic.

The coordinator independently repeated the native TypeScript dependency walk from both live routers after the final split: 18 reachable plugin modules, 42 local edges, zero syntax diagnostics, zero missing or computed imports, and zero cycles. Its raw evidence is retained at `🗑️generated/coordinator/publication-final-local-module-graph.json`.

## Schema and contextual authority

The taxonomy adds or uses these exact contextual mappings:

| Name | Parent kind | Resolved kind |
| --- | --- | --- |
| `📣️plugin-publication-source-ownership` | `schema` | `plugin-publication-source-ownership` |
| `📣️plugin-publication-source-ownership` | `fixtures` | `plugin-publication-source-ownership` |
| `📣️plugin-publication-source-ownership` | `tests` | `plugin-publication-source-ownership` |
| `🧾️source-epoch` | `plugin-descriptor-emitter` | `os-describe-source-epoch` |
| `🏗️component-build` | `plugin-descriptor-emitter` | `os-describe-component-build` |
| `🛂️descriptor-emission` | `plugin-descriptor-emitter` | `os-descriptor-emission` |
| `🏭️fresh-component` | `plugin-descriptor-emitter` | `os-describe-fresh-component` |
| `🔎️discovery` | `registry` | `os-registry-discovery` |
| `🎮️playground` | `registry` | `os-registry-playground` |
| `🔎️discovery` | `os-registry-playground` | `os-registry-playground-discovery` |
| `🧭️session` | `os-registry-playground` | `os-registry-playground-session` |
| `🧰️framework-catalog` | `registry` | `os-registry-framework-catalog` |
| `📽️projection` | `registry` | `os-registry-projection` |
| `🗿️taxonomy-validation` | `registry` | `os-registry-taxonomy-validation` |
| `🌳️surface-scaffold` | `registry` | `os-registry-surface-scaffold` |
| `🛂️descriptor-verification` | `registry` | `os-registry-descriptor-verification` |
| `✅️catalog-verification` | `registry` | `os-registry-catalog-verification` |
| `✅️trusted-stdio-catalog` | `registry` | `os-registry-trusted-stdio-catalog` |
| `🚀️launch` | `registry` | `os-registry-launch` |
| `📖️catalog-view` | `registry` | `os-registry-catalog-view` |

The `plugin-registry` and `playground-session` generator input patterns were updated with the exact semantic source owners in UTF-8 byte order. Strict taxonomy loading validates both taxonomy and generator workspace contracts.

## Consumer closure

The portable fixture records 63 source, runtime, test, generator, dynamic-import, and source-as-data consumers. Each listed consumer resolves and imports only the semantic owner paths. No consumer imports either old behavior-bearing script path. Hub source-epoch/source-as-data references, plugin and extension build scripts, OS development/session consumers, WGPU source authority, GIS, generated projection tests, and registry checks are included.

- `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧾️source-epoch/🟦️.ts`
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts`
- `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧾️source-epoch/🟦️.ts`

## Runtime and oracle evidence

| Check | Result |
| --- | --- |
| Direct portable publication ownership gate | 6 passed, 0 failed, 389 assertions |
| Registered Nx `@semio-tech/repo-lib:test-plugin-publication-source-ownership` | 6 passed, 0 failed, 389 assertions; 1.0 s Nx run |
| Strict `loadTaxonomy()` + `validateTaxonomy()` | `[]` |
| Scoped inventory, all 15 owners + both routers | 17 scopes, zero violations |
| Registered package body/fixed-script policy | 120 passed, 0 failed, 525 assertions |
| Independent final router dependency graph | 18 modules, 42 local edges, 0 parse/missing/computed imports, 0 cycles |
| Final Bun registry entry bundle | 292 modules, 12.31 MB |
| Final Bun descriptor entry bundle | 285 modules, 12.10 MB |
| Registry generation/freshness | 59 plugins, 61 playgrounds, 50 framework entries generated; `check-generated` passed |
| Runtime catalog/session imports | registry 59, playgrounds 61, projected 59, staged session 59; selected host variant `s` |
| Native catalog selection oracle | 23 cases: 4 planned, 19 denied, zero publication |
| Session + generated projection tests | 2 files, 7 passed |
| Catalog completeness | 12 passed |
| Plugin identity | 3 passed |
| Trusted stdio catalog | 4 passed |
| Fresh component source epoch | 17 physical laws; bounded-input 2; unsafe-input 7; dep-info 19 + 3 |
| Fresh staging | 11 laws passed |
| Fresh process behavior | 8 laws passed, covering success, errors, spawn failure, cancellation, deadline, and output bound |
| Focused launch preview authority | 1 passed, 4 skipped |
| Registered Rust descriptor quick suite | 20 passed, 2 skipped across 2 binaries; all 3 generator dependencies passed |
| Registry generated catalog/launch freshness after native run | passed |

The fresh-component fixtures use schema-required `appChannelVersion: 17`. Publication continues to use bounded reads, cancellation/deadline checkpoints, temporary directories, and atomic renames. The native selection oracle remains planning-only and produced no live publication. Session tests used the isolated session fixture paths and did not write the canonical session output.

## Validation limits

An earlier full launch-contract run reached one separate current WGPU profile-policy mismatch: its fixture expected only the lowpoly and puzzle overrides while live Cargo authority contained nine different package overrides. The publication-specific preview assertion was repaired and passed; this lane did not rewrite that WGPU policy fixture.

The Rust quick profile deliberately skipped two tests selected out by the repository's quick-test policy. The 20 selected native tests compiled and passed after the obsolete binding removal.

## Exact changed-file attribution

This is the lane-owned union of semantic owners, routers, the 63 fixture-recorded consumer files, and the support/authority files changed for schema, tests, native binding, launch, or source-epoch closure. It intentionally excludes unrelated concurrent workspace changes.

- `.🧬semio/🦑️repo/🔣️taxonomy.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`
- `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️trusted-stdio-catalog/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌳️surface-scaffold/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧰️framework-catalog/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧫️fixtures/🧊️fresh-staging/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧾️source-epoch/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️plugin-publication-source-ownership/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📣️plugin-publication-source-ownership/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📣️plugin-publication-source-ownership/🔣️.json`

The prior manifestless report was also amended with its separately derived 68-path exact attribution list, as requested.

## Cleanup

All compiler products, bundle outputs, source snapshots, Nx workspace data, generated evidence JSON, and private preview output for this lane were confined to `🗑️generated/sol-plugin-publication-script-extraction`. That scratch directory is removed after this report is retained.
