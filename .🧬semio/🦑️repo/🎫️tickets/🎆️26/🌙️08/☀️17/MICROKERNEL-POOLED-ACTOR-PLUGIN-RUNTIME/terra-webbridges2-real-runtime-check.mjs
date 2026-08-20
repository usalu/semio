// [DEBUG] terra-web-bridges (re-run 2): actually IMPORTS and CALLS the real generated bridge.js +
// host-shim.js against the real transpiled semio_framework_os_scale_fixture component (production
// namespace semio:framework/*, the genuinely collapsed `world actor`) — not just a syntax check.
// Bun ships JSPI natively (confirmed by terra-jco-spike-report.md), so this is a real component call,
// not a mock.
globalThis.self = globalThis; // host-shim.js's effectRequest/postFireAndForget expect a worker-like `self`; unused here since this fixture imports no host-async, but harmless to provide.
const { createActorApi } = await import("./terra-webbridges2-fullpipeline-out/scalefixture2/bridge.js");
console.log("[DEBUG] bridge.js imported OK, createActorApi =", typeof createActorApi);
const api = await createActorApi("terra-webbridges2-test-actor");
console.log("[DEBUG] createActorApi() resolved OK, api keys =", Object.keys(api).sort().join(","));
const result = await api.describe();
console.log("[DEBUG] api.describe() resolved OK, byteLength =", result?.byteLength, "isUint8Array =", result instanceof Uint8Array);
console.log("ALL RUNTIME CHECKS PASS");
