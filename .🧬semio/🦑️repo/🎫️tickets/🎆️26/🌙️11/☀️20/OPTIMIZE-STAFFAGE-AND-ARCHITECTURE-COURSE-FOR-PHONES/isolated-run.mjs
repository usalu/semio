import { fileURLToPath } from "node:url";
import { run, serve } from "./isolated-browser-runner.ts";

const scratch = fileURLToPath(new URL("./🗑️generated/w", import.meta.url));
const [action, file, port] = process.argv.splice(2, 2);
if (file === undefined) throw Error("Specify a verification entry");
if (action === "run") {
  await run(scratch, file);
} else if (action === "serve") {
  await serve(scratch, file, Number(process.argv[2] ?? port));
} else {
  throw Error("Expected run or serve");
}
