/** 🗑️ Replay of the `deleteEntities` fixture against the TypeScript executor: the same rows the Rust reducer replays (`entity-editing/🧪️tests`), so the route of the retained-command-dispositions fixture is executed in both languages and not only counted. Returns the failed expectations (empty when the executor agrees with every row).
 *
 * 🧫️ `📇️registry/🧬️contract/🧪️tests/🧫️fixtures/🗑️delete-entities/🔣️.json`; 🔗️ executor `📇️registry/🧬️contract/🖥️app-surface/🧰️entity-editing/🗑️delete/🟦️.ts`. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { entitiesArgsV1, executeDeleteEntitiesV1, NORM_DELETE_NOTICES_V1, type BilingualTextV1, type DocumentTreeV1 } from "../../📇️registry/🧬️contract/🖥️app-surface/🧰️entity-editing/🗑️delete/🟦️.ts";

type CaseV1 = Readonly<{ id: string; collection: string; ids: readonly string[]; outcome: "emitted" | "refused"; edits?: readonly unknown[]; remaining: readonly string[]; mutations?: number; undoSteps?: number; code?: string; message?: BilingualTextV1; refuseRemovalAt?: number; resolverError?: string }>;

/** 🧫️ The fixture document of the multi-delete law. */
export type DeleteEntitiesFixtureV1 = Readonly<{ document: DocumentTreeV1; notices: Readonly<Record<string, BilingualTextV1>>; arguments: readonly Readonly<{ id: string; args: unknown; parsed?: unknown; error?: string }>[]; cases: readonly CaseV1[] }>;

/** 📖️ Reads the committed fixture. */
export function readDeleteEntitiesFixtureV1(): DeleteEntitiesFixtureV1 {
  return JSON.parse(readFileSync(join(import.meta.dir, "../../📇️registry/🧬️contract/🧪️tests/🧫️fixtures/🗑️delete-entities/🔣️.json"), "utf8")) as DeleteEntitiesFixtureV1;
}

const idsOf = (tree: DocumentTreeV1, collection: string): string[] => ((tree[collection] as readonly { id: string }[] | undefined) ?? []).map((row) => row.id);

/** 🔁️ Executes every row of `fixture` and lists the expectations that do not hold. */
export function deleteEntitiesFailuresV1(fixture: DeleteEntitiesFixtureV1 = readDeleteEntitiesFixtureV1()): string[] {
  const failures: string[] = [];
  const expect = (ok: boolean, message: string) => {
    if (!ok) failures.push(message);
  };
  expect(JSON.stringify(fixture.notices) === JSON.stringify(Object.fromEntries(Object.keys(fixture.notices).map((code) => [code, NORM_DELETE_NOTICES_V1[code]]))), "the notices of the executor equal the notices of the fixture");
  for (const row of fixture.arguments) {
    const parsed = entitiesArgsV1(row.args);
    if (row.error === undefined) expect(parsed.ok && JSON.stringify({ collection: parsed.collection, ids: parsed.ids }) === JSON.stringify(row.parsed), `arguments '${row.id}' parse`);
    else expect(!parsed.ok && parsed.error === row.error, `arguments '${row.id}' fail with '${row.error}'`);
  }
  const original = JSON.stringify(fixture.document);
  for (const row of fixture.cases) {
    const verdict = executeDeleteEntitiesV1<string>(fixture.document, { collection: row.collection, ids: row.ids }, (_tree, edit) => (edit.index === row.refuseRemovalAt ? { ok: false, error: row.resolverError ?? "invariant" } : { ok: true, mutations: [`remove ${edit.path}[${edit.index}]`] }));
    expect(JSON.stringify(fixture.document) === original, `'${row.id}' leaves the input document untouched`);
    if (row.outcome === "emitted") {
      expect(verdict.outcome === "emitted", `'${row.id}' is emitted`);
      if (verdict.outcome !== "emitted") continue;
      expect(JSON.stringify(verdict.edits) === JSON.stringify(row.edits), `'${row.id}' edits`);
      expect(JSON.stringify(idsOf(verdict.tree, row.collection)) === JSON.stringify(row.remaining), `'${row.id}' remaining rows`);
      expect(verdict.mutations.length === row.mutations, `'${row.id}' mutations`);
      expect(verdict.undoSteps === row.undoSteps, `'${row.id}' is one undo step`);
    } else {
      expect(verdict.outcome === "refused", `'${row.id}' is refused`);
      if (verdict.outcome !== "refused") continue;
      expect(verdict.code === row.code, `'${row.id}' code ${verdict.code} is ${row.code}`);
      expect(JSON.stringify(verdict.notice) === JSON.stringify(fixture.notices[row.code ?? ""]), `'${row.id}' notice`);
      if (row.message !== undefined) expect(JSON.stringify(verdict.message) === JSON.stringify(row.message), `'${row.id}' message`);
      expect(JSON.stringify(idsOf(fixture.document, row.collection)) === JSON.stringify(row.remaining), `'${row.id}' deletes nothing`);
    }
  }
  return failures;
}
