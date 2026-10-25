const l = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts");
const { renderRepoErrorV1 } = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts");
for (const options of [{ resource: "", mode: "shared" }, { resource: "x", mode: "weird" }, { resource: "x", mode: "shared", timeoutMs: -1 }]) {
  try { await l.acquireResourceLease({ directory: "/tmp/lang-d-lease", signal: new AbortController().signal, ...options } as any); console.log("acquired?"); }
  catch (error) { console.log((error as Error).name, "|", renderRepoErrorV1(error as any, "en"), "|", renderRepoErrorV1(error as any, "de")); }
}
