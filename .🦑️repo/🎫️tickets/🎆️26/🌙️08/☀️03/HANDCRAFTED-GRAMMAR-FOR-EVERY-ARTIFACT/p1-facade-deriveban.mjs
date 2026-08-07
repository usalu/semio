import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const ticket = path.dirname(fileURLToPath(import.meta.url));
const facade = fs.readFileSync(path.join(ticket, "facade.path"), "utf8").trim();
const scriptPath = "/Users/ueli/Documents/semio/📜️script.ts";

let fac = fs.readFileSync(facade, "utf8");

// Ensure FragmentRegistry + envelope in grammar reexport
if (!fac.includes("FragmentRegistry")) {
  const re = /pub use crate::os_dsl::grammar::\{([^}]+)\}/;
  if (!re.test(fac)) throw new Error("grammar use not found");
  fac = fac.replace(re, (m, inner) => {
    const add = [
      "FragmentRegistry",
      "verify_protocol_envelope",
      "Framing",
      "Block",
      "Field",
      "Prim",
      "Count",
      "ProtocolTrace",
      "ProtocolMismatch",
    ].filter((n) => !inner.includes(n));
    return `pub use crate::os_dsl::grammar::{${inner.trim().replace(/,\s*$/, "")}, ${add.join(", ")}}`;
  });
}

// Remove LanguageSpec::derived method if present (not test fn names)
fac = fac.replace(/\n\s*pub fn derived\s*\([^)]*\)[\s\S]*?\n\s*\}/g, (m) => {
  if (m.includes("LanguageSpec") || m.includes("derived(") || m.includes("Self {")) {
    return "\n    // deleted LanguageSpec::derived (P1/M3b)\n";
  }
  return m;
});
// More precise: find impl LanguageSpec block method named derived
{
  const idx = fac.search(/fn derived\s*\(/);
  if (idx >= 0) {
    // check it's not a test
    const before = fac.slice(Math.max(0, idx - 200), idx);
    if (!before.includes("#[test]") && !before.includes("fn derived_")) {
      // find method end - naive brace match from idx
      let i = fac.indexOf("{", idx);
      let depth = 0;
      for (; i < fac.length; i++) {
        if (fac[i] === "{") depth++;
        else if (fac[i] === "}") {
          depth--;
          if (depth === 0) {
            i++;
            break;
          }
        }
      }
      // include leading whitespace/doc
      let start = idx;
      while (start > 0 && fac[start - 1] !== "\n") start--;
      // include doc comments
      while (start > 0) {
        const prevLineStart = fac.lastIndexOf("\n", start - 2) + 1;
        const prev = fac.slice(prevLineStart, start);
        if (prev.trim().startsWith("///") || prev.trim().startsWith("//")) start = prevLineStart;
        else break;
      }
      fac = fac.slice(0, start) + "    // deleted LanguageSpec::derived (P1/M3b)\n" + fac.slice(i);
    }
  }
}

fs.writeFileSync(facade, fac);
console.log("facade FragmentRegistry", fac.includes("FragmentRegistry"));

// Derive ban policy in script.ts
let script = fs.readFileSync(scriptPath, "utf8");
if (!script.includes("policyGenericCodecDeriveBreaches")) {
  const helper = `
//#region 🛡️HandcraftedDeriveBan
/** Mid-migration: forbid NEW generic codec derive attributes; shrink to empty at P6. */
const POLICY_GENERIC_CODEC_DERIVE_EXEMPTIONS = new Set<string>([
  // filled dynamically below from current corpus snapshot — P6 empties this set and deletes emission
]);

function policyGenericCodecDeriveBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const pluginsRoot = join(repoRoot, "✏️s");
  if (!existsSync(pluginsRoot)) return breaches;
  const deriveNames = ["DocumentDsl", "OpText", "DocumentPack", "OpBinary"] as const;
  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (entry.name === "node_modules" || entry.name === "target" || entry.name.startsWith(".")) continue;
      const full = join(dir, entry.name);
      if (entry.isDirectory()) {
        walk(full);
        continue;
      }
      if (!entry.name.endsWith(".rs")) continue;
      const rel = full.slice(repoRoot.length + 1).replace(/\\\\/g, "/");
      if (POLICY_GENERIC_CODEC_DERIVE_EXEMPTIONS.has(rel)) continue;
      let text = "";
      try {
        text = readFileSync(full, "utf8");
      } catch {
        continue;
      }
      for (const name of deriveNames) {
        const re = new RegExp("#\\\\[derive\\\\(([^)]*)\\\\)]");
        // scan all derive attrs
        const attrRe = /#\\[derive\\(([^\\]]*)\\)\\]/g;
        let m: RegExpExecArray | null;
        while ((m = attrRe.exec(text))) {
          if (m[1].split(",").map((s) => s.trim()).includes(name)) {
            breaches.push({
              kind: "generic-codec-derive",
              summary: \`\${rel} derives \${name} — handcrafted codec required (P6 deletes emission)\`,
              path: rel,
            });
          }
        }
      }
    }
  };
  walk(pluginsRoot);
  return breaches;
}
//#endregion 🛡️HandcraftedDeriveBan
`;
  // Insert before policyHandcraftedSpecP3Breaches definition
  const anchor = "function policyHandcraftedSpecP3Breaches";
  const ai = script.indexOf(anchor);
  if (ai < 0) throw new Error("policyHandcraftedSpecP3Breaches not found");
  script = script.slice(0, ai) + helper + "\n" + script.slice(ai);

  // Wire into the aggregator
  script = script.replace(
    "...policyEmptyExampleBreaches(repoRoot),",
    "...policyEmptyExampleBreaches(repoRoot),\n    ...policyGenericCodecDeriveBreaches(repoRoot),",
  );

  // Seed exemptions with current offenders so gate stays green mid-migration
  // We'll compute by running a quick scan after write - for now leave set empty and populate next.
  fs.writeFileSync(scriptPath, script);
  console.log("inserted derive ban policy");
} else {
  console.log("derive ban already present");
}

fs.appendFileSync(
  path.join(ticket, "progress-v2.md"),
  `\n\n## Facade + derive-ban\n- FragmentRegistry exported from dsl facade\n- policyGenericCodecDeriveBreaches staged (exemptions to be seeded)\n`,
);
console.log("done");
