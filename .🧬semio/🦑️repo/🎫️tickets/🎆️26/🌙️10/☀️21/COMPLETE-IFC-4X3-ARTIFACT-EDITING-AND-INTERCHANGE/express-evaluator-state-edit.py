from pathlib import Path
p=Path("C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/evaluator.ts")
s=p.read_text(encoding="utf-8")
old=" work=0;depth=0;readonly declarations="
assert old in s
s=s.replace(old," #work=0;#depth=0;readonly work!:number;readonly depth!:number;readonly declarations=",1)
old=" readonly control:ExpressControl;private readonly controlScope:ReturnType<typeof captureExpressControlScope>;"
assert old in s
s=s.replace(old," readonly control!:ExpressControl;readonly #controlScope:ReturnType<typeof captureExpressControlScope>;",1)
old="this.controlScope=captureExpressControlScope(control);this.control=this.controlScope.control;"
assert old in s
s=s.replace(old,"this.#controlScope=captureExpressControlScope(control);Object.defineProperties(this,{work:{get:()=>this.#work,enumerable:true,configurable:false},depth:{get:()=>this.#depth,enumerable:true,configurable:false},control:{value:this.#controlScope.control,enumerable:true,writable:false,configurable:false}});",1)
s=s.replace("this.controlScope","this.#controlScope").replace("this.control.","this.#controlScope.control.").replace("this.work","this.#work").replace("this.depth","this.#depth")
p.write_text(s,encoding="utf-8")
