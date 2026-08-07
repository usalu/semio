import fs from "fs";
import path from "path";

const ticket = "/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS";
const flowRoot = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
const coreName = fs.readdirSync(flowRoot).find((n) => /core/.test(n));
const coreFile = path.join(flowRoot, coreName, fs.readdirSync(path.join(flowRoot, coreName)).find((n) => n.includes("component")));
const core = fs.readFileSync(coreFile, "utf8").split("\n");

const needles = ["PendingExtension", "seed_flow_eval", "InvokeExtension", "pending_extension", "ContributedExtension", "finish_pending", "extension_id"];
for (const n of needles) {
  const hits = [];
  core.forEach((l, i) => {
    if (l.includes(n)) hits.push(i + 1);
  });
  console.log(n, hits.slice(0, 20).join(","), "total", hits.length);
}

// Dump EvalBridge region
const evalBridge = core.findIndex((l) => l.includes("#region") && l.includes("EvalBridge"));
console.log("EvalBridge at", evalBridge + 1);
fs.writeFileSync(path.join(ticket, "core-eval-bridge.rs"), core.slice(evalBridge, evalBridge + 200).join("\n"));

// Check hex column example
function walk(dir, pred, out = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory() && !["node_modules", "target"].includes(e.name)) walk(p, pred, out);
    else if (pred(e.name, p)) out.push(p);
  }
  return out;
}
const hex = walk("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural", (n, p) => p.includes("hex") && n.endsWith(".semio"));
console.log("hex files", hex);
for (const f of hex) {
  const t = fs.readFileSync(f, "utf8");
  const kinds = [...t.matchAll(/"kind"\s*:\s*"([^"]+)"/g)].map((m) => m[1]);
  const kinds2 = [...t.matchAll(/brep\.[a-zA-Z0-9.]+/g)].map((m) => m[0]);
  console.log(f);
  console.log("  kinds", [...new Set(kinds2)].slice(0, 30));
}

// Sample one example for structure
const sample = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/box-fillet-preview/🗣️dsls/♻️reuse/🧬️component.procedural.procedural3d.dsl.semio";
const st = fs.readFileSync(sample, "utf8");
fs.writeFileSync(path.join(ticket, "sample-box-fillet.semio"), st.slice(0, 3000));
const kinds = [...new Set([...st.matchAll(/brep\.[a-zA-Z0-9.]+/g)].map((m) => m[0]))];
console.log("box-fillet kinds", kinds);

// Check wave2 summary if any for how invoke-backed ops work
const summaries = fs.readdirSync(ticket).filter((n) => n.includes("wave2") || n.includes("summary"));
console.log("summaries", summaries);

// BIM note: extension_id wire id
const bim = fs.readFileSync(path.join(ticket, "bim-component.rs"), "utf8");
console.log("bim extension_id strings:", [...bim.matchAll(/extension_id:\s*"([^"]+)"/g)].map((m) => m[1]));
console.log("bim ExtensionBundle::new:", [...bim.matchAll(/ExtensionBundle::new\("([^"]+)"/g)].map((m) => m[1]));

// Check if there's already a GeometryKernel or similar in core near end of file for insertion point
console.log("core file ends with regions:");
core.forEach((l, i) => {
  if (l.includes("#region") || l.includes("#endregion")) {
    if (i > 7500 || i < 50) console.log(i + 1, l);
  }
});

// Check register line 5866 context - fixture_kind_infos
fs.writeFileSync(path.join(ticket, "core-fixture-register.rs"), core.slice(5840, 5900).join("\n"));
