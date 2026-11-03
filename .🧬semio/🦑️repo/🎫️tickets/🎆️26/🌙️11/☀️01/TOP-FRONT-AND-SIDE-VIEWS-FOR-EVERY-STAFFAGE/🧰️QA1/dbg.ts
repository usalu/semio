/** [DEBUG] QA1: runs several debug scripts in one process. */
import { join } from "node:path";
const here = import.meta.dir;
const run = async (file: string, ...args: string[]) => { process.argv = [process.argv[0]!, join(here, file), ...args]; console.log("#####", file, args.join(" ")); await import(join(here, file) + "?" + args.join("_")); };
await run("shapes.ts", "herring-gull", "200", "standing");
