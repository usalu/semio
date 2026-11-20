/** 🧪️ Exercises the committed oracle's framing through the pinned interpreter and wheel cache. */
import {resolve, dirname} from "node:path";
import {homedir} from "node:os";

const cache = resolve(homedir(), ".cache/uv/archive-v0");
const wheel = (await Array.fromAsync(new Bun.Glob("*/pypdfium2_raw/bindings.py").scan({cwd: cache})))[0];
if (wheel === undefined) throw Error("Cached PDFium wheel required");
const script = await Bun.file("✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🧵️write/🧪️tests/🔮️oracle/🐍️.py").text();
const program = "import sys\nsys.path.insert(0, " + JSON.stringify(resolve(cache, dirname(dirname(wheel)))) + ")\n" + script;
const generated = new URL("./🗑️generated/", import.meta.url);
const published = await Promise.all(Array.from({length: 25}, async (_, index) => Bun.file(new URL("pdfium-input-" + index + ".pdf", generated)).bytes()));
const input = Buffer.concat(published.flatMap(bytes => {
    const header = Buffer.alloc(4);
    header.writeUInt32BE(bytes.byteLength);
    return [header, bytes];
}));
const child = Bun.spawn(["repo", "tools", "exec", "--light", "python3", "--", "-I", "-c", program], {stdin: input, stdout: "pipe", stderr: "inherit"});
const [result, exit] = await Promise.all([new Response(child.stdout).arrayBuffer(), child.exited]);
if (exit !== 0 || result.byteLength !== published.length * 8) throw Error("PDFium oracle framing probe failed " + exit + " (" + result.byteLength + " bytes)");
const actual = new Uint8Array(result);
const samples = (await Bun.file(new URL("pdfium-ffi-pixels.log", generated)).text()).split("\n").filter(row => /^\d+ /.test(row)).flatMap(row => row.split(" ").slice(1).map(Number));
if (samples.length !== actual.length || samples.some((value, at) => value !== actual[at])) throw Error("Python output differs from PDFium C API");
console.log("Verified", published.length, "PDFium frame replies");
