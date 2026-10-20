# 🔒 Hermetic classes: the class table declares its toolchains (bun provisioned, go, python, dotnet and cmake pending) and six non-TypeScript classes over them; a member of a class whose toolchain is not provisioned as a declared tree keeps host state under the closed reason `toolchain`, with the pin of the toolchain in its key, until the toolchain flips to provisioned.
import json
from layout import *
from patchlib import chain, replace_n

H = B + "/🏃️execution/🔒️hermetic"
TABLE = H + "/🔣️.json"
ENGINE = H + "/🟦️.ts"
TEST = H + "/🧪️tests/🟦️.ts"
SCHEMA = B + "/🏃️execution/🧬️schema/🔒️hermetic-classes.json"
EXECUTION = BZ + "/🏃️execution/🟦️.ts"
EXECUTION_SCHEMA = BZ + "/🏃️execution/🧬️schema/🔣️.json"
MARKER = "the non-TypeScript classes"

def pattern(kind, project):
    return '^bun "[^"]*🧰️native/📜️script\\.ts" fingerprint ' + kind + (' ".*"' if project else '') + '$'

PENDING = {
    "go": {"en": "The pinned Go distribution and the module cache are not yet declared as content-addressed trees of the action.", "de": "Die gepinnte Go-Distribution und der Modul-Cache sind noch nicht als inhaltsadressierte Bäume der Aktion deklariert."},
    "python": {"en": "The managed interpreter and the locked environment are not yet declared as content-addressed trees of the action.", "de": "Der verwaltete Interpreter und die gesperrte Umgebung sind noch nicht als inhaltsadressierte Bäume der Aktion deklariert."},
    "dotnet": {"en": "The pinned .NET SDK and the restored packages are not yet declared as content-addressed trees of the action.", "de": "Das gepinnte .NET SDK und die wiederhergestellten Pakete sind noch nicht als inhaltsadressierte Bäume der Aktion deklariert."},
    "cmake": {"en": "The pinned CMake, Ninja and the compiler are not yet declared as content-addressed trees of the action.", "de": "Das gepinnte CMake, Ninja und der Compiler sind noch nicht als inhaltsadressierte Bäume der Aktion deklariert."},
}

def indent(text, spaces):
    return "\n".join((" " * spaces + line) if line else line for line in text.split("\n"))

def toolchains_block():
    rows = [{"id": "bun", "identity": "bun", "provisioned": True, "covered": []}]
    for kind in ("go", "python", "dotnet", "cmake"):
        rows.append({"id": kind, "identity": kind, "provisioned": False, "covered": [pattern(kind, kind == "python")], "pending": PENDING[kind]})
    return '  "toolchains": ' + indent(json.dumps(rows, ensure_ascii=False, indent=2), 2).lstrip() + ',\n'

def klass(identifier, toolchain, description, targets, projects):
    return {"id": identifier, "description": description, "evidence": "assumed", "toolchain": toolchain, "select": {"targets": targets, "projects": projects, "excludeRoots": ["**/🦀️*", "**/🦀️*/**"]}, "dependencies": "toolchain", "packages": [], "providers": {}}

CLASSES = [
    klass("go-test", "go", "Go test targets of the Go modules: the repository script runs the declared module tests, which read only the declared sources and the module cache the toolchain provides.", ["test", "test-*"], ["@semio-tech/*-go"]),
    klass("go-build", "go", "Go build targets of the Go modules: the compiler reads the declared sources and writes only the declared outputs.", ["build", "build-*"], ["@semio-tech/*-go"]),
    klass("python-test", "python", "Python test targets of the uv projects: the managed interpreter runs the declared tests inside the locked environment of the project.", ["test", "test-*"], ["@semio-tech/*-py"]),
    klass("python-build", "python", "Python build targets of the uv projects: the build backend reads the declared sources and the locked environment and writes only the declared outputs.", ["build", "build-*"], ["@semio-tech/*-py"]),
    klass("dotnet-build", "dotnet", "Build targets of the .NET projects: the pinned SDK compiles the declared sources into an artifacts path outside the package.", ["build", "build-*"], ["@semio-tech/*-dotnet"]),
    klass("cmake-check", "cmake", "Pure laws of the CMake driver: a function of the CMake root files, the native sources and the pinned CMake and Ninja.", ["cpp-custody", "cpp-test-toolchain"], ["workspace"]),
]

def classes_block():
    return ",\n".join(indent(json.dumps(row, ensure_ascii=False, indent=2), 4) for row in CLASSES)

def table(text):
    start = text
    text, _ = chain(text, [("table-toolchains", '  "classes": [\n', toolchains_block() + '  "classes": [\n')])
    text, _ = replace_n(text, '      "evidence": "assumed",\n      "select": {', '      "evidence": "assumed",\n      "toolchain": "bun",\n      "select": {', 5, "table-bun")
    text, _ = chain(text, [("table-classes", '      "providers": {}\n    }\n  ],\n  "exceptions": []\n}', '      "providers": {}\n    },\n' + classes_block() + '\n  ],\n  "exceptions": []\n}')])
    return text, text != start

def schema(text):
    return chain(text, [
        ("schema-required", '    "strategies",\n    "classes",\n    "exceptions"\n  ],', '    "strategies",\n    "toolchains",\n    "classes",\n    "exceptions"\n  ],'),
        ("schema-property", '    "strategies": {\n      "$ref": "#/definitions/Strategies"\n    },\n', '    "strategies": {\n      "$ref": "#/definitions/Strategies"\n    },\n    "toolchains": {\n      "type": "array",\n      "minItems": 1,\n      "maxItems": 16,\n      "items": {\n        "$ref": "#/definitions/Toolchain"\n      }\n    },\n'),
        ("schema-dependencies", '    "Dependencies": {\n      "description": "`links` links the workspace packages the script imports by name; `packages` adds the third-party packages the project and the class name, as one content-addressed tree.",\n      "enum": [\n        "links",\n        "packages"\n      ]\n    },',
         '    "Dependencies": {\n      "description": "`links` links the workspace packages the script imports by name; `packages` adds the third-party packages the project and the class name, as one content-addressed tree; `toolchain` leaves the dependencies to the toolchain of the class, which provides them as its own tree.",\n      "enum": [\n        "links",\n        "packages",\n        "toolchain"\n      ]\n    },\n    "Toolchain": {\n      "description": "A toolchain a class runs on. A provisioned toolchain is a declared input of the action; an unprovisioned one keeps its members on host state under the reason `toolchain`, keyed by the pin of the toolchain, until it is provisioned. `covered` lists the runtime commands the toolchain already determines, so a member may drop them from its inputs.",\n      "type": "object",\n      "additionalProperties": false,\n      "required": [\n        "id",\n        "identity",\n        "provisioned",\n        "covered"\n      ],\n      "properties": {\n        "id": {\n          "$ref": "#/definitions/Id"\n        },\n        "identity": {\n          "enum": [\n            "bun",\n            "go",\n            "python",\n            "dotnet",\n            "cmake"\n          ]\n        },\n        "provisioned": {\n          "type": "boolean"\n        },\n        "covered": {\n          "type": "array",\n          "uniqueItems": true,\n          "items": {\n            "type": "string",\n            "minLength": 1,\n            "maxLength": 512\n          }\n        },\n        "pending": {\n          "type": "object",\n          "additionalProperties": false,\n          "required": [\n            "en",\n            "de"\n          ],\n          "properties": {\n            "en": {\n              "type": "string",\n              "minLength": 1\n            },\n            "de": {\n              "type": "string",\n              "minLength": 1\n            }\n          }\n        }\n      }\n    },'),
        ("schema-class-required", '        "id",\n        "description",\n        "evidence",\n        "select",\n', '        "id",\n        "description",\n        "evidence",\n        "toolchain",\n        "select",\n'),
        ("schema-class-property", '        "select": {\n          "$ref": "#/definitions/Select"\n        },\n        "dependencies": {', '        "toolchain": {\n          "$ref": "#/definitions/Id"\n        },\n        "select": {\n          "$ref": "#/definitions/Select"\n        },\n        "dependencies": {'),
    ])

def engine(text):
    return chain(text, [
        ("engine-inputs", 'export function hermeticInputsV1(inputs: readonly unknown[], environment: Readonly<Record<string, string | undefined>>): HermeticInputsV1 | {readonly refused: "host-inputs"} {\n  const env: Record<string, string> = {}, packages = new Set<string>(), admitted = new Set(table.environment.names), covered = new Set(table.runtime.covered);',
         'export function hermeticInputsV1(inputs: readonly unknown[], environment: Readonly<Record<string, string | undefined>>, toolchain: string = "bun"): HermeticInputsV1 | {readonly refused: "host-inputs"} {\n  const env: Record<string, string> = {}, packages = new Set<string>(), admitted = new Set(table.environment.names), covered = new Set(table.runtime.covered), patterns = hermeticToolchainV1(toolchain).covered.map(source => new RegExp(source, "u"));'),
        ("engine-runtime", '      if (!covered.has(row.runtime)) return {refused: "host-inputs"};', '      if (!covered.has(row.runtime) && !patterns.some(pattern => pattern.test(row.runtime as string))) return {refused: "host-inputs"};'),
        ("engine-toolchain", '/** 🧮️ Reads the environment and package inputs of a target:', '/** 🧰️ One toolchain of the table by its id. */\nexport function hermeticToolchainV1(id: string): (typeof table.toolchains)[number] {\n  const row = table.toolchains.find(entry => entry.id === id);\n  if (!row) throw new Error(`Unknown hermetic toolchain: ${id}`);\n  return row;\n}\n\n/** 🧮️ Reads the environment and package inputs of a target:'),
    ])

def execution(text):
    return chain(text, [
        ("execution-import", 'import{HERMETIC_TABLE_V1,hermeticInputsV1,hermeticSelectV1', 'import{HERMETIC_TABLE_V1,hermeticInputsV1,hermeticSelectV1,hermeticToolchainV1'),
        ("execution-decision", ' if(selected?.kind==="excepted")return{hermetic:false,reason:"excepted"};\n', ' if(selected?.kind==="excepted")return{hermetic:false,reason:"excepted"};\n if(selected?.kind==="class"&&!hermeticToolchainV1(selected.class.toolchain).provisioned)return{hermetic:false,reason:"toolchain"};\n'),
        ("execution-host", 'if(selected?.kind==="class"?"refused" in hermeticInputsV1(rows,{}):', 'if(selected?.kind==="class"?"refused" in hermeticInputsV1(rows,{},selected.class.toolchain):'),
        ("execution-plan", 'const host=hermeticInputsV1(options.values,options.environment);', 'const host=hermeticInputsV1(options.values,options.environment,row.toolchain);'),
    ])

def execution_schema(text):
    return chain(text, [("execution-reason", '        "dependencies": "The installed dependency generation cannot be declared as a content-addressed tree: it is absent or an entry leaves its layout."\n', '        "dependencies": "The installed dependency generation cannot be declared as a content-addressed tree: it is absent or an entry leaves its layout.",\n        "toolchain": "The class runs on a toolchain the class table has not provisioned as a declared tree of the action yet, so the target keeps host state, keyed by the pin of that toolchain."\n')])

TEST_CODE = '''
test("the non-TypeScript classes name a provisioned or a pending toolchain and every pending member keeps host state keyed by its pin",()=>{
 const toolchains=new Map(HERMETIC_TABLE_V1.toolchains.map(row=>[row.id,row]));
 expect(toolchains.size).toBe(HERMETIC_TABLE_V1.toolchains.length);
 for(const row of HERMETIC_TABLE_V1.classes)expect(toolchains.has(row.toolchain),row.id).toBe(true);
 for(const row of HERMETIC_TABLE_V1.toolchains){
  expect(row.provisioned||"pending" in row,row.id).toBe(true);
  expect(row.provisioned&&row.covered.length>0).toBe(false);
  for(const source of row.covered)expect(()=>new RegExp(source,"u")).not.toThrow();
 }
 expect(HERMETIC_TABLE_V1.classes.filter(row=>row.toolchain==="bun").every(row=>row.dependencies!=="toolchain")).toBe(true);
 expect(HERMETIC_TABLE_V1.classes.filter(row=>row.toolchain!=="bun").every(row=>row.dependencies==="toolchain"&&row.providers&&Object.keys(row.providers).length===0)).toBe(true);
 const command=(kind:string,tail="")=>`bun "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/📜️script.ts" fingerprint ${kind}${tail}`;
 expect(hermeticInputsV1([{runtime:command("python",' "a/b"')}],{},"python")).toEqual({env:{},packages:[]});
 expect(hermeticInputsV1([{runtime:command("go")}],{},"go")).toEqual({env:{},packages:[]});
 expect("refused" in hermeticInputsV1([{runtime:command("go")}],{},"python")).toBe(true);
 expect("refused" in hermeticInputsV1([{runtime:command("go")}],{})).toBe(true);
 expect("refused" in hermeticInputsV1([{runtime:"go version"}],{},"go")).toBe(true);
 expect("refused" in hermeticInputsV1([{runtime:command("go","; rm -rf /")}],{},"go")).toBe(true);
 for(const [project,target,id] of [["@semio-tech/repo-tickets-go","test","go-test"],["@semio-tech/repo-cli-go","build","go-build"],["@semio-tech/ui-styling-py","test","python-test"],["@semio-tech/ui-styling-py","build","python-build"],["@semio-tech/ui-styling-dotnet","build","dotnet-build"],["workspace","cpp-custody","cmake-check"],["workspace","cpp-test-toolchain","cmake-check"]] as const){
  const selected=hermeticSelectV1({project,target,root:"x"});
  expect({project,target,kind:selected.kind,class:selected.kind==="class"?selected.class.id:undefined}).toEqual({project,target,kind:"class",class:id});
 }
 expect(hermeticSelectV1({project:"@semio-tech/repo-tickets-go",target:"dev",root:"x"}).kind).toBe("unselected");
 expect(hermeticSelectV1({project:"workspace",target:"cpp-build",root:"x"}).kind).toBe("unselected");
});
'''

def test_append(text):
    if MARKER in text: return text, False
    return text.rstrip("\n") + "\n" + TEST_CODE, True

def edits():
    return [(TABLE, table), (SCHEMA, schema), (ENGINE, engine), (EXECUTION, execution), (EXECUTION_SCHEMA, execution_schema), (TEST, test_append)]
