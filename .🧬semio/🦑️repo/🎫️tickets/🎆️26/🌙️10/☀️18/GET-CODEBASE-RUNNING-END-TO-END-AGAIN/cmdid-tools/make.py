#!/usr/bin/env python3
"""🪪 [cmdid] Builds staged batch `identity` from the CURRENT bytes of <root> (default: live) with anchored edits (each anchor must match its declared number of times, else exit): the captured command check authenticates regardless of implementation pins, the closure schema and the command fixture gain the pinned/unpinned outcome per case, the command law exercises both pin states explicitly, and the two laws that evaluate the actual `executeCapturedBuildSourceV1` text get the port it reads today (`buildSourceClosureTextV1`) plus a guard that names any missing port.
usage: python3 -I make.py [--root DIR] [--batch NAME]"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
COMMAND = L + "📥️source/🔏️closure/🚪️command/🟦️.ts"
FIXTURE = L + "📥️source/🔏️closure/🚪️command/🔣️.json"
SCHEMA = L + "📥️source/🔏️closure/🧬️schema/🔣️.json"
LAW = L + "⌨️command/🧪️tests/🟦️.ts"

OLD_DOC = '/** 🚪️ Refuses raw or differently captured entry execution before any command body. */\n'
NEW_DOC = '/** 🚪️ Refuses raw or differently captured entry execution before any command body, whatever implementation pin holds: a malformed proof or an entry evaluated without both identities of the captured loader is never authenticated; well-formed identities that name another snapshot or loader than this process loaded are a stale implementation (the outermost command restarts), and under a pin, which never restarts, the same entry is refused as unauthenticated instead of admitted. */\n'
OLD_IF = 'resolve(proof.root)!==proof.root)throw Error("Authenticated captured build command entry required");if(!implementationPinnedV1()&&(identity!==proof.sha256||loaderIdentity!==proof.loaderSha256||proof.loaderSha256!==createHash("sha256").update(buildSourceClosureStreamLoaderV1).digest("hex")))throw staleImplementationV1("Stale loaded build command loader: the snapshot was captured after a peer saved the loader this process loaded; restart required");}'
NEW_IF = 'resolve(proof.root)!==proof.root||validateJsonSchemaSubset(schema.definitions.CommandIdentities,{identity,loaderIdentity},schema).length)throw Error("Authenticated captured build command entry required");if(identity!==proof.sha256||loaderIdentity!==proof.loaderSha256||proof.loaderSha256!==createHash("sha256").update(buildSourceClosureStreamLoaderV1).digest("hex"))throw implementationPinnedV1()?Error("Authenticated captured build command entry required"):staleImplementationV1("Stale loaded build command loader: the snapshot was captured after a peer saved the loader this process loaded; restart required");}'

OLD_CASE = '''    "CommandCase": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "id",
        "identity",
        "loader",
        "accepted"
      ],
      "properties": {
        "id": {
          "type": "string",
          "minLength": 1
        },
        "identity": {
          "enum": [
            "same",
            "absent",
            "different"
          ]
        },
        "loader": {
          "enum": [
            "same",
            "absent",
            "different"
          ]
        },
        "accepted": {
          "type": "boolean"
        }
      }
    },
'''
NEW_CASE = '''    "CommandIdentities": {
      "description": "The two identities the captured stream loader compiles into every module it evaluates: an entry evaluated without both was never loaded by a captured loader.",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "identity",
        "loaderIdentity"
      ],
      "properties": {
        "identity": {
          "type": "string",
          "pattern": "^[a-f0-9]{64}$"
        },
        "loaderIdentity": {
          "type": "string",
          "pattern": "^[a-f0-9]{64}$"
        }
      }
    },
    "CommandCase": {
      "description": "One evaluated entry against one proof: `proof` says whether the proof names the loader this process loaded or one a peer saved later; `unpinned` and `pinned` are the outcomes without and with an implementation pin (a pin never admits what is refused without it, and never restarts).",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "id",
        "identity",
        "loader",
        "proof",
        "unpinned",
        "pinned"
      ],
      "properties": {
        "id": {
          "type": "string",
          "minLength": 1
        },
        "identity": {
          "enum": [
            "same",
            "absent",
            "different"
          ]
        },
        "loader": {
          "enum": [
            "same",
            "absent",
            "different"
          ]
        },
        "proof": {
          "enum": [
            "loaded",
            "peer-saved"
          ]
        },
        "unpinned": {
          "enum": [
            "accepted",
            "authentication",
            "stale"
          ]
        },
        "pinned": {
          "enum": [
            "accepted",
            "authentication"
          ]
        }
      }
    },
'''

def case(id, identity, loader, proof, unpinned, pinned):
    return '    {\n      "id": "%s",\n      "identity": "%s",\n      "loader": "%s",\n      "proof": "%s",\n      "unpinned": "%s",\n      "pinned": "%s"\n    }' % (id, identity, loader, proof, unpinned, pinned)
OLD_CASES = '''  "cases": [
    {
      "id": "authenticated-captured-entry",
      "identity": "same",
      "loader": "same",
      "accepted": true
    },
    {
      "id": "raw-entry",
      "identity": "absent",
      "loader": "absent",
      "accepted": false
    },
    {
      "id": "changed-source",
      "identity": "different",
      "loader": "same",
      "accepted": false
    },
    {
      "id": "changed-loader",
      "identity": "same",
      "loader": "different",
      "accepted": false
    }
  ],
'''
NEW_CASES = '  "cases": [\n' + ",\n".join([
    case("authenticated-captured-entry", "same", "same", "loaded", "accepted", "accepted"),
    case("raw-entry", "absent", "absent", "loaded", "authentication", "authentication"),
    case("raw-source-identity", "absent", "same", "loaded", "authentication", "authentication"),
    case("raw-loader-identity", "same", "absent", "loaded", "authentication", "authentication"),
    case("changed-source", "different", "same", "loaded", "stale", "authentication"),
    case("changed-loader", "same", "different", "loaded", "stale", "authentication"),
    case("peer-saved-loader", "same", "same", "peer-saved", "stale", "authentication"),
]) + '\n  ],\n'

OLD_IMPORT = 'import{requireCapturedBuildCommandV1}from"../../📥️source/🔏️closure/🚪️command/🟦️.ts";\n'
NEW_IMPORT = OLD_IMPORT + 'import{implementationPinnedV1,withImplementationPinV1}from"../../📥️inputs/🔏️revision/🟨️.mjs";\nimport{STALE_IMPLEMENTATION_CODES_V1}from"../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";\n'
OLD_TEST = 'test("captured command entry refuses changed and absent evaluated identities against SQLite",()=>{const loader=createHash("sha256").update(buildSourceClosureStreamLoaderV1).digest("hex"),identity="a".repeat(64),proof={version:1 as const,root:process.cwd(),snapshot:null,sha256:identity,loaderSha256:loader,recheck:()=>{}},db=new Database(":memory:");try{for(const row of commandFixture.cases){expect(new Ajv({strict:true}).validate(sourceSchema.definitions.CommandCase,row)).toBe(true);const selected=row.identity==="same"?identity:row.identity==="absent"?undefined:"b".repeat(64),selectedLoader=row.loader==="same"?loader:row.loader==="absent"?undefined:"c".repeat(64),oracle=db.query("select coalesce(?1=?2 and ?3=?4,0) accepted").get(selected??null,identity,selectedLoader??null,loader)as{accepted:number};expect(Boolean(oracle.accepted)).toBe(row.accepted);if(row.accepted)expect(()=>requireCapturedBuildCommandV1(proof,selected,selectedLoader)).not.toThrow();else expect(()=>requireCapturedBuildCommandV1(proof,selected,selectedLoader)).toThrow();}}finally{db.close();}});\n'
NEW_TEST = 'test("captured command entry refuses changed and absent evaluated identities with and without an implementation pin against SQLite",async()=>{const loaded=createHash("sha256").update(buildSourceClosureStreamLoaderV1).digest("hex"),identity="a".repeat(64),db=new Database(":memory:"),outcome=(run:()=>void):string=>{try{run();return"accepted";}catch(error){const fault=error as Error&{code?:string};return fault.code!==undefined&&STALE_IMPLEMENTATION_CODES_V1.has(fault.code)?"stale":fault.message==="Authenticated captured build command entry required"&&fault.code===undefined?"authentication":"unexpected: "+fault.message;}};try{expect(new Set(commandFixture.cases.map(row=>row.unpinned+"/"+row.pinned))).toEqual(new Set(["accepted/accepted","authentication/authentication","stale/authentication"]));for(const row of commandFixture.cases){expect(new Ajv({strict:true}).validate(sourceSchema.definitions.CommandCase,row)).toBe(true);const loaderSha256=row.proof==="loaded"?loaded:"d".repeat(64),proof={version:1 as const,root:process.cwd(),snapshot:null,sha256:identity,loaderSha256,recheck:()=>{}},selected=row.identity==="same"?identity:row.identity==="absent"?undefined:"b".repeat(64),selectedLoader=row.loader==="same"?loaderSha256:row.loader==="absent"?undefined:"c".repeat(64),check=()=>requireCapturedBuildCommandV1(proof,selected,selectedLoader);for(const pinned of[false,true]){const oracle=db.query("select case when ?1 is null or ?3 is null then \'authentication\' when ?1=?2 and ?3=?4 and ?4=?5 then \'accepted\' when ?6 then \'authentication\' else \'stale\' end outcome").get(selected??null,identity,selectedLoader??null,loaderSha256,loaded,pinned?1:0)as{outcome:string},actual=pinned?await withImplementationPinV1(async()=>{expect(implementationPinnedV1(),row.id).toBe(true);return outcome(check);}):await routing.withoutImplementationPinV1(async()=>{expect(implementationPinnedV1(),row.id).toBe(false);return outcome(check);});expect(oracle.outcome,row.id).toBe(pinned?row.pinned:row.unpinned);expect(actual,row.id+(pinned?" pinned":" unpinned")).toBe(oracle.outcome);}}}finally{db.close();}});\n'

OLD_CLOSURE_IMPORT = 'import{buildSourceClosureIdentityV1}from"../../📥️source/🔏️closure/🟦️.ts";\n'
NEW_CLOSURE_IMPORT = 'import{buildSourceClosureIdentityV1,buildSourceClosureTextV1}from"../../📥️source/🔏️closure/🟦️.ts";\n'
OLD_PORTS = 'buildSourceClosureStreamLoaderV1,buildSourceClosureIdentityV1,requireIssuedScriptInvocationV1:routing.'
NEW_PORTS = 'buildSourceClosureStreamLoaderV1,buildSourceClosureTextV1,buildSourceClosureIdentityV1,requireIssuedScriptInvocationV1:routing.'
OLD_RUN = 'const run=new Function(...Object.keys(ports),'
NEW_RUN = 'expect(missingCommandPortsV1(tree,fn,ports)).toEqual([]);' + OLD_RUN
OLD_HELPER = '\ntest("actual captured command publication retains'
NEW_HELPER = '\n/** 🔌️ Names the module imports an extracted command function reads that its injected ports do not provide, so a law that evaluates the actual function text fails on the missing port instead of a ReferenceError inside the run. */\nfunction missingCommandPortsV1(tree:ts.SourceFile,fn:ts.Node,ports:object):string[]{const imported=new Set(tree.statements.flatMap(node=>ts.isImportDeclaration(node)&&node.importClause&&!node.importClause.isTypeOnly?[...(node.importClause.name?[node.importClause.name.text]:[]),...(node.importClause.namedBindings&&ts.isNamedImports(node.importClause.namedBindings)?node.importClause.namedBindings.elements.filter(element=>!element.isTypeOnly).map(element=>element.name.text):[])]:[])),seen=new Set<string>(),visit=(node:ts.Node):void=>{if(ts.isIdentifier(node))seen.add(node.text);ts.forEachChild(node,visit);};visit(fn);return[...imported].filter(name=>seen.has(name)&&!Object.hasOwn(ports,name)).sort();}\ntest("actual captured command publication retains'

EDITS = {COMMAND: [(OLD_DOC, NEW_DOC, 1), (OLD_IF, NEW_IF, 1)], SCHEMA: [(OLD_CASE, NEW_CASE, 1)], FIXTURE: [(OLD_CASES, NEW_CASES, 1)], LAW: [(OLD_IMPORT, NEW_IMPORT, 1), (OLD_TEST, NEW_TEST, 1), (OLD_CLOSURE_IMPORT, NEW_CLOSURE_IMPORT, 1), (OLD_PORTS, NEW_PORTS, 2), (OLD_RUN, NEW_RUN, 2), (OLD_HELPER, NEW_HELPER, 1)]}

def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
def sha(data): return hashlib.sha256(data).hexdigest()
def main():
    root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
    batch = sys.argv[sys.argv.index("--batch") + 1] if "--batch" in sys.argv else "identity"
    if not os.path.isdir(os.path.join(root, "🧰️framework")): sys.exit("root without 🧰️framework: " + root)
    out = os.path.join(HERE, "staged", batch)
    if os.path.isdir(out): shutil.rmtree(out)
    os.makedirs(os.path.join(out, "files")); os.makedirs(os.path.join(out, "base"))
    manifest = {}
    for index, (rel, edits) in enumerate(EDITS.items(), 1):
        base = open(os.path.join(root, rel), "rb").read(); text = base.decode("utf8")
        for old, new, times in edits:
            if text.count(old) != times: sys.exit("anchor matches %d times (want %d) in %s: %s" % (text.count(old), times, rel, old[:80]))
            text = text.replace(old, new)
        name = "f%02d" % index
        open(os.path.join(out, "base", name), "wb").write(base); open(os.path.join(out, "files", name), "wb").write(text.encode("utf8"))
        manifest[rel] = {"file": name, "base": sha(base), "new": sha(text.encode("utf8"))}
    json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    for tool in ("stage.py", "verify.sh"): shutil.copy2(os.path.join(HERE, tool), os.path.join(out, tool))
    print("batch", batch, "->", out, len(manifest), "files")
main()
