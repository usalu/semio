import { readFileSync, readdirSync, writeFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";

const OS = readFileSync("/tmp/os-path.txt", "utf8").trim();
const TICKET = readFileSync("/tmp/os-ticket-path.txt", "utf8").trim();
const tsvName = readdirSync(TICKET).find((f) => f.endsWith("all-crates.tsv"));
const crates = readFileSync(join(TICKET, tsvName), "utf8")
  .trim()
  .split("\n")
  .map((line) => {
    const [name, path] = line.split("\t");
    return { name, path };
  });

const KERNEL_PREFIXES = [
  "semio-framework-os-kernel-store",
  "semio-framework-os-kernel-protocol",
  "semio-framework-os-kernel-dsl",
  "semio-framework-os-kernel-pack",
  "semio-framework-os-kernel-infinite",
  "semio-framework-os-kernel-flow-core",
  "semio-framework-kernel-infinite-world",
];

function isKernel(name) {
  return KERNEL_PREFIXES.some((p) => name === p || name.startsWith(p + "-"));
}

const kernel = crates.filter((c) => isKernel(c.name));
const reports = [];
for (const c of kernel) {
  const dir = dirname(c.path);
  const candidates = [join(dir, "📦️lib.rs"), join(dir, "lib.rs"), join(dir, "src/lib.rs")];
  // also find any *lib.rs
  for (const f of readdirSync(dir)) {
    if (f.endsWith("lib.rs")) candidates.push(join(dir, f));
  }
  const libPath = candidates.find((p) => existsSync(p));
  let head = "";
  let bytes = 0;
  let mods = [];
  let pathAttrs = [];
  if (libPath) {
    const text = readFileSync(libPath, "utf8");
    bytes = Buffer.byteLength(text);
    head = text.split("\n").slice(0, 80).join("\n");
    mods = [...text.matchAll(/^\s*(?:pub\s+)?mod\s+(\w+)/gm)].map((m) => m[1]);
    pathAttrs = [...text.matchAll(/#\[path\s*=\s*"([^"]+)"\]/g)].map((m) => m[1]);
  }
  const cargo = readFileSync(c.path, "utf8");
  const pkgDeps = [...cargo.matchAll(/^([A-Za-z0-9_-]+)\s*=\s*\{[^\n]*package\s*=\s*"([^"]+)"/gm)].map(
    (m) => ({ alias: m[1], package: m[2] }),
  );
  const hasTargetCfg = /\[target\./.test(cargo);
  const crateType = [...cargo.matchAll(/crate-type\s*=\s*\[([^\]]+)\]/g)].map((m) => m[1]);
  reports.push({
    name: c.name,
    cargo: c.path,
    libPath: libPath ?? null,
    bytes,
    mods,
    pathAttrs,
    pkgDeps,
    hasTargetCfg,
    crateType,
    head,
  });
}

const invName = "kernel-inventory.json";
const sumName = "kernel-summary.txt";
// prefer emoji-prefixed scratch names if convention
const invPath = join(TICKET, "\u{1F9EA}".length ? "\u{1F9EA}kernel-inventory.json".replace("\\u{1F9EA}", "🧪") : invName);
writeFileSync(join(TICKET, "🧪kernel-inventory.json"), JSON.stringify(reports, null, 2));
writeFileSync(
  join(TICKET, "🧪kernel-summary.txt"),
  [
    "kernel crates: " + reports.length,
    ...reports.map(
      (r) =>
        r.name +
        "\tbytes=" +
        r.bytes +
        "\tmods=" +
        r.mods.length +
        "\tdeps=" +
        r.pkgDeps.length +
        "\tlib=" +
        (r.libPath ? "yes" : "NO") +
        "\ttargetCfg=" +
        r.hasTargetCfg,
    ),
  ].join("\n") + "\n",
);
console.log("wrote inventory for " + reports.length + " kernel crates");
for (const r of reports) {
  console.log(r.name + " bytes=" + r.bytes + " mods=" + r.mods.join(",") + " paths=" + r.pathAttrs.length);
}
