#!/usr/bin/env bun
/**
 * 🚪️ Copies the language-neutral representation scaffolds (g4, ebnf, abnf, ksy, spicy, ts, graphql, json, proto, grammar/protocol)
 * of the shooting `🚪️io/{📝️text,💾️binary}/{📸️snapshot,🔺️diff,🧬️mutations,💡️inferences}` facets into the bim subset, renaming the
 * vocabulary. The Rust of each facet and the mutation wire protocol are never touched.
 */
import { cpSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, em, repo, subset } from "./r3-f1-paths.ts";

const pluginsDir = join(repo, readdirSync(repo).find((n) => n.startsWith("✏") && n.endsWith("s"))!);
const plugins = child(pluginsDir, "plugins");
const shooting = child(child(child(child(child(child(child(plugins, "shooting"), "artifacts"), "shooting"), "standards"), "1"), "subsets"), "any");
void cpSync;
const rename = (text: string) =>
  text
    .replaceAll("s/shooting/shooting/", "s/bim/model/")
    .replaceAll("semio.s.shooting.shooting", "semio.s.bim.model")
    .replaceAll("shooting.shooting", "bim.model")
    .replaceAll("Shooting_shooting", "Bim_model")
    .replaceAll("shootingShooting", "bimModel")
    .replaceAll("Shooting", "Model")
    .replaceAll("shooting", "bim");

const skip = new Set([`mutations/${em(0x1f4e1)}.protocol.semio`, `mutations/${em(0x1f5e3)}mutations.grammar.semio`]);
let copied = 0;
for (const kind of ["text", "binary"]) {
  const from = child(child(shooting, "io"), kind);
  const to = join(child(subset, "io"), em(kind === "text" ? 0x1f4dd : 0x1f4be) + kind);
  for (const facet of ["snapshot", "diff", "mutations", "inferences"]) {
    const src = child(from, facet);
    const dst = join(to, em({ snapshot: 0x1f4f8, diff: 0x1f53a, mutations: 0x1f9ec, inferences: 0x1f4a1 }[facet]!) + facet);
    mkdirSync(dst, { recursive: true });
    for (const name of readdirSync(src)) {
      const file = join(src, name);
      if (statSync(file).isDirectory() || name.endsWith(".rs") || skip.has(`${facet}/${name}`)) continue;
      writeFileSync(join(dst, name), rename(readFileSync(file, "utf8")));
      copied += 1;
    }
  }
}
console.log(`scaffolds copied: ${copied}`);
