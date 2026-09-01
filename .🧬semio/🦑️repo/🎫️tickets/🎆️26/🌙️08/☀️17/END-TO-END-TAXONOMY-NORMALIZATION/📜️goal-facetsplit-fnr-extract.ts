// 📜️ Facet-split restoration for 🏗️fem / 📸️remodel / 🗒️note — undoes the inlining of
// mutation `🔺️diff` / `↩️inverse` bodies back into their direct `🦀️.rs` leaf.
// Ground truth shape: ✏️s/🔌️plugins/🌿️vcs/…/🧬️mutations/🏷️add-tag/
// Authoritative extracted-code source: commit bb06c41f73f0122fbed315b7487428b976f99921
import { execSync } from "child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "fs";

const REPO = "/Users/ueli/Documents/semio";
const COMMIT = "bb06c41f73f0122fbed315b7487428b976f99921";
const PLUGINS = ["🏗️fem", "📸️remodel", "🗒️note"];

function sh(cmd: string): string {
  return execSync(cmd, { cwd: REPO, maxBuffer: 1024 * 1024 * 64, encoding: "utf8" });
}

function gitShow(path: string): string | null {
  try {
    return execSync(`git show ${COMMIT}:"${path.replace(/"/g, '\\"')}"`, { cwd: REPO, maxBuffer: 1024 * 1024 * 16, encoding: "utf8" });
  } catch {
    return null;
  }
}

// 🔎️ Corrected predicate from coordinator: a direct leaf is inlined when it defines
// `pub (async )?fn diff(` with no sibling 🔺️diff/ dir, or `pub (async )?fn inverse(`
// with no sibling ↩️inverse/ dir.
function discoverWorklist(): string[] {
  const out: string[] = [];
  for (const plugin of PLUGINS) {
    const listed = sh(`git ls-files '✏️s/🔌️plugins/${plugin}'`).split("\n").filter(Boolean);
    for (const f of listed) {
      if (!/\/🧬️schema\/🧬️mutations\/[^/]+\/🦀️\.rs$/.test(f)) continue;
      const abs = `${REPO}/${f}`;
      if (!existsSync(abs)) continue;
      const content = readFileSync(abs, "utf8");
      const hasDiffFn = /^\s*pub (async )?fn diff\(/m.test(content);
      const hasInvFn = /^\s*pub (async )?fn inverse\(/m.test(content);
      const dir = f.slice(0, f.length - "🦀️.rs".length - 1);
      const hasDiffDir = existsSync(`${REPO}/${dir}/🔺️diff`);
      const hasInvDir = existsSync(`${REPO}/${dir}/↩️inverse`);
      if ((hasDiffFn && !hasDiffDir) || (hasInvFn && !hasInvDir)) out.push(f);
    }
  }
  return out;
}

function stripRegion(content: string, outerName: string, innerName: string): string {
  const outerWithInner = new RegExp(`\\n*//#region ${outerName}\\n\\n?//#region ${innerName}\\n[\\s\\S]*?//#endregion ${innerName}\\n//#endregion ${outerName}\\n*`);
  const innerOnly = new RegExp(`\\n*//#region ${innerName}\\n[\\s\\S]*?//#endregion ${innerName}\\n*`);
  if (outerWithInner.test(content)) return content.replace(outerWithInner, "\n\n");
  if (innerOnly.test(content)) return content.replace(innerOnly, "\n\n");
  throw new Error(`no ${innerName} region found`);
}

function fixMutationPath(content: string): string {
  return content.split("::mutation::").join("::");
}

function relFromPlugin(plugin: string, path: string): string {
  const prefix = `✏️s/🔌️plugins/${plugin}/`;
  if (!path.startsWith(prefix)) throw new Error(`unexpected prefix: ${path}`);
  return path.slice(prefix.length);
}

const report: string[] = [];
let processed = 0;
let skipped: string[] = [];

for (const plugin of PLUGINS) {
  const worklist = discoverWorklist().filter((f) => f.startsWith(`✏️s/🔌️plugins/${plugin}/`));
  const glueRelPath = `✏️s/🔌️plugins/${plugin}/📦️packages/🦀️rust/📦️glue.rs`;
  let glue = readFileSync(`${REPO}/${glueRelPath}`, "utf8");

  for (const leafRel of worklist) {
    const dirRel = leafRel.slice(0, leafRel.length - "/🦀️.rs".length);
    const dirAbs = `${REPO}/${dirRel}`;
    const leafAbs = `${REPO}/${leafRel}`;
    let content = readFileSync(leafAbs, "utf8");

    const diffFacetRel = `${dirRel}/🔺️diff/🦀️component.rs`;
    const invFacetRel = `${dirRel}/↩️inverse/🦀️component.rs`;
    const pinnedDiff = gitShow(diffFacetRel);
    const pinnedInv = gitShow(invFacetRel);
    if (pinnedDiff === null || pinnedInv === null) {
      skipped.push(`${leafRel} (missing pinned original: diff=${pinnedDiff !== null} inverse=${pinnedInv !== null})`);
      continue;
    }

    // 1️⃣ write facet files from the pinned original, fixing the removed `mutation` submodule path shift
    mkdirSync(`${dirAbs}/🔺️diff`, { recursive: true });
    mkdirSync(`${dirAbs}/↩️inverse`, { recursive: true });
    writeFileSync(`${dirAbs}/🔺️diff/🦀️component.rs`, fixMutationPath(pinnedDiff));
    writeFileSync(`${dirAbs}/↩️inverse/🦀️component.rs`, fixMutationPath(pinnedInv));

    // 2️⃣ strip the inlined bodies from the direct leaf
    content = stripRegion(content, "🔺️diff", "🔖️Diff");
    content = stripRegion(content, "↩️inverse", "🔖️Inverse");
    content = content.replace(/\n{3,}/g, "\n\n").replace(/\n*$/, "\n");

    // 3️⃣ delegate the impl's dispatch calls to the facet modules
    if (!content.includes("diff(self, base)") || !content.includes("inverse(self, base)")) {
      skipped.push(`${leafRel} (unexpected call-site shape after strip)`);
      continue;
    }
    content = content.replace("diff(self, base)", "super::diff::diff(self, base)");
    content = content.replace("inverse(self, base)", "super::inverse::inverse(self, base)");
    writeFileSync(leafAbs, content);

    // 4️⃣ mount the facet modules in glue.rs beside the existing `mod component;`
    const relLeaf = "../../" + relFromPlugin(plugin, leafRel);
    const relDiff = "../../" + relFromPlugin(plugin, diffFacetRel);
    const relInv = "../../" + relFromPlugin(plugin, invFacetRel);
    const anchorRe = new RegExp(`( *)#\\[path = "${relLeaf.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}"\\]\\n`);
    const m = glue.match(anchorRe);
    if (!m) {
      skipped.push(`${leafRel} (glue.rs mount anchor not found: ${relLeaf})`);
      continue;
    }
    if (glue.includes(`#[path = "${relDiff}"]`)) {
      // already mounted (idempotent re-run)
    } else {
      const indent = m[1];
      const insertion = `${indent}#[path = "${relDiff}"]\n${indent}pub mod diff;\n${indent}#[path = "${relInv}"]\n${indent}pub mod inverse;\n`;
      glue = glue.replace(anchorRe, insertion + m[0]);
    }

    processed++;
  }

  writeFileSync(`${REPO}/${glueRelPath}`, glue);
  report.push(`${plugin}: ${worklist.length} in worklist`);
}

console.log(`processed=${processed}`);
console.log(`skipped=${skipped.length}`);
for (const s of skipped) console.log(`SKIP: ${s}`);
for (const r of report) console.log(r);
