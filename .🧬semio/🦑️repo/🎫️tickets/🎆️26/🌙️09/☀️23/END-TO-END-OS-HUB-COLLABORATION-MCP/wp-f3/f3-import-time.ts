/** ⏱️ F3 — wall time to import the os-dev script router (no command run). */
const t0 = performance.now();
await import(process.argv[2]!);
console.log(`[f3-import] ${process.argv[2]} imported in ${(performance.now() - t0).toFixed(0)} ms`);
