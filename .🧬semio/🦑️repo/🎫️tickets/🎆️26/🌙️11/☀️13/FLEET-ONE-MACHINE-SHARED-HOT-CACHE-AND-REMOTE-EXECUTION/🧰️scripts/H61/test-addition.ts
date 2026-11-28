test("a bundle refused for a peer edit is built again inside the command from a fresh capture and the refusal names the moved source",()=>{
 const text=readFileSync(join(import.meta.dir,"..","..","🟦️.ts"),"utf8"),ready=text.slice(text.indexOf("const ready=async"),text.indexOf("if (present) return ready"));
 expect(ready).toContain("retryOnConcurrentEditV1(\"build\"");expect(ready).toContain("attempt===1?{authorizerFiles,authorizer,compiler}:await captureNxAuthorizerPairV1(");
 const body=text.slice(text.indexOf("async function ensureNxAuthorizerBundleV1"));
 expect(body).toContain("nxAuthorizerSourcesMovedV1(root,sources,signal)");expect(body).toContain("concurrentEditV1(\"bytes\",moved,");
 expect(text).not.toContain("concurrentEditV1(\"bytes\",sources.manifest.entry");
});
