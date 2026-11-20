import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const source = dirname(fileURLToPath(import.meta.url));
const generated = join(source, "..", "🗑️generated", "native-actor");
const manifest = join(generated, "Cargo.toml");
const contents = await readFile(join(source, "Cargo.toml"), "utf8");
await mkdir(generated, { recursive: true });
await writeFile(manifest, contents.replaceAll('path = "../../', 'path = "../../../').replace('path = "probe.rs"', 'path = "../../native-actor/probe.rs"'));
const args = ["run", "--offline", "--manifest-path", manifest];
const command = process.argv.includes("--native") ? ["cargo", ...args] : ["repo", "tools", "exec", "cargo", "--", ...args];
const child = Bun.spawn(command, { stdout: "inherit", stderr: "inherit" });
process.exit(await child.exited);
