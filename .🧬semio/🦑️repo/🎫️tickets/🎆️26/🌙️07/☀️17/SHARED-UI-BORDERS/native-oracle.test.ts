import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv/dist/2020";
import lodash from "lodash";

const directory = `${process.cwd()}/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🧱️chrome-paint-and-navbar`;
const cases = JSON.parse(readFileSync(`${directory}/border-cases.json`, "utf8"));
const schema = JSON.parse(readFileSync(`${directory}/border-cases.schema.json`, "utf8"));
const priorities = ["normal", "focus", "active", "waiting", "loading", "introduced", "celebrated", "hover"];

test("native neutral frames match independent lodash interval composition", () => {
  expect(new Ajv().compile(schema)(cases)).toBe(true);
  for (const fixture of cases) {
    const edges = fixture.frames.flatMap((frame: any) => {
      const [x, y, w, h] = frame.rect;
      return [[false, y, x, x + w, 0, "top"], [false, y + h, x, x + w, -fixture.stroke, "bottom"], [true, x, y, y + h, 0, "left"], [true, x + w, y, y + h, -fixture.stroke, "right"]].map(([vertical, line, start, end, inset, side]) => ({ vertical, line, start, end, inset, owner: frame.owner, state: frame.state, id: `${frame.owner}:${side}` }));
    });
    const output: any[] = [];
    for (const group of Object.values(lodash.groupBy(edges, edge => `${edge.vertical}:${edge.line}`)) as any[][]) {
      const cuts = lodash.sortBy(lodash.uniq(group.flatMap(edge => [edge.start, edge.end])));
      for (let index = 1; index < cuts.length; index++) {
        const start = cuts[index - 1], end = cuts[index];
        const covering = group.filter(edge => edge.start <= start && edge.end >= end);
        if (!covering.length) continue;
        const winner = lodash.orderBy(covering, [edge => priorities.indexOf(edge.state), "owner", "id"], ["desc", "asc", "asc"])[0];
        output.push({ owner: winner.owner, state: winner.state, rect: winner.vertical ? [winner.line + winner.inset, start, fixture.stroke, end - start] : [start, winner.line + winner.inset, end - start, fixture.stroke] });
      }
    }
    expect(lodash.mapValues(lodash.groupBy(output, "owner"), values => values.length)).toEqual(fixture.expectedCounts);
    for (const stroke of fixture.expectedStrokes) expect(output).toContainEqual(stroke);
  }
});
