import fs from 'node:fs/promises';
import path from 'node:path';
import assert from 'node:assert/strict';
import {createRequire} from 'node:module';

export async function run(root) {
  const owner=path.join(root,'🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive');
  const source=await fs.readFile(path.join(owner,'🔁️mutations-expansion/🦀️.rs'),'utf8');
  const fixture=JSON.parse(await fs.readFile(path.join(owner,'🧪️tests/🧫️fixtures/❄️cold-dispatch/🔣️.json'),'utf8'));
  const require=createRequire(path.join(root,'package.json'));
  delete require.cache[require.resolve('web-tree-sitter')];
  const Parser=require('web-tree-sitter');
  await Parser.init();
  const parser=new Parser();
  parser.setLanguage(await Parser.Language.load(path.join(root,'node_modules/.bun/tree-sitter-wasms@0.1.13/node_modules/tree-sitter-wasms/out/tree-sitter-rust.wasm')));
  assert.equal((source.match(/impl #impl_generics .*::ColdMutation</g)||[]).length,1);
  assert.match(source,/reference\.as_ref\(\)\.map\(.*\)\.unwrap_or\(\(&snapshot_ty, &diff_ty\)\)/);
  assert.doesNotMatch(source,/if let Some\(\(reference_ty, _\)\)/);
  assert.match(source,/mutation_inverse_rows_failures::<#snapshot_ty, #reference_ty, #name>/);
  const template=source.split('let cold_reference = quote! {')[1].split('\n    };')[0];
  const methods=['diff','inverse','foreign_step_source'];
  const vectors=['diff_arms','inverse_arms','cold_foreign_source_arms'];
  const armTemplates=vectors.map(vector=>source.split(vector+'.push(quote! { ')[1].split(' });')[0]);
  const kind=source.split('let cold_kind = quote! { ')[1].split(' };')[0];
  const rows=[];
  for(const test of fixture.cases) {
    const reference=test.reference??test.snapshot;
    const delta=test.referenceDiff??test.diff;
    assert.equal(reference,test.expected.reference);
    assert.equal(delta,test.expected.referenceDiff);
    let output=template;
    for(let i=0;i<methods.length;i++) {
      const arms=test.variants.map(leaf=>armTemplates[i].replaceAll('#cold_kind',kind).replaceAll('#variant_ident',leaf.variant).replaceAll('#payload_ty',leaf.payload));
      output=output.replace('#(#'+vectors[i]+'),*',arms.join(','));
    }
    for(const [token,value] of Object.entries({impl_generics:'',ty_generics:'',where_clause:'',reference_diff_ty:delta,reference_ty:reference,snapshot_ty:test.snapshot,aggregate_ty:'Ops'})) output=output.replaceAll('#'+token,value);
    assert.doesNotMatch(output,/#/);
    const tree=parser.parse(output);
    assert.equal(tree.rootNode.hasError(),false,test.name);
    const impls=tree.rootNode.namedChildren.filter(node=>node.type==='impl_item');
    assert.equal(impls.length,test.expected.implementations);
    const functions=impls[0].descendantsOfType('function_item');
    assert.equal(functions.length,methods.length);
    for(let i=0;i<methods.length;i++) {
      const method=functions[i];
      assert.equal(method.childForFieldName('name').text,methods[i]);
      const matches=method.descendantsOfType('match_expression');
      assert.equal(matches.length,1);
      const arms=matches[0].descendantsOfType('match_arm');
      assert.equal(arms.length,test.variants.length);
      arms.forEach((arm,j)=>{
        assert.equal(arm.childForFieldName('pattern').text,'Self::'+test.variants[j].variant+'(payload)');
        const call=arm.descendantsOfType('call_expression')[0];
        assert.equal(call.childForFieldName('function').text.replaceAll(' ',''),'<'+test.variants[j].payload+'as::semio_framework_os_command::ColdMutationKind<'+reference+',Ops>>::'+methods[i]);
      });
    }
    assert.equal((output.match(/admit_inverse/g)||[]).length,test.expected.admissionCalls);
    assert.match(output,/MutationOutcome<Self::ReferenceDiff>/);
    rows.push({name:test.name,reference,delta,methods:methods.length,arms:test.variants.length*methods.length});
    tree.delete();
  }
  parser.delete();
  return {mode:'quote-template instantiation and independent tree-sitter AST; native macro execution unverified',cases:rows};
}
