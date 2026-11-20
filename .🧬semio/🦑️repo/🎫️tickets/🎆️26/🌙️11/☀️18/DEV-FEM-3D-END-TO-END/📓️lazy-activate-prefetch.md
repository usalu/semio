# Lazy-activate prefetch errors

Files: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🚀️activation-vite-plugin/🏭️runtime/🟦️.ts` (fixed), `.../🧪️tests/🟦️.ts` (new AST law).

## Root cause

Each lazy build called `receiveScriptProcessInvocation(process.env, ...)`. That function is the *birth* of the process: when no ambient invocation exists (the vite child process has none; its envelope arrives only through `SEMIO_SCRIPT_PROCESS_INVOCATION`) it sets the module flag `received = true` and refuses a second call with "Original process birth already received". The vite process had never received its envelope, so the first prefetch consumed the birth and the 73 later ones were refused. A child invocation is never obtained by receiving the env again; it is derived from the live received invocation (`withScriptInvocationSignalV1` / `withHandledScriptInvocationSignalV1`, then `scriptCompilerAdmissionControlV1(child, ...)` = `createIssuedResourceControlV1`). Spawned repo scripts get their own envelope via `scriptInvocationEnvironment`, but the build runs in-process, so no spawn is needed.

## First error ("Live Original resource control issuance required")

The first prefetch (animate) did receive the birth, then wrapped the issued control as `{...parent, publish}` to forward "Waiting for compiler capacity" lines. Issued controls are authenticated by identity in a private WeakMap (`controls.get(control)` in `requireIssuedResourceControlV1`); a spread copy is a structural copy that is not in the map, so the first `requireIssuedResourceControlV1` inside the build driver threw that message.

## Fix

- `activateServer` receives the process envelope exactly once, lazily at the first build, holds it for the server lifetime (`serverInvocation`, released in `dispose`), and declares itself a service with `enterScriptProcessServiceV1` so the stall watchdog never kills the idle dev server.
- Each build runs in `withHandledScriptInvocationSignalV1(original, job.controller.signal, child => ...)` and gets its own `scriptCompilerAdmissionControlV1(child, "vite:lazy-activate:<id>", 0)`; the control is passed unwrapped.
- A genuine build failure is captured inside the callback and rethrown after it, so it does not poison the long-lived parent scope (`track` would otherwise close its admission for every later build). Release/abort maps to `cancelled <id>` as before.
- Removed: the "Waiting for compiler capacity (Nms)" progress forwarding (needs a non-issued wrapper control). Build log lines still reach listeners.

## Test

`bun test ./🧪️tests/🟦️.ts` in the plugin folder: passes (AST law: one receive, inside `serverInvocation`; derived invocation per build; control from `child`; no control spread). No live end-to-end run; the running dev server (6087) was not touched and keeps the old code until restarted.

## Open risk

If a child scope's tracked work fails inside the build driver (outside the callback), `settle` may still close the parent's admission; observe after restart.
