# Generation2d Receiving Owner Scope

The eighteen current bad receiving-owner attributes belong to Procedural generation2d, which is not reached by the current Puzzle physical2d, Hub-native or Hub-WASM first-party Cargo closures. No generation2d production source, attribute, dependency or generator was changed. This is a scoped source/dependency finding, not native acceptance of generation2d or Puzzle.

## Actual Manifest Trace

The retained generation2d-reachability.ts input parses the actual Cargo TOML through the registered Bun tool. It resolves each package's authored workspace inheritance and local dependency path base, reads the target package manifest, and follows regular/build dependencies plus only the selected root's dev dependencies. It also calculates a conservative superset enabling every optional dependency and every platform-specific dependency table; it does not need to assume which feature set is enabled to establish the negative result.

| Root | Ordinary path graph | All optional/platform path edges | Generation2d reached |
|---|---:|---:|---|
| physical Puzzle2d test | 723 manifests | 766 manifests | No |
| Puzzle Hub native test | 805 | 848 | No |
| Puzzle Hub production WASM | 804 | 848 | No |

All first-party dependency edges in the scanned graphs resolved through actual local paths; there were zero unresolved first-party registry-only edges. Full visited manifests and root metadata are retained in the disposable generated reachability JSON. No Cargo metadata/build/test command or compiler was launched for this investigation.

The actual roots were Puzzle2d's package manifest under artifact/2d/packages/rust and the composition Hub Puzzle package manifest under hub/compositions/puzzle/packages/rust. The Hub production graph excludes root dev dependencies; the native test graphs include their real root dev dependencies. Registry third-party crates cannot introduce these local authored source files through a missing local first-party edge in this trace.

## Rust Mount And Emission Check

The registered Python source/template audit inspected 141,098 code/template/config files under the first-party plugin/dev, framework, Hub and agent roots. Its explicit excluded output/vendor directory names are recorded in the generated metadata. After distinguishing attribute assignments from ordinary native_owned namespace references, exactly eighteen wrong assignments remain, all in generation2d. No bad receiving-owner assignment outside that artifact was found by this bounded scan.

Outside-artifact Rust references were classified rather than treated as dependencies: generated dev-catalog and layout entries are strings; most references are comments; the typed consumers belong to the separate Procedural Hub/bridge or separately owned development tests. No Rust path/include mount from the pending Puzzle closures to these generation2d source bodies was found. The direct Cargo graph also does not reach their actual owners.

The active NativeReceive parser/emitter was read directly:

- Value derive expansion container_native_owned_path uses explicit native_owned, then the authored value crate, then canonical semio_framework_value_native_owned.
- Native receiving expansion obtains that selected owner and emits native_decoding::owned paths.
- Enum helper/carrier generation propagates the selected native_path into generated attributes; it does not hardcode pack-json.
- canonical_json ownership is a separate concern and does not select the receiving owner.

Therefore a generated enum helper can repeat an already-wrong authored explicit input, but no independent generator/default/template selecting pack-json was demonstrated. The parser/emitter requires no compatibility export, fallback or repair. The invalid generation2d declarations are a separate current maintenance issue; under the parent scope instruction they remain untouched.

## Retained Evidence

Input scripts: generation2d-reachability.ts and generation2d-source-audit.py. Disposable outputs: generated/generation2d-reachability.json and generated/generation2d-source-audit.json. The eighteen exact source paths are already listed in native-compile-audit.md. These metadata files contain paths, digests/search matches and classifications, not copies of whole version-controlled source files.

Root's physical2d request 11101 retains compiler ownership as last reported by root. This lane has no native or producer request queued or running.
