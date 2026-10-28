import {existsSync,mkdirSync,realpathSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import scratch from "./🔣️.json" with {type:"json"};

const canonical=(path:string):string=>{try{return realpathSync(path);}catch{return resolve(path);}};

/** 🔭️ Names the first file at or above `directory` through which Cargo or rustup would reach a build started below it: an enclosing workspace manifest, a configuration other than Cargo's home one (Cargo reads that one wherever it runs), or a toolchain file; `undefined` when none does.
 * @see https://doc.rust-lang.org/cargo/reference/config.html#hierarchical-structure
 * @see https://doc.rust-lang.org/cargo/reference/workspaces.html#root-package
 * @see https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file */
export function cargoAncestorFileV1(directory:string,environment:NodeJS.ProcessEnv=process.env):string|undefined{
 const home=canonical(environment.CARGO_HOME||join(homedir(),".cargo"));
 for(let current=canonical(directory);;current=dirname(current)){
  for(const row of scratch.ancestorFiles){const path=join(current,row.path);if(existsSync(path)&&!(row.kind==="configuration"&&canonical(dirname(path))===home))return path;}
  if(dirname(current)===current)return undefined;
 }
}

/** 🏖️ Returns the directory a test creates its scratch Cargo workspaces in: the first candidate (by default the test artifact directory, then the system temporary directory) that nothing above reaches Cargo from, so a run whose artifact directory lies inside a checkout never builds with that checkout's wrapper, compiler, target directory, workspace or toolchain; refuses when every candidate is reached, naming the files. */
export function cargoScratchBaseV1(candidates:readonly (string|undefined)[]=[process.env.SEMIO_TEST_ARTIFACT_DIR,tmpdir()],environment:NodeJS.ProcessEnv=process.env):string{
 const reached:string[]=[];
 for(const candidate of candidates){if(!candidate?.trim())continue;const directory=resolve(candidate);mkdirSync(directory,{recursive:true});const file=cargoAncestorFileV1(directory,environment);if(file===undefined)return directory;reached.push(file);}
 throw Error("Scratch Cargo workspace outside every enclosing Cargo configuration, workspace and toolchain file required; reached through: "+reached.join(", "));
}
