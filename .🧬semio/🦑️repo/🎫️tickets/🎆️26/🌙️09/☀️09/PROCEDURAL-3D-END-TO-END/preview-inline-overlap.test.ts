import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";

const fixture = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🧫️fixtures/preview-inline-overlap.json").json();

test("exact inline values carry all applicable directional ports", () => {
  const database = new Database(":memory:", { strict: true });
  const query = database.query(`
    with ports as (
      select case json_extract(case when leaf.type = 'object' then leaf.value end, '$."$schema"')
               when 'plane' then json_array('plane',
                 json_extract(leaf.value, '$.origin.x'), json_extract(leaf.value, '$.origin.y'), json_extract(leaf.value, '$.origin.z'),
                 json_extract(leaf.value, '$.normal.x'), json_extract(leaf.value, '$.normal.y'), json_extract(leaf.value, '$.normal.z'))
               else json(leaf.value)
             end as geometry,
             'h:' || ?1 || '@' || case side.key when 'in' then '!i' else '' end || channel.key as port
      from json_each(?2) as side, json_each(side.value) as channel, json_tree(channel.value) as leaf
      where leaf.type = 'object' and json_extract(case when leaf.type = 'object' then leaf.value end, '$."$schema"') in ('point', 'vector', 'plane', 'mesh')
        and not exists (
          select 1 from json_tree(channel.value) as container
          where container.type = 'object' and json_extract(case when container.type = 'object' then container.value end, '$."$schema"') in ('point', 'vector', 'plane', 'mesh')
            and (instr(leaf.fullkey, container.fullkey || '.') = 1 or instr(leaf.fullkey, container.fullkey || '[') = 1)
        )
      order by port
    )
    select json_group_array(port) as members from ports group by geometry
  `);
  const groups = query.all(fixture.widget, JSON.stringify(fixture.evaluation[fixture.widget])).map(({ members }) => JSON.parse(members));
  expect(groups.flat().length).toBe(fixture.instances);
  expect(groups.length).toBe(fixture.distinctMeshes);
  expect(groups.filter(group => group.length > 1).sort()).toEqual(fixture.groups);
  database.close();
});
