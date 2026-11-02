import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const oracle = resolve(dirname(fileURLToPath(import.meta.url)), "🗑️generated/o");
mkdirSync(oracle, { recursive: true });
writeFileSync(resolve(oracle, "package.json"), JSON.stringify({ name: "storybook-typography-oracle", private: true, devDependencies: { "tailwind-merge": "3.4.0" } }));
const run = async (args: string[], env = process.env): Promise<void> => {
  const child = Bun.spawn([process.execPath, ...args], { env, stdout: "inherit", stderr: "inherit" });
  const cancel = () => child.kill("SIGTERM");
  process.once("SIGTERM", cancel);
  process.once("SIGINT", cancel);
  try {
    const code = await child.exited;
    if (code) throw new Error(`Oracle command exited ${code}`);
  } finally {
    process.removeListener("SIGTERM", cancel);
    process.removeListener("SIGINT", cancel);
  }
};
try {
  await run(["install", "--cwd", oracle, "--ignore-scripts", "--linker=hoisted"]);
  await run(["test", "./🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🧪️tests/🔀️merge-conflicting-utilities/🟦️.test.ts"], { ...process.env, CLASS_COMPOSITION_ORACLE: resolve(oracle, "node_modules/tailwind-merge/dist/bundle-mjs.mjs") });
} finally { rmSync(oracle, { recursive: true, force: true }); }
