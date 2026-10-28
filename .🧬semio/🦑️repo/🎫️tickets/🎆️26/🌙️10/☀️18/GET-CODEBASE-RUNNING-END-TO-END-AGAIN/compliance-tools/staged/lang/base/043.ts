import { existsSync, realpathSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { declareBuildTestEnvelopeV1, registerBuildTestGrantIssuerV1, type BuildTestGrantIssuerV1, type BuildTestGrantV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import { processTestProtocolV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🗂️suites/🟦️.ts";
import { testSandboxEnvironmentV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { createBuildBootstrapCapabilitiesV1 } from "../../🏗️build/📥️caller/📤️child/🟦️.ts";
import { issueRepositoryScriptProvidersV1 } from "../../🏃️process/🔌️providers/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../🏃️process/🌿️environment/🧪️test-output/🟦️.ts";

/** 🧭️ Finds the checkout a bare test file runs in: the nearest directory at or above the working directory that holds `MODULE.bazel`, as the suite runner finds it. */
function nativeBirthCheckoutV1(start: string = process.cwd()): string {
  for (let directory = resolve(start); ; directory = dirname(directory)) {
    if (existsSync(join(directory, "MODULE.bazel"))) return realpathSync(directory);
    if (dirname(directory) === directory) throw Error("Checkout holding MODULE.bazel required for a bare library test");
  }
}

/** 🌿️ Names the environment the repository's official test birth hands a test file: the checkout as workspace root, the prototype build mode, the repository script providers and an artifact directory, the configured one or a per-run sandbox below the scratch the suite row declares. */
function nativeBirthEnvironmentV1(grant: BuildTestGrantV1, base: NodeJS.ProcessEnv): Readonly<{ environment: NodeJS.ProcessEnv; release(): void }> {
  const root = nativeBirthCheckoutV1(), environment: NodeJS.ProcessEnv = { ...base, SEMIO_WORKSPACE_ROOT: root };
  environment.SEMIO_BUILD_MODE ??= "prototype";
  let release = (): void => {};
  if (environment.SEMIO_TEST_ARTIFACT_DIR?.trim()) Object.assign(environment, repoTestArtifactEnvironment(root, "build-contract", environment));
  else {
    const sandbox = testSandboxEnvironmentV1(environment, { route: "build-contract", checkout: root, ...(grant.scratch === "checkout" ? { parent: join(root, processTestProtocolV1.checkoutScratchDirectory) } : {}) });
    Object.assign(environment, sandbox.environment);
    release = () => sandbox.release();
  }
  issueRepositoryScriptProvidersV1(environment);
  return { environment, release };
}

/** 🏭️ Issues the native build caller grant of a library test: the bootstrap capabilities of the declared build command, derived exactly as the official runner derives them for the target that runs the file. */
const nativeBuildCallerIssuerV1: BuildTestGrantIssuerV1 = {
  prepare(grant, base) {
    const { environment, release } = nativeBirthEnvironmentV1(grant, base), changed: Record<string, string | undefined> = {};
    for (const name of new Set([...Object.keys(base), ...Object.keys(environment)])) if (environment[name] !== base[name]) changed[name] = environment[name];
    return { environment: changed, release };
  },
  capabilities: (grant, environment) => createBuildBootstrapCapabilitiesV1(environment.SEMIO_WORKSPACE_ROOT ?? nativeBirthCheckoutV1(), grant.arguments, environment),
  async matches(grant, inherited, environment) {
    const actual = inherited as { arguments?: unknown; buildMode?: unknown; tool?: unknown; workspaceRoot?: unknown };
    if (!Array.isArray(actual.arguments) || actual.arguments.some(argument => typeof argument !== "string") || typeof actual.workspaceRoot !== "string") return false;
    const root = realpathSync(actual.workspaceRoot);
    try {
      const expected = await createBuildBootstrapCapabilitiesV1(root, actual.arguments as string[], { ...environment, SEMIO_WORKSPACE_ROOT: root, ...(typeof actual.buildMode === "string" ? { SEMIO_BUILD_MODE: actual.buildMode } : {}), ...(typeof actual.tool === "string" ? { SEMIO_BUILD_TOOL: actual.tool } : {}) });
      return JSON.stringify(expected) === JSON.stringify(inherited) && grant.kind === "native-build-caller";
    } catch { return false; }
  },
};
registerBuildTestGrantIssuerV1("native-build-caller", nativeBuildCallerIssuerV1);

/** 🪪️ Names the suite row a library test file belongs to: its id, which fixes the owner and the build command `@semio-tech/repo-lib:test-<suite>` whose grant the file is born with, and the scratch the row declares. @see ../../🏗️build/🗂️suites/🔣️.json */
export type LibraryBuildTestEnvelopeV1 = Readonly<{ suite: string; scratch?: "sandbox" | "checkout"; maximumElapsedMilliseconds?: number }>;

/** 🎫️ Declares the birth of a library test file: bare, the process is born with the native build caller grant of its suite target and a sandboxed artifact directory (so `bun test ./file` from the checkout gives the truth); beneath the official runner the inherited grant must be a genuine native build caller grant, else the birth is refused. */
export function declareLibraryBuildTestEnvelopeV1(row: LibraryBuildTestEnvelopeV1): void {
  declareBuildTestEnvelopeV1({ owner: "test:" + row.suite, maximumElapsedMilliseconds: row.maximumElapsedMilliseconds ?? 300000, capabilities: { grant: { kind: "native-build-caller", arguments: ["run", "@semio-tech/repo-lib:test-" + row.suite], scratch: row.scratch ?? "sandbox" } } });
}
