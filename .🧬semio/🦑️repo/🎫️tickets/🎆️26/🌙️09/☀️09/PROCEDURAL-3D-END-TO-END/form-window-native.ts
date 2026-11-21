import { readdir, mkdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";

const checkout = process.cwd();
const cache = join(checkout, ".cache/b/r/m/u/l/p46a1ba/prototype/build");
const output = join(import.meta.dirname, "🗑️generated/form-window");
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
const [forms, windows, cases] = await Promise.all(["✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🪟️tree-windows/🦀️.rs","form-window-native.test.rs"].map((file, i) => readFile(join(i === 2 ? import.meta.dirname : checkout, file), "utf8")));
const states = windows.slice(windows.indexOf("#[derive(Clone, Debug, Default)]"), windows.indexOf("impl ShellDockState"));
const scheduler = windows.slice(windows.indexOf("pub fn tree_window_sendable_identifier"));
const helpers = forms.slice(forms.indexOf("fn ui_assembly_error"), forms.indexOf("/// 🖼️ Encodes one typed scene"));
const binding = forms.slice(forms.indexOf("fn generation_control_binding"), forms.indexOf("fn generation_control_args"));
const presenter = forms.slice(forms.indexOf("pub struct GenerationFormPage"), forms.indexOf("/// 📋️ Renders one record-budgeted page"));
const fixture = join(checkout, "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧫️fixtures/form-page-window.json");
const source = join(output, "actual.rs");
const witness = join(output, "form-window");
await writeFile(source, [
  'use semio_plugin::plugin_app_close_prelude::*;',
  'use semio_ui_locale::{Locale, Terminology};',
  'use std::collections::{BTreeMap, BTreeSet};',
  'use semio_ui_wgpu_light::tree_window::TreeWindowRequest as MeasuredWindow;',
  states,
  scheduler.replace('requests: &[TreeWindowRequest]', 'requests: &[MeasuredWindow]'),
  helpers,
  binding,
  presenter,
  'const FIXTURE: &str = include_str!(' + JSON.stringify(fixture) + ');',
  '#[path = '+JSON.stringify(join(checkout, "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🧪️fixture/🦀️.rs"))+'] mod ui_arena_fixture;',
  cases,
].join("\n"));
await run([...rustc, "--test", source,
  ...extern("semio-plugin", "ef0e149730ae97a3"),
  ...extern("semio-framework", "0867ff94b58fce9c"),
  ...extern("semio-framework-job", "689697170f9e2e90"),
  ...extern("semio-framework-job-testing-caller", "382719391b729040"),
  ...extern("semio-framework-value", "86eb0cf0fad7ae66"),
  ...extern("semio-ui-wgpu-light", "17c1f07f7fb2cc60"),
  ...extern("semio-ui-locale", "ade9a041184c270e"),
  ...extern("serde_json", "4e790474e9a23a5f"),
  ...cacheArgs, "-o", witness,
]);
await run([witness]);
