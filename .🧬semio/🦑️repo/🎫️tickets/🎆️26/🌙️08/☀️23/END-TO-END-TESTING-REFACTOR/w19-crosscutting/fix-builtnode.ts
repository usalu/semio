import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust";
const list = readFileSync(
  "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w19-crosscutting/builtnode-files.txt",
  "utf8",
).split("\n").filter(Boolean);

let changed = 0;
for (const rel of list) {
  const path = resolve(ROOT, rel);
  const before = readFileSync(path, "utf8");
  let s = before;

  s = s.replace(/^([ \t]*)use semio_framework_plugin::UiNode;\n([ \t]*)use super::\*;/m, "$1use semio_framework_plugin::Component;\n$1use super::*;");

  s = s.replace(
    /( *)let UiNode::ComponentScene\(node\) = (.+?) else \{ panic!\("expected ComponentScene"\) \};\n *let scene = node\.text_editor\.expect\("text.editor scene"\);/g,
    '$1let node = $2.expect("render");\n$1let Component::Surface(props) = node.component else { panic!("expected a retained text surface") };\n$1let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(&props).expect("decode text scene");',
  );

  s = s.replace(
    /( *)let UiNode::ComponentScene\(node\) = (.+?) else \{ panic!\("expected ComponentScene"\) \};\n *let scene = node\.table\.expect\("table scene"\);/g,
    '$1let node = $2.expect("render");\n$1let Component::Surface(props) = node.component else { panic!("expected a retained table surface") };\n$1let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(&props).expect("decode table scene");',
  );

  s = s.replace(/( *)let UiNode::Stack\((\w+)\) = (.+?) else \{ panic!\("expected Stack"\) \};/g, '$1let $2 = $3.expect("render");');

  s = s.replace(
    /( *)let UiNode::Tree\(node\) = (.+?) else \{ panic!\("expected Tree"\) \};\n *let root = &node\.sections\[0\]\.items\[0\];/g,
    '$1let node = $2.expect("render");\n$1let section = node.children.get(0).expect("tree section");\n$1let root = section.children.get(0).expect("tree root");',
  );

  s = s.replace(/&(\w+)\.items\.as_ref\(\)\.unwrap\(\)\[(\d+)\]/g, '$1.children.get($2).expect("child")');
  s = s.replace(/let (\w+) = (\w+)\.items\.as_ref\(\)\.expect\("root has children"\);/g, "let $1 = &$2.children;");
  s = s.replace(/assert_eq!\((root|child|a|item0)\.id, /g, "assert_eq!($1.key.as_str(), ");
  s = s.replace(/assert_eq!\((children)\[(\d+)\]\.id, /g, 'assert_eq!($1.get($2).expect("child").key.as_str(), ');

  if (s !== before) {
    writeFileSync(path, s);
    changed++;
  }
}
console.log(`rewrote ${changed} of ${list.length}`);
