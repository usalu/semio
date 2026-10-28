"""🧪️ Test edits of lane langbudget, applied by make.py to the current live files: the argv budget law, the catalog-image tests and the loader fixtures that carry the catalog row."""
import sys

LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
IDENT = LIB + "🏗️build/📥️source/🔏️closure/🚪️command/🧪️tests/🏷️loader-identity/🟦️.ts"
CSITES = LIB + "🏗️build/📥️source/🔏️closure/🧪️tests/🧭️sites/🟦️.ts"
SITES_TESTS = LIB + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧪️tests/🟦️.ts"
LAW = LIB + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧭️closure-law/🟦️.ts"
SCHEDULING_TESTS = LIB + "🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts"
STREAM_TESTS = LIB + "🏗️build/📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts"
SITES_IMPORT = "../../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts"


def edit(text, old, new, count=1):
    if text.count(old) != count:
        sys.exit("anchor count %d != %d: %r" % (text.count(old), count, old[:90]))
    return text.replace(old, new)


def ident_edit(text):
    text = edit(text, 'import {buildSourceClosureStreamLoaderV1} from "../../../🟦️.ts";',
                'import {buildSourceClosureLoaderV1,buildSourceClosureStreamLoaderV1} from "../../../🟦️.ts";\n'
                'import {spawnSync} from "node:child_process";\n'
                'import {sealedPolicyV1} from "../../../../../🏛️sealed/🟦️.ts";\n'
                'import {closureRefusalMessagesV1} from "' + SITES_IMPORT + '";')
    return text + r'''
const commandLineLengthV1=(argv:readonly string[])=>Number(spawnSync("python3",["-I","-c","import json,subprocess,sys;print(len(subprocess.list2cmdline(json.load(sys.stdin)))+1)"],{input:JSON.stringify(argv),encoding:"utf8"}).stdout);
const headroomCharactersV1=4500;
/** 🧮️ Contract: the `--eval` loader of a captured command is part of its command line, which Windows bounds at 32767 characters, so the loader stays below that bound minus the room an executable path and the arguments need (measured against Python's independent command-line quoting), and carries no refusal text: the catalog may grow without limit because the loader renders from the authenticated snapshot instead of inlining it. */
test("the captured command line stays within the operating system limit with room for arguments, whatever the refusal catalog holds",()=>{
 const limit=sealedPolicyV1.maximumCommandLineCharacters,executable="/"+"x".repeat(259),digest="0".repeat(64),worst=["run","project:target","--configuration=release","--parallelism=8","--","a\"".repeat(1500)],argv=[executable,"--eval",buildSourceClosureStreamLoaderV1,"--",digest,...worst];
 expect(commandLineLengthV1(argv),"command line of the longest realistic invocation").toBeLessThanOrEqual(limit);
 expect(commandLineLengthV1([executable,"--eval",buildSourceClosureStreamLoaderV1,"--",digest]),"command line without arguments").toBeLessThanOrEqual(limit-headroomCharactersV1+executable.length);
 expect(buildSourceClosureStreamLoaderV1.length,"stream loader characters").toBeLessThanOrEqual(limit-headroomCharactersV1);
 for(const[name,loader]of[["stream",buildSourceClosureStreamLoaderV1],["file",buildSourceClosureLoaderV1]] as const)for(const[code,row]of Object.entries(closureRefusalMessagesV1))for(const language of Object.values(row))expect(loader.includes(language.refusal.split("{")[0]!),name+" loader inlines the "+code+" refusal text").toBe(false);
});
'''


def csites_edit(text):
    text = edit(text, 'import{closureRefusalMessagesV1}from"../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";',
                'import{closureRefusalCatalogPathV1,closureRefusalMessagesV1}from"../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";\n'
                'import sitesCatalog from"../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧬️schema/🔣️.json";')
    text = edit(text, 'mkdtempSync(join(realpathSync(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"cs-"))', 'mkdtempSync(join(realpathSync(tmpdir()),"cs-"))')
    text = edit(text, "const source=await captureNxAuthorizerSourcesV1(area,fixture.entry,new AbortController().signal,{installed:true,roots:new Map(Object.entries(fixture.roots))}),bytes=nxAuthorizerCapturedBytesV1(source),imports=nxAuthorizerCapturedImportsV1(area,source);",
                "const source=await captureNxAuthorizerSourcesV1(area,fixture.entry,new AbortController().signal,{installed:true,roots:new Map(Object.entries(fixture.roots))}),bytes=new Map(nxAuthorizerCapturedBytesV1(source)),imports=new Map(nxAuthorizerCapturedImportsV1(area,source));bytes.set(closureRefusalCatalogPathV1,Buffer.from(JSON.stringify(sitesCatalog)));imports.set(closureRefusalCatalogPathV1,{});")
    text = edit(text, "snapshot={version:1 as const,root:area,entry:fixture.entry,files},text=JSON.stringify(snapshot),sha=buildSourceClosureIdentityV1(snapshot);",
                "snapshot={version:1 as const,root:area,entry:fixture.entry,files},text=JSON.stringify(snapshot),sha=buildSourceClosureIdentityV1(snapshot),bare={...snapshot,files:files.filter(file=>file.path!==closureRefusalCatalogPathV1)},bareText=JSON.stringify(bare),bareSha=buildSourceClosureIdentityV1(bare);")
    text = edit(text, "  put(fixture.drift.file,fixture.drift.text);\n",
                "  const bareResult=await runLoaderV1(area,bareText,bareSha,\"en\");expect(typeof bareResult,String(bareResult)).toBe(\"object\");\n"
                "  for(const row of fixture.cases)if(row.outcome===\"refused\"){const message=((bareResult as Record<string,Outcome>)[row.name] as{message:string}).message;expect(message,row.name).toStartWith(row.code+\": {\");expect(message,row.name).not.toContain(\"Fix:\");}\n"
                "  put(fixture.drift.file,fixture.drift.text);\n")
    return text


def csites_lang_edit(text):
    return text + r'''
/** 🪪️ A loader started without the digest of its snapshot refuses before anything is authenticated, by the machine token of the refusal and no catalog text. */
test("the real loader names a refusal raised before authentication by its code",async()=>{
 const child=spawn(process.execPath,["--eval",buildSourceClosureStreamLoaderV1,"--"],{stdio:["ignore","pipe","pipe","pipe"]});let err="";child.stderr!.on("data",data=>err+=data);(child.stdio[3] as NodeJS.WritableStream).end("");
 await new Promise(resolve=>child.on("close",resolve));expect(err).toContain("digest-required: {}");expect(err).not.toContain("Fix:");
},60000);
'''


def stests_edit(text):
    text = edit(text, 'import{closureLiteralSitesV1,closurePackageNameV1,closureRefusalTextV1,closureRefusalMessagesV1,closureSpecifierKindV1}from"../🟦️.ts";',
                'import{closureLiteralSitesV1,closurePackageNameV1,closureRefusalCatalogPathV1,closureRefusalCatalogTableV1,closureRefusalTextV1,closureRefusalMessagesV1,closureSpecifierKindV1}from"../🟦️.ts";')
    text = edit(text, 'import{dirname,join}from"node:path";', 'import{createHash}from"node:crypto";\nimport{readFileSync}from"node:fs";\nimport{dirname,join,sep}from"node:path";')
    return text + r'''
/** 📍️ The catalog path names this very schema file, and the table reader gives its messages only for bytes whose declared hash holds. */
test("the catalog path names the schema file and the table reader refuses altered bytes",()=>{
 const file=join(here,"..","🧬️schema","🔣️.json"),bytes=readFileSync(file),hash=(value:Uint8Array)=>createHash("sha256").update(value).digest("hex"),encoded=bytes.toString("base64"),sha=hash(bytes),bare=Buffer.from("{}"),open=Buffer.from("[");
 expect(file.split(sep).join("/").endsWith("/"+closureRefusalCatalogPathV1),closureRefusalCatalogPathV1).toBe(true);
 expect(closureRefusalCatalogTableV1(encoded,sha,hash)).toEqual(closureRefusalMessagesV1);
 for(const[name,table]of[["wrong hash",closureRefusalCatalogTableV1(encoded,hash(bare),hash)],["absent bytes",closureRefusalCatalogTableV1(undefined,sha,hash)],["absent hash",closureRefusalCatalogTableV1(encoded,undefined,hash)],["not base64",closureRefusalCatalogTableV1("not base64!",sha,hash)],["no messages",closureRefusalCatalogTableV1(bare.toString("base64"),hash(bare),hash)],["not json",closureRefusalCatalogTableV1(open.toString("base64"),hash(open),hash)]] as const)expect(table,name).toBeUndefined();
});
'''


def law_edit(text):
    text = edit(text, 'import{closureRefusalV1,closureSpecifierKindV1,closureTargetSchemesV1}from"../../🧭️sites/🟦️.ts";',
                'import{closureRefusalCatalogPathV1,closureRefusalV1,closureSpecifierKindV1,closureTargetSchemesV1}from"../../🧭️sites/🟦️.ts";')
    return text + r'''
/** 📍️ A loader renders every refusal after authentication from the catalog image of its snapshot, so every command closure must carry the catalog file. */
test("every command closure carries the refusal catalog",async()=>{
 for(const entry of kernels){const source=await captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal,{installed:true});expect([...nxAuthorizerCapturedBytesV1(source).keys()],entry).toContain(closureRefusalCatalogPathV1);}
},600000);
'''


def scheduling_lang_edit(text):
    text = edit(text, 'import corpusSchema from "./🧬️schema/🔣️.json";',
                'import corpusSchema from "./🧬️schema/🔣️.json";\n'
                'import {closureRefusalCatalogPathV1} from "../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";\n'
                'import sitesCatalog from "../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧬️schema/🔣️.json";')
    return edit(text, 'file("keyed.ts",\'export const k="K1";\')];', 'file("keyed.ts",\'export const k="K1";\'),file(closureRefusalCatalogPathV1,JSON.stringify(sitesCatalog))];')


def stream_lang_edit(text):
    text = edit(text, 'expect(wrong.err).toContain("Immutable implementation snapshot changed");', 'expect(wrong.err).toContain("snapshot-changed: {}");')
    return edit(text, 'expect(refused.err).toContain("Bounded immutable implementation snapshot required");', 'expect(refused.err).toContain("snapshot-bounded: {}");')


LANG_EDITS = ((CSITES, csites_lang_edit), (SCHEDULING_TESTS, scheduling_lang_edit), (STREAM_TESTS, stream_lang_edit))
EDITS = ((IDENT, ident_edit), (CSITES, csites_edit), (SITES_TESTS, stests_edit), (LAW, law_edit))
