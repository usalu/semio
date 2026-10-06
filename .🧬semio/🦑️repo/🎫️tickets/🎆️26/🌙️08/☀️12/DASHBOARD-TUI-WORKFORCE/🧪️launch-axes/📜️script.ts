import assert from "node:assert/strict";
import { readFileSync, writeFileSync, renameSync } from "node:fs";
import { join, resolve } from "node:path";
const root=process.cwd(),generated=resolve(import.meta.dir,"../🗑️generated"),dashboard="🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard",mode=process.argv[2];
function edit(path:string,transform:(text:string)=>string):void {const file=join(root,path),before=readFileSync(file,"utf8"),after=transform(before);assert.notEqual(before,after);const stage=join(generated,`launch-axes-${Date.now()}.ts`);writeFileSync(stage,after);assert.equal(readFileSync(file,"utf8"),before);renameSync(stage,file);}
if(mode==="test") {
  edit(`${dashboard}/🌳️command-tree/🧪️tests/🔬️unit/🦀️.rs`,text=>{const start=text.lastIndexOf("#[test]",text.indexOf("fn playground_renderers"));assert.ok(start>=0);const end=text.lastIndexOf("#[test]",text.indexOf("fn workspace_targets"));assert.ok(end>start);return text.slice(0,start)+readFileSync(join(import.meta.dir,"📓️test.rs.md"),"utf8")+"\n"+text.slice(end);});
  edit(`${dashboard}/🧪️tests/🌀️control-plane/🟦️.ts`,text=>text+readFileSync(join(import.meta.dir,"📓️oracle.ts.md"),"utf8"));
}
if(mode==="fix") {
  edit(`${dashboard}/📦️packages/🦀️rust/Cargo.toml`,text=>{const anchor='[dependencies]';assert.ok(text.includes(anchor));return text.replace(anchor,`${anchor}\nui_locale = { path = "../../../../../../🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust", package = "semio-framework-ui-locale" }`);});
  edit(`${dashboard}/🌳️command-tree/🦀️.rs`,text=>{const start=text.indexOf("fn inject_playground_dev(");assert.ok(start>=0);const end=text.indexOf("fn verb_rank(",start);assert.ok(end>start);const result=text.slice(0,start)+readFileSync(join(import.meta.dir,"📓️implementation.rs.md"),"utf8")+"\n"+text.slice(end);const before='    } else {\n        node.children.sort_by(|a, b| a.label.cmp(&b.label));';const after='    } else if node.key == "language" {\n        node.children.sort_by_key(|child| ui_locale::Locale::ALL.iter().position(|locale| locale.as_str() == child.key).unwrap_or(usize::MAX));\n    } else {\n        node.children.sort_by(|a, b| a.label.cmp(&b.label));';assert.ok(result.replaceAll("\r\n","\n").includes(before));return result.replaceAll("\r\n","\n").replace(before,after);});
}
if(mode==="audit") {
  edit('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️live/📜️script.ts',text=>{const before='...(example === "all" ? ["all"] : ["examples", example])])';assert.ok(text.includes(before));return text.replace(before,'...(example === "all" ? ["all"] : ["examples", example]), "language", "en", "terminology", "native"])');});
}
if(mode==="fixture-path") {
  for(const path of [`${dashboard}/🌳️command-tree/🧪️tests/🔬️unit/🦀️.rs`, '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️launch-axes/📓️test.rs.md'])edit(path,text=>text.replace('../../../../🧫️fixtures/🗣️launch-axes/', '../../../🧫️fixtures/🗣️launch-axes/'));
}
if(mode==="ordering-test") {
  const before='            assert!(selection.leaf.is_none(), "language authority cannot be implicit");';
  const after=before+'\n            let language = selection.children.iter().find(|child| child.key == "language").unwrap();\n            assert_eq!(language.children.iter().map(|child|child.key.as_str()).collect::<Vec<_>>(),fixture["locales"].as_array().unwrap().iter().map(|locale|locale.as_str().unwrap()).collect::<Vec<_>>());\n            assert!(language.children.iter().all(|child|child.leaf.is_none()));';
  for(const path of [`${dashboard}/🌳️command-tree/🧪️tests/🔬️unit/🦀️.rs`, '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️launch-axes/📓️test.rs.md'])edit(path,text=>{assert.ok(text.includes(before));return text.replace(before,after);});
}
if(mode==="lock")edit('Cargo.lock',text=>{const start=text.indexOf('name = "semio-framework-repo-dashboard"'),end=text.indexOf('[[package]]',start);assert.ok(start>=0&&end>start);const section=text.slice(start,end);assert.ok(!section.includes('"semio-framework-ui-locale"'));return text.slice(0,start)+section.replace(' "semio-framework-ui",',' "semio-framework-ui",\n "semio-framework-ui-locale",')+text.slice(end);});
if(mode==="smoke-audit")edit('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️live/📜️script.ts',text=>{
  const changes=[
    ['let node = tree;', 'const menuRenderer=renderer==="native-smoke"?"wgpu-native":renderer;\nlet node = tree;'],
    ['["dev", "draw", "draw", renderer,','["dev", "draw", "draw", menuRenderer,'],
    ['const leaf = node.leaf, command =','if(renderer==="native-smoke")extra.push(...(node.leaf.args.includes("--")?[]:["--"]),"--smoke");\nconst leaf = node.leaf, command ='],
    ['if (renderer === "target") {\n    if (session', 'if (renderer === "target" || renderer === "native-smoke") {\n    if (session'],
    ['if (renderer === "react" || renderer === "wgpu-wasm") await browserRuntime();','if (renderer === "react" || renderer === "wgpu-wasm") await browserRuntime();\n  if(renderer==="native-smoke"){const plain=text.replace(/\\x1b\\[[0-9;?]*[A-Za-z]/g, "");const match=plain.match(/\\{\\s*"booted"\\s*:\\s*true,[\\s\\S]*?"windowDocuments"\\s*:\\s*\\[[\\s\\S]*?\\]\\s*\\}/);assert.ok(match,plain.slice(-6000));const report=JSON.parse(match[0]);assert.equal(report.booted,true);assert.equal(report.session?.pluginId,"draw");console.log(`[DEBUG] native smoke ${JSON.stringify(report)}`);}'],
    ['renderer !== "wgpu-native" && renderer !== "target"', 'renderer !== "wgpu-native" && renderer !== "target" && renderer !== "native-smoke"']
  ];for(const [before,after]of changes){assert.ok(text.includes(before),before);text=text.replace(before,after);}return text;
});
