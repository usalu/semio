# Current Full Caching Script Strict Diagnostics

Actual strict checks each settled with exit1. These diagnostics are retained as an audit; no all-code strict pass is claimed. The latest run includes the current diagnostics callback, explicit environment, direct helper imports and async child changes.

## root-cache-test-artifact-types.log

Actual diagnostic count: 250

```text
🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts(33,429): error TS18046: 'metadataFixture' is of type 'unknown'.
🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts(37,275): error TS18046: 'metadataFixture' is of type 'unknown'.
🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts(37,359): error TS18046: 'metadataFixture' is of type 'unknown'.
🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts(37,714): error TS18046: 'metadataFixture' is of type 'unknown'.
🧰️framework/🔨️modules/🚪️io/🔀️stream-mux/🟦️.ts(154,25): error TS18048: 'shape' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts(62,42): error TS18048: 'bundles.bun' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts(62,65): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts(67,50): error TS18048: 'bundles.esbuild' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts(72,139): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string | ArrayBufferLike | ArrayBufferView<ArrayBufferLike>'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts(41,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts(67,66): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(73,14): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(215,10): error TS18048: 'row' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(215,23): error TS18048: 'row' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(216,84): error TS18048: 'row' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(216,115): error TS18048: 'row' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(217,16): error TS2322: Type '{ examples: readonly string[]; engines: readonly string[]; variant?: string | undefined; catalogDefault?: boolean; pluginId?: string | undefined; cratePath?: string | undefined; app?: string; brand?: string; devContribution?: string; ... 12 more ...; assets?: readonly AssetSpecRow[] | undefined; }' is not assignable to type 'PlaygroundEntry'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(217,78): error TS18048: 'row' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(217,103): error TS18048: 'row' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts(50,19): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(88,62): error TS2345: Argument of type '(AreaState | undefined)[]' is not assignable to parameter of type 'readonly AreaState[]'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(102,57): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(104,55): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(156,41): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(175,9): error TS2488: Type '[string, unknown] | undefined' must have a '[Symbol.iterator]()' method that returns an iterator.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(401,21): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(404,11): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(405,17): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(417,3): error TS2322: Type '(string | undefined)[]' is not assignable to type 'string[]'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(417,14): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts(1164,46): error TS18048: 'pathname' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts(1167,34): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts(1202,22): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts(1202,54): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts(51,19): error TS18048: 'source' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts(53,28): error TS18048: 'source' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts(54,28): error TS18048: 'source' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts(54,102): error TS18048: 'source' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts(55,28): error TS18048: 'source' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts(74,51): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts(76,67): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🧪️tests/🧊️mesh-pack-decode/🟦️.ts(42,14): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🧪️tests/🧊️mesh-pack-decode/🟦️.ts(43,14): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🧪️tests/🧊️mesh-pack-decode/🟦️.ts(44,14): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/📦️package/🟦️.ts(26,57): error TS2769: No overload matches this call.
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/📦️package/🟦️.ts(26,127): error TS2769: No overload matches this call.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts(25,40): error TS7016: Could not find a declaration file for module '../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📇️registry/🟦️.ts(72,46): error TS18048: 'other' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📇️registry/🟦️.ts(72,65): error TS18048: 'other' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📇️registry/🟦️.ts(72,106): error TS18048: 'entry' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📇️registry/🟦️.ts(72,124): error TS18048: 'entry' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📝️derived-config/🧪️tests/📝️derived-config/🟦️.ts(80,72): error TS2339: Property 'profile' does not exist on type 'object'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🟦️bun/📜️script.ts(27,79): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🟦️bun/📜️script.ts(47,28): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🟦️bun/📜️script.ts(73,28): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🟦️bun/📜️script.ts(88,26): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/✅️verification/🟦️.ts(4,20): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts(40,1494): error TS18046: 'lock' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts(40,1604): error TS18046: 'lock' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts(40,3899): error TS18046: 'locked' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts(40,3937): error TS18046: 'locked' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts(40,4021): error TS2571: Object is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🛠️binaryen-toolchain/🟦️.ts(28,43): error TS7016: Could not find a declaration file for module '../../../../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts(68,79): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts(522,37): error TS2300: Duplicate identifier 'invocation'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts(678,106): error TS18048: 'digest' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts(882,11): error TS2300: Duplicate identifier 'invocation'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts(883,49): error TS2339: Property 'getText' does not exist on type 'ScriptInvocation<object>'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts(1082,27): error TS2339: Property 'getText' does not exist on type 'ScriptInvocation<object>'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts(1082,93): error TS2339: Property 'getText' does not exist on type 'ScriptInvocation<object>'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌐️browser/🟦️.ts(101,123): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌐️next-runtime/🟦️.ts(30,146): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌐️next-runtime/🟦️.ts(52,145): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🔐️custody/🟦️.ts(34,11): error TS2339: Property 'SEMIO_SCRIPT_PROCESS_INVOCATION' does not exist on type '{ NX_SKIP_NX_CACHE: string; NX_DISABLE_NX_CACHE: string; NX_SKIP_REMOTE_CACHE: string; NX_DAEMON: string; NX_ISOLATE_PLUGINS: string; NX_WORKSPACE_DATA_DIRECTORY: string; NX_CACHE_DIRECTORY: string; NX_FORCE_REUSE_CACHED_GRAPH: string; NODE_ENV?: string | undefined; TZ?: string | undefined; }'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🔐️custody/🟦️.ts(34,188): error TS2339: Property 'SEMIO_SCRIPT_CAPABILITIES' does not exist on type '{ NX_SKIP_NX_CACHE: string; NX_DISABLE_NX_CACHE: string; NX_SKIP_REMOTE_CACHE: string; NX_DAEMON: string; NX_ISOLATE_PLUGINS: string; NX_WORKSPACE_DATA_DIRECTORY: string; NX_CACHE_DIRECTORY: string; NX_FORCE_REUSE_CACHED_GRAPH: string; NODE_ENV?: string | undefined; TZ?: string | undefined; }'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🔐️custody/🟦️.ts(34,225): error TS2339: Property 'SEMIO_BUILD_GRAPH_PATH' does not exist on type '{ NX_SKIP_NX_CACHE: string; NX_DISABLE_NX_CACHE: string; NX_SKIP_REMOTE_CACHE: string; NX_DAEMON: string; NX_ISOLATE_PLUGINS: string; NX_WORKSPACE_DATA_DIRECTORY: string; NX_CACHE_DIRECTORY: string; NX_FORCE_REUSE_CACHED_GRAPH: string; NODE_ENV?: string | undefined; TZ?: string | undefined; }'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🔐️custody/🟦️.ts(34,259): error TS2339: Property 'SEMIO_BUILD_GRAPH_DIGEST' does not exist on type '{ NX_SKIP_NX_CACHE: string; NX_DISABLE_NX_CACHE: string; NX_SKIP_REMOTE_CACHE: string; NX_DAEMON: string; NX_ISOLATE_PLUGINS: string; NX_WORKSPACE_DATA_DIRECTORY: string; NX_CACHE_DIRECTORY: string; NX_FORCE_REUSE_CACHED_GRAPH: string; NODE_ENV?: string | undefined; TZ?: string | undefined; }'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🔐️custody/🟦️.ts(34,295): error TS2339: Property 'SEMIO_BUILD_REUSE_GRAPH' does not exist on type '{ NX_SKIP_NX_CACHE: string; NX_DISABLE_NX_CACHE: string; NX_SKIP_REMOTE_CACHE: string; NX_DAEMON: string; NX_ISOLATE_PLUGINS: string; NX_WORKSPACE_DATA_DIRECTORY: string; NX_CACHE_DIRECTORY: string; NX_FORCE_REUSE_CACHED_GRAPH: string; NODE_ENV?: string | undefined; TZ?: string | undefined; }'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(15,53): error TS7016: Could not find a declaration file for module '../../../🏗️build/📥️inputs/🔒️lock/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔒️lock/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(16,41): error TS7016: Could not find a declaration file for module '../../../🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(17,74): error TS7016: Could not find a declaration file for module '../../../🏗️build/📥️inputs/🧮️digest/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(19,32): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(21,49): error TS7016: Could not find a declaration file for module '../../../../🧪️test/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(22,34): error TS7016: Could not find a declaration file for module '../../🕸️graph/📤️export/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/📤️export/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(45,43): error TS18046: 'frozen' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(45,107): error TS18046: 'frozen' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(45,182): error TS18046: 'frozen' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(70,53): error TS18046: 'input' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(71,132): error TS18046: 'sourceInputEnv' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(72,59): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(72,96): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(72,151): error TS7006: Parameter 'path' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(72,190): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(72,227): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(74,54): error TS7006: Parameter 'path' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(85,81): error TS18046: 'plan' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(85,90): error TS7006: Parameter 'index' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(114,194): error TS7006: Parameter 'child' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(131,3): error TS7053: Element implicitly has an 'any' type because expression of type '"native-build-release"' can't be used to index type '{ build: { executor: string; cache: boolean; dependsOn: string[]; outputs: string[]; options: { command: string; }; } & { defaultConfiguration: "release" | "prototype"; configurations: Record<string, unknown>; metadata: Record<...>; }; }'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(138,40): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(144,52): error TS18046: 'lock' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(147,96): error TS18046: 'input' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(154,33): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(161,303): error TS2339: Property 'SEMIO_SCRIPT_CAPABILITIES' does not exist on type '{ SEMIO_SCRIPT_PROCESS_INVOCATION: string; NODE_ENV?: string | undefined; TZ?: string | undefined; }'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(162,574): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(176,106): error TS7016: Could not find a declaration file for module '../../../🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(176,392): error TS7006: Parameter 'edge' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(177,145): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(177,199): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(178,63): error TS7006: Parameter 'edge' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(179,103): error TS18046: 'goOwner' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(179,242): error TS18046: 'goDependency' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(180,56): error TS18046: 'goDependency' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(181,35): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(181,94): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(181,130): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(181,170): error TS18046: 'goDependency' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(182,53): error TS18046: 'goDependency' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(182,127): error TS7006: Parameter 'pattern' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(182,205): error TS7006: Parameter 'pattern' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(183,15): error TS18046: 'goOwner' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(183,49): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(186,69): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(186,69): error TS2571: Object is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(194,89): error TS18048: 'line' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(194,100): error TS18048: 'line' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(198,15): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(198,15): error TS2571: Object is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(245,13): error TS7022: 'root' implicitly has type 'any' because it does not have a type annotation and is referenced directly or indirectly in its own initializer.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(251,15): error TS7022: 'entries' implicitly has type 'any' because it does not have a type annotation and is referenced directly or indirectly in its own initializer.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(251,119): error TS7022: 'actual' implicitly has type 'any' because it does not have a type annotation and is referenced directly or indirectly in its own initializer.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(251,191): error TS7022: 'oracle' implicitly has type 'any' because it does not have a type annotation and is referenced directly or indirectly in its own initializer.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎯️selected-runtime/🟦️.ts(34,43): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🏗️web-build/🟦️.ts(61,129): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🏗️web-build/🟦️.ts(251,15): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🏗️web-build/🟦️.ts(252,18): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/🟦️.ts(113,32): error TS2339: Property 'package' does not exist on type 'object'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/🟦️.ts(133,126): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/🟦️.ts(133,153): error TS2339: Property 'sort' does not exist on type 'readonly string[]'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔁️graph-revision/🟦️.ts(11,43): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔒️trunk-lockfile/🟦️.ts(72,45): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔗️command-imports/🟦️.ts(11,66): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔗️import-edges/🟦️.ts(10,43): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🖥️services/🟦️.ts(116,35): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️browser-serving/🟦️.ts(30,31): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️browser-serving/🟦️.ts(31,54): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️browser-serving/🟦️.ts(33,26): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️browser-serving/🟦️.ts(65,18): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️live-activation/🟦️.ts(22,43): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️native-renderer-outputs/🟦️.ts(29,108): error TS2698: Spread types may only be created from object types.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️native-renderer-outputs/🟦️.ts(73,51): error TS7006: Parameter 'error' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️native-runtime/🟦️.ts(26,61): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts(5,63): error TS7016: Could not find a declaration file for module '../../../🕸️dependencies/🟦️typescript/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts(18,32): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts(24,31): error TS2538: Type 'undefined' cannot be used as an index type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(107,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(111,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(112,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(113,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(114,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(115,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(116,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(117,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(119,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(120,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📜️script.ts(3,37): error TS7016: Could not find a declaration file for module './🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📜️script.ts(4,37): error TS7016: Could not find a declaration file for module './🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📜️script.ts(14,47): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(720,63): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(2336,19): error TS18048: 'taxonomy.semanticDirectoryKinds.standard' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(2336,124): error TS18048: 'taxonomy.semanticDirectoryKinds.subset' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5534,573): error TS2345: Argument of type 'PackageGlueGrammarSpec | undefined' is not assignable to parameter of type 'PackageGlueGrammarSpec'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5681,49): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5684,45): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5685,60): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5687,63): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5687,161): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5690,224): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5697,85): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5699,49): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5700,101): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5705,105): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5709,398): error TS2345: Argument of type 'PackageGlueGrammarSpec | undefined' is not assignable to parameter of type 'PackageGlueGrammarSpec'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5713,375): error TS2345: Argument of type 'PackageGlueGrammarSpec | undefined' is not assignable to parameter of type 'PackageGlueGrammarSpec'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5724,107): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5724,190): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5851,65): error TS2345: Argument of type 'PackageGlueGrammarSpec | undefined' is not assignable to parameter of type 'PackageGlueGrammarSpec'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(8906,65): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(8931,38): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(8959,38): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(8960,100): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(8997,62): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(8999,58): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9052,22): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9053,175): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9073,32): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9078,9): error TS2322: Type 'string | undefined' is not assignable to type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9087,105): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9087,117): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9100,157): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9112,45): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9144,28): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9229,56): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9304,74): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9309,100): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9324,48): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9366,75): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9404,88): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9464,214): error TS2345: Argument of type 'number | undefined' is not assignable to parameter of type 'number'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9504,67): error TS18048: 'token' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9505,46): error TS18048: 'token' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9524,46): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9525,86): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9535,11): error TS18048: 'name' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9547,58): error TS18048: 'name' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9577,48): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9577,115): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9588,80): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9592,54): error TS18048: 'name' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9599,93): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9607,99): error TS2345: Argument of type 'number | undefined' is not assignable to parameter of type 'number'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9618,68): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9622,66): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9726,9): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9727,15): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9807,14): error TS2538: Type 'undefined' cannot be used as an index type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9807,35): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9811,29): error TS2538: Type 'undefined' cannot be used as an index type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(11201,146): error TS2345: Argument of type 'PackageGlueGrammarSpec | undefined' is not assignable to parameter of type 'PackageGlueGrammarSpec'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(11202,64): error TS2345: Argument of type 'PackageGlueGrammarSpec | undefined' is not assignable to parameter of type 'PackageGlueGrammarSpec'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(11209,91): error TS18048: 'sourceGrammar' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(11749,3): error TS2322: Type '(string | undefined)[][]' is not assignable to type 'readonly string[][]'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🧪️tests/⚡️inputs/🟦️.ts(15,28): error TS7016: Could not find a declaration file for module '../../🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🧪️tests/⚡️inputs/🟦️.ts(44,60): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(601,16): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(629,10): error TS18048: 'line' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(630,43): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1215,33): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1291,36): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1301,12): error TS18048: 'best' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1308,38): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1318,12): error TS18048: 'best' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1328,23): error TS2322: Type 'string | undefined' is not assignable to type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1346,33): error TS2322: Type 'string | undefined' is not assignable to type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1581,30): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1583,16): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(3572,33): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(3572,52): error TS2322: Type 'string | undefined' is not assignable to type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🧪️tests/🦀️inputs/🟦️.ts(24,32): error TS7016: Could not find a declaration file for module '../../../🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🧪️tests/🦀️inputs/🟦️.ts(45,43): error TS7016: Could not find a declaration file for module '../../../../📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
```

## root-cache-dispatch-broad-types2.log

Actual diagnostic count: 250

```text
🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts(33,429): error TS18046: 'metadataFixture' is of type 'unknown'.
🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts(37,275): error TS18046: 'metadataFixture' is of type 'unknown'.
🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts(37,359): error TS18046: 'metadataFixture' is of type 'unknown'.
🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts(37,714): error TS18046: 'metadataFixture' is of type 'unknown'.
🧰️framework/🔨️modules/🚪️io/🔀️stream-mux/🟦️.ts(154,25): error TS18048: 'shape' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts(62,42): error TS18048: 'bundles.bun' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts(62,65): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts(67,50): error TS18048: 'bundles.esbuild' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts(72,139): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string | ArrayBufferLike | ArrayBufferView<ArrayBufferLike>'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts(41,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts(67,66): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(73,14): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(215,10): error TS18048: 'row' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(215,23): error TS18048: 'row' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(216,84): error TS18048: 'row' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(216,115): error TS18048: 'row' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(217,16): error TS2322: Type '{ examples: readonly string[]; engines: readonly string[]; variant?: string | undefined; catalogDefault?: boolean; pluginId?: string | undefined; cratePath?: string | undefined; app?: string; brand?: string; devContribution?: string; ... 12 more ...; assets?: readonly AssetSpecRow[] | undefined; }' is not assignable to type 'PlaygroundEntry'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(217,78): error TS18048: 'row' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts(217,103): error TS18048: 'row' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts(50,19): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(88,62): error TS2345: Argument of type '(AreaState | undefined)[]' is not assignable to parameter of type 'readonly AreaState[]'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(102,57): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(104,55): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(156,41): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(175,9): error TS2488: Type '[string, unknown] | undefined' must have a '[Symbol.iterator]()' method that returns an iterator.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(401,21): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(404,11): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(405,17): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(417,3): error TS2322: Type '(string | undefined)[]' is not assignable to type 'string[]'.
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts(417,14): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts(1164,46): error TS18048: 'pathname' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts(1167,34): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts(1202,22): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts(1202,54): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts(51,19): error TS18048: 'source' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts(53,28): error TS18048: 'source' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts(54,28): error TS18048: 'source' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts(54,102): error TS18048: 'source' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts(55,28): error TS18048: 'source' is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts(74,51): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts(76,67): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🧪️tests/🧊️mesh-pack-decode/🟦️.ts(42,14): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🧪️tests/🧊️mesh-pack-decode/🟦️.ts(43,14): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/💻️os/🧪️tests/🧊️mesh-pack-decode/🟦️.ts(44,14): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/📦️package/🟦️.ts(26,57): error TS2769: No overload matches this call.
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/📦️package/🟦️.ts(26,127): error TS2769: No overload matches this call.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts(29,40): error TS7016: Could not find a declaration file for module '../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📇️registry/🟦️.ts(72,46): error TS18048: 'other' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📇️registry/🟦️.ts(72,65): error TS18048: 'other' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📇️registry/🟦️.ts(72,106): error TS18048: 'entry' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📇️registry/🟦️.ts(72,124): error TS18048: 'entry' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📝️derived-config/🧪️tests/📝️derived-config/🟦️.ts(80,72): error TS2339: Property 'profile' does not exist on type 'object'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🟦️bun/📜️script.ts(27,79): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🟦️bun/📜️script.ts(47,28): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🟦️bun/📜️script.ts(73,28): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🟦️bun/📜️script.ts(88,26): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/✅️verification/🟦️.ts(4,20): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts(40,1494): error TS18046: 'lock' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts(40,1604): error TS18046: 'lock' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts(40,3899): error TS18046: 'locked' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts(40,3937): error TS18046: 'locked' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts(40,4021): error TS2571: Object is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🛠️binaryen-toolchain/🟦️.ts(28,43): error TS7016: Could not find a declaration file for module '../../../../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts(68,79): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts(522,37): error TS2300: Duplicate identifier 'invocation'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts(678,106): error TS18048: 'digest' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts(882,11): error TS2300: Duplicate identifier 'invocation'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts(883,49): error TS2339: Property 'getText' does not exist on type 'ScriptInvocation<object>'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts(1082,27): error TS2339: Property 'getText' does not exist on type 'ScriptInvocation<object>'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts(1082,93): error TS2339: Property 'getText' does not exist on type 'ScriptInvocation<object>'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌐️browser/🟦️.ts(101,123): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌐️next-runtime/🟦️.ts(30,146): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌐️next-runtime/🟦️.ts(52,145): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🔐️custody/🟦️.ts(34,11): error TS2339: Property 'SEMIO_SCRIPT_PROCESS_INVOCATION' does not exist on type '{ NX_SKIP_NX_CACHE: string; NX_DISABLE_NX_CACHE: string; NX_SKIP_REMOTE_CACHE: string; NX_DAEMON: string; NX_ISOLATE_PLUGINS: string; NX_WORKSPACE_DATA_DIRECTORY: string; NX_CACHE_DIRECTORY: string; NX_FORCE_REUSE_CACHED_GRAPH: string; NODE_ENV?: string | undefined; TZ?: string | undefined; }'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🔐️custody/🟦️.ts(34,188): error TS2339: Property 'SEMIO_SCRIPT_CAPABILITIES' does not exist on type '{ NX_SKIP_NX_CACHE: string; NX_DISABLE_NX_CACHE: string; NX_SKIP_REMOTE_CACHE: string; NX_DAEMON: string; NX_ISOLATE_PLUGINS: string; NX_WORKSPACE_DATA_DIRECTORY: string; NX_CACHE_DIRECTORY: string; NX_FORCE_REUSE_CACHED_GRAPH: string; NODE_ENV?: string | undefined; TZ?: string | undefined; }'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🔐️custody/🟦️.ts(34,225): error TS2339: Property 'SEMIO_BUILD_GRAPH_PATH' does not exist on type '{ NX_SKIP_NX_CACHE: string; NX_DISABLE_NX_CACHE: string; NX_SKIP_REMOTE_CACHE: string; NX_DAEMON: string; NX_ISOLATE_PLUGINS: string; NX_WORKSPACE_DATA_DIRECTORY: string; NX_CACHE_DIRECTORY: string; NX_FORCE_REUSE_CACHED_GRAPH: string; NODE_ENV?: string | undefined; TZ?: string | undefined; }'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🔐️custody/🟦️.ts(34,259): error TS2339: Property 'SEMIO_BUILD_GRAPH_DIGEST' does not exist on type '{ NX_SKIP_NX_CACHE: string; NX_DISABLE_NX_CACHE: string; NX_SKIP_REMOTE_CACHE: string; NX_DAEMON: string; NX_ISOLATE_PLUGINS: string; NX_WORKSPACE_DATA_DIRECTORY: string; NX_CACHE_DIRECTORY: string; NX_FORCE_REUSE_CACHED_GRAPH: string; NODE_ENV?: string | undefined; TZ?: string | undefined; }'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🔐️custody/🟦️.ts(34,295): error TS2339: Property 'SEMIO_BUILD_REUSE_GRAPH' does not exist on type '{ NX_SKIP_NX_CACHE: string; NX_DISABLE_NX_CACHE: string; NX_SKIP_REMOTE_CACHE: string; NX_DAEMON: string; NX_ISOLATE_PLUGINS: string; NX_WORKSPACE_DATA_DIRECTORY: string; NX_CACHE_DIRECTORY: string; NX_FORCE_REUSE_CACHED_GRAPH: string; NODE_ENV?: string | undefined; TZ?: string | undefined; }'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(15,53): error TS7016: Could not find a declaration file for module '../../../🏗️build/📥️inputs/🔒️lock/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔒️lock/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(16,41): error TS7016: Could not find a declaration file for module '../../../🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(17,74): error TS7016: Could not find a declaration file for module '../../../🏗️build/📥️inputs/🧮️digest/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(19,32): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(21,49): error TS7016: Could not find a declaration file for module '../../../../🧪️test/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(22,34): error TS7016: Could not find a declaration file for module '../../🕸️graph/📤️export/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/📤️export/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(45,43): error TS18046: 'frozen' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(45,107): error TS18046: 'frozen' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(45,182): error TS18046: 'frozen' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(70,53): error TS18046: 'input' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(71,132): error TS18046: 'sourceInputEnv' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(72,59): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(72,96): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(72,151): error TS7006: Parameter 'path' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(72,190): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(72,227): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(74,54): error TS7006: Parameter 'path' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(85,81): error TS18046: 'plan' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(85,90): error TS7006: Parameter 'index' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(114,194): error TS7006: Parameter 'child' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(131,3): error TS7053: Element implicitly has an 'any' type because expression of type '"native-build-release"' can't be used to index type '{ build: { executor: string; cache: boolean; dependsOn: string[]; outputs: string[]; options: { command: string; }; } & { defaultConfiguration: "release" | "prototype"; configurations: Record<string, unknown>; metadata: Record<...>; }; }'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(138,40): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(144,52): error TS18046: 'lock' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(147,96): error TS18046: 'input' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(154,33): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(161,303): error TS2339: Property 'SEMIO_SCRIPT_CAPABILITIES' does not exist on type '{ SEMIO_SCRIPT_PROCESS_INVOCATION: string; NODE_ENV?: string | undefined; TZ?: string | undefined; }'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(162,574): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(176,106): error TS7016: Could not find a declaration file for module '../../../🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(176,392): error TS7006: Parameter 'edge' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(177,145): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(177,199): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(178,63): error TS7006: Parameter 'edge' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(179,103): error TS18046: 'goOwner' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(179,242): error TS18046: 'goDependency' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(180,56): error TS18046: 'goDependency' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(181,35): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(181,94): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(181,130): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(181,170): error TS18046: 'goDependency' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(182,53): error TS18046: 'goDependency' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(182,127): error TS7006: Parameter 'pattern' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(182,205): error TS7006: Parameter 'pattern' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(183,15): error TS18046: 'goOwner' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(183,49): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(186,69): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(186,69): error TS2571: Object is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(194,89): error TS18048: 'line' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(194,100): error TS18048: 'line' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(198,15): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(198,15): error TS2571: Object is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(245,13): error TS7022: 'root' implicitly has type 'any' because it does not have a type annotation and is referenced directly or indirectly in its own initializer.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(251,15): error TS7022: 'entries' implicitly has type 'any' because it does not have a type annotation and is referenced directly or indirectly in its own initializer.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(251,119): error TS7022: 'actual' implicitly has type 'any' because it does not have a type annotation and is referenced directly or indirectly in its own initializer.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(251,191): error TS7022: 'oracle' implicitly has type 'any' because it does not have a type annotation and is referenced directly or indirectly in its own initializer.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎯️selected-runtime/🟦️.ts(34,43): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🏗️web-build/🟦️.ts(61,129): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🏗️web-build/🟦️.ts(251,15): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🏗️web-build/🟦️.ts(252,18): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/🟦️.ts(113,32): error TS2339: Property 'package' does not exist on type 'object'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/🟦️.ts(133,126): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/🟦️.ts(133,153): error TS2339: Property 'sort' does not exist on type 'readonly string[]'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔁️graph-revision/🟦️.ts(11,43): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔒️trunk-lockfile/🟦️.ts(72,45): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔗️command-imports/🟦️.ts(11,66): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔗️import-edges/🟦️.ts(10,43): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🖥️services/🟦️.ts(116,35): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️browser-serving/🟦️.ts(30,31): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️browser-serving/🟦️.ts(31,54): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️browser-serving/🟦️.ts(33,26): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️browser-serving/🟦️.ts(65,18): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️live-activation/🟦️.ts(22,43): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️native-renderer-outputs/🟦️.ts(29,108): error TS2698: Spread types may only be created from object types.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️native-renderer-outputs/🟦️.ts(73,51): error TS7006: Parameter 'error' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️native-runtime/🟦️.ts(26,61): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts(5,63): error TS7016: Could not find a declaration file for module '../../../🕸️dependencies/🟦️typescript/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts(18,32): error TS7016: Could not find a declaration file for module '../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts(24,31): error TS2538: Type 'undefined' cannot be used as an index type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(107,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(111,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(112,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(113,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(114,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(115,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(116,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(117,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(119,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts(120,16): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📜️script.ts(3,37): error TS7016: Could not find a declaration file for module './🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📜️script.ts(4,37): error TS7016: Could not find a declaration file for module './🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📜️script.ts(14,47): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(720,63): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(2336,19): error TS18048: 'taxonomy.semanticDirectoryKinds.standard' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(2336,124): error TS18048: 'taxonomy.semanticDirectoryKinds.subset' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5534,573): error TS2345: Argument of type 'PackageGlueGrammarSpec | undefined' is not assignable to parameter of type 'PackageGlueGrammarSpec'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5681,49): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5684,45): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5685,60): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5687,63): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5687,161): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5690,224): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5697,85): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5699,49): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5700,101): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5705,105): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5709,398): error TS2345: Argument of type 'PackageGlueGrammarSpec | undefined' is not assignable to parameter of type 'PackageGlueGrammarSpec'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5713,375): error TS2345: Argument of type 'PackageGlueGrammarSpec | undefined' is not assignable to parameter of type 'PackageGlueGrammarSpec'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5724,107): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5724,190): error TS18048: 'contract' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(5851,65): error TS2345: Argument of type 'PackageGlueGrammarSpec | undefined' is not assignable to parameter of type 'PackageGlueGrammarSpec'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(8906,65): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(8931,38): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(8959,38): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(8960,100): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(8997,62): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(8999,58): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9052,22): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9053,175): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9073,32): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9078,9): error TS2322: Type 'string | undefined' is not assignable to type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9087,105): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9087,117): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9100,157): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9112,45): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9144,28): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9229,56): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9304,74): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9309,100): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9324,48): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9366,75): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9404,88): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9464,214): error TS2345: Argument of type 'number | undefined' is not assignable to parameter of type 'number'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9504,67): error TS18048: 'token' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9505,46): error TS18048: 'token' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9524,46): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9525,86): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9535,11): error TS18048: 'name' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9547,58): error TS18048: 'name' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9577,48): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9577,115): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9588,80): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9592,54): error TS18048: 'name' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9599,93): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9607,99): error TS2345: Argument of type 'number | undefined' is not assignable to parameter of type 'number'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9618,68): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9622,66): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9726,9): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9727,15): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9807,14): error TS2538: Type 'undefined' cannot be used as an index type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9807,35): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(9811,29): error TS2538: Type 'undefined' cannot be used as an index type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(11201,146): error TS2345: Argument of type 'PackageGlueGrammarSpec | undefined' is not assignable to parameter of type 'PackageGlueGrammarSpec'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(11202,64): error TS2345: Argument of type 'PackageGlueGrammarSpec | undefined' is not assignable to parameter of type 'PackageGlueGrammarSpec'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(11209,91): error TS18048: 'sourceGrammar' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts(11749,3): error TS2322: Type '(string | undefined)[][]' is not assignable to type 'readonly string[][]'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🧪️tests/⚡️inputs/🟦️.ts(15,28): error TS7016: Could not find a declaration file for module '../../🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🧪️tests/⚡️inputs/🟦️.ts(44,60): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(601,16): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(629,10): error TS18048: 'line' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(630,43): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1215,33): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1291,36): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1301,12): error TS18048: 'best' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1308,38): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1318,12): error TS18048: 'best' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1328,23): error TS2322: Type 'string | undefined' is not assignable to type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1346,33): error TS2322: Type 'string | undefined' is not assignable to type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1581,30): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(1583,16): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(3572,33): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(3572,52): error TS2322: Type 'string | undefined' is not assignable to type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🧪️tests/🦀️inputs/🟦️.ts(24,32): error TS7016: Could not find a declaration file for module '../../../🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🧪️tests/🦀️inputs/🟦️.ts(45,43): error TS7016: Could not find a declaration file for module '../../../../📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs' implicitly has an 'any' type.
```

