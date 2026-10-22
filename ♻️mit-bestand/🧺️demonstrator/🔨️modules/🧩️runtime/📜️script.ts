#!/usr/bin/env repo
import {scriptCompilerAdmissionControlV1} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {frameworkOsViteArtifactInputsV1} from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🏗️builder/🌐️vite/📥️artifacts/🟦️.ts";
import { repositoryCachePartitionRootV1 } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { DEMONSTRATOR_RUNTIME_TARGETS, demonstratorRuntimeModuleLayout } from "./🟦️.ts";
import { serveVite, closeServiceSession, openServiceSession, publishServiceReady, readServiceSession } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts";
import { readDemonstratorActivation } from "./♻️activation/🟦️.ts";
import { builtPluginModulesRootIn, playgroundSessionsRootV1, playgroundSessionStagedOutputPath } from "@semio-tech/framework-os-dev/activation";
import { DEMONSTRATOR_E2E_OWNER, demonstratorE2eInvocationPid, demonstratorE2eSessionRoot } from "./🧪️e2e/🟦️.ts";
import { readBuildMode } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";

/** 🎪️ Verifies the runtime union completed by the outer build preparation graph. */
class PreparationScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Preparation mode belongs to the build tool's prototype or release configuration");
    const mode = readBuildMode(process.env.SEMIO_BUILD_MODE), profile = mode;
    const moduleRoot = builtPluginModulesRootIn(this.repoRoot, profile), sessionsRoot = playgroundSessionsRootV1(this.repoRoot, profile);
    const { pluginModuleDirNames, extensionModuleDirNames } = demonstratorRuntimeModuleLayout(DEMONSTRATOR_RUNTIME_TARGETS.map(row => row.pluginId));
    for (const name of [...pluginModuleDirNames.slice(2), ...extensionModuleDirNames]) {
      const directory = join(moduleRoot, name), marker = JSON.parse(readFileSync(join(directory, ".semio-artifact.json"), "utf8"));
      if (!Array.isArray(marker.files) || !marker.files.includes("🌉️bridge.js") || !marker.files.includes("🔣️.json") || marker.files.some((file: string) => !existsSync(join(directory, file)))) throw new Error(`Incomplete Demonstrator runtime component: ${name}`);
    }
    for (const target of DEMONSTRATOR_RUNTIME_TARGETS) {
      const file = playgroundSessionStagedOutputPath(sessionsRoot, target.variant);
      const session = (await import(pathToFileURL(file).href)).PLAYGROUND_SESSION;
      if (session.variant !== target.variant || session.registryPluginId !== target.pluginId) throw new Error(`Mismatched Demonstrator session: ${target.variant}`);
    }
    console.log(`Prepared Demonstrator ${mode}: ${DEMONSTRATOR_RUNTIME_TARGETS.length} runtime variants`);
  }
}

/** 📡️ Validates activation after all outer preparation prerequisites have completed. */
class ActivationScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Demonstrator activation accepts no arguments");
    const { receipt, laneReceiptDirectories } = readDemonstratorActivation(this.repoRoot);
    console.log(`Activated Demonstrator prototype: ${receipt.plugins.length} completed components from ${laneReceiptDirectories.length} lanes`);
  }
}

/** 🖥️ Serves completed runtime artifacts for the lifetime owned by the build tool. */
class ServeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Demonstrator serving accepts no arguments");
    readDemonstratorActivation(this.repoRoot);
    const root = resolve(this.root, "../.."), controller = new AbortController();
    const interrupt = (): void => { process.exitCode = 130; controller.abort(); }, terminate = (): void => { process.exitCode = 143; controller.abort(); };
    process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
    try {
      await serveVite({control:scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), invocation: this.invocation, mode: readBuildMode(process.env.SEMIO_BUILD_MODE), artifactInputs: frameworkOsViteArtifactInputsV1(repositoryCachePartitionRootV1(this.repoRoot, "products"), {version:1,mode:readBuildMode(process.env.SEMIO_BUILD_MODE),renderer:"react",command:"serve",engines:"play"}), root, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), configLoader: "native", host: process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1", port: Number(process.env.MIT_BESTAND_DEMONSTRATOR_PORT ?? 6029), signal: this.invocation.control.signal, ready: url => console.log(`Demonstrator ready: ${url}`) });
    } finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); }
  }
}

/** 🆕️ Prepares an invocation-specific E2E generation after its runtime and browser prerequisites. */
class PrepareTestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Demonstrator E2E preparation accepts no arguments");
    readDemonstratorActivation(this.repoRoot);
    const session = await openServiceSession(demonstratorE2eSessionRoot(this.repoRoot), DEMONSTRATOR_E2E_OWNER, demonstratorE2eInvocationPid(process.env));
    console.log(`Prepared Demonstrator E2E service ${session.id}`);
  }
}

/** 🧪️ Owns the isolated E2E listener and announces its prepared generation over HTTP. */
class ServeTestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Demonstrator E2E serving accepts no arguments");
    const sessionRoot = demonstratorE2eSessionRoot(this.repoRoot), session = readServiceSession(sessionRoot, DEMONSTRATOR_E2E_OWNER, demonstratorE2eInvocationPid(process.env));
    const root = resolve(this.root, "../.."), controller = new AbortController();
    const interrupt = (): void => { process.exitCode = 130; controller.abort(); }, terminate = (): void => { process.exitCode = 143; controller.abort(); };
    process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
    try {
      readDemonstratorActivation(this.repoRoot);
      await serveVite({control:scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), invocation: this.invocation, mode: readBuildMode(process.env.SEMIO_BUILD_MODE), artifactInputs: frameworkOsViteArtifactInputsV1(repositoryCachePartitionRootV1(this.repoRoot, "products"), {version:1,mode:readBuildMode(process.env.SEMIO_BUILD_MODE),renderer:"react",command:"serve",engines:"play"}), root, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), configLoader: "native", host: "127.0.0.1", port: 0, signal: this.invocation.control.signal, session, ready: async url => { await publishServiceReady(sessionRoot, session, url, controller.signal); console.log(`Demonstrator E2E ready: ${url}`); } });
    } finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); await closeServiceSession(sessionRoot, session); }
  }
}

const router = new ScriptRouter(import.meta.dir).register("prepare", PreparationScript).register("activate", ActivationScript).register("serve", ServeScript).register("prepare-test", PrepareTestScript).register("serve-test", ServeTestScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));
