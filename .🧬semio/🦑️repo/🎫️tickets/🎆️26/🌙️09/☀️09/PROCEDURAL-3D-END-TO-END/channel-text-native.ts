import { readdir, mkdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";

const checkout = process.cwd();
const cache = join(checkout, ".cache/b/r/m/u/l/p46a1ba/prototype/build");
const output = join(import.meta.dirname, "🗑️generated/channel-text");
await mkdir(output, { recursive: true });
const libraries = (await Promise.all((await readdir(cache, { withFileTypes: true })).filter((entry) => entry.isDirectory()).map(async ({ name }) =>
  (await readdir(join(cache, name), { withFileTypes: true })).filter((entry) => entry.isDirectory()).map(({ name: id }) => join(cache, name, id, "out"))))).flat();
const cacheArgs = libraries.flatMap((dir) => ["-L", "dependency=" + dir]);
const extern = (name, id) => {
  const crate = name.replaceAll("-", "_");
  return ["rlib", "rmeta"].flatMap((suffix) => ["--extern", crate + "=" + join(cache, name, id, "out", "lib" + crate + "-" + id + "." + suffix)]);
};
const rustc = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2024"];
const run = async (argv, env = process.env) => {
  const command = Bun.spawn(argv, { env, stdout: "inherit", stderr: "inherit" });
  const status = await command.exited;
  if (status !== 0) process.exit(status);
};
const semantic = join(checkout, "✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui");
const source = await readFile(join(semantic, "🦀️.rs"), "utf8");
const arena = join(checkout, "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🧪️fixture/🦀️.rs");
const channel = join(semantic, "📊️channel-values/🦀️.rs");
const fixture = join(semantic, "📊️channel-values/🧪️tests/🔣️.json");
const keyFixture = join(semantic, "📊️channel-values/🧪️tests/📛️keys.json");
const cases = await readFile(join(import.meta.dirname, "channel-text-native.test.rs"), "utf8");
const witnessSource = join(output, "actual.rs");
const witness = join(output, "channel-text");
await writeFile(witnessSource, [
  'use semio_plugin::plugin_app_close_prelude::*;',
  'use semio_ui_locale::{Locale, Terminology};',
  source.slice(source.indexOf("fn ui_assembly_error"), source.indexOf("/// 🖼️ Encodes one typed scene")),
  '#[path = '+JSON.stringify(arena)+'] mod ui_arena_fixture;',
  '#[path = '+JSON.stringify(channel)+'] mod channel_values;',
  cases.replace("PREVIEW_FIXTURE", JSON.stringify(fixture)).replace("KEY_FIXTURE", JSON.stringify(keyFixture)),
].join("\n"));
await run([...rustc, "--test", witnessSource, "-C", "strip=debuginfo",
  ...extern("semio-plugin", "ef0e149730ae97a3"),
  ...extern("semio-framework", "0867ff94b58fce9c"),
  ...extern("semio-framework-job", "689697170f9e2e90"),
  ...extern("semio-framework-job-testing-caller", "382719391b729040"),
  ...extern("semio-framework-value", "86eb0cf0fad7ae66"),
  ...extern("semio-framework-pack-json", "50749e9bbc0c0f7c"),
  ...extern("semio-ui-locale", "ade9a041184c270e"),
  ...extern("semio-ui-scene", "7ccb916b92f1a066"),
  ...extern("serde_json", "4e790474e9a23a5f"),
  ...cacheArgs, "-o", witness,
]);
await run([witness, "--test-threads=1"], { ...process.env, SEMIO_CHANNEL_PREVIEW_BROWSER_TREE: join(output, "snapshots.json") });
