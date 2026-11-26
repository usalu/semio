#!/usr/bin/env repo
import { resolve } from "node:path";
import { spawn } from "node:child_process";
import { setTimeout as sleep } from "node:timers/promises";
import { terminateOwnedChildTreeGracefullyV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts";
import { OwnedCommandFailure, ownedCommandExitCodeV1, type OwnedCommandOutcome } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📪️outcome/🟦️.ts";
import { newHostCapacityIdV1, readLiveHostCapacityReservationV1, withHostCapacityV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts";
import { sizeHostRequestV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import { hostCapacityDirectoryV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/📖️status/🟦️.ts";
import { ensureHostWardenV1, readHostWardenStateV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts";
import { BUILD_HOST_CAPACITY_POLICY_V1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🎛️control/🟦️.ts";
import { formatStampProtocolV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs";
import { runWithStaleReexecV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🔄️reexec/🟨️.mjs";
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
const restart = formatStampProtocolV1();
const handlers = (["SIGINT", "SIGTERM"] as const).map(name => [name, () => controller.abort(name)] as const);
const control = {
  signal,
  remainingMilliseconds: () => null,
  publish: () => undefined,
  yieldContinuation: () => new Promise<void>(done => setImmediate(done)),
};

function requireReservation(env: NodeJS.ProcessEnv): void {
  const grant = readLiveHostCapacityReservationV1(env, policy);
  if (grant?.retainReservation !== true || grant.reservedMegabytes < memory || grant.cpu < cpu) {
    throw Error(`native-exec requires a live retained reservation of ${memory} MiB and ${cpu} core(s)`);
  }
}

/** 🪂️ Launches a target with its original script envelope after validating the task grant. Native compiler controllers keep their own compiler admission and count under the task grant. */
async function launch(env: NodeJS.ProcessEnv): Promise<OwnedCommandOutcome> {
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
    const outcome = await new Promise<OwnedCommandOutcome>((accept, reject) => {
      child.once("error", reject);
      child.once("close", (status, signal) => accept({ version: 1, status, signal }));
    });
    signal.throwIfAborted();
    return outcome;
  } finally {
    try { await cleanup; }
    finally { signal.removeEventListener("abort", stop); }
  }
}

/** 🔄️ Reopens the Original child under the same live capacity grant when it yields to the repository restart protocol. */
async function launchWithRestart(env: NodeJS.ProcessEnv): Promise<number> {
  let status = 0;
  const outcome = await runWithStaleReexecV1(async () => {
    const terminal = await launch({ ...env });
    if (terminal.signal === null && terminal.status === restart.exitCode) throw new OwnedCommandFailure("native-exec", terminal);
    status = ownedCommandExitCodeV1(terminal);
  }, {
    environment: env,
    outermost: true,
    ranMilliseconds: 0,
    sleep: (milliseconds: number) => sleep(milliseconds, undefined, { signal }),
    launch: ({ environment }) => launchWithRestart(environment),
  });
  return outcome.status === "completed" ? status : outcome.code;
}

for (const [name, handler] of handlers) process.on(name, handler);
try {
  if (environment.SEMIO_HOST_CAPACITY_GRANT !== undefined || environment.SEMIO_HOST_CAPACITY_PLAN !== undefined) {
    process.exitCode = await launchWithRestart(environment);
  } else {
    const request = { ...command, fixedMegabytes: memory, minimumMegabytes: memory, cpuPerJob: cpu, retainReservation: true, label: "native-exec" };
    const session = currentAgentSessionV1({ environment });
    const capacityDirectory = hostCapacityDirectoryV1(environment);
    ensureHostWardenV1();
    process.exitCode = await withHostCapacityV1({
      request, policy, environment, control, directory: capacityDirectory, warden: () => readHostWardenStateV1(capacityDirectory),
      session: { address: agentSessionAddressV1(session), weight: session.budget.weight },
    }, handle => launchWithRestart({ ...environment, ...handle.environment }));
  }
} catch (error) {
  if (!signal.aborted) throw error;
  process.exitCode = signal.reason === "SIGINT" ? 130 : 143;
} finally {
  for (const [name, handler] of handlers) process.removeListener(name, handler);
}
