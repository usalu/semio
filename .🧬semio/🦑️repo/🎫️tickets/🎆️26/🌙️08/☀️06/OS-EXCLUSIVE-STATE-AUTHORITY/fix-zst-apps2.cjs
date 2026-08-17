const { readFileSync, writeFileSync, readdirSync } = require("fs");
const { join } = require("path");
const plugins = join("✏️s","🔌️plugins");
function findPlugin(pred){ return readdirSync(plugins).find(pred); }
const lowpoly = findPlugin(x=>[...x].some(c=>c.codePointAt(0)===0x1F3ED));
const cad = findPlugin(x=>[...x].some(c=>c.codePointAt(0)===0x1F4D0));
const puzzle = findPlugin(x=>[...x].some(c=>c.codePointAt(0)===0x1F9E9));
const space = findPlugin(x=>[...x].some(c=>c.codePointAt(0)===0x1FA90));
function zst(file, structName) {
  let s = readFileSync(file, "utf8");
  const before = s;
  const re = new RegExp("struct " + structName + " \\{[\\s\\S]*?\\n\\}");
  const m = s.match(re);
  if (!m) { console.log("skip missing", structName, file); return fieldsFrom(s, structName); }
  const fields = [...m[0].matchAll(/^\\s*(?:pub(?:\\([^)]*\\))?\\s+)?(\\w+)\\s*:/gm)].map(x => x[1]);
  s = s.replace(re, "#[derive(Default, Clone, Copy)]\nstruct " + structName + ";");
  // Replace struct literal initializers Self { ... } when assigning the app type
  s = s.replace(new RegExp("\\b" + structName + "\\s*\\{[^;]*?\\}", "g"), (block) => {
    if (/^struct /.test(block)) return block;
    return structName;
  });
  writeFileSync(file, s);
  console.log("zst", structName, fields.join(","), file);
}
function fieldsFrom(){}
zst(join(plugins,lowpoly,"🎛️apps",lowpoly,"🦀️component.rs"), "LowpolyPlayApp");
zst(join(plugins,cad,"🎛️apps",cad,"🦀️component.rs"), "CadPlayApp");
zst(join(plugins,space,"🎛️apps",readdirSync(join(plugins,space,"🎛️apps")).find(x=>x.includes("home")||[...x].some(c=>c.codePointAt(0)===0x1F3E0)),"🦀️component.rs"), "HomeApp");
zst(join(plugins,space,"🎛️apps",space,"🦀️component.rs"), "SpaceApp");
const puzzleApps=join(plugins,puzzle,"🎛️apps");
for (const app of readdirSync(puzzleApps)) {
  const f=join(puzzleApps,app,"🦀️component.rs");
  try{readFileSync(f);}catch{continue;}
  const s=readFileSync(f,"utf8");
  for (const name of ["Puzzle2dPlayApp","Puzzle3dPlayApp","Puzzle5dPlayApp"]) {
    if (s.includes("struct "+name+" {")) zst(f, name);
  }
}
