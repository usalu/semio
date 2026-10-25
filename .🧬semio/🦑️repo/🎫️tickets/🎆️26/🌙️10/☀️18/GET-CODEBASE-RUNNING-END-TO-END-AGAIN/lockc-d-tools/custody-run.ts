import { acquireCargoCustodyV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🔐️custody/🟦️.ts";
const [directory, buildDirectory, profile, ...command] = process.argv.slice(2), started = Date.now();
const custody = await acquireCargoCustodyV1({ directory, buildDirectory, profile, signal: new AbortController().signal });
console.log(JSON.stringify({ event: "custody", waitedMs: Date.now() - started, inherited: custody.inherited }));
const child = Bun.spawn(command, { stdout: "inherit", stderr: "inherit", env: process.env });
const code = await child.exited;
custody.release();
process.exit(code);
