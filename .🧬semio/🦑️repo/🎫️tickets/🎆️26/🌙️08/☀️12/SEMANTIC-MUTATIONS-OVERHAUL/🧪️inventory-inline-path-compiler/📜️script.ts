import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";

const root = mkdtempSync(join(tmpdir(), "semio-inventory-inline-path-"));
const write = (path: string, source: string): void => { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, source); };
try {
  mkdirSync(join(root, "📦️crate", "subsets"), { recursive: true });
  write(join(root, "📦️crate", "📦️glue.rs"), "#[path = \".\"] pub mod standards { pub mod subsets { #[path = \".\"] pub mod schema { #[path = \"../../mutations/insert.rs\"] pub mod insert_page; } } }\npub mod command;\n");
  write(join(root, "📦️crate", "command.rs"), "use crate::standards::subsets::schema::insert_page::Mutation;\npub fn command(_: Mutation) {}\n");
  write(join(root, "mutations", "insert.rs"), "pub struct Mutation;\n");
  const result = Bun.spawnSync(["rustc", "--crate-name", "probe", "--crate-type", "lib", "📦️glue.rs", "-o", "out.rlib"], { cwd: join(root, "📦️crate"), stdout: "pipe", stderr: "pipe" });
  console.log(`[DEBUG] ${JSON.stringify({ exitCode: result.exitCode, stdout: new TextDecoder().decode(result.stdout), stderr: new TextDecoder().decode(result.stderr) })}`);
  if (result.exitCode !== 0) process.exitCode = 1;
} finally {
  rmSync(root, { recursive: true, force: true });
}
