/** 🧪 Lays out the restore fixture inside the ticket scratch tree and writes the build graph that selects it. */
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
const WS = resolve(import.meta.dir, "../../../../../../../../.."), G = join(import.meta.dir, "../../🗑️generated/cachefix-bz");
const FX = join(G, "fx/restore"), root = relative(WS, FX);
const mutate = process.argv[2] === "--mutate";
if (mutate) { writeFileSync(join(FX, "input.json"), JSON.stringify({ values: [1, 2, 3, 4, 5] }) + "\n"); console.log("mutated input.json"); process.exit(0); }
rmSync(FX, { recursive: true, force: true });
mkdirSync(FX, { recursive: true });
writeFileSync(join(FX, "input.json"), JSON.stringify({ values: [1, 2, 3, 4] }) + "\n");
writeFileSync(join(FX, "gen.ts"), `import { chmodSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
const input = JSON.parse(readFileSync("input.json", "utf8"));
mkdirSync("out/sub", { recursive: true });
mkdirSync("dist", { recursive: true });
writeFileSync("out/a.txt", "alpha " + input.values.join(",") + "\\n");
writeFileSync("out/sub/b.bin", Uint8Array.from({ length: 256 }, (_, i) => i));
writeFileSync("out/run.sh", "#!/bin/sh\\necho restored\\n");
chmodSync("out/run.sh", 0o755);
writeFileSync("dist/c.json", JSON.stringify({ squares: input.values.map((v: number) => v * v) }) + "\\n");
writeFileSync("extra.txt", "undeclared\\n");
console.log("generated");
`);
writeFileSync(join(FX, "consume.ts"), `import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
mkdirSync("result", { recursive: true });
writeFileSync("result/r.txt", readFileSync("out/a.txt", "utf8").toUpperCase());
console.log("consumed");
`);
writeFileSync(join(FX, "stamp.ts"), `import { mkdirSync, writeFileSync } from "node:fs";
mkdirSync("stamp", { recursive: true });
writeFileSync("stamp/s.txt", String(Date.now()) + "\\n");
console.log("stamped");
`);
writeFileSync(join(FX, "missing.ts"), `console.log("produces nothing");\n`);
const t = (command: string, inputs: unknown[], outputs: string[] | undefined, extra: object = {}) => ({ execution: { kind: "command", command, cwd: root }, inputs, ...(outputs ? { outputs } : {}), cache: true, ...extra });
const graph = { version: 1, projects: { "fx-restore": { name: "fx-restore", root, targets: {
  generate: t("bun ./gen.ts", ["{projectRoot}/gen.ts", "{projectRoot}/input.json"], ["{projectRoot}/out", "{projectRoot}/dist/c.json"]),
  consume: t("bun ./consume.ts", ["{projectRoot}/consume.ts", { dependentTasksOutputFiles: "**/*" }], ["{projectRoot}/result"], { dependsOn: ["generate"] }),
  stamp: t("bun ./stamp.ts", ["{projectRoot}/stamp.ts"], ["{projectRoot}/stamp"], { cache: false }),
  missing: t("bun ./missing.ts", ["{projectRoot}/missing.ts"], ["{projectRoot}/never"]),
} } }, dependencies: [], namedInputs: {} };
writeFileSync(join(G, "graph.json"), JSON.stringify(graph));
console.log("fixture ready", root);
