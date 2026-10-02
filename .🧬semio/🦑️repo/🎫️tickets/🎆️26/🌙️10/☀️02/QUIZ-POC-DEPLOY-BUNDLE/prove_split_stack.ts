/** 🧪️ The split deployment end to end with the two deliverables themselves, on this machine:
 *
 * - the backend is the staged bundle (`dist/proctor`: `compose.yaml`, `Caddyfile`, the image), run with `docker compose`
 *   as the host runs it — only the host name (`localhost`, so Caddy uses its internal CA), the ports and the granted
 *   site origin differ, through the `.env` the bundle offers for exactly that;
 * - the frontend is a release build of the site for that backend origin, served as static files from another origin,
 *   as a CDN serves it.
 *
 * A learner then plays in Chromium: with the backend up (decided by the proctor), with the backend stopped (decided and
 * saved on the device), and after it is started again (everything arrives; a fresh device finds it). The throw-away
 * compose project and its volumes are removed afterwards.
 *
 * `bun prove_split_stack.ts [bundle directory] [scratch directory]` */
import { spawnSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium, type Browser, type Page } from "playwright";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
import { answerRun, card, connection, enter, expectFeedback, goHome, learnerId, percent, playQuiz, quizOf, shownName, submitRun, type Device, type SourceQuiz } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🚶️learner/🟦️.ts";
import { siteArtifactProblems } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/🟦️.ts";
import { serveStaticSite } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🧱️stack/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, "../../../../../../..");
const bundleRoot = join(repoRoot, "🎓️teaching", "🏛️architecture", "❓️quiz", "📦️packages", "🟦️typescript");
const bundle = resolve(process.argv[2] ?? join(bundleRoot, "dist", "proctor"));
const scratch = resolve(process.argv[3] ?? join(here, "🗑️generated", "split-proof"));
const PROJECT = "architecture-quiz-split-proof";
const PORTS = { site: 6191, http: 18083, https: 18446 };
const siteOrigin = `http://127.0.0.1:${PORTS.site}`;
const proctorOrigin = `https://localhost:${PORTS.https}`;
const stack = join(scratch, "proctor");
const site = join(scratch, "site");

function say(line: string): void {
  console.log(`[split] ${line}`);
}

function run(command: string, args: readonly string[], cwd: string, env: NodeJS.ProcessEnv = process.env): string {
  const done = spawnSync(command, [...args], { cwd, env, encoding: "utf8", windowsHide: true, maxBuffer: 64 * 1024 * 1024 });
  if (done.status !== 0) throw new Error(`${command} ${args.slice(0, 4).join(" ")} exited ${done.status}: ${`${done.stdout}${done.stderr}`.split("\n").slice(-12).join("\n")}`);
  return `${done.stdout}${done.stderr}`;
}

function compose(...args: string[]): string {
  return run("docker", ["compose", "--project-name", PROJECT, ...args], stack);
}

async function device(browser: Browser): Promise<Device & { readonly seen: string[] }> {
  const context = await browser.newContext({ baseURL: siteOrigin, viewport: { width: 1440, height: 900 }, locale: "en-GB", ignoreHTTPSErrors: true });
  const page: Page = await context.newPage();
  const seen: string[] = [];
  page.on("pageerror", (error) => seen.push(`page error: ${error.message}`));
  await context.exposeBinding("quizSecurityViolation", (_source, violation: string) => void seen.push(`content security policy: ${violation}`));
  await context.addInitScript(() => document.addEventListener("securitypolicyviolation", (event) => (window as unknown as { quizSecurityViolation(violation: string): void }).quizSecurityViolation(`${event.effectiveDirective} blocked ${event.blockedURI || "inline code"}`)));
  return { context, page, locale: "en", problems: [], expectingFailures: async (body) => (await body(), 0), seen };
}

async function best(learner: Device, quiz: SourceQuiz): Promise<number> {
  return percent(await card(learner.page, quiz.id).locator("li", { hasText: "%" }).innerText());
}

async function tone(learner: Device, wanted: string, timeout: number): Promise<string> {
  await connection(learner).and(learner.page.locator(`[data-tone="${wanted}"]`)).waitFor({ timeout });
  return (await connection(learner).innerText()).replace(/\s+/gu, " ").trim();
}

async function play(learner: Device, quiz: SourceQuiz): Promise<number> {
  await playQuiz(learner, quiz.id);
  await answerRun(learner, quiz, "perfect");
  await submitRun(learner);
  const { score } = await expectFeedback(learner, quiz, "perfect");
  await goHome(learner);
  return score;
}

if (!existsSync(join(bundle, "compose.yaml")) || !existsSync(join(bundle, "proctor-image.tar"))) throw new Error(`${bundle} is no staged bundle; run docker-stack-bundle first`);
rmSync(scratch, { recursive: true, force: true });
mkdirSync(stack, { recursive: true });
for (const file of ["compose.yaml", "Caddyfile"]) cpSync(join(bundle, file), join(stack, file));
mkdirSync(join(stack, "certificates"), { recursive: true });
writeFileSync(join(stack, ".env"), [`PROCTOR_HOST=localhost`, `PROCTOR_ALLOWED_ORIGINS=${siteOrigin}`, `QUIZ_HTTP_PORT=${PORTS.http}`, `QUIZ_HTTPS_PORT=${PORTS.https}`].map((line) => `${line}\n`).join(""));
say(`backend: the bundle's compose.yaml and Caddyfile in ${stack}, .env: host localhost, ports ${PORTS.http}/${PORTS.https}, granted origin ${siteOrigin}`);
say(run("docker", ["load", "--input", join(bundle, "proctor-image.tar")], stack).trim().split("\n").at(-1) ?? "");

let browser: Browser | undefined;
let served: Awaited<ReturnType<typeof serveStaticSite>> | undefined;
let up = false;
try {
  up = true;
  compose("up", "--detach", "--wait", "--wait-timeout", "180");
  say(`backend up: ${compose("ps", "--format", "{{.Service}} {{.Status}}").trim().split("\n").join("; ")}`);

  say(`frontend: release build for ${proctorOrigin}`);
  run(process.execPath, ["./📜️script.ts", "build", "--outDir", site, "--emptyOutDir"], bundleRoot, { ...process.env, PROCTOR_URL: proctorOrigin });
  const problems = siteArtifactProblems(site, proctorOrigin);
  if (problems.length > 0) throw new Error(`the build is no site artifact:\n- ${problems.join("\n- ")}`);
  served = await serveStaticSite(site, PORTS.site);
  say(`frontend served as static files at ${served.origin}`);

  process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH;
  browser = await chromium.launch();
  const [online, offline] = [quizOf("cooling"), quizOf("demand")];
  const name = `Split Proof ${Math.random().toString(36).slice(2, 8)}`;
  const learner = await device(browser);
  await enter(learner, { kind: "pseudonym", handle: name });
  say(`1. backend up: ${await shownName(learner)} registered; ${await tone(learner, "calm", 60_000)}`);
  say(`   ${online.id} played: ${await play(learner, online)} %; ${await tone(learner, "calm", 60_000)}`);
  const id = await learnerId(learner);

  compose("stop");
  say(`2. backend stopped: ${compose("ps", "--all", "--format", "{{.Service}} {{.State}}").trim().split("\n").join("; ")}`);
  say(`   ${await tone(learner, "alert", 60_000)}`);
  say(`   ${offline.id} played without the backend: ${await play(learner, offline)} %; best on the overview ${await best(learner, offline)} %`);
  await learner.page.reload();
  await learner.page.locator("[data-layered-overview]").waitFor();
  const queued = await learner.page.evaluate(() => Object.keys(localStorage).filter((key) => key.includes(".outbox/")).length);
  say(`   after a reload: ${await shownName(learner)}, ${offline.id} ${await best(learner, offline)} %, ${online.id} ${await best(learner, online)} %; ${queued} commands wait on the device; ${await tone(learner, "alert", 60_000)}`);
  if (queued === 0) throw new Error("nothing waited on the device while the backend was stopped");

  compose("start");
  compose("up", "--detach", "--wait", "--wait-timeout", "180");
  say(`3. backend started again: ${await tone(learner, "calm", 120_000)}`);
  const left = await learner.page.evaluate(() => Object.keys(localStorage).filter((key) => key.includes(".outbox/")).length);
  if (left !== 0) throw new Error(`${left} commands still wait on the device`);
  const elsewhere = await device(browser);
  await enter(elsewhere, { kind: "pseudonym", handle: name });
  const found = { id: await learnerId(elsewhere), online: await best(elsewhere, online), offline: await best(elsewhere, offline) };
  say(`   a fresh device entering ${name}: the same learner ${found.id === id}, ${online.id} ${found.online} %, ${offline.id} ${found.offline} %`);
  if (found.id !== id || found.online !== 100 || found.offline !== 100) throw new Error(`the backend does not hold what the device did: ${JSON.stringify(found)}`);
  const seen = [...learner.seen, ...elsewhere.seen];
  if (seen.length > 0) throw new Error(`the pages saw: ${seen.join(" | ")}`);
  say("page errors and content security policy violations: none");
  say("split deployment proven");
} finally {
  await browser?.close();
  await served?.close();
  if (up) spawnSync("docker", ["compose", "--project-name", PROJECT, "down", "--volumes", "--timeout", "30"], { cwd: stack, encoding: "utf8", windowsHide: true });
}
