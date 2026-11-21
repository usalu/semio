import { readdir, mkdir } from "node:fs/promises";
import { join } from "node:path";

const checkout = process.cwd();
const cache = join(checkout, ".cache/b/r/m/u/l/p46a1ba/prototype/build");
const output = join(import.meta.dirname, "🗑️generated/form-native");
await mkdir(output, { recursive: true });
const libraries = (await Promise.all((await readdir(cache, { withFileTypes: true })).filter((entry) => entry.isDirectory()).map(async ({ name }) =>
  (await readdir(join(cache, name), { withFileTypes: true })).filter((entry) => entry.isDirectory()).map(({ name: id }) => join(cache, name, id, "out"))))).flat();
const cacheArgs = libraries.flatMap((dir) => ["-L", "dependency=" + dir]);
const extern = (name, id) => {
  const crate = name.replaceAll("-", "_");
  return ["rlib", "rmeta"].flatMap((suffix) => ["--extern", crate + "=" + join(cache, name, id, "out", "lib" + crate + "-" + id + "." + suffix)]);
};
const rustc = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2024"];
const run = async (argv) => {
  const command = Bun.spawn(argv, { stdout: "inherit", stderr: "inherit" });
  const status = await command.exited;
  if (status !== 0) process.exit(status);
};
const builder = join(output, "libsemio_ui_contract_builder.rlib");
await run([...rustc, "--crate-name", "semio_ui_contract_builder", "--crate-type", "rlib",
  join(checkout, "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs"),
  ...extern("serde", "03d5f757c21f115c"),
  ...extern("semio-ui-contract-components", "6fe9d8115a153db4"),
  ...extern("semio-ui-contract-values", "7dbb40f340a78b53"),
  ...extern("semio-ui-contract-budget", "11b2083363fb9352"),
  ...extern("semio-framework-value", "6ed1f1f2355b9a7c"),
  ...cacheArgs, "-o", builder,
]);
const witness = join(output, "form-witness");
await run([...rustc, "--test", join(import.meta.dirname, "form-native-witness.rs"),
  "--extern", "semio_ui_contract_builder=" + builder, ...cacheArgs, "-o", witness]);
await run([witness]);
