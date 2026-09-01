import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { RECIPES } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🏭️generator/📜️script.ts";

const D = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any";
const fixturesDir = join(process.cwd(), D, "🧫️fixtures");

function digest(path: string) {
  const bytes = readFileSync(path);
  return { sha256: `sha256:${createHash("sha256").update(bytes).digest("hex")}`, bytes: bytes.length };
}

const manifests = RECIPES.map((recipe) => {
  const dir = join(fixturesDir, recipe.id);
  const files: Record<string, unknown>[] = [];
  const before = digest(join(dir, "before.docx"));
  files.push({ role: "before-docx", path: `../🧫️fixtures/${recipe.id}/before.docx`, mediaType: "application/vnd.openxmlformats-officedocument.wordprocessingml.document", sha256: before.sha256, bytes: before.bytes });
  if (recipe.outcome !== "rejected") {
    const after = digest(join(dir, "after.docx"));
    files.push({ role: "after-docx", path: `../🧫️fixtures/${recipe.id}/after.docx`, mediaType: "application/vnd.openxmlformats-officedocument.wordprocessingml.document", sha256: after.sha256, bytes: after.bytes });
  }
  return {
    schema: "semio.repository-test.fixture/v2",
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.docx", standard: "ecma-376", subset: "any" },
    mutation: recipe.mutation,
    outcome: recipe.outcome,
    units: { length: "unitless", angle: "degree" },
    files,
    generator: {
      oracle: "jszip-fast-xml-parser-docx-ecma-376-mutate",
      packageVersion: "jszip@3.10.1 + fast-xml-parser@5.11.1",
      engineFamily: "zip-xml",
      engineVersion: "jszip@3.10.1 + fast-xml-parser@5.11.1",
      command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      platform: "darwin-arm64",
    },
    provenance: {
      source: "generated",
      license: "MIT (jszip dual MIT/GPL-3.0-or-later, used under MIT) + MIT (fast-xml-parser)",
      attribution: "Generated with jszip (MIT) and fast-xml-parser (MIT)",
      security: "scanned-clean",
      privacy: "no-personal-data",
    },
    comparisonProfile: "semantic-docx-ecma-376-jszip-v1",
    reproducible: true,
    family: "structural",
    notes: recipe.notes,
  };
});

process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
