import { readdirSync, readFileSync, existsSync } from "node:fs";
import { join, relative, dirname } from "node:path";

const root = process.cwd();

interface CrateInfo {
  path: string;
  dir: string;
  pkgName: string;
  libName: string;
}

const crates: CrateInfo[] = [];

function walk(dir: string) {
  if (dir.includes("node_modules") || dir.includes("target") || dir.includes(".git") || dir.includes(".🦑️repo")) return;
  for (const ent of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, ent.name);
    if (ent.isDirectory()) walk(full);
    else if (ent.name === "Cargo.toml" && full !== join(root, "Cargo.toml")) {
      const content = readFileSync(full, "utf8");
      const pkgMatch = content.match(/\[package\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
      const libMatch = content.match(/\[lib\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
      if (pkgMatch) {
        const pkgName = pkgMatch[1];
        const libName = libMatch ? libMatch[1] : pkgName.replaceAll("-", "_");
        crates.push({
          path: full,
          dir: dirname(full),
          pkgName,
          libName,
        });
      }
    }
  }
}
walk(root);

console.log(`Indexed ${crates.length} crates.`);

const exactPkgNames = new Set(crates.map(c => c.pkgName));
const libNameToCrates = new Map<string, CrateInfo[]>();
const aliasToCrates = new Map<string, CrateInfo[]>();

function addAlias(alias: string, crate: CrateInfo) {
  if (!alias) return;
  const normalized = alias.replaceAll("-", "_").toLowerCase();
  const list = aliasToCrates.get(normalized) ?? [];
  if (!list.includes(crate)) list.push(crate);
  aliasToCrates.set(normalized, list);
}

const PREFIX_WORDS = new Set(["semio", "s", "framework", "os", "kernel", "plugin", "module", "tech", "app"]);

function generateVariants(name: string): string[] {
  const variants = new Set<string>([name]);
  const parts = name.split("-");
  
  let firstNonPrefix = parts.length - 1;
  for (let i = 0; i < parts.length; i++) {
    if (!PREFIX_WORDS.has(parts[i])) {
      firstNonPrefix = i;
      break;
    }
  }

  const coreSuffix = parts.slice(firstNonPrefix).join("-");
  variants.add(coreSuffix);

  const prefixParts = parts.slice(0, firstNonPrefix);
  for (let i = 0; i < prefixParts.length; i++) {
    variants.add(parts.slice(i).join("-"));
    variants.add([prefixParts[i], ...parts.slice(firstNonPrefix)].join("-"));
  }

  return Array.from(variants);
}

for (const c of crates) {
  const libList = libNameToCrates.get(c.libName) ?? [];
  if (!libList.includes(c)) libList.push(c);
  libNameToCrates.set(c.libName, libList);

  addAlias(c.libName, c);
  addAlias(c.pkgName, c);

  for (const variant of generateVariants(c.pkgName)) {
    addAlias(variant, c);
  }
  if (c.libName) {
    for (const variant of generateVariants(c.libName.replaceAll("_", "-"))) {
      addAlias(variant, c);
    }
  }
}

function resolvePackageName(pkg: string, cwd: string): string {
  if (exactPkgNames.has(pkg)) return pkg;

  // 1. Check local Cargo.toml in cwd, cwd/rs, dirname(cwd)
  for (const candidateDir of [cwd, join(cwd, "rs"), dirname(cwd)]) {
    const cargoPath = join(candidateDir, "Cargo.toml");
    if (existsSync(cargoPath)) {
      const content = readFileSync(cargoPath, "utf8");
      const pkgMatch = content.match(/\[package\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
      const libMatch = content.match(/\[lib\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
      const localPkg = pkgMatch ? pkgMatch[1] : null;
      const localLib = libMatch ? libMatch[1] : (localPkg ? localPkg.replaceAll("-", "_") : null);
      if (localPkg) {
        if (
          pkg === localLib ||
          pkg === localPkg ||
          pkg.replaceAll("-", "_") === localLib ||
          pkg.replaceAll("_", "-") === localPkg
        ) {
          return localPkg;
        }
      }
    }
  }

  // 2. Direct libName lookup
  const byLib = libNameToCrates.get(pkg);
  if (byLib && byLib.length > 0) {
    if (byLib.length === 1) return byLib[0].pkgName;
    let best = byLib[0];
    let bestDist = Infinity;
    for (const item of byLib) {
      const dist = relative(cwd, item.dir).length;
      if (dist < bestDist) {
        bestDist = dist;
        best = item;
      }
    }
    return best.pkgName;
  }

  // 3. Alias / prefix-variant lookup
  const normPkg = pkg.replaceAll("-", "_").toLowerCase();
  const byAlias = aliasToCrates.get(normPkg);
  if (byAlias && byAlias.length > 0) {
    if (byAlias.length === 1) return byAlias[0].pkgName;
    let best = byAlias[0];
    let bestDist = Infinity;
    for (const item of byAlias) {
      const dist = relative(cwd, item.dir).length;
      if (dist < bestDist) {
        bestDist = dist;
        best = item;
      }
    }
    return best.pkgName;
  }

  // 4. Fallback if cwd has Cargo.toml
  for (const candidateDir of [cwd, join(cwd, "rs")]) {
    const cargoPath = join(candidateDir, "Cargo.toml");
    if (existsSync(cargoPath)) {
      const content = readFileSync(cargoPath, "utf8");
      const pkgMatch = content.match(/\[package\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
      if (pkgMatch) return pkgMatch[1];
    }
  }

  return pkg;
}

const testFailedIds = [
  { pkg: "db_actor", cwd: join(root, "🧰️framework/🛍️product/💻️os/🔨️module/🛢️db/🎭️actor/⚡️implementation/🦀️rust") },
  { pkg: "architect_spine", cwd: join(root, "✏️s/🔌️plugin/🏛️architect/🔨️module/🦴️spine/⚡️implementation/🦀️rust") },
  { pkg: "energy_engine", cwd: join(root, "✏️s/🔌️plugin/🔋️energy/🔨️module/⚙️engine/⚡️implementation/🦀️rust") },
  { pkg: "kernel_3d_scene", cwd: join(root, "🧰️framework/🛍️product/💻️os/🔨️module/🧊️3d/🎬️scene/⚡️implementation/🦀️rust") },
  { pkg: "mathematical_graph_manifest", cwd: root },
  { pkg: "framework_editor", cwd: root },
  { pkg: "framework_surface_node_graph", cwd: root },
  { pkg: "ui_wgpu", cwd: root },
  { pkg: "fem_core", cwd: root },
  { pkg: "norm_core", cwd: root },
  { pkg: "animate_core", cwd: root },
  { pkg: "db", cwd: root },
  { pkg: "vcs", cwd: root },
  { pkg: "repo_cli", cwd: root },
  { pkg: "store", cwd: root },
  { pkg: "trinity_jack_lsp", cwd: root },
  { pkg: "pack_cli", cwd: root },
  { pkg: "db_core", cwd: root },
  { pkg: "db_state", cwd: root },
  { pkg: "db_index", cwd: root },
];

console.log("\nTesting Resolution:");
for (const test of testFailedIds) {
  const resolved = resolvePackageName(test.pkg, test.cwd);
  console.log(`  "${test.pkg}" -> "${resolved}"`);
}
