#!/usr/bin/env bun
/** 🔧 Moves playground re-exports to end of index.ts and drops SExtension playground imports. */

import { readFileSync, writeFileSync } from "node:fs";
import { globSync } from "node:glob";

const INDEX_PATHS = [
  ...globSync("{**/core/index.ts,**/host-core/index.ts,cad/js/renderer/core/index.ts}", {
    cwd: "/Users/ueli/Documents/semio",
    absolute: true,
    ignore: ["**/node_modules/**"],
  }),
];

const PLAYGROUND_EXPORT_RE = /^export \{[^}]+\} from "\.\/playground\.ts";?\s*$/m;
const PLAYGROUND_IMPORT_RE = /^import \{ (\w+PlayAppDefinition) \} from "\.\/playground\.ts";?\s*\n\n\/\*\* @emoji 🧩/m;

for (const path of INDEX_PATHS) {
  let src = readFileSync(path, "utf8");
  if (!src.includes('from "./playground.ts"')) continue;

  const exportMatch = src.match(PLAYGROUND_EXPORT_RE);
  if (!exportMatch) continue;

  const exportLine = exportMatch[0].trim();
  const exportIndex = src.indexOf(exportLine);
  const testsIndex = src.search(/\n\/\/ #region 🧪Tests|\n\/\/#region 🧪Tests|\nif \(import\.meta\.vitest\)/);
  const insertAt = testsIndex >= 0 ? testsIndex : src.length;

  src = src.replace(`${exportLine}\n`, "");
  src = src.replace(`${exportLine}`, "");

  if (!src.includes(exportLine)) {
    src = `${src.slice(0, insertAt)}\n${exportLine}\n${src.slice(insertAt)}`;
  }

  src = src.replace(PLAYGROUND_IMPORT_RE, (_match, _name) => `/** @emoji 🧩`);

  src = src.replace(
    /export function (build\w+ProgramDefinition)\(\): PlatformDefinition \{\n\tconst app = \w+PlayAppDefinition;\n\treturn \{\n\t\tid: "([^"]+)",\n\t\tname: "([^"]+)",\n\t\tapiVersion: "1",\n\t\tapps: \[\{ id: "[^"]+", label: app\.label, controllerId: app\.controllerId, modes: app\.modes, defaultModeId: app\.defaultModeId \}\],\n\t\tcreatePlatformApi: \(\) => \(\{\}\),\n\t\};\n\}/g,
    (_m, fn, id, name) => {
      const controllerConst = guessControllerConst(src, id);
      const appId = id.includes(".") ? id.split(".").pop()! : id;
      return `export function ${fn}(): PlatformDefinition {
	return {
		id: "${id}",
		name: "${name}",
		apiVersion: "1",
		apps: [{ id: "${appId}", label: "${name}", controllerId: ${controllerConst}, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}`;
    },
  );

  writeFileSync(path, src);
  console.log("patched", path);
}

function guessControllerConst(src: string, programId: string): string {
  const candidates = [...src.matchAll(/export const (\w+_PLAY_CONTROLLER_ID) = /g)].map((m) => m[1]);
  if (candidates.length === 1) return candidates[0]!;
  const slug = programId.replace(/\./g, "_").toUpperCase();
  for (const c of candidates) {
    if (c.includes(slug) || c.includes(programId.toUpperCase().replace(/\./g, "_"))) return c;
  }
  return candidates[0] ?? '"unknown-controller"';
}
