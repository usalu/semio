import { describe, expect, test } from "bun:test";
import { ToolErrorV1, toolForCommandV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { runToolsCommandV1 } from "../../⌨️command/🟦️.ts";

const context = (lines: string[]) => ({ environment: { ...process.env, SEMIO_LANGUAGE: "en" }, write: (line: string) => { lines.push(line); } });

describe("repo tools exec accepts an executable a registered tool owns", () => {
  test("the registry maps an owned executable to its tool", () => {
    expect(toolForCommandV1("rustc")?.id).toBe("cargo");
    expect(toolForCommandV1("bunx")?.id).toBe("bun");
    expect(toolForCommandV1("python3")?.id).toBe("python");
    expect(toolForCommandV1("not-a-tool")).toBeUndefined();
  });
  test("a tool id still runs", async () => {
    expect(await runToolsCommandV1(["exec", "--light", "node", "--", "--version"], context([]))).toBe(0);
  });
  test("an owned executable runs as that executable of its tool", async () => {
    expect(await runToolsCommandV1(["exec", "--light", "bunx", "--", "--version"], context([]))).toBe(0);
  });
  test("an unknown name is refused with the registered tools listed", async () => {
    const error = await runToolsCommandV1(["exec", "--light", "no-such-tool", "--"], context([])).catch((caught: unknown) => caught);
    expect(error).toBeInstanceOf(ToolErrorV1);
    expect(String((error as Error).message)).toContain("no-such-tool");
  });
});
