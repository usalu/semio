import {resolve} from "node:path";
import {writeFileSync, appendFileSync} from "node:fs";
import {spawn} from "node:child_process";

const taskRepoRoot = resolve(import.meta.dir, "../../../../../../../..");
writeFileSync(resolve(import.meta.dir, "../🗑️generated/r15/costs-script-entry.log"), "[DEBUG] source runner entered at " + taskRepoRoot + "\n");

/** 🧪️ Executes existing BIM source witnesses through Nx without copying source files or discovering unrelated projects. */
async function main(args: string[]): Promise<void> {
  if (args.length !== 1 || args[0] !== "costs") throw new Error("Expected costs");
  const output = resolve(import.meta.dir, "../🗑️generated/r15/costs-source-process.log");
  writeFileSync(output, "");
  const child = spawn(process.execPath, ["test", resolve(taskRepoRoot, "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💰️costs/🧪️tests/🟦️.ts")], {cwd: taskRepoRoot, windowsHide: true, stdio: ["ignore", "pipe", "pipe"]});
  child.stdout.on("data", bytes => appendFileSync(output, bytes));
  child.stderr.on("data", bytes => appendFileSync(output, bytes));
  const stop = (): void => {child.kill();};
  const timer = setTimeout(stop, 30000);
  process.once("SIGINT", stop);
  process.once("SIGTERM", stop);
  try {
    const code = await new Promise<number>((accept, reject) => {child.once("error", reject); child.once("close", code => accept(code ?? 1));});
    if (code !== 0) throw new Error("Source tests failed; see costs-source-process.log");
  } finally {
    clearTimeout(timer);
    process.off("SIGINT", stop);
    process.off("SIGTERM", stop);
  }
}

await main(process.argv.slice(2));
