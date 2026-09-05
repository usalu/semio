# Nx Unicode Transport Diagnosis

## Observed failure

Nx 21.6.11 reported `@semio-tech/framework-renderer-wgpu` at both the actual `📺️renderer` owner and a phantom `���️renderer` owner. A filesystem census found only the real directory, with UTF-8 basename bytes `f09f93baefb88f72656e6465726572`; its actual project manifest exists. The phantom directory does not. No project was renamed, deleted, or assigned a different name to hide this failure.

Read-only inspection of `.nx/workspace-data/project-graph.json`, `file-map.json`, and `source-maps.json` found no U+FFFD character. This does not prove that every internal cache is unaffected; it bounds what was actually inspected.

## Exact byte-boundary cause

The installed `node_modules/nx/src/utils/consume-messages-from-socket.js` calls `data.toString()` separately for each socket chunk, then concatenates the resulting strings. The plugin-worker and plugin-pool both use this decoder. A UTF-8 sequence split between chunks is therefore replaced before reassembly.

The language-neutral reproduction input is one JSON frame followed by byte `04`:

```json
{"root":"modules/📺️renderer"}
```

Split the UTF-8 byte stream two bytes into `📺` (`f0 9f | 93 ba`). Feeding those two legal transport chunks into the actual installed Nx consumer yields `modules/���️renderer`, exactly the observed phantom spelling. Decoding the complete frame with independent WHATWG `TextDecoder` preserves `modules/📺️renderer`. This test does not create either directory or alter dependency code.

## Existing repository boundary

The repository already disables this lossy IPC path: root `📜️script.ts` sets `NX_ISOLATE_PLUGINS=false`, and `devToolingEnv` forces the same value. The existing `Nx Unicode project transport` tests cover overriding an explicitly true caller setting and building the actual describe graph. Direct `bun x nx` invocations bypass the root launcher; this review used some such diagnostic commands before identifying the existing boundary.

A fresh ticket-local workspace-data directory with plugin isolation explicitly disabled and project-graph cache disabled successfully resolved the sole correct renderer project through canonical `bun nx`. Evidence: `🗑️generated/metabolism-glb/nx-unicode-diagnostic.log`. This was a bounded diagnostic, not a new permanent cache policy; existing cache contents were preserved. The default-cache canonical `bun nx show project @semio-tech/framework-renderer-wgpu --json` then also succeeded with the exact correct root (`nx-unicode-default-cache.log`), followed by successful canonical `@semio-tech/framework-os:generate-wgpu` (six exact artifacts, one changed).

For bounded `nx exec` diagnostics, `bun nx exec` is intercepted as the root package's `nx` lifecycle script and Nx incorrectly seeks a workspace target called `nx`. The working diagnostic invocation is `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- …`; the explicit environment retains the same in-process transport policy. Normal registered targets continue through `bun nx run`.

No dependency patch, new compatibility layer, cleanup, Git mutation, or production source change was made during this diagnosis. Future verification in this review uses the canonical repository launcher, preserving its already established in-process plugin policy.
