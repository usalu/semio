#!/usr/bin/env bun
import * as b from "brepjs";

function unwrap(v: unknown, what: string): unknown {
  if (v !== null && typeof v === "object" && "ok" in (v as Record<string, unknown>)) {
    const r = v as { ok: boolean; value?: unknown; error?: unknown };
    if (!r.ok) throw new Error(`${what}: ${JSON.stringify(r.error)}`);
    return r.value;
  }
  return v;
}
const call = (name: string, ...args: unknown[]): unknown => unwrap((b as any)[name](...args), name);

function describeErr(e: unknown): string {
  if (e instanceof Error) return `Error(${e.constructor.name}): ${e.message}`;
  const proto = Object.getPrototypeOf(e);
  const keys = (() => { try { return Object.getOwnPropertyNames(e as object); } catch { return []; } })();
  const str = (() => { try { return String(e); } catch { return "?"; } })();
  return `nonError proto=${proto?.constructor?.name ?? proto} keys=${JSON.stringify(keys)} String()=${str}`;
}

function tryIt<T>(label: string, fn: () => T): T | undefined {
  try {
    const r = fn();
    console.log(`OK   ${label}`);
    return r;
  } catch (e) {
    console.log(`FAIL ${label} -- ${describeErr(e)}`);
    return undefined;
  }
}

function measure(label: string, shape: unknown): void {
  try {
    const solids = (call("getSolids", shape) as unknown[]).length;
    const vol = solids > 0 ? call("measureVolume", shape) : 0;
    const faces = (call("getFaces", shape) as unknown[]).length;
    const edges = (call("getEdges", shape) as unknown[]).length;
    const verts = (call("getVertices", shape) as unknown[]).length;
    const valid = call("isValidSolid", shape);
    console.log(`  MEASURE ${label}: solids=${solids} volume=${vol} faces=${faces} edges=${edges} vertices=${verts} validSolid=${valid}`);
  } catch (e) {
    console.log(`  MEASURE ${label} FAILED -- ${describeErr(e)}`);
  }
}

await (b as any).init();

console.log("=== error introspection ===");
tryIt("box(0,10,10)", () => call("box", 0, 10, 10));
tryIt("cylinder(-5,10)", () => call("cylinder", -5, 10));

console.log("\n=== tangential contact: sphere tangent to box face, fuse/intersect ===");
{
  // box 20x20x20 at origin, sphere radius 5 centered so it touches the top face (z=20) tangentially
  const box = call("box", 20, 20, 20);
  const ball = call("translate", call("sphere", 5), [10, 10, 25]); // sphere center z=25, radius5 -> bottom at z=20, tangent
  const fuseRes = tryIt("fuse(box, tangentSphere)", () => call("fuse", box, ball));
  if (fuseRes !== undefined) measure("fuse tangent sphere", fuseRes);
  const interRes = tryIt("intersect(box, tangentSphere)", () => call("intersect", box, ball));
  if (interRes !== undefined) measure("intersect tangent sphere", interRes);
  const cutRes = tryIt("cut(box, tangentSphere)", () => call("cut", box, ball));
  if (cutRes !== undefined) measure("cut tangent sphere", cutRes);
}

console.log("\n=== coincident-faces: boxes sharing coincident side planes, partial z overlap ===");
{
  // A: x[0,10] y[0,10] z[0,10]; B: x[0,10] y[0,10] z[5,15] -- all 4 side planes coincide, overlap z[5,10]
  const a = call("box", 10, 10, 10);
  const bb = call("translate", call("box", 10, 10, 10), [0, 0, 5]);
  const cutRes = tryIt("cut(coincident-faces)", () => call("cut", a, bb));
  if (cutRes !== undefined) measure("cut coincident-faces", cutRes);
  const fuseRes = tryIt("fuse(coincident-faces)", () => call("fuse", a, bb));
  if (fuseRes !== undefined) measure("fuse coincident-faces", fuseRes);
  const interRes = tryIt("intersect(coincident-faces)", () => call("intersect", a, bb));
  if (interRes !== undefined) measure("intersect coincident-faces", interRes);
}

console.log("\n=== identical operands: fuse/intersect ===");
{
  const a = call("box", 12, 12, 12);
  const same = call("box", 12, 12, 12);
  const fuseRes = tryIt("fuse(identical)", () => call("fuse", a, same));
  if (fuseRes !== undefined) measure("fuse identical", fuseRes);
  const a2 = call("box", 12, 12, 12);
  const same2 = call("box", 12, 12, 12);
  const interRes = tryIt("intersect(identical)", () => call("intersect", a2, same2));
  if (interRes !== undefined) measure("intersect identical", interRes);
}

console.log("\n=== nearly-identical operands (epsilon larger in one dim): cut/fuse/intersect ===");
{
  const a = call("box", 12, 12, 12);
  const nearly = call("box", 12, 12, 12.001);
  const cutRes = tryIt("cut(nearly-identical)", () => call("cut", a, nearly));
  if (cutRes !== undefined) measure("cut nearly-identical", cutRes);
  const a2 = call("box", 12, 12, 12);
  const nearly2 = call("box", 12, 12, 12.001);
  const fuseRes = tryIt("fuse(nearly-identical)", () => call("fuse", a2, nearly2));
  if (fuseRes !== undefined) measure("fuse nearly-identical", fuseRes);
  const a3 = call("box", 12, 12, 12);
  const nearly3 = call("box", 12, 12, 12.001);
  const interRes = tryIt("intersect(nearly-identical)", () => call("intersect", a3, nearly3));
  if (interRes !== undefined) measure("intersect nearly-identical", interRes);
}

console.log("\n=== contained operand: fuse/intersect ===");
{
  const outer = call("box", 20, 20, 20);
  const inner = call("translate", call("box", 6, 6, 6), [7, 7, 7]);
  const fuseRes = tryIt("fuse(contained)", () => call("fuse", outer, inner));
  if (fuseRes !== undefined) measure("fuse contained", fuseRes);
  const outer2 = call("box", 20, 20, 20);
  const inner2 = call("translate", call("box", 6, 6, 6), [7, 7, 7]);
  const interRes = tryIt("intersect(contained)", () => call("intersect", outer2, inner2));
  if (interRes !== undefined) measure("intersect contained", interRes);
}

console.log("\n=== disjoint: fuse ===");
{
  const a = call("box", 10, 10, 10);
  const away = call("translate", call("box", 10, 10, 10), [100, 0, 0]);
  const fuseRes = tryIt("fuse(disjoint)", () => call("fuse", a, away));
  if (fuseRes !== undefined) measure("fuse disjoint", fuseRes);
}

console.log("\n=== partial overlap: fuse ===");
{
  const left = call("box", 10, 10, 10);
  const right = call("translate", call("box", 10, 10, 10), [5, 5, 5]);
  const fuseRes = tryIt("fuse(partial-overlap)", () => call("fuse", left, right));
  if (fuseRes !== undefined) measure("fuse partial-overlap", fuseRes);
}

console.log("\n=== coplanar cutter: fuse/intersect variant ===");
{
  // base box z[0,20]; second box z[0,10] but bigger footprint x[-5,25] y[-5,25] -- bottom faces coincide (z=0)
  const box = call("box", 20, 20, 20);
  const slab = call("translate", call("box", 30, 30, 10), [-5, -5, 0]);
  const fuseRes = tryIt("fuse(coplanar-cutter)", () => call("fuse", box, slab));
  if (fuseRes !== undefined) measure("fuse coplanar-cutter", fuseRes);
  const box2 = call("box", 20, 20, 20);
  const slab2 = call("translate", call("box", 30, 30, 10), [-5, -5, 0]);
  const interRes = tryIt("intersect(coplanar-cutter)", () => call("intersect", box2, slab2));
  if (interRes !== undefined) measure("intersect coplanar-cutter", interRes);
}

console.log("\n=== splits into several bodies: intersect (comb tool) ===");
{
  const box = call("box", 30, 30, 10);
  // comb: three separate teeth as one compound tool, each overlapping the box footprint
  const tooth1 = call("translate", call("box", 5, 30, 20), [0, 0, -5]);
  const tooth2 = call("translate", call("box", 5, 30, 20), [12, 0, -5]);
  const tooth3 = call("translate", call("box", 5, 30, 20), [24, 0, -5]);
  const comb = call("compound", [tooth1, tooth2, tooth3]);
  const interRes = tryIt("intersect(box, comb)", () => call("intersect", box, comb));
  if (interRes !== undefined) measure("intersect splits result", interRes);
}
