/** 🔮️ OCCT (brepjs-opencascade) reference for generation3d `sphere-box-fuse`: faces, volume, and mesh triangle counts per tolerance. */
const b = (await import("brepjs")) as unknown as Record<string, (...args: unknown[]) => unknown>;
await (b.init as () => Promise<void>)();
const unwrap = (value: unknown): unknown => (value !== null && typeof value === "object" && "ok" in (value as object) ? ((value as { ok: boolean; value?: unknown; error?: unknown }).ok ? (value as { value: unknown }).value : (() => { throw new Error(JSON.stringify((value as { error: unknown }).error)); })()) : value);
const call = (name: string, ...args: unknown[]): unknown => unwrap(b[name](...args));
const sphere = call("sphere", 1.2);
const box = call("box", 1.5, 1.5, 1.5);
const fused = call("fuse", sphere, box);
const faces = call("getFaces", fused) as unknown[];
const surfaceKinds = faces.map((face) => { try { return String(call("getSurfaceType", face)); } catch { return "?"; } });
console.log(JSON.stringify({ faces: faces.length, surfaceKinds, volume: call("measureVolume", fused) }));
for (const [tolerance, angularTolerance] of [[0.05, 0.5], [0.05, 0.3], [0.05, 0.1], [0.0025, 0.5]] as const) {
  const mesh = call("mesh", fused, { tolerance, angularTolerance }) as { triangles: ArrayLike<number>; vertices: ArrayLike<number> };
  console.log(JSON.stringify({ tolerance, angularTolerance, triangles: mesh.triangles.length / 3, vertices: mesh.vertices.length / 3 }));
}
