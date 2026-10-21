import { afterAll, afterEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { decideToolSpawnV1, findToolV1, installToolGuardV1, ToolErrorV1 } from "../../🟦️.ts";

const root = mkdtempSync(join(tmpdir(), "tool-guard-"));
mkdirSync(join(root, "x"));
writeFileSync(join(root, "x", "caller.ts"), "const calls: number[] = [];\nexport const sync = (cmd: string[]) => { const result = Bun.spawnSync(cmd); calls.push(1); return result; };\nexport const async = (cmd: string[]) => { const child = (Bun as any).spawn({ cmd }); calls.push(1); return child; };\n");
const caller = await import(join(root, "x", "caller.ts"));
const me = "x/caller.ts";
let uninstall: (() => void) | undefined;
afterEach(() => { uninstall?.(); uninstall = undefined; });
afterAll(() => rmSync(root, { recursive: true, force: true }));

describe("runtime spawn guard", () => {
  test("decides per command, caller and mode", () => {
    const cargo = findToolV1("cargo")!;
    expect(decideToolSpawnV1("cargo", "a/b.ts", {}).action).toBe("refuse");
    expect(decideToolSpawnV1("/usr/bin/cargo", cargo.adapter, {}).action).toBe("allow");
    expect(decideToolSpawnV1("cargo", "a/b.ts", { mode: "warn" }).action).toBe("warn");
    expect(decideToolSpawnV1("cargo", "a/b.ts", { mode: "off" }).action).toBe("allow");
    expect(decideToolSpawnV1("cargo", "a/b.ts", { tolerated: new Set(["a/b.ts\0cargo"]) }).action).toBe("allow");
    expect(decideToolSpawnV1("echo", "a/b.ts", {})).toEqual({ action: "allow", tool: null });
    expect(decideToolSpawnV1("cargo", undefined, {}).action).toBe("allow");
  });
  test("refuses Bun.spawnSync of a registered tool from a repository file and passes other commands", () => {
    uninstall = installToolGuardV1({ root, mode: "refuse" });
    expect(() => caller.sync(["git", "--version"])).toThrow(ToolErrorV1);
    expect(() => caller.async(["cargo", "--version"])).toThrow("runToolV1(\"cargo\"");
    expect(new TextDecoder().decode(caller.sync(["echo", "fine"]).stdout).trim()).toBe("fine");
  });
  test("tolerates a ledgered caller, warns once in warn mode and restores Bun on removal", () => {
    const lines: string[] = [];
    uninstall = installToolGuardV1({ root, mode: "warn", write: line => lines.push(line), environment: { SEMIO_LANGUAGE: "de" } });
    caller.sync(["git", "--version"]);
    caller.sync(["git", "--version"]);
    expect(lines.length).toBe(1);
    expect(lines[0]).toContain("Direkter Start von git");
    uninstall();
    const before = Bun.spawnSync;
    uninstall = installToolGuardV1({ root, mode: "refuse", tolerated: new Set([`${me}\0git`]) });
    expect(caller.sync(["git", "--version"]).exitCode).toBe(0);
    uninstall();
    uninstall = undefined;
    expect(Bun.spawnSync).toBe(before);
  });
});
