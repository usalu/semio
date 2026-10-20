// 🧪 Builds the dep-info overlay: the unit directories of one crate's test closure copied from the real tree and a framework workspace manifest filtered to them (read-only on the real repository).
import {readFileSync, writeFileSync, existsSync, mkdirSync, cpSync, rmSync} from "node:fs";
import {dirname, join} from "node:path";
import {readCargoResolutionPolicyV1, createCargoResolutionContextV1, owningWorkspaceV1, cargoDependenciesV1} from "/private/tmp/gk/sb/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🦀️resolution/🟦️.ts";
const REAL = "/Users/ueli/Documents/semio", OUT = "/private/tmp/gk/di/overlay", crate = process.argv[2]!;
rmSync(OUT, {recursive: true, force: true});
const cache = new Map<string, any>();
const read = async (p: string) => { if (!cache.has(p)) { try { cache.set(p, Bun.TOML.parse(readFileSync(join(REAL, p), "utf8"))); } catch { cache.set(p, null); } } return cache.get(p); };
const host = {read};
const closure = new Set<string>([crate]);
const visit = async (d: string, tests: boolean) => { const m = await read(`${d}/Cargo.toml`); const ws = await owningWorkspaceV1(host, d); for (const r of cargoDependenciesV1(d, m, ws, tests)) if (r.path !== undefined && !closure.has(r.path)) { closure.add(r.path); await visit(r.path, true); } };
await visit(crate, true);
// dev-dependencies of dependencies are never built for the root: closure above already follows only normal+build below the root
const units = new Set<string>();
for (const d of closure) { const i = d.indexOf("/📦️packages/"); units.add(i > 0 ? d.slice(0, i) : d); }
for (const extra of process.argv.slice(3)) units.add(extra);
const top = [...units].filter((u) => ![...units].some((v) => v !== u && u.startsWith(v + "/")));
for (const u of top) { mkdirSync(dirname(join(OUT, u)), {recursive: true}); cpSync(join(REAL, u), join(OUT, u), {recursive: true, filter: (s) => !s.includes("/node_modules") && !s.includes("/target/") && !s.endsWith("/Cargo.lock") }); }
const ws = await owningWorkspaceV1(host, crate);
const wsRoot = ws!.root, wsm = ws!.manifest as any;
const have = (p: string) => existsSync(join(OUT, wsRoot, p, "Cargo.toml"));
const toml = (v: any): string => typeof v === "string" ? JSON.stringify(v) : typeof v === "number" || typeof v === "boolean" ? String(v) : Array.isArray(v) ? `[${v.map(toml).join(", ")}]` : `{ ${Object.entries(v).map(([k, x]) => `${JSON.stringify(k)} = ${toml(x)}`).join(", ")} }`;
const members = [...closure].filter((c) => c.startsWith(wsRoot === "" ? "" : wsRoot + "/") || wsRoot === "").map((c) => (wsRoot === "" ? c : c.slice(wsRoot.length + 1))).filter(have);
const deps = Object.entries(wsm.workspace.dependencies).filter(([_k, v]: [string, any]) => typeof v !== "object" || !v.path || have(v.path));
let text = (await read(`${wsRoot ? wsRoot + "/" : ""}Cargo.toml`)) ? "" : "";
const features = (await read(`${wsRoot ? wsRoot + "/" : ""}Cargo.toml`))["cargo-features"];
text += features ? `cargo-features = ${toml(features)}\n\n` : "";
text += `[workspace]\nresolver = ${toml(wsm.workspace.resolver)}\nmembers = ${toml(members)}\n\n[workspace.package]\n${Object.entries(wsm.workspace.package).map(([k, v]) => `${k} = ${toml(v)}`).join("\n")}\n\n[workspace.dependencies]\n${deps.map(([k, v]) => `${JSON.stringify(k)} = ${toml(v)}`).join("\n")}\n\n${Object.entries(wsm.workspace.lints).map(([tool, lints]: [string, any]) => `[workspace.lints.${tool}]\n${Object.entries(lints).map(([k, v]) => `${JSON.stringify(k)} = ${toml(v)}`).join("\n")}\n`).join("\n")}`;
mkdirSync(join(OUT, wsRoot), {recursive: true});
writeFileSync(join(OUT, wsRoot, "Cargo.toml"), text);
cpSync(join(REAL, "rust-toolchain.toml"), join(OUT, "rust-toolchain.toml"));
console.log(JSON.stringify({crate, closure: closure.size, units: top.length, members: members.length, wsRoot}));
