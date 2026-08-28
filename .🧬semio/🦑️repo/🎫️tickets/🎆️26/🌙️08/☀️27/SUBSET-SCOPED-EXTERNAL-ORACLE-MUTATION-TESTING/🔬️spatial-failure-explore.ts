#!/usr/bin/env bun
// Scratch exploration for the spatial-relationship / failure fixture additions.
// NOT part of the generator. Run: bun 🔬️spatial-failure-explore.ts
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

function tryIt(label: string, fn: () => unknown): unknown {
  try {
    const r = fn();
    console.log(`OK   ${label}`);
    return r;
  } catch (e) {
    console.log(`FAIL ${label} -- ${(e as Error).message}`);
    return undefined;
  }
}

await (b as any).init();

console.log("=== open shell (5 of 6 faces, no cap) ===");
{
  const box = call("box", 10, 10, 10) as unknown;
  const faces = (b as any).getFaces(box) as unknown[];
  console.log("box faces:", faces.length);
  const openShell = tryIt("sewShells(5 faces)", () => call("sewShells", faces.slice(0, 5)));
  if (openShell !== undefined) {
    tryIt("isValidSolid(openShell)", () => (b as any).isValidSolid(openShell));
    console.log("isValid(openShell) =", (b as any).isValid(openShell));
    const other = call("translate", call("box", 10, 10, 10), [5, 5, 5]);
    tryIt("cut(openShell, other)", () => call("cut", openShell, other));
    tryIt("fuse(openShell, other)", () => call("fuse", openShell, other));
    tryIt("intersect(openShell, other)", () => call("intersect", openShell, other));
    tryIt("getSolids(openShell)", () => (b as any).getSolids(openShell));
    tryIt("exportSTEP(openShell)", () => (b as any).exportSTEP(openShell));
  }
}

console.log("\n=== self-intersecting bowtie profile -> solid ===");
{
  // bowtie quad: edges cross in the middle
  const p1: [number, number, number] = [0, 0, 0];
  const p2: [number, number, number] = [10, 10, 0];
  const p3: [number, number, number] = [10, 0, 0];
  const p4: [number, number, number] = [0, 10, 0];
  const e1 = call("line", p1, p2);
  const e2 = call("line", p2, p3);
  const e3 = call("line", p3, p4);
  const e4 = call("line", p4, p1);
  const w = tryIt("wireLoop(bowtie)", () => call("wireLoop", [e1, e2, e3, e4]));
  if (w !== undefined) {
    const f = tryIt("face(bowtie wire)", () => call("face", w));
    if (f !== undefined) {
      const solid = tryIt("thicken(bowtie face, 5)", () => call("thicken", f, 5));
      if (solid !== undefined) {
        console.log("isValid(bowtie solid) =", (b as any).isValid(solid));
        tryIt("isValidSolid(bowtie solid)", () => (b as any).isValidSolid(solid));
        const other = call("box", 20, 20, 20);
        tryIt("cut(other, bowtieSolid)", () => call("cut", other, solid));
      }
    }
  }
  // alternative: polygon() with same self-crossing points
  tryIt("polygon(bowtie points)", () => call("polygon", [p1, p2, p3, p4]));
}

console.log("\n=== self-intersecting via overlapping-box weld (solid() from crossing faces) ===");
{
  const a = call("box", 10, 10, 10);
  const bb = call("translate", call("box", 10, 10, 10), [5, 5, 5]);
  const facesA = (b as any).getFaces(a) as unknown[];
  const facesB = (b as any).getFaces(bb) as unknown[];
  tryIt("solid(facesA+facesB interpenetrating)", () => call("solid", [...facesA, ...facesB]));
}

console.log("\n=== non-manifold compound touching at a single vertex ===");
{
  const a = call("box", 10, 10, 10);
  const bb = call("translate", call("box", 10, 10, 10), [10, 10, 10]); // touches a at (10,10,10) only
  const comp = tryIt("compound([a,b])", () => call("compound", [a, bb]));
  if (comp !== undefined) {
    console.log("isValid(compound) =", (b as any).isValid(comp));
    tryIt("getSolids(compound)", () => (b as any).getSolids(comp));
    const third = call("translate", call("box", 5, 5, 5), [100, 100, 100]);
    tryIt("cut(compound, third)", () => call("cut", comp, third));
    tryIt("fuse(compound, third)", () => call("fuse", comp, third));
    tryIt("intersect(compound, third)", () => call("intersect", comp, third));
    tryIt("exportSTEP(compound)", () => (b as any).exportSTEP(comp));
  }
}

console.log("\n=== invalid input: degenerate / negative params ===");
tryIt("box(0,10,10)", () => call("box", 0, 10, 10));
tryIt("box(-5,10,10)", () => call("box", -5, 10, 10));
tryIt("cylinder(-5,10)", () => call("cylinder", -5, 10));
tryIt("cylinder(5,0)", () => call("cylinder", 5, 0));
tryIt("ellipse(5,10)", () => call("ellipse", 5, 10));

console.log("\n=== vertex-touching boxes (for spatial-relationship) ===");
{
  const a = call("box", 10, 10, 10);
  const bb = call("translate", call("box", 10, 10, 10), [10, 10, 10]);
  const fused = tryIt("fuse(vertex-touching)", () => call("fuse", a, bb));
  if (fused !== undefined) {
    console.log("solids:", ((b as any).getSolids(fused) as unknown[]).length, "volume:", (b as any).measureVolume(fused));
  }
  tryIt("cut(vertex-touching)", () => call("cut", a, bb));
  tryIt("intersect(vertex-touching)", () => call("intersect", a, bb));
}
