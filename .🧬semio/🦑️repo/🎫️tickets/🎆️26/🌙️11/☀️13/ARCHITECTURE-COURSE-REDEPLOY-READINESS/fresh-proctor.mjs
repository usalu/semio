/** 🆕️ Fresh artifacts for the ticket launchers: builds the proctor through `repo cargo` (incremental, so cheap when nothing changed) and returns the newest product, refusing one older than its Rust sources; says whether a static site build is older than its sources. Run the laws with `node --test fresh-proctor.test.mjs`. */
import { spawnSync } from "node:child_process";
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repo = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../..");
const SKIPPED = new Set(["target", "dist", "node_modules", ".git", ".cache", "🧪️tests"]);
const RUST = /\.(rs|toml|lock)$/;
const SITE = /\.(ts|tsx|css|json|svg|html|md|mjs)$/;
const PROCTOR_SOURCES = ["🎓️teaching/🛂️proctor", "🧰️framework/🛍️products/🖥️server", "🧰️framework/🛍️products/📚️course", "🧰️framework/🔨️modules/📡️replication", "🧰️framework/🔨️modules/⏳️async", "🧰️framework/🔨️modules/🔀️dispatch", "🧰️framework/🔨️modules/🔏️hash", "🎓️teaching/Cargo.toml", "🎓️teaching/Cargo.lock"];
const SITE_SOURCES = ["🎓️teaching/🏛️architecture/📚️course", "🧰️framework/🛍️products/📚️course", "🧰️framework/🛍️products/🖥️server"];

/** 🔎️ The newest file below `roots` whose name matches `kind`: its path and mtime, or `undefined`. */
export function newestSource(roots, kind) {
  let newest;
  const visit = (path) => {
    let stat;
    try { stat = statSync(path); } catch { return; }
    if (stat.isDirectory()) {
      for (const name of readdirSync(path)) if (!SKIPPED.has(name)) visit(join(path, name));
    } else if (kind.test(path) && (newest === undefined || stat.mtimeMs > newest.mtimeMs)) newest = { file: path, mtimeMs: stat.mtimeMs };
  };
  roots.forEach(visit);
  return newest;
}

/** ⏳️ Why an artifact of `builtMs` is older than `source`, or `undefined` when it is at least as new. */
export function staleReason(builtMs, source) {
  return source !== undefined && source.mtimeMs > builtMs ? `older than ${source.file} by ${Math.round((source.mtimeMs - builtMs) / 1000)} s` : undefined;
}

/** 🥇️ The newest `<hash>/out/proctor` below a Cargo `build/teaching-proctor` directory. */
export function newestProduct(directory) {
  if (!existsSync(directory)) return undefined;
  const found = readdirSync(directory).map((hash) => join(directory, hash, "out", "proctor")).filter(existsSync);
  return found.sort((a, b) => statSync(b).mtimeMs - statSync(a).mtimeMs)[0];
}

/** 🛠️ Builds the proctor in `configuration` (`prototype` or `release`) and returns its path; throws when the build fails or the product is older than its sources. `PROCTOR_BUILD=off` skips the build and keeps the staleness check. */
export function freshProctor(configuration = "prototype") {
  if (process.env.PROCTOR_BUILD !== "off") {
    const path = `${homedir()}/.local/share/semio/node/v24.15.0/bin:${homedir()}/.cargo/bin:${process.env.PATH}`;
    const built = spawnSync("repo", ["cargo", "--store=shared", `--configuration=${configuration}`, "--", "build", "--manifest-path", "🎓️teaching/Cargo.toml", "-p", "teaching-proctor", ...(configuration === "release" ? ["--release"] : [])], { cwd: repo, env: { ...process.env, PATH: path, SEMIO_LANGUAGE: "en" }, stdio: ["ignore", process.stderr, process.stderr] });
    if (built.status !== 0) throw new Error(`building the ${configuration} proctor failed (exit ${built.status})`);
  }
  const lanes = join(repo, ".cache/b/r/m/u/l");
  const products = readdirSync(lanes).map((lane) => newestProduct(join(lanes, lane, configuration, "build/teaching-proctor"))).filter(Boolean);
  const executable = products.sort((a, b) => statSync(b).mtimeMs - statSync(a).mtimeMs)[0];
  if (executable === undefined) throw new Error(`no built ${configuration} proctor below ${lanes}`);
  const reason = staleReason(statSync(executable).mtimeMs, newestSource(PROCTOR_SOURCES.map((p) => join(repo, p)), RUST));
  if (reason) throw new Error(`the ${configuration} proctor ${executable} is ${reason}`);
  return executable;
}

/** 🌐️ Whether the static site in `out` (a build tagged with the proctor origin it bakes in) is at least as new as every site source. */
export function siteReason(out, origin) {
  const marker = join(out, ".built-for");
  if (!existsSync(join(out, "index.html")) || !existsSync(marker)) return "no build";
  if (readFileSync(marker, "utf8").trim() !== origin) return `built for another proctor origin than ${origin}`;
  return staleReason(statSync(marker).mtimeMs, newestSource(SITE_SOURCES.map((p) => join(repo, p)), SITE));
}

/** 🏷️ Tags a finished site build with the proctor origin it bakes in. */
export function markSite(out, origin) {
  writeFileSync(join(out, ".built-for"), origin + "\n");
}
