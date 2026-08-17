import { readdirSync, statSync, readFileSync, writeFileSync, existsSync } from "fs";
import { join } from "path";

const repo = "/Users/ueli/Documents/semio";

function findChild(parent, substr) {
  for (const name of readdirSync(parent)) {
    const p = join(parent, name);
    try {
      if (statSync(p).isDirectory() && name.includes(substr)) return p;
    } catch {}
  }
  return null;
}

function findNamed(root, name, maxDepth = 6) {
  const out = [];
  function walk(d, depth) {
    if (depth > maxDepth) return;
    let kids;
    try { kids = readdirSync(d); } catch { return; }
    for (const n of kids) {
      const p = join(d, n);
      let st;
      try { st = statSync(p); } catch { continue; }
      if (!st.isDirectory()) continue;
      if (n === name) out.push(p);
      else walk(p, depth + 1);
    }
  }
  walk(root, 0);
  return out;
}

const fw = findChild(repo, "framework");
const mods = findChild(fw, "modules");
const ui = findChild(mods, "ui");
const el = findChild(ui, "elements");
const packages = findChild(ui, "packages");
const ts = findChild(packages, "typescript");
const targets = findChild(ts, "targets");
const react = findChild(targets, "react");
const barrel = join(react, readdirSync(react).find((n) => n.includes("index") && n.endsWith(".tsx")));

const ticketCands = findNamed(join(repo, ".🦑️repo"), "UI-ELEMENT-CO-LOCATION-RESTRUCTURE", 8);
const ticket = ticketCands.map((p) => ({ p, n: readdirSync(p).length })).sort((a, b) => b.n - a.n)[0].p;

const yarn = String.fromCodePoint(0x1f9f6) + String.fromCodePoint(0xfe0f);
const outPath = join(ticket, `${yarn}w6-core-icons-button-context.txt`);

const lines = readFileSync(barrel, "utf8").split("\n");

const dirNames = ["Button","Icons","ContextMenu","DragHandle","ButtonGroup","PanelTabBar","Avatar","Panel","Label"];
const dirInfo = {};
for (const name of dirNames) {
  const p = join(el, name);
  dirInfo[name] = existsSync(p) ? readdirSync(p).sort() : null;
}
const core = findChild(el, "core");
const coreKids = core ? readdirSync(core).sort() : [];

const regionLines = [];
for (let i = 0; i < lines.length; i++) {
  const l = lines[i];
  if (/^\/\/\s*#region\b/.test(l) || /^\/\/\s*#endregion\b/.test(l)) {
    if (/Button|Icon|Context|Drag|Handle|PanelTab|Label|Avatar|Group/i.test(l)) {
      regionLines.push(`${i + 1}:${l}`);
    }
  }
}

const symbols = [
  "Button","ButtonGroup","ButtonGroupItem","ContextMenu","ContextMenuItem","DragHandle",
  "Icon","ControlIcon","renderControlIcon","IconSource","IconName",
  "CheckIcon","ChevronDownIcon","ChevronUpIcon","ChevronLeftIcon","ChevronRightIcon",
  "CloseIcon","Maximize2Icon","Minimize2Icon","ChevronDownIconAlt",
  "PanelTabBar","PanelTabNode","findPanelTabNode","progressPanelTabSelection","usePanelTabSelection",
  "Label","useLabel",
];

function findHits(sym) {
  const pats = [
    new RegExp(`^export (const|function|type|interface|class|let|enum) ${sym}\\b`),
    new RegExp(`^export \\{[^}]*\\b${sym}\\b`),
    new RegExp(`^const ${sym}\\b`),
    new RegExp(`^function ${sym}\\b`),
    new RegExp(`^type ${sym}\\b`),
    new RegExp(`^interface ${sym}\\b`),
    new RegExp(`^import \\{[^}]*\\b${sym}\\b`),
  ];
  const hits = [];
  for (let i = 0; i < lines.length; i++) {
    if (pats.some((p) => p.test(lines[i]))) hits.push([i + 1, lines[i].slice(0, 180)]);
  }
  return hits;
}

const status = {};
for (const sym of symbols) {
  const hits = findHits(sym);
  const inline = hits.filter((h) => /^export (const|function|type|interface|class|let|enum)/.test(h[1]));
  const bare = hits.filter((h) => /^(const|function|type|interface) /.test(h[1]));
  const imp = hits.filter((h) => h[1].startsWith("import {") && h[1].includes("elements/"));
  let kind = "NOT_FOUND";
  if (inline.length || bare.length) kind = "STILL_IN_BARREL";
  else if (imp.length) kind = "EXTRACTED_REEXPORT";
  else if (hits.length) kind = "BARREL_REFERENCE_ONLY";
  status[sym] = [kind, hits.slice(0, 6)];
}

function componentFile(dir) {
  if (!existsSync(dir)) return null;
  return readdirSync(dir).map((n) => join(dir, n)).find((p) => p.endsWith(".tsx") && p.includes("component"));
}

const iconsDir = join(el, "Icons");
const iconsComp = componentFile(iconsDir);
const iconsExports = iconsComp ? readFileSync(iconsComp, "utf8").split("\n").filter((l) => l.startsWith("export")) : [];
const iconsPreview = iconsComp ? readFileSync(iconsComp, "utf8").split("\n").slice(0, 50).join("\n") : "";

const buttonDir = join(el, "Button");
const buttonFiles = existsSync(buttonDir) ? readdirSync(buttonDir).sort() : [];
const buttonTsx = buttonFiles.some((n) => n.endsWith(".tsx"));
const buttonRs = buttonFiles.some((n) => n.endsWith(".rs"));

function leafText(name) {
  const comp = componentFile(join(el, name));
  return comp ? readFileSync(comp, "utf8") : "";
}
const avT = leafText("Avatar");
const pnT = leafText("Panel");
const avEx = avT.split("\n").filter((l) => l.startsWith("export"));
const pnEx = pnT.split("\n").filter((l) => l.startsWith("export") && /Tab|Panel/.test(l));

function regionSpan(substr) {
  for (let i = 0; i < lines.length; i++) {
    if (/^\/\/\s*#region\b/.test(lines[i]) && lines[i].includes(substr)) {
      let depth = 0;
      for (let j = i; j < lines.length; j++) {
        if (/^\/\/\s*#region\b/.test(lines[j])) depth++;
        else if (/^\/\/\s*#endregion\b/.test(lines[j])) {
          depth--;
          if (depth === 0) return [i + 1, j + 1, j - i + 1];
        }
      }
    }
  }
  return null;
}

const spans = {};
for (const key of ["Icons","ContextMenu","Avatar","Button","ButtonGroup","DragHandle","PanelTab","Label","Icon "]) {
  spans[key] = regionSpan(key);
}

const buttonRegionCandidates = [];
const dragRegionCandidates = [];
const panelTabRegionCandidates = [];
const groupRegionCandidates = [];
for (let i = 0; i < lines.length; i++) {
  const l = lines[i];
  if (!/^\/\/\s*#region\b/.test(l)) continue;
  if (/Button/.test(l)) buttonRegionCandidates.push(`${i + 1}:${l}`);
  if (/Drag|Handle/i.test(l)) dragRegionCandidates.push(`${i + 1}:${l}`);
  if (/PanelTab|TabBar/.test(l)) panelTabRegionCandidates.push(`${i + 1}:${l}`);
  if (/ButtonGroup|Group/.test(l)) groupRegionCandidates.push(`${i + 1}:${l}`);
}

const storyHits = {};
const storyNames = ["Button","ButtonGroup","ContextMenu","Label","PanelTabBar","Icons","DragAndDrop"];
function walkStories(d, depth = 0) {
  if (depth > 8 || !existsSync(d)) return;
  for (const n of readdirSync(d)) {
    const p = join(d, n);
    let st;
    try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) walkStories(p, depth + 1);
    else {
      for (const name of storyNames) {
        if (n === `${name}.stories.tsx`) {
          storyHits[name] = storyHits[name] || [];
          storyHits[name].push(p.replace(repo + "/", ""));
        }
      }
    }
  }
}
walkStories(join(repo, ".storybook"));

const fanout = {};
function walkEl(d, depth = 0) {
  if (depth > 6) return;
  for (const n of readdirSync(d)) {
    const p = join(d, n);
    let st;
    try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) walkEl(p, depth + 1);
    else if (n.endsWith(".tsx") || n.endsWith(".ts")) {
      const t = readFileSync(p, "utf8");
      if (!t.includes("W3-interim")) continue;
      for (const s of symbols) {
        if (new RegExp(`\\b${s}\\b`).test(t)) fanout[s] = (fanout[s] || 0) + 1;
      }
    }
  }
}
walkEl(el);

const R = [];
R.push("# w6-core-icons-button-context inventory (PAUSE — no barrel edits)");
R.push(`generated: ${new Date().toISOString()}`);
R.push(`ticket: ${ticket}`);
R.push(`elements: ${el}`);
R.push(`barrel: ${barrel} (lines=${lines.length})`);
R.push("barrel lock: HELD BY PARENT — this session did NOT edit index.tsx");
R.push("");
R.push("## Element directories");
for (const name of ["Button","Icons","ContextMenu","DragHandle","ButtonGroup","PanelTabBar","Avatar","Panel"]) {
  const files = dirInfo[name];
  if (files == null) R.push(`- ${name}/: MISSING`);
  else R.push(`- ${name}/: EXISTS files=[${files.join(", ")}]`);
}
R.push(`- core/: [${coreKids.join(", ")}]`);
R.push(`- Button has .tsx component: ${buttonTsx}; has .rs: ${buttonRs}; files=[${buttonFiles.join(", ")}]`);
R.push(`- Icons files: [${(dirInfo.Icons || []).join(", ")}]`);
R.push("");
R.push("## Icons leaf preview / exports");
R.push(iconsPreview);
R.push("--- exports ---");
for (const e of iconsExports) R.push(`  ${e}`);
R.push("");
R.push("## Avatar drag-related");
R.push(`- Avatar component present: ${!!avT}`);
R.push(`- contains DragHandle/DraggableAvatar: ${/DragHandle|DraggableAvatar/.test(avT)}`);
for (const e of avEx) if (/Drag|Avatar/.test(e)) R.push(`  ${e}`);
R.push("");
R.push("## Panel tab-related");
R.push(`- Panel component present: ${!!pnT}`);
R.push(`- contains PanelTabBar/PanelTabNode: ${/PanelTabBar|PanelTabNode/.test(pnT)}`);
for (const e of pnEx.slice(0, 40)) R.push(`  ${e.slice(0, 180)}`);
R.push("");
R.push("## Barrel regions matching targets");
for (const r of regionLines) R.push(`  ${r}`);
R.push("");
R.push("## Region span estimates (stack-matched)");
for (const [k, v] of Object.entries(spans)) R.push(`  ${k}: ${JSON.stringify(v)}`);
R.push("Button region candidates:");
for (const c of buttonRegionCandidates) R.push(`  ${c}`);
R.push("Drag/Handle region candidates:");
for (const c of dragRegionCandidates) R.push(`  ${c}`);
R.push("PanelTab region candidates:");
for (const c of panelTabRegionCandidates) R.push(`  ${c}`);
R.push("Group region candidates:");
for (const c of groupRegionCandidates.slice(0, 40)) R.push(`  ${c}`);
R.push("");
R.push("## Symbol residence");
R.push("STILL_IN_BARREL = export/const/function/type definition still in barrel body");
R.push("EXTRACTED_REEXPORT = barrel import {...} from elements/... then export");
for (const sym of symbols) {
  const [kind, hits] = status[sym];
  R.push(`- ${sym}: ${kind}`);
  for (const [ln, tx] of hits.slice(0, 5)) R.push(`    L${ln}: ${tx}`);
}
R.push("");
R.push("## Storybook story locations");
for (const n of Object.keys(storyHits).sort()) {
  for (const p of storyHits[n]) R.push(`- ${n}: ${p}`);
}
R.push("");
R.push("## W3-interim fanout (element leaves still importing these from barrel)");
for (const [s, c] of Object.entries(fanout).sort((a, b) => b[1] - a[1])) {
  R.push(`  ${c}\t${s}`);
}
R.push("");
R.push("## Verdict (inventory only — waiting for barrel lock)");
R.push("1. Button/: EXISTS but only Rust component.rs — NO react component.tsx. See Button symbol status → extract when lock free.");
R.push("2. Icons/: EXISTS with react component + story; currently Cursor-only (see exports). Icon/ControlIcon/renderControlIcon/IconSource/IconName/individual icons still barrel-resident in Icons region → EXTEND Icons/ when lock free. Prefer co-locate under Icons/; core/Icons only if module-top-level cycle break needed.");
R.push("3. ContextMenu/: MISSING. ContextMenu region exists in barrel → CREATE dir + EXTRACT when lock free.");
R.push("4. DragHandle/: MISSING as own dir. See Avatar DraggableAvatar + barrel DragHandle status → place with Avatar or create DragHandle when lock free.");
R.push("5. ButtonGroup/: MISSING. See ButtonGroup/ButtonGroupItem status → CREATE + EXTRACT when lock free if still inline.");
R.push("6. PanelTabBar/: MISSING as own dir. See Panel leaf + barrel PanelTab* status before deciding Panel vs own element.");
R.push("");
R.push("STOPPING per parent instruction. No edits to ui-react barrel.");

writeFileSync(outPath, R.join("\n") + "\n");
console.log("WROTE", outPath);
console.log("dirs", JSON.stringify(dirInfo, null, 2));
console.log("buttonTsx", buttonTsx, "buttonRs", buttonRs);
console.log("iconsExports", iconsExports);
for (const sym of ["Button","Icon","ControlIcon","renderControlIcon","IconSource","IconName","CheckIcon","ContextMenu","ContextMenuItem","DragHandle","ButtonGroup","ButtonGroupItem","PanelTabBar","PanelTabNode","Label"]) {
  console.log(`  ${sym}: ${status[sym][0]}`);
}
console.log("spans", JSON.stringify(spans));
