import { readdir, mkdir } from "node:fs/promises";
import { join } from "node:path";

const checkout = process.cwd();
const cache = join(checkout, ".cache/b/r/m/u/l/p46a1ba/prototype/build");
const output = join(import.meta.dirname, "🗑️generated/field-native");
await mkdir(output, { recursive: true });
const libraries = (await Promise.all((await readdir(cache, { withFileTypes: true })).filter((entry) => entry.isDirectory()).map(async ({ name }) =>
  (await readdir(join(cache, name), { withFileTypes: true })).filter((entry) => entry.isDirectory()).map(({ name: id }) => join(cache, name, id, "out"))))).flat();
const cacheArgs = libraries.flatMap((dir) => ["-L", "dependency=" + dir]);
const extern = (name, id, alias = name.replaceAll("-", "_")) => {
  const crate = name.replaceAll("-", "_");
  return ["rlib", "rmeta"].flatMap((suffix) => ["--extern", alias + "=" + join(cache, name, id, "out", "lib" + crate + "-" + id + "." + suffix)]);
};
const rustc = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2024"];
const run = async (argv) => {
  const command = Bun.spawn(argv, { stdout: "inherit", stderr: "inherit" });
  const status = await command.exited;
  if (status !== 0) process.exit(status);
};
const contract = extern("semio-ui-contract", "486106d692c3fe02", "ui_contract");
const retained = extern("semio-ui-retained-tree", "3d24f176c1e8cfad");
const owner = join(output, "libfield_projection.rlib");
await run([...rustc, "--crate-name", "field_projection", "--crate-type", "rlib",
  join(checkout, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs"),
  ...contract, ...retained,
  ...extern("semio-ui-declaration", "34f9e0188ba5d281"),
  ...extern("semio-ui-wgpu-layout", "3336fb0d58304c22"),
  ...cacheArgs, "-o", owner,
]);
const witness = join(output, "field-witness");
await run([...rustc, "--test", join(import.meta.dirname, "field-native-witness.rs"),
  "--extern", "field_projection=" + owner, ...contract, ...retained,
  ...extern("serde_json", "4e790474e9a23a5f"),
  ...cacheArgs, "-L", "dependency=" + output, "-o", witness]);
await run([witness]);
