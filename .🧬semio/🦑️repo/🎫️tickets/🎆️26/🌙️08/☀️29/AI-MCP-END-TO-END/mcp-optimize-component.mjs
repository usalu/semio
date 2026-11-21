import { readFile, writeFile } from "node:fs/promises";
import { pathToFileURL } from "node:url";

const [jcoEntry, input, output, wasmOptBin] = process.argv.slice(2);
if (!jcoEntry || !input || !output || !wasmOptBin) {
  throw Error("Pinned JCO, source component, result path, and Binaryen adapter are required");
}
const { optimizeComponent } = await import(new URL("./cmd/opt.js", pathToFileURL(jcoEntry)));
const optArgs = ["-O2", "--enable-bulk-memory", "--enable-nontrapping-float-to-int", "--enable-sign-ext", "--strip-debug"];
const { component } = await optimizeComponent(await readFile(input), { wasmOptBin, optArgs, noVerify: true });
await writeFile(output, component, { flag: "wx" });
console.log(`[DEBUG] Optimized candidate has ${component.byteLength} bytes and requires native qualification`);
