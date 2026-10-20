// usage: bun ts-closure-proof.ts <path to a TypeScript units engine directory (…/🟦️typescript/🧩️units)>
// Three units top -> mid -> low with a fake compiler (declaration = the export lines of the source). Only `low` changes its exported type; `mid` re-exports it by reference, so mid's declaration text is unchanged.
// A dependent that keys direct declaration digests leaves `top` cached although `top` is checked against `low`'s declarations through mid's view; a closure key re-checks it.
import {mkdirSync, mkdtempSync, readFileSync, realpathSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, join, relative} from "node:path";
import {createHash} from "node:crypto";

const engineDirectory = process.argv[2]!;
const engine = await import(join(engineDirectory, "🟦️.ts"));
const rules = JSON.parse(readFileSync(join(engineDirectory, "🔣️.json"), "utf8"));
const document = engine.readTypeScriptUnitsDocumentV1(rules);
const workspace = realpathSync(mkdtempSync(join(tmpdir(), "ts-closure-")));
const write = (path: string, text: string): void => {mkdirSync(dirname(join(workspace, path)), {recursive: true}); writeFileSync(join(workspace, path), text);};
write("tsconfig.json", "{}");
write("low/i.ts", "export type T = {x: number};\n");
write("mid/i.ts", "import {T} from \"../low/i.ts\";\nexport declare const m: T;\n");
write("top/i.ts", "import {m} from \"../mid/i.ts\";\nexport const v = m.x;\n");
const units = [
  {id: "dir:low", label: "low", files: ["low/i.ts"], json: [], dependencies: [], leaf: false},
  {id: "dir:mid", label: "mid", files: ["mid/i.ts"], json: [], dependencies: ["dir:low"], leaf: false},
  {id: "dir:top", label: "top", files: ["top/i.ts"], json: [], dependencies: ["dir:mid"], leaf: false},
];
const plan = {version: 1, family: "tsconfig.json", units, ambient: [], manifests: [], order: units.map(unit => unit.id), components: [], digest: ""};
const sha = (text: string | Uint8Array): string => createHash("sha256").update(text).digest("hex");
const host = {
  workspace, compiler: "fake", options: "o", environment: "e", family: "tsconfig.json", typeRoots: [], typeFiles: [], store: join(workspace, ".store"), session: "s", state: join(workspace, ".state"),
  digest: (path: string): string => sha(readFileSync(join(workspace, path))),
  tsc: async (config: string) => {
    const parsed = JSON.parse(readFileSync(config, "utf8")) as {compilerOptions: {outDir: string}; files: string[]};
    for (const file of parsed.files) {
      const source = readFileSync(file, "utf8").split("\n").filter(line => line.startsWith("export")).join("\n");
      const target = join(parsed.compilerOptions.outDir, relative(workspace, file).replace(/\.ts$/, ".d.ts"));
      mkdirSync(dirname(target), {recursive: true}); writeFileSync(target, source);
    }
    return {code: 0, output: ""};
  },
};
const run = async (): Promise<Record<string, string>> => Object.fromEntries((await engine.checkTypeScriptUnitsV1(plan, document, host, {concurrency: 1, signal: new AbortController().signal})).units.map((row: {id: string; status: string}) => [row.id, row.status]));
await run();
write("low/i.ts", "export type T = {y: number};\n");
const second = await run();
console.log(JSON.stringify(second));
console.log(second["dir:top"] === "hit" ? "UNSOUND: top stayed cached although low's exported type changed under it" : "SOUND: top was re-checked");
