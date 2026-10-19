// 🏛️ Router-law lifts of the library command scripts that stay at their path: skeleton in `📜️script.ts`, implementation in the owner module.
import {readFileSync} from "node:fs";
import {accepts, syntaxOk} from "../lib.ts";
import {planLift, type LiftOptions} from "../lift.ts";
import {LIB, REPO, TOOLS, Vfs} from "../vfs.ts";
import {RELOCATED, repoint, repointKinds} from "../refs-lib.ts";
import {childList} from "./children.ts";
import {moduleScripts} from "./modules.ts";

const E = (cp: number): string => String.fromCodePoint(cp) + "\ufe0f";
const caching = `L/${E(0x26A1)}caching`, boot = `${caching}/${E(0x1F680)}bootstrap`;
export const LIFT_OPTIONS: Record<string, LiftOptions> = {
  [`${boot}/${E(0x1F4E6)}dependencies/${E(0x1F3D7)}native/${E(0x1F4DC)}script.ts`]: {routerRoot: "workspace"},
  [`${boot}/${E(0x1F4E6)}dependencies/${E(0x1F4DC)}script.ts`]: {routerRoot: "workspace"},
  [`${boot}/${E(0x1F6E0)}tools/${E(0x1F578)}wasm/${E(0x1F4DC)}script.ts`]: {routerRoot: "workspace"},
  [`${caching}/${E(0x1F6A6)}ci/${E(0x1F4DC)}script.ts`]: {routerRoot: "workspace"},
  [`L/${E(0x1F5C2)}workspaces/${E(0x1F980)}cargo/${E(0x1F4DC)}script.ts`]: {routerRoot: "workspace"},
  [`${caching}/${E(0x1F4E6)}artifacts/${E(0x1F4CB)}native-orchestration/${E(0x1F4E5)}invocation/${E(0x1F6E0)}preparation/${E(0x1F4DC)}script.ts`]: {
    preprocess: (src) => {
      const at = src.indexOf("if(import.meta.main) await receiveScriptProcessInvocation");
      if (at < 0) throw new Error("preparation terminal anchor missing");
      const tail = `/** ${E(0x1F6E1)} Runs \`prepare\` inside the bounded repository preparation scope that the entry opens before any command. */
export class BoundedPreparationScript extends Script {
  async run(args: string[]): Promise<void> {
    const root = getWorkspaceRoot();
    await withRepositoryCargoPreparationV1(findWorkspaceRoot(import.meta.dir), root, this.invocation, createIssuedResourceControlV1(this.invocation, "native:preparation-entry", 0), process.env, async (environment, phase) => {
      class BoundedPreparation extends PreparationScript { protected override readonly environment = environment; protected override phase() { return phase; } }
      await new BoundedPreparation(root, findWorkspaceRoot(root), this.invocation).run(args);
    });
  }
}

/** ${E(0x1F512)} Runs \`synchronize\` inside the bounded repository preparation scope, writing through the compiler workspace custody. */
export class BoundedSynchronizeScript extends Script {
  async run(args: string[]): Promise<void> {
    const root = getWorkspaceRoot();
    await withRepositoryCargoPreparationV1(findWorkspaceRoot(import.meta.dir), root, this.invocation, createIssuedResourceControlV1(this.invocation, "native:preparation-entry", 0), process.env, async (environment, phase, store) => {
      class BoundedSynchronize extends SynchronizeScript {
        protected override readonly environment = environment;
        protected override phase() { return phase; }
        protected override execute(command: string, args: string[], cwd: string, label: string, budget: number, options: OwnedCommandOptions): Promise<void> { return withCompilerWorkspaceWriteV1(store.storage, cargoPreparationStorageFromEnvironmentV1(environment).leaseDirectory, phase.leaseControl, () => super.execute(command, args, cwd, label, budget, options)); }
      }
      await new BoundedSynchronize(root, findWorkspaceRoot(root), this.invocation).run(args);
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("prepare", BoundedPreparationScript).register("synchronize", BoundedSynchronizeScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => router.run(process.argv.slice(2), original));
`;
      return src.slice(0, at) + tail;
    },
  },
  [`L/${E(0x1F4E6)}packages/${E(0x1F7E6)}typescript/${E(0x1F4DC)}script.ts`]: {
    preprocess: (src) => {
      const lazy = /\.registerLazy\("test-tool-run-policy-source",async\(\)=>\s*\(await import\("([^"]+)"\)\)\.ToolRunPolicySourceScript\)/u.exec(src);
      if (!lazy) throw new Error("registerLazy anchor missing");
      const anchor = /\/\*\* [^\n]*`@semio-tech\/repo-lib` router[^\n]*\n/u.exec(src);
      if (!anchor) throw new Error("router docstring anchor missing");
      const klass = `/** ${E(0x1F4A4)} Loads the tool-run policy source owner only for its explicit test command. */\nclass ToolRunPolicySourceTestScript extends BundleScript {\n  async run(args: string[]): Promise<void> {\n    const { ToolRunPolicySourceScript } = await import("${lazy[1]}");\n    await new ToolRunPolicySourceScript(this.root, this.repoRoot, this.invocation).run(args);\n  }\n}\n\n`;
      return src.replace(lazy[0], '.register("test-tool-run-policy-source",ToolRunPolicySourceTestScript)').replace(anchor[0], klass + anchor[0]);
    },
  },
};

export const inPlaceList = (): string[] => [`${LIB}/${E(0x1F9EA)}tests/${E(0x1F439)}executable-owners/${E(0x1F4DC)}script.ts`];

export const BLOCKED = (): string[] => [
  `${LIB}/${E(0x26A1)}caching/${E(0x1F4BE)}storage/${E(0x1F512)}enforcement/${E(0x1F3D7)}compiler/${E(0x1F6AA)}ingress/${E(0x1F4DC)}script.ts`,
  `${LIB}/${E(0x26A1)}caching/${E(0x1F680)}bootstrap/${E(0x1F4DC)}script.ts`,
  `${LIB}/${E(0x1F3D7)}build/${E(0x1F4DC)}script.ts`,
];

export const liftList = (): string[] => {
  const skip = new Set([...inPlaceList(), ...childList().map((d) => `${d}/${E(0x1F4DC)}script.ts`), ...moduleScripts(), ...BLOCKED()]);
  const rows = readFileSync(`${TOOLS}/library-43.txt`, "utf8").split("\n").filter(Boolean).filter((p) => !skip.has(p));
  const last = (p: string): number => p.includes(`${E(0x1F4E6)}packages/${E(0x1F7E6)}typescript/`) ? 1 : 0;
  return rows.sort((a, b) => last(a) - last(b));
};

export function planLifts(v: Vfs, log: (row: Record<string, unknown>) => void, only?: string): void {
  for (const p of liftList()) {
    if (only && !p.includes(only)) continue;
    const short = p.replace(LIB + "/", "L/");
    const plan = planLift(p, v.read(p), v, LIFT_OPTIONS[short] ?? {});
    if (!plan.ok) { log({kind: "lift", script: short, result: plan.reason}); continue; }
    const scriptOk = syntaxOk(p, plan.script) && accepts(plan.script), moduleOk = syntaxOk(plan.modulePath, plan.module);
    if (!scriptOk || !moduleOk) { log({kind: "lift", script: short, result: "invalid-output", scriptOk, moduleOk, module: plan.modulePath.replace(LIB + "/", "L/")}); continue; }
    v.write(plan.modulePath, plan.module);
    v.write(p, plan.script);
    RELOCATED.set(p, plan.modulePath);
    const touched = plan.exported ? repoint(v, p, plan.modulePath, true, new Set([p])) : [];
    const readers = (LIFT_OPTIONS[short] ?? {}).readers === "none" ? [] : repointKinds(v, p, plan.modulePath, new Set(["READ"]), (f) => f.includes("🧪️tests") || f.includes("🧼️"));
    log({kind: "lift", script: short, result: "lifted", module: plan.modulePath.replace(LIB + "/", "L/"), merged: plan.mergedInto, commands: plan.commands, repointed: touched.length, sourceReaders: readers.map((f) => f.replace(LIB + "/", "L/"))});
  }
}

export function planInPlace(v: Vfs, log: (row: Record<string, unknown>) => void): void {
  for (const p of inPlaceList()) {
    const before = v.read(p), call = "if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir, resolve(import.meta.dir, \"../../../../../../..\")).register(\"test\", TestScript), { invocation: original }));";
    if (!before.includes(call)) throw new Error("executable-owners terminal anchor missing");
    const after = before.replace(call, "const router = new ScriptRouter(import.meta.dir).register(\"test\", TestScript);\nif (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));");
    if (!syntaxOk(p, after) || !accepts(after)) { log({kind: "in-place", script: p.replace(LIB, "L"), result: "invalid-output"}); continue; }
    v.write(p, after);
    log({kind: "in-place", script: p.replace(LIB, "L"), result: "edited"});
  }
}
