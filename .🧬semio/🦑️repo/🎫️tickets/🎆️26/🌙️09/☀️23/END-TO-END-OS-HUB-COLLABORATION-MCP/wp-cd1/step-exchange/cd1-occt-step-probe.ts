/**
 * 🔭️ CD1 OCCT STEP probe: imports STEP exchange files through OpenCascade (brepjs-opencascade) and reports what OCCT sees —
 * shape types, counts, volume, area, length, bounds. `bun cd1-occt-step-probe.ts <file.step|--design>`; `--design` writes the
 * planned AP214 exchange structure (solid + free face + free wire, metre, three shape_definition_representations) first.
 */
import * as B from "brepjs";
import initOpenCascade from "brepjs-opencascade";
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
const require = createRequire("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/package.json");
const wasm = require.resolve("brepjs-opencascade/src/brepjs_single.wasm");
const oc = await (initOpenCascade as any)({ locateFile: (p: string) => (p === "brepjs_single.wasm" ? wasm : p) });
(B as any).initFromOC(oc);

function design(variant: string): string {
  const lines: string[] = [];
  let id = 0;
  const e = (t: string) => { id += 1; lines.push(`#${id}=${t};`); return id; };
  const app = e("APPLICATION_CONTEXT('automotive design')");
  e(`APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#${app})`);
  const pc = e(`PRODUCT_CONTEXT('',#${app},'mechanical')`);
  const prod = e(`PRODUCT('semio','semio','',(#${pc}))`);
  const pdf = e(`PRODUCT_DEFINITION_FORMATION('','',#${prod})`);
  const pdc = e(`PRODUCT_DEFINITION_CONTEXT('part definition',#${app},'design')`);
  const pd = e(`PRODUCT_DEFINITION('design','',#${pdf},#${pdc})`);
  const pds = e(`PRODUCT_DEFINITION_SHAPE('','',#${pd})`);
  const len = e("(LENGTH_UNIT()NAMED_UNIT(*)SI_UNIT($,.METRE.))");
  const ang = e("(NAMED_UNIT(*)PLANE_ANGLE_UNIT()SI_UNIT($,.RADIAN.))");
  const sol = e("(NAMED_UNIT(*)SI_UNIT($,.STERADIAN.)SOLID_ANGLE_UNIT())");
  const unc = e(`UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-07),#${len},'distance_accuracy_value','confusion accuracy')`);
  const ctx = e(`(GEOMETRIC_REPRESENTATION_CONTEXT(3)GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#${unc}))GLOBAL_UNIT_ASSIGNED_CONTEXT((#${len},#${ang},#${sol}))REPRESENTATION_CONTEXT('',''))`);
  const pt = (x: number, y: number, z: number) => e(`CARTESIAN_POINT('',(${x.toFixed(1)},${y.toFixed(1)},${z.toFixed(1)}))`);
  const dir = (x: number, y: number, z: number) => e(`DIRECTION('',(${x.toFixed(1)},${y.toFixed(1)},${z.toFixed(1)}))`);
  const vtx = (p: number[]) => e(`VERTEX_POINT('',#${pt(p[0], p[1], p[2])})`);
  const edge = (a: number, pa: number[], b: number, pb: number[]) => {
    const d = [pb[0] - pa[0], pb[1] - pa[1], pb[2] - pa[2]];
    const n = Math.hypot(d[0], d[1], d[2]);
    const line = e(`LINE('',#${pt(pa[0], pa[1], pa[2])},#${e(`VECTOR('',#${dir(d[0] / n, d[1] / n, d[2] / n)},1.0)`)})`);
    return e(`EDGE_CURVE('',#${a},#${b},#${line},.T.)`);
  };
  const plane = (o: number[], nz: number[]) => e(`PLANE('',#${e(`AXIS2_PLACEMENT_3D('',#${pt(o[0], o[1], o[2])},#${dir(nz[0], nz[1], nz[2])},$)`)})`);
  const face = (loop: [number, boolean][], o: number[], nz: number[]) => {
    const oes = loop.map(([ec, fwd]) => `#${e(`ORIENTED_EDGE('',*,*,#${ec},${fwd ? ".T." : ".F."})`)}`);
    const el = e(`EDGE_LOOP('',(${oes.join(",")}))`);
    const bound = e(`FACE_OUTER_BOUND('',#${el},.T.)`);
    return e(`ADVANCED_FACE('',(#${bound}),#${plane(o, nz)},.T.)`);
  };
  const P = [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0], [0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]];
  const V = P.map(vtx);
  const E = (i: number, j: number) => edge(V[i], P[i], V[j], P[j]);
  const b01 = E(0, 1), b12 = E(1, 2), b23 = E(2, 3), b30 = E(3, 0);
  const t45 = E(4, 5), t56 = E(5, 6), t67 = E(6, 7), t74 = E(7, 4);
  const v04 = E(0, 4), v15 = E(1, 5), v26 = E(2, 6), v37 = E(3, 7);
  const faces = [
    face([[b30, false], [b23, false], [b12, false], [b01, false]], [0, 0, 0], [0, 0, -1]),
    face([[t45, true], [t56, true], [t67, true], [t74, true]], [0, 0, 1], [0, 0, 1]),
    face([[b01, true], [v15, true], [t45, false], [v04, false]], [0, 0, 0], [0, -1, 0]),
    face([[b12, true], [v26, true], [t56, false], [v15, false]], [1, 0, 0], [1, 0, 0]),
    face([[b23, true], [v37, true], [t67, false], [v26, false]], [1, 1, 0], [0, 1, 0]),
    face([[b30, true], [v04, true], [t74, false], [v37, false]], [0, 1, 0], [-1, 0, 0]),
  ];
  const shell = e(`CLOSED_SHELL('',(${faces.map((f) => `#${f}`).join(",")}))`);
  const msb = e(`MANIFOLD_SOLID_BREP('',#${shell})`);
  const Q = [[0, 0, 2], [1, 0, 2], [1, 1, 2], [0, 1, 2]];
  const QV = Q.map(vtx);
  const q = [0, 1, 2, 3].map((i) => edge(QV[i], Q[i], QV[(i + 1) % 4], Q[(i + 1) % 4]));
  const freeFace = face(q.map((x) => [x, true] as [number, boolean]), [0, 0, 2], [0, 0, 1]);
  const openShell = e(`OPEN_SHELL('',(#${freeFace}))`);
  const W = [[0, 0, 3], [2, 0, 3], [2, 1, 3]];
  const WV = W.map(vtx);
  const w1 = edge(WV[0], W[0], WV[1], W[1]), w2 = edge(WV[1], W[1], WV[2], W[2]);
  if (variant === "single") {
    const sbsm = e(`SHELL_BASED_SURFACE_MODEL('',(#${openShell}))`);
    const ces = e(`CONNECTED_EDGE_SET('',(#${w1},#${w2}))`);
    const ebwm = e(`EDGE_BASED_WIREFRAME_MODEL('',(#${ces}))`);
    const rep = e(`SHAPE_REPRESENTATION('',(#${msb},#${sbsm},#${ebwm}),#${ctx})`);
    e(`SHAPE_DEFINITION_REPRESENTATION(#${pds},#${rep})`);
  } else {
    const absr = e(`ADVANCED_BREP_SHAPE_REPRESENTATION('',(#${msb}),#${ctx})`);
    e(`SHAPE_DEFINITION_REPRESENTATION(#${pds},#${absr})`);
    const sbsm = e(`SHELL_BASED_SURFACE_MODEL('',(#${openShell}))`);
    const mssr = e(`MANIFOLD_SURFACE_SHAPE_REPRESENTATION('',(#${sbsm}),#${ctx})`);
    e(`SHAPE_DEFINITION_REPRESENTATION(#${pds},#${mssr})`);
    const ces = e(`CONNECTED_EDGE_SET('',(#${w1},#${w2}))`);
    const ebwm = e(`EDGE_BASED_WIREFRAME_MODEL('',(#${ces}))`);
    const ebwsr = e(`EDGE_BASED_WIREFRAME_SHAPE_REPRESENTATION('',(#${ebwm}),#${ctx})`);
    e(`SHAPE_DEFINITION_REPRESENTATION(#${pds},#${ebwsr})`);
  }
  return ["ISO-10303-21;", "HEADER;", "FILE_DESCRIPTION((''),'2;1');", "FILE_NAME('semio.step','',(''),(''),'semio','','');", "FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));", "ENDSEC;", "DATA;", ...lines, "ENDSEC;", "END-ISO-10303-21;", ""].join("\n");
}

function unwrapResult(value: any, label: string): any {
  if (value && typeof value === "object" && "ok" in value) {
    if (!value.ok) throw new Error(`${label}: ${JSON.stringify(value.error ?? value).slice(0, 300)}`);
    return value.value;
  }
  return value;
}

async function probe(text: string, label: string) {
  try {
    const imported = unwrapResult(await (B as any).importSTEP(new Blob([text])), "importSTEP");
    const shape = imported instanceof Promise ? unwrapResult(await imported, "importSTEP") : imported;
    const count = (fn: string) => { try { return (B as any)[fn](shape).length; } catch (error) { return `err ${String(error).slice(0, 80)}`; } };
    const measure = (fn: string) => { try { const r = unwrapResult((B as any)[fn](shape), fn); return typeof r === "number" ? +r.toFixed(9) : r; } catch (error) { return `err ${String(error).slice(0, 80)}`; } };
    const bounds = (() => { try { const b: any = (B as any).getBounds(shape); return [b.xMin, b.yMin, b.zMin, b.xMax, b.yMax, b.zMax].map((v: number) => +v.toFixed(9)); } catch (error) { return `err ${String(error).slice(0, 80)}`; } })();
    console.log(JSON.stringify({ label, type: (() => { try { return (B as any).shapeType(shape); } catch { return "?"; } })(), solids: count("getSolids"), shells: count("getShells"), faces: count("getFaces"), wires: count("getWires"), edges: count("getEdges"), vertices: count("getVertices"), volume: measure("measureVolume"), area: measure("measureArea"), length: measure("measureLength"), bounds }));
  } catch (error) {
    console.log(JSON.stringify({ label, refused: String(error).slice(0, 400) }));
  }
}

const arg = process.argv[2] ?? "--design";
if (arg === "--design") {
  for (const variant of ["multi", "single"]) {
    const text = design(variant);
    writeFileSync(`/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-cd1-work/s15/design-${variant}.step`, text);
    await probe(text, `design-${variant}`);
  }
} else {
  for (const file of process.argv.slice(2)) await probe(readFileSync(file, "utf8"), file.split("/").pop() ?? file);
}
