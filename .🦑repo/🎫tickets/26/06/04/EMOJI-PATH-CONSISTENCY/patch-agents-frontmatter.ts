import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../../");

const technologyEmoji: Record<string, string> = {
  compose: "🏘️",
  ui: "🖱️",
  framework: "🥅",
  cad: "📐",
  coda: "🔬",
  repo: "🧰",
  puzzle: "🧩",
  infinite: "♾️",
  gis: "🌐",
  mathematical: "🧮",
  reasoning: "🧠",
};

const areaPath: Record<string, string> = {
  "compose/client/lib/sketchpad/js": "🏘️compose✍️sketchpad",
  "ui/react": "🖱️ui⚛️react",
  "gis/map": "🌐gis📍map",
  "gis/terrain": "🌐gis⛰️terrain",
  "puzzle/3d": "🧩puzzle🏙️3d",
  "puzzle/5d": "🧩puzzle👯5d",
  "infinite/cavas": "♾️infinite✈️cavas",
  "infinite/world": "♾️infinite🏙️world",
  "reasoning/mindmap": "🧠reasoning🗺️mindmap",
  "reasoning/mindmap/wires": "🧠reasoning🔗wires",
  "mathematical/graph/port/directed": "🧮mathematical⭕graphs",
  "mathematical/graph/port/undirected": "🧮mathematical⭕graphs",
  "mathematical/graph/normal/directed": "🧮mathematical⭕graphs",
  "mathematical/graph/normal/undirected": "🧮mathematical⭕graphs",
};

const bundleEmojiFix: Record<string, string> = {
  "compose/client/lib/sketchpad/js": "✍️",
  "compose/client/ui/vscode": "🖱️",
};

const subBundleEmoji: Record<string, { name: string; emoji: string }> = {
  "gis/map": { name: "map", emoji: "📍" },
  "gis/terrain": { name: "terrain", emoji: "⛰️" },
  "puzzle/3d": { name: "3d", emoji: "📷" },
  "puzzle/5d": { name: "5d", emoji: "👯" },
  "infinite/cavas": { name: "cavas", emoji: "✈️" },
  "infinite/world": { name: "world", emoji: "🏙️" },
  "ui/react": { name: "react", emoji: "⚛️" },
};

function rel(p: string): string {
  return p.slice(root.length + 1);
}

function upsertFrontmatter(body: string, fields: Record<string, string | { name: string; emoji: string; description?: string; kind?: string }>): string {
  const hasFm = body.startsWith("---\n");
  const end = hasFm ? body.indexOf("\n---\n", 4) : -1;
  const content = hasFm && end > 0 ? body.slice(end + 5) : body;
  const lines: string[] = ["---"];
  for (const [k, v] of Object.entries(fields)) {
    if (typeof v === "string") {
      lines.push(`${k}: ${v}`);
      continue;
    }
    lines.push(`${k}:`);
    for (const [bk, bv] of Object.entries(v)) {
      lines.push(` ${bk}: ${bv}`);
    }
  }
  lines.push("---", "");
  return `${lines.join("\n")}${content.replace(/^\n+/, "")}`;
}

function patchTechnologyAgents(relPath: string, text: string): string | null {
  const name = relPath.split("/")[0];
  const emoji = technologyEmoji[name];
  if (!emoji) return null;
  if (text.startsWith("---")) {
    let next = text.replace(/^---\nemoji: .*\n/m, `---\nemoji: ${emoji}\n`);
    next = next.replace(/^---\ntechnology: spatial\n/m, `---\ntechnology: cad\nemoji: ${emoji}\n`);
    if (name === "ui" && !next.includes("\nemoji:")) {
      next = next.replace(/^---\ntechnology: ui\n/m, `---\ntechnology: ui\nemoji: ${emoji}\n`);
    }
    if (name === "framework" && !next.includes("\nemoji:")) {
      next = next.replace(/^---\ntechnology: framework\n/m, `---\ntechnology: framework\nemoji: ${emoji}\n`);
    }
    return next;
  }
  return upsertFrontmatter(text, { emoji });
}

function patchAreaAgents(relPath: string, text: string): string | null {
  const path = areaPath[relPath];
  if (!path) return null;
  const tech = relPath.split("/")[0];
  const sub = subBundleEmoji[relPath];
  const bundleEmoji = bundleEmojiFix[relPath];
  const fields: Record<string, string | object> = {
    technology: tech,
    path,
  };
  if (sub) {
    fields.bundle = {
      name: sub.name,
      emoji: bundleEmoji ?? sub.emoji,
      description: `${sub.name} area for ${tech}.`,
      kind: "library",
    };
  }
  if (text.startsWith("---")) {
    let out = text;
    if (!out.includes("\npath:")) {
      out = out.replace(/^(---\ntechnology: [^\n]+\n)/m, `$1path: ${path}\n`);
    } else {
      out = out.replace(/^path: .*\n/m, `path: ${path}\n`);
    }
    if (bundleEmoji) {
      out = out.replace(/^(\s+emoji: ).*$/m, `$1${bundleEmoji}`);
    }
    return out;
  }
  return upsertFrontmatter(text, fields);
}

const techRoots = new Set(Object.keys(technologyEmoji));
const created: string[] = [];
const updated: string[] = [];

for (const tech of ["gis", "mathematical"]) {
  const p = join(root, tech, "AGENTS.md");
  const body = `# ${tech === "gis" ? "Gis" : "Mathematical"}\n\n`;
  writeFileSync(p, upsertFrontmatter(body, { emoji: technologyEmoji[tech]! }));
  created.push(rel(p));
}

function walk(dir: string) {
  for (const name of readdirSync(dir)) {
    if (name === "node_modules" || name === "target" || name === ".git") continue;
    const full = join(dir, name);
    const r = rel(full);
    if (name === "AGENTS.md") {
      const text = readFileSync(full, "utf8");
      let out: string | null = null;
      if (techRoots.has(r.split("/")[0]) && r.split("/").length === 2) {
        out = patchTechnologyAgents(r, text);
      } else if (areaPath[r]) {
        out = patchAreaAgents(r, text);
      }
      if (out && out !== text) {
        writeFileSync(full, out);
        updated.push(r);
      }
      continue;
    }
    if (statSync(full).isDirectory()) walk(full);
  }
}

walk(root);

console.log(JSON.stringify({ created, updated }, null, 2));
