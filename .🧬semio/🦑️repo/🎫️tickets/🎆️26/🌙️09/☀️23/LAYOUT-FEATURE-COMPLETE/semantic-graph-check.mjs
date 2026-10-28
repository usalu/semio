/** 🧪️ Reproducible ticket evidence; invokes no native build or stale repository runner. */
import { readFile, readdir } from 'node:fs/promises';
import { resolve, dirname } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { createRequire } from 'node:module';
const ticket = dirname(fileURLToPath(import.meta.url));
const root = resolve(ticket, '../../../../../../..');
const sub = resolve(root, '✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any');
const schema = resolve(sub, '🧬️schema');
const mutations = resolve(schema, '🧬️mutations');
const graph = resolve(mutations, '🧪️tests/🕸️graph');
const require = createRequire(resolve(root, 'package.json'));
const { verifySemanticGraph } = await import(pathToFileURL(resolve(graph, 'oracle.mjs')).href);
const { verifyGraphValidation, graphValid } = await import(pathToFileURL(resolve(mutations, '🧪️tests/✅️validation/oracle.mjs')).href);
const json = async file => JSON.parse(await readFile(file, 'utf8'));
const semantic = verifySemanticGraph(await json(resolve(sub, '🧪️tests/🧫️fixtures/🕸️graph/🔣️.json')));
const validation = verifyGraphValidation(await json(resolve(sub, '🧪️tests/🧫️fixtures/✅️validation/🔣️.json')));
const fixtures = resolve(sub, '🧪️tests/🧫️fixtures/🧬️mutations');
let goldenSnapshots = 0;
for(const relative of await readdir(fixtures, {recursive:true})) {
  if(!relative.includes('📸️snapshot') || !relative.endsWith('🔣️.json')) continue;
  if(!graphValid(await json(resolve(fixtures, relative)))) throw new Error('Invalid golden snapshot: '+relative);
  goldenSnapshots++;
}
const treeSitterModule = require.resolve('web-tree-sitter');
delete require.cache[treeSitterModule];
const Parser = require(treeSitterModule);
await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(resolve(root, 'node_modules/.bun/tree-sitter-wasms@0.1.13/node_modules/tree-sitter-wasms/out/tree-sitter-rust.wasm')));
const changedRust = ['🔢change-frame-columns','🏛️update-page-columns','🗄️set-page-parent','✋️drag-frames','🔃️rotate-frames','🗜️scale-frames','🌱create-page','🗑️delete-page','✍️edit-story','➕create-frame','➖delete-frame','📑️update-text-frame','📖create-story','🖇️create-link','✒️set-story-runs'].map(name=>resolve(mutations,name,'🦀️.rs'));
changedRust.push(resolve(schema,'🔺️diff/🧩️patchable-frames/🦀️.rs'), resolve(schema,'✅️validation/🦀️.rs'), resolve(schema,'✂️story-splice/🦀️.rs'), resolve(schema,'🦀️.rs'), resolve(mutations,'🦀️.rs'), resolve(graph,'🦀️.rs'), resolve(mutations,'🧪️tests/🔬️unit/🦀️.rs'));
const ieeeRoot=resolve(root,'🧰️framework/🔨️modules/🌱️value');
const ieeeOracle=await import(pathToFileURL(resolve(ieeeRoot,'🔢️ieee754/🧪️tests/oracle.mjs')).href);
const ieee=ieeeOracle.verifyIeeeIdentity(await json(resolve(ieeeRoot,'🔢️ieee754/🧪️tests/🧫️fixtures/🔣️.json')));
const ieeeLeaves=['↔️change-page-width','↕️change-page-height','✋️drag-frames','🎨change-frame-fill','🎨️update-character-style','🏛️update-page-columns','📏resize-frame','📐update-grid','📐update-page-margins','📑️update-text-frame','📖️update-parent-page','📝️update-paragraph-style','🔃️rotate-frames','🔄️rotate-frame','🕹️move-frame','🖊️change-frame-stroke','🗜️scale-frames'].map(name=>resolve(mutations,name,'🦀️.rs'));
changedRust.push(...ieeeLeaves,resolve(schema,'🟰️equality/🦀️.rs'),resolve(schema,'🔺️diff/🩹️row-patches/🦀️.rs'),resolve(schema,'🔺️diff/🧱️grid-text-and-guide-deltas/🦀️.rs'),...['🌳️intrinsic/🦀️.rs','🦀️.rs','📋️list/🟰️equal/🦀️.rs','📋️list/🟰️equal/🧪️tests/🦀️.rs','🔢️ieee754/🧪️tests/🦀️.rs'].map(name=>resolve(ieeeRoot,name)));
const uniqueRust=[...new Set(changedRust)];
for(const file of uniqueRust) {
  const tree=parser.parse(await readFile(file,'utf8'));
  if(tree.rootNode.hasError()) throw new Error('Rust grammar error: '+file);
  tree.delete();
}
parser.delete();
export const evidence={semantic,validation,ieee,goldenSnapshots,rustGrammarFiles:uniqueRust.length,maxChangedRustPath:Math.max(...uniqueRust.map(file=>file.length)),nativeExecution:'not run'};
console.log(evidence);
