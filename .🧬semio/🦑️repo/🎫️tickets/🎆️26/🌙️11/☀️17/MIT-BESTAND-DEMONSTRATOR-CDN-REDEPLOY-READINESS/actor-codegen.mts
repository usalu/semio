/** 🎰️ Runs a pinned actor decoder measurement with the native compiler's own jobserver. */
import { mkdirSync } from "node:fs";
import { createJobserverV1, defaultJobserverStyleV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/🟦️.ts";

const [directory, tokens, ...command] = process.argv.slice(2);
if (!directory || !tokens || !command.length) throw Error("Actor measurement needs directory, tokens and command");
mkdirSync(directory, { recursive: true });
const server = await createJobserverV1({ version: 1, tokens: Number(tokens), style: defaultJobserverStyleV1() }, directory);
try {
  const status = await server.withClient(async env => {
    const child = Bun.spawn(command, { env: { ...process.env, ...env }, stdin: "inherit", stdout: "inherit", stderr: "inherit" });
    return child.exited;
  });
  process.exitCode = status;
} finally {
  server.close();
}
