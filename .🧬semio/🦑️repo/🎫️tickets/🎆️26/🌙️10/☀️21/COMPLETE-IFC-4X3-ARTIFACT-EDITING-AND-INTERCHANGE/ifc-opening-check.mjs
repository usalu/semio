/** 🧪️ Typecheck and execute the permanent IFC4 geometry source suite. */
import fs from "node:fs/promises";
import path from "node:path";
import {createRequire, stripTypeScriptTypes} from "node:module";
import {pathToFileURL} from "node:url";
export async function runOpeningProjection(root, ticket, work = 2_000_000) {
  const subset = path.join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any"), owner = path.join(subset, "🧬️schema/📐️geometry"), source = path.join(owner, "🟦️.ts"), test = path.join(owner, "🧪️tests/🔬️unit/🟦️.test.ts");
  const require = createRequire(path.join(root, "package.json")), ts = require("typescript");
  const config = ts.readConfigFile(path.join(root, "tsconfig.json"), ts.sys.readFile), options = ts.convertCompilerOptionsFromJson(config.config.compilerOptions, root).options;
  const diagnostics = ts.getPreEmitDiagnostics(ts.createProgram([source, test], options)).map(item => ({file: item.file?.fileName, message: ts.flattenDiagnosticMessageText(item.messageText, "\n")}));
  const output = path.join(ticket, "🗑️generated/bool-opening-" + Date.now());
  await fs.mkdir(output, {recursive: true});
  const cache = new Map();
  async function compile(sourcePath) {
    sourcePath = path.resolve(sourcePath);
    if (cache.has(sourcePath)) return cache.get(sourcePath);
    const target = path.join(output, "module" + cache.size + ".mjs"); cache.set(sourcePath, target);
    let source = stripTypeScriptTypes(await fs.readFile(sourcePath, "utf8"), {mode: "transform"});
    for (const match of [...source.matchAll(/from\s*["']([^"']+)["']/g)]) {
      let dependency = match[1];
      if (dependency === "@semio-tech/framework/value/ieee754") dependency = path.join(root, "🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts");
      else if (dependency.startsWith(".")) dependency = path.resolve(path.dirname(sourcePath), dependency);
      else continue;
      source = source.replace(match[0], "from " + JSON.stringify(pathToFileURL(await compile(dependency)).href));
    }
    source = source.replaceAll("import.meta.url", JSON.stringify(pathToFileURL(sourcePath).href));
    await fs.writeFile(target, source); return target;
  }
  const geometry=await import(pathToFileURL(await compile(source)).href),io=await import(pathToFileURL(await compile(path.join(subset,"🚪️io/🟦️.ts"))).href),snapshot=io.decodeIfc4(await fs.readFile(path.join(ticket,"official-slab-openings.ifc"),"utf8"));const started=performance.now(),scene=geometry.projectControlled(snapshot,{...geometry.DEFAULT_GEOMETRY_LIMITS,work});return {diagnostics,elapsedMs:performance.now()-started,status:scene.status,work:scene.work,issues:scene.issues.map(i=>({...i,entity:String(i.entity)})),meshes:scene.meshes.map(m=>({product:String(m.product),representation:String(m.representation),vertices:m.vertices.length,triangles:m.triangles.length,volume:m.triangles.reduce((s,ids)=>{const[a,b,c]=ids.map(i=>m.vertices[i]);return s+(a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6;},0)})),output};
}
