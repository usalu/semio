# Admitted Worker Compiler Closure Audit

Read-only source audit after the coordinated dependency sync request joined terminal1 without running its body. No Native command, Source capture or compiler execution was launched for this audit.

## Bounds and Runtime Dependencies

The authored generated entry statically imports admission/👷️.ts plus the process routing facade. A finite TypeScript runtime-import approximation, excluding erased type imports and import.meta.vitest branches and including the actual authorizer schema RuntimeDataCompanions, reaches84files and1,068,328source bytes. Every relative path resolves and is<=256UTF16units. No nonworkspace bare runtime dependency or cycle appears; the only non-node bare runtime import is allowed bun:sqlite. The child graph excludes the parent admission harness, parent Ajv/TypeScript oracle and authorizer/compiler implementation, so it does not recursively compile its own harness.

This is static analysis, not a claim that Bun capture ran. Actual capture uses Bun.Transpiler.scan, privately held no-follow bytes, maximum1024files and16,777,216totalbytes, and4,194,304bytes per source. The observed approximate closure has substantial headroom; actual compiler evidence remains pending the owning maintenance suite.

## Confirmed Companion DATA Mismatch

The real nested worker capture has stronger DATA census semantics than the parent authorizer compiler. captureBuildSourceClosureV1 in build/📥️source/🔏️closure/🟦️.ts calls `await includeSchemas((dirname(path)==="."?"":dirname(path)+"/")+"🧬️schema")` for every nonJSON source. includeSchemas recursively enqueues all regular JSON files. requireSourceSelectionDataV1 subsequently checks every snapshot.files path/hash against the actual compiler-injected admitted parent selection.

The parent capture in caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts instead follows Bun scanner imports plus only schema-owned RuntimeDataCompanions. Its private workspace resolver admits exact first-party private package exports, excludes types conditions, rejects divergent runtime targets and never treats an arbitrary external library as a captured workspace source. No physical-hash witness can substitute for admitted parent provenance.

Reproducing only the nested capture literal-plus-recursive-schema traversal from the real worker yields24files/304,997bytes. Two sibling schemas are absent from the parent runtime-import/declared-companion census. Neither owner imports its sibling schema and neither has a RuntimeDataCompanions row:

```json
[
  {
    "owner": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/📋️data/🟨️.mjs",
    "schema": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/📋️data/🧬️schema/🔣️.json",
    "bytes": 1615
  },
  {
    "owner": "🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🟦️.ts",
    "schema": "🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🧬️schema/🔣️.json",
    "bytes": 2031
  }
]
```

The data module is reached through the worker Cargo receipt decoder; the language module is reached through Cargo resolution→operation help→public language resolver. The current parent capture therefore omits genuine nested snapshot DATA, which would cause the preserved admitted-parent guard to refuse. This is source-contract evidence; no Native failure is claimed.

The narrow proposed repair is to declare these two defining sibling schema paths as DATA companions of their real owner modules in the existing authorizer RuntimeDataCompanions schema. Do not add unused imports, fabricated Source selections or fresh physical-hash evidence. A language-neutral fixture and actual registry/capture-semantic regression are being prepared first in the existing admitted worker test suite; shared production changes remain coordinated through Root.

## Test-First Companion Repair Boundary

The closed admitted fixture schema and neutral corpus were saved first with two workerCompanions pairs. The existing Ajv and owned-validator fixture test validates those fields; the new regression invokes the actual captureNxAuthorizerSourcesV1 against the real worker and checks both declared companion DATA and captured manifest membership with independent SQLite JSON projection. It also executes the extracted actual selection DATA guard against bytes issued by real capture, rejects omission of each companion, and structurally verifies the private requireAdmitted wrapper still passes actual admittedSourcesV1. That isolated DATA-guard test creates no issued authority.

Before production, both actual registry rows were absent (schema SHA256`d652d72de969f174021a5a2932e780acd9d47261e0161a1cadcb6bc128815952`). This is a source-red observation, not an executed failure. After the test save, only the two owner→schema pairs were appended to the existing RuntimeDataCompanions.const registry. Every original row and all other schema fields were semantically preserved. The consumer already enqueues those DATA companions before import scanning; no runtime import, provenance fallback, guard, bound or Source-capture rule changed.

Source-only postrepair approximation is86files/1,071,974bytes. Test source has zero parser diagnostics. Actual Bun capture, compiler admission and all Native assertions remain pending; no test pass is inferred from parsing or registry readback.

```json
[
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔏️admission/🔣️.json",
    "utf16Units": 192,
    "bytes": 5398,
    "sha256": "27986fc5dcdc4da1f2334d6a36e25a8987546567fbc36de937ee9ca76167edf9"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔏️admission/🧫️.json",
    "utf16Units": 192,
    "bytes": 891,
    "sha256": "64d29ea2d0d55f62d3db9e3226710a3c0c55eed61f0ea7afff4b8d5ba38f95df"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔏️admission/🧪️.ts",
    "utf16Units": 190,
    "bytes": 11848,
    "sha256": "c013c3fc74daddabf65157a554a593366ea5e51cbaf4bb1668d8382e9ca8eb90",
    "parseDiagnostics": 0
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧬️schema/🔣️.json",
    "utf16Units": 148,
    "bytes": 23408,
    "sha256": "07078383288aa488eab4a83af28c42923ce9692b2a83beaa578c281645ddfea0"
  }
]
```

## Finite Parser and SDK Boundary Census

Read-only follow-up after Source0 refused at the bootstrap installed-parser witness. Ten current roots were inspected: admission harness/body/oracle, session root/credit/controller/inference tests, production session/worker, and retained release proof. All ten parse without diagnostics. No direct createRequire/require.resolve call appears in those roots.

The parent oracle modules alone import TypeScript/Ajv/bun:test for AST and independent validation. These stay in the ordinary budgeted test subprocess: process/🧪️testing/🎛️execution/🟦️.ts calls spawn(command,args,...) with the authored bun test arguments. The admission harness separately captures the tiny generated entry and actual body, compiles its genuine byte snapshot through compileNxAuthorizerSnapshotV1, checks the actual worker census and starts only compiled bytes. The real body imports Node intrinsics, bun:sqlite and first-party modules; it does not import its parent harness/oracles or third-party parser SDKs.

The actual authorizer capture resolves bare specifiers only through its captured private first-party workspace export registry and checks Bun.resolve against that exact publication. Its compiler snapshot plugin admits only captured files, its fixed external intrinsics and closed entry role, then injects actual parent Source selection. Production worker preparation calls captureBuildSourceClosureV1 plus the unchanged requireAdmittedBuildSourceClosureV1; neither the new companion DATA rows nor the fixture create an ambient dependency fallback.

Additional transitive call sites are real but do not establish another current failure: process/🔌️providers/🟦️.ts creates require from its own import.meta.url and later loads an environment-designated provider path synchronously or asynchronously; process/🛠️repository-tooling/🟦️.ts exposes Bun.resolveSync(typescript/bin/tsc) through resolveTypescriptCompilerV1; filesystem descriptor operations load the fixed Bun system builtin bun:ffi through createRequire(import.meta.url). These are separately owned provider, tooling and platform ports, not arbitrary installed parser calls in the authored fixture body. Their execution compatibility is not inferred merely from their presence in an import graph.

The retained release proof itself has no dynamic parser/SDK resolution. Its authored operation enters public withViteArtifactV1 and reads only the live release artifact plus exact build receipt. It is dispatched later through an ordinary owned Bun exec, and remains blocked with the other Native checks until the real bootstrap/provider correction and release. No second confirmed blocker was found in this finite census. No Source/test changes or Native commands were made.

## Resolve-Only Bootstrap Witness Review

The coherent owner repair removes bootstrap createRequire and selects only schema-owned InstalledParserEntries DATA: TypeScript lib/typescript.js and @iarna/toml toml.js. The helper checks plain manifest DATA through own property descriptors and exact package name/main (with optional one leading ./); it invokes no external resolver or executable parser. Nine language-neutral counterexamples/positives, independent SQLite entry selection and an actual captured-helper comparison against raw native resolution are registered. Test scratch uses fileURLToPath and remains inside issued artifacts.

Generation scope/identity/binding target, locked version, Native path containment, whole-package digest and 2048entry/1024file/32MiB limits remain in the witness. Four reviewed TypeScript files parse with zero diagnostics and paths<=145UTF16 units. No Native pass is inferred.

One concrete physical entry-kind gap was reported once to Root for owner correction: the new realpath(join(directory,declaredEntry)) is checked for package containment but not regular-file kind. An empty directory named lib/typescript.js could therefore pass this entry selection and the later directory walk, whereas the previous native resolver would refuse an empty main directory. The declared exact executable entry should be an actual regular file before the whole-package walk; missing/directory physical counterexamples should retain the refusal. This is independent source-contract evidence, not a failure observed in the genuine Source0. Correction and final coherent readback are pending.

### Entry-Kind Resolution and Final Source Review

The owner corrected the concrete gap with readInstalledBuildParserEntryPathV1. It validates the declared scalar entry, takes bigint lstat of that exact path, rejects missing/nonregular/symlink entries before realpath, then re-lstats the declared path and compares dev/ino/mode/nlink/size/mtimeNs/ctimeNs while enforcing canonical package containment. Bootstrap uses this helper and retains its Native storage path guard. Generation binding, exact locked package version, whole-package walk/hash and all existing size/count bounds remain intact.

Seven schema-first physical cases now execute the actual extracted path helper with independent SQLite kind projection: two ordinary entries, both parser directory entries, both missing entries and a symbolic entry. Directory/symlink/missing cases refuse before resolution; missing primary identity is retained. The nine closed entry DATA cases and genuine captured-helper versus raw native-resolution regression remain registered. The fileURLToPath correction and correct bootstrap URL are present. Final finite source review is clean; actual Native execution remains required.

```json
[
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🟦️.ts",
    "bytes": 17442,
    "sha256": "4963e14c9a3aa42ede7b04e0a2597f20dbf3714b3714a10c7c7d9ebb30a4c45f",
    "parseDiagnostics": 0
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/📦️parsers/🟦️.ts",
    "bytes": 3385,
    "sha256": "3edb9e6bd861bc008b128586fdb332c901244102f53cc03c4efb26c1963c377e",
    "parseDiagnostics": 0
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/📦️parsers/🧬️schema/🔣️.json",
    "bytes": 5930,
    "sha256": "3153455ec300b307a913831f9e79981d8afd36996341f4a6de63fea2175b2ce5"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/📦️parsers/🧪️tests/📥️resolution/🔣️.json",
    "bytes": 2891,
    "sha256": "09b304a777416e1d9a0335e98a56f36f9c7aae4476fd4db10b28847517556f6d"
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/📦️parsers/🧪️tests/📥️resolution/🟦️.ts",
    "bytes": 8807,
    "sha256": "b71a4a3e26e6e38ab465cc1a8b79fc5e62dd7e8f32e846aa3351dea8d1d94589",
    "parseDiagnostics": 0
  }
]
```


## Source0 96118 — Terminal And Postbaseline Authority Evidence

The owning result `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/🗑️generated/e2e-source0-witness-result.json` records natural whole 1 at 2026-10-21T19:18:23.305Z, 40,260 log bytes and zero completed test bodies. The parent retained status 1 / signal null. The primary was `Uncaptured implementation import refused`, before backend or test admission. Its importer is the closures DIRECTORY (`/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures`), with specifier `../../../../../../../🏗️build/📥️inputs/🦀️resolution/🟦️.ts`; declaringImage, declared and targetImage are all false. This records actual context, without attributing a loaded module hash or actor.

The 19:15:19.312Z before and 19:18:23.296Z after kernel records contain 864 paths each. Three defining files changed after the baseline:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts`: 850e011ac702b579ec522e1ecd497e952949debc0e86f56e4bbcda14e57479f0 (28068 bytes, mtimeNs 1792610071313294360) → c319d10068d2e36d5cf088426f1bfa0cd135fef22bea71a579e396ea4d16f9f3 (27974 bytes, mtimeNs 1792610216803673433).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔁️bootstrap/🟨️.mjs`: 9763ca95f8c08c993a6df9e356ae2cf58707e59609f2f9dba5d7cde97fa95f25 (8813 bytes, mtimeNs 1792610074129732307) → 2aed34792eebaed636e76253e9bc9ad29872731305580dc429700f3f123da830 (9593 bytes, mtimeNs 1792610301009337466).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/📦️parsers/🧪️tests/📥️resolution/🟦️.ts`: 0a16366ae1d9fe21f020a1e342110dc1a3aa26079c5c074ededddd2828009cae (8950 bytes, mtimeNs 1792609949855631808) → e190ccd8d5e264aaee279723a8a56762276d6605cb880547f5ab284a280d0154 (9078 bytes, mtimeNs 1792610195970441906).

The Source closure import refusal condition changed from `!candidate||!images.has(candidate)||!declared&&candidate!==entry` to `!candidate||!images.has(candidate)`. Removing the undeclared-edge condition changes authorization for captured targets; it is separate from formatting diagnostics. The unchanged diagnostics AST regression still asserts the earlier three-part condition for BOTH Bun loader strings, while its declaring-module and require expectations remain intact. All four diagnostics helper/schema/test/fixture hashes are unchanged in the retained snapshots. No test execution or exact evaluated Source identity is inferred from these physical observations. Original 26 controls, package/lock and Source0 target metadata remained unchanged in the owning result.

Before this report-only append, all three edited reports were checked absent from both 864-path kernel lists, every current `test-workspace-build` explicit input glob (27), and all six runtime entries. No production/test Source, Native process or guard was changed. The next admitted-parent map/edge repair belongs to the other owning High; our SourceDemand → caller-birth → publication chain remains held pending Root release.


## Canonical Admitted Resolution Map — Independent Saved Readback

Source-only review of the coherent replacement found no concrete remaining defect. The closed AdmittedResolutions DATA checks the same normalized root, exact count/index/hash parity against privately injected physical Sources, sorted unique UTF-8 paths and complete target membership. Nested capture resolves exact declaring-file edges, checks issued Original before physical reads, preserves no-follow ancestry/FD/hash checks, and includes only admitted sibling-schema/literal DATA. The two previously added owner/schema companions remain present. No caller-provided resolution map or ambient physical resolver is accepted.

Bun and Node derive both injected metadata fields from the same ordered captured rows. The strict Bun undeclared-edge guard is restored, so the earlier unchanged diagnostics guard assertion matches the current implementation. The new actual Source child regression covers nested file resolution, copied Original refusal and changed Source refusal. Runtime’s closed Node map contract adds three map/parity/physical-change cases; the actual entry uses both metadata fields, compares exact values and roots, validates both actual owned schemas, checks exact value/schema edges, and uses SQLite for ordered index/hash parity and edge counts. These are inspected tests, not executed results.

Reviewed physical boundary:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts`: 28132 bytes, SHA-256 `e300e4620f63f26a2300670a06d928d3e4e00d633c49306d6782280a348d2cad`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧬️schema/🔣️.json`: 14020 bytes, SHA-256 `8da0c538d56cf09ee455890264fca3fd003019855e15d1da6765d071ec5259bb`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🟨️.mjs`: 9015 bytes, SHA-256 `b0653019a93e3e6188f977639edf6a0cf1f33c1d5a2dc281060ec82d32e78dbc`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🕸️resolution/🟦️.ts`: 10982 bytes, SHA-256 `fc796044d94f3aca0c2e1255bc3656dba4accaac88c08b2958c8c6f154ea00ce`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🕸️resolution/🔣️.json`: 1222 bytes, SHA-256 `d73dee59478003d293975e74a00cce585c33f4fe49f94257efce71619b63590a`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/🟦️.ts`: 7336 bytes, SHA-256 `7aee8ff0a97d16bac8f9f954fef2700242b86ae1e1d4774e3d7ebb7b5bb1d57a`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/🧫️cases.json`: 35354 bytes, SHA-256 `a963687f23b925354be0867032d43cb125288a888cb6ad4e58d944697b6a7e07`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/🧬️schema/🔣️.json`: 2241 bytes, SHA-256 `78ed6d22973d26cb8a10d10971ec1cfb155a90eb6b08613e3e8920985e30d566`.

This append is outside the current 27 explicit Source0 input patterns and six runtime entries, and the retained 864-file snapshots. No production/test edit, SDK probe or Native command ran during this review. Heavy chains remain held for the owning Source0/global release.


## Raw Test Parent Versus Admitted Child — Fixture Route Audit

No raw-map admission gap was found in the registered Node adapter, admitted worker, session or inference fixture routes reviewed below. This is source evidence only; no compiler/child/Native body was executed during the review.

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/🟦️.ts`: the raw Bun test constructs a bounded snapshot and calls `buildSourceClosureNodeArgumentsV1` followed by the owned Node launcher. It does not call `captureBuildSourceClosureV1`, `requireAdmittedBuildSourceClosureV1` or `prepareClosureReadSourceV1`. Its maps are output assertions of the immutable reader adapter, not an unchecked claim of admitted parent authority.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔏️admission/🟦️.ts`: `runAdmittedClosureWorkerTestV1` receives the actual process Original, checks `requireIssuedResourceControlInvocationV1(control,original)`, and uses the real `captureNxAuthorizerSourcesV1` → `snapshotNxAuthorizerSourcesV1` → `compileNxAuthorizerSnapshotV1(...,control,"bun","program")` path. Its generated static entry imports the body and actual process receiver, and the owned child receives the projected envelope derived from that same parent control. The birth witness checks the projected owner/deadline/budget. Initial compiler admission legitimately creates the parent map; it does not require a map already injected into the raw test module.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🟦️.ts`: the actual compiler derives physical Sources and import projection from its validated private capture, then defines BOTH `import.meta.semioCapturedPhysicalSources` and `import.meta.semioCapturedSourceResolutions` in each non-JSON module before bundling. This provisions the compiled body and session module before any nested capture.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔏️admission/👷️.ts`: only this genuinely compiled body calls the actual worker session/prepare paths. `prepareClosureReadSourceV1` receives the child process Original, requires matching issued control, captures by the injected exact map, and calls the strict private admitted-selection guard before Node arguments. Publication and all five negative worker modes run here.
- Session and inference registration parents delegate to `runAdmittedClosureWorkerTestV1`; their remaining raw opaque-channel terminal proof has no nested Source capture. The admission companion test uses raw `captureNxAuthorizerSourcesV1` only for initial compiler DATA census and separately executes an isolated pure guard; it does not manufacture an Original or invoke nested capture from that raw module.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🕸️resolution/🟦️.ts`: the actual nested-resolution law starts with `captureCompilerBuildSourceClosureV1` under genuine Original/control, then launches the immutable Bun stream loader. That loader injects the maps before the compiled entry calls nested capture and tests copied Original/changed Source refusals.

Accordingly, successful future Node map/parity laws alone will not be reported as real worker admission acceptance. The compiled admitted-worker and nested-resolution laws still must execute through their registered Native targets. No ambient physical Source fallback, unchecked map or fake Original was introduced.


## Converted Bazel Implementation Tests — Independent Route Review

Reviewed current `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🔏️implementation/🟦️.ts` and adjacent neutral fixture, plus the actual Bazel identity function and compiler/loader owners. TypeScript parse diagnostics: zero. All four original named test registrations are unconditional; no artifact-missing early return remains. The first three obtain mandatory artifacts through the real test Original. The fourth remains a direct schema/AST/SQLite law without nested Source admission.

`admittedBazelFixtureV1` authors a small driver that statically imports the subject, Bazel identity owner and actual process receiver. The raw parent legitimately calls initial `captureCompilerBuildSourceClosureV1` with an issued matching control; the returned snapshot is sent through the exact auxiliary immutable Bun stream loader under that control and its projected process environment. Only the admitted child calls `captureBuildSourceClosureV1` and `bazelImplementationIdentityV1` under the received Original. The subject static import and existing schema/literal DATA census put the fixture modules and JSON resources into that parent selection. No raw nested-capture fallback exists in the parsed test module.

The two esbuild calls, SQLite hash mutation comparison, exact captured-file roster, real immutable loader output/refusals, and planner/result Ajv plus owned-validator comparisons remain in executed test callback bodies. They were inspected, not run. Late Source edits, uncaptured dynamic import, explicit recheck and module identity refusals are still asserted; the entry bytes remain unchanged during the identity mutation law. Owned loader launches retain actual compiler control and matching resource environment.

One concrete cleanup caveat was relayed to Root: tests one and three perform unconditional finally cleanup (`db.close(); await rm(...)` or `await rm(...)`), so cleanup failure can replace a prior semantic/child failure. No prior attribution is claimed. This review made no Source/test or Native change; actual four-body execution remains pending.


## Independent Runtime-DATA Discovery Review

The current authorizer uses cumulative directory/result/entry/declared-byte state across its closure, bounded opendir buffer1 rather than whole-directory readdir allocation, explicit path limits and no-follow ancestor/leaf observations, complete directory stamps, and the existing bounded Source-byte capture. Its closed policy retains1024 directories/entries/results,16MiB cumulative declared DATA and depth32. Twelve schema-first portable scenarios cover missing, symbolic link, path overflow, entry/depth/byte bounds, directory race, nonregular kind, literal-link refusal, primary undefined plus close failure, and cancellation. A genuine compiler fixture compares discovered paths with SQLite, checks no executable import edges for DATA and rejects malformed captured JSON. No actual Native test or producer was run by this review.

One precise stability gap was relayed to the coordinator: the final directory ancestry/stamp validation precedes await stream.close(); after successful close only cancellation is checked. A directory stamp changed during that await can therefore escape the final census check. A portable close-mutation scenario should require refusal after clean close while preserving an earlier primary, including undefined. Final stability signoff is held pending the owning implementation decision; no Source was edited by this reviewer.


## Authorizer DATA Discovery — Post-Close Independent Review

The saved owner `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts` now checks `if(failed)throw primary` before the final `await discoveryDirectoryHoldsV1(ancestry,signal); signal.throwIfAborted()` after joining directory close. A prior body failure, including thrown undefined, is tracked with the explicit failed flag and wins over close failure/cancellation. Clean-close cancellation is checked before the final holds pass; body/close failure never reaches that pass. The final holds check rejects namespace mutation occurring during an otherwise successful close.

The adjacent `🧪️tests/📥️data/{🔣️.json,🟦️.ts}` and owner schema `$defs.RuntimeDataDiscoveryCase` consistently retain all prior twelve scenarios and add `close-mutation` (thirteen total). Its actual-function port marks the directory closed, then changes its lstat stamp only in that scenario. Existing SQLite acceptance projection rejects it; cleanup counts still require every opened stream to close. The old undefined-primary/close and cancellation identity assertions remain. Owner/test TypeScript parse diagnostics are zero. No test execution, Source edit or Native command accompanied this review.


## Post-Close Discovery Gap Resolved

A fresh final four-file read verifies the saved clean-path post-close directory/stamp recheck. Earlier discovery failures, including undefined, are rethrown before that check; clean close cancellation remains checked before the final holds. The final neutral schema/fixture/test consistently mount bounded-data-close-mutation as the13thcase. Its close port records the directory, its lstat port changes the stamp only afterward, and the actual function must refuse. All original12cases, SQLite oracle, actual capture fixture and authorizer test-root import remain present. Both independent reviews accepted this current Source boundary; owner/test TypeScript parsing reported zero syntax errors. Native runtime assertions remain pending.

The implementation and case landed concurrently with the delegated narrow work. Fresh-read reconciliation preserved the completed production change and removed only a redundant extra draft while both cases existed; the final sole working close-mutation case is preserved. No provenance is inferred from earlier naming observations. Current owner16743bytes SHA2566f27386b6fb65b97d66b8aa25f9aa6f7f525aa38a842c39f3ef85b2722a94e71; schema29156bytes SHA256a52bdcf6656a2c3c648b31cf1f9a2ad5e24b3b8987c63f2eba91cdf6a07f6966; cases2122bytes SHA256b133cb9ca54aefbc53e097ccecd8d12ac7ee69219c0a369b6577d576997fd2de; test6703bytes SHA256ac684fa3a0cac64b8c86ae89b055052705894f988d0274d5f8a1dd517f7d1415. Limits, interfaces and ordinary error/custody checks remain unchanged. All four current boundary paths are retained in the closure inventory and Source is frozen.


## Generation Fragment Inflation Cap — Test-First Correction

Current-state audit observed Source-closure schema SHA-256 `8da0c538d56cf09ee455890264fca3fd003019855e15d1da6765d071ec5259bb`, 14,020 bytes. Its closed snapshot/parent DTOs and private binding/strict import checks retain 2048 files, 4 MiB/file, 16 MiB Source, 24 MiB snapshot and 256-unit paths. Current Node channel stays 16 MiB; initial cohort admission now counts actual transport fragments conservatively. No old Source-schema bytes were retained, so no exact historical semantic delta, loaded identity or actor is claimed.

The intrinsic fragment bug was independently traced through packed descriptor checks, remaining-proof debit, decompression, parser and producer: the worker allowed 256 MiB inflation for generation fragments, while the genuine generation producer/readFragment/readGlobal cap them at 64 MiB. Shape validation after JSON.parse cannot impose that byte bound. Schema and eleven neutral role/remaining vectors were saved first. The existing named compressed-role test now executes actual payload extraction with a decompressor-limit spy, preserves the exact sentinel primary, and compares observed maxOutputLength to independent SQLite min(actual producer/generic policy, remaining). It allocates no expanded document. All previous seven real inflation-refusal cases remain.

The corrected supplemental Node v24.15.0 / TypeScript 5.9.3 / SQLite 3.53.4 probe ran through `repo --tool=nx --lang=en capacity run --memory=512 --cpu=1`, heap 256 MiB. Before production change, actual red exit 1 examined all eleven vectors and failed only fragment-plus-one: expected 67,108,864, observed 67,108,865. A single worker expression now chooses GENERATION_POLICY_V1.maximumFragmentBytes only for generation-fragment before the existing min with remaining budget. Index/frontier 512 MiB, generic 256 MiB, all other limits and the typed generation scanner are unchanged. The same probe green exited 0, eleven cases passed, failures empty, five pre/post input hashes stable. Generated `fragment-inflation-cap-{red,green}.{json,log}` retains results. Two earlier probe-port omissions (resolve then sep) are separately retained as `fragment-inflation-cap-probe-port-defect[ -r1].{json,log}`; they were isolated harness failures, not production evidence.

No new named test body was added: `all compressed closure roles retain real inflation refusal and bounded transport context` is still imported by the existing closure root and owning maintenance target. Actual Bun/Native test execution remains pending; the pure-port probe does not replace it. Worker and test parse diagnostics are zero. The typed scanner remains byte-identical SHA-256 `47de492bc591d3b3216f59b85e7c18aa9a1ad44e6fd537937579354b028a3b17`.

Frozen changed boundary (all absolute paths <=192 UTF-16 units):

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/👷️read.mjs`: 30429 bytes; SHA-256 `8ae833aea06f1a69c6544afc8b732ceeb8d07c869dbee4eb3215446dcb9bc778`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧬️schema/🔣️.json`: 40766 bytes; SHA-256 `f3a6e26bf8ac1670e78605d2e950c8fa74936597aa704edb641c0f51e616132d`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📥️inference/🔣️.json`: 4638 bytes; SHA-256 `0474e1a45c9314ebe305cb3e59b3ab01e0ffddea3ae5c455e6658894c11c9129`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📥️inference/🟦️.ts`: 12629 bytes; SHA-256 `db9630ddbd88eef057367fa05319748bc096fe52734ea3f6ea7a318b59b37cb4`.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/fragment-inflation-cap-probe.mjs`: 5140 bytes; SHA-256 `d77dbc1b227fffc7dedbc710e7a07c8340278ff7bca5640365914e7b51b9f2c3`.

A separate pre-existing transport test mismatch was relayed without editing it: the closure root AST oracle expects object-literal write arguments while current fragmented session loops write identifier frame. Runtime/Root coordinate that test-only repair.


## Root Transport AST — Canonical Binary Frame Alignment

Schema/fixture first extended the existing transport contract with exact canonical helper/loop bindings and five language-neutral frame vectors: empty input, 65,537-byte input, 4 MiB exact, 4 MiB + 1, and a three-byte Unicode scalar split across the binary boundary. Existing request 65,535/65,536/65,537 boundaries and all prior fixtures remain. No production limit or session code changed.

The existing named body `shared graph closure reader uses bounded owned stdin across platform command limits` now checks the actual closureReadInputFramesV1 call arguments for captured.bootstrap and packet under the real I/S policies, each frame iterator’s collection/channel binding, and the separately bounded credit/null text controls. It executes the actual pure frame helper, validates each closed Binary DTO against the actual owned input schema, requires canonical base64 and frozen frames, rejoins exact UTF-8 bytes and compares byte/frame counts with independent SQLite arithmetic. The helper returns binary frames even across an interior Unicode byte boundary; no text loss or cap raise is accepted.

Static source-oracle readback: root test parses with zero diagnostics, exactly one framing declaration after renaming the old worker-entry variable, actual two collections are bootstrapFrames/requestFrames, all four semantic write sites are accounted, and the neutral transport object equals the actual schema const. This is source validation; the full Bun named body is still unrun. No extra supplemental probe was needed. Admission/🧪️.ts had no stale direct-write assertion, so it remained unchanged. Runtime’s production/session and three test files were preserved.

Final shared-schema/test boundary:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧬️schema/🔣️.json`: 42364 bytes; SHA-256 `c440632001fe5e943b3f8e148eb94715adda54169b4aaa0c991f245480a985b1`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🧫️cases.json`: 10613 bytes; SHA-256 `fe13534d1ef3b174e1a12efccb1596386987c6a2e1ddab757ee154a7ff4e5122`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🟦️.ts`: 27864 bytes; SHA-256 `5eaa0df7e978e17fd3047ac33d30caf7496d658cf507e5671e0893e13977fef0`.

The new framing contract preserves the earlier eleven fragment-inflation vectors and single role-specific worker cap. No new named body, runtime entry, target or external dependency was introduced. All own Source/test edits are frozen pending the global Source0 release.


## Source0 71282 — Readiness Startup Audit

Root relayed the owning Source0 actual whole 1 before test bodies: `Exact captured closure worker readiness required`, worker status 1 / signal null, with 949 selected inputs, 26 controls and package/lock/metadata observations stable. This is a pretest child-startup refusal, not a completed assertion. All our Native chains remain held; no unchanged retry was launched.

A finite read-only static-import census from the current worker found 23 runtime/JSON modules totaling 309,554 bytes. This is physical Source analysis, not an admitted snapshot or evaluated-identity claim. No bare external dependency, revision/self-FD module, Bun-only startup call or role-decoder auto-execution was found. Top-level work comprises constants/policies and the inventory factory. All ten JSON imports across the worker, Cargo resolution, help/options and process-language path explicitly use a JSON type attribute. The immutable Node loader still verifies the exact bootstrap identity, closed manifest, bounded regular paths/module hashes and complete import-target membership, then imports the worker before publishing readiness.

No concrete non-framing startup defect emerged from this review. Runtime owns current bootstrap/binary-ready tracing; the other High owns actual stderr provenance. No Source edit, child/probe or Native command accompanied this audit. The static path/byte census does not replace actual captured-parent map or startup acceptance.


## Source0 71282 Readiness Audit

The current bootstrap → canonical binary frames → owned channel stdin → immutable Node loader → READY path has no source-proven framing mismatch in this bounded review. The channel decodes original binary bytes and joins stdin publication; the adapter reads exactly the bootstrap LF, validates manifest identity/paths/import targets, imports the captured export-only worker, then emits READY. Requests wait for READY. This is source review, not execution acceptance.

The actual Source0 result is whole 1 before bodies, with worker status 1 and null signal. Its 40,118-byte log contains only the generic readiness failure and its s0d3 artifact directory is empty. The underlying startup refusal cannot be attributed from retained DATA. The current parent has a concrete diagnostic gap: readiness mismatch records disposal status, then `if(failed)throw primary` bypasses the later captured stderr projection. Bounded channel diagnostics are available inside that mismatch but are not attached. This explains the loss of diagnostic detail, not the startup failure itself.

No Source/test edits, new invocation, raw cache reads or process actions occurred. Unit → catalog → canonical checks remain held for explicit Source0 backend/capacity release.


## Raw DTO Startup Diagnostic — Actual Node 24.15

The single qualified repo capacity run reserved 512 MiB and joined whole 0. The retained ticket input worker-startup-raw-dto-probe.mjs is 6,193 bytes, SHA-256 d0f42bb999c6ec26cc342f615c544b8632e1404ca3e8b73bbdda8ab7558cf748. It dynamically reads current implementation bytes and constructs a source-only literal-import fixture; it does not copy checked-in Source into this ticket.

Actual pinned executable /opt/homebrew/Cellar/node@24/24.15.0/bin/node reported v24.15.0. Parent and child heap limits were 256 MiB. The fixture included 23 files / 318,089 source bytes; the actual exported argv builder produced 8,327 Windows command units and the actual canonical binary helper produced one 434,987-byte bootstrap frame. Exact READY was observed for digest 094b3d777aef7cf348c68d016b1721c2d3c26ad1b737b80408459d95874a00c1. Explicit null plus EOF joined child status 0 / null signal. Stderr was empty, untruncated; all 27 pre/post input hashes remained equal. Full mapping and hashes are in generated/worker-startup-raw-dto.json; complete capacity output is generated/worker-startup-raw-dto.log.

This is only a raw DTO loader/module-startup diagnostic. It supplies no captured-parent Source provenance, Original, immutable capture admission, physical cache proof or Native acceptance. Production selects literal node under its own Native PATH; this pinned-executable result does not establish that executable identity or equality with the actual failed Source0 snapshot. No Source/test changes or Native invocation occurred.

Production executable boundary: captureOwnedCommandLiveV1 passes the literal command to child_process.spawn with shell:false. Its issued resource environment helper preserves the supplied environment/PATH and replaces only the serialized Original envelope. No node resolution/authentication occurs in this channel layer. The failed invocation has no retained PATH/executable witness, so no actual production-node identity is inferred.


## Captured-Parent Node Bootstrap Serialization Boundary

A current source-level defect is visible in the representation boundary: the captured Bun loader applies compiler.transformSync followed by redirect to the entire Nodeadapter module. redirect rewrites lexical imports in nested functions, including node:module, node:fs and node:fs/promises inside capturedNodeMainV1, to Bun-only virtual aliases. The adapter subsequently serializes the evaluated function with capturedNodeMainV1.toString(). Therefore a captured parent can emit child Node code containing its parent-specific aliases before bootstrap/READY. This is an exact current call-graph/source finding; the failed process has no retained stderr witness yet, so its precise evaluated bytes are not claimed. The physical/pinned raw DTO probe bypasses the captured-parent rewrite and cannot disprove this boundary defect.

A hermetic child-source literal, parsed and validated as Node source before descriptor construction, avoids post-transform function serialization. Required regressions must exercise the emitted source from an actual captured Bun parent, in addition to independent Node builtin/forbidden-alias source oracles. No production patch or extra invocation was made in this audit.


## Bun To Node Bootstrap Serialization — Independent Source Mechanism

A subsequent exact source trace identified a concrete cross-runtime serialization hazard beyond the earlier static-import census. The current Node adapter defines capturedNodeMainV1 with dynamic imports of node:module, node:fs and node:fs/promises, then constructs the Node eval text using capturedNodeMainV1.toString(). The immutable Bun loader applies its builtin redirect after transpilation to the whole loaded module. Its scanner explicitly rewrites those dynamic import literals to ./.semio-captured-module, ./.semio-captured-fs and ./.semio-captured-fs-promises, including imports nested inside the serialized function.

Consequently, when the adapter is evaluated through that Bun capture route, the later function serialization carries Bun-only virtual aliases into the Node bootstrap. The first node:module import occurs before registerHooks installation; the Node eval cannot resolve the Bun virtual namespace. A raw physical adapter probe bypasses this transformation and therefore cannot establish the captured-Bun route. This is a source-level causal mechanism, not a claim that this audit observed the actual evaluated argv or exact stderr of Source0 71282. The owning diagnostic/runtime evidence is still required.

Root delegated a schema/test-first verbatim-bootstrap repair to Runtime High; no Bun resolver guard weakening, concatenated hidden imports, physical Source fallback or ambient alias was proposed. This reviewer made no production/test edit or additional probe. The existing fourteen Node fixture cases and both map projections must remain, alongside actual captured-Bun-to-Node coverage.


## Root Causal Parent Transformation Trace

Root and both execution Highs independently traced the exact current source path after Source071282: Bun onLoad applies compiler.transformSync to each entire captured module, then redirect(transformed). That redirect maps literal dynamic node:module/node:fs/node:fs/promises imports to the Bun-only virtual aliases ./.semio-captured-module, ./.semio-captured-fs and ./.semio-captured-fs-promises, without excluding nested function bodies. All three imports occur inside capturedNodeMainV1. Only after module evaluation does the Node adapter construct buildSourceClosureNodeLoaderV1 from capturedNodeMainV1.toString(). The resulting child code therefore inherits parent-only aliases before Node registerHooks or readiness. This is exact call-graph/source evidence; the failed managed process did not retain its evaluated argv or stderr, so no runtime identity is retroactively claimed.

The physical raw DTO probe bypassed that Bun parent transformation and passed with pinned Node24.15, which cannot refute this path. Runtime High owns the minimal static verbatim Node bootstrap representation plus schema-neutral emitted-builtin/no-alias and genuine captured-parent regression; no parent loader guard, Original, import authority or size cap will be relaxed. Acceptance High owns a finite actual-Bun compiler/redirect red/green pure transformation probe, separately scoped from actual capture/Original admission. Other owning High independently retains bounded readiness diagnostics. All required Native/application checks remain pending.


## Actual Observed-Parent Bun Causal Red And Green

The retained ticket-only `node-bootstrap-causal-probe.mjs` ran via qualified `repo ... capacity run --memory=512 --cpu=1 -- bun`, requiring the actual failed-parent compiler version Bun 1.3.14 before transformation and reporting `/Users/ueli/.bun/bin/bun`. This is deliberately the observed Source0 compiler, not an assertion about the authored future Bun tool pin. It reads current Source, executes the actual sourceClosureLoaderV1 generator with actual policy/diagnostic helper, extracts that generated loader’s pure redirect through TypeScript AST, and uses real Bun.Transpiler target bun on the actual adapter. It evaluates the exact transformed function/exported initializer before inspecting the exported Node eval. No module bytes were copied into a ticket Source file, and no Original, admitted capture, Node child, SDK provisioning or Native producer was invoked.

Actual red naturally exited 1 on the single no-virtual-alias assertion. The evaluated loader was 9287 bytes, SHA-256 `96038b57598398f6585c969e88dc786998e85adf9d953e2d4c8a1e0156a0aa43`, and contained precisely ./.semio-captured-module, ./.semio-captured-fs and ./.semio-captured-fs-promises in startup imports. All five pre/post input hashes were stable. The initial data-URL and virtual-plugin evaluator attempts failed before this assertion and remain separately retained as `node-bootstrap-causal-evaluator-defect[-r1].{json,log}`; they are probe infrastructure refusals, not the causal red.

After Runtime saved its test-first verbatim-bootstrap correction, actual green naturally exited 0: 7757 bytes, SHA-256 `6026e5d515fa270a09b3f4ad5c2eb9ea9345b47e1b564cc9df24da2e8859eaaa`, all six real node: imports and no virtual aliases or failures, with five stable pre/post inputs. The probe’s initializer evaluator was adjusted to accept the actual literal declaration after the representation change; it does not implement a production fallback. Full detached evidence is `🗑️generated/node-bootstrap-causal-{red,green}.{json,log}`.

Independent production readback found the interpolation-free String.raw literal’s embedded function body byte-identical to the previous body; function-after-parent-transform serialization alone was removed. All fourteen Node fixture cases/map projections remain. The existing captured-entry case now uses a genuine authorizer-admitted program, then the actual immutable Source stream loader over owned auxiliary fd3, before returning the descriptor to the actual Node case. Review caught and Runtime corrected the generated runOwnedCommand call from four to its actual six arguments (explicit label, 60000 ms, same-control options). No other concrete issue remained in that reviewed route. These are source and pure-transform results; the registered actual Original/Node fixture and Source0 are still pending.


## Normative Tuple Validation Contract

Root verified the primary [JSON Schema Draft-07 Validation, sections6.4.1–6.4.2](https://json-schema.org/draft-07/draft-handrews-json-schema-validation-01#rfc.section.6.4.1). An array-valued items declaration applies schemas by matching position; additionalItems constrains only tuple overflow, is ignored for homogeneous or absent items, and defaults to an empty schema. This supports an owner-level validator correction rather than weakening SnapshotManifest. Runtime found Rust already implements this contract and consumes the shared neutral corpus; TypeScript compiled dispatch is the inconsistent implementation. Existing shared corpus will be extended before the TypeScript repair and compared against Ajv plus the real v2 producer. No quotation or alternate legacy format is introduced.


## Actual Source0 Manifest Failure And Tuple Validator Repair

Source0 70115 passed actual worker READY and advanced eleven graph-closure-proof events before whole 1, with zero test bodies: the worker failure frame named Closed graph record manifest required. Current encoder emits ordinary v2 manifests with positional project/name+digest pairs. The normative SnapshotManifest already expresses those as Draft-07 tuple items; the TypeScript owned validator instead prepared that tuple array as one schema, yielding schema-must-be-object on each field. No mounted cache payload was read to establish the mismatch.

Schema-first shared corpus adds 22 valid-schema cases for positional and nested/ref tuples, prefix/default/true/false/schema additionalItems, homogeneous/absent-items ignore behavior, boolean schemas, minItems and nonarray applicability. All old 62 cases and five comparisons remain. Rust already implements these rules and its two native corpus laws include this same JSON; no Rust code change was needed. TypeScript now prepares positional plans separately and applies additionalItems only to tuple overflow, preserving the existing homogeneous path and all worker/hash/link/physical/Source policies.

The retained validator-tuple-probe.mjs imports the actual current TypeScript implementation on pinned Node24.15 and the actual encodeGraphRecordsV1 producer, with independent Ajv. Qualified repo capacity512 / heap256 RED joined 1: eight valid tuple cases and the real two-project/4-record/590-byte manifest falsely refused. GREEN joined 0: all 84 cases, five comparisons and the same producer manifest agree with Ajv; seven pre/post inputs stable. Logs/results are generated/validator-tuple-{red,green}.{log,json}; exact frozen identities are generated/validator-tuple-boundary.json. This is pure helper/producer validation, not Native/Original/worker or Rust acceptance.

The correct pending owning Native target is @semio-tech/schema-validator-rs:test: its router first executes the shared TypeScript/Bun+Ajv and esbuild/Node corpus, then admitted Cargo semio-framework-schema-validator whose native tests include the same cases. The TypeScript test-shape target covers shape guards only and does not substitute for this corpus gate. No Native duplicate was launched.


## Canonical V2 Snapshot Fixture Correction

Source0 70115 reached actual worker readiness, then failed before test bodies at the closed graph-record manifest validator. The tuple validator defect and its independently corroborated repair are recorded above. Separately, the root physical closure fixture still published plain v1 graph JSON under the current snapshot namespace; its reference-change scenarios also wrote noncanonical snapshot DTOs. This fixture debt is now corrected without accepting legacy manifests.

A closed neutral snapshot contract was saved in the existing schema and fixture before updating the test. Every snapshot is now emitted by actual encodeGraphRecordsV1 using the current graph record policy: gzip records retain their compressed SHA filenames, manifest bytes retain their SHA identity, Ajv checks the normative v2 manifest, and actual decodeGraphRecordsV1 round-trips the source graph. The nonempty project tuple exercises the repaired tuple seam. Both reference-change mutations use valid producer output and reuse existing record bytes so their intended publication change is isolated.

All 21 physical scenario names and existing named bodies remain. Source, lock, inference, generation and old graph-JSON roles remain unchanged. The exact retiredFiles=1 assertion remains: coarse snapshot recovery retires the manifest while record garbage stays. No worker, validator or Node bootstrap production was edited by this correction. Current source parsing reports zero diagnostics, the two imported owner paths exist, the neutral schema/fixture contract is identical, and all paths are below 256 UTF-16 units. Low source review accepted the coherent boundary; actual Native execution is still pending.

Frozen identities:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧬️schema/🔣️.json`: 51057 bytes, 180 UTF-16 units, SHA256 `78c82c9e35fd90aebcce2660207b3cf1d2a929536002d67624724c5bb3f3f9d1`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🧫️cases.json`: 10791 bytes, 184 UTF-16 units, SHA256 `d2987f57377221c39fd05b0f0939856009a50a503506684d3319e75ec9f01309`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🟦️.ts`: 29326 bytes, 177 UTF-16 units, SHA256 `a2b726ecac7ba7787ab4d970d455a1d9b16c4a9d0efa92bf3ebed3a0204eba76`.


## SourceDemand Actual Terminal — 2026-10-21

The authorized current invocation used explicit --tool=nx --lang=en, prototype mode, --parallel=1 --workers=off, trace authority and ticket-scoped artifacts. Native executor handle 70455 joined naturally with exit 1. Normal capacity admitted one job with 3584 MiB reserved; the process completed parser/current-generation inspection, passed the graph-image queue and ran closure proof. It did not reach snapshot/backend admission or any test body.

The retained worker diagnostic identifies an actual persisted snapshot `snapshots/2d61db03258b7af9601a3b5896c190710d0579138ea3123de459355e2c1eadd9.json`: 124213751 packed bytes, manifestVersion 1, manifestProjects null, manifestDependencies 873, check snapshot-manifest. The strict current reader refused it with Closed graph record manifest required; child status 1, signal null. This is direct worker diagnostic evidence of a stale v1 publication, distinct from the separately repaired owned tuple validator and v2 test fixtures. No mounted payload was manually opened and no legacy acceptance was introduced.

Complete direct log: `🗑️generated/source-v2-ready-20261021T211528.log`,85788 bytes,SHA256 `81a837a734345b25dd205eaef7c9a4751aa1936de27833f5ddb411995d16702e`. Matching artifacts are in `🗑️generated/source-v2-ready-20261021T211528`. Caller-birth and publication were not dispatched because the required predecessor did not pass. No retry, Source edit or foreign process action followed. Root and Runtime have the exact refusal for provider-owned resolution.


## Existing Recovery Interface Audit For The Rejected Snapshot

Read-only Source inspection after the actual version-1 refusal found no existing exact public invalid-snapshot repair command that avoids the same closure preflight. This is an interface finding, not evidence that the persisted bytes came from a historical producer. No mounted payload was read or changed.

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🟦️.ts`: exported recoverBuildGraphImageHeadroomV1 acquires genuine image maintenance; HeldV1 invokes retireBuildGraphClosureHeadroomV1 before its later SHA-authenticated snapshot inventory/removal. The later manifest retirement therefore cannot cure a row that aborts complete proof first.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts`: captureBuildGraphClosureProofV1 reads every required-role snapshot through the strict worker; retireBuildGraphClosureHeadroomV1 captures before retirement. Optional-group invalidation also requires a live private complete proof and does not convert required snapshots into optional members.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts`: normal withRepositoryBuildGraphArtifactV1 owns encoded-v2 snapshot publication and private retainSnapshotBudgetV1. The latter can authenticate and retire older manifest bytes under snapshot resource writers, but executes within ordinary graph-store admission and only for retention pressure. It is not a public unconditional stale-publication repair operation. Its existing earlier-format reference helper is not a safe diagnostic/repair command and was not invoked.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📜️script.ts`: runInactiveNxCacheResetV1 is explicitly graphless but hard-bound to the inactive empty nx capsule. Its contract cannot authorize graph partition publication removal.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts`: CurrentMaintenance/Artifact/NativePayload commands are scoped to recorded Nx pool or namespace allocation/image journals, with exact bounded payload proofs; none names the graph snapshot namespace.
- Caching cache-prune orchestration operates Cargo/compiler areas and test evidence under its own policy. It is not graph publication recovery.

No recovery, reset, raw removal, capacity override, Source change or successor test command was executed. Root received the circularity and closest existing producer authority so the common provider owner can choose the actual repair. SourceDemand, caller-birth, publication and later release gates remain unpassed.


## Current Snapshot Producer And Reader Map

Three bounded source searches ran through qualified repo capacity128 and joined0, retaining snapshot-source-map.log, snapshot-callsite-map.log and snapshot-environment-map.log. Searches covered authored repository/process sources and current workspace TS/MJS/Rust/JSON/Python/Go caller tokens, explicitly excluding dependency/cache/generated/build trees. No mounted payload was read. This establishes current authored call paths, not the identity or bytes of an older running producer.

The one direct production publisher located is budget/graph/withRepositoryBuildGraphArtifactV1: it validates readBuildGraphV1, unconditionally calls encodeGraphRecordsV1 with records.version2, and publishes encoded.manifestBytes at snapshots/<digest>.json plus each <recorddigest>.gz via exclusive build-graph-store custody, temporary exclusive creation, sync and rename. Canonical command runtime (both selected Nx and Bazel), bootstrap Nx fallback and the existing profiled script all call this same publisher; the public storage facade only reexports it. No current whole-graph-v1 publisher to that directory was found. This does not attribute the actual version1 artifact, its age, or any still-running captured implementation.

Canonical withRepositoryBuildGraphPublicationV1 validates the issued pin coordinate/digest, retains real graph-image consumer and exact snapshot reader custody, verifies stable descriptor/path/parent identity and bytes, then decodes v2 manifest and SHA-authenticated gzip records with the same policy. Nx private graph-pin consumers delegate to that API. Closure role() classifies all snapshots/<64hex>.json as required snapshot and all matching .gz as snapshot-record; its worker requires current v2, full links and content identity. The private retention helper manifestRecordsV1 has an explicit non-v2/>16MiB no-record result for eviction bookkeeping; it is not a current v1 publisher or a usable v1 graph decoder. Existing isolated tests either emit current v2, intentionally negative old-whole-graph cases, or opaque physical headroom fixtures in separate private test images; none is evidence for this actual production artifact.

A current supported reader mismatch was found in ✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🕸️artifact-graph/🟦️.ts readStdioArtifactBuildGraphPin: it authenticates a pinned file then calls readBuildGraphV1(JSON.parse(source)), requiring a whole graphv1 although current publisher emits v2. This cannot emit the bad artifact, but it will reject fresh canonical pins. Root assigned a separate schema-first correction through the existing canonical pinned publication reader and actual issued control, without fallback or raw mounted-cache access. No source changes were made during this audit.


## Existing Snapshot-Root Retirement Safety Contract

The existing coarse headroom implementation already retires exact snapshot manifest roots without deleting record/source/lock payloads: under genuine kernel-EX image custody it inventories only snapshots/<64-hex>.json, validates bounded same-image regular single-link leaves, authenticates SHA256 against the filename and rechecks physical identity before unlink. Its current ordering reaches this phase only after complete proof; this audit does not authorize moving it or execute any deletion.

Normal producer custody in budget/graph/🟦️.ts holds build-graph-store:<image.root> EX while publishing/retaining immutable manifests. Snapshot consumers in build/graph/🟦️.ts retain the actual Graph Store reader plus build-graph:<full snapshot path> SH across no-follow descriptor reads, hash validation, record decoding, consumer callback and final identity checks. Whole-image EX excludes currently retained image consumers; per-snapshot EX mirrors existing normal budget retirement. Canonical and selected publication heads are separate whole-graph JSON with authority.graph binding their raw-byte SHA. They are not encoded snapshot-digest heads. Environment PATH+DIGEST is the explicit immutable pin ingress and validates the exact snapshots location and digest under the retained reader.

These fences cover current admitted consumers. They do not establish that no process holds a future, unopened PATH+DIGEST value: no separate pending-pin registry was found. Existing normal retention likewise reasons about actual held readers. A future pin whose bytes were retired must still fail strict missing-byte admission, never resolve a fallback. A provider-owned invalid-root recovery can preserve every referenced gzip record, Source and lock byte and require a fresh complete strict proof afterward; this audit gives no basis to delete those referenced payloads or relax current formats.

During the read-only audit a new store/format owner was observed with mtime 2026-10-21T21:35:11.388Z, refusing unmarked/prior-format stores and naming repo:cache-reset-graph. It belongs to the concurrent provider repair; no assertion of its final coherence or successful recovery is made here. Findings and the current-reader versus future-pin distinction were relayed to Root before execution.


## Root Review Of Canonical Pin Reader Authority

Root read current withRepositoryBuildGraphPublicationV1 in repo-lib/build/graph. It validates the existing process.env path/digest pin, requires the exact store.root/snapshots/<digest>.json path, retains private Store/content custody, verifies the manifest digest, decodes actual v2 records and passes the genuine actual control through its callback with final rechecks. It has no workspace graph recomputation fallback. Runtime’s stdio reader delegation can therefore preserve the outer immutable pin while replacing only the incorrect raw-v1 parse. The optional test environment override can be removed without replacing the actual pin authority. Source-only review; no reader/Native acceptance claimed.


## Pin Reader Save Boundary And Remaining Validation

Root independently reread all seven files in generated/stdio-pin-v2-boundary.json, captured at2026-10-21T21:46:38.987Z: 7/7 current hashes match that frozen boundary. Low accepted canonical immutable pin custody and all thirteen neutral cases. Exact named-input ownership entries and explicit public export are included. Optional pure source oracle has not run because its owner is awaiting an analysis-only read tool; no probe/Native acceptance is claimed. The actual existing stdio-js:package-graph target is required after common recovery and exercises this path under actual Original. Root preserved the fresh close inventory and all older entries; now295 unique paths.


## Uncompleted Optional Analysis Read

Runtime’s own orchestration cell523 remained pending while only reading prefixes of two retained probe logs and process.execPath. It contained no Native launch, mutation, issued control or child handle. After prolonged waits Runtime terminated only that own read-only orchestration cell; it produced no evidence and no probe pass is claimed. The nested analysis kernel was not reset, no other agent/process was stopped, and normal Native execution ownership remains available. The optional13-case source oracle was not launched and does not gate common recovery; actual stdio-js:package-graph remains mandatory.


## Recovery Review Boundary — 2026-10-21T22:03Z

The coordinating checkpoint reports 78 required Source0 names (the previous74 retained plus three format laws and one closed reset-profile law),27 explicit inputs and six entries. No reset or successful Source0 is recorded. Native review is correcting projected quarantine path length, a post-yield directory recheck and the positive interrupted-retirement fixture: genuine factory/capture/close/interruption/resume with authenticated payload/evidence stages and canonical evidence whitelist is required. Our thirteen obligations stay pending and all own Source is frozen. No foreign agent/process action, memory-floor/grant change, raw image deletion or unchanged retry occurred.


## Coherent Provider Recovery Core Review

Read the shared oct21-graph-format-gate.md coherent boundary and current format gate, Store reopen helper, reset API/driver/schema, worker marker branch and five registered core test bodies. This is source review only; Native durable continuation remains a separate pending review and no reset or test was executed here.

The public reset API accepts only graph or nx, derives all paths from actual Original capabilities and declared policy, retains genuine lease/image EX custody while capturing the canonical reservation, normally closes the consumed image before Native terminal retirement, and resumes a durable retirement before opening a replacement. Image/lease cleanup retains the first primary even when its value is undefined or false. The gate authenticates the actual canonical Native image before marker work, permits a missing marker only under actual EX with stable zero/absent journal and a bounded platform-service-only root census, and initializes a closed v2/gzip marker. Ordinary Store admission closes/reopens the consumed receiver with a two-open bound; existing maintenance uses its current EX.

The worker assigns exactly format.json to store-format, imports the canonical marker schema, enforces 2048 bytes and the closed marker before generic JSON handling, and keeps reference scanning. Packed/expanded budget debit and typed512MiB/fragment64MiB/generic256MiB limits remain. Four format laws plus one explicit reset-profile law are mounted; their recording ports are not actual Native deletion proof.

One concrete Source-custody omission blocks an unqualified final signoff: the new canonical marker schema imported by the worker is absent from all four defining lists observed at review—closureImplementationURLsV1 (14 old entries), graphConsumerURLsV1 (20 old entries), Graph graphImplementationUrlsV1 and inference/♻️revision implementationFiles. Existing source-current and pinned rows therefore omit that new defining schema. Root received the exact append-only/schema-first correction requirement with fixed fixture index5 retained. No other concrete core issue was found in this finite readback.

Frozen reviewed core identities (before that pending custody correction):

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/📇️format/🟦️.ts`: SHA256 `96c88633f58cff306fa2a5d8d43072dfa622a03ea387f90f6d4ff2081decbb59`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/📇️format/🧬️schema/🔣️.json`: SHA256 `6371110b6a4b5829353c5c7a27fba48d4c94a23b8ae82b8f622d831bd737b8a0`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts`: SHA256 `f000ec71116a6ba2727820ddfc68d2f15ef6aed87ee9b46ac62dd7e3f22c87d0`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📜️script.ts`: SHA256 `f4f7b2e295ba5c0d3c420ba9fee19d767d67841e2d844f6c24cee20cf5afd25b`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧬️schema/🔣️.json`: SHA256 `202417d548f9c0235a2b1ad215cef83c3f7788a531cfbae41093829d9cc26274`.

Exact omitted-schema custody lists at this review:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts`, `closureImplementationURLsV1`: 14 authored array elements; canonical marker-schema path absent.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs`, `graphConsumerURLsV1`: 20 authored array elements; canonical marker-schema path absent.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts`, `graphImplementationUrlsV1`: 43 authored array elements; canonical marker-schema path absent.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/♻️revision/🟨️.mjs`, `implementationFiles`: 58 authored array elements; canonical marker-schema path absent.

The expected closed closure Source-law corpus moves from14sources/29states to15sources/31states when the new schema is appended. Existing fixture index5 remains unchanged. These counts describe authored array elements (some inference elements are declared constants), not a measured live capture.


## Marker Schema Source Custody Correction — 2026-10-22

The missing canonical marker-schema dependency is now appended exactly once to all four existing defining lists. Closed neutral schema and fixture were saved first:15sources/31states include unpinned marker-schema change refusal and pinned original-byte acceptance. The old14source rows and29state rows remain deeply identical, and fixed fixture index5 is retained. The existing named actual-function/AST law now checks the actual closure owner URL list as well as storage, Graph and inference lists, so omission in any of the four is directly asserted. Existing Ajv, SQLite, Bun/Node SHA parity, loaded rows, observation projection and real guard laws remain.

Static RED before the implementation append confirmed the new dependency absent in each original array. Static readback after append shows closureImplementationURLsV1=15,graphConsumerURLsV1=21,graphImplementationUrlsV1=44,implementationFiles=59,each with exactly one canonical marker schema. Marker4943bytes stays within the existing65536-byte bound. All five changed TS/MJS sources parse with zero diagnostics; changed paths are107–168UTF16 units. No Source cap, current/pinned guard, worker decoder, Native custody or reset implementation was changed. Actual existing repo-graph-maintenance:test gate is still unrun; this is not a runtime pass claim.

Frozen changed files:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🔏️closure/📋️schema.json`: 2791bytes,SHA256 `494bcf276efd05fe0d6297e1681749bb5b30b7014ee51f6ab9d8aa3c9f395064`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🔏️closure/🧫️cases.json`: 10249bytes,SHA256 `47ffc6d7c0139178c62de939441cce6b9e913278b10a588dc1c655266558af80`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🔏️closure/🟦️.ts`: 8426bytes,SHA256 `356388b22d234d52c34284b11c45c1566d5ff066009d82aa603de272db4dfea0`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts`: 50913bytes,SHA256 `0b9d60f955a81a281984b82086844f18248013fb45840a2f290cdef2e8146f95`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs`: 45380bytes,SHA256 `481e38cf26964e78a7327222874fea82bd33206b2ac8aa558340caf078357694`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts`: 90353bytes,SHA256 `fc507ba0591f9955fa85103706b135eef216d40a9e1320e4f6aaa4aa309dab50`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/♻️revision/🟨️.mjs`: 9919bytes,SHA256 `a8fad34f2ba9dd494ff5dc47f2a73f3186d279377e78a7efaae65aafda0d5ffb`.


### Store Format Schema — Four-Roster Readback

2026-10-22 independent read-only review confirms canonical opening/🏪️store/📇️format/🧬️schema/🔣️.json occurs exactly once in each defining roster: closures owner closureImplementationURLsV1, build storage graphConsumerURLsV1, Graph graphImplementationUrlsV1, and inference revision implementationFiles. Current schema/fixture contains fifteen Sources and thirty-one states; fixtureIndex remains5 and the test still checks that position against fixture schema. The appended marker is4943 bytes under unchanged65536 Source bound. The actual owner URL roster is now asserted alongside all other rosters; loaded Graph/inference/observation facts and pinned/unpinned guard states retain existing tests. New marker-changed unpinned refuses; pinned state retains loaded authority. No cap/guard relaxation or Native execution observed. Prior fourteen Sources/twenty-nine states remain in current prefix; exact historical deep-equality is the implementing High’s retained-before comparison, not independently reconstructed from a version-control mutation.

Preservation evidence is retained as generated/marker-schema-custody-boundary.json: the pre-edit14-source array SHA256 `1e202140fd4dafe46fe8c59f6e26cea0f5374f9f88119d8a262d982cbc3f3917` equals the post-edit first14 rows; the pre-edit29-state array SHA256 `4699d4a7cee2da974fda809f9e7e4c1062fe2323ff1b41c297245faf805f813c` equals the post-edit first29 rows. Both exact JSON equality checks were true using the retained pre-edit parsed fixture; index5 was unchanged. No checked-in Source was copied into the ticket.


### Remaining Pinned Manifest Consumer — Concrete Counterexample

2026-10-22 finite read-only scan of authored TS/JS/MJS/Rust under ✏️s, 🧰️framework and ♻️mit-bestand for SEMIO_BUILD_GRAPH_PATH/DIGEST and canonical publication reader names found another current production mismatch: library/🏗️build/📥️inputs/🎯️selected/🦀️rust/🟦️.ts, `withSelectedRustNativeSnapshotV1`, captures the pinned manifest with a64MiB bound and matching digest, then calls `readBuildGraphV1(JSON.parse(bytes.toString("utf8")))`. This interprets the canonical current v2 record manifest as a whole v1 BuildGraph before task selection/fallback, so the valid v2 publication cannot pass. It should consume the existing canonical owned publication API, retaining actual Original/control and complete snapshot/record custody; no v1 fallback is warranted. Parent and Runtime High were informed immediately; no code or process action performed.

The corrected Stdio consumer, private Nx graph-pin leaf, orchestration caller and graph reuse path delegate to withRepositoryBuildGraphPublicationV1. Digest-only namespace/seal bindings are not raw snapshot readers. The scan excluded managed cache/generated/dependency directories, symlinks and files above1MiB and does not claim every repository language or all other product roots were covered. It stops at this concrete counterexample rather than asserting an exhaustive all-consumer pass.


The same finite pin-reader search was completed across remaining authored product roots 🌎️hub, 🎓️teaching, 🏢️semio-tech, 👴️leutwiler, root 🧪️tests and root-level TS/JS/MJS/Rust files: no additional matching consumers. Framework matches were classified: canonical publication owner, private Nx pin leaf, orchestration caller and graph reuse delegate correctly; bootstrap/runtime bind or publish pin environment, selected Rust contains the reported raw interpretation, namespace/seal/startup only bind digest identities, and remaining matches are tests/public reexports. Internal canonical graph publication JSON is a different graph-cache wire and is not the pinned snapshot manifest. Managed/generated/dependency/symlink/over1MiB exclusions remain explicit. No further production raw-v1 pin interpretation was found in this bounded authored-source scope.


## Canonical Selected Rust Reader Saved Boundary

Root independently re-read five saved files; hashes match Runtime High boundary and independent Low review is clean. Present exact pins are consumed through canonical v2 publication inside the existing ALS-held Store; all task/snapshot continuations remain awaited under the actual publication control. Previous absent-pin/daemon/watch/nonrun/already-snapshot guard is retained. The raw whole-graph snapshot parser is removed in favor of canonical role/record/hash/ownership validation. Sixteen neutral cases are statically mounted as the new81st common law. Qualified syntax/import/Ajv checks naturally0 are supplemental; no owning Native test has executed. No workspace graph regeneration or old-format decoder is added.

| File | Bytes | SHA256 |
|---|---:|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️selected/🦀️rust/🟦️.ts` | 9330 | `1c295c8b68ca8a7d72e4eea509a6455385c5a0e70e81a2dabd904953e822ee3f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️selected/🦀️rust/🧪️tests/📌️publication/🟦️.ts` | 9679 | `e72843a7623ffe1828d16efc75a342524ca21c882d98d78d967d9046d58ef2ca` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️selected/🦀️rust/🧪️tests/📌️publication/🧫️cases.json` | 2235 | `7debecd56f656d8afc9ea6be0504aa6ff638871ade5198cf958cb32ab4e6cff9` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️selected/🦀️rust/🧪️tests/📌️publication/🧬️schema/🔣️.json` | 895 | `bfe53f45865a5cb8555b02b7fca777ede4c77f3bfd237e00fe70046aa4ae032b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🟦️.ts` | 7624 | `407d72ccca94d7e7b1661e888e9eb3702b790d64f464bf1c911769ba7d7fa8cd` |

Fresh close inventory merged all five previously absent paths: 300 unique, maximum absolute UTF16 length204, no generated entries.
