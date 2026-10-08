import{test,expect}from'bun:test';import{readFileSync}from'node:fs';import{resolve,dirname}from'node:path';
const root=resolve(import.meta.dir,'../../../../../../..');
const kernel='🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust';
const command='🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs';
const paths=JSON.parse(readFileSync(resolve(import.meta.dir,'📥️oct8-spr-mutation-testing-manifest-paths.json'),'utf8'));
test('actual kernel mutation fixture capability is requested only by authored dev dependencies',()=>{
 const own=Bun.TOML.parse(readFileSync(resolve(root,kernel,'Cargo.toml'),'utf8')) as any;
 expect(own.features['mutation-testing']).toEqual([]);expect(own.features.default).not.toContain('mutation-testing');
 for(const path of paths){const cargo=Bun.TOML.parse(readFileSync(resolve(root,path),'utf8')) as any;expect(cargo['dev-dependencies']['semio-framework-os-kernel'].features).toContain('mutation-testing');for(const table of [cargo.dependencies,cargo['build-dependencies'],...Object.values(cargo.target??{}).flatMap((target:any)=>[target.dependencies,target['build-dependencies']])])for(const value of Object.values(table??{}) as any[])expect(value?.features??[]).not.toContain('mutation-testing');}
 expect(paths.length).toBe(129);console.log('[DEBUG] actual129BunTOML dev requests; zero normal/build/default mutation-testing requests');
});
test('actual SPR production excludes both fixture filesystem census owners',async()=>{
 const {runtimeRustReferencesV1}=await import(resolve(root,'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🟦️.ts'));
 const source=readFileSync(resolve(root,command),'utf8');
 const context={read:()=>undefined,features:[]};
 const production=runtimeRustReferencesV1(source,command,context);
 expect(production.filter(row=>row.expression?.startsWith('Filesystem'))).toEqual([]);
 const testing=runtimeRustReferencesV1(source,command,{...context,features:['mutation-testing']});
 expect(testing.some(row=>row.expression==='Filesystem read_dir requires actual read input ownership')).toBe(true);
 expect(testing.some(row=>row.expression==='Filesystem read_to_string requires actual read input ownership')).toBe(true);
 const facade=readFileSync(resolve(root,'🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️.rs'),'utf8');
 expect(facade).toContain('#[cfg(any(test, feature = "mutation-testing"))]\npub use crate::os_spr::command::{mutation_fixture_ops, mutation_inverse_rows_failures};');
 console.log('[DEBUG] actual twoSPR filesystem census implementations absent from production and retained by explicit test capability');
});
test('actual Semio CLI caller-file intake belongs to its explicit native binary capability',async()=>{
 const {runtimeRustReferencesV1}=await import(resolve(root,'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🟦️.ts'));
 const path='🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs',source=readFileSync(resolve(root,path),'utf8');
 const references=(features:string[],cfg:string[])=>runtimeRustReferencesV1(source,path,{read:()=>undefined,features,cfg}).filter(row=>row.expression?.startsWith('Filesystem'));
 expect(references([],['target_arch="wasm32"'])).toEqual([]);
 expect(references([],['target_arch="x86_64"'])).toEqual([]);
 expect(references(['native-bin'],['target_arch="x86_64"']).length).toBe(1);
 console.log('[DEBUG] actual Semio file payload intake remains native-bin owned and absent from audited guest libraries');
});
