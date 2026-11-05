/** 🗑️ Replays the shared deletion law through Bun’s independent assertions. */
import { expect, test } from "bun:test";
import { deleteEntitiesFailuresV1, readDeleteEntitiesFixtureV1 } from "../🟦️.ts";
import { executeDeleteEntitiesV1, executeDeleteSelectionV1 } from "../../../📇️registry/🧬️contract/🖥️app-surface/🧰️entity-editing/🗑️delete/🟦️.ts";

test("typed deletion and raw argument admission satisfy the shared fixture", () => {
  expect(deleteEntitiesFailuresV1()).toEqual([]);
});

test("committed rows equal an independent filter by selected identity", () => {
  const fixture = readDeleteEntitiesFixtureV1();
  for (const row of fixture.cases.filter(row => row.outcome === "emitted")) {
    const before = structuredClone(fixture.document);
    const verdict = executeDeleteSelectionV1(fixture.document, row.collection, row.ids, (_tree, edit) => ({ ok: true, mutations: [edit] }));
    expect(verdict.outcome).toBe("emitted");
    if (verdict.outcome !== "emitted") throw Error(row.id);
    const rows = fixture.document[row.collection] as readonly { id: string }[];
    expect(verdict.tree).toEqual({ ...fixture.document, [row.collection]: rows.filter(item => !row.ids.includes(item.id)) });
    expect(fixture.document).toEqual(before);
    expect(verdict.undoSteps).toBe(1);
  }
});

test("raw empty arguments are invalid while typed empty selection has its domain refusal", () => {
  const { document } = readDeleteEntitiesFixtureV1();
  const resolve = () => { throw Error("A refusal must never resolve a mutation"); };
  expect(executeDeleteEntitiesV1(document, { collection: "beams", ids: [] }, resolve)).toEqual({ outcome: "invalid", error: "an entity command needs at least one id" });
  expect(executeDeleteSelectionV1(document, "beams", [], resolve)).toMatchObject({ outcome: "refused", code: "norm.entity-delete-empty" });
});
