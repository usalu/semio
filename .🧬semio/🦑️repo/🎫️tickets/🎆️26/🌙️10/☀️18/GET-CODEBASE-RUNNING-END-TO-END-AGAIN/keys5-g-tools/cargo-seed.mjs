// keys5-g cargo-seed: the files a cargo resolution token (SEMIO_CARGO_LOCK_*) is DERIVED from for one crate, graph-free: the production resolution module over a recording read host (the same host shape the graph inference uses). A file oracle sees these reads as undeclared; the token covers them by meaning.
// usage: bun cargo-seed.mjs <crate dir | Cargo.toml> [--root <workspace>]   prints one repository-relative path per line (sorted), then a JSON summary on stderr
import {createRequire} from "node:module";
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {join, resolve} from "node:path";
const root = resolve(process.argv.includes("--root") ? process.argv[process.argv.indexOf("--root") + 1] : "/Users/ueli/Documents/semio");
const target = process.argv[2].replace(/\/Cargo\.toml$/u, "");
const LIB = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🦀️resolution");
const {createCargoResolutionContextV1, cargoResolutionPartsV1, readCargoResolutionPolicyV1} = await import(join(LIB, "🟦️.ts"));
const policy = readCargoResolutionPolicyV1(JSON.parse(readFileSync(join(LIB, "🔣️.json"), "utf8")));
const toml = createRequire(join(root, "package.json"))("@iarna/toml").parse;
const touched = new Set();
const bytes = (path) => { touched.add(path); try { return readFileSync(join(root, path)); } catch (error) { if (error?.code === "ENOENT" || error?.code === "ENOTDIR") return null; throw error; } };
const sha = (buffer) => createHash("sha256").update(buffer).digest("hex");
const host = {read: async (path) => { const buffer = bytes(path); return buffer === null ? null : {value: toml(buffer.toString("utf8")), sha: sha(buffer)}; }, digest: async (path) => { const buffer = bytes(path); return buffer === null ? null : sha(buffer); }};
const parts = await cargoResolutionPartsV1(createCargoResolutionContextV1(host, policy), target === "" ? "Cargo.toml" : `${target}/Cargo.toml`, {tolerant: true});
process.stdout.write([...touched].filter((path) => { try { readFileSync(join(root, path)); return true; } catch { return false; } }).sort().join("\n") + "\n");
process.stderr.write(JSON.stringify({crate: target, parts, files: touched.size}) + "\n");
