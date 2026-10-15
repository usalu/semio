# Graph-Only Native Independent Review

Read-only Source and retained SDK file review, 2026-10-15. No builds, provision, Originals, gates, SQLite opens or production edits. Stored hash comparisons below were made without new live SDK/image custody and therefore are provenance observations, not issued authority. Findings sent to Nx.

Library-relative paths are beneath `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Private route removes the second Rust reader

Ticket `headroom/📜️script.ts` 225–228 validates the exact private installation link under its shared binding resource, then opens `withRepositoryNxGraphNativeStorageV1`, binds exact private workspace/tooling/implementation coordinates, and **returns/awaits** captureOwnedCommandV1 for the real installed Nx child. It does not wrap that child in ordinary NxScript.run or withRepositoryNxStorageV1.

Graph-only owner `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts` 60–65 prepares actual graph storage, opens graph cache allocation and nested native graph backing, and retains namespace/image guards through the awaited body. Its preparation at32–35 opens graph partition and graph maintenance only. No Rust image SH is acquired by this owner. A task can therefore later open its sole Rust maintenance image and request EX without the prior ordinary-Nx-parent Rust SH cycle. Task bodies that independently open another Rust reader must still be assessed separately.

Child process capture joins the actual close event before success/refusal (`process/🎛️owned-execution/📥️capture/🟦️.ts`43–45), including termination after cancellation. Parent graph-only namespace remains held until this promise settles; finally awaits all real namespace releases at graph61/104. No detached-child lifetime is implied for callers that fail to return/await their children.

## Normal CLI and synchronous Native ownership

Ordinary bootstrap NxScript.run still retains full Rust+graph+socket storage (`🚀️bootstrap/📜️script.ts`368); it is correctly separate from private graph-only maintenance. Its binder now exposes SEMIO_NX_NATIVE_GRAPH_DIRECTORY from an actually issued graph namespace/native image. Installed Nx patch wraps normal main in `withOriginalNxGraphNativeStorageV1` before running main/snapshot, and wraps task executor in real loaded-tooling use plus graph owner (patch1641–1642,1702–1705).

Graph require function56–57 looks up the exact directory in private AsyncLocalStorage, requires live scope and matching workspace, rechecks actual store/control, exact environment binding and physical native-root device/inode before returning contained path. Environment text alone cannot populate that map. Received owner65 reconstructs a real bind from admitted invocation/repository authority, compares every descriptor value, then requires that private actual directory before body. Watchers are installed only through an actually held namespace handle at90. JSON descriptors select/check coordinates; they do not register structural image authority.

Acquisition83–87 holds namespace SH plus real enforcing native image. Release removes the private entry, closes its real image, and releases namespace in finally; prior nested private entries are restored only while still live. Full/graph-only outer owners await release and preserve primary plus cleanup errors. These mechanisms provide the declared awaited callback lifetime; no broad accepted arbitrary detached-bind guarantee is inferred.

## Exact --version fast path

Bootstrap exempts only exact one-argument --version at366. Installed SDK main skips graph admission when argv[2] is --version, then its actual version branch117–118 prints version and exits through handleNxVersionCommand184; platform Native assertion and dotenv/cache directory loading explicitly skip that case61/71. This is a static non-Native version path and does not grant normal commands environment-only graph access.

Installed main's NX_COMPLETE completion branch runs before its version branch. No claim is made here that every hostile combination of completion environment plus --version executes only the version branch; supported ordinary exact version behavior is the reviewed case. Native operations remain guarded at their actual directory consumers.

## Fresh SDK provenance and limitation

Observed SDK `60f95aa15d05e3957d142b61b7d3a663484c180683142df4820ca2ec71de1c2a` has authorizer bundle284415 bytes, SHA256 `b30e497b5923543b637b394e43cb0a3b502bac3b58f347eb61f1d545ab16fb2f`, matching its stored output receipt. Actual source manifest bytes match stored authorizer sourceSha256. Bundle exports contain both graph require and received graph owner. Installed `node_modules/nx/dist/bin/nx.js` contains the actual graph main wrapper and version branch, rather than only a proposed patch.

Its captured graph owner Source row is SHA `c639e33459f9aa0552a7ff676c089a4a2977eafd85b5b4b0a39666f50b315024`; current working graph owner bytes differ. Thus fresh SDK runtime results apply to the captured owner version, not automatically every latest Source edit. This is a qualification boundary, not evidence of tampering. Full authenticated SDK identity and patch receipt checks remain the actual runtime owner's job.

## Result

No immediate blocking Source defect found in the current awaited graph-only private route. It addresses the previously identified parent RustSH self-wait while preserving genuine graph Native ownership. Keep latest Source versus captured SDK qualification explicit; preserve actual child joins and no environment bypass. No runtime success is independently claimed by this review.
