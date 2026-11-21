import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";

const cases = (await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🧫️fixtures/preview-evaluation.json").json()).cases;
const db = new Database(":memory:");
const shape = db.query(`
  with root(input) as (values (?)), object(input) as (select input from root where json_valid(input) and json_type(input)='object'),
  duplicates as (select parent, key from object, json_tree(input) where typeof(key)='text' group by parent,key having count(*)>1)
  select case when exists(select 1 from duplicates) then null else input end input from object
`);
const ports = db.query(`
  select nodes.key||'@'||case when sides.key='in' then '!i' when channels.key='' or substr(channels.key,1,1)='!' then '!o' else '' end||channels.key id
  from json_each(?) nodes, json_each(nodes.value) sides, json_each(sides.value) channels
  where nodes.type='object' and sides.key in ('in','out') and sides.type='object' order by nodes.key,channels.key,sides.key
`);
for (const row of cases) {
  test(row.id, () => {
    const input = (shape.get(row.text) as {input: string | null} | null)?.input ?? null;
    const object = input === null ? null : JSON.parse(input);
    const global = typeof object?.error === "string" ? object.error : null;
    const keys = input === null ? [] : (ports.all(input) as {id:string}[]).map(port=>port.id);
    expect({ valid:object!==null && global===null, global, keys }).toEqual({ valid:row.valid, global:row.global, keys:row.keys });
  });
}
