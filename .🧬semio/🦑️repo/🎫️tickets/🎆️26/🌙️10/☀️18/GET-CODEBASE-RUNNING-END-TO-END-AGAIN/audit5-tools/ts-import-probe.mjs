const m = await import(process.argv[2]);
const cases = {
  base: `import {a} from "./a.ts";\nexport const x=1;`,
  reexportStar: `import {a} from "./a.ts";\nexport * from "./b.ts";`,
  reexportNamed: `import {a} from "./a.ts";\nexport {y} from "./c.ts";`,
  typeOnly: `import type {a} from "./a.ts";\nexport const x=1;`,
  dynamic: `import {a} from "./a.ts";\nconst z=await import("./d.ts");`,
  importMeta: `import {a} from "./a.ts";\nnew URL("./e.json", import.meta.url);`,
  requireCall: `import {a} from "./a.ts";\nrequire("./f.cjs");`,
  jsonWith: `import {a} from "./a.ts";\nimport s from "./g.json" with {type:"json"};`,
};
for (const [k, v] of Object.entries(cases)) {
  try { console.log(k, JSON.stringify(m.moduleSourceImports("/x/y.ts", v)), JSON.stringify(m.productionModuleSourceImports?.("/x/y.ts", v)), JSON.stringify(m.runtimeModuleSourceImports?.("/x/y.ts", v))); } catch (e) { console.log(k, "THROW", e.message); }
}
