/**
 * 🐹️ Graph-free canonical Go test run of one module inside a mirror (C/mirror-tools/mirror.py): the canonical plan of the MIRROR's go-plan library builds the overlay, `go test -overlay` runs the module with the mirror's go.work, no network.
 * usage: bun go-mirror.ts --root <mirror-root> --module <repo-relative module dir> [--run <regex>] [--short] [--tmp inside|os] [--log <file>]
 * `--tmp inside` (default) puts TMPDIR and SEMIO_TEST_ARTIFACT_DIR below the mirror root (a git repository of its own), the situation of the official route; the run then prints every path the run added below `.🧬semio` of the mirror (the real-checkout pollution oracle: files created or rewritten anywhere in the mirror outside the scratch dir).
 */
import { spawnSync } from "node:child_process";
import { globSync, mkdirSync, readFileSync, realpathSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const args = process.argv.slice(2);
const flag = (name: string, fallback = ""): string => { const at = args.indexOf(name); return at < 0 ? fallback : args[at + 1] ?? fallback; };
const root = realpathSync(resolve(flag("--root")));
const moduleRoot = join(root, flag("--module"));
const { canonicalGoPlan } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts"));
const plan = canonicalGoPlan(moduleRoot);
const scratch = flag("--tmp", "inside") === "inside" ? join(root, ".tp-scratch") : realpathSync(process.env.TMPDIR ?? "/tmp");
mkdirSync(scratch, { recursive: true });
const overlay = join(scratch, "go-overlay.json");
writeFileSync(overlay, JSON.stringify({ Replace: plan.replacements }));
const artifacts = join(scratch, "artifacts");
mkdirSync(artifacts, { recursive: true });
const goWork = readFileSync(join(root, "go.work"), "utf8");
const inWork = [...goWork.matchAll(/^\t\.\/(.+)$/gm)].some((match) => join(root, match[1]!) === moduleRoot);
const live = resolve(import.meta.dir, "../../../../../../../..");
const goHome = globSync(join(live, ".cache/b/t/m/native/go/*/*/bin/go"))[0]?.slice(0, -"/bin/go".length);
if (!goHome) throw new Error("pinned Go missing below the live .cache (read-only use)");
const env = { ...process.env, PATH: `${goHome}/bin:${process.env.PATH}`, GOROOT: goHome, GOTOOLCHAIN: "local", GOPROXY: "off", GOFLAGS: "-mod=readonly", GOWORK: inWork ? join(root, "go.work") : "off", TMPDIR: scratch, SEMIO_TEST_ARTIFACT_DIR: artifacts };
const marker = join(scratch, "marker");
writeFileSync(marker, "");
const changed = (): string[] => spawnSync("find", [".", "-type", "f", "-newer", marker, "-not", "-path", "./.tp-scratch/*", "-not", "-path", "./.git/*", "-not", "-path", "*/node_modules/*", "-not", "-path", "./.🧬semio/🦑️repo/⚡️cache/*", "-not", "-path", "*/target/*"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\n").filter(Boolean);
const goArgs = ["test", `-overlay=${overlay}`, "-count=1", ...(args.includes("--short") ? ["-short"] : []), ...(flag("--run") ? ["-run", flag("--run")] : []), ...plan.packages];
const result = spawnSync("nice", ["-n", "10", "go", ...goArgs], { cwd: moduleRoot, env, encoding: "utf8", maxBuffer: 1 << 28 });
const text = `${result.stdout}${result.stderr}`;
if (flag("--log")) writeFileSync(flag("--log"), text);
const lines = text.split("\n");
console.log(lines.filter((line) => /^(ok|FAIL|--- FAIL|panic|\?)\s/.test(line) || /^FAIL$/.test(line)).join("\n"));
const added = changed();
console.log(`[tp-go] exit=${result.status} packages=${plan.packages.length} files-written-in-the-checkout=${added.length}${added.length ? "\n" + added.slice(0, 20).join("\n") : ""}`);
process.exit(result.status ?? 1);
