B = L + "/🏗️build"
SCRIPT = B + "/🚪️door/📜️script.ts"
MODE = B + "/🏛️sealed/🧭️mode/🟦️.ts"
EDITS = [
    (MODE, None, '''import policy from"../🔣️.json"with{type:"json"};

/** 🧭️ Tells the front-door script, before it loads anything else, whether this start runs the live tree: it reads the policy table only, so a live start never loads the sealed store, its trampoline or the promoter; every other value, a malformed one included, is answered by the trampoline, which owns the refusal text. A law holds it to the verdict of `implementationModeV1`.
 * @see ../🟦️.ts the policy and the mode rule, ../🚦️trampoline/🟦️.ts the start of a sealed run */
export function liveImplementationStartV1(environment:Readonly<Record<string,string|undefined>>):boolean{
 const raw=environment[policy.modeVariable];
 return raw===undefined||raw===""?policy.defaultMode==="live":raw.toLowerCase()==="live";
}
'''),
    (SCRIPT, '''import {findSealedRootV1} from "../🏛️sealed/🟦️.ts";
import {startImplementationV1} from "../🏛️sealed/🚦️trampoline/🟦️.ts";
''', '''import {liveImplementationStartV1} from "../🏛️sealed/🧭️mode/🟦️.ts";
'''),
    (SCRIPT, ''' const argv=process.argv.slice(2),root=findSealedRootV1(import.meta.dirname);
 await startImplementationV1(root,argv,process.env,async()=>{const{runBuildDoorMainV1}=await import("./🏃️main/🟦️.ts");await runBuildDoorMainV1(root,argv,process.env);});
''', ''' const argv=process.argv.slice(2),runLive=async(root:string)=>{const{runBuildDoorMainV1}=await import("./🏃️main/🟦️.ts");await runBuildDoorMainV1(root,argv,process.env);};
 if(liveImplementationStartV1(process.env)){const{findBuildDoorRootV1}=await import("./🟦️.ts");await runLive(findBuildDoorRootV1(import.meta.dirname));}
 else{
  const[{findSealedRootV1},{startImplementationV1}]=await Promise.all([import("../🏛️sealed/🟦️.ts"),import("../🏛️sealed/🚦️trampoline/🟦️.ts")]),root=findSealedRootV1(import.meta.dirname);
  await startImplementationV1(root,argv,process.env,()=>runLive(root));
 }
'''),
]

STARTUP = B + "/🚪️door/🧪️tests/🚦️startup"
GROUPS = '''      "id": "host-capacity",
      "match": [
        "/🏗️build/🌡️host-capacity/",
        "/🏃️process/🏗️build/🌡️host-capacity/"
      ]
    }
  ],'''
ROW_TAIL = '''        "host-capacity"
      ],
      "maximumModules": 17,
      "maximumBytes": 96000'''
LEAF_FORBID = '''        "graph",
        "native-build",
        "test-harness",
        "cargo",
        "caller",
        "invocation",
        "selection",
        "resident",
        "bootstrap",
        "store-receipt",
        "tools",
        "revision",
        "routing",
        "leases",
        "host-capacity",
        "sealed"'''


def startup_table(text):
    assert text.count(GROUPS) == 1 and text.count(ROW_TAIL) == 4
    text = text.replace(GROUPS, '''      "id": "host-capacity",
      "match": [
        "/🏗️build/🌡️host-capacity/",
        "/🏃️process/🏗️build/🌡️host-capacity/"
      ]
    },
    {
      "id": "sealed",
      "match": [
        "/🏗️build/🏛️sealed/"
      ],
      "except": [
        "/🏗️build/🏛️sealed/🧭️mode/"
      ]
    }
  ],''')
    text = text.replace(ROW_TAIL, '''        "host-capacity",
        "sealed"
      ],
      "maximumModules": 16,
      "maximumBytes": 74000''')
    leaf = '''    {
      "id": "sealed-mode-leaf",
      "kind": "import",
      "module": "../../../🏛️sealed/🧭️mode/🟦️.ts",
      "forbid": [
''' + LEAF_FORBID + '''
      ],
      "maximumModules": 1,
      "maximumBytes": 1500
    },
    {
      "id": "negative-control-cargo-execution",'''
    anchor = '''    {
      "id": "negative-control-cargo-execution",'''
    assert text.count(anchor) == 1
    return text.replace(anchor, leaf)


EDITS.append((STARTUP + "/🔣️.json", startup_table, None))
EDITS.append((STARTUP + "/🟦️.ts", 'const trampoline=["/🏛️sealed/🟦️.ts","/🏛️sealed/🚦️trampoline/🟦️.ts","/🚪️door/🏃️main/🟦️.ts"]', 'const trampoline=["/🏛️sealed/🟦️.ts","/🏛️sealed/🚦️trampoline/🟦️.ts","/🏛️sealed/🧭️mode/🟦️.ts","/🚪️door/🏃️main/🟦️.ts"]'))

EDITS.append((B + "/🏛️sealed/🧭️mode/🧪️tests/🟦️.ts", None, '''/** 🧭️ Contract of the live-start predicate of the front-door script: it gives exactly the verdict of the mode rule of the sealed store (`implementationModeV1`), a value that rule refuses is never a live start, and the module reads the policy table only, so a live start loads nothing of the store. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{implementationModeV1}from"../../🟦️.ts";
import policy from"../../🔣️.json";
import{liveImplementationStartV1}from"../🟦️.ts";

const values:(string|undefined)[]=[undefined,"","live","LIVE","Live","sealed","SEALED","promote","fast"," live","live ","l"];

test("a live start is exactly the verdict of the mode rule of the store",()=>{
 for(const value of values){
  const environment={[policy.modeVariable]:value};
  let verdict:boolean;try{verdict=implementationModeV1(environment)==="live";}catch{verdict=false;}
  expect(liveImplementationStartV1(environment),String(value)).toBe(verdict);
 }
 expect(liveImplementationStartV1({})).toBe(implementationModeV1({})==="live");
});

test("the predicate imports the policy table and nothing else",()=>{
 const text=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8");
 expect([...text.matchAll(/from\\s*"([^"]+)"/gu)].map(match=>match[1])).toEqual(["../🔣️.json"]);
});
'''))
