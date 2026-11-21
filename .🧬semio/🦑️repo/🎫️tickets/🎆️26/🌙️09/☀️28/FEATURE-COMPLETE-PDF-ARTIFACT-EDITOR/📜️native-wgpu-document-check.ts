/** 🔎️ Checks the installed native Program Bridge against the production WGPU dependency cohort. */
import {existsSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, extname, join, resolve} from "node:path";

const root = process.cwd();
const out = join(import.meta.dir,"🗑️generated","native-wgpu-save");
const target = resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️program-bridge/📦️packages/🦀️rust");
const manifest = Bun.TOML.parse(readFileSync(join(target,"Cargo.toml"),"utf8")) as {
  package:{name:string;edition:string};lib:{path:string};
  dependencies:Record<string,{package?:string}>;
  target:Record<string,{dependencies:Record<string,{package?:string}>}>;
};
type Crate = {path:string;features:string[]};
const crates = new Map<string,Crate[]>();
const directories = new Set<string>();
for (const line of readFileSync(join(out,"wgpu-host-clipboard-admission.log"),"utf8").split("\n")) {
  if (!line.startsWith('{"reason":"compiler-artifact"')) continue;
  const row = JSON.parse(line);
  const name = row.package_id.split("#").at(-1).split("@")[0];
  const paths: string[] = row.filenames.filter((p:string) => [".rmeta",".rlib",".dylib",".so",".dll"].includes(extname(p)) && existsSync(p));
  for (const path of paths) directories.add(dirname(path));
  if (paths.length) {
    const candidates = crates.get(name) ?? [];
    candidates.push({path:paths.find(path=>extname(path)===".rmeta") ?? paths[0],features:row.features});
    crates.set(name,candidates);
  }
}
const owner = crates.get(manifest.package.name)!.at(-1)!;
const identities = new Map<string,string>(JSON.parse(readFileSync(owner.path+".semio-deps","utf8")).dependencies);
const native = manifest.target['cfg(not(target_arch = "wasm32"))'].dependencies;
const dependencies = {...manifest.dependencies,...native};
const args = [
  "--crate-name",manifest.package.name.replaceAll("-","_"),"--edition",manifest.package.edition,
  "--crate-type","rlib","--emit=metadata","-o",join(out,"native-picker-current-bridge.rmeta"),
  resolve(target,manifest.lib.path),
  ...owner.features.flatMap(name=>["--cfg",'feature="'+name+'"']),
  ...[...directories].sort().map(path=>"-Ldependency="+path),
];
for (const [alias, declaration] of Object.entries(dependencies)) {
  const candidates = crates.get(declaration.package ?? alias) ?? [];
  const dependency = candidates.findLast(({path})=>{
    const name = path.split(/[\\/]/u).at(-1)!.replace(/^lib/u,"").replace(/\.[^.]+$/u,"");
    const hash = identities.get(name);
    return hash !== undefined && (!existsSync(path+".semio-deps") || JSON.parse(readFileSync(path+".semio-deps","utf8")).svh === hash);
  });
  if (!dependency) throw Error("Missing the exact production crate "+alias);
  args.push("--extern",alias.replaceAll("-","_")+"="+dependency.path);
}
const rsp = join(out,"native-picker-current-bridge.args");
writeFileSync(rsp,args.join("\n")+"\n");
const child = Bun.spawn(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","@"+rsp],{
  env: {...process.env,CARGO_MANIFEST_DIR:target,CARGO_PKG_NAME:manifest.package.name,CARGO_PKG_VERSION:"0.1.0"},
  stdout:"inherit",stderr:"inherit",
});
process.exitCode = await child.exited;
