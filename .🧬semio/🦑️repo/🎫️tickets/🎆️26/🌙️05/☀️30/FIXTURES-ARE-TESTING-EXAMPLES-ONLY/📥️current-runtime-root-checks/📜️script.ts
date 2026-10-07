import { expect, test } from "bun:test";
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
let root = import.meta.dir;
while (!existsSync(join(root, "bun.lock"))) root = dirname(root);
const library = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const output = join(dirname(import.meta.dir), "🗑️generated/current-runtime-root-checks");
mkdirSync(output, { recursive: true });
for (const [name, path, filter] of [["selected-cargo", "🗂️workspaces/🦀️cargo/🧪️tests/🟦️.ts", "preparation follows actual|selected package preparation"], ["native-progress", "⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts", "native progress follows|dashboard-selected"], ["library-search-path", "⚡️caching/🧪️tests/📚️library-search-path/🟦️.ts", ""], ["readme", "🧪️tests/🔖️readme-current-source-revision/🟦️.ts", ""], ["nextest", "🧪️tests/🔬️workspace-contract/🟦️.ts", "retains explicit task artifacts"]]) test(`actual library inline consumer ${name}`, async () => {
  const child = Bun.spawn([process.execPath, "test", join(library, path!), ...(filter ? ["-t", filter] : [])], {
    cwd: root,
    env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: output, SEMIO_REPO_TEST_ARTIFACT_DIR: output },
    stdout: "pipe", stderr: "pipe",
  });
  const timer = setTimeout(() => child.kill("SIGTERM"), 290000);
  const stop = () => child.kill("SIGTERM");
  process.once("SIGINT", stop); process.once("SIGTERM", stop);
  try {
    const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    writeFileSync(join(output, name + ".log"), stdout + stderr);
    console.log(`[DEBUG] ${name} actual suite exit=${code}: ${stderr.replace(/\u001b\[[0-9;]*m/gu, "").split("\n").filter((line) => /pass|fail|filtered|Ran \d|error:|Error:/u.test(line)).join(" | ")}`);
    expect(code, name).toBe(0);
  } finally {
    clearTimeout(timer);
    process.off("SIGINT", stop); process.off("SIGTERM", stop);
  }
}, 300000);

const os=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules");
for(const[name,path,filter]of[["store-replay","🏪️store/🧪️tests/🧪️supersede-replay/🟦️.ts",""],["store-clamp","🏪️store/📨️messages/✂️clamp/🧪️tests/🟦️.ts",""],["history-sqlite","🏪️store/📜️space-history/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts","History partial cleanup"]])test(name!,async()=>{const child=Bun.spawn([process.execPath,"test",join(os,path!),...(filter?["-t",filter]:[])],{cwd:root,env:{...process.env,SEMIO_TEST_ARTIFACT_DIR:output,SEMIO_REPO_TEST_ARTIFACT_DIR:output},stdout:"pipe",stderr:"pipe"});const[out,err,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);writeFileSync(join(output,name+".log"),out+err);console.log(`[DEBUG] ${name} exit=${code} ${err.split("\n").filter(line=>/pass|fail|filtered|error:/u.test(line)).join(" | ")}`);expect(code,name).toBe(0);},300000);
test("utf8",async()=>{const child=Bun.spawn([process.execPath,"test",join(root,"🧰️framework/🔨️modules/🌱️value/📝️text/🧪️tests/📏️utf8/🟦️.ts")],{cwd:root,stdout:"pipe",stderr:"pipe"});const[out,err,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);writeFileSync(join(output,"utf8.log"),out+err);console.log(`[DEBUG] utf8 ${out+err}`);expect(code).toBe(0);});
for(const[name,fn]of[["🧩️composition","testRecursiveOwnedDocumentReplacementOracle"],["♻️publication-retirement-authority","testPublicationRetirementAuthorityOracle"]])test(name!,async()=>{const module=await import(join(os,"🔌️plugin/🧪️tests",name!,"🟦️.ts"));module[fn!]();console.log(`[DEBUG] ${name} semantic oracle completed`);});
test("native renderer cache outputs",async()=>{const module=await import(join(library,"⚡️caching/🧪️tests/🧊️native-renderer-outputs/🟦️.ts"));await module.testNativeRendererOutputs(root,output);console.log("[DEBUG] actual native renderer build/cache/restore/republication/held child completed");},600000);
for(const[owner,path]of[["📝️todos","📝️todo-markdown-roundtrip"],["🎯️goals","🎯️goal-document-codec"],["🧑️contributors","🪪️contributor-identity-parse"]])test(owner!,async()=>{const base=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules",owner!);const adapter=(await import(join(base,"🧪️tests",path!,"🟦️.ts"))).default;for(const[id,scenario]of Object.entries(adapter.scenarios)){const result=await(scenario as any).oracle({input:(uri:string)=>uri.startsWith("schema://")?join(base,"🧬️schema/🔣️.json"):join(base,"🧫️fixtures",uri.slice("shared://".length))});expect(result).toBeDefined();console.log(`[DEBUG] ${owner}/${id} result=${JSON.stringify(result)}`);}});
