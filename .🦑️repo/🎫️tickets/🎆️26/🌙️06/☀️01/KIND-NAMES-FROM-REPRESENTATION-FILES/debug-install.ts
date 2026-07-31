#!/usr/bin/env bun
import { readFile } from "node:fs/promises";
import { join } from "node:path";

const { Session } = await import("/Users/ueli/Documents/compose/compose/client/lib/js/index.ts");
const path = process.argv[2] ?? join(import.meta.dir, "../../../../../compose/fixtures/architect.harness.kit.compose.json");
const session = await Session.openInMemory({ timeoutMs: 120_000 });
const store = (await session.stores())[0]!;
const json = await readFile(path, "utf8");
const r = await store.installProjection(json);
console.log(path, r);
await session.dispose();
