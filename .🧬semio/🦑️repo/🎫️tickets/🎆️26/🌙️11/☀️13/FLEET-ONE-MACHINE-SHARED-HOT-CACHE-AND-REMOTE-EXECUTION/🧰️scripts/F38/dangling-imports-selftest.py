import importlib.util,sys
sys.argv=["x"]
src=open(__import__("os").path.join(__import__("os").path.dirname(__import__("os").path.abspath(__file__)),"..","coordinator","dangling-imports.py")).read().split("missing = 0")[0]
g={};exec(src,g)
s=lambda t:list(g["specifiers"](t))
assert s('import a from "./a.ts";\nimport {b} from "../b";\nimport "./c";\nexport * from "./d";')==["./a.ts","../b","./c","./d"]
assert s('const x=await import("./e");const y=require("./f");')==["./e","./f"]
assert s('const t=`import a from "./nope"`;')==[]
assert s('expect(source).toContain(\'import * as o from "../../x.mjs"\');')==[]
assert s('/* import a from "./c" */ // import "./d"\n')==[]
assert s('const t=`${f()} x ${ `import a from "./z"` }`; import q from "./q";')==["./q"]
assert s('const r=/import from "x"/; import q from "./q";')==["./q"]
assert s('const a=b/2; const c=d/3; import z from "./z"')==["./z"]
assert s('new URL("./x", import.meta.url)')==[]
assert s('const o={ "./x": 1 }')==[]
assert s('f(`a${{a:1}.a}`); import q from "./q"')==["./q"]
print("ok")
assert g["decode"]("../\\uD83E\\uDDF0\\uFE0Fx")=="../\U0001F9F0️x", g["decode"]("../\\uD83E\\uDDF0\\uFE0Fx")
assert g["decode"]("a\\u{1f9eb}b")=="a\U0001f9ebb"
assert g["decode"]("\\ud83d\\udd28\\ufe0f")=="\U0001f528️"
print("ok2")
