const m = await import(process.argv[2]);
const lanes: any = new Proxy({}, { get: () => () => 0 });
const t = Date.now();
const r = await m.toolJobCoverageRun(process.cwd(), lanes);
console.log("[CENSUS] ms", Date.now() - t, "failures", r.failures.length);
for (const f of r.failures) console.log("[FAILURE]", f.slice(0, 400));
console.log("[CENSUS] remaining", r.remainingCommands.length, "gates", JSON.stringify(r.pipelineGates));
