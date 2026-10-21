import { describe, expect, test } from "bun:test";
import registryDocument from "../../🔣️.json" with { type: "json" };
import { findToolV1, probeToolV1, readToolRegistryV1, resolveToolV1, runToolSyncV1, runToolV1, systemToolHostV1, toolCallerFileV1, ToolErrorV1, type ToolAdmissionV1, type ToolProgressV1 } from "../../🟦️.ts";

const row = (id: string, command: string, capacity: unknown, callers: string[]) => ({
  id, kind: "runtime", summary: { en: "Probe tool.", de: "Prüfwerkzeug." }, capabilities: ["run"], commands: [command], platforms: ["darwin", "linux", "win32"], pin: { state: "system" },
  provision: { kind: "system", command: "install it" }, locate: [{ kind: "environment", name: "PROBE_" + id.toUpperCase() }], probe: { arguments: ["--version"], pattern: "^([0-9]+\\.[0-9]+\\.[0-9]+)" },
  adapter: "x/adapter.ts", adapterScope: [], allowedCallers: callers, environment: { mode: "inherit", pass: [], deny: ["LEAK_*"], set: { FIXED: "yes" } }, capacity,
});
const document = JSON.parse(JSON.stringify(registryDocument));
document.tools.push(row("heavy", "heavy-tool", { mode: "admit", operation: "command", kind: "build" }, ["only/here/**"]), row("light", "light-tool", { mode: "none" }, ["**"]));
const registry = readToolRegistryV1(document);
const environment = { ...process.env, PROBE_HEAVY: process.execPath, PROBE_LIGHT: process.execPath, LEAK_ME: "1", SEMIO_LANGUAGE: "en" };
const options = { registry, environment, trusted: true, root: process.cwd() };
const program = (source: string) => ["-e", source];

describe("runToolV1", () => {
  test("captures output under the sanitized environment of the registry", async () => {
    const result = await runToolV1("heavy", program("console.log(process.env.FIXED, process.env.SEMIO_TOOL_RUN, process.env.LEAK_ME)"), { ...options, capture: true, admission: false });
    expect(result.stdout.trim()).toBe("yes heavy undefined");
    expect(result.status).toBe(0);
    expect(result.resolution.source).toBe("environment");
  });
  test("throws on a failure by default and reports it with check false", async () => {
    await expect(runToolV1("light", program("process.exit(3)"), { ...options, capture: true })).rejects.toMatchObject({ code: "failed", status: 3 });
    expect((await runToolV1("light", program("process.exit(3)"), { ...options, capture: true, check: false })).status).toBe(3);
    expect((await runToolV1("light", program("process.exit(4)"), { ...options, check: false, stdout: "ignore" })).status).toBe(4);
  });
  test("streams lines of a tool to the caller", async () => {
    const lines: string[] = [];
    await runToolV1("light", program("console.error('from the tool')"), { ...options, stdout: "ignore", onLine: (line, stream) => lines.push(`${stream}:${line}`) });
    expect(lines).toEqual(["stderr:from the tool"]);
  });
  test("refuses an unknown tool and a caller the registry does not allow, in the chosen language", async () => {
    await expect(runToolV1("nope", [], options)).rejects.toMatchObject({ code: "unknown-tool" });
    const refused = await runToolV1("heavy", [], { ...options, trusted: false, caller: "other/place.ts", admission: false }).catch(error => error as ToolErrorV1);
    expect(refused).toMatchObject({ code: "caller-refused" });
    expect(refused.message).toContain("other/place.ts may not run heavy");
    const german = await runToolV1("heavy", [], { ...options, trusted: false, caller: "other/place.ts", environment: { ...environment, SEMIO_LANGUAGE: "de" } }).catch(error => error as ToolErrorV1);
    expect(german.message).toContain("darf heavy nicht ausführen");
    await expect(runToolV1("heavy", program("0"), { ...options, trusted: false, caller: "only/here/a.ts", capture: true, admission: false })).resolves.toMatchObject({ status: 0 });
    await expect(runToolV1("heavy", program("0"), { ...options, trusted: false, caller: "other/place.ts", environment: { ...environment, SEMIO_TOOL_GUARD: "off" }, capture: true, admission: false })).resolves.toMatchObject({ status: 0 });
  });
  test("names the provisioning command when a tool is missing", async () => {
    const missing = await runToolV1("light", [], { ...options, environment: { SEMIO_LANGUAGE: "de" } }).catch(error => error as ToolErrorV1);
    expect(missing).toMatchObject({ code: "tool-missing" });
    expect(missing.message).toContain("nicht gefunden");
    expect(missing.message).toContain("install it");
  });
  test("cancels a running tool and its process tree", async () => {
    for (const capture of [true, false]) {
      const controller = new AbortController(), started = performance.now();
      setTimeout(() => controller.abort(), 150);
      await expect(runToolV1("light", program("setTimeout(() => {}, 30000)"), { ...options, capture, signal: controller.signal, stdout: "ignore" })).rejects.toMatchObject({ code: "cancelled" });
      expect(performance.now() - started).toBeLessThan(10000);
    }
  });
  test("ends a captured run at its deadline", async () => {
    const result = await runToolV1("light", program("setTimeout(() => {}, 30000)"), { ...options, capture: true, timeoutMs: 200, check: false });
    expect(result.status).toBeNull();
    expect(result.signal).not.toBeNull();
  });
  test("waits for machine capacity before a heavy tool starts and runs it with the grant", async () => {
    const asks: string[] = [], events: ToolProgressV1["phase"][] = [];
    const admission: ToolAdmissionV1 = async (ask, body) => { asks.push(`${ask.tool}:${ask.operation}:${ask.kind}`); return body({ environment: { SEMIO_GRANT: "granted" }, jobs: 2 }); };
    const result = await runToolV1("heavy", program("console.log(process.env.SEMIO_GRANT)"), { ...options, capture: true, admission, onProgress: event => events.push(event.phase) });
    expect(result.stdout.trim()).toBe("granted");
    expect(asks).toEqual(["heavy:command:build"]);
    expect(events).toEqual(["admitting", "running", "finished"]);
    await runToolV1("light", program("0"), { ...options, capture: true, admission });
    expect(asks.length).toBe(1);
  });
  test("passes a cancellation that arrives while waiting for capacity to the gate", async () => {
    const controller = new AbortController();
    const admission: ToolAdmissionV1 = (_ask, _body, signal) => new Promise((_, reject) => signal!.addEventListener("abort", () => reject(new ToolErrorV1("cancelled", "heavy", "waiting cancelled"))));
    setTimeout(() => controller.abort(), 50);
    await expect(runToolV1("heavy", program("0"), { ...options, capture: true, admission, signal: controller.signal })).rejects.toMatchObject({ code: "cancelled" });
  });
});

describe("runToolSyncV1", () => {
  test("runs a light tool and refuses a heavy one", () => {
    const result = runToolSyncV1("light", program("console.log('sync')"), options);
    expect(result.stdout.trim()).toBe("sync");
    expect(() => runToolSyncV1("heavy", program("0"), options)).toThrow("cannot run synchronously");
    expect(runToolSyncV1("light", program("process.exit(2)"), options).status).toBe(2);
    expect(() => runToolSyncV1("light", program("process.exit(2)"), { ...options, check: true })).toThrow("light failed");
  });
});

describe("caller and probe", () => {
  test("accepts the declared caller as a path, a file URL or an absolute path", async () => {
    const allowed = { ...options, trusted: false, capture: true, admission: false };
    for (const caller of ["only/here/a.ts", "file://" + process.cwd() + "/only/here/a.ts", process.cwd() + "/only/here/a.ts"]) await expect(runToolV1("heavy", program("0"), { ...allowed, caller })).resolves.toMatchObject({ status: 0 });
    await expect(runToolV1("heavy", program("0"), { ...allowed, caller: process.cwd() + "/elsewhere/a.ts" })).rejects.toMatchObject({ code: "caller-refused" });
    await expect(runToolV1("heavy", program("0"), { ...allowed, caller: "elsewhere/a.ts", environment: { ...environment, SEMIO_TOOL_GUARD: "warn" } })).resolves.toMatchObject({ status: 0 });
  });
  test("finds the first stack frame outside the tools module relative to the root", () => {
    const stack = ["Error", "    at runToolV1 (/r/🧰️framework/🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts:1:1)", "    at job (file:///r/a/b%20c.ts:3:4)", "    at x (/other/z.ts:1:1)"].join("\n");
    expect(toolCallerFileV1("/r", stack)).toBe("a/b c.ts");
    expect(toolCallerFileV1("/elsewhere", "    at f (/r/a.ts:1:1)")).toBeUndefined();
    expect(toolCallerFileV1("/r", "")).toBeUndefined();
  });
  test("asks a resolved tool for its version", () => {
    const entry = findToolV1("light", registry)!, resolution = resolveToolV1(entry, systemToolHostV1(process.cwd(), environment), registry);
    expect(probeToolV1(entry, resolution, environment)).toBe(Bun.version);
  });
});
