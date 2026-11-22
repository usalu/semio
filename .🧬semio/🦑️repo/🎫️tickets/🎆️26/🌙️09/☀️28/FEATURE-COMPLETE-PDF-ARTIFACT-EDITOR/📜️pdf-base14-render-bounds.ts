/** 📐 Confirms the Base 14 paint candidate complies with source closure limits on every platform. */
import assert from "node:assert/strict";
import { dirname, join, resolve } from "node:path";

const { readRustSourceClosureV1 } = await import(resolve("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const { CRATE_HARD_BUDGET } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/💰️budget/🟦️.ts"));
const base = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base";
const symbolic=process.argv.includes("--symbolic");
const candidate: [string,string,string][] = await Bun.file(join(import.meta.dir,"🗑️generated","pdf-base14",symbolic ? "symbolic-code-overlay.json" : "font-code-overlay.json")).json();
const overlay = new Map<string,string>();
for (const [path, before, after] of candidate) {
  assert.equal(new Bun.CryptoHasher("sha256").update(await Bun.file(path).bytes()).digest("hex"),before,path);
  overlay.set(resolve(path),after);
}
const projects = [
  "🔨️modules/🔤️fonts/📦️packages/🦀️rust",
  "✏️editor/🖼️page/📦️packages/🦀️rust",
  "✏️editor/🖼️page/🧪️tests/🔬️unit/📦️packages/🦀️rust",
];
const dir = join(import.meta.dir,"🗑️generated","pdf-base14");
const packages: [string,string|null,string][] = await Bun.file(join(dir,"font-package-overlay.json")).json();
const assets: {path:string;sha256:string;base64:string}[] = await Bun.file(join(dir,"font-asset-overlay.json")).json();
const files = new Map<string,Uint8Array>();
for (const [path,body] of overlay) files.set(path,Buffer.from(body));
for (const [path,before,body] of packages) {
  if (before !== null) assert.equal(new Bun.CryptoHasher("sha256").update(await Bun.file(path).bytes()).digest("hex"),before,path);
  else assert(!await Bun.file(path).exists(),path);
  files.set(resolve(path),Buffer.from(body));
}
for (const {path,sha256,base64} of assets) {
  const bytes = Buffer.from(base64,"base64");
  assert.equal(new Bun.CryptoHasher("sha256").update(bytes).digest("hex"),sha256,path);
  files.set(resolve(path),bytes);
}
const read = async (path:string) => files.get(path) ?? (await Bun.file(path).exists() ? await Bun.file(path).bytes() : null);
const authored = async (path:string) => await Bun.file(path).exists() ? await Bun.file(path).bytes() : null;
const fund = (input: readonly {bytes:number;activeLines:number}[]) => [input.length,input.reduce((n,s)=>n+s.activeLines,0),input.reduce((n,s)=>n+s.bytes,0)];
for (const [path,before] of packages) if (before === null && path.endsWith("/Cargo.toml")) {
  projects.push(dirname(path).slice(base.length+1));
}
if (symbolic) projects.push("🔨️modules/⬇️lift/📦️packages/🦀️rust");
for (const project of projects) {
  const path = resolve(base,project,"Cargo.toml");
  const owner = dirname(path);
  const manifest = Bun.TOML.parse(new TextDecoder().decode(await read(path) ?? undefined)) as {
    package: { name: string; metadata?: { semio?: { "rust-source-boundary"?: {
      maxSourceFiles?:number; maxActiveLines?:number; maxSourceBytes?:number;
    } } } }; lib: { path: string }; features?: Record<string,string[]>;
  };
  const source = resolve(owner,manifest.lib.path);
  const local=manifest.package.metadata?.semio?.["rust-source-boundary"];
  const ceiling=[
    Math.min(CRATE_HARD_BUDGET.files,local?.maxSourceFiles??Infinity),
    Math.min(CRATE_HARD_BUDGET.lines,local?.maxActiveLines??Infinity),
    Math.min(CRATE_HARD_BUDGET.bytes,local?.maxSourceBytes??Infinity),
  ];
  const features=Object.keys(manifest.features??{}).filter(feature=>feature!=="default");
  for (const [os,arch,family] of [["linux","x86_64","unix"],["macos","aarch64","unix"],["windows","x86_64","windows"],["wasi","wasm32","wasm"]]) {
    const platform=[family,`target_os="${os}"`,`target_arch="${arch}"`,`target_family="${family}"`];
    for (const enabled of features.length ? [[],features] : [[]]) for (const test of [false,true]) {
      const cfg=[...platform,...enabled.map(feature=>`feature="${feature}"`)];
      const selected = [...cfg,...(test?["test"]:[])];
      const counts=fund(await readRustSourceClosureV1([source],process.cwd(),selected,read,{manifestDirectory:owner,generated() {}}));
      const admitted=counts.every((count,index)=>count<=ceiling[index]!);
      console.log("[DEBUG]",os,manifest.package.name,test?"test":"lib",enabled.join(",")||"base",...counts,admitted?"admitted":"over budget");
      if (!admitted) {
        if (await authored(source)) console.log("[DEBUG] Live baseline",...fund(await readRustSourceClosureV1([source],process.cwd(),selected,authored,{manifestDirectory:owner,generated() {}})));
        process.exitCode=1;
      }
    }
  }
}
