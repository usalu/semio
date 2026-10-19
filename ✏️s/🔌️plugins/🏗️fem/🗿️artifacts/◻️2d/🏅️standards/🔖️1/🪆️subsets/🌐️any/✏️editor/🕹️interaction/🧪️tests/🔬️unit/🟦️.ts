import assert from "node:assert/strict";
import { testResultSourceIdentity } from "../../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🏷️result-source/🧪️tests/🟦️.ts";
import { Matrix3, Vector2 } from "three";
import { applyPatch } from "fast-json-patch";
import reconnect from "../../../🧪️tests/🔬️unit/🔣️.json" with { type: "json" };
import transform from "../../../../../🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/🔣️.json" with { type:"json" };
import selection from "../../../🎮️commands/🗂️remove-selection/🧪️tests/🔬️unit/🔣️.json" with { type:"json" };
import { encodeScopedIdentity, decodeScopedIdentity } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🧾️wire/🪪️scoped-identity/🟦️.ts";

/** 🔮️ Independent Three matrix and typed scope oracles adjudicate the shared native mutation scenarios. */
export function testFem2dEventMechanisms(): void {
  testResultSourceIdentity();
  for (const order of [[reconnect.a, reconnect.b], [reconnect.b, reconnect.a]]) {
    const rows: number[][] = [];
    applyPatch(rows, order.map(({ x, y }) => ({ op: "add" as const, path: "/-", value: [x, y] })), true, true);
    assert.equal(rows.length, reconnect.expectedAddedNodes);
    assert.deepEqual(rows.sort((a, b) => a[0]! - b[0]!), reconnect.expectedPoints);
  }
  const m = transform.mine, cosine = Math.cos(m.angle), sine = Math.sin(m.angle);
  const matrix = new Matrix3().set(cosine*m.sx, -sine*m.sy, m.pivotX+m.dx-cosine*m.sx*m.pivotX+sine*m.sy*m.pivotY, sine*m.sx, cosine*m.sy, m.pivotY+m.dy-sine*m.sx*m.pivotX-cosine*m.sy*m.pivotY, 0,0,1);
  const point = ([x,y]: readonly number[]) => new Vector2(x,y).applyMatrix3(matrix).toArray();
  const moved: Omit<typeof transform.before, "regions"> & { regions: Array<Omit<typeof transform.before.regions[number], "holes"> & { holes: number[][][] }> } = structuredClone(transform.before);
  for (const node of moved.nodes) if (m.nodeIds.includes(node.id)) [node.x,node.y] = point([node.x,node.y]);
  for (const region of moved.regions) if (m.regionIds.includes(region.id)) { region.outline = region.outline.map(point); region.holes = region.holes.map(hole => hole.map(point)); }
  assert.deepEqual(moved.nodes, [{id:"n1",x:2,y:0}]);
  const inverse = [
    ...transform.before.regions.filter((region,index) => JSON.stringify(region) !== JSON.stringify(moved.regions[index])).map((region,index) => ({mutation:"changeRegion",id:region.id,patch:Object.fromEntries((["outline","holes"] as const).filter(field => JSON.stringify(region[field]) !== JSON.stringify(moved.regions[index]![field])).map(field => [field,region[field]]))})),
    ...transform.before.nodes.filter((node,index) => JSON.stringify(node) !== JSON.stringify(moved.nodes[index])).map((node,index) => ({mutation:"changeNode",id:node.id,patch:Object.fromEntries((["x","y"] as const).filter(field => node[field] !== moved.nodes[index]![field]).map(field => [field,node[field]]))})),
  ];
  assert.deepEqual(inverse, transform.inverse);
  for (const operation of [...transform.collaborator,...inverse]) {
    const rows = operation.mutation === "changeNode" ? moved.nodes : moved.regions;
    Object.assign(rows.find(row => row.id === operation.id)!, operation.patch);
  }
  assert.deepEqual(moved, transform.expected);
  for (const target of [...selection.completeSelection,...selection.loadSelection,...selection.protectedSelection]) {
    const segments = decodeScopedIdentity(target);
    assert.equal(encodeScopedIdentity(segments), target);
  }
  console.log("[DEBUG] FEM2d sparse transform inverse and independent peer fields agree with Three matrix geometry; typed deletion identities round-trip through shared scope codec");
}
