import { readFileSync, writeFileSync } from "node:fs";

const path = "C:/git/semio/🧰️framework/🔨️modules/🖱️ui/⌨️tui/📟️vt/🧪️tests/🔬️unit/🦀️.rs";
let text = readFileSync(path, "utf8");

const helper = `/// 🥒️ Whether the first tag of \`feature\` names \`capability\`; the tests bind their feature file with \`include_str!\`, so renaming it breaks the build.
fn names_capability(feature: &str, capability: &str) -> bool {
    feature.lines().next().is_some_and(|line| line == format!("@capability-{capability}"))
}

#[test]
fn shared_streams_reproduce_every_screen_pyte_shows() {
`;
const bindings: Array<[string, string, string]> = [
  ["shared_streams_reproduce_every_screen_pyte_shows", "⌨️tui-terminal-streams", "tui-terminal-screen"],
  ["shared_keys_encode_to_the_bytes_terminfo_promises", "⌨️tui-terminal-keys", "tui-terminal-keys"],
  ["shared_pointer_events_encode_to_the_reports_prompt_toolkit_decodes", "⌨️tui-terminal-mouse", "tui-terminal-mouse"],
];

text = text.replace("#[test]\nfn shared_streams_reproduce_every_screen_pyte_shows() {\n", helper);
for (const [name, folder, capability] of bindings) {
  const head = `fn ${name}() {\n`;
  if (!text.includes(head)) throw new Error(name);
  text = text.replace(head, `${head}    assert!(names_capability(include_str!("../../../../🧪️tests/${folder}/🥒️.feature"), "${capability}"));\n`);
}
writeFileSync(path, text, "utf8");
console.log("bound");
