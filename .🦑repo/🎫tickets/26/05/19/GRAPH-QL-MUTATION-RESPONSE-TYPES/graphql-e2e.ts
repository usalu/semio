/**
 * @emoji 🧪 Node E2E: openSessionInMemory → installProjection → mutations (no Vitest).
 * Run: `bun .repo/🎫/26/05/19/GRAPH-QL-MUTATION-RESPONSE-TYPES/graphql-e2e.ts`
 */
delete process.env.COMPOSE_JS_RUN_EMBEDDED_TESTS;
delete process.env.COMPOSE_REACT_RUN_EMBEDDED_TESTS;
delete process.env.VITEST;

import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = fileURLToPath(new URL(".", import.meta.url));
const repoRoot = resolve(here, "../../../../../..");
const { openSessionInMemory } = await import(resolve(repoRoot, "compose/client/lib/js/index.ts"));

const minimalKit = {
  id: "kit-e2e-node",
  name: "E2E Node Kit",
  types: { hash: "h", items: [{ id: "type-e2e", name: "Capsule" }] },
  designs: {
    hash: "h",
    items: [
      {
        id: "design-e2e",
        name: "Flat",
        pieces: {
          hash: "h",
          items: [
            {
              id: "piece-e2e",
              name: "P1",
              type: { id: "type-e2e" },
              center: { u: 0, v: 0 },
              plane: {
                origin: { x: 0, y: 0, z: 0 },
                xAxis: { x: 1, y: 0, z: 0 },
                yAxis: { x: 0, y: 1, z: 0 },
              },
            },
          ],
        },
      },
    ],
  },
};

console.log("[DEBUG] graphql-e2e: installProjection + mutations");
const session = await openSessionInMemory({ timeoutMs: 120_000 });
try {
  const store = (await session.stores())[0];
  if (store == null) throw new Error("no store after openInMemory");
  const installed = await store.installProjection(JSON.stringify(minimalKit));
  if (!installed.ok) throw new Error(`installProjection: ${installed.error?.message ?? "failed"}`);

  const kit = await store.wip().theKit().kit();
  const tag = await kit.createTag("e2e-node-tag");
  if (!tag.ok) throw new Error(`createTag: ${tag.error?.message ?? "failed"}`);
  if ((await kit.tags()).length < 1) throw new Error("createTag did not materialize");

  const renamed = await kit.rename("e2e-renamed-kit");
  if (!renamed.ok) throw new Error(`rename: ${renamed.error?.message ?? "failed"}`);
  if ((await kit.name()) !== "e2e-renamed-kit") throw new Error("rename did not persist");

  const started = await store.startAlternative("e2e-alt");
  if (!started.ok) throw new Error(`startAlternative: ${started.error?.message ?? "failed"}`);
  const alts = await store.wip().alternatives();
  if (alts.length < 1) throw new Error("startAlternative did not expose graph branch");

  const pieces = await store.design("design-e2e").pieces();
  if (pieces.length < 1) throw new Error("design pieces missing after installProjection");

  console.log("[DEBUG] graphql-e2e: kit", await kit.name(), "alts", alts.length, "pieces", pieces.length);
} finally {
  await session.dispose();
}

console.log("[DEBUG] graphql-e2e: PASS");
