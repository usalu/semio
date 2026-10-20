// Scratch driver (bun): plans the Go or Python units of a checkout read-only and prints numbers.
// usage: bun units-cli.ts <module-dir> plan go|python <repo-root> [--out file.json] [--roots a,b]
// <module-dir> is the overlay copy of L/🏗️build/🧩️units (see overlay.sh).
import {spawnSync} from "node:child_process";
import {readFileSync, writeFileSync} from "node:fs";
import {join} from "node:path";

const [moduleDir, command, language, root, ...rest] = process.argv.slice(2);
if (!moduleDir || command !== "plan" || !language || !root) {console.error("usage: bun units-cli.ts <module-dir> plan go|python <repo-root> [--out file] [--roots a,b]"); process.exit(2);}
const units = await import(join(moduleDir, "🟦️.ts"));
const policy = units.readUnitsPolicyV1(JSON.parse(readFileSync(join(moduleDir, "🔣️.json"), "utf8")));
const option = (name: string): string | undefined => {const at = rest.indexOf(name); return at < 0 ? undefined : rest[at + 1];};
const patterns = language === "go" ? ["*.go", "*go.mod", "go.work", "*.c", "*.h", "*.s"] : ["*.py", "*.pyi"];
const tracked = spawnSync("git", ["ls-files", "-z", "--", ...patterns], {cwd: root, encoding: "utf8", maxBuffer: 512 * 1024 * 1024}).stdout.split("\0").filter(Boolean);
const cache = new Map<string, string>();
const read = (path: string): string => {let text = cache.get(path); if (text === undefined) {text = readFileSync(join(root, path), "utf8"); cache.set(path, text);} return text;};
const started = performance.now();
let graph, extra: Record<string, unknown> = {};
if (language === "go") {
  const defaults = spawnSync("go", ["env", "GOOS", "GOARCH", "CGO_ENABLED"], {encoding: "utf8"}).stdout.trim().split("\n");
  const minor = Number(/go1\.(\d+)/.exec(spawnSync("go", ["version"], {encoding: "utf8"}).stdout)?.[1] ?? 25);
  const planned = units.planGoUnitsV1({files: tracked, read}, {policy, context: {goos: defaults[0], goarch: defaults[1], goMinor: minor, cgo: defaults[2] === "1", tags: []}, toolchain: `go1.${minor} ${defaults[0]}/${defaults[1]}`, tests: true});
  graph = planned.graph; extra = {packages: planned.packages.length};
} else {
  const planned = units.planPythonUnitsV1({files: tracked, read}, {policy, roots: (option("--roots") ?? "").split(",").filter((value, index) => value !== "" || index === 0), toolchain: "python3", pins: {}});
  graph = planned.graph; extra = {modules: planned.modules.length, lexicalErrors: Object.keys(planned.lexicalErrors).length};
}
const plan = units.planUnitsV1(units.readUnitGraphV1(graph, policy));
const edges = graph.units.reduce((sum: number, unit: {dependencies: unknown[]}) => sum + unit.dependencies.length, 0);
console.log(JSON.stringify({language, files: tracked.length, units: graph.units.length, edges, layers: Math.max(...Object.values(plan.layer) as number[]) + 1, externals: new Set(graph.units.flatMap((unit: {externals: string[]}) => unit.externals)).size, milliseconds: Math.round(performance.now() - started), ...extra}));
const out = option("--out");
if (out) writeFileSync(out, JSON.stringify(graph));
