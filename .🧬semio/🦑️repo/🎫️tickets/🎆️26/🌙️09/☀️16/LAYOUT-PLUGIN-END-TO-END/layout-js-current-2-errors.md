# LayoutJS Current2 Actual Failure Details

```text
@semio-tech/layout-js: 
@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📄️paragraph/🧪️tests/🟦️.ts:
@semio-tech/layout-js: 1 | /** ⏱️ A defining owner commits actual bounded work through one expiring observation loan. */
@semio-tech/layout-js: 2 | export interface RetainedWorkTurn{commitActualWork(units?:number):void;actualWork():number}
@semio-tech/layout-js: 3 | const activeTurns=new WeakSet<RetainedWorkTurn>();
@semio-tech/layout-js: 4 | /** 🔐️ Only a defining counter's current issued observation loan may receive actual work. */
@semio-tech/layout-js: 5 | export function assertRetainedWorkTurn(turn:RetainedWorkTurn):void{if(!activeTurns.has(turn))throw Error("Retained work turn loan is unissued or expired");turn.actualWork();}
@semio-tech/layout-js:                                                                                                        ^
@semio-tech/layout-js: error: Retained work turn loan is unissued or expired
@semio-tech/layout-js:       at assertRetainedWorkTurn (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/⏱️turn/🟦️.ts:5:100)
@semio-tech/layout-js:       at slot (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts:70:70)
@semio-tech/layout-js:       at step (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📄️paragraph/🟦️.ts:20:38)
@semio-tech/layout-js:       at advance (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📄️paragraph/🟦️.ts:23:485)
@semio-tech/layout-js:       at checkLayoutFlow (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📄️paragraph/🧪️tests/🧵️flow/🟦️.ts:11:1734)
@semio-tech/layout-js:       at run (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📄️paragraph/🧪️tests/🧵️flow/🔮️oracle.mjs:2:1493)
@semio-tech/layout-js:       at async <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📄️paragraph/🧪️tests/🟦️.ts:4:73)
@semio-tech/layout-js: ✗ shared story flow and address cache [265.01ms]
@semio-tech/layout-js: 
@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧪️tests/🟦️.ts:
@semio-tech/layout-js: 1 | /** 🔮️ Test-only first-party resource admission and glyph coverage are checked by Node crypto and independent OpenType. */
@semio-tech/layout-js: 2 | import fs from 'node:fs';import path from 'node:path';import {createRequire} from 'node:module';import {fileURLToPath} from 'node:url';import crypto from 'node:crypto';import assert from 'node:assert/strict';
@semio-tech/layout-js: 3 | export function repositoryRoot(){let at=path.dirname(fileURLToPath(import.meta.url));for(;;){if(fs.existsSync(path.join(at,'package.json'))&&fs.existsSync(path.join(at,'🧰️framework')))return at;const parent=path.dirname(at);if(parent===at)throw Error('Repository root unavailable');at=parent;}}
@semio-tech/layout-js: 4 | export function run(root){const require=createRequire(path.join(root,'package.json')),ts=require('typescript'),opentype=require('opentype.js'),Ajv=require('ajv/dist/2020'),owner=path.join(root,'🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font'),layout=path.join(root,'✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any'),catalogRoot=path.join(layout,'✏️editor/⚙️engine/🎬️scene/🔤️text-fonts'),old=require.extensions['.ts'],nativeTypescript=typeof Bun!=='undefined';if(!nativeTypescript)require.extensions['.ts']=(mod,file)=>mod._compile(ts.transpileModule(fs.readFileSync(file,'utf8'),{fileName:file,compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.CommonJS,esModuleInterop:true}}).outputText,file);try{const {layoutFontCount,layoutFontSource}=require(path.join(catalogRoot,'🟦️.ts')),{FontFaceAdmissionJob}=require(path.join(owner,'🟦️.ts')),{RetainedWorkLedger}=require(path.join(owner,'🎟️ | ... truncated 
@semio-tech/layout-js: ✓ Layout source adoption refuses wrong-snapshot-close [3.23ms]
@semio-tech/layout-js: ✓ Layout source adoption refuses comment-only and duplicated physical owners [6.11ms]
@semio-tech/layout-js: 1 | /** ⏱️ A defining owner commits actual bounded work through one expiring observation loan. */
@semio-tech/layout-js: 2 | export interface RetainedWorkTurn{commitActualWork(units?:number):void;actualWork():number}
@semio-tech/layout-js: 3 | const activeTurns=new WeakSet<RetainedWorkTurn>();
@semio-tech/layout-js: 4 | /** 🔐️ Only a defining counter's current issued observation loan may receive actual work. */
@semio-tech/layout-js: 5 | export function assertRetainedWorkTurn(turn:RetainedWorkTurn):void{if(!activeTurns.has(turn))throw Error("Retained work turn loan is unissued or expired");turn.actualWork();}
@semio-tech/layout-js:                                                                                                                                                              ^
@semio-tech/layout-js: error: Retained work turn loan is unissued or expired
@semio-tech/layout-js:       at assertRetainedWorkTurn (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/⏱️turn/🟦️.ts:5:154)
@semio-tech/layout-js:       at slot (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts:70:70)
@semio-tech/layout-js:       at append (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🗃️plan/🟦️.ts:3:795)
@semio-tech/layout-js:       at testExportPlan (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🗃️plan/🧪️tests/🟦️.ts:7:709)
@semio-tech/layout-js:       at run (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🗃️plan/🧪️tests/🔮️oracle.mjs:3:934)
@semio-tech/layout-js:       at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🟦️.ts:46:103)
@semio-tech/layout-js: ✗ Original export plan custody, order and cancellation agree with independent oracle [65.72ms]
@semio-tech/layout-js: 
@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts:
@semio-tech/layout-js: ✓ demo-session > ships primary asset [1.88ms]
@semio-tech/layout-js: 
@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖼️canvas/🧵️paragraph-work/🧪️tests/🟦️.ts:
@semio-tech/layout-js: 1 | import fs from 'node:fs/promises';import fsSync from 'node:fs';import path from 'node:path';import {createRequire} from 'node:module';import {pathToFileURL,fileURLToPath} from 'node:url';import assert from 'node:assert/strict';
@semio-tech/layout-js: 2 | export async function run(root){const require=createRequire(path.join(root,'package.json')),Module=require('node:module'),ts=require('typescript'),sharp=require('sharp'),opentype=require('opentype.js');const old=require.extensions['.ts'],nativeTypescript=typeof Bun!=='undefined';if(!nativeTypescript)require.extensions['.ts']=(mod,file)=>mod._compile(ts.transpileModule(fsSync.readFileSync(file,'utf8').replaceAll('import.meta.url',JSON.stringify(pathToFileURL(file).href)),{fileName:file,compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.CommonJS,esModuleInterop:true}}).outputText,file);
@semio-tech/layout-js: ✓ testLayoutDrawingGraph [100.72ms]
@semio-tech/layout-js: ✓ testPaidDrawingGraph [0.58ms]
@semio-tech/layout-js: 1 | /** ⏱️ A defining owner commits actual bounded work through one expiring observation loan. */
@semio-tech/layout-js: 2 | export interface RetainedWorkTurn{commitActualWork(units?:number):void;actualWork():number}
@semio-tech/layout-js: 3 | const activeTurns=new WeakSet<RetainedWorkTurn>();
@semio-tech/layout-js: 4 | /** 🔐️ Only a defining counter's current issued observation loan may receive actual work. */
@semio-tech/layout-js: 5 | export function assertRetainedWorkTurn(turn:RetainedWorkTurn):void{if(!activeTurns.has(turn))throw Error("Retained work turn loan is unissued or expired");turn.actualWork();}
@semio-tech/layout-js:                                                                                                        ^
@semio-tech/layout-js: error: Retained work turn loan is unissued or expired
@semio-tech/layout-js:       at assertRetainedWorkTurn (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/⏱️turn/🟦️.ts:5:100)
@semio-tech/layout-js:       at slot (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts:70:70)
@semio-tech/layout-js:       at append (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🗃️plan/🟦️.ts:3:795)
@semio-tech/layout-js:       at testExportPlan (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🗃️plan/🧪️tests/🟦️.ts:7:709)
@semio-tech/layout-js:       at run (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🗃️plan/🧪️tests/🔮️oracle.mjs:3:934)
@semio-tech/layout-js:       at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/behavior.test.ts:76:38)
@semio-tech/layout-js: ✗ testExportPlan [21.32ms]
@semio-tech/layout-js: ✓ independent JSON Patch preserves authored operation objects [0.12ms]
@semio-tech/layout-js: 6 tests failed:
@semio-tech/layout-js: ✗ shared story flow and address cache [265.01ms]
@semio-tech/layout-js: ✗ headless font resources agree with independent coverage [121.97ms]
@semio-tech/layout-js: ✗ Original export plan custody, order and cancellation agree with independent oracle [65.72ms]
@semio-tech/layout-js: ✗ interactive outline publication, styles and original cancellation [133.26ms]
@semio-tech/layout-js: ✗ runLayoutDrawingExportTests [50.49ms]
```
