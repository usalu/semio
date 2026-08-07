/**
 * Wave 3.c — clean 3d TS stale wasm imports, document examples, polish.
 */
import fs from "fs";
import path from "path";

const REPO = "/Users/ueli/Documents/semio";
const TICKET = path.join(REPO, ".🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS");

function findFile(dir, pred) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory() && !["node_modules", "target"].includes(e.name)) {
      const hit = findFile(p, pred);
      if (hit) return hit;
    } else if (pred(p, e.name)) return p;
  }
  return null;
}

// --- TS index: remove stale flow_extension_brep.js loaders ---
{
  const tsRoot = path.join(REPO, "✏️s/🔨️modules");
  const d3 = path.join(tsRoot, fs.readdirSync(tsRoot).find((n) => n.includes("3d")));
  const index = findFile(path.join(d3, "📦️packages", "🟦️typescript"), (p, n) => n === "index.ts" || n.includes("index.ts"));
  // prefer 📦️index.ts
  const idxFile =
    findFile(path.join(d3, "📦️packages", "🟦️typescript"), (p, n) => n.includes("index") && n.endsWith(".ts") && !n.includes("vitest")) ||
    index;
  console.log("ts index", idxFile);
  let text = fs.readFileSync(idxFile, "utf8");
  fs.writeFileSync(path.join(TICKET, "3d-index-before.ts"), text);

  // Replace wasm loader functions with clear errors pointing at extension install / flow core tessellate export
  // Find loadBrepWasm / similar regions by searching for flow_extension_brep
  if (!text.includes("flow_extension_brep")) {
    console.log("SKIP ts already clean");
  } else {
    // Soften: replace import paths with throw documenting removal
    text = text.replace(
      /const mod = \(await import\("[^"]*flow_extension_brep\.js"\)\)[\s\S]*?;/g,
      `throw new Error("flow_extension_brep wasm pack removed — use flow core tessellate export or install the packaged brep extension");`,
    );
    // More careful line-based cleanup for the two loader functions
    const lines = text.split("\n");
    const out = [];
    let skippingImportBlock = false;
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      if (line.includes("flow_extension_brep.js") || line.includes("flow_extension_brep_bg.wasm")) {
        // replace whole statement lines that import the stale module
        if (line.includes("await import")) {
          out.push(`    throw new Error("stale flow_extension_brep wasm pack removed (Wave 3.c); use host flow tessellate or the packaged brep extension");`);
          // skip continuation lines until semicolon-only balance — naive: skip until line with ;
          while (i + 1 < lines.length && !lines[i].includes(";")) {
            i++;
          }
          continue;
        }
        if (line.includes("readFileSync") && line.includes("flow_extension_brep")) {
          continue; // drop
        }
        if (line.includes("import(") && line.includes("flow_extension_brep")) {
          out.push(`    throw new Error("stale flow_extension_brep wasm pack removed (Wave 3.c); use host flow tessellate or the packaged brep extension");`);
          continue;
        }
      }
      out.push(line);
    }
    text = out.join("\n");
    // Comment the alias @semio-tech/flow-module-brep if present
    fs.writeFileSync(idxFile, text);
    console.log("OK patched 3d index (pass1)");
  }

  const vitest = findFile(path.join(d3, "📦️packages", "🟦️typescript"), (p, n) => n.includes("vitest") && n.endsWith(".ts"));
  console.log("vitest", vitest);
  if (vitest) {
    let v = fs.readFileSync(vitest, "utf8");
    if (v.includes("flow-module-brep") || v.includes("flow_extension_brep")) {
      v = v.replace(/\s*"@semio-tech\/flow-module-brep":\s*resolve\([^)]+\),?\n/, "\n");
      fs.writeFileSync(vitest, v);
      console.log("OK removed vitest alias");
    }
  }
}

// --- Document procedural3d examples ---
{
  const examples = [];
  function walk(dir) {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = path.join(dir, e.name);
      if (e.isDirectory() && !["node_modules", "target"].includes(e.name)) walk(p);
      else if (e.name.endsWith(".semio")) {
        const t = fs.readFileSync(p, "utf8");
        if (t.includes("brep.")) examples.push(p);
      }
    }
  }
  walk(path.join(REPO, "✏️s/🔌️plugins/🌀️procedural"));
  const note = `# Procedural3d examples requiring the Brep flow extension

These \`.semio\` graphs use \`brep.*\` operator kinds. After Wave 3.c those operators are **not** compile-time builtins.

They work when the packaged extension \`semio-s-plugin-flow-extension-brep\` (\`flow-extension-brep\` / manifest id \`brep\`) is **installed and enabled** for hosts \`flow-play\` and \`procedural3d-play\` (dual \`Contribution::FlowExtension\`).

Operator kind ids are unchanged (\`brep.prim3d.box\`, \`brep.solid.fillet\`, …) — no graph rewrites required; contribution registration supplies the same kinds at runtime.

## Graphs

${examples.map((p) => `- \`${path.relative(REPO, p)}\``).join("\n")}
`;
  fs.writeFileSync(path.join(TICKET, "brep-examples-require-extension.md"), note);
  console.log("documented", examples.length, "examples");

  // Also drop a short AGENTS-adjacent note next to examples reuse folder if a README exists — user said no extra files outside ticket. Ticket only.
}

// --- Fix procedural3d silly comment ---
{
  const eng = path.join(
    REPO,
    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts",
    fs.readdirSync(path.join(REPO, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts")).find((n) => n.includes("procedural3d")),
    "⚙️engine",
  );
  const engFile = path.join(eng, fs.readdirSync(eng).find((n) => n.includes("component")));
  let t = fs.readFileSync(engFile, "utf8");
  t = t.replace(
    "use flow_extension_brep::tessellate_geometry; // crate-root re-export via flow alias",
    "use flow_extension_brep::tessellate_geometry;",
  );
  fs.writeFileSync(engFile, t);
  console.log("OK cleaned p3d import");
}

// --- Update flow cargo description ---
{
  const p = JSON.parse(fs.readFileSync(path.join(TICKET, "wave3c-paths.json"), "utf8"));
  let cargo = fs.readFileSync(p.flowCargo, "utf8");
  cargo = cargo.replace(
    /description = "[^"]*"/,
    'description = "OS flow family — core + brep geometry session; operators (light/draw/brep/bim) are packaged extensions"',
  );
  fs.writeFileSync(p.flowCargo, cargo);
  console.log("OK flow cargo description");
}

console.log("phase C done");
