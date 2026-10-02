import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import contract from "../../🧫️fixtures/🚮️projection/🔣️.json";
import schema from "../../🧬️schema/🚮️projection/🔣️.json";

const owner = resolve(fileURLToPath(new URL("../..", import.meta.url)));
const root = process.cwd();

test("closed deletion projections retain hostile absence and every original roster", () => {
  const admit = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(admit(contract)).toBe(true);
  const duplicate = structuredClone(contract);
  duplicate.projections[1] = duplicate.projections[0] as typeof duplicate.projections[1];
  expect(admit(duplicate)).toBe(false);
  const subset = structuredClone(contract);
  subset.projections[1].scenarios!.pop();
  expect(admit(subset)).toBe(false);
  const retained = structuredClone(contract);
  retained.projections[0].absent = [];
  expect(admit(retained)).toBe(false);
});

test("independent Node reads the exact authored adapters and complete feature groups", () => {
  const rows = contract.projections.filter(row => "feature" in row);
  const oracle = Bun.spawnSync(["node", "-e", String.raw`const fs=require("node:fs");const rows=JSON.parse(process.argv[1]);for(const row of rows){const text=fs.readFileSync(row.feature,"utf8");for(const s of row.scenarios){if(!text.includes("@id-"+s.id))throw Error(s.id);for(const id of s.examples){const count=text.split("\n").filter(l=>new RegExp("^\\s*\\|\\s*"+id+"\\s*\\|").test(l)).length;if(count!==2)throw Error(id+":"+count);}}const a=fs.readFileSync(row.adapter,"utf8");if(!a.includes("pub fn adapter()"))throw Error(row.adapter);}console.log(JSON.stringify(rows.map(r=>({id:r.id,scenarios:r.scenarios.reduce((n,s)=>n+Math.max(1,s.examples.length),0)}))));`, JSON.stringify(rows)], { cwd: root });
  expect(oracle.exitCode, oracle.stderr.toString()).toBe(0);
  expect(JSON.parse(oracle.stdout.toString())).toEqual([{ id: "png-pdf-absent", scenarios: 3 }, { id: "ifc2x3-step-absent", scenarios: 9 }]);
  expect(readFileSync(resolve(root, contract.projections[2].manifest!), "utf8")).toContain('name = "authored_assembly"');
});

test("owned producer admits current physical projection inputs before any compiler", () => {
  const result = Bun.spawnSync(["bun", resolve(owner, "📜️script.ts"), "projection-plan", root], { cwd: root });
  expect(result.exitCode, result.stderr.toString()).toBe(0);
  const receipt = JSON.parse(result.stdout.toString());
  expect(receipt.contract).toEqual(contract);
  expect(receipt.materialized).toBe(false);
  expect(receipt.nativeExecuted).toBe(false);
  expect(receipt.inputs.length).toBe(5);
  expect(receipt.inputs.every((row: { sha256: string; bytes: number }) => /^[a-f0-9]{64}$/u.test(row.sha256) && row.bytes > 0)).toBe(true);
});

test("materializer refuses unauthored deletion scopes before touching a workspace", () => {
  const result = Bun.spawnSync(["bun", resolve(owner, "📜️script.ts"), "projection-materialize", root, "foreign-owner"], { cwd: root });
  expect(result.exitCode).not.toBe(0);
  expect(result.stdout.toString()).toBe("");
  expect(result.stderr.toString()).toContain("Unknown authored deletion projection");
});
