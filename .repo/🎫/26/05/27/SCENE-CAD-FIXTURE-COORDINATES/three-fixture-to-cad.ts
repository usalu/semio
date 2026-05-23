/** One-off: convert elements.scene.fixture/v1 vectors from legacy Three Y-up to CAD Z-up. */
import { readFileSync, writeFileSync } from "node:fs";
import { Euler, Matrix4, Quaternion, Vector3 } from "three";

const eulerCadToThree = new Euler(-Math.PI / 2, 0, 0, "XYZ");
const mCadToThree = new Matrix4().makeRotationFromEuler(eulerCadToThree);
const mThreeToCad = mCadToThree.clone().invert();
const qCadToThree = new Quaternion().setFromRotationMatrix(mCadToThree);

function threeVec3ToCad(v: readonly number[]): [number, number, number] {
	const out = new Vector3(v[0], v[1], v[2]).applyMatrix4(mThreeToCad);
	return [out.x, out.y, out.z];
}

function threeQuatToCad(q: readonly number[]): [number, number, number, number] {
	const qt = new Quaternion(q[0], q[1], q[2], q[3]);
	const out = qCadToThree.clone().invert().multiply(qt);
	return [out.x, out.y, out.z, out.w];
}

function walk(value: unknown, key?: string): unknown {
	if (Array.isArray(value)) {
		if (value.length === 3 && value.every((n) => typeof n === "number") && isVec3Key(key)) {
			return threeVec3ToCad(value);
		}
		if (value.length === 4 && value.every((n) => typeof n === "number") && key === "orientation") {
			return threeQuatToCad(value);
		}
		return value.map((item) => walk(item, key));
	}
	if (value && typeof value === "object") {
		const out: Record<string, unknown> = {};
		for (const [k, v] of Object.entries(value)) {
			out[k] = walk(v, k);
		}
		return out;
	}
	return value;
}

function isVec3Key(key: string | undefined): boolean {
	return key === "position" || key === "origin" || key === "target" || key === "size";
}

const target = process.argv[2];
if (!target) {
	console.error("usage: bun three-fixture-to-cad.ts <fixture.json>");
	process.exit(1);
}
const raw = JSON.parse(readFileSync(target, "utf8")) as unknown;
const converted = walk(raw) as Record<string, unknown>;
writeFileSync(target, `${JSON.stringify(converted, null, 2)}\n`, "utf8");
console.log(`converted ${target}`);
