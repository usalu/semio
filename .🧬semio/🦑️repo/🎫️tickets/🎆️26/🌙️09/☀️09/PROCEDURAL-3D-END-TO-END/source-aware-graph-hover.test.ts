import { expect, test } from "bun:test";
import { from, scan } from "rxjs";
import { isDeepStrictEqual } from "node:util";
import { Database } from "bun:sqlite";

const cases = await Bun.file("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🧫️fixtures/🔣️hover.json").json();
for (const scenario of cases.filter(row => row.window)) {
  test(scenario.id, () => {
    type Mark = { hoveredId?: string; portId?: string; portIds?: string[]; portDirection?: "in" | "out"; edgeId?: string };
    type State = { authored: boolean; painted?: Mark; write?: Mark };
    const writes: (Mark | undefined)[] = [];
    from(scenario.steps).pipe(scan((state: State, step: any): State => {
      const own = step.scene?.hoverSourceWindowId === scenario.window;
      let candidate: Mark | undefined;
      if (step.scene) {
        const { nodeId, edgeId, portId, portIds, portDirection } = step.scene;
        candidate = edgeId ? { edgeId } : { hoveredId: nodeId, ...(portId && { portId }),
          ...(portDirection && { portDirection }), ...(portIds?.length && { portIds }) };
      }
      if ("scene" in step && !own && (candidate || state.authored)) {
        const write = candidate ?? {};
        if (state.authored && candidate && isDeepStrictEqual(write, state.painted)) return { ...state, write: undefined };
        return { authored: Boolean(candidate), painted: write, write };
      }
      return { authored: step.local ? false : state.authored, painted: step.publish ? step.read : state.painted };
    }, { authored: false })).subscribe(state => writes.push(state.write));
    expect(writes).toEqual(scenario.steps.map(step => step.write ?? undefined));
  });
}

test("inspector rows qualify each channel side without losing its authored name", async () => {
  const fixture = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🖱️channel-hover.json").text();
  const db = new Database(":memory:");
  try {
    const rows = db.query(`
      WITH rows AS (
        SELECT value r FROM json_each(?1, '$.cases')
      ), channels AS (
        SELECT r ->> '$.widget' AS widget, r ->> '$.port' AS port, r ->> '$.target.id' AS expected,
          (r ->> '$.kind') IN ('connected', 'literal', 'numeric', 'boolean') OR (r ->> '$.incoming') IS 1 AS incoming
        FROM rows
      )
      SELECT expected, 'h:' || widget || '@' ||
        CASE WHEN incoming THEN '!i' WHEN port = '' OR substr(port,1,1) = '!' THEN '!o' ELSE '' END ||
        replace(replace(replace(replace(port,'%','%25'),'@','%40'),'#','%23'),'~','%7E') AS actual
      FROM channels
    `).all(fixture);
    expect(rows).toHaveLength(14);
    for (const row of rows) expect(row.actual).toBe(row.expected);
  } finally {
    db.close();
  }
});
