import { spawn } from "node:child_process";
import { readFileSync } from "node:fs";
import { createInterface } from "node:readline";
import { resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../..");
const source = readFileSync(resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts"), "utf8");
const terminalSource = source.split("\n").find(line => line.startsWith("function nativeBuildChildTerminalV1("));
if (!terminalSource) throw Error("Native driver terminal observation would be untested");
const runtime = new Bun.Transpiler({loader:"ts"}).transformSync(terminalSource);
const observe = new Function(runtime + "\nreturn nativeBuildChildTerminalV1;")() as
  (child: ReturnType<typeof spawn>) => Promise<number>;

const lineCount = 1024;
for (const exitCode of [0, 101]) {
  const child = spawn(process.execPath, ["-e", `
    const count = ${lineCount};
    const payload = "x".repeat(65500);
    for (let i=0; i<count; i++) {
      process.stdout.write(JSON.stringify({sequence:i,payload})+"\\n");
      process.stderr.write(i+"\\n");
    }
    process.stdout.write(JSON.stringify({finished:true})+"\\n");
    process.exitCode = ${exitCode};
  `], { detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"] });
  const terminal = observe(child);
  let complete = false;
  let stdout = 0;
  let stderr = 0;
  const lines = (async () => {
    for await (const line of createInterface({input: child.stdout!, crlfDelay: Infinity})) {
      const message = JSON.parse(line);
      if (message.finished) {
        await Bun.sleep(20);
        complete = true;
      } else {
        if (message.sequence !== stdout || message.payload.length !== 65500) throw Error("Cargo output lost a line");
        stdout++;
        if (stdout % 16 === 0) await Bun.sleep(1);
      }
    }
  })();
  const diagnostics = (async () => {
    for await (const line of createInterface({input: child.stderr!, crlfDelay: Infinity})) {
      if (Number(line) !== stderr++) throw Error("Cargo diagnostics lost a line");
    }
  })();
  let remaining: ReturnType<typeof setTimeout> | undefined;
  const deadline = new Promise<never>((_, reject) => {
    remaining = setTimeout(() => {
      child.kill("SIGKILL");
      reject(Error(`Child settlement timed out after stdout ${stdout}, stderr ${stderr}, finished ${complete}`));
    }, 15_000);
  });
  try {
    const [exit] = await Promise.race([Promise.all([terminal, lines, diagnostics]), deadline]);
    if (exit !== exitCode || !complete || stdout !== lineCount || stderr !== lineCount) {
      throw Error(`Child settlement lost events. Exit ${exit}, stdout ${stdout}, stderr ${stderr}, finished ${complete}`);
    }
  } finally {
    clearTimeout(remaining);
  }
}
console.log("[DEBUG] Both Cargo exit outcomes and 128 MiB of announced output settled through the pinned Bun subprocess and readline.");
