import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { Box3, Vector3, Vector4 } from "three";
import { NURBSCurve } from "three/addons/curves/NURBSCurve.js";
import { NURBSSurface } from "three/addons/curves/NURBSSurface.js";

const data = JSON.parse(readFileSync("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️nurbs-cuboid.json", "utf8"));
const knots = [0, 0, 1, 1];
const weighted = ([x, y, z]) => new Vector4(x, y, z, 1);
for (const row of data.cases) {
  const expected = [...row.edge.controls, ...row.patch.controls.flat(), row.corners[1]].map((xyz) => new Vector3(...xyz));
  const bounds = new Box3().setFromPoints(expected);
  const min = new Vector3(...row.corners[0]);
  const max = new Vector3(...row.corners[1]);
  assert.deepEqual(bounds.min, min);
  assert.deepEqual(bounds.max, max);
  const { x, y, z } = bounds.getSize(new Vector3());
  assert.equal(x * y * z, row.volume);
  assert.equal(2 * (x * y + y * z + z * x), row.area);
  const line = new NURBSCurve(1, knots, row.edge.controls.map(weighted));
  const edge = line.getPoint(row.edge.parameter);
  assert.ok(edge.distanceTo(new Vector3(...row.edge.point)) < data.tolerance);
  const sheet = new NURBSSurface(1, 1, knots, knots, row.patch.controls.map((edges) => edges.map(weighted)));
  const surface = new Vector3();
  sheet.getPoint(...row.patch.parameter, surface);
  assert.ok(surface.distanceTo(new Vector3(...row.patch.point)) < data.tolerance);
}

const envelope = `ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('Three.js cuboid fixture'),'2;1');
FILE_NAME('fixture.step','2024-01-01T00:00:00',(''),(''),'three','semio','');
FILE_SCHEMA(('CONFIG_CONTROL_DESIGN'));
ENDSEC;
DATA;
/* Fixture uses independent instance numbering and face order. */
#1=(LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.));
#2=(NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.));
#3=(NAMED_UNIT(*) SI_UNIT($,.STERADIAN.));
#4=UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(0.0000001),#1,'distance_accuracy_value','');
#5=(GEOMETRIC_REPRESENTATION_CONTEXT(3) GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#4)) GLOBAL_UNIT_ASSIGNED_CONTEXT((#1,#2,#3)) REPRESENTATION_CONTEXT('Context3D','3D Context'));
#6=APPLICATION_CONTEXT('configuration controlled 3D design of mechanical parts and assemblies');
#7=MECHANICAL_CONTEXT('',#6,'mechanical');
#8=APPLICATION_PROTOCOL_DEFINITION('international standard','config_control_design',1994,#6);
#9=PRODUCT('fixture','fixture','',(#7));
#10=PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE('','',#9,.NOT_KNOWN.);
#11=PRODUCT_DEFINITION_CONTEXT('part definition',#6,'design');
#12=PRODUCT_DEFINITION('design','',#10,#11);
#13=PRODUCT_DEFINITION_SHAPE('','',#12);
#14=SHAPE_DEFINITION_REPRESENTATION(#13,#15);
#15=ADVANCED_BREP_SHAPE_REPRESENTATION('',(#16,#20),#5);
#16=AXIS2_PLACEMENT_3D('',#17,#18,$);
#17=CARTESIAN_POINT('',(0.,0.,0.));
#18=DIRECTION('',(0.,0.,1.));
#20=MANIFOLD_SOLID_BREP('',#19);
`;
const pointTag = (point) => point.toArray().join(",");
const ordered = (a, b) => pointTag(a) < pointTag(b) ? [a, b] : [b, a];
const real = (x) => Number.isInteger(x) ? x + "." : String(x);

function exchangeFixture(row, spline = true) {
  const stock = new Box3(...row.corners.map((xyz) => new Vector3(...xyz)));
  const points = new Map();
  const vertices = new Map();
  const edges = new Map();
  const records = [];
  const add = (code) => {
    const id = "#" + (101 + records.length);
    records.push(id + "=" + code + ";");
    return id;
  };
  const point = (p) => {
    const key = pointTag(p);
    if (!points.has(key)) points.set(key, add("CARTESIAN_POINT('',(" + p.toArray().map(real).join(",") + "))"));
    return points.get(key);
  };
  const direction = (p) => add(`DIRECTION('',(${p.toArray().map(real).join(",")}))`);
  const vertex = (p) => {
    const key = pointTag(p);
    if (!vertices.has(key)) vertices.set(key, add(`VERTEX_POINT('',${point(p)})`));
    return vertices.get(key);
  };
  const edgeUse = (a, b) => {
    const key = ordered(a, b).map(pointTag).join("|");
    if (!edges.has(key)) {
      const axis = a.toArray().findIndex((value, i) => value !== b.getComponent(i));
      const ascends = axis === 2 || (axis === 0 ? a.y === stock.min.y : a.x === stock.max.x);
      const forward = (a.getComponent(axis) < b.getComponent(axis)) === ascends;
      const [start, end] = forward ? [a, b] : [b, a];
      let curve;
      if (spline) {
        curve = add(`B_SPLINE_CURVE_WITH_KNOTS('',1,(${point(start)},${point(end)}),.UNSPECIFIED.,.F.,.F.,(2,2),(0.,1.),.UNSPECIFIED.)`);
      } else {
        const delta = end.clone().sub(start);
        const vector = add(`VECTOR('',${direction(delta.clone().normalize())},${real(delta.length())})`);
        curve = add(`LINE('',${point(start)},${vector})`);
      }
      edges.set(key, [add(`EDGE_CURVE('',${vertex(start)},${vertex(end)},${curve},.T.)`), pointTag(start)]);
    }
    const [edge, start] = edges.get(key);
    return add(`ORIENTED_EDGE('',*,*,${edge},${start === pointTag(a) ? ".T." : ".F."})`);
  };
  const axes = [0, 1, 2].map((axis) => new Vector3().setComponent(axis, 1));
  const faceIds = [];
  for (const axis of [0, 2, 1]) {
    for (const isUpper of [true, false]) {
      const normal = axes[axis].clone().multiplyScalar(isUpper ? 1 : -1);
      const x = normal.clone().cross(axes[axis === 0 ? 1 : 0]);
      const y = normal.clone().cross(x);
      const origin = stock.min.clone();
      if (isUpper) origin.setComponent(axis, stock.max.getComponent(axis));
      const [uAxis, vAxis] = [x, y].map((v) => v.toArray().findIndex((value) => value !== 0));
      const values = (direction, i) => {
        const component = (p) => (p.getComponent(i) - origin.getComponent(i)) * direction.getComponent(i);
        return [component(stock.min), component(stock.max)].sort((a, b) => a - b);
      };
      const u = values(x, uAxis), v = values(y, vAxis);
      const [[a, b], [c, d]] = u.map((u) => v.map((v) => origin.clone().addScaledVector(x, u).addScaledVector(y, v)));
      const curveRefs = [edgeUse(a, c), edgeUse(c, d), edgeUse(d, b), edgeUse(b, a)];
      const loop = add(`EDGE_LOOP('',(${curveRefs.join(",")}))`);
      const bound = add(`FACE_OUTER_BOUND('',${loop},.T.)`);
      let surface;
      if (spline) {
        surface = add(`B_SPLINE_SURFACE_WITH_KNOTS('',1,1,((${point(a)},${point(b)}),(${point(c)},${point(d)})),.UNSPECIFIED.,.F.,.F.,.F.,(2,2),(2,2),(${u.map(real)}),(${v.map(real)}),.UNSPECIFIED.)`);
      } else {
        const placement = add(`AXIS2_PLACEMENT_3D('',${point(origin)},${direction(normal)},${direction(x)})`);
        surface = add(`PLANE('',${placement})`);
      }
      faceIds.push(add(`ADVANCED_FACE('',(${bound}),${surface},.T.)`));
    }
  }
  records.push(`#19=CLOSED_SHELL('',(${faceIds.join(",")}));`);
  return envelope + records.reverse().join("\n") + "\nENDSEC;\nEND-ISO-10303-21;\n";
}

if (process.argv[2]) {
  for (const [index, row] of data.cases.entries()) {
    writeFileSync(join(process.argv[2], `nurbs-external-${index}.step`), exchangeFixture(row));
    writeFileSync(join(process.argv[2], `analytic-external-${index}.step`), exchangeFixture(row, false));
  }
}
