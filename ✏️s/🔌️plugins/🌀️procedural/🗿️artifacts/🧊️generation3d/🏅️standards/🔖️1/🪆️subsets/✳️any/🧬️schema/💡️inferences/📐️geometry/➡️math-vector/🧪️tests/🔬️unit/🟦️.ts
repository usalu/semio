import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { Plane, Vector3 } from "three";
import { disagreements, type FixtureCase, type OracleResult } from "../../../🧪️tests/🧰️oracle-support/🟦️.ts";

type Triple = [number, number, number];
type Fixture = { tolerance: number; angleTolerance: number; cases: FixtureCase[] };
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as Fixture;
const prefix = "generation3d.geometry.";

const vec = (inputs: Record<string, unknown>, port: string): Vector3 => new Vector3(...(inputs[port] as Triple));
const triple = (v: Vector3): Triple => [v.x, v.y, v.z];
const overflowing = (v: Vector3): boolean => ![v.x, v.y, v.z].every(Number.isFinite);
const finite = (outputs: Record<string, unknown>, vectors: Vector3[]): OracleResult => (vectors.some(overflowing) ? { fault: { code: `${prefix}math-overflow` } } : { outputs });
const zero = (port: string): OracleResult => ({ fault: { code: `${prefix}zero-vector`, port } });

/** ➡️ Recomputes one vector case with three's Vector3 and Plane. */
const oracle = ({ kind, inputs }: FixtureCase): OracleResult => {
  switch (kind) {
    case "math.vectorFromComponents": return { outputs: { vector: [inputs.x, inputs.y, inputs.z] } };
    case "math.pointFromComponents": return { outputs: { point: [inputs.x, inputs.y, inputs.z] } };
    case "math.vectorComponents": { const v = vec(inputs, "vector"); return { outputs: { x: v.x, y: v.y, z: v.z } }; }
    case "math.pointComponents": { const v = vec(inputs, "point"); return { outputs: { x: v.x, y: v.y, z: v.z } }; }
    case "math.vectorAdd": { const v = vec(inputs, "a").add(vec(inputs, "b")); return finite({ vector: triple(v) }, [v]); }
    case "math.vectorSubtract": { const v = vec(inputs, "a").sub(vec(inputs, "b")); return finite({ vector: triple(v) }, [v]); }
    case "math.vectorScale": { const v = vec(inputs, "vector").multiplyScalar(inputs.factor as number); return finite({ vector: triple(v) }, [v]); }
    case "math.vectorLength": return { outputs: { length: vec(inputs, "vector").length() } };
    case "math.vectorNormalize": { const v = vec(inputs, "vector"); return v.lengthSq() === 0 ? zero("vector") : { outputs: { vector: triple(v.normalize()) } }; }
    case "math.vectorDot": return { outputs: { result: vec(inputs, "a").dot(vec(inputs, "b")) } };
    case "math.vectorCross": { const v = vec(inputs, "a").cross(vec(inputs, "b")); return finite({ vector: triple(v) }, [v]); }
    case "math.vectorAngle": {
      const [a, b] = [vec(inputs, "a"), vec(inputs, "b")];
      return a.lengthSq() === 0 ? zero("a") : b.lengthSq() === 0 ? zero("b") : { outputs: { angle: a.angleTo(b) } };
    }
    case "math.pointOffset": { const v = vec(inputs, "point").add(vec(inputs, "offset")); return finite({ point: triple(v) }, [v]); }
    case "math.pointDistance": return { outputs: { distance: vec(inputs, "a").distanceTo(vec(inputs, "b")) } };
    case "math.pointInterpolate": { const v = new Vector3().lerpVectors(vec(inputs, "a"), vec(inputs, "b"), inputs.t as number); return finite({ point: triple(v) }, [v]); }
    case "math.vectorBetween": { const v = vec(inputs, "to").sub(vec(inputs, "from")); return finite({ vector: triple(v) }, [v]); }
    case "math.planeFromPointNormal": {
      const [origin, normal] = [vec(inputs, "origin"), vec(inputs, "normal")];
      return normal.lengthSq() === 0 ? zero("normal") : { outputs: { plane: { origin: triple(origin), normal: triple(new Plane().setFromNormalAndCoplanarPoint(normal.clone().normalize(), origin).normal) } } };
    }
    case "math.planeFromPoints": {
      const [a, b, c] = [vec(inputs, "a"), vec(inputs, "b"), vec(inputs, "c")];
      const twiceArea = new Vector3().subVectors(b, a).cross(new Vector3().subVectors(c, a)).length();
      if (twiceArea <= 1e-12 * a.distanceTo(b) * a.distanceTo(c) || twiceArea === 0) return { fault: { code: `${prefix}plane-degenerate`, port: "c" } };
      return { outputs: { plane: { origin: triple(a), normal: triple(new Plane().setFromCoplanarPoints(a, b, c).normal) } } };
    }
    case "math.planeComponents": { const plane = inputs.plane as { origin: Triple; normal: Triple }; return { outputs: { origin: plane.origin, normal: plane.normal } }; }
    default: throw new Error(`no oracle for ${kind}`);
  }
};

test("every math.vector kind has a fixture case and three reproduces every fixture outcome", () => {
  const kinds = new Set(fixture.cases.map(fixtureCase => fixtureCase.kind));
  expect(kinds.size).toBe(19);
  const cases = fixture.cases.map(fixtureCase => (fixtureCase.kind === "math.vectorAngle" ? { ...fixtureCase, tolerance: fixture.angleTolerance } : fixtureCase));
  expect(disagreements(cases, oracle, fixture.tolerance)).toEqual([]);
});
