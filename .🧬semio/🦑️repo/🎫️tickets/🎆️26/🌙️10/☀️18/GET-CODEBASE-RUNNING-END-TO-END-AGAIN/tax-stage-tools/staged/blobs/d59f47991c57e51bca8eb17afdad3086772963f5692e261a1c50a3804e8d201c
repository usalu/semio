import { withJavascriptDependencyStorageV1, prepareJavascriptDependencies } from "../../💾️storage/🟦️.ts";
import { join } from "node:path";
import { Script, checkScriptInvocation, createIssuedResourceControlV1, findWorkspaceRoot } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withRepositoryTestArtifactsV1, repositoryTestArtifactEnvironmentV1 } from "../../../../../../🧪️test/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import { runRepositoryCommand } from "../../../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { getWorkspaceRoot } from "../../../../../🗂️workspaces/🟦️.ts";
import { dirname as scriptModuleDirname, resolve as scriptModuleResolve } from "node:path";

/** 📍️ Location of the former `📜️script.ts` of this module, kept for code that resolves siblings of its entry. */
const scriptModulePath = scriptModuleResolve(import.meta.dir, "../../📜️script.ts");
const scriptModuleDirectory = scriptModuleDirname(scriptModulePath);



/** 📦️ Synchronizes the locked JavaScript environment without loading application modules. */
export class SyncScript extends Script {
  protected readonly mode: "sync" | "lock" = "sync";
  async run(args: string[]): Promise<void> {
    if (args.length && !(args.length===1 && args[0]==="--all") && !(args.length===2 && args[0]==="--scope")) throw new Error("Dependency synchronization accepts --all or --scope <owner>");
    checkScriptInvocation(this.invocation);
    const selection=args[0]==="--all"||!args.length?{kind:"all" as const}:{kind:"scope" as const,owner:args[0]==="--scope"?args[1]!:"."};
    const phaseControl=createIssuedResourceControlV1(this.invocation,"bun:dependency-storage",0);
    await withJavascriptDependencyStorageV1(findWorkspaceRoot(getWorkspaceRoot()),this.invocation,phaseControl,(store,control)=>prepareJavascriptDependencies(this.mode,findWorkspaceRoot(getWorkspaceRoot()),store,control,selection));
  }
}

/** 🔒️ Refreshes only the declared workspace lock through the selected Bun runtime. */
export class RefreshLockScript extends SyncScript {
  protected readonly mode = "lock" as const;
}

/** ⚖️ Executes portable dependency command and independent Node oracle laws. */
export class ContractCheckScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Dependency contract check accepts no arguments");
    checkScriptInvocation(this.invocation);
    await withRepositoryTestArtifactsV1(findWorkspaceRoot(getWorkspaceRoot()),this.invocation,createIssuedResourceControlV1(this.invocation,"dependencies:tests",0),()=>runRepositoryCommand(this.invocation,process.execPath,["test",join(scriptModuleDirectory,"🧪️tests/🟦️.ts")],findWorkspaceRoot(getWorkspaceRoot()),"dependencies:contract-check",{version:1,maximumElapsedMilliseconds:120000,output:{maximumBytes:134217728,maximumLines:65536}},{env:repositoryTestArtifactEnvironmentV1(findWorkspaceRoot(getWorkspaceRoot()))}));
    checkScriptInvocation(this.invocation);
  }
}
