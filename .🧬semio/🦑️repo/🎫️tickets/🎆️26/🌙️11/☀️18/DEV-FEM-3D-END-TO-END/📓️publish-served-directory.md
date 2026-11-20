# Publication rename blocked by the dev server's served directory

## Finding

The `EPERM` rename of `.cache/b/p/m/browser/prototype/modules/🏗️fem` was caused by open file handles inside the directory that the dev server leaked.

Watchers are not the cause:
- `server.watch` is `null`.
- The recursive source watcher (`🔌️vite-plugins/🔖️source-freshness`) skips `.cache` via `UNWATCHED_REPOSITORY_SEGMENTS`.
- The activation receipt watcher watches the separate runtime `activation` directory.

The static route `/🔌️plugin-modules/<dir>/…` is mounted through `staticDirMountVitePlugins` -> `createStaticDirMiddleware` -> `serveFileWithValidatorsV1` in `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/📤️file-serving/🟦️.ts`.

That function streamed with `createReadStream(filePath).pipe(res)`. `pipe` does not destroy its source when the destination closes or errors. A browser that aborts a download, such as a reload during the 366 MB `core.wasm` fetch, leaves the file stream open and paused on backpressure, so the handle stays open until GC. On Windows an open handle inside a directory blocks renaming that directory (`EPERM`).

## Fix

`pipe` is replaced by `pipeline(createReadStream(filePath), res, () => undefined)`, so the handle closes on response end, error or client drop.

Publication was not changed: the 5 s retry window stays as is. A generation or pointer scheme would be a larger contract change. It is only worth it if a long, still-running download (a 366 MB fetch lasts seconds on loopback) turns out to block publication again.

## Test

`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/📦️asset-delivery/🟦️.ts`, new case "releases the file handle when the client drops mid-download so the served directory can be renamed". It serves an 8 MB file to a tiny-buffer response, destroys the response, then renames the containing directory.

- Fails with `pipe` (EPERM, verified by temporarily reverting).
- Passes with `pipeline`. The existing validators test also passes.

Command:
`repo tools exec --light --lang=en bun -- test ./…/asset-delivery/🟦️.ts -t "dev-served|releases the file handle"`

## Residual risk

Any other long-lived reader of the module directory (for example a legitimate in-flight download at swap time) can still delay the swap by seconds. The existing retry covers that case.
