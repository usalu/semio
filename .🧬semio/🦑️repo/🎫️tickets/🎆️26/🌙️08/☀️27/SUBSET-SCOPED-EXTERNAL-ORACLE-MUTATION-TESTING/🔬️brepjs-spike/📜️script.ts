// 🔬️ brepjs qualification spike. Answers, with evidence rather than documentation: can this package
// build exact BRep solids, apply Booleans, export STEP, tessellate at a declared tolerance and
// measure volume — on THIS machine, offline, from the version already in node_modules?
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const out = process.env.SEMIO_SPIKE_OUT ?? join(import.meta.dir, "📤️out");
mkdirSync(out, { recursive: true });

const checks: { id: string; met: boolean; detail: string }[] = [];
const record = (id: string, met: boolean, detail: string) => {
  checks.push({ id, met, detail });
  console.log(`${met ? "✔" : "✘"} ${id} — ${detail}`);
};

const b = (await import("brepjs")) as any;

try {
  await b.init();
  record("kernel-init", true, `kernel tier ${JSON.stringify(b.getKernelTier?.() ?? "unknown")}`);
} catch (error) {
  record("kernel-init", false, String(error));
}

try {
  const caps = b.getKernelCapabilities?.();
  record("kernel-capabilities", true, JSON.stringify(caps));
} catch (error) {
  record("kernel-capabilities", false, String(error));
}

const unwrap = (value: any, what: string): any => {
  if (value && typeof value === "object" && "ok" in value) {
    if (value.ok === false) throw new Error(`${what}: ${JSON.stringify(value.error ?? value)}`);
    return value.value;
  }
  return value;
};

let cutSolid: any = null;
try {
  // 📐️`box(dx, dy, dz)` sits CORNER-at-origin while `cylinder(r, h)` sits AXIS-at-origin, so the bore
  // has to be moved to the box's centre — a cylinder left at the origin is only a quarter inside it.
  const a = unwrap(b.box(20, 20, 20), "box a");
  const c = unwrap(b.cylinder(5, 40), "cylinder");
  const moved = unwrap(b.translate(c, [10, 10, -10]), "translate");
  cutSolid = unwrap(b.cut(a, moved), "cut");
  // 🧩️OCCT returns a COMPOUND from a Boolean even when it holds exactly one solid, so the solid
  // COUNT is the assertion — `isSolid` on the compound is false and says nothing about the result.
  record("boolean-cut", cutSolid !== null && b.getSolids(cutSolid).length === 1, `cut produced a compound holding ${b.getSolids(cutSolid).length} solid(s) (isSolid on the compound itself is ${b.isSolid(cutSolid)})`);
} catch (error) {
  record("boolean-cut", false, String(error));
}

try {
  const volume = unwrap(b.measureVolume(cutSolid), "measureVolume");
  const expected = 20 * 20 * 20 - Math.PI * 5 * 5 * 20;
  const relative = Math.abs(volume - expected) / expected;
  record("measure-volume", relative < 1e-9, `${volume} vs analytic ${expected} (relative error ${relative.toExponential(3)})`);
} catch (error) {
  record("measure-volume", false, String(error));
}

try {
  const area = unwrap(b.measureArea(cutSolid), "measureArea");
  record("measure-area", Number.isFinite(area) && area > 0, String(area));
} catch (error) {
  record("measure-area", false, String(error));
}

try {
  record("valid-solid", b.isValidSolid(cutSolid) === true, `isValidSolid=${b.isValidSolid(cutSolid)}`);
} catch (error) {
  record("valid-solid", false, String(error));
}

try {
  const step = unwrap(b.exportSTEP(cutSolid), "exportSTEP");
  const text = typeof step === "string" ? step : await (step as Blob).text();
  writeFileSync(join(out, "cut.step"), text);
  record("export-step", text.startsWith("ISO-10303-21"), `${text.length} bytes, header ${JSON.stringify(text.slice(0, 13))}`);
} catch (error) {
  record("export-step", false, String(error));
}

try {
  const step2 = unwrap(b.exportSTEP(cutSolid), "exportSTEP second");
  const text2 = typeof step2 === "string" ? step2 : await (step2 as Blob).text();
  const text1 = require("node:fs").readFileSync(join(out, "cut.step"), "utf8");
  const differing = text1 === text2 ? [] : text1.split("\n").map((line: string, index: number) => [index, line, text2.split("\n")[index]]).filter(([, a, c]: any) => a !== c).slice(0, 4);
  record("step-self-determinism", text1 === text2, text1 === text2 ? "two exports of the same shape are byte-identical" : `first differing lines: ${JSON.stringify(differing)}`);
} catch (error) {
  record("step-self-determinism", false, String(error));
}

try {
  const meshed = unwrap(b.mesh(cutSolid, { tolerance: 1e-3, angularTolerance: 0.1 }), "mesh");
  const vertices = meshed.vertices ?? meshed.positions;
  const triangles = meshed.triangles ?? meshed.indices;
  writeFileSync(join(out, "cut.mesh.json"), JSON.stringify({ vertexCount: vertices.length / 3, triangleCount: triangles.length / 3 }, null, 2));
  record("tessellate", vertices.length > 0 && triangles.length > 0, `${vertices.length / 3} vertices, ${triangles.length / 3} triangles at tolerance 1e-3`);
} catch (error) {
  record("tessellate", false, String(error));
}

try {
  // 📥️`importSTEP` takes the same Blob shape `exportSTEP` hands back, not the decoded text.
  const text = require("node:fs").readFileSync(join(out, "cut.step"), "utf8");
  const reimported = unwrap(b.importSTEP(new Blob([text])), "importSTEP");
  const shape = reimported instanceof Promise ? unwrap(await reimported) : reimported;
  const volume = unwrap(b.measureVolume(Array.isArray(shape) ? shape[0] : (shape.shape ?? shape)), "reimported volume");
  const original = unwrap(b.measureVolume(cutSolid), "original volume");
  const relative = Math.abs(volume - original) / original;
  record("step-round-trip", relative < 1e-9, `reimported volume ${volume} vs ${original} (relative ${relative.toExponential(3)})`);
} catch (error) {
  record("step-round-trip", false, String(error));
}

try {
  const bounds = unwrap(b.getBounds(cutSolid), "getBounds");
  record("bounding-box", true, JSON.stringify(bounds));
} catch (error) {
  record("bounding-box", false, String(error));
}

try {
  const faces = b.getFaces(cutSolid);
  const edges = b.getEdges(cutSolid);
  const solids = b.getSolids(cutSolid);
  record("topology-counts", true, `${solids.length} solid(s), ${faces.length} face(s), ${edges.length} edge(s)`);
} catch (error) {
  record("topology-counts", false, String(error));
}

const report = { probe: "brepjs-occt", checkedAt: new Date().toISOString(), package: "brepjs", checks };
writeFileSync(join(out, "📤️report.json"), `${JSON.stringify(report, null, 2)}\n`);
console.log(`\n${checks.filter((c) => c.met).length}/${checks.length} criteria met`);
