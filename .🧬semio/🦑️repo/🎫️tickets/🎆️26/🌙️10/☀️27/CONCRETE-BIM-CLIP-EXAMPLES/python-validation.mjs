/** 🧪 Temporary pure-model Python verification through a JavaScript API; native rendering remains a separate gate. */
import {mkdir, readFile, writeFile} from "node:fs/promises";
import {dirname, join} from "node:path";
import {fileURLToPath} from "node:url";
import {createHash} from "node:crypto";
import {Worker} from "node:worker_threads";

const ticket = dirname(fileURLToPath(import.meta.url));
const directory = join(ticket, "🗑️generated", "pyodide-029");
const base = "https://cdn.jsdelivr.net/pyodide/v0.29.0/full/";
const runtimeFiles = ["pyodide.mjs", "pyodide.asm.js", "pyodide.asm.wasm", "python_stdlib.zip", "pyodide-lock.json"];

/** 📥 Read a test dependency from the ticket cache or fetch the pinned release, checking package integrity. */
async function dependency(name, digest) {
  const path = join(directory, name);
  if (path.length > 256) throw new Error("Validation dependency path exceeds 256 characters");
  let data;
  try { data = await readFile(path); } catch {
    const response = await fetch(base + name);
    if (!response.ok) throw new Error(`${name}: HTTP ${response.status}`);
    data = Buffer.from(await response.arrayBuffer());
    await writeFile(path, data);
  }
  if (digest && createHash("sha256").update(data).digest("hex") !== digest) throw new Error(`${name}: integrity mismatch`);
  return data;
}

/** 🐍 Open an isolated test interpreter with NumPy, SciPy and Shapely; no product dependency or shell process is added. */
export async function openValidator(root) {
  await mkdir(directory, {recursive: true});
  await Promise.all(runtimeFiles.map(name => dependency(name)));
  const lock = JSON.parse(await readFile(join(directory, "pyodide-lock.json"), "utf8"));
  const packages = new Set();
  function include(name) {
    name = name.toLowerCase().replace(/[-_.]+/g, "-");
    if (packages.has(name)) return;
    packages.add(name);
    lock.packages[name].depends.forEach(include);
  }
  ["numpy", "scipy", "shapely", "jsonschema"].forEach(include);
  await Promise.all([...packages].map(name => dependency(lock.packages[name].file_name, lock.packages[name].sha256)));
  const worker = new Worker(`
    const {parentPort, workerData} = require("node:worker_threads");
    const {pathToFileURL} = require("node:url");
    (async () => {
      const {loadPyodide} = await import(pathToFileURL(workerData.directory + "/pyodide.mjs"));
      const output = [];
      const python = await loadPyodide({indexURL: workerData.directory + "/", packageCacheDir: workerData.directory,
        stdout: value => output.push(value), stderr: value => output.push(value)});
      python.FS.mkdir("/repo");
      python.FS.mount(python.FS.filesystems.NODEFS, {root: workerData.root}, "/repo");
      python.runPython("import sys; sys.dont_write_bytecode=True; sys.path.insert(0, '/repo')");
      await python.loadPackage(["numpy", "scipy", "shapely", "jsonschema"]);
      parentPort.postMessage({ready: true});
      parentPort.on("message", async ({id, code}) => {
        output.length = 0;
        try {
          const value = await python.runPythonAsync(code);
          parentPort.postMessage({id, result: value?.toJs ? value.toJs() : value, output});
        } catch (error) { parentPort.postMessage({id, error: String(error), output}); }
      });
    })().catch(error => parentPort.postMessage({error: String(error)}));
  `, {eval: true, workerData: {directory, root}});
  await new Promise((resolve, reject) => {
    worker.once("error", reject);
    worker.once("message", message => message.ready ? resolve() : reject(new Error(message.error)));
  });
  let sequence = 0;
  return {
    run(code, timeout = 55000) {
      return new Promise((resolve, reject) => {
        const id = ++sequence;
        const timer = setTimeout(() => {worker.off("message", done); reject(new Error("Validation observation timed out"));}, timeout);
        function done(message) {
          if (message.id !== id) return;
          clearTimeout(timer);
          worker.off("message", done);
          resolve(message);
        }
        worker.on("message", done);
        worker.postMessage({id, code});
      });
    },
    dispose: () => worker.terminate()
  };
}
