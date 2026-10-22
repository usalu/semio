import {join, resolve} from "node:path";
import {createIssuedResourceControlV1, receiveScriptProcessInvocation, Script, ScriptRouter} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withResourceLeases} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {requireNativeStorageReservationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {NATIVE_TOOLCHAINS_V1, pythonEnvironmentOfProjectV1, pythonProjectForRootV1, toolchainIdentityV1, toolchainHostV1, toolchainMessageV1, type ToolchainKindV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {getWorkspaceRoot} from "../../../../📦️packages/🟦️typescript/📇️inventory/🗂️workspaces/🟦️.ts";
import {withRepositoryToolingStoreV1} from "../../../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import {pinnedNativeEnvironmentV1} from "./🟦️.ts";
import {systemNativeToolPortV1} from "./🔌️port/🟦️.ts";

const kinds: readonly ToolchainKindV1[] = ["python", "dotnet", "cmake", "go"];

/** 📥️ Provisions the checksum-pinned CMake and Ninja below the tools partition once; later calls find them. */
export class PrepareScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error(toolchainMessageV1("usage", {command: "prepare", choices: "no arguments"}));
    const workspace = getWorkspaceRoot();
    await withRepositoryToolingStoreV1(workspace, this.invocation, createIssuedResourceControlV1(this.invocation, "cache:entry", 0), async store => {
      requireNativeStorageReservationV1(store.storage, store.control);
      await store.recheck();
      if (store.root !== repositoryCachePartitionRootV1(workspace, "tools")) throw new Error(toolchainMessageV1("directory", {path: store.root}));
      const signal = store.control.signal;
      await withResourceLeases({directory: join(workspace, ".cache/b/l/m/data"), signal, control: store.control, resources: [{resource: "native-tools:v1", mode: "exclusive"}]}, async () => {
        await pinnedNativeEnvironmentV1({}, store.root, systemNativeToolPortV1(workspace, readNativeCallerAuthorityV1(this.invocation.capabilities).network.offline), signal, true);
      });
    });
  }
}

/** 🔏️ Prints the identity digest of one toolchain class without installing or spawning anything: `fingerprint <python|dotnet|cmake|go> [project root]`, or every class as one JSON object. */
export class FingerprintScript extends Script {
  async run(args: string[]): Promise<void> {
    const [kind, root, extra] = args;
    if (extra !== undefined || (kind !== undefined && !kinds.includes(kind as ToolchainKindV1)) || (root !== undefined && kind !== "python")) throw new Error(toolchainMessageV1("usage", {command: "fingerprint", choices: kinds.join(", ") + ", for python also a project root"}));
    const project = root === undefined ? undefined : pythonProjectForRootV1(root)?.id, host = toolchainHostV1();
    if (kind !== undefined) {
      console.log(toolchainIdentityV1(kind as ToolchainKindV1, {host, ...(project === undefined ? {} : {project})}).digest);
      return;
    }
    console.log(JSON.stringify(Object.fromEntries(kinds.map(name => [name, toolchainIdentityV1(name, {host}).digest]))));
  }
}

/** 🐍️ Prints the plan of one Python project as JSON: its environment directory, the variables and the uv arguments, so a shell, a test runner or an agent runs `uv` exactly where the repository wants it. */
export class EnvironmentScript extends Script {
  async run(args: string[]): Promise<void> {
    const [kind, project, extra] = args;
    if (kind !== "python" || project === undefined || extra !== undefined || !NATIVE_TOOLCHAINS_V1.python.projects.some(row => row.id === project)) throw new Error(toolchainMessageV1("usage", {command: "environment", choices: "python and one of " + NATIVE_TOOLCHAINS_V1.python.projects.map(row => row.id).join(", ")}));
    const workspace = resolve(getWorkspaceRoot()), roots = {dependencies: repositoryCachePartitionRootV1(workspace, "dependencies"), tools: repositoryCachePartitionRootV1(workspace, "tools")};
    console.log(JSON.stringify(pythonEnvironmentOfProjectV1(workspace, project, roots)));
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => new ScriptRouter(import.meta.dir).register("prepare", PrepareScript).register("fingerprint", FingerprintScript).register("environment", EnvironmentScript).run(process.argv.slice(2), original));
