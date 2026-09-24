/** 🔮️ OCCT (brepjs-opencascade) reference for generation3d `sphere-cut-with-torus` at the tangent radius r = major + minor = 2.5. */
const b = (await import("brepjs")) as unknown as Record<string, (...args: unknown[]) => unknown>;
await (b.init as () => Promise<void>)();
const unwrap = (value: unknown): unknown => (value !== null && typeof value === "object" && "ok" in (value as object) ? ((value as { ok: boolean; value?: unknown; error?: unknown }).ok ? (value as { value: unknown }).value : (() => { throw new Error(JSON.stringify((value as { error: unknown }).error)); })()) : value);
const call = (name: string, ...args: unknown[]): unknown => unwrap(b[name](...args));
for (const radius of [2.5, 3.0]) {
  try {
    const cut = call("cut", call("sphere", radius), call("torus", 2.0, 0.5));
    const faces = call("getFaces", cut) as unknown[];
    console.log(JSON.stringify({ radius, faces: faces.length, volume: call("measureVolume", cut), expectedCavity: (4 / 3) * Math.PI * radius ** 3 - 2 * Math.PI ** 2 * 2.0 * 0.25 }));
  } catch (error) {
    console.log(JSON.stringify({ radius, error: String(error) }));
  }
}
