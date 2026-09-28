/** 🧩️ C12: runs one command against ONE `serve s react dev` from S18's shared `ensureDevServe` (joined to `hubUrl`), then stops
 * the serve it started. The command sees the serve URL as `C12_SERVE_URL`; credentials stay in the caller's env.
 * usage: bun c12-with-serve.ts <port> <hubUrl> -- <command …> */
import { spawn } from "node:child_process";
const repoRoot = "/Users/ueli/Documents/semio";
const { ensureDevServe } = await import(`${repoRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts`);
const [portText, hubUrl, separator, ...command] = process.argv.slice(2);
if (separator !== "--" || command.length === 0) throw new Error("usage: bun c12-with-serve.ts <port> <hubUrl> -- <command …>");
const port = Number(portText);
const serve = await ensureDevServe({ repoRoot, port, variant: "s", locale: "en", hubUrl, bootBoundMs: 900_000, logPath: `${repoRoot}/.🧬semio/🌐hub/s14-c12-logs/serve-${port}-with.txt`, onProgress: (_status: unknown, text: string) => console.log(`[serve] ${text}`) });
let code = 1;
try {
  code = await new Promise<number>((resolve) => {
    const child = spawn(command[0]!, command.slice(1), { stdio: "inherit", env: { ...process.env, C12_SERVE_URL: serve.url.replace(/\/$/u, "") } });
    child.on("exit", (exitCode) => resolve(exitCode ?? 1));
  });
} finally {
  await serve.stop();
  console.log(`[serve] ${serve.reused ? "reused (left running)" : "stopped"}; command exit ${code}`);
}
process.exitCode = code;
