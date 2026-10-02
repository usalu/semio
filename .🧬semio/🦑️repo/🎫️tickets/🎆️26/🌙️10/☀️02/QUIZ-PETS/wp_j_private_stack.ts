#!/usr/bin/env bun
/** 🧱️ Ticket tool of work package J: a private stack of the architecture quiz on ports nobody else uses, for looking at
 * the pets in the real app. It never builds anything with cargo: it runs a private copy of the proctor executable that
 * is already built, over a scratch data directory.
 *
 * `dev`       — development proctor on 8921, the site's dev server on 6191 (proxying to it, private dependency cache,
 *               not watching the sources).
 * `rehearsal` — production-mode proctor on 8922 that admits exactly the static origin, and the release build found in
 *               `<scratch>/site-rehearsal` (built with `PROCTOR_URL=http://127.0.0.1:8922`) served on 6192 as a CDN
 *               would, so the document's Content-Security-Policy is the real one.
 *
 * Usage (from the repository root): bun ".../wp_j_private_stack.ts" <dev|rehearsal>
 * `WP_STACK_SCRATCH` names the folder under `🗑️generated` that holds the scratch (default `wp-j`; work package N uses `wp-n`);
 * `WP_STACK_SITE_PORT` and `WP_STACK_PROCTOR_PORT` replace the ports of the topology (work package P: 6193 and 8923, 6194 and 8924).
 * The process ids of everything started are written to `<scratch>/stack-<topology>.pids`; the tool runs until it is
 * killed and takes its children with it when it is asked to stop. */
import { spawn, type ChildProcess } from "node:child_process";
import { copyFileSync, mkdirSync, openSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { serveStaticSite } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🧱️stack/🟦️.ts";

const topology = process.argv[2] === "rehearsal" ? "rehearsal" : "dev";
const ticket = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(ticket, "../../../../../../..");
const scratch = join(ticket, "🗑️generated", process.env.WP_STACK_SCRATCH ?? "wp-j");
const work = join(scratch, `stack-${topology}`);
const defaults = topology === "dev" ? { site: 6191, proctor: 8921 } : { site: 6192, proctor: 8922 };
const ports = { site: Number(process.env.WP_STACK_SITE_PORT ?? defaults.site), proctor: Number(process.env.WP_STACK_PROCTOR_PORT ?? defaults.proctor) };
const site = `http://127.0.0.1:${ports.site}`;
const proctor = `http://127.0.0.1:${ports.proctor}`;
const bundleRoot = join(repoRoot, "🎓️teaching", "🏛️architecture", "❓️quiz", "📦️packages", "🟦️typescript");
const built = join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", "cargo", "target", "debug", process.platform === "win32" ? "proctor.exe" : "proctor");

rmSync(join(work, "proctor-data"), { recursive: true, force: true });
mkdirSync(join(work, "proctor-data"), { recursive: true });
const copy = join(work, `proctor-${process.pid}${process.platform === "win32" ? ".exe" : ""}`);
copyFileSync(built, copy);

const inherited = Object.fromEntries(Object.entries(process.env).filter(([name]) => !name.startsWith("PROCTOR_")));
const children: ChildProcess[] = [];
const launch = (command: string, args: readonly string[], cwd: string, env: NodeJS.ProcessEnv, log: string): ChildProcess => {
  const sink = openSync(join(work, log), "a");
  const child = spawn(command, [...args], { cwd, env, stdio: ["ignore", sink, sink], windowsHide: true });
  children.push(child);
  return child;
};

launch(copy, ["serve"], repoRoot, { ...inherited, PROCTOR_PORT: String(ports.proctor), PROCTOR_DATA: join(work, "proctor-data"), PROCTOR_CATALOG: join(repoRoot, "🎓️teaching", "🏛️architecture", "❓️quiz", "🔣️.json"), ...(topology === "dev" ? { PROCTOR_MODE: "development" } : { PROCTOR_MODE: "production", PROCTOR_ALLOWED_ORIGINS: site }) }, "proctor.log");

let closeStatic: (() => Promise<void>) | undefined;
if (topology === "dev") launch(process.execPath, [join(bundleRoot, "📜️script.ts"), "dev-site"], bundleRoot, { ...inherited, TEACHING_ARCHITECTURE_QUIZ_PORT: String(ports.site), PROCTOR_PORT: String(ports.proctor), TEACHING_ARCHITECTURE_QUIZ_CACHE: join(work, "node_modules", ".vite"), TEACHING_ARCHITECTURE_QUIZ_WATCH: "off" }, "site.log");
else closeStatic = (await serveStaticSite(join(scratch, "site-rehearsal"), ports.site)).close;

writeFileSync(join(scratch, `stack-${topology}.pids`), `${[process.pid, ...children.map((child) => child.pid)].join("\n")}\n`);
process.stdout.write(`${topology}: site ${site}, proctor ${proctor}, logs in ${work}\n`);

const stop = async (): Promise<void> => {
  for (const child of children) child.kill();
  await closeStatic?.();
  process.exit(0);
};
for (const signal of ["SIGINT", "SIGTERM", "SIGHUP"] as const) process.on(signal, () => void stop());
await new Promise(() => {});
