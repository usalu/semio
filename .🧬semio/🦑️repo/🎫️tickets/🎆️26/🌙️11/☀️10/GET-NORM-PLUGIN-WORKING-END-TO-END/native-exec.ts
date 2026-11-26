#!/usr/bin/env repo
import { resolve } from "node:path";
import { spawn } from "node:child_process";
import { terminateOwnedChildTreeGracefullyV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts";
import { newHostCapacityIdV1, readLiveHostCapacityReservationV1, withHostCapacityV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts";
import { sizeHostRequestV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import { hostCapacityDirectoryV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/📖️status/🟦️.ts";
import { ensureHostWardenV1, readHostWardenStateV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts";
import { BUILD_HOST_CAPACITY_POLICY_V1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🎛️control/🟦️.ts";
import { agentSessionAddressV1, currentAgentSessionV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🪪️agent-sessions/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../..");
const [cwd, ...argv] = process.argv.slice(2);
if (!cwd || !argv[0]) throw Error("Usage: native-exec.ts <cwd> <script> [args...]");
const directory = resolve(root, cwd);
const program = resolve(directory, argv[0]);
const target = process.env.NATIVE_EXEC_TARGET ?? "@semio-tech/framework-os-dev:activate-din4108-react-prototype";
const environment = { SEMIO_BUILD_MODE: "prototype", ...process.env } as NodeJS.ProcessEnv;
const policy = BUILD_HOST_CAPACITY_POLICY_V1;
const command = sizeHostRequestV1(policy, { id: newHostCapacityIdV1(), operation: "command" });
if (!command) throw Error("Repository command capacity policy is missing");
const browser = target === "@semio-tech/semio-tech-play:serve-standards" ||
  program === resolve(root, "🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/📜️script.ts") && argv[1] === "serve";
const browserBudget = browser && policy.envelopes.find(row => row.name === "end-to-end");
if (browser && !browserBudget) throw Error("Repository end-to-end capacity envelope is missing");
const memory = browserBudget ? browserBudget.megabytes : command.fixedMegabytes;
const cpu = browserBudget ? browserBudget.cpu : command.cpuPerJob;
const controller = new AbortController();
const signal = controller.signal;
const handlers = (["SIGINT", "SIGTERM"] as const).map(name => [name, () => controller.abort(name)] as const);
const control = {
  signal,
  remainingMilliseconds: () => null,
  publish: () => undefined,
  yieldContinuation: () => new Promise<void>(done => setImmediate(done)),
};

function requireReservation(env: NodeJS.ProcessEnv): void {
  const grant = readLiveHostCapacityReservationV1(env, policy);
  if (!grant || grant.reservedMegabytes < memory || grant.cpu < cpu) {
    throw Error(`native-exec requires a live reservation of ${memory} MiB and ${cpu} core(s)`);
  }
}

/** 🪂️ Launches a target with its original script envelope after validating the task grant. Native compiler controllers keep their own compiler admission and count under the task grant. */
async function launch(env: NodeJS.ProcessEnv): Promise<number> {
  requireReservation(env);
  signal.throwIfAborted();
  const { createScriptProcessEnvelope, readScriptPolicy, scriptProcessEnvironment } = await import(resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
  const { createBuildBootstrapCapabilitiesV1 } = await import(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📤️child/🟦️.ts"));
  const { issueRepositoryScriptProvidersV1 } = await import(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts"));
  issueRepositoryScriptProvidersV1(env);
  const envelope = createScriptProcessEnvelope(readScriptPolicy({ version: 1, owner: "native-exec", maximumElapsedMilliseconds: 0 }), await createBuildBootstrapCapabilitiesV1(root, ["run", target], env), Date.now());
  requireReservation(env);
  signal.throwIfAborted();
  const child = spawn("repo", argv, { cwd: directory, stdio: "inherit", windowsHide: true, env: scriptProcessEnvironment(envelope, env) as NodeJS.ProcessEnv });
  let cleanup: Promise<void> | undefined;
  const stop = () => {
    cleanup ??= terminateOwnedChildTreeGracefullyV1(child, { initialSignal: signal.reason === "SIGINT" ? "SIGINT" : "SIGTERM" });
  };
  signal.addEventListener("abort", stop, { once: true });
  try {
    const status = await new Promise<number>((accept, reject) => {
      child.once("error", reject);
      child.once("exit", (code, reason) => accept(code ?? (reason === "SIGINT" ? 130 : reason === "SIGTERM" ? 143 : reason === "SIGKILL" ? 137 : 1)));
    });
    signal.throwIfAborted();
    return status;
  } finally {
    try { await cleanup; }
    finally { signal.removeEventListener("abort", stop); }
  }
}

for (const [name, handler] of handlers) process.on(name, handler);
try {
  if (environment.SEMIO_HOST_CAPACITY_GRANT !== undefined || environment.SEMIO_HOST_CAPACITY_PLAN !== undefined) {
    process.exitCode = await launch(environment);
  } else {
    const request = { ...command, fixedMegabytes: memory, minimumMegabytes: memory, cpuPerJob: cpu, label: "native-exec" };
    const session = currentAgentSessionV1({ environment });
    const capacityDirectory = hostCapacityDirectoryV1(environment);
    ensureHostWardenV1();
    process.exitCode = await withHostCapacityV1({
      request, policy, environment, control, directory: capacityDirectory, warden: () => readHostWardenStateV1(capacityDirectory),
      session: { address: agentSessionAddressV1(session), weight: session.budget.weight },
    }, handle => launch({ ...environment, ...handle.environment }));
  }
} catch (error) {
  if (!signal.aborted) throw error;
  process.exitCode = signal.reason === "SIGINT" ? 130 : 143;
} finally {
  for (const [name, handler] of handlers) process.removeListener(name, handler);
}
