/** 🧹️ Old development data against the real proctor (work package "UI polish", item 3). It never touches the developer's
 * own `.🧬semio/🎓️teaching/proctor-dev/`: it works on copies of it under `<ticket>/🗑️generated/polish/`.
 *
 * `bun ui_polish_dev_data.ts <copy of the v1 data directory> <work directory> [proctor port]`
 *
 * 1. The launcher's own folder: a throw-away checkout root whose `.🧬semio/🎓️teaching/proctor-dev` is a copy of the v1
 *    data. `settleDevelopmentData` must move it aside and say so; the proctor built from the working tree must then
 *    serve over fresh data of the format it reads, and what was set aside must still be the v1 file, byte for byte.
 * 2. A folder the developer named (`PROCTOR_DATA`): `settleDevelopmentData` must refuse in one message and leave it.
 * 3. The proctor itself, development and production mode, on a copy of the v1 data: it must refuse loudly, exit
 *    non-zero and leave the file as it was — the dev launcher's convenience is not the binary's behaviour.
 * @see ../../../../../../../🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts */
import { createHash } from "node:crypto";
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { awaitReady } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🧱️stack/🟦️.ts";
import { PROCTOR_DATABASE_FILE, PROCTOR_DEV_CATALOG, PROCTOR_DEV_DATA_DIRECTORY, PROCTOR_STORAGE_FORMAT, buildProctor, launchProctor, proctorDevelopmentEnvironment, proctorReady, settleDevelopmentData, storedProctorFormat } from "../../../../../../../🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const [sourceArgument, workArgument, port = "8801"] = process.argv.slice(2);
if (sourceArgument === undefined || workArgument === undefined) throw new Error("usage: bun ui_polish_dev_data.ts <copy of the v1 data directory> <work directory> [proctor port]");
const source = resolve(sourceArgument);
const work = resolve(workArgument);

const digest = (file: string): string => createHash("sha256").update(readFileSync(file)).digest("hex");
const original = digest(join(source, PROCTOR_DATABASE_FILE));
const checks: string[] = [];
const check = (label: string, passed: boolean, detail = ""): void => {
  checks.push(`${passed ? "PASS" : "FAIL"} ${label}${detail === "" ? "" : ` — ${detail}`}`);
  console.log(checks.at(-1));
};

rmSync(work, { recursive: true, force: true });
mkdirSync(work, { recursive: true });
const catalog = join(repoRoot, ...PROCTOR_DEV_CATALOG);
const executable = await buildProctor(repoRoot);

/** 🚀️ Runs `proctor serve` on `data` in `mode` until it exits or answers ready; answers how it ended and its log. */
async function serve(data: string, mode: "development" | "production", name: string): Promise<{ readonly ready: boolean; readonly status: number | undefined; readonly log: string }> {
  const log = join(work, `${name}.log`);
  const origin = `http://127.0.0.1:${port}`;
  const env = { ...process.env, PROCTOR_PORT: port, PROCTOR_DATA: data, PROCTOR_CATALOG: catalog, PROCTOR_MODE: mode, ...(mode === "production" ? { PROCTOR_ALLOWED_ORIGINS: "http://127.0.0.1:6071" } : {}) };
  const proctor = launchProctor(executable, repoRoot, ["serve"], env, log);
  try {
    await awaitReady({ label: `the ${mode} proctor at ${origin}`, probe: () => proctorReady(origin), exited: proctor.exited, timeoutMs: 60_000, say: () => undefined });
    return { ready: true, status: undefined, log: readFileSync(log, "utf8") };
  } catch {
    return { ready: false, status: await Promise.race([proctor.exited, new Promise<undefined>((accept) => setTimeout(() => accept(undefined), 2_000))]), log: readFileSync(log, "utf8") };
  } finally {
    await proctor.stop();
  }
}

console.log("— 1. the launcher's own folder");
const checkout = join(work, "checkout");
const own = join(checkout, ...PROCTOR_DEV_DATA_DIRECTORY);
cpSync(source, own, { recursive: true });
const env = proctorDevelopmentEnvironment(checkout, { PROCTOR_PORT: port });
check("the copy is of another format than the proctor reads", JSON.stringify(await storedProctorFormat(own)) === JSON.stringify({ schema: PROCTOR_STORAGE_FORMAT.schema, version: 1 }), JSON.stringify(await storedProctorFormat(own)));
const started = Date.now();
const said = await settleDevelopmentData(checkout, env);
console.log(`[launcher] ${said}`);
const aside = readdirSync(dirname(own)).filter((name) => name.startsWith("proctor-dev.v1-"));
check("it says one message that names the folder, calls it disposable development data of an older format and says how to reset", said !== undefined && said.includes(own) && said.includes("disposable development data in an older storage format") && said.includes(`delete ${own}`), `${Date.now() - started} ms`);
check("the folder was moved aside to one sibling named by format and time", aside.length === 1 && !existsSync(own), aside.join(", "));
check("what was set aside is the v1 file, byte for byte", aside.length === 1 && digest(join(dirname(own), aside[0]!, PROCTOR_DATABASE_FILE)) === original);
const fresh = await serve(env.PROCTOR_DATA!, "development", "fresh");
check("the proctor then serves over fresh data", fresh.ready, fresh.ready ? "GET /instance answered ready" : `exit ${fresh.status}: ${fresh.log.trim().split("\n").at(-1)}`);
check("the fresh data, left behind by a proctor that was ended at once, is of the format the proctor reads", JSON.stringify(await storedProctorFormat(own)) === JSON.stringify(PROCTOR_STORAGE_FORMAT), `${JSON.stringify(await storedProctorFormat(own))} in ${readdirSync(own).join(", ")}`);
check("settling again finds nothing to do", (await settleDevelopmentData(checkout, env)) === undefined && existsSync(own));

console.log("— 2. a folder the developer named");
const named = join(work, "named");
cpSync(source, named, { recursive: true });
const refusal = await settleDevelopmentData(checkout, proctorDevelopmentEnvironment(checkout, { PROCTOR_PORT: port, PROCTOR_DATA: named })).then(
  () => undefined,
  (error: unknown) => (error instanceof Error ? error.message : String(error)),
);
console.log(`[launcher] ${refusal}`);
check("it refuses in one message that names the folder and says how to go on", refusal !== undefined && refusal.includes(named) && refusal.includes("PROCTOR_DATA names that folder, so it is left as it is"));
check("the named folder holds its database as it was (beside it at most the two files of SQLite's write-ahead log)", readdirSync(named).filter((name) => !/-(?:shm|wal)$/u.test(name)).join() === PROCTOR_DATABASE_FILE && digest(join(named, PROCTOR_DATABASE_FILE)) === original, readdirSync(named).join(", "));

console.log("— 3. the proctor itself");
for (const mode of ["development", "production"] as const) {
  const data = join(work, `binary-${mode}`);
  cpSync(source, data, { recursive: true });
  const outcome = await serve(data, mode, `refused-${mode}`);
  const line = outcome.log.split("\n").find((entry) => entry.includes("holds format")) ?? "";
  console.log(`[proctor ${mode}] ${line.trim()}`);
  check(`proctor serve (${mode}) refuses the v1 file loudly and exits non-zero`, !outcome.ready && outcome.status !== undefined && outcome.status !== 0 && line.includes(`holds format ${PROCTOR_STORAGE_FORMAT.schema} v1; this proctor reads ${PROCTOR_STORAGE_FORMAT.schema} v${PROCTOR_STORAGE_FORMAT.version}`), `exit ${outcome.status}`);
  check(`proctor serve (${mode}) leaves the v1 file as it was`, digest(join(data, PROCTOR_DATABASE_FILE)) === original);
}

check("the source copy is as it was", digest(join(source, PROCTOR_DATABASE_FILE)) === original);
const failed = checks.filter((line) => line.startsWith("FAIL"));
console.log(`${checks.length - failed.length}/${checks.length} checks passed`);
process.exit(failed.length === 0 ? 0 : 1);
