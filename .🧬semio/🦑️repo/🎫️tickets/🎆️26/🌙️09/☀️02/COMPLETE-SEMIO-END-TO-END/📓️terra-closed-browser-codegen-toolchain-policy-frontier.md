# Closed Browser Codegen Toolchain Policy Frontier

## Outcome

The production codegen path should use the repository-pinned Bun executable, not provision or discover Node.  The current implementation has already made that important switch: [`browser-bundle/📜️script.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts) resolves `process.execPath` and launches it at lines 48–90 with `--no-install --no-env-file`; the child also requires `Bun.version === 1.3.14`.

That is only a runtime/version admission today, not a sealed compiler policy.  The smallest next slice is an internally-derived `BrowserCodegenPolicyV1` in this same script, followed by a source-snapshot capsule.  Do not add repo-wide Node provisioning unless the Bun/JCO equivalence law fails.  Node remains useful as an independent JSPI/JCO oracle, but it must not become a trusted catalog codegen input through `PATH`.

This was a source-only audit.  I did not launch JCO, Nx, Node, Bun, or a compiler.

## Current Evidence

| Concern | Current source-backed state | Consequence |
| --- | --- | --- |
| Runtime selection | The builder verifies `Bun.version`, resolves `realpathSync(process.execPath)`, and passes that absolute executable to `runExactCargoLawProcess` (browser-bundle script lines 48–49, 66). `runExactCargoLawProcess` gives `spawn` exactly `options.env`, not a merged environment (repo library lines 1875–1906). | No inherited `node` command is used for the production derived artifact. |
| Declared toolchain | Root [`package.json`](../../../../../../package.json) lines 54–57 declares `engines.bun` and `packageManager: bun@1.3.14`, but no Node engine. Unix setup verifies/install-pins that Bun version (native bootstrap `🐚️.sh` lines 445–468); Windows installs Bun but no Node (bootstrap `🔵️.ps1` lines 751–770, 858–870). | There is a repo-owned Bun identity and no repo-owned Node identity to seal. |
| Ambient environment | The codegen child receives only locale/time/temp variables, plus Windows `SystemRoot`, and denies `NODE_OPTIONS`, `NODE_PATH`, `BUN_OPTIONS`, `BUN_PRELOAD`, and `PATH` in child code (browser-bundle script lines 62–75). | This is substantially better than the general `devToolingEnv`, which inherits `process.env` except two Node/IDE variables (library lines 2276–2287). |
| JCO dependency | JCO is a loose dev dependency (`^1.7.0`) in the OS dev TypeScript package, while `bun.lock` resolves JCO `1.27.0` and `jco-transpile 0.6.1` (package lines 28–39; lock lines 845–849). The builder checks only JCO's package version at line 74. | A matching `1.27.0` package directory with changed code is currently accepted. |
| Native transitive input | `jco-transpile` statically imports `oxc-minify` even when the present options omit optimization (`node_modules/@bytecodealliance/jco-transpile/dist/transpile.js` lines 1–10, 60–72). `oxc-minify` loads a target N-API binary during module evaluation and honors `NAPI_RS_NATIVE_LIBRARY_PATH` / `NAPI_RS_FORCE_WASI` (its `index.js` lines 67–74, 530–546). Its lock entry has target-specific bindings, including Darwin, Linux, and Windows variants (bun.lock lines 1197 onward, 2747 onward). | Bun compatibility and the selected N-API binary are material compiler inputs; version strings alone cannot describe them. The current exact environment does remove those NAPI variables because it does not inherit the parent environment. |
| Caller authority | Public `BrowserActorBuildControl` contains `repoRoot` and `evidenceRoot` (browser-bundle script line 15). `repoRoot` selects both the root `package.json` checked at line 48 and the child `cwd` used to resolve `@bytecodealliance/jco` at line 90. | A caller can choose a different workspace/node_modules set which still claims Bun `1.3.14` and JCO `1.27.0`; that is public compiler-policy authority. |
| Artifact identity | `ClosedBrowserActorArtifactV1` contains a fixed string `codegenPolicy`, the raw component SHA-256, and closed output SHA-256 (lines 14, 127). The existing artifact attestation packet correctly keeps this derived actor separate from raw `PackageHashes`. | The catalog has nowhere immutable to bind the compiler/dependency closure that produced the derived bytes. |

The default top-level `@bytecodealliance/jco` path and the now-selected browser-conditioned public API are materially different closures.  The former reaches the general `jco-transpile/dist/transpile.js` path, which imports `oxc-minify` and consequently selects a native N-API binding.  The latter is now selected by the actual child command at browser-bundle script lines 66–81: `--conditions=browser` plus `import("@bytecodealliance/jco/component")`.  JCO's `./component` export resolves to `dist/browser.js`; that file imports only `@bytecodealliance/jco-transpile/component`, whose public export resolves to `vendor/js-component-bindgen-component.js`.  The vendor generator imports the four Preview2 shim entrypoints `cli`, `filesystem`, `io`, and `random`, then fetches its two adjacent core Wasm files (`js-component-bindgen-component.core.wasm` and `.core2.wasm`).  It does not itself import `oxc-minify`.

This is a substantial reduction and avoids the observed default-JCO Bun failure (`preview2` Node-worker `tcp_wrap`), but it is not a reason to hash only three JavaScript files.  The browser policy must snapshot the *actual conditional module closure*: JCO `dist/browser.js`; JCO-transpile's conditional `./component` export plus both adjacent cores; all statically reached Preview2 shim JS modules/assets from those four entrypoints; and any transitive imports resolved while evaluating that closure.  Resolve the closure under the exact `{ conditions: ["browser"], package entry: "@bytecodealliance/jco/component" }` tuple and reject a package export or file outside it.  Record this tuple in the policy.  Do not retain the old OXC/N-API binding in the browser-generator policy unless a resolution trace proves it was actually reached; it belongs only to the Node/default-JCO oracle policy.

## Minimal Bun-First Policy Slice

Implement the following private helpers and types in the existing browser-bundle `📜️script.ts`; do not make an executable, repository root, package map, environment, or policy digest a `BrowserActorBuildControl` input.

```ts
type BrowserCodegenPolicyV1 = Readonly<{
  schema: "semio.os.browser-codegen-policy.v1";
  revision: 1;
  runtime: Readonly<{ kind: "bun"; version: "1.3.14"; executableSha256: string }>;
  lock: Readonly<{ sha256: string; packages: readonly BrowserLockedPackageV1[] }>;
  sources: readonly BrowserSourceDigestV1[];
  options: Readonly<{ jco: "1.27.0"; name: "browser-actor"; instantiation: "async"; asyncMode: "jspi"; nodejsCompat: false; base64Cutoff: 0; importInterfaces: readonly string[]; asyncImports: readonly string[] }>;
  sha256: string;
}>;
```

`BrowserActorBuildControl` should shrink to cancellation/progress only. Derive the repository root from the builder's own `import.meta.dir` (six parents from this script) and derive a private scratch owner from that trusted catalog/build invocation. A ticket test can supply `SEMIO_TEST_ARTIFACT_DIR` to the test harness, but it must not select the JCO working directory or toolchain closure. `evidenceRoot` is presently only a filesystem location, yet its public ownership also makes the generated config/output parent mutable by the caller; move it behind that scratch owner while doing the control reduction.

`BrowserCodegenPolicyV1` is built before JCO execution:

1. Verify the root package manager is exactly `bun@1.3.14`; canonicalize `process.execPath`; require an absolute regular executable; hash it through an opened descriptor with bounded reads and pre/post `fstat`, following the existing Cargo executable pattern in library lines 1851–1871. Persist the digest, not the machine path. Recheck metadata after the child exits. This detects ordinary replacement; a later hardening that requires hostile-writer TOCTOU resistance must execute a verified private copy rather than claim that hashing a pathname authorizes its later execution.
2. Parse the repository's `bun.lock` as the authority and traverse from resolved `@bytecodealliance/jco@1.27.0` through the *observed browser-condition module closure*, preserving sorted package name/version/integrity/dependency-edge/condition/export-subpath rows. Do not hash the entire lock: an unrelated UI dependency should not change this compiler policy. The selected production closure begins with JCO, JCO-transpile, and Preview2 shim; include any package that the fixed resolver reaches. `@oxc-minify/binding-*` is not a browser-generator input merely because it is a dependency of the default JCO API. It is instead required in the separate Node/default-JCO oracle policy if that oracle continues to use `transpileBytes`.
3. For every reachable package directory, walk only regular, non-symlink entries in sorted logical-path order; use open/fstat/read/fstat checks and record `{logicalPath, byteLength, sha256}`. Copy those verified bytes into an owned `scratch/node_modules` capsule and run the absolute Bun executable with the capsule as its `cwd`. Revalidate the copied map before launching. This prevents the currently mutable workspace `node_modules` from changing after the lock/source fingerprint but before `import "@bytecodealliance/jco"` resolves. The closure must be graph-derived, not a hand-maintained JCO/oxc package list: the top-level JCO module also exposes the optimizer and packages can add imports under the same lock graph.
4. Hash the current first-party builder, `🌐️host/🟦️.ts`, `🌐️wasi/🟦️.ts`, and the policy schema/options into `sources`; these affect the final `Bun.build` closure after transpilation. Canonical JSON with all sorted arrays and no absolute paths produces `policy.sha256`.
5. Retain the present child environment shape, adding an explicit denial law for `NAPI_RS_NATIVE_LIBRARY_PATH`, `NAPI_RS_FORCE_WASI`, `NAPI_RS_ENFORCE_VERSION_CHECK`, `NODE_COMPILE_CACHE`, `NODE_V8_COVERAGE`, `BUN_*`, `npm_config_*`, proxy/credential variables, and dynamic-loader variables. `PATH` stays absent. On Windows preserve only a validated `SystemRoot`/`WINDIR` necessary to start Bun; use the owned scratch for `TEMP` and `TMP`. The production browser closure should not depend on any N-API variable; their denial remains useful to prove the selected condition did not accidentally regress to the default JCO path.

The present empty owned configuration, `--no-install`, `--no-env-file`, and `--conditions=browser` launch controls (browser-bundle script lines 60–72) are the right substrate. Do **not** write `preload = []`: Bun rejects that configuration. Bun's parser requires its configuration as one argument, `--config=<owned-path>`: the split form `--config <path>` consumes the following `--eval` in the observed diagnostic and produces empty output. Keep the equals spelling under an exact launch law. These controls still do not bind the package bytes or prevent a caller-selected `cwd`, so they are not the policy by themselves.

## Bun Acceptance Before Removing Node From the Build Story

There is no existing source law that compares the selected Bun browser-component generator with the selected Node oracle for identical component/options/output. The browser closure deliberately avoids `oxc-minify`; the Node/default-JCO oracle may not. Thus the law must name both import entrypoints and their distinct policies, rather than quietly treating a default import and browser-conditioned import as interchangeable.

Add one language-agnostic `browser-codegen-bun-jco-equivalence-v1` vector beside the existing canonical actor fixture. It supplies the same component, map, JSPI mode, async imports, and limits to:

1. the candidate absolute Bun subprocess with the sanitized/capsuled environment, `--conditions=browser`, and `@bytecodealliance/jco/component` `generate`; and
2. a separately spawned Node compatibility oracle that explicitly names its entrypoint (`transpileBytes` or `generate`) and has its own verified dependency closure.

Compare sorted JCO import interfaces, file names, per-file byte lengths/SHA-256, and the final closed ESM byte sequence/SHA-256. The Node half is a test oracle only; it is never an input to `buildClosedBrowserActorArtifactV1` or a catalog policy. A current production acceptance can rely on Bun only after this vector is observed green for the exact selected target binding. If it fails, fail codegen closed and then design a *separate* repo-owned Node provisioner/schema; do not fall back to ambient `node`.

Additional first laws:

1. Mutate one JCO transitive file, the selected oxc N-API file, a lock integrity/edge, or `🌐️host`/`🌐️wasi` after policy derivation: policy admission rejects before codegen.
2. Try a matching-version JCO directory through a caller-supplied root: the public API has no such root and cannot redirect resolution.
3. Inject every denied environment variable; child records the fixed allowlist and produces the identical output/policy.
4. Change a package file after snapshot: capsule execution either uses the verified copied bytes or rejects; it never reads the changed workspace file.
5. At catalog/worker integration, mutate policy digest, source component/descriptor link, closed payload SHA/Blake3/length, or import interface; reject before serving target bytes or importing an actor.

## Catalog Boundary

Do not put this identity into raw `PackageHashes`. That record continues to describe the raw component/core/descriptor source package. The trusted catalog owns a separate derived execution payload record:

```text
semio.os.closed-browser-actor.v1
  source: { componentSha256, componentBlake3, descriptorByteSha256 }
  policySha256
  payload: { mediaType: text/javascript, sha256, blake3, byteLength }
  importInterfaces
```

The catalog generation hashes this record. A verified `DocumentOpenPlan`/target lease selects the derived payload; the private broker verifies the exact body before activation. Neither a client URL nor a document-plan caller selects a runtime executable, codegen policy, package root, or raw component-to-browser conversion.

## Node Contingency (Not a Current Change)

If and only if the Bun equivalence vector fails, add a dedicated codegen-toolchain owner that declares the exact supported Node range/version in repository metadata and provisions it in both existing Unix and Windows bootstraps. Its resolver must return an absolute canonical regular executable, version-check it, hash it with opened-descriptor checks, and run with the same capsule/sanitized environment. It must not accept `SEMIO_NODE`, `PATH` selection at invocation time, or an unrecorded system path. The stored policy still contains only digest/version/target identity, never a host filesystem path.

At present that is unnecessary expansion: the repository intentionally pins Bun, the builder now invokes that Bun directly through the browser-conditioned public JCO generator, and Node is retained exclusively for independent JSPI/JCO fixture execution.

## Current Browser-Target Capsule Update (Source Audit After `13993`)

Root reports that the registered artifact law is now green for the real browser-target `Bun.build` capsule: public `@bytecodealliance/jco/component`, the two JCO-transpile core Wasm inputs, and official browser Preview2 providers.  This update narrows the policy to that actual execution route.  It does **not** qualify the artifact as a catalog-trusted/plan-served browser target, and it does not treat the separate Node high-level final-byte comparison as a compiler input.

### Exact Current Closure

The current helper at [`browser-bundle/📜️script.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts#L47) resolves only these package exports before it invokes `Bun.build`:

| Logical entry | Current resolved implementation | Policy relevance |
| --- | --- | --- |
| `@bytecodealliance/jco/component` | `@bytecodealliance/jco/dist/browser.js` | The browser JCO facade exports `generate` and imports the `component` subpath below. |
| `@bytecodealliance/jco-transpile/component` | `@bytecodealliance/jco-transpile/vendor/js-component-bindgen-component.js` | The executable component generator.  Its general package dependencies such as `oxc-minify` are not reached by this export. |
| `@bytecodealliance/preview2-shim/{cli,filesystem,io,random}` | each package's browser condition, under `dist/browser/` | These are the four vendor-generator imports; their relative browser imports (currently including `environment.js` and `config.js`) are part of the same executable graph. |
| `js-component-bindgen-component.core.wasm`, `js-component-bindgen-component.core2.wasm` | adjacent to the JCO-transpile vendor generator | The generator obtains these with `new URL(..., import.meta.url)` at the vendor source's core instantiation site. |
| `node:fs/promises` | built-in external only | The vendor generator has an exact dynamic fallback for a no-`fetch` runtime.  It is a declared built-in capability, not a package file. |

The first-party closure after JCO output is separate but equally policy-relevant: this script, [`🌐️host/🟦️.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts), [`🌐️wasi/🟦️.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts), and the process runner [`🦑️repo library/🟦️typescript/🟦️.ts`](../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts#L1875).  The latter controls cancellation, child process tree termination, and bounded output; it must be recorded until the codegen runner is extracted into a smaller first-party owned helper.  The TypeScript parser used to validate compiler/generated bundles is also an execution-affecting tool input and needs a pinned resolved version plus source digest.

Do **not** pull the default `@bytecodealliance/jco`/OXC/N-API branch into this policy.  The browser component entry is a different public export.  Mutating a default-only OXC file must not invalidate this browser policy; that belongs to the Node oracle policy, if the oracle remains registered.

### Smallest Sealed Policy and Artifact Record

Replace the fixed string `codegenPolicy: "semio.os.browser-jco-1.27.0-jspi.v1"` at script lines 15 and 163 with a structured, internally-created record and a digest.  No field below may contain an absolute host path.

```text
BrowserCodegenPolicyV1
  schema: "semio.os.browser-codegen-policy.v1"
  revision: 1
  runtime: { kind: "bun", version: "1.3.14", executableSha256 }
  resolver: {
    entry: "@bytecodealliance/jco/component",
    conditions: ["browser"],
    build: { target: "browser", format: "esm", splitting: false,
             minify: false, write: false,
             external: ["node:fs/promises"] }
  }
  compiler: {
    module: { sha256, byteLength },
    cores: [{ name, sha256, byteLength }, ...sorted],
    sources: [{ logicalPath, sha256, byteLength }, ...sorted],
    allowedBuiltinDynamicImports: ["node:fs/promises"]
  }
  firstParty: [{ logicalPath, sha256, byteLength }, ...sorted]
  generate: { exact fixed name/map/JSPI/options/importInterfaces/asyncImports }
  sha256: canonical-serialization digest of every preceding field

ClosedBrowserActorArtifactV1
  schema
  source: { componentSha256, componentBlake3, descriptorBytesSha256 }
  codegen: { policySha256, policyRevision: 1 }
  payload: { mediaType: "text/javascript", sha256, blake3, byteLength }
  importInterfaces: sorted unique
```

The artifact should carry the policy digest, not a mutable source list.  The immutable catalog record owns the complete canonical policy and binds `policySha256`; the target/body verifier must require exact policy and payload bytes before private handoff.  Raw `PackageHashes` remains raw package identity and must not be overloaded with this derived execution payload.

Build the policy from a private builder context: shrink `BrowserActorBuildControl` (line 16) to cancellation/progress only.  Its public `repoRoot` currently selects `package.json`, the child `cwd`, and therefore package resolution (lines 79, 104); its `evidenceRoot` selects the compiler/core/config parent (lines 86–96).  Derive source/package roots from this builder's own module location and let the catalog build owner allocate the private scratch directory.  Test-only evidence ownership belongs outside the public production control API.

For source provenance, resolve the table above with the fixed browser condition, then recursively parse static relative imports from each allowed package root; sort logical paths; reject a resolution outside the admitted package roots.  Package-export names and the conditional route are policy fields.  The executable seal is the resulting compiler module plus its two copied cores, but its provenance rows must enumerate the small input closure that produced them.  This is deliberately narrower than walking all JCO package dependencies or all of `bun.lock`.

### Current P0 Copy and Parser Holes

1. `buildBrowserCodegenModule` does `lstatSync(path)` followed by `readFileSync(path)` for each core (lines 64–66).  Unlike the generated-output reader at lines 123–151, this has no opened-descriptor identity/pre/post-read check.  A replacement can alter a core between check and copy.  Extract the stable bounded reader and use it for the entry source, vendor source, every closure row, and each core.
2. Writing `compiler.mjs` and the cores under caller-selected `evidenceRoot`, then importing `pathToFileURL(compilerPath)` in the child (lines 68–70, 110–111), leaves an execution TOCTOU after copy.  A post-run hash merely detects replacement after the untrusted code ran.  The sealed path needs a private scratch owner plus a self-contained compiler capsule: retain stable verified bytes and make the child load the module/cores from that capsule, not a mutable caller directory.  The component core URLs must be bound to the copied exact bytes before evaluation; a path check followed by `import()` is not sufficient authority.
3. The initial `Bun.build` itself runs in the parent before the sanitized child (line 51).  The green ambient-poison law establishes useful current behavior but does not turn a parent `node_modules` view into policy authority.  Snapshot/verify the small browser closure before this build, and make the build execute from the owned snapshot.  Hash `process.execPath` through an opened descriptor (the library has the exact pattern at lines 1851–1871) and bind it to the policy; `realpathSync` alone at line 81 is not an executable identity.
4. The current AST only counts two matching `new URL` string literals (lines 55–63).  It does not check parse diagnostics, the `URL` constructor's second operand, arbitrary additional `new URL`/`fetch`/dynamic import sites, or the exact external capability.  A strict capsule parser must admit the presently required `import("node:fs/promises")` fallback and the two exact `new URL("./js-component-bindgen-component.core{,2}.wasm", import.meta.url)` sites, while rejecting every other dynamic import, URL, fetch, bare package import, source URL, or network capability.  It must also require exactly the configured `Bun.build` shape, including `write: false` explicitly.
5. The child rereads JCO version through `import.meta.resolve("@bytecodealliance/jco")` at line 111.  Once it imports the sealed compiler capsule this is redundant, reintroduces mutable resolver/cwd authority, and can disagree with the capsule.  Record the resolved entry/package version in policy and remove that child resolution.
6. The final actor bundle already rejects import/export module specifiers, dynamic `import`, and `import.meta` (lines 297–306).  Add a law that its accepted AST has no external URL/fetch escape and no absolute workspace path.  This validates a closed ESM delivery payload, not a sandbox; worker termination, argument aggregate, and core-memory limits remain separate containment work.

### First Minimal Laws

1. **`browser-codegen-browser-closure-v1`**: records the exact sorted browser closure rows and compiler/core digests.  A byte change in JCO browser facade, vendor generator, a reached Preview2 browser module, either core, TypeScript parser/runner/host/WASI, or Bun executable changes `policySha256` and prevents catalog admission.  A default-only OXC file change leaves this particular policy unchanged.
2. **`browser-codegen-capsule-parser-v1`**: accepts the current two-core plus exact `node:fs/promises` fallback shape; rejects malformed JS, a third core, non-`import.meta.url` core base, a changed external, another dynamic import, `fetch`, source URL, package import, or any closure escape.
3. **`browser-codegen-stable-input-v1`**: a controlled rename/replace of compiler JS or either core between metadata check and child execution cannot result in execution of replacement bytes.  It must reject before JCO evaluation or run only the immutable captured capsule.  Test the same property for generated output (the latter is partly covered today).
4. **`browser-codegen-no-caller-root-v1`**: the production build API has no root/evidence fields.  A test harness can allocate evidence, but cannot redirect package resolution, child cwd, config, or codegen output parent.
5. **`closed-browser-artifact-policy-binding-v1`**: mutate policy digest, component/descriptor binding, final JS payload/hash/length, or import-interface ordering; trusted catalog/target-body admission rejects before serving/importing bytes.

The green Node final-byte comparison should stay a sixth, independent oracle law.  It proves the fixed selected generation semantics; it does not authorize Node, its default JCO/OXC graph, or ambient Node resolution as production compiler inputs.

## Capsule Helper Review Before Builder Integration

This source-only review covers the new private `closeBrowserCodegenModule` at [`browser-bundle/📜️script.ts:70`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts#L70), its neutral fixture at [`🧪️fixtures/🔒️compiler-capsule`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🔒️compiler-capsule), and the existing builder.  Root reports the helper/source gate green, but it is not yet integrated into `buildBrowserCodegenModule`; no artifact trust claim follows from this helper alone.

### What Is Correctly Narrowed

The helper parses before and after transformation; requires exactly two named core inputs; replaces the two direct `fetchCompile(new URL("./core…", import.meta.url))` sites with base64; replaces the one loader; and rejects residual AST module imports, dynamic `import`, `import.meta`, `fetch`, and `URL`.  The new stable-reader helper also improves core loading: it opens a regular non-symlink descriptor, has a shared aggregate admission, and compares pre/post descriptor/path identity before returning bytes (script `:44-68`).  It is a sound building block for a captured compiler module.

### P0 — Indirect Evaluation Is Still a Parser Escape

The post-transform validator only detects an AST `ImportKeyword`; it does not reject ordinary calls to `eval`, `Function`, `require`, `process`, or `globalThis` (script `:107-112`).  Consequently the following shape is accepted by the current parser even though it restores residual module/process authority at evaluation time:

```js
eval("import('node:net')");
Function("return process")();
require("node:fs");
```

The present hostile fixture tests direct `await import("node:net")`, a static import, and direct `fetch`, but not an indirect evaluator.  This is a real fail-open of the parser's stated residual-module-IO fence, not merely a missing broad sandbox feature.

The smallest immediate law mutates the actual captured JCO compiler source (not only the miniature fixture) by appending one indirect-eval, Function-constructor, and `require` variant.  Each must reject from `closeBrowserCodegenModule` **before** the child process starts; assert neither generated file nor output manifest exists.  Add direct AST denials for `eval`, `Function`, `AsyncFunction` constructor acquisition, `require`, `process`, `Bun`, `Deno`, and `globalThis` only if the current, source-pinned capsule has none of them.  If the real JCO source requires one, the helper cannot honestly be a sandbox: bind and approve the exact compiler digest *before* it is evaluated and document this parser as an IO-closure transform only.

That distinction matters: the helper necessarily preserves the generator's large program and arbitrary top-level expressions.  No practical syntax allowlist for two core URLs makes that arbitrary program safe to execute.  Its authorization boundary is the sealed codegen policy/digest; its syntax boundary only prevents untracked imports from an otherwise trusted compiler.

### Child Handoff: Exact Safe Boundary

Do not pass data URLs for compiler/core/component bytes through `--eval` or command arguments.  The admitted compiler is up to 8 MiB, components/cores up to 64 MiB, and base64 adds roughly one third; permitted inputs exceed normal argv limits by orders of magnitude.  `runExactCargoLawProcess` currently launches with stdin set to `ignore` ([repo library `🟦️.ts:1875`](../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts#L1875)), so it cannot currently carry a verified capsule stream.

Root's proposed small-argv protocol is coherent if implemented exactly:

1. Parent stable-reads the raw JCO output and two cores; runs `closeBrowserCodegenModule`; writes the **closed** compiler source and the component snapshot to its private scratch; records exact `{sha256, byteLength}` of each.
2. The child receives only private file names and expected scalar digests/lengths.  It reopens each through a bounded descriptor reader, validates identity/length/SHA-256 before evaluation, imports the verified compiler bytes through a `data:` URL, and passes the verified component bytes directly to `generate`.
3. Remove the subsequent child `import.meta.resolve("@bytecodealliance/jco")`/package-version reread presently at script `:190`; it is an unnecessary mutable package/cwd reopen after the sealed import.
4. A replacement before the child opens a file produces a digest mismatch and no evaluation; a replacement after descriptor open cannot change the byte buffer/data-URL imported by that child.  No post-run pathname hash is offered as a substitute.

The first genuine mutation laws should rename/replace (a) the closed compiler file and (b) the component file after parent snapshot but before child dispatch.  Require child failure before `generate`, no output body/manifest, and source output directories still empty.  A source reader replacement law after the child has opened its descriptor must instead prove identical bytes/output, not spuriously require failure.

### Pinned Bun In-Memory Compiler Import: Resolved

The observed `NameTooLong` was an import-specifier limit of the data URL, not an argv limit: the child had already read and SHA-256-checked the compiler and component through the bounded descriptor reader.  On the pinned `bun 1.3.14`, I ran a small no-build diagnostic and observed both `await import(URL.createObjectURL(new Blob(["export const value = 37"], { type: "text/javascript" })))` and a `Bun.plugin(... builder.module(...))` virtual module import succeed.  Bun documents object URLs for `Blob` and its `PluginBuilder.module` virtual-module API ([object URLs](https://bun.com/reference/globals/URL/createObjectURL), [virtual modules](https://bun.com/reference/bun/PluginBuilder/module)).

The integrated production capsule has now selected the smaller Blob route and is reported green: `artifact14 z19zql`, with a `115731`-byte final actor, Node exact-byte oracle, strict manifest/capsule laws, WASI 15, and host 22.  The child creates the URL **only** from its verified in-memory compiler bytes, imports it, and executes `revokeObjectURL` plus zeroing in `finally`; argv remains only scalar paths/digests/lengths.  Both compiler and component pre-dispatch replacement laws now reject `captured input identity` before generation, leave no generated actor files, and keep stdout empty.  This resolves the data-URL path-length defect without adding a policy bypass or caller module URL authority.

Keep `Bun.plugin` out of this first slice.  Although its probe passed, plugin registrations are process-global and have no disposal mechanism suitable for a long-lived codegen host.  Blob URLs have one explicit lifetime, work in the already-isolated short-lived child, and avoid a specifier registry/collision surface.  The bounded admission is unchanged: the closed compiler text, raw component, final actor, and each retained core remain under `browserActorMaximumBytes` (64 MiB); the child must reject before Blob creation when the verified compiler byte length exceeds that cap.  Blob import removes the additional Base64 URL expansion, but it does not lower the existing input/output memory budget or establish a browser sandbox.

### Exact Bun Source-Graph Capture

Current Bun documentation states that `BuildConfig.metafile: true` exposes inputs/imports/outputs, and that `onLoad` runs after resolution but before Bun reads/parses a module ([BuildConfig](https://bun.com/reference/bun/BuildConfig), [Plugins](https://bun.com/docs/runtime/plugins)).  In the current browser-bundle script, `Bun.build` already has a virtual-entry plugin at `:352`; that is the reusable first-party seam.

Use both hooks with different roles:

* `onLoad({ filter: /.*/, namespace: "file" }, args => …)` is the authority mechanism.  Admit `args.path` only under the canonical allowed browser roots; stable-read it once through `readBrowserBuildFile`; record canonical logical path/digest/length; and return the captured `contents` with the fixed JS loader.  Bun then parses the supplied bytes rather than reopening that source file.  Repeated paths must return the original captured bytes and be byte/digest identical.
* `metafile: true` is the coverage witness, not the capture mechanism.  After a successful build, canonicalize its input path set and require exact equality with the captured `onLoad` source rows (apart from the explicit `node:fs/promises` external and the two manually captured runtime core Wasm files).  Do not serialize absolute metadata paths; map them to logical package-relative names before policy hashing.
* `onResolve` alone is insufficient.  It receives a requested specifier/importer before default resolution; merely logging it neither tells policy the final conditional browser file nor prevents Bun from reopening it.  Keep it only to reject/record requested unsupported schemes.  The actual resolved file identity comes from `onLoad` plus metafile cross-check.

The snapshot plugin must use only the build's explicit plugin list and its return `contents`; it must reject non-JS source/assets, a resolved path outside the four current browser package roots, and any metafile input not captured.  The two vendor cores are not ordinary JS graph inputs because the generator obtains them by URL; capture them with the existing descriptor reader and bind their digest into the same policy.

Add one focused **`browser-codegen-onload-snapshot-v1`** law: a test hook replaces a previously `onLoad`-captured browser source after the hook returned its bytes but before the build completes.  The build output and source-row digest must remain baseline-identical, and `metafile.inputs` must exactly match captured rows.  A replacement of an un-captured/foreign path must be rejected before build output.  This proves no `node_modules` source reopen after capture; it does not claim source resolution itself is a sandbox.

## Integrated Source-Capture Review After `21165`

This review is source-only.  Root reports the six source laws under receipt `21165` green (including the Node/WebCrypto oracle, exact metafile witness, and post-`onLoad` replacement), and reports the pre-existing artifact fourteen-law run green at `brrzKd`.  Those receipts are root's qualification evidence; this audit inspected the current implementation at [`browser-bundle/📜️script.ts:123`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts#L123) without rerunning it.

### Current Coverage Semantics Are Soundly Narrow

`captureBrowserCodegenSources` canonicalizes each admitted root once, intercepts every resolved `file`-namespace source with `onLoad`, and supplies the retained `readBrowserBuildFile` bytes as Bun's `contents` (`:126-150`).  That reader holds one descriptor across bounded reads and checks the descriptor and pathname pre/post identity, so a replacement either occurs before capture and becomes the captured source or is rejected; it is not reopened by Bun after the hook returns.  The source fixture's replacement callback proves the latter condition and its independent Node import proves the retained output semantics (`:681-726`).

The metafile equality at `:152-156` is correctly a completeness witness: it requires the exact set of bundled JS file inputs to equal the snapshot map.  It does not pretend to cover the declared `node:fs/promises` built-in, which is external by fixed build configuration (`:132`) and then handled by the separate exact compiler-capsule transform.  It also does not cover the two vendor Wasm cores, which intentionally never enter Bun's JS graph and are separately descriptor-read at `:174-176`.  `write: false`, one output, the 128-source/8-MiB source admission, and the 8-MiB bundled-output cap make the scope explicit.

I found no current path that lets an uncaptured *file* source reach this Bun build.  The old broad parser concerns do not apply to this capture boundary: this is a source-authority snapshot, not a claim that a trusted compiler is a general-purpose sandbox.

### P0 — Captured Provenance Must Leave the Evidence Directory in Memory

The current capture returns frozen `inputs`, but `buildBrowserCodegenModule` serializes them only to mutable scratch evidence (`compiler-sources.json`) and returns just `{ path, sha256, byteLength }` (`:167-180`).  Therefore the existing source rows prove a test/evidence fact but cannot yet bind a catalog policy.  A later policy builder that rereads this JSON has reopened mutable scratch authority; a builder that simply omits it loses the source closure that produced the captured compiler.

The smallest correction is the one root has selected: extend the **private** return to retain `inputs` and exact core rows in memory.  Construct and digest this record before any policy publication:

```text
{
  schema: "semio.os.browser-codegen-source-manifest.v1",
  sources: [{ logicalPath, sha256, byteLength }, ...sorted],
  cores: [{ name, sha256, byteLength }, ...sorted],
  compiler: { sha256, byteLength }
}
```

The compiler-policy builder consumes that frozen value directly.  Scratch `compiler-sources.json` remains diagnostic-only and is never a read authority.  The policy records the manifest digest and complete rows; the derived actor record binds only the resulting policy digest.  A law must mutate the scratch JSON after capture and prove the canonical policy digest/record is unchanged, then mutate one retained source/core row before policy construction and prove policy admission rejects or changes deterministically.  This completes source provenance without expanding the runtime containment scope.

### Small Root-Identity Preflight

The three current roots supplied at `:167-171` are fixed and disjoint.  Still, the reusable helper uses the first `find` match (`:137`) without checking unique logical-root names or nested/equal canonical real paths.  It also treats a resolved source exactly equal to a root as out of scope because it requires `root + sep`.

Preflight `roots` before `Bun.build`: require 1–64 unique nonempty ASCII logical names; canonical real paths; and no equal/nested canonical roots.  Match `path === root.path || path.startsWith(root.path + sep)`.  This does not alter the present closure; it prevents a future caller from creating ambiguous `logicalPath` identities or an accidental root-entry rejection.  One compact neutral mutation vector covers duplicate logical name, equal path, ancestor/descendant path, and root-as-entry, with all invalid rows rejected before `onLoad`/Bun build.

### Existing First-Party Helpers for the Policy Record

Use the existing helpers rather than a browser-specific generic copy:

* [`canonicalJson`](../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts#L1688) is exported by the first-party normalization module.  It recursively sorts object keys and deterministically sorts identity-bearing row arrays.  Build fixed-schema values with source/core/interface arrays explicitly sorted first, then compute `sha256(canonicalJson(unsignedPolicy))`; persist exactly that UTF-8 text and reject a parsed record whose reserialization differs.  Its array-identity behavior means it should be used only after the policy schema has explicitly defined order, never as an excuse for accepting arbitrary user arrays.
* [`exactCargoExecutableFingerprint`](../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts#L1851) already implements the needed stable executable read: absolute non-symlink → canonical path → one retained descriptor → 64-KiB streaming SHA-256 → pre/post file identity, size, and mtime check.  It is private and Cargo-named, so the clean reuse is to rename/promote this existing implementation to an exported ownership-neutral `exactExecutableFingerprint`, retaining `runExactCargoLaws` as its existing consumer.  Do not duplicate weaker `readFileSync(process.execPath)` logic in browser-bundle.

The policy must store only executable digest/version/runtime kind, never the returned host path.  The private builder may use the canonical path immediately for its process launch after hashing, while the catalog retains the portable identity tuple.

### Executable Fingerprint Is a Receipt, Not Yet a Cross-Platform Launch Seal

The existing helper is exactly the right streaming digest/regular-file primitive, but it does **not** itself bind the executable later run by a pathname.  It hashes its retained descriptor at [`library/🟦️.ts:1851-1871`](../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts#L1851), whereas `runExactCargoLawProcess` launches the supplied pathname later at [`:1875-1883`](../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts#L1875).  A replacement with another `bun 1.3.14` binary between those points preserves the version assertion but executes bytes not represented by `executableSha256`.

For the immediate policy record, classify that digest accurately as a **tool receipt identity** unless the desktop-installed Bun executable is an explicit trusted host assumption.  For an actual build-tool launch seal, extend the same first-party stable-reader seam to copy verified executable bytes into a private executable snapshot and launch only that owned copy; check the copy's length/digest before launch and retain it through child exit.  The implementation must be platform-specific where executable permissions/copy semantics differ, but the public policy tuple remains `{ kind, version, sha256 }`.  Do not claim that the current descriptor hash alone closes this TOCTOU.

## Current First-Party Runtime Closure And Policy Ownership

This addendum is a source-only review of the requested host/WASI-policy integration. I did not run the active `56701` qualification, Bun, Cargo, or a browser. It supersedes the older suggestion to treat the executable fingerprint as a hostile-desktop launch sandbox: the current bounded policy explicitly trusts the pinned installed Bun binary, retains its fingerprint receipt, rechecks it after the child, and withholds publication on a changed receipt.

### Exact Current Closure

The final actor bundle is still assembled from a virtual TypeScript entry at [`browser-bundle/📜️script.ts:341-416`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts#L341). Its generated text imports **absolute live filesystem paths** for exactly these two first-party runtime modules:

| Captured logical row | Live import currently emitted | Current transitive source imports | Policy consequence |
| --- | --- | --- | --- |
| `browser-bundle/🌐️host/🟦️.ts` | `createBrowserHostActivation` at script `:349` | none | Its bytes must be retained before the final `Bun.build`. |
| `browser-bundle/🌐️wasi/🟦️.ts` | `createBrowserWasiActivation` at script `:350` | none | Its bytes and the exported interface vocabulary used by the builder must be retained before codegen. |

An exact source scan found no `import`/`export … from` declarations in either module; their browser globals are platform APIs, not relative source dependencies. Thus the two-file closure is sufficient **today**. A future relative/bare import from either snapshot namespace must fail closed and require an explicit new captured row; it must never silently fall back to Bun's file loader.

### P0 — A Byte Snapshot Alone Does Not Close The Final Build

The proposed pre-codegen host/WASI snapshots are necessary but are ineffective if `closedBrowserActorBundle` continues to emit the absolute import specifiers at `:349-350` and the existing virtual-entry hook keeps `resolveDir: import.meta.dir` at `:413-416`. Bun will resolve and reopen the current filesystem path.

Keep the public generic component routine out of the trusted path and add a private retained runtime-input parameter, for example:

```ts
type BrowserFirstPartyRuntimeSnapshot = Readonly<{
  host: Readonly<{ canonicalPath: string; row: BrowserCodegenSourceDigest; bytes: Buffer }>;
  wasi: Readonly<{ canonicalPath: string; row: BrowserCodegenSourceDigest; bytes: Buffer }>;
}>;
```

Capture both through the existing descriptor-owning `readBrowserBuildFile` (`:47-68`) before JCO generation; construct the sealed policy from only `row`, but retain `bytes` privately through the final build. In the final `Bun.build` plugin:

1. Retain the current virtual root `semio:closed-browser-actor`.
2. In `onResolve`, map only the two exact canonical absolute specifiers used by the virtual root to distinct private paths in a `semio-first-party-snapshot` namespace.
3. In `onLoad` for that namespace, return the corresponding retained `bytes` with `loader: "ts"`—no filename read, `resolveDir`, or second digest check.
4. Reject every other resolution whose importer namespace is either the virtual root or this snapshot namespace. In particular, reject a relative import from a future host/WASI version instead of resolving it through the workspace.

The already closed JCO component body has no imports, so these are the complete expected resolutions. Set `metafile: true` for this final build and require exactly the virtual entry plus the two snapshot namespace rows; treat the metafile only as a coverage witness, never as an authority to reopen sources. Add a cancellation check immediately before the final build and after `await build.outputs[0].text()`; zero the retained host/WASI buffers on every non-publication exit just as the component snapshot is cleared at `:280`.

One narrow mutation law is enough to establish this boundary: capture both rows, rename/replace `🌐️host/🟦️.ts` or `🌐️wasi/🟦️.ts` immediately after capture, then complete the final build. Require baseline-identical actor bytes and policy SHA-256; separately append a relative import to a snapshot fixture and require failure before any output is returned. A capture-only law that leaves the current file imports intact does not prove this property.

### Policy Record: Exact Inputs And Missing Schema Rows

`sealBrowserCodegenPolicy` already makes an owned canonical/deep-frozen record at [`browser-bundle/📜️script.ts:126-138`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts#L126), while the schema currently permits a runtime, compiler, exactly three package rows, first-party rows, and fixed options ([`🧬️codegen-policy/🧬️.schema.json:5-18`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧬️codegen-policy/🧬️.schema.json#L5)). Complete the planned `revision: 1` record from in-memory captures in this order:

1. **Runtime receipt.** Use the exported [`exactExecutableFingerprint`](../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts#L1851) before dispatch, execute only its canonical `path`, fingerprint again after child completion, and compare complete `{path, sha256, byteLength}` before returning an artifact. Store only `{kind:"bun", version:"1.3.14", executable:{sha256,byteLength}}`, never a host path. A changed or unreadable second receipt destroys the in-memory output and returns an error; it is not published to the future catalog producer.
2. **Compiler rows.** Reuse the frozen `inputs` and core digest rows now returned from `buildBrowserCodegenModule` (`script :175-195`), not `compiler-sources.json`, which remains mutable diagnostic evidence. Preserve the fixed browser resolver/build tuple already passed to `captureBrowserCodegenSources` (`:147`, `:177-190`) in `options`.
3. **Exactly three JCO package provenance rows.** The bounded production closure roots are `@bytecodealliance/jco@1.27.0`, `@bytecodealliance/jco-transpile@0.6.1`, and `@bytecodealliance/preview2-shim@0.20.1`; their resolved `bun.lock` records are at lines `845`, `847`, and `849`. For each schema package row, retain the stable-read package manifest SHA-256 and a SHA-256 of the **canonical parsed selected lock row**, not a fragile text slice and not the entire unrelated workspace lockfile. The current three-row schema is appropriate for these JCO roots and must reject duplicate/out-of-order names before sealing.
4. **First-party rows.** Stable-read and sort: this builder [`📜️script.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts), runtime [`🌐️host/🟦️.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts), [`🌐️wasi/🟦️.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts), child runner/fingerprint [`📦️packages/🟦️typescript/🟦️.ts`](../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts), canonical serializer [`🧹️normalization/🟦️.ts`](../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts), and [`🧬️codegen-policy/🧬️.schema.json`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧬️codegen-policy/🧬️.schema.json). All six fit the schema maximum of sixteen.
5. **TypeScript parser.** This is not a first-party row: `typescript` is a static production import at `script :8` and drives both `closeBrowserCodegenModule` (`:82-120`) and generated-actor parsing (`:420-426`). The planned policy otherwise has no identity for a changed parser. Add a narrowly named `parser` object to the schema, with `{name:"typescript", version:"5.9.3", manifestSha256, lockRowSha256, entry:{logicalPath,sha256,byteLength}}`, or expand the fixed package list to four rows. Do not mislabel this third-party parser under `firstParty`.

The other static external import, `Ajv2020` at `script :7`, is used solely by source-law functions (for example `:577`, `:658`, `:748`). It is nevertheless evaluated when this production module loads and is absent from the planned policy. The smallest clean correction is to move that import into each test-only law (or one test-only `loadAjv` helper), eliminating AJV from production builder initialization. `fast-json-stable-stringify` is already a test-local dynamic oracle at `:749` and needs no production row.

`browserWasiInterfaces` is also statically read from WASI at module load (`:9`, then `:21`, `:344-348`). Snapshotting WASI later correctly freezes the final emitted actor, but it does not retrospectively prove that its preloaded interface list came from the same bytes. The small coherent rule is: capture the WASI source before policy creation and include the frozen sorted interface array in policy `options`; reject if a fresh policy capture would change that array's source digest while the builder has already used the loaded vocabulary. This is a source-consistency fence, not a demand to turn the build script itself into a separately sandboxed process.

### Production API Shrink

There is no production catalog/Hub caller yet. `rg` finds only the in-module laws and the actor-import fixture call at [`🧪️fixtures/🌊️actor-import/📜️script.ts:120`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/📜️script.ts#L120). Therefore this is the right time to remove caller-controlled compiler authority without migration support:

* Shrink exported `BrowserActorBuildControl` at script `:17` to `{cancelled?, progress?}`. It must not expose `repoRoot` (currently selects both package-manager proof and child `cwd` at `:205`/`:262`) or `evidenceRoot` (currently chooses the config/component/output parent at `:212-217`).
* Make repository/source roots and scratch ownership private to the policy/catalog builder. The fixture must use a test-only private construction path, with `SEMIO_TEST_ARTIFACT_DIR` supplying evidence only to the test harness; it cannot choose package lookup or the codegen cwd.
* `closedBrowserActorBundle` remains exported only because the actor-import fixture dynamically imports it at fixture `:446-449`. It accepts raw JCO source/cores and emits a bundle with live imports, so it must not be a trusted-catalog API. Make the retained-snapshot bundler private and move the raw routine behind a test-only fixture export (or exercise it through the in-module law) before a catalog producer is introduced. The sole public production entry should accept a verified component plus cancellation/progress and return a policy-bound derived receipt.

### Minimum Neutral Laws Before Catalog Wiring

1. `browser-codegen-first-party-snapshot-v1`: host and WASI replacement after descriptor capture produces the old retained actor/policy bytes; a new snapshot-namespace import rejects.
2. `browser-codegen-policy-source-row-v1`: mutate each of the six first-party rows, the TypeScript parser entry, each selected JCO manifest/lock row, compiler input/core, an option/interface, or the executable receipt. Each mutation changes/rejects the policy before artifact publication. Mutating `compiler-sources.json` alone changes nothing.
3. `browser-codegen-policy-no-caller-root-v1`: the exported production control type has no root/evidence field, and a fixture cannot redirect package resolution/cwd while retaining a valid policy SHA-256.
4. `browser-codegen-policy-post-executable-recheck-v1`: replace the trusted installed executable in a controlled test seam after the child begins/returns; an unequal second receipt returns no artifact. This asserts receipt/postcheck behavior only, not hostile desktop executable containment.
5. `closed-browser-policy-catalog-binding-v1`: a future catalog producer must reject a policy digest, component/descriptor binding, output digest/length, or sorted interface mismatch before a private target-body route returns any byte.

No policy/catalog/worker wiring is claimed by this audit. The deliverable from this slice is a policy-bound in-memory derived actor receipt whose host/WASI imports are demonstrably the captured bytes; trusted catalog storage, plan selection, broker handoff, and worker containment remain later boundaries.
