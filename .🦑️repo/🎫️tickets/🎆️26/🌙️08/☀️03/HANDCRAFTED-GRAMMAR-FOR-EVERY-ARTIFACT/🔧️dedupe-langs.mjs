import { readFileSync, writeFileSync, readdirSync } from "fs";
import { join } from "path";

function facetFile(pl, art, facet) {
  const pluginsRoot = readdirSync(".").find((x) => x.endsWith("s") && x.length <= 4);
  const plugins = join(pluginsRoot, readdirSync(pluginsRoot).find((x) => x.includes("plugins")));
  const plugin = readdirSync(plugins).find((x) => x.includes(pl));
  const artifacts = readdirSync(join(plugins, plugin)).find((x) => x.includes("artifacts"));
  const artifact = readdirSync(join(plugins, plugin, artifacts)).find((x) => x.includes(art));
  const facetDir = readdirSync(join(plugins, plugin, artifacts, artifact)).find((x) => x.includes(facet));
  return join(plugins, plugin, artifacts, artifact, facetDir, "🦀️component.rs");
}

function stripFn(t, name) {
  const re = new RegExp(`\\nfn ${name}\\(\\) \\{[\\s\\S]*?\\n\\}\\n`);
  if (re.test(t)) return { t: t.replace(re, "\n"), removed: true };
  const re2 = new RegExp(`fn ${name}\\(\\) \\{[\\s\\S]*?\\n\\}\\n`);
  if (re2.test(t)) return { t: t.replace(re2, "\n"), removed: true };
  return { t, removed: false };
}

function dedupeEngine(path, myFnName, idFixes = {}) {
  let t = readFileSync(path, "utf8");
  t = t.replace(new RegExp(`\\s*${myFnName}\\(\\);\\n`, "g"), "\n");
  const r = stripFn(t, myFnName);
  t = r.t;
  console.log(path, "removed", myFnName, r.removed);

  if (t.includes("fn pilot_language_hooks")) {
    t = t.replace(/\nfn pilot_language_hooks\([\s\S]*?\n\}\n/, "\n");
    t = t.replace(/pilot_language_hooks\(/g, "dsl::passthrough_hooks(");
    console.log("  switched to passthrough_hooks");
  }

  for (const [from, to] of Object.entries(idFixes)) {
    const before = t;
    t = t.replaceAll(`id: "${from}"`, `id: "${to}"`);
    t = t.replaceAll(`dsl::passthrough_hooks("${from}")`, `dsl::passthrough_hooks("${to}")`);
    if (before !== t) console.log("  id", from, "->", to);
  }

  writeFileSync(path, t);
}

dedupeEngine(facetFile("dag", "dag", "engine"), "register_artifact_languages", {
  dag: "dag.document",
  "dag.ops": "dag.op",
});
dedupeEngine(facetFile("note", "note", "engine"), "register_note_languages", {
  note: "note.document",
  "note.ops": "note.op",
});
dedupeEngine(facetFile("fem", "2d", "engine"), "register_fem2d_languages", {
  fem2d: "fem.fem2d",
  "fem2d.ops": "fem.fem2d.op",
  "fem2d.diff": "fem.fem2d.diff",
  "fem2d.pack": "2d.pack",
  "fem2d.spr": "2d.spr",
});
dedupeEngine(facetFile("fem", "3d", "engine"), "register_fem3d_languages", {
  fem3d: "fem.fem3d",
  "fem3d.ops": "fem.fem3d.op",
  "fem3d.diff": "fem.fem3d.diff",
  "fem3d.pack": "3d.pack",
  "fem3d.spr": "3d.spr",
});

{
  const path = facetFile("writer", "writer", "engine");
  let t = readFileSync(path, "utf8");
  const count = (t.match(/id: "writer\.document"/g) || []).length;
  console.log("writer.document count", count);
  if (count > 1) {
    const jackEnd = t.indexOf('id: "jack"');
    const afterJack = t.indexOf("hooks: jack_hooks,", jackEnd);
    const close = t.indexOf("});", afterJack);
    const rest = t.slice(close + 3);
    const firstWriterDoc = rest.indexOf('id: "writer.document"');
    const secondWriterDoc = rest.indexOf('id: "writer.document"', firstWriterDoc + 1);
    if (firstWriterDoc >= 0 && secondWriterDoc >= 0) {
      const absSecond = close + 3 + secondWriterDoc;
      const start = t.lastIndexOf("dsl::register_language", absSecond);
      const spr = t.indexOf('id: "writer.spr"', absSecond);
      const sprClose = t.indexOf("});", t.indexOf("hooks:", spr)) + 3;
      t = t.slice(0, start) + t.slice(sprClose);
      console.log("removed duplicate writer LanguageSpec series");
    }
  }
  if (t.includes("fn pilot_language_hooks")) {
    t = t.replace(/\nfn pilot_language_hooks\([\s\S]*?\n\}\n/, "\n");
    t = t.replace(/pilot_language_hooks\(/g, "dsl::passthrough_hooks(");
  }
  t = t.replaceAll('id: "writer"', 'id: "writer.document"');
  t = t.replaceAll('dsl::passthrough_hooks("writer")', 'dsl::passthrough_hooks("writer.document")');
  t = t.replaceAll('id: "writer.ops"', 'id: "writer.op"');
  t = t.replaceAll('dsl::passthrough_hooks("writer.ops")', 'dsl::passthrough_hooks("writer.op")');
  writeFileSync(path, t);
}

for (const row of [["dag", "dag"], ["note", "note"], ["fem", "2d"], ["fem", "3d"], ["writer", "writer"]]) {
  const t = readFileSync(facetFile(row[0], row[1], "engine"), "utf8");
  console.log(
    row.join("/"),
    "register_language",
    (t.match(/register_language/g) || []).length,
    "dupFns",
    /register_artifact_languages|register_note_languages|register_fem2d_languages|register_fem3d_languages/.test(t),
  );
  console.log((t.match(/pub fn register\(\) \{[\s\S]*?\n\}/) || [])[0]);
  const ids = [...t.matchAll(/id: "([^"]+)"/g)].map((m) => m[1]).filter((id) => !["jack"].includes(id) || true);
  console.log("  ids", ids.filter((id, i, a) => a.indexOf(id) === i));
}
