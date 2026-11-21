import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";

const fixture = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🧪️tests/🧫️fixtures/🔬️unit/🔣️.json").json();
const db = new Database(":memory:", { strict: true });
const endpoints = db.query(`
  with wires as (
    select json_extract(value, '$.id') as id,
      json_extract(value, '$.from') as source, json_extract(value, '$.fromPort') as output,
      json_extract(value, '$.to') as destination, json_extract(value, '$.toPort') as input
    from json_each(?)
  )
  select source as widget, output as port, 0 as input from wires
  where 'e:' || id = ? or 'h:' || destination || '@!i' || input = ?
  union
  select destination, input, 1 from wires where 'e:' || id = ?
`);
for (const row of fixture.expected.hoverSources) {
  test(row.target, () => {
    const ports = new Set<string>();
    const inputWidgets = new Set<string>();
    const outputWidgets = new Set<string>();
    for (const item of endpoints.all(JSON.stringify(fixture.hostSnapshot.synapses), row.target, row.target, row.target) as {widget:string;port:string;input:number}[]) {
      if (!item.port) (item.input ? inputWidgets : outputWidgets).add(item.widget);
      else ports.add(item.widget + "@" + (item.input ? "!i" : "") + item.port);
    }
    expect([...ports].sort()).toEqual(row.ports);
    expect([...inputWidgets].sort()).toEqual(row.inputWidgets);
    expect([...outputWidgets].sort()).toEqual(row.outputWidgets);
  });
}
