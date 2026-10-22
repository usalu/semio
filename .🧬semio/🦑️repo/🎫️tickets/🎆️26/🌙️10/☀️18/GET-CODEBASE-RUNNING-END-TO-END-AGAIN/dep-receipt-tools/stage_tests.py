import json, os, re, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from stage import GEN, STAGE, once

TESTS = GEN + "/🧪️tests"
TEST_FILES = {
    "tests.ts": TESTS + "/🟦️.ts",
    "fixtures.json": TESTS + "/🔣️.json",
    "ingress.ts": TESTS + "/🚪️ingress/🟦️.ts",
    "followup.ts": TESTS + "/🛡️followup/🟦️.ts",
    "incremental-tests.ts": GEN + "/../🧾️incremental/🧪️tests/🟦️.ts",
}


def read(name):
    return open(TEST_FILES[name], encoding="utf-8").read()


def unwrap(text, title):
    start = text.index('test("' + title)
    header = '()=>withOriginal(async original=>{'
    at = text.index(header, start)
    if at - start > 400:
        raise SystemExit("header too far for " + title)
    text = text[:at] + "async()=>{" + text[at + len(header):]
    for closer, replacement in (("\n}));", "\n});"), ("\n}),60000);", "\n},60000);")):
        end = text.find(closer, at)
        next_test = text.find('\ntest("', at + 1)
        if end != -1 and (next_test == -1 or end < next_test):
            return text[:end] + replacement + text[end + len(closer):]
    raise SystemExit("closer not found for " + title)


def fixtures():
    data = json.loads(read("fixtures.json"))
    for key in ("implementation", "fences", "legacyImplementationFiles"):
        data.pop(key)
    rename = {
        "closure file rewritten with identical bytes matches": "tool source file rewritten with identical bytes matches",
        "closure file touched without change matches": "tool source file touched without change matches",
        "file outside the closure edited matches": "file next to the tool source edited matches",
        "file added next to the closure matches": "file added next to the tool source matches",
    }
    cases = []
    for case in data["cases"]:
        name = case["name"]
        if name in ("entry list changed falls back", "closure evidence tampered falls back"):
            continue
        if name.startswith("closure ") and case["expect"] == "mismatch:implementation-changed":
            case["name"] = "tool source " + name[len("closure "):].replace(" falls back", " matches")
            case["expect"] = "match"
        elif name in rename:
            case["name"] = rename[name]
        cases.append(case)
    data["cases"] = cases
    reasons = {
        "lock edited falls back": {"path": "w:bun.lock", "detail": "bytes"},
        "lock replaced by equal bytes falls back": {"path": "w:bun.lock", "detail": "identity"},
        "bunfig appears falls back": {"path": "w:bunfig.toml", "detail": "appeared"},
        "member manifest edited falls back": {"path": "pkgs/b/package.json", "detail": "bytes"},
        "workspace npmrc appears falls back": {"path": "w:.npmrc"},
        "runtime changed falls back": {"detail": "bun"},
        "bun revision changed falls back": {"detail": "revision"},
    }
    oracle = (set(reasons) - {"runtime changed falls back", "bun revision changed falls back"}) | {
        "untouched tree matches",
        "source file edited matches",
        "irrelevant file added matches",
        "home npmrc appears falls back",
    }
    for case in data["cases"]:
        if case["name"].startswith("tool source ") or case["name"].startswith("file next to the tool source") or case["name"].startswith("file added next to the tool source"):
            oracle.add(case["name"])
    for case in data["cases"]:
        if case["name"] in reasons:
            case["reason"] = reasons[case["name"]]
        if case["name"] in oracle:
            case["oracle"] = True
    data["cases"].insert(
        [c["name"] for c in data["cases"]].index("lock edited falls back"),
        {"name": "walk protocol stamp changed falls back", "mutations": [], "reseal": "walkStamp", "expect": "mismatch:protocol-changed", "reason": {"detail": "semio.dependency-generation-walk/0"}, "oracle": True},
    )
    data["cases"].insert(
        [c["name"] for c in data["cases"]].index("lock edited falls back"),
        {"name": "tool source entry and a transitive import edited together matches", "mutations": [{"op": "write", "path": "impl/entry.ts", "text": "export const changed=1;\n"}, {"op": "write", "path": "impl/deep/later.ts", "text": "export const later=2;\n"}], "expect": "match", "oracle": True},
    )
    data["reasons"] = [
        {"class": "input-changed", "path": "w:bun.lock", "detail": "bytes", "en": "An installer input changed its content: w:bun.lock.", "de": "Eine Installer-Eingabe hat ihren Inhalt geändert: w:bun.lock."},
        {"class": "input-changed", "path": "w:bunfig.toml", "detail": "appeared", "en": "An installer input appeared that the receipt recorded as missing: w:bunfig.toml.", "de": "Eine Installer-Eingabe ist aufgetaucht, die der Beleg als fehlend festgehalten hatte: w:bunfig.toml."},
        {"class": "manifest-changed", "path": "pkgs/b/package.json", "detail": "bytes", "en": "A workspace manifest changed its content: pkgs/b/package.json.", "de": "Ein Workspace-Manifest hat seinen Inhalt geändert: pkgs/b/package.json."},
        {"class": "runtime-changed", "detail": "bun", "en": "The Bun version changed.", "de": "Die Bun-Version hat sich geändert."},
        {"class": "protocol-changed", "detail": "semio.dependency-generation-walk/0", "en": "The dependency generation protocol changed: the receipt was sealed with semio.dependency-generation-walk/0, this tool declares semio.dependency-generation-walk/1.", "de": "Das Protokoll der Abhängigkeitsgeneration hat sich geändert: Der Beleg wurde mit semio.dependency-generation-walk/0 versiegelt, dieses Werkzeug deklariert semio.dependency-generation-walk/1."},
        {"class": "configuration-changed", "path": "w:.npmrc", "en": "The installer configuration changed: w:.npmrc.", "de": "Die Installer-Konfiguration hat sich geändert: w:.npmrc."},
        {"class": "membership-churn", "path": "pkgs/a", "en": "More than 4096 directories changed since the receipt, the first at pkgs/a.", "de": "Seit dem Beleg haben sich mehr als 4096 Verzeichnisse geändert, das erste bei pkgs/a."},
        {"class": "sha-changed", "en": "The combined read of the scope no longer equals the receipt.", "de": "Das kombinierte Lesen des Bereichs entspricht nicht mehr dem Beleg."},
    ]
    return json.dumps(data, ensure_ascii=False, indent=1) + "\n"


def tests():
    t = read("tests.ts")
    t = once(t, 'import {build as esbuild} from "esbuild";\n', "", "t-esbuild")
    t = once(t, 'import {dirname,join,relative,resolve} from "node:path";', 'import {dirname,join,relative} from "node:path";', "t-path")
    t = once(t, "import {captureGenerationReceiptV1,generationAnchorV1,", "import {captureGenerationReceiptV1,describeGenerationMismatchV1,generationAnchorV1,", "t-describe")
    t = once(t, "type Case={name:string;mutations:Mutation[];expect:string;tamper?:string;port?:string;relist?:number;reseal?:string};", "type Case={name:string;mutations:Mutation[];expect:string;tamper?:string;port?:string;relist?:number;reseal?:string;reason?:{path?:string;detail?:string};oracle?:boolean};", "t-case")
    t = once(t, "const subjectOf=(root:string,loadedAtMs:number,env:Record<string,string|undefined>={}):Subject=>Object.freeze({root,entries:fixtures.implementation.entries,loadedAtMs,ambient:", "const subjectOf=(root:string,env:Record<string,string|undefined>={}):Subject=>Object.freeze({ambient:", "t-subject")
    t = once(t, "async function capture(original:ScriptInvocation,root:string,store:string,startedAtMs:number,subject:Subject=subjectOf(root,startedAtMs),implementation?:Awaited<ReturnType<typeof glue.captureJavascriptDependencyGenerationImplementationV1>>){", "async function capture(root:string,store:string,startedAtMs:number,subject:Subject=subjectOf(root)){", "t-capture")
    t = once(t, "workspace:root,storeRoot:store,scope:\".\",startedAtMs,implementation:implementation??await glue.captureJavascriptDependencyGenerationImplementationV1(original,subject),runtime,", "workspace:root,storeRoot:store,scope:\".\",startedAtMs,runtime,", "t-capture-input")
    t = once(t, "abort?:AbortController,port?:string,subject:Subject=subjectOf(root,0),implementationRoot:string=root):GenerationVerifyInputV1{", "abort?:AbortController,port?:string,subject:Subject=subjectOf(root)):GenerationVerifyInputV1{", "t-verify-sig")
    t = once(t, "  implementationEntries:port===\"entries\"?[...fixtures.implementation.entries,\"impl/other.ts\"]:subject.entries,implementationFile:glue.javascriptDependencyGenerationImplementationFileV1(implementationRoot),producer:", "  producer:", "t-verify-input")
    t = t.replace("capture(original,", "capture(")
    t = once(t, "subject=subjectOf(root,Date.now()+10000,env);", "subject=subjectOf(root,env);", "t-case-subject")
    t = once(t, "if(row.tamper===\"frontier\"||row.tamper===\"execution\"||row.tamper===\"closure\"){const name=row.tamper===\"frontier\"?receipt.membership.frontier.name:row.tamper===\"execution\"?receipt.execution.evidence.name:receipt.implementation.evidence.name;", "if(row.tamper===\"frontier\"||row.tamper===\"execution\"){const name=row.tamper===\"frontier\"?receipt.membership.frontier.name:receipt.execution.evidence.name;", "t-tamper")
    t = once(t, "   if(row.reseal===\"scopeReadSha256\"){const{sha256:_,...body}=receipt;receipt=sealGenerationReceiptV1({...body,scopeReadSha256:\"1\".repeat(64)});}\n", "   if(row.reseal===\"scopeReadSha256\"){const{sha256:_,...body}=receipt;receipt=sealGenerationReceiptV1({...body,scopeReadSha256:\"1\".repeat(64)});}\n   if(row.reseal===\"walkStamp\"){const{sha256:_,...body}=receipt;receipt=sealGenerationReceiptV1({...body,walkStamp:protocol.walkStamp.replace(/\\d+$/u,\"0\")});}\n", "t-reseal")
    t = once(t, "    expect(outcome(verdict),row.name).toBe(row.expect);\n", "    expect(outcome(verdict),row.name).toBe(row.expect);\n    if(row.reason)expect(verdict,row.name).toMatchObject(row.reason);\n    if(row.oracle){const python=oracleVerify(root,store,receipt);expect(python.status==='match'?'match':'mismatch:'+python.class,row.name).toBe(row.expect);if(row.reason)expect(python,row.name).toMatchObject(row.reason);}\n", "t-reason")
    t = once(t, "const outcome=(result:{status:string;class?:string})=>result.status===\"match\"?\"match\":\"mismatch:\"+result.class;\n", """const outcome=(result:{status:string;class?:string})=>result.status==="match"?"match":"mismatch:"+result.class;
const keyingOracle=fileURLToPath(new URL("./🐍️keying.py",import.meta.url)),schemaPath=fileURLToPath(new URL("../🧬️schema/🔣️.json",import.meta.url)),python=()=>testPythonV1()??"python3";
const oracleVerify=(root:string,store:string,receipt:GenerationReceiptV1)=>{
 const file=root+".receipt.json";writeFileSync(file,JSON.stringify(receipt));
 const run=spawnSync(python(),["-I",keyingOracle,"verify",file,root,store,JSON.stringify(runtime),schemaPath],{encoding:"utf8"});rmSync(file,{force:true});
 if(run.status!==0)throw Error("keying oracle failed: "+run.stderr);return JSON.parse(run.stdout)as{status:string;class?:string;path?:string;detail?:string};
};
""", "t-oracle")
    t = once(t, 'import {tmpdir} from "node:os";', 'import {tmpdir} from "node:os";\nimport {fileURLToPath} from "node:url";', "t-url")
    for title in (
        "generation receipt contract:",
        "generation receipt: every fixture case verifies exactly",
        "generation receipt: a tampered or schema-invalid receipt",
        "generation receipt: capture defers rather than seal",
        "generation receipt: verification honors cancellation",
        "generation receipt: the frontier and the seal equal",
    ):
        t = unwrap(t, title)
    t = once(t, "  const settledSubject=subjectOf(root,Date.now()+10000),young=await capture(root,store,Date.now()-10000,settledSubject);expect(young.result.status).toBe(\"deferred\");\n  const unloaded=await capture(root,store,Date.now()+10000,subjectOf(root,Date.now()-10000));expect(unloaded.result).toMatchObject({status:\"deferred\",reason:\"implementation-unsettled\"});\n", "  const settledSubject=subjectOf(root),young=await capture(root,store,Date.now()-10000,settledSubject);expect(young.result.status).toBe(\"deferred\");\n", "t-defers")
    start = t.index("const closureOracle=String.raw`")
    end = t.index('test("generation receipt producer: the observation is sorted')
    keying = open(STAGE + "/keying.ts.txt", encoding="utf-8").read()
    t = t[:start] + keying + "\n" + t[end:]
    t = once(t, "  const subject=subjectOf(root,Date.now()+10000),generationControl=", "  const subject=subjectOf(root),generationControl=", "t-glue-subject") if "const subject=subjectOf(root,Date.now()+10000),generationControl=" in t else t
    t = once(t, "walk as never,walked,Date.now()+10000,original,subject);", "walk as never,walked,Date.now()+10000,subject);", "t-glue-capture1")
    t = once(t, "async custody=>custody.observed()),Date.now()+10000,original,subject);", "async custody=>custody.observed()),Date.now()+10000,subject);", "t-glue-capture2")
    return t


def peers():
    out = {}
    for name in ("ingress.ts", "followup.ts"):
        text = read(name)
        text = once(text, "const subject=Object.freeze({root,entries:peer.implementation.entries,loadedAtMs:Date.now()+10000,ambient:", "const subject=Object.freeze({ambient:", name + "-subject")
        out[name] = text
    return out


if __name__ == "__main__":
    os.makedirs(STAGE, exist_ok=True)
    open(STAGE + "/fixtures.json", "w", encoding="utf-8").write(fixtures())
    open(STAGE + "/tests.ts", "w", encoding="utf-8").write(tests())
    for name, text in peers().items():
        open(STAGE + "/" + name, "w", encoding="utf-8").write(text)
    incremental = once(read("incremental-tests.ts"), "generation.executionSuffix,generation.implementation.closureSuffix];", "generation.executionSuffix];", "incremental-suffixes")
    open(STAGE + "/incremental-tests.ts", "w", encoding="utf-8").write(incremental)
    print("staged tests")
