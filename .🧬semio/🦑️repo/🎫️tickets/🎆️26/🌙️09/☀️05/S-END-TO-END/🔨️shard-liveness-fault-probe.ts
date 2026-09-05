import * as tsm from "typescript";
const ts: any = (tsm as any).default ?? tsm;
import { readFileSync } from "node:fs";
import vm from "node:vm";

const headPath = "/private/tmp/claude-501/-Users-ueli-Documents-semio/2b78f0cf-ec36-4c47-9869-b75d7b31d161/scratchpad/lane-g/glue-head.ts";
const source = ts.createSourceFile(headPath, readFileSync(headPath, "utf8"), ts.ScriptTarget.Latest, true);
const decl = source.statements.find(n => ts.isFunctionDeclaration(n) && n.name?.text === "shardWorkerSource") as ts.FunctionDeclaration;
const returned = decl.body!.statements.find(ts.isReturnStatement)!.expression!;
const headWorker = (returned as ts.NoSubstitutionTemplateLiteral).text;
const { shardWorkerSource } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts");
const nowWorker = shardWorkerSource();

function probe(code: string, label: string) {
  const captured: any[] = [];
  let callback: any = null;
  const postFault = new Error("post-after-observation");
  const checkpoint = Object.freeze({ ordinary: "checkpoint" });
  const broken = vm.createContext({
    WebAssembly: { Suspending: class {}, promising: (v: unknown) => v },
    api: { checkpoint: async () => checkpoint },
    self: { addEventListener: (_k: string, h: any) => { callback = h; }, postMessage: (m: any) => { captured.push(m); if (m.kind === "result" && m.ok) throw postFault; } },
  });
  new vm.Script(code).runInContext(broken);
  new vm.Script('actors.set("a", { api, activationGeneration: 1n, pendingAssets: [] });').runInContext(broken);
  return callback({ data: { kind: "checkpoint", requestId: "r4", actorId: "a" } }).then(() => {
    console.log(label, JSON.stringify(captured.at(-1)), "| String(postFault) =", String(postFault));
  });
}
await probe(headWorker, "HEAD:");
await probe(nowWorker, "NOW: ");
