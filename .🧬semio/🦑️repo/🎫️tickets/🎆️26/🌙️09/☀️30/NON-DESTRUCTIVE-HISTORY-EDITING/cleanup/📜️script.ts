import { lstat, realpath, rm } from "node:fs/promises";
import { resolve, relative, isAbsolute } from "node:path";

const root = await realpath(resolve(import.meta.dir, "../🗑️generated"));
const command = process.argv.slice(2).join(" ");
const paths = command === "retire-completed-targets"
  ? ["coord/base-target", "coord/train-target", "s5-agnostic/target", "s5-text-stdio/target", "s5-gates/target"]
  : command === "retire-completed-verification"
    ? ["e2e", "s3-w1g", "s3-w2c", "s5-e2e"]
    : undefined;
if (!paths) throw Error("Expected a named completed-output retirement command");
for (const name of paths) {
  const path = resolve(root, name);
  let state;
  try { state = await lstat(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") continue; throw error; }
  const actual = await realpath(path), contained = relative(root, actual);
  if (!state.isDirectory() || state.isSymbolicLink() || isAbsolute(contained) || contained.startsWith("..") || actual !== path) throw Error(`Refused generated target: ${name}`);
  await rm(path, { recursive: true });
  console.log(`Retired completed ticket compiler output: ${name}`);
}
