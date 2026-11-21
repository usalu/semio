import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
const fixture = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔌️graph-ports/🔣️.json").json();
const source = await Bun.file("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧰️graph-core/🟦️.ts").text();
function section(first, last) {
  const start = source.indexOf(first);
  const end = source.indexOf(last, start + first.length);
  if (start < 0 || end < 0) throw Error(first);
  return source.slice(start, end).replace("export function", "function");
}
const bridgeFn = new Bun.Transpiler({loader:"ts"}).transformSync([
  section("export function nodeGraphPickChannel", "\n}\n"),
  section("export function nodeGraphHoverActionArgs", "\n}\n"),
  section("export function nodeGraphSelectionActionArgs", "\n}\n"),
  section("export function portHandleId", "\n}\n"),
  section("export function projectedFlowPortCode", "\n}\n"),
  section("export function authoredFlowChannel", "\n}\n"),
].map(code => code + "\n}").join("\n"));
const host = new Function(bridgeFn + "\nreturn { portHandleId, nodeGraphPickChannel, nodeGraphHoverActionArgs, nodeGraphSelectionActionArgs, projectedFlowPortCode, authoredFlowChannel };")();
const domain = { id: "graph", handleTargetPrefix: "", edgeTargetPrefix: "", nodeTargetPrefix: "" };
const sqlite = new Database(":memory:");
const independent = sqlite.query(`
  with channel(code) as (select replace(replace(replace(replace(?3, '%', '%25'), '@', '%40'), '#', '%23'), '~', '%7E'))
  select ?1 || '@' || case when ?2 then '!i' || code when ?3 = '' or substr(?3, 1, 1) = '!' then '!o' || code else code end as target
  from channel
`);
for (const row of fixture.cases) {
  test(row.target + " round-trips through the browser port and hover adapters", () => {
    const result = independent.get(row.widget, row.input, row.channel);
    expect(result.target).toBe(row.target);
    const code = host.portHandleId({ id: row.target });
    expect(code).toBe(row.code);
    const intent = host.nodeGraphHoverActionArgs(domain, row.widget, code);
    expect(JSON.parse(intent.targets)).toEqual([{ granularity: "handle", id: row.target }]);
    expect(host.nodeGraphPickChannel({ domain: "handle", id: row.target })).toEqual({ nodeId: row.widget, portId: row.code });
  });
}

test("typed native picks and restored semantic scene marks preserve both sides", () => {
  const projection = new Map();
  for (const row of fixture.cases) {
    const node = projection.get(row.widget) ?? { id: row.widget, inputs: [], outputs: [] };
    (row.input ? node.inputs : node.outputs).push({ id: row.target, code: row.channel });
    projection.set(row.widget, node);
  }
  const nodes = Array.from(projection.values());
  for (const row of fixture.cases) {
    const raw = { widgetId: row.widget, port: row.channel, direction: row.input ? "in" : "out" };
    expect(host.projectedFlowPortCode(nodes, raw)).toBe(row.code);
    expect(host.authoredFlowChannel(nodes, row.widget, row.code)).toEqual(raw);
  }
});

test("renderer publications keep a node, wire and handle with the same authored ID distinct", () => {
  const row = fixture.interactionCollision;
  const namespace = { id: "graph", nodeTargetPrefix: "n:", edgeTargetPrefix: "e:", handleTargetPrefix: "h:" };
  const expected = row.targets.map(({ granularity, target }) => ({ granularity, id: target }));
  const actual = host.nodeGraphSelectionActionArgs(namespace, { nodeIds: [row.raw], edgeIds: [row.raw], handleIds: [row.raw] });
  const byKind = (rows: { granularity: string; id: string }[]) => rows.sort((left, right) => left.granularity.localeCompare(right.granularity));
  expect(byKind(JSON.parse(actual.targets))).toEqual(byKind(expected.slice()));
  for (const args of [
    host.nodeGraphHoverActionArgs(namespace, row.raw),
    host.nodeGraphHoverActionArgs(namespace, row.widget, row.channel),
    host.nodeGraphHoverActionArgs(namespace, undefined, undefined, row.raw),
  ]) expect(expected).toContainEqual(JSON.parse(args.targets)[0]);
  const unique = sqlite.query("select count(distinct value ->> '$.id') as count from json_each(?)").get(actual.targets);
  expect(unique.count).toBe(3);
});
