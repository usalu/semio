/**
 * @emoji 📚 Same contract as Storybook `createStorybookKitGraphqlHandle` via `@semio-tech/compose-js`.
 * Run: `bun .repo/🎫/26/05/19/GRAPH-QL-MUTATION-RESPONSE-TYPES/storybook-kit-e2e.ts`
 */
delete process.env.COMPOSE_JS_RUN_EMBEDDED_TESTS;
delete process.env.VITEST;

import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = fileURLToPath(new URL(".", import.meta.url));
const repoRoot = resolve(here, "../../../../../..");
const { openSessionInMemory } = await import(resolve(repoRoot, "compose/client/lib/js/index.ts"));

const minimalKit = {
  id: "kit-storybook-e2e",
  name: "Storybook E2E",
  types: { hash: "h", items: [{ id: "t1", name: "Kind-A" }] },
  designs: { hash: "h", items: [] },
};

const session = await openSessionInMemory({ timeoutMs: 120_000 });
try {
  const store = (await session.stores())[0];
  if (store == null) throw new Error("no store");
  const installed = await store.installProjection(JSON.stringify(minimalKit));
  if (!installed.ok) throw new Error(installed.error?.message ?? "installProjection failed");
  const kit = await store.wip().theKit().kit();
  if ((await kit.name()) !== "Storybook E2E") throw new Error("kit name mismatch");
  const types = await kit.types();
  if ((await types[0]?.name()) !== "Kind-A") throw new Error("type name mismatch");
  console.log("[DEBUG] storybook-kit-e2e: PASS");
} finally {
  await session.dispose();
}
