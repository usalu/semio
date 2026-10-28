from pathlib import Path
import json
ticket=Path(__file__).parent
root=ticket.parents[6]
path=root/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🟦️.ts"
source=path.read_text(encoding="utf8")
a=source.index("class Control{");b=source.index("interface Plane",a)
source=source[:a]+'''class Control{
 #work=0;readonly #budget:number;readonly #faces:number;readonly #depth:number;readonly #observer?:()=>void;readonly #progress?:(work:number,budget:number)=>boolean;
 constructor(options:SolidBooleanOptions){this.#budget=options.maxWork??2_000_000;this.#faces=options.maxPolygons??200_000;this.#depth=options.maxDepth??256;this.#observer=options.tick;this.#progress=options.progress;if([this.#budget,this.#faces,this.#depth].some(v=>!Number.isSafeInteger(v)||v<1))fail("invalid-limits");}
 tick(cost=1):void{this.#work+=cost;if(this.#work>this.#budget)fail("work-limit");const observer=this.#observer,progress=this.#progress;observer?.();if(progress?.(this.#work,this.#budget)===false)fail("cancelled");}
 check(depth:number,count=0):void{if(depth>this.#depth)fail("depth-limit");if(count>this.#faces)fail("polygon-limit");}
}
'''+source[b:]
path.write_text(source,encoding="utf8")
manifest=ticket/"🗑️generated/boolean-cooperation-source.json"
old=ticket/"🗑️generated/boolean-cooperation-before-receiver-source.json"
assert not old.exists()
old.write_bytes(manifest.read_bytes())
census=ticket/"boolean-cooperation-source-census.py"
text=census.read_text(encoding="utf8").replace('"cooperative.test.ts"]','"cooperative.test.ts","control-receiver.json","control-receiver.schema.json","control-receiver.test.ts"]')
census.write_text(text,encoding="utf8")
config=ticket/"boolean-tsconfig.json"
value=json.loads(config.read_text(encoding="utf8"))
value["files"].append("../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🧪️tests/control-receiver.test.ts")
config.write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG] callback invocation is unbound and control grants/counters are ECMAScript-private")

