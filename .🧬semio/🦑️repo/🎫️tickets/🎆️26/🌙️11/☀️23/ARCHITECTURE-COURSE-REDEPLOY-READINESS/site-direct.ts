/** 🛶️ Runs the course site without the build graph, for hosts where no graph service answers.
 * `build <mode>` writes the Vite artifact to the package's `dist/<mode>`; `dev` starts the dev server on port 6061 at once and, unless a proctor answers on `PROCTOR_PORT`, compiles the dev proctor into a private target directory and starts it beside it.
 * The config's two requirements are supplied here: its compiler cache path, and the assignment PDFs over the runtime channel — the ones the site host of `🚀️deploy/🔣️.json` publishes, kept in the local cache, since compiling them needs the Print pipeline of the graph route.
 * Usage (from the repository root): node <this file> <build <prototype|release>|dev> */
import { spawn, type ChildProcess } from "node:child_process";
import { randomBytes } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createServer } from "node:http";
import { join, resolve } from "node:path";

const [command, requested] = process.argv.slice(2);
const mode = command === "dev" ? "prototype" : requested;
if (!["build", "dev"].includes(command ?? "") || !["prototype", "release"].includes(mode ?? "")) throw Error("Usage: site-direct.ts <build <prototype|release>|dev>");
const workspace = resolve(import.meta.dirname, "../../../../../../..");
const site = join(workspace, "🎓️teaching/🏛️architecture/📚️course");
const course = JSON.parse(readFileSync(join(site, "🔣️.json"), "utf8"));
const deployment = JSON.parse(readFileSync(join(site, "🚀️deploy/🔣️.json"), "utf8"));
const cache = join(workspace, ".cache", "b", "p", "m", "vite", mode!, "architecture-course-direct");
const pdfs = join(workspace, ".cache", "b", "p", "m", "vite", "architecture-course-direct-pdfs");
mkdirSync(cache, { recursive: true });
mkdirSync(pdfs, { recursive: true });
for (const assignment of course.assignments) for (const document of Object.values<{ file: string }>(assignment.documents)) {
  if (existsSync(join(pdfs, document.file))) continue;
  const answer = await fetch(`https://${deployment.site.host}/assignments/${document.file}`, { signal: AbortSignal.timeout(60_000) });
  const bytes = new Uint8Array(await answer.arrayBuffer());
  if (!answer.ok || new TextDecoder().decode(bytes.subarray(0, 5)) !== "%PDF-") throw Error(`Published assignment PDF unavailable: ${document.file}`);
  writeFileSync(join(pdfs, document.file), bytes);
}
const secret = randomBytes(32).toString("hex");
const server = createServer((request, response) => {
  const path = new URL(request.url ?? "/", "http://localhost").pathname;
  const file = join(pdfs, path.startsWith("/pdfs/") ? path.slice(6).replaceAll(/[^A-Za-z0-9.-]/gu, "") : "-");
  if (request.headers.authorization !== "Bearer " + secret || !existsSync(file)) { response.statusCode = 404; return void response.end(); }
  const bytes = readFileSync(file);
  response.setHeader("content-type", "application/pdf");
  response.setHeader("content-length", bytes.length);
  response.end(bytes);
});
await new Promise<void>(accept => server.listen(0, "127.0.0.1", accept));
const address = server.address();
if (!address || typeof address === "string") throw Error("Listener required");
const children: ChildProcess[] = [];
const stop = (status: number): void => { for (const child of children) child.kill(); server.close(); server.closeAllConnections(); process.exit(status); };
const proctorPort = process.env.PROCTOR_PORT ?? String(deployment.proctor.port);
const answers = (): Promise<boolean> => fetch(`http://127.0.0.1:${proctorPort}/instance`, { signal: AbortSignal.timeout(2_000) }).then(answer => answer.ok, () => false);
if (command === "dev" && !(await answers())) void (async () => {
  const data = join(workspace, ".🧬semio", "🎓️teaching", "proctor-dev-direct");
  mkdirSync(data, { recursive: true });
  const target = join(".cache", "b", "r", "m", "u", "l", "acd", "t");
  const binary = process.platform === "win32" ? "proctor.exe" : "proctor";
  const committed = join(workspace, ".cache", "b", "r", "m", "u", "l", "ach", "t", "debug", binary);
  let executable = join(workspace, target, "debug", binary);
  for (let attempt = 1; ; attempt++) {
    const build = spawn("repo", ["tools", "exec", "cargo", "--", "build", "--manifest-path", "🎓️teaching/Cargo.toml", "-p", "teaching-proctor", "--bin", "proctor", "--config", 'build.rustc="rustc"', "--config", 'build.rustc-wrapper=""', "--config", `build.target-dir=${JSON.stringify(target.replaceAll("\\", "/"))}`, "--config", `build.build-dir=${JSON.stringify(target.replaceAll("\\", "/"))}`], { cwd: workspace, stdio: "inherit" });
    if (await new Promise<number | null>(accept => build.on("exit", accept)) === 0) break;
    if (existsSync(committed)) { executable = committed; console.log("[site-direct] the live tree does not compile; starting the proctor built from the last committed snapshot"); break; }
    if (attempt === 100) throw Error("The proctor did not compile in 100 attempts");
    console.log(`[site-direct] the proctor did not compile (attempt ${attempt}); the shared tree may be mid-edit, trying again in three minutes`);
    await new Promise(accept => setTimeout(accept, 180_000));
  }
  const proctor = spawn(executable, ["serve"], { cwd: workspace, stdio: "inherit", env: { ...process.env, PROCTOR_PORT: proctorPort, PROCTOR_DATA: data, PROCTOR_COURSE: join(site, "🔣️.json") } });
  children.push(proctor);
  proctor.on("exit", status => { console.log(`[site-direct] the proctor ended with status ${status}`); });
  for (let waited = 0; !(await answers()); waited += 2) { if (waited > 1_800) throw Error("The proctor did not answer within 30 minutes"); await new Promise(accept => setTimeout(accept, 2_000)); }
  console.log(`[site-direct] the proctor answers on http://127.0.0.1:${proctorPort}`);
})().catch(error => console.log(`[site-direct] no proctor: ${error instanceof Error ? error.message : String(error)}; the site stays up and waits for one on port ${proctorPort}`));
const channels = JSON.stringify([{ name: "course-assignment-pdfs", endpoint: `http://127.0.0.1:${address.port}/pdfs`, secret }]);
const args = ["tools", "exec", "vite", "--", ...(command === "build" ? ["build"] : ["--host", "127.0.0.1", "--port", "6061", "--strictPort"]), "--mode", mode!, "--configLoader", "native", "--config", "../../🏗️builder/🌐️vite/🟦️.ts"];
const vite = spawn("repo", args, { cwd: join(site, "📦️packages/🟦️typescript"), stdio: "inherit", env: { ...process.env, PROCTOR_PORT: proctorPort, SEMIO_BUILD_MODE: mode, SEMIO_VITE_CACHE_DIRECTORY: cache, SEMIO_VITE_RUNTIME_CHANNELS_V1: channels } });
children.push(vite);
for (const signal of ["SIGINT", "SIGTERM"] as const) process.on(signal, () => stop(130));
vite.on("exit", status => stop(status ?? 1));
