import {readFileSync,existsSync} from "node:fs";
import {resolve,join} from "node:path";
import {strict as assert} from "node:assert";

const root=resolve("✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999");
const parse=path=>Bun.TOML.parse(readFileSync(path,"utf8"));
const workspace=parse(join(root,"Cargo.toml"));
const lock=parse(join(root,"Cargo.lock"));
const entries=new Map(lock.package.map(entry=>[entry.name,entry]));
for(const name of ["semio-s-artifact-norm-en1999-diff-rows-details","semio-s-artifact-norm-en1999-scene-visual-laws"]){
 const path=workspace.workspace.dependencies[name].path;
 assert(workspace.workspace.members.includes(path));
 const directory=join(root,path),manifest=parse(join(directory,"Cargo.toml"));
 assert.equal(manifest.package.name,name);
 assert.equal(resolve(directory,manifest.package.workspace),root);
 assert(existsSync(resolve(directory,manifest.lib.path)));
 const deps={...manifest.dependencies,...manifest["dev-dependencies"]};
 for(const [dependency,spec] of Object.entries(deps)){
  assert(entries.has(dependency),`missing locked dependency ${dependency}`);
  if(spec.workspace){
   const inherited=workspace.workspace.dependencies[dependency];
   assert(inherited,`missing workspace dependency ${dependency}`);
   if(inherited.path)assert.equal(parse(resolve(root,inherited.path,"Cargo.toml")).package.name,dependency);
  }
  if(spec.path){
   const target=parse(resolve(directory,spec.path,"Cargo.toml"));
   assert.equal(target.package.name,dependency);
  }
 }
 assert.deepEqual([...entries.get(name).dependencies].sort(),Object.keys(deps).sort());
 assert.equal(entries.get(name).version,workspace.workspace.package.version);
 console.log(name,Object.keys(deps).length,"dependencies verified");
}
const parent=entries.get("semio-s-artifact-norm-en1999-diff");
assert(parent.dependencies.includes("semio-s-artifact-norm-en1999-diff-rows-details"));
const diff=parse(join(root,"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml"));
assert(diff.features["test-support"].includes("semio-s-artifact-norm-en1999-diff-rows-details/test-support"));
assert(diff["dev-dependencies"]["semio-s-artifact-norm-en1999-diff-rows-details"].features.includes("test-support"));
