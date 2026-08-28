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
  if (e instanceof Error) return `${e.constructor.name}: ${e.message}`;
  try { return `nonError: ${JSON.stringify(e)}`; } catch { return `nonError(unstringifiable): ${String(e)}`; }
}

function tryIt<T>(label: string, fn: () => T): T | undefined {
  try {
    const r = fn();
    console.log(`OK   ${label} =>`, typeof r === "object" ? "[obj]" : r);
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

console.log("=== degenerate params, precise errors ===");
tryIt("box(0,10,10)", () => call("box", 0, 10, 10));
tryIt("box(-5,10,10)", () => call("box", -5, 10, 10));
tryIt("cylinder(-5,10)", () => call("cylinder", -5, 10));
tryIt("cylinder(5,0)", () => call("cylinder", 5, 0));
tryIt("cylinder(5,-10)", () => call("cylinder", 5, -10));
{
  const neg = tryIt("box(-5,10,10) full", () => call("box", -5, 10, 10));
  if (neg !== undefined) measure("negative-width box", neg);
}
{
  const zeroH = tryIt("cylinder(5,0) full", () => call("cylinder", 5, 0));
  if (zeroH !== undefined) measure("zero-height cylinder", zeroH);
}

console.log("\n=== open shell: fuse failure detail ===");
{
  const box = call("box", 10, 10, 10);
  const faces = call("getFaces", box) as unknown[];
  const openShell = call("sewShells", faces.slice(0, 5));
  console.log("openShell isValid:", call("isValid", openShell), "isValidSolid:", (() => { try { return call("isValidSolid", openShell); } catch (e) { return describeErr(e); } })());
  measure("openShell alone", openShell);
  const other = call("translate", call("box", 10, 10, 10), [5, 5, 5]);
  const cutRes = tryIt("cut(openShell, other)", () => call("cut", openShell, other));
  if (cutRes !== undefined) measure("cut(openShell,other) result", cutRes);
  const interRes = tryIt("intersect(openShell, other)", () => call("intersect", openShell, other));
  if (interRes !== undefined) measure("intersect(openShell,other) result", interRes);
  tryIt("fuse(openShell, other) -- expect fail", () => call("fuse", openShell, other));
}

console.log("\n=== self-intersecting bowtie solid: detail ===");
{
  const p1: [number, number, number] = [0, 0, 0];
  const p2: [number, number, number] = [10, 10, 0];
  const p3: [number, number, number] = [10, 0, 0];
  const p4: [number, number, number] = [0, 10, 0];
  const e1 = call("line", p1, p2), e2 = call("line", p2, p3), e3 = call("line", p3, p4), e4 = call("line", p4, p1);
  const w = call("wireLoop", [e1, e2, e3, e4]);
  const f = call("face", w);
  const solid = call("thicken", f, 5);
  console.log("bowtie solid isValid:", call("isValid", solid));
  console.log("bowtie solid isValidSolid:", (() => { try { return call("isValidSolid", solid); } catch (e) { return describeErr(e); } })());
  measure("bowtie solid alone", solid);
  const box = call("box", 20, 20, 20);
  const cutRes = tryIt("cut(box, bowtieSolid)", () => call("cut", box, solid));
  if (cutRes !== undefined) measure("cut(box,bowtieSolid) result", cutRes);
  const fuseRes = tryIt("fuse(box, bowtieSolid)", () => call("fuse", box, solid));
  if (fuseRes !== undefined) measure("fuse(box,bowtieSolid) result", fuseRes);
  const interRes = tryIt("intersect(box, bowtieSolid)", () => call("intersect", box, solid));
  if (interRes !== undefined) measure("intersect(box,bowtieSolid) result", interRes);
}

console.log("\n=== non-manifold vertex-touching compound: detail ===");
{
  const a = call("box", 10, 10, 10);
  const bb = call("translate", call("box", 10, 10, 10), [10, 10, 10]);
  const comp = call("compound", [a, bb]);
  measure("compound alone", comp);
  const third = call("translate", call("box", 5, 5, 5), [100, 100, 100]);
  const cutRes = tryIt("cut(compound, third)", () => call("cut", comp, third));
  if (cutRes !== undefined) measure("cut(compound,third) result", cutRes);
  const fuseRes = tryIt("fuse(compound, third)", () => call("fuse", comp, third));
  if (fuseRes !== undefined) measure("fuse(compound,third) result", fuseRes);
  const interRes = tryIt("intersect(compound, third)", () => call("intersect", comp, third));
  if (interRes !== undefined) measure("intersect(compound,third) result", interRes);

  // compound boxes vertex-touching fused directly with each other via boolean, for comparison
  const fusedDirect = tryIt("fuse(a,b) vertex-touching direct", () => call("fuse", a, bb));
  if (fusedDirect !== undefined) measure("fuse(a,b) vertex-touching direct result", fusedDirect);
  const cutDirect = tryIt("cut(a,b) vertex-touching direct", () => call("cut", a, bb));
  if (cutDirect !== undefined) measure("cut(a,b) vertex-touching direct result", cutDirect);
  const interDirect = tryIt("intersect(a,b) vertex-touching direct", () => call("intersect", a, bb));
  if (interDirect !== undefined) measure("intersect(a,b) vertex-touching direct result", interDirect);
}

console.log("\n=== edge-touching boxes: cut/intersect (for spatial-relationship coverage) ===");
{
  const a = call("box", 10, 10, 10);
  const diag = call("translate", call("box", 10, 10, 10), [10, 10, 0]);
  const cutRes = tryIt("cut(edge-touching)", () => call("cut", a, diag));
  if (cutRes !== undefined) measure("cut(edge-touching) result", cutRes);
  const interRes = tryIt("intersect(edge-touching)", () => call("intersect", a, diag));
  if (interRes !== undefined) measure("intersect(edge-touching) result", interRes);
}

console.log("\n=== face-touching boxes: cut/intersect ===");
{
  const a = call("box", 10, 10, 10);
  const right = call("translate", call("box", 10, 10, 10), [10, 0, 0]);
  const cutRes = tryIt("cut(face-touching)", () => call("cut", a, right));
  if (cutRes !== undefined) measure("cut(face-touching) result", cutRes);
  const interRes = tryIt("intersect(face-touching)", () => call("intersect", a, right));
  if (interRes !== undefined) measure("intersect(face-touching) result", interRes);
}
