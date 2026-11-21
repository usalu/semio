import { readdir, mkdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";

const checkout = process.cwd();
const buildDir = process.argv[2] ?? join(checkout, ".cache/b/r/m/u/l/p46a1ba");
const jsonHash = process.argv[3] ?? "50749e9bbc0c0f7c";
const cache = join(buildDir, "prototype/build");
const output = join(import.meta.dirname, "🗑️generated/preview-channel-history");
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
const geometry = await readFile(join(checkout, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧊️preview-geometry/🦀️.rs"), "utf8");
const preview = await readFile(join(checkout, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs"), "utf8");
const graphPorts = join(checkout, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔌️graph-ports/🦀️.rs");
const fixture = join(checkout, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🧫️fixtures/preview-value-paths.json");
const tests = "#[test]\nfn resolves_the_exact_prior_channel_direction_and_path() {\n    use semio_framework_pack_json::{object, parse, JsonMemberPolicy, Value};\n    let fixture = parse(include_str!(HISTORY_FIXTURE), JsonMemberPolicy::Reject).unwrap();\n    let widget_value = |side: &str, value: &Value| (side.to_owned(), object([(\"body\".to_owned(), value.clone())]));\n    for case in fixture[\"cases\"].as_array().unwrap() {\n        let previous_side = case[\"previousSide\"].as_str().unwrap_or(\"out\");\n        let current_side = case[\"currentSide\"].as_str().unwrap_or(\"out\");\n        let peer_side = if previous_side == \"out\" { \"in\" } else { \"out\" };\n        let mut sides = vec![widget_value(previous_side, &case[\"previous\"])];\n        if let Some(peer) = case.get(\"previousPeer\") { sides.push(widget_value(peer_side, peer)); }\n        let previous = object([(\"widget\".to_owned(), object(sides))]);\n        let current = object([(\"widget\".to_owned(), object([widget_value(current_side, &case[\"current\"])]))]);\n        let items = preview_channel_items_for_widget(&current, \"widget\");\n        assert_eq!(items.len(), case[\"expected\"].as_object().unwrap().len(), \"{}\", case[\"name\"]);\n        for item in items {\n            let expected = case[\"expected\"].get(item.handle.as_str()).unwrap_or_else(|| panic!(\"{}: unexpected {}\", case[\"name\"], item.handle));\n            assert_eq!(preview_history_handle(&previous, \"widget\", &item), expected.as_str(), \"{}: {}\", case[\"name\"], item.handle);\n        }\n    }\n}\n";
const witnessSource = join(output, "actual.rs");
const witness = join(output, "channel-history");
await writeFile(witnessSource, [
  "#[path = " + JSON.stringify(graphPorts) + "] mod semio_s_artifact_procedural_3d_model;",
  preview.slice(preview.indexOf("/// 🎯️ Raw IDs decoded once"), preview.indexOf("/// 🔌️ Geometry endpoints reached")),
  geometry.slice(geometry.indexOf("pub fn is_brep_geometry_handle"), geometry.indexOf("/// 🪪️ Preview-enabled widgets")),
  tests.replace("HISTORY_FIXTURE", JSON.stringify(fixture)),
].join("\n"));
await run([...rustc, "--test", witnessSource, "-C", "strip=debuginfo",
  ...extern("semio-framework-pack-json", jsonHash),
  ...cacheArgs, "-o", witness,
]);
await run([witness]);
