import { describe, expect, test } from "bun:test";
import { captureOwnedCommandV1, runOwnedCommand } from "../../../🎛️owned-execution/🟦️.ts";
import { bunSpawnToolSyncV1, bunSpawnToolV1, execToolSyncV1, routeRegisteredCommandV1, spawnToolSyncV1, spawnToolV1 } from "../../🟦️.ts";

const poisoned = { ...process.env, NODE_OPTIONS: "--max-old-space-size=64", VSCODE_INSPECTOR_OPTIONS: "x", KEEP_ME: "1" };

describe("registry routing of commands", () => {
  test("routes a registered bare name and the running Bun to the registry's executable and sanitized environment", () => {
    const named = routeRegisteredCommandV1("bun", ["-v"], process.cwd(), poisoned), self = routeRegisteredCommandV1(process.execPath, ["-v"], process.cwd(), poisoned);
    for (const routed of [named, self]) {
      expect(routed.tool).toBe("bun");
      expect(routed.command).toBe(process.execPath);
      expect(routed.environment.NODE_OPTIONS).toBeUndefined();
      expect(routed.environment.VSCODE_INSPECTOR_OPTIONS).toBeUndefined();
      expect(routed.environment.KEEP_ME).toBe("1");
      expect(routed.environment.SEMIO_TOOL_RUN).toBe("bun");
    }
  });
  test("leaves unknown commands, explicit paths and a switched-off guard alone", () => {
    expect(routeRegisteredCommandV1("echo", ["x"], process.cwd(), poisoned).tool).toBeNull();
    expect(routeRegisteredCommandV1("/usr/bin/git", ["x"], process.cwd(), poisoned)).toMatchObject({ command: "/usr/bin/git", tool: null });
    expect(routeRegisteredCommandV1("bun", ["x"], process.cwd(), { ...poisoned, SEMIO_TOOL_GUARD: "off" })).toMatchObject({ command: "bun", tool: null });
  });
  test("follows the registry inside runOwnedCommand and captureOwnedCommandV1", async () => {
    const lines: string[] = [];
    await runOwnedCommand("bun", ["-e", "console.error(process.env.NODE_OPTIONS ?? 'none', process.env.SEMIO_TOOL_RUN)"], process.cwd(), "funnel", 0, { env: poisoned, stdout: "ignore", onLine: line => lines.push(line) });
    expect(lines).toEqual(["none bun"]);
    const tail = await captureOwnedCommandV1("bun", ["-e", "console.log(process.env.NODE_OPTIONS ?? 'none', process.env.SEMIO_TOOL_RUN)"], process.cwd(), { env: poisoned });
    expect(tail.trim()).toBe("none bun");
  });
  test("keeps Bun's call shapes in the Bun drop-ins", async () => {
    const program = ["-e", "console.log(process.env.SEMIO_TOOL_RUN, process.env.NODE_OPTIONS ?? 'none')"];
    expect(new TextDecoder().decode(bunSpawnToolSyncV1(["bun", ...program], { env: poisoned }).stdout).trim()).toBe("bun none");
    expect(new TextDecoder().decode(bunSpawnToolSyncV1({ cmd: [process.execPath, ...program], env: poisoned }).stdout).trim()).toBe("bun none");
    expect(new TextDecoder().decode(bunSpawnToolSyncV1(["git", "--version"]).stdout)).toStartWith("git version");
    const child = bunSpawnToolV1(["bun", ...program], { env: poisoned, stdout: "pipe" });
    expect((await new Response(child.stdout).text()).trim()).toBe("bun none");
    expect(await child.exited).toBe(0);
  });
  test("keeps Node's call shapes in the drop-in spawns", async () => {
    expect(spawnToolSyncV1("git", ["--version"], { encoding: "utf8" }).stdout).toStartWith("git version");
    expect(execToolSyncV1("git", ["--version"], { encoding: "utf8" })).toStartWith("git version");
    expect(Buffer.isBuffer(execToolSyncV1("git", ["--version"]))).toBe(true);
    const child = spawnToolV1("bun", ["-e", "console.log(process.env.SEMIO_TOOL_RUN)"], { env: poisoned, stdio: ["ignore", "pipe", "inherit"] });
    let out = "";
    child.stdout!.on("data", bytes => { out += bytes; });
    await new Promise(done => child.once("close", done));
    expect(out.trim()).toBe("bun");
  });
});
