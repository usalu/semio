Actual selected behavior execution failure details from stripped log, unsealed diagnostic.

@semio-tech/layout-js: bun test v1.4.2 (744846f84)
@semio-tech/layout-js: 
@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📄️paragraph/🧪️tests/🟦️.ts:
@semio-tech/layout-js: 1 | import fs from 'node:fs/promises';import fsSync from 'node:fs';import path from 'node:path';import {createRequire} from 'node:module';import {pathToFileURL,fileURLToPath} from 'node:url';
@semio-tech/layout-js: 2 | export async function run(root){const require=createRequire(path.join(root,'package.json')),ts=require('typescript'),old=require.extensions['.ts'],native=typeof Bun!=='undefined';if(!native)require.extensions['.ts']=(mod,file)=>mod._compile(ts.transpileModule(fsSync.readFileSync(file,'utf8').replaceAll('import.meta.url',JSON.stringify(pathToFileURL(file).href)),{fileName:file,compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.CommonJS,esModuleInterop:true,resolveJsonModule:true}}).outputText,file);try{const owner=path.join(root,'🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font'),here=path.dirname(fileURLToPath(import.meta.url));for(const file of Object.keys(require.cache))if(file.startsWith(owner)||file.startsWith(path.resolve(here,'../..')))delete require.cache[file];const fixture=JSON.parse(await fs.readFile(path.join(here,'../🧫️fixtures/🧵️flow.json'),'utf8')),fonts=path.join(root,'🧰️framework/🔨️modules/🖼️assets/🔤️fonts'),[source,fallback]=await | ... truncated 
@semio-tech/layout-js: SyntaxError: export 'RetainedWorkGrant' not found in '../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts'
@semio-tech/layout-js:       at run (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📄️paragraph/🧪️tests/🧵️flow/🔮️oracle.mjs:2:1459)
@semio-tech/layout-js:       at async <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📄️paragraph/🧪️tests/🟦️.ts:4:73)
@semio-tech/layout-js: ✗ shared story flow and address cache [258.18ms]
@semio-tech/layout-js: 
@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧪️tests/🟦️.ts:
@semio-tech/layout-js: 1 | /** 🔮️ Test-only first-party resource admission and glyph coverage are checked by Node crypto and independent OpenType. */
@semio-tech/layout-js: 2 | import fs from 'node:fs';import path from 'node:path';import {createRequire} from 'node:module';import {fileURLToPath} from 'node:url';import crypto from 'node:crypto';import assert from 'node:assert/strict';
@semio-tech/layout-js: 3 | export function repositoryRoot(){let at=path.dirname(fileURLToPath(import.meta.url));for(;;){if(fs.existsSync(path.join(at,'package.json'))&&fs.existsSync(path.join(at,'🧰️framework')))return at;const parent=path.dirname(at);if(parent===at)throw Error('Repository root unavailable');at=parent;}}
@semio-tech/layout-js: 4 | export function run(root){const require=createRequire(path.join(root,'package.json')),ts=require('typescript'),opentype=require('opentype.js'),Ajv=require('ajv/dist/2020'),owner=path.join(root,'🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font'),layout=path.join(root,'✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any'),catalogRoot=path.join(layout,'✏️editor/⚙️engine/🎬️scene/🔤️text-fonts'),old=require.extensions['.ts'],nativeTypescript=typeof Bun!=='undefined';if(!nativeTypescript)require.extensions['.ts']=(mod,file)=>mod._compile(ts.transpileModule(fs.readFileSync(file,'utf8'),{fileName:file,compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.CommonJS,esModuleInterop:true}}).outputText,file);try{const {layoutFontCount,layoutFontSource}=require(path.join(catalogRoot,'🟦️.ts')),{FontFaceAdmissionJob}=require(path.join(owner,'🟦️.ts')),{FontWorkLedger}=require(path.join(owner,'🎟️work | ... truncated 

@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧪️tests/🟦️.ts:
@semio-tech/layout-js: 1 | /** 🔮️ Test-only first-party resource admission and glyph coverage are checked by Node crypto and independent OpenType. */
@semio-tech/layout-js: 2 | import fs from 'node:fs';import path from 'node:path';import {createRequire} from 'node:module';import {fileURLToPath} from 'node:url';import crypto from 'node:crypto';import assert from 'node:assert/strict';
@semio-tech/layout-js: 3 | export function repositoryRoot(){let at=path.dirname(fileURLToPath(import.meta.url));for(;;){if(fs.existsSync(path.join(at,'package.json'))&&fs.existsSync(path.join(at,'🧰️framework')))return at;const parent=path.dirname(at);if(parent===at)throw Error('Repository root unavailable');at=parent;}}
@semio-tech/layout-js: 4 | export function run(root){const require=createRequire(path.join(root,'package.json')),ts=require('typescript'),opentype=require('opentype.js'),Ajv=require('ajv/dist/2020'),owner=path.join(root,'🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font'),layout=path.join(root,'✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any'),catalogRoot=path.join(layout,'✏️editor/⚙️engine/🎬️scene/🔤️text-fonts'),old=require.extensions['.ts'],nativeTypescript=typeof Bun!=='undefined';if(!nativeTypescript)require.extensions['.ts']=(mod,file)=>mod._compile(ts.transpileModule(fs.readFileSync(file,'utf8'),{fileName:file,compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.CommonJS,esModuleInterop:true}}).outputText,file);try{const {layoutFontCount,layoutFontSource}=require(path.join(catalogRoot,'🟦️.ts')),{FontFaceAdmissionJob}=require(path.join(owner,'🟦️.ts')),{FontWorkLedger}=require(path.join(owner,'🎟️work | ... truncated 
@semio-tech/layout-js: SyntaxError: export 'RetainedWorkGrant' not found in '../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts'
@semio-tech/layout-js:       at run (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧪️tests/🔮️oracle.mjs:4:909)
@semio-tech/layout-js:       at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧪️tests/🟦️.ts:4:82)
@semio-tech/layout-js: ✗ headless font resources agree with independent coverage [98.02ms]
@semio-tech/layout-js: 
@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🖼️frame-planning/🧪️tests/🟦️.ts:
@semio-tech/layout-js: ✓ bounded frame paint addresses and shared metadata collisions [972.82ms]
@semio-tech/layout-js: 
@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🎨️fidelity/🟦️.ts:
@semio-tech/layout-js: ✓ portable vector probes agree with an independent SVG renderer and PNG decoder [9.38ms]

@semio-tech/layout-js: ✓ Layout source adoption refuses wrong-snapshot-close [3.29ms]
@semio-tech/layout-js: ✓ Layout source adoption refuses comment-only and duplicated physical owners [6.07ms]
@semio-tech/layout-js: 1 | import fs from 'node:fs';import path from 'node:path';import {fileURLToPath} from 'node:url';import {createRequire} from 'node:module';
@semio-tech/layout-js: 2 | /** 🔬️ Original retained plan order agrees with independent stable source projection and schema validation. */
@semio-tech/layout-js: 3 | export function run(root){const require=createRequire(path.join(root,'package.json')),ts=require('typescript'),old=require.extensions['.ts'],native=typeof Bun!=='undefined',here=path.dirname(fileURLToPath(import.meta.url));if(!native)require.extensions['.ts']=(mod,file)=>mod._compile(ts.transpileModule(fs.readFileSync(file,'utf8'),{fileName:file,compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.CommonJS,esModuleInterop:true,resolveJsonModule:true}}).outputText,file);try{for(const key of Object.keys(require.cache))if(key.startsWith(path.resolve(here,'..')))delete require.cache[key];const validate=new(require('ajv/dist/2020'))({strict:true}).compile(JSON.parse(fs.readFileSync(path.resolve(here,'../🔣️.json'),'utf8'))),fixture=JSON.parse(fs.readFileSync(path.join(here,'🧫️fixtures/🔣️.json'),'utf8'));if(!validate(fixture))throw Error('plan funding fixture schema');return require(path.join(here,'🟦️.ts')).testExportPlan((items,layers)=>require('lodash/sortBy')(items,item=>layers. | ... truncated 
@semio-tech/layout-js: SyntaxError: export 'RetainedWorkGrant' not found in '../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts'
@semio-tech/layout-js:       at run (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🗃️plan/🧪️tests/🔮️oracle.mjs:3:900)
@semio-tech/layout-js:       at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🟦️.ts:46:103)
@semio-tech/layout-js: ✗ Original export plan custody, order and cancellation agree with independent oracle [17.09ms]
@semio-tech/layout-js: 
@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts:
@semio-tech/layout-js: ✓ demo-session > ships primary asset [0.25ms]
@semio-tech/layout-js: 
@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖼️canvas/🧵️paragraph-work/🧪️tests/🟦️.ts:
@semio-tech/layout-js: 1 | /** 🔮️ Test-only first-party resource admission and glyph coverage are checked by Node crypto and independent OpenType. */

@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖼️canvas/🧵️paragraph-work/🧪️tests/🟦️.ts:
@semio-tech/layout-js: 1 | /** 🔮️ Test-only first-party resource admission and glyph coverage are checked by Node crypto and independent OpenType. */
@semio-tech/layout-js: 2 | import fs from 'node:fs';import path from 'node:path';import {createRequire} from 'node:module';import {fileURLToPath} from 'node:url';import crypto from 'node:crypto';import assert from 'node:assert/strict';
@semio-tech/layout-js: 3 | export function repositoryRoot(){let at=path.dirname(fileURLToPath(import.meta.url));for(;;){if(fs.existsSync(path.join(at,'package.json'))&&fs.existsSync(path.join(at,'🧰️framework')))return at;const parent=path.dirname(at);if(parent===at)throw Error('Repository root unavailable');at=parent;}}
@semio-tech/layout-js: 4 | export function run(root){const require=createRequire(path.join(root,'package.json')),ts=require('typescript'),opentype=require('opentype.js'),Ajv=require('ajv/dist/2020'),owner=path.join(root,'🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font'),layout=path.join(root,'✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any'),catalogRoot=path.join(layout,'✏️editor/⚙️engine/🎬️scene/🔤️text-fonts'),old=require.extensions['.ts'],nativeTypescript=typeof Bun!=='undefined';if(!nativeTypescript)require.extensions['.ts']=(mod,file)=>mod._compile(ts.transpileModule(fs.readFileSync(file,'utf8'),{fileName:file,compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.CommonJS,esModuleInterop:true}}).outputText,file);try{const {layoutFontCount,layoutFontSource}=require(path.join(catalogRoot,'🟦️.ts')),{FontFaceAdmissionJob}=require(path.join(owner,'🟦️.ts')),{FontWorkLedger}=require(path.join(owner,'🎟️work | ... truncated 
@semio-tech/layout-js: SyntaxError: export 'RetainedWorkGrant' not found in '../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts'
@semio-tech/layout-js:       at unknown:1:1
@semio-tech/layout-js:       at run (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧪️tests/🔮️oracle.mjs:4:909)
@semio-tech/layout-js:       at /Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧪️tests/🟦️.ts:4:82
@semio-tech/layout-js: ✗ interactive outline publication, styles and original cancellation [43.44ms]
@semio-tech/layout-js: 
@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts:
@semio-tech/layout-js: ✓ demo > ships primary asset [1.29ms]
@semio-tech/layout-js: 
@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/👁️layer-order/🟦️.ts:

@semio-tech/layout-js: 
@semio-tech/layout-js: ✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/behavior.test.ts:
@semio-tech/layout-js: 
@semio-tech/layout-js: # Unhandled error between tests
@semio-tech/layout-js: -------------------------------
@semio-tech/layout-js: SyntaxError: export 'RetainedWorkGrant' not found in '../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts'
@semio-tech/layout-js: -------------------------------
@semio-tech/layout-js: 5 tests failed:
@semio-tech/layout-js: ✗ shared story flow and address cache [258.18ms]
@semio-tech/layout-js: ✗ headless font resources agree with independent coverage [98.02ms]
@semio-tech/layout-js: ✗ Original export plan custody, order and cancellation agree with independent oracle [17.09ms]
@semio-tech/layout-js: ✗ interactive outline publication, styles and original cancellation [43.44ms]
@semio-tech/layout-js:  35 pass
@semio-tech/layout-js:  5 fail
@semio-tech/layout-js:  1 error

Counts: ['graph: 0 of 887 projects inferred by 3 worker processes in 644 s (863 failed, 0 lost, 0 restarts)', '@semio-tech/layout-js: 5 tests failed:', '@semio-tech/layout-js:  35 pass', '@semio-tech/layout-js:  5 fail', '@semio-tech/layout-js:  298 expect() calls', '@semio-tech/layout-js: Ran 40 tests across 16 files. [5.40s]']
