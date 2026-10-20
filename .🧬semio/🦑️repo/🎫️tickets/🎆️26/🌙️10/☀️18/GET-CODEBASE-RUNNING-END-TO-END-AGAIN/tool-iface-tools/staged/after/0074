import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import registryDocument from "../../🔣️.json" with { type: "json" };
import registrySchema from "../../🧬️schema/🔣️.json" with { type: "json" };
import { findToolV1, readToolPinV1, readToolRegistryV1, toolCallerAllowedV1, toolForCommandV1, toolGlobMatchesV1, toolMessageV1, toolRawSpawnAllowedV1, TOOL_REGISTRY_V1 } from "../../🟦️.ts";

const root = (() => { for (let directory = dirname(fileURLToPath(import.meta.url)); ; directory = dirname(directory)) { if (existsSync(join(directory, "MODULE.bazel"))) return directory; if (dirname(directory) === directory) return process.cwd(); } })();
const clone = (): any => JSON.parse(JSON.stringify(registryDocument));

describe("tool registry document", () => {
  test("validates against its schema with a third-party validator", () => {
    const ajv = new (Ajv as any)({ strict: false, allErrors: true });
    expect(ajv.validate(registrySchema, registryDocument)).toBe(true);
    expect(TOOL_REGISTRY_V1.tools.length).toBe(registryDocument.tools.length);
  });
  test("registers every tool the repository leans on", () => {
    for (const id of ["repo", "bun", "bazel", "nx", "cargo", "go", "uv", "python", "dotnet", "cmake", "ninja", "tsc", "vite", "vitest", "playwright", "tectonic", "wasm-opt", "jco", "hdiutil", "docker", "git", "gh"]) expect(findToolV1(id)).toBeDefined();
  });
  test("gives every command one owner and resolves aliases", () => {
    expect(toolForCommandV1("bunx")?.id).toBe("bun");
    expect(toolForCommandV1("/usr/bin/python3")?.id).toBe("python");
    expect(toolForCommandV1("cargo.exe")?.id).toBe("cargo");
    expect(toolForCommandV1("unknown")).toBeUndefined();
  });
  test("refuses duplicate ids, shared commands and unknown via tools", () => {
    const duplicate = clone(); duplicate.tools.push(duplicate.tools[1]);
    expect(() => readToolRegistryV1(duplicate)).toThrow("duplicate tool");
    const shared = clone(); shared.tools[2].commands.push("bun");
    expect(() => readToolRegistryV1(shared)).toThrow("belongs to");
    const via = clone(); via.tools.find((tool: any) => tool.id === "tsc").locate[1].via = "nope";
    expect(() => readToolRegistryV1(via)).toThrow("unknown tool nope");
    const bad = clone(); bad.tools[0].id = "Bad Id";
    expect(() => readToolRegistryV1(bad)).toThrow("invalid");
  });
  test("points at files that exist: adapters, installers and pin sources", () => {
    for (const tool of TOOL_REGISTRY_V1.tools) {
      expect(existsSync(join(root, tool.adapter)) || tool.adapter.includes("🧰️tools/")).toBe(true);
      if (tool.provision.installer !== undefined) expect(existsSync(join(root, tool.provision.installer))).toBe(true);
      const pin = readToolPinV1(tool, root);
      if (tool.pin.state === "pinned" || tool.pin.state === "constrained") expect(pin.version).not.toBeNull();
      if (tool.pin.digests !== undefined) expect(pin.digests.length).toBeGreaterThan(0);
      if (tool.pin.state === "floating") expect(tool.pin.gap).toBeDefined();
    }
  });
  test("reads the pins from the existing pin files instead of copying them", () => {
    const bun = readToolPinV1(findToolV1("bun")!, root), go = readToolPinV1(findToolV1("go")!, root);
    const bunFile = JSON.parse(readFileSync(join(root, findToolV1("bun")!.pin.digests!.file), "utf8")).definitions.Release.const;
    expect(bun.version).toBe(bunFile.version);
    expect(bun.digests.map(row => row.sha256).sort()).toEqual(bunFile.distributions.map((row: any) => row.sha256).sort());
    expect(go.digests.every(row => /^[a-f0-9]{64}$/u.test(row.sha256))).toBe(true);
    expect(new Set(go.digests.map(row => `${row.platform}-${row.architecture}`)).size).toBe(6);
  });
  test("renders every message in both languages with placeholders filled", () => {
    for (const key of Object.keys(TOOL_REGISTRY_V1.messages) as Array<keyof typeof TOOL_REGISTRY_V1.messages>) {
      const placeholders = (text: string) => [...text.matchAll(/\{(\w+)\}/gu)].map(match => match[1]).sort();
      expect(placeholders(TOOL_REGISTRY_V1.messages[key].en)).toEqual(placeholders(TOOL_REGISTRY_V1.messages[key].de));
    }
    expect(toolMessageV1("unknownTool", { tool: "x", known: "a, b" }, "de")).toBe("Unbekanntes Werkzeug x. Registrierte Werkzeuge: a, b.");
    expect(toolMessageV1("unknownTool", { tool: "x", known: "a" }, "en")).toContain("Unknown tool x");
  });
  test("matches globs and decides callers and raw spawns", () => {
    expect(toolGlobMatchesV1("a/**/c.ts", "a/b/d/c.ts")).toBe(true);
    expect(toolGlobMatchesV1("a/*/c.ts", "a/b/d/c.ts")).toBe(false);
    expect(toolGlobMatchesV1("**/🧪️tests/**", "x/🧪️tests/y.ts")).toBe(true);
    const cargo = findToolV1("cargo")!;
    expect(toolCallerAllowedV1(cargo, "✏️s/🔌️plugins/x/🏭️generator/🟦️.ts")).toBe(false);
    expect(toolCallerAllowedV1(cargo, "✏️s/🔌️plugins/x/🧪️tests/🟦️.ts")).toBe(true);
    expect(toolRawSpawnAllowedV1(cargo, cargo.adapter)).toBe(true);
    expect(toolRawSpawnAllowedV1(cargo, "✏️s/x/🟦️.ts")).toBe(false);
  });
});
