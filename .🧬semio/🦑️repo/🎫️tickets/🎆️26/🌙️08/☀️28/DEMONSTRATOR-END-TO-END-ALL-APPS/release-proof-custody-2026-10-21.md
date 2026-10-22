# Release Proof Under Live Artifact Custody

Read-only design evidence; no release build, probe or new Source code was executed.

The existing public API is sufficient: `withViteArtifactV1(workspace,"release","demonstrator:site",original,phase,callback)` from `@semio-tech/repo-lib/dev`. It opens the products partition under the real Original, derives the same viteStorageLayoutV1 output used by buildViteArtifact, and enrolls an artifact reader/task. Owner exactly matches Demonstrator site runBuildScript. Layout is managed productsroot/vite/release/SHA256(owner)[0:16]/out; do not manually inspect that path.

Inside the accepted callback, artifact.files is the parsed/closed `.semio-artifact.json` inventory bound to owner and finite output budgets (1GiB/65536 files). The reader already checks the physical manifest descriptor, exact declared versus actual file list, no symlinks/singlelinks/path256, hashes every actual descriptor and retains image+artifact SH custody. `await artifact.recheck()` before returning repeats identity/content inspection; reader also rechecks during drainage. This proves a coherent publication, not merely a successful command or a copied receipt.

Concrete future executable route: an explicitly authored ticket verification script dispatched through `SEMIO_BUILD_MODE=release repo exec --projects=@semio-tech/mit-bestand-demonstrator -- bun <absolute-ticket>/release-proof.ts`, using receiveScriptProcessInvocation/BundleScript and an issued Original-derived phase. Script calls the existing public withViteArtifactV1 above and writes only detached proof DATA to ticket generated output after accepted callback succeeds. No such script or target exists yet; no code is proposed as already executed. The existing API, not an added infrastructure feature, supplies all required custody.

Minimal proof assertions inside that callback: exact owner, finite nonempty inventory, expected authored HTML/JS/WASM/runtime browser assets derived from current build configuration/catalog, and artifact.recheck succeeds. If HTML or a Vite manifest must be read, do so only inside the live callback and only for a declared bounded artifact.files member; do not read managed paths before/after callback. `.semio-artifact.json` is the required repository publication receipt; the Vite config does not establish a separately named Vite manifest, so do not assert an assumed `.vite/manifest.json`. Ordinary `serve` is prototype activation/Vite service and does not serve this release publication. Release claim remains successful coherent build publication; browser behavior is proven separately by authored E2E.

Exact owners:

- /Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/🔨️modules/📦️site/🟦️.ts runBuildScript ownerdemonstrator:site.
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts buildViteArtifact publication.
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/💾️storage/🟦️.ts withViteArtifactV1.
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/🟦️.ts live enrollment/drainage.
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/📥️inspection/🟦️.ts physical manifest/hash proof.
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts explicit publicdev reexport.

## Store Receipt Crosscheck

The fixed/default uncached release target declares `.🧾️store/build.json`. Its actual store receipt schema is closed version/project/target/entries/links; entries contain path/kind/bytes/files/sha256. Bounds are4MiB and4096entries,224unit receiptpaths. The producer records after settled publication. Its tree digest sorts UTF8 Buffer paths and hashes each `path + newline + size + newline + fileSHA + newline` sequence. These receipt facts supplement the actual live artifact reader; neither a detached path nor receipt alone grants managed-storage custody. The managed `.semio-artifact.json` has exact version/owner/files and NO modefield; releaseidentity must come from the actual fixed release target and withViteArtifactV1 release argument, not an invented manifest property.

## Issued Original and Phase Contract

The proposed script must receive the actual parent envelope, not construct a ScriptInvocation/control object. Public routing exports receiveScriptProcessInvocation, BundleScript, ScriptRouter and withIssuedResourceControlV1. The exact pattern is:

```ts
class ReleaseProof extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("release proof accepts no arguments");
    await withIssuedResourceControlV1(this.invocation, "release-proof", 120000, phase =>
      withViteArtifactV1(this.repoRoot, "release", "demonstrator:site", this.invocation, phase, async artifact => {
        if (!artifact.files.length) throw Error("release inventory required");
        await artifact.recheck();
      }));
  }
}
const router = new ScriptRouter(import.meta.dirname).register("verify", ReleaseProof);
await receiveScriptProcessInvocation(process.env, original => router.run(process.argv.slice(2), original));
```

This is a contract sketch only, not an authored/executed script. Imports must use the explicit public routing package/facade and @semio-tech/repo-lib/dev. ScriptRouter resolves the workspace from paired markers; BundleScript checks the received invocation. receiveScriptProcessInvocation refuses a missing/nonactual parent envelope and duplicate process birth. withIssuedResourceControlV1 tracks and settles an issued child phase, joins lease-task operations and checks Original remaining authority on both sides. Pass this.invocation as Original and the issued phase as phaseControl; do not substitute a plain AbortSignal or hand-built control. The 120000ms request is bounded by Original, not a deadline extension. The proposed command must include the registered operation: `SEMIO_BUILD_MODE=release repo exec --projects=@semio-tech/mit-bestand-demonstrator -- bun <absolute-ticket>/release-proof.ts verify`.

Bun provisioning choice is the documented normal `repo preflight` then `repo tools setup all` prerequisite when tools are not already ready, followed by the ordinary repo exec route. This audit has not established that bare `bun` resolves on every host merely because the repo executable itself is running; do not substitute an arbitrary system Bun or hard-code the current host executable. Confirm the owned provisioned Bun resolution from the normal execution/tool interface before authoring the final command. No tools setup/exec command was run by this audit.

## Normal Nx Exec Bun Provisioning Readback

The previous bare-Bun uncertainty is resolved for explicit `repo --tool=nx exec`. In repoLib/⚡️caching/🚀️bootstrap/📜️script.ts, runWithPreparedToolingStore awaits nxBootstrapServices().provisionNxTools(root,store,phaseControl), then activateNxTools, before constructing the Nx environment. Its tooling result contains an absolute bun path. pinnedNxRuntimeEnvironment(source,tooling.bun) refuses nonabsolute paths, removes duplicate PATH spellings case-insensitively on Windows, and prepends dirname(tooling.bun). That environment is passed through nativeNxChildEnvironmentV1 and nxChildEnvironment to spawnNxProcess(node,[nxCli,...args],...). Therefore the concrete proposed command should select Nx explicitly: `SEMIO_BUILD_MODE=release repo --tool=nx exec --projects=@semio-tech/mit-bestand-demonstrator -- bun <absolute-ticket>/release-proof.ts verify`. Normal admitted exec provisions/activates its locked Bun automatically; it does not rely on the host system Bun. No separate raw Bun invocation or tools-setup execution is required by this route. This is source-backed provisioning evidence, not an executed release verification.

## Retained Script Independent Readback

The subsequently authored ticket release-proof.ts was read, not executed. Its actual receiver→router→BundleScript→tracked issued phase→withViteArtifact chain retains genuine authority and writes detached proof after reader drainage. The hardcoded index.html assertion is justified: Demonstrator config installs semioEmojiIndexHtmlVitePlugin; its closeBundle copies emitted 🌐️.html to STATIC_SITE_HTML_ALIASES index.html and404.html. Runtime sources declare relative destination directories, so destination+slash inventory-prefix checks match publication. Artifact inspection excludes only .semio-artifact.json from the declared artifact.files inventory, while tree digest includes it; receipt.files===artifact.files.length+1 is correct. The script parses only the ordinary bounded task receipt, selects the one exact live output entry and digests only that held artifact; unrelated receipt store entries are not walked. No concrete defect found in this source review. Actual release build and actual proof execution remain required.
