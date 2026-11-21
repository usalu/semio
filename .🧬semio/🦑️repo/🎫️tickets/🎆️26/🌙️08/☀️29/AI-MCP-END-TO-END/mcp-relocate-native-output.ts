import { lstatSync, mkdirSync, realpathSync, symlinkSync } from "node:fs";
import { join, resolve } from "node:path";

const generated = resolve(import.meta.dir, "🗑️generated");
const build = join(generated, "native-host", "debug", "build");
const spill = join(generated, "host-spill");
const realBuild = realpathSync(build);
if (!realBuild.endsWith("/.ticket-AI-MCP-END-TO-END/native-host/debug/build")) throw Error("Exact ticket host clone required");
mkdirSync(spill, { recursive: true });
for (const name of process.argv.slice(2)) {
  if (!/^semio-[a-z0-9-]+$/u.test(name)) throw Error("Exact package name required");
  const source = join(build, name);
  if (!lstatSync(source).isDirectory()) throw Error("Original package directory required");
  const destination = join(spill, name);
  mkdirSync(destination);
  const copied = await Bun.spawn(["rsync", "-aH", "--remove-source-files", "--stats", source + "/", destination + "/"], { stdout: "inherit", stderr: "inherit" }).exited;
  if (copied) throw Error(`Transfer failed for ${name}, both paths retained`);
  const emptied = await Bun.spawn(["find", source, "-depth", "-type", "d", "-empty", "-delete"], { stdout: "inherit", stderr: "inherit" }).exited;
  if (emptied) throw Error(`Could not retire empty stages for ${name}`);
  symlinkSync(destination, source, "dir");
  if (realpathSync(source) !== destination) throw Error(`Logical path could not be restored for ${name}`);
  console.log("[DEBUG]", name, destination);
}
