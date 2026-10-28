"""📣️ memonotice: a graph memo that cannot serve says why (reason and the first relevant change) in the command's language, like Bazel's "planning again"; before, a miss was silent and only a profile mark named it.
Usage: python3 -I edit.py <root> apply|check|revert   (backups in base/<n>, taken on the first apply)."""
import os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
M = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/"
EDITS = [
    (M + "🚪️command/🟦️.ts", [
        ('/** 🗣️ The one line a served graph prints, in the language of the command. */\nexport async function renderGraphMemoServedV1(answer:Extract<MemoAnswerV1,{hit:true}>,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<string>{\n const {resolveLanguageV1}=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts"),{CATALOG_LANGUAGES_V1}=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts");\n return renderGraphMemoTextV1("served",{projects:answer.projects,seconds:Math.round(answer.ageMilliseconds/1000),rows:answer.rows,rechecked:answer.rechecked},resolveLanguageV1(CATALOG_LANGUAGES_V1,[],environment as NodeJS.ProcessEnv).language);\n}\n',
         '/** 🌐️ The language of the command the memo speaks in. */\nasync function memoLanguageV1(environment:Readonly<Record<string,string|undefined>>):Promise<string>{\n const {resolveLanguageV1}=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts"),{CATALOG_LANGUAGES_V1}=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts");\n return resolveLanguageV1(CATALOG_LANGUAGES_V1,[],environment as NodeJS.ProcessEnv).language;\n}\n\n/** 🗣️ The one line a served graph prints, in the language of the command. */\nexport async function renderGraphMemoServedV1(answer:Extract<MemoAnswerV1,{hit:true}>,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<string>{\n return renderGraphMemoTextV1("served",{projects:answer.projects,seconds:Math.round(answer.ageMilliseconds/1000),rows:answer.rows,rechecked:answer.rechecked},await memoLanguageV1(environment));\n}\n\n/** 📣️ The one line a memo that cannot serve prints: the reason and, where one exists, the first relevant change. */\nexport async function renderGraphMemoRefusedV1(answer:Extract<MemoAnswerV1,{hit:false}>,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<string>{\n return renderGraphMemoTextV1("refused",{reason:answer.detail===undefined?answer.reason:answer.reason+": "+answer.detail},await memoLanguageV1(environment));\n}\n'),
        (' input.mark("graph.memo",served===undefined?"off":served.hit?"hit":"miss:"+served.reason);\n',
         ' input.mark("graph.memo",served===undefined?"off":served.hit?"hit":"miss:"+served.reason);\n if(served!==undefined&&!served.hit&&served.reason!=="none")input.notice(await renderGraphMemoRefusedV1(served,input.environment));\n'),
    ]),
    (M + "🧪️tests/🟦️.ts", [
        (' expect(renderGraphMemoTextV1("served",{projects:3,seconds:4,rows:5,rechecked:6},"xx")).toContain("3 projects");\n',
         ' expect(renderGraphMemoTextV1("served",{projects:3,seconds:4,rows:5,rechecked:6},"xx")).toContain("3 projects");\n'),
    ]),
]
LAW = '''
test("a memo that cannot serve says why: the reason and the first relevant change, in the command's language; a request without a record stays quiet",async()=>{
 const {planGraphWithMemoV1}=await import("../🚪️command/🟦️.ts");
 const run=async(answer:unknown,language:string)=>{const notices:string[]=[];const memo={lookup:async()=>answer,cursor:undefined,expected:()=>undefined,seal:async()=>"light"} as unknown as Parameters<typeof planGraphWithMemoV1>[0]["memo"];
  await planGraphWithMemoV1({memo,stem:"/x/memo",parse:text=>JSON.parse(text) as {projects:Record<string,unknown>},record:()=>{},notice:text=>{notices.push(text);},mark:()=>{},environment:{SEMIO_LANGUAGE:language},plan:async()=>({plan:JSON.stringify({projects:{a:{}}}),pin:undefined})});return notices;};
 const changed=await run({hit:false,reason:"changed",detail:"src/a.ts"},"en");
 expect(changed).toHaveLength(1);expect(changed[0]).toContain("changed: src/a.ts");expect(changed[0]).toContain("graph is planned");
 expect((await run({hit:false,reason:"journal"},"de"))[0]).toContain("(journal)");
 expect((await run({hit:false,reason:"journal"},"de"))[0]).toContain("Graph wird geplant");
 expect(await run({hit:false,reason:"none"},"en")).toEqual([]);
});
'''
root, mode = sys.argv[1], sys.argv[2]
status = 0
for n, (rel, pairs) in enumerate(EDITS):
    path, backup = os.path.join(root, rel), os.path.join(HERE, "base", str(n))
    text = open(path, encoding="utf-8").read()
    if mode == "revert":
        shutil.copyfile(backup, path); print("reverted", rel[-50:]); continue
    new = text
    for old, replacement in pairs:
        if replacement in new and old != replacement: continue
        if new.count(old) != 1: print("ANCHOR", rel[-50:], new.count(old), old[:60]); status = 2; continue
        new = new.replace(old, replacement)
    if rel.endswith("🧪️tests/🟦️.ts") and LAW not in new: new = new + LAW
    if mode == "check": print("applied" if new == text else "pending", rel[-50:]); continue
    if new != text:
        if not os.path.exists(backup): shutil.copyfile(path, backup)
        tmp = path + ".memonotice"; open(tmp, "w", encoding="utf-8").write(new); os.replace(tmp, path); print("applied", rel[-50:])
sys.exit(status)
