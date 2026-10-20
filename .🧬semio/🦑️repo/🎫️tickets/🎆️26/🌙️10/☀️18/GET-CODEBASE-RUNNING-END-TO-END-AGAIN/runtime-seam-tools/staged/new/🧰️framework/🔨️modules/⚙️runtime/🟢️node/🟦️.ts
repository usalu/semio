import type { RuntimeAdapterV1 } from "../📋️contract/🟦️.ts";
import { openFileV1 } from "../🪶️portable/📂️file/🟦️.ts";
import { compileGlobV1 } from "../🪶️portable/🌐️glob/🟦️.ts";
import { parseJsoncV1 } from "../🪶️portable/📝️jsonc/🟦️.ts";
import { isMainModuleV1 } from "../🪶️portable/🏁️main/🟦️.ts";
import { resolveModuleV1 } from "../🪶️portable/🧭️resolve/🟦️.ts";
import { semverOrderV1, semverSatisfiesV1 } from "../🪶️portable/🔢️semver/🟦️.ts";
import { serveHttpV1 } from "../🪶️portable/🌍️serve/🟦️.ts";
import { nodeSqliteConstructorV1 } from "../🪶️portable/🗃️sqlite/🟦️.ts";
import { spawnSyncV1, spawnV1 } from "../🪶️portable/🚀️spawn/🟦️.ts";
import { stdinStreamV1, stdinTextV1 } from "../🪶️portable/📥️stdin/🟦️.ts";
import { parseTomlV1 } from "../🪶️portable/📄️toml/🟦️.ts";
import { whichV1 } from "../🪶️portable/🔎️which/🟦️.ts";

/** 🟢️ The Node adapter: standard APIs and our portable code only; families without a standard equivalent stay absent and are reported as missing. */
export const nodeRuntimeAdapterV1: RuntimeAdapterV1 = {
  id: "node",
  identity: () => ({ id: "node", version: process.versions.node, revision: "", key: `node:${process.versions.node}`, executable: process.execPath }),
  glob: compileGlobV1,
  toml: parseTomlV1,
  jsonc: parseJsoncV1,
  which: whichV1,
  resolve: resolveModuleV1,
  sleep: milliseconds => new Promise(accept => setTimeout(accept, milliseconds)),
  spawn: spawnV1,
  spawnSync: spawnSyncV1,
  stdinText: stdinTextV1,
  stdinStream: stdinStreamV1,
  file: openFileV1,
  serve: serveHttpV1,
  sqlite: nodeSqliteConstructorV1,
  semverSatisfies: semverSatisfiesV1,
  semverOrder: semverOrderV1,
  main: isMainModuleV1,
  gc: synchronous => { (globalThis as { gc?: () => void }).gc?.(); void synchronous; },
};
