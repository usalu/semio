#!/usr/bin/env bun
/**
 * 🧪️ Writes the three oracle cases of WP-16 into the subset (idempotent): `🧨️infer-bim-1-clashes` (numpy), `⚖️infer-bim-1-rules` (shapely) and `💬️export-bim-1-bcf` (zipfile + lxml): the language-agnostic
 * `🥒️.feature` from `🗑️generated/w2-wp16-coordination/drafts/cases`, the python oracle from the ticket inputs (`r12-w2-wp16-*-oracle.py` -> `🐍️.py`), the Rust subject adapter, and the oracle rows in the subset
 * oracle manifest and the plugin host packages.
 */
import { copyFileSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { child, subset } from "./r3-f1-paths.ts";

const tests = child(subset, "tests");
const drafts = join(import.meta.dir, "🗑️generated", "w2-wp16-coordination", "drafts", "cases");

const adapter = (title: string, topic: string, scenario: string, subjectName: string, input: string, slug: string, decode: string) => `//! ${title} Rust adapter (subject role only). The third-party reproduction lives in \`🐍️.py\` beside this file; this adapter answers the same scenario from the inference of the very same committed
//! input, and the platform compares both projections under \`floating-point-v1\`. It registers no oracle handler: a subject that re-read the committed expectation would be a self-comparison reporting a pass.
//!
//! The subject half is \`sut\`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::{${decode}, encode_inference_projection_json};

    /// ${topic}
    pub fn ${subjectName}(ctx: &Context) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("${input}")).ok_or_else(|| "the scenario names no ${input}".to_string())?;
        let snapshot = ${decode}(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed input is not UTF-8: {error}"))?)?;
        let table = encode_inference_projection_json(&snapshot, "${slug}").ok_or_else(|| "${slug} is not an inference of s.bim.model".to_string())?;
        let projection = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls; ids are the feature's \`@id-*\` tags.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = built.subject("${scenario}", subject::${subjectName});
    }
    built
}
//#endregion 🔖️Registration
`;

const cases = [
  { dir: "🧨️infer-bim-1-clashes", feature: "clashes.feature", oracle: "r12-w2-wp16-clash-oracle.py", rust: adapter("🧨️ BIM inference case `infer-bim-1-clashes`,", "The clash table of the committed frame.", "clashes-frame", "frame", "📸️snapshot", "clashes", "decode_model_snapshot_json") },
  { dir: "⚖️infer-bim-1-rules", feature: "rules.feature", oracle: "r12-w2-wp16-rules-oracle.py", rust: adapter("⚖️ BIM inference case `infer-bim-1-rules`,", "The rule table of the committed house.", "rules-limits", "limits", "📸️snapshot", "rules", "decode_model_snapshot_json") },
  { dir: "💬️export-bim-1-bcf", feature: "bcf.feature", oracle: "r12-w2-wp16-bcf-oracle.py", rust: adapter("💬️ BIM export case `export-bim-1-bcf`,", "The topic table of the committed frame model.", "export-bcf-frame", "frame", "model.json", "bcf", "decode_model_snapshot_json") },
];
for (const row of cases) {
  const dir = join(tests, row.dir);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "🥒️.feature"), readFileSync(join(drafts, row.feature), "utf8"));
  copyFileSync(join(import.meta.dir, row.oracle), join(dir, "🐍️.py"));
  writeFileSync(join(dir, "🦀️.rs"), row.rust);
}

const manifestPath = join(child(subset, "oracles"), "🔣️.json");
const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
const entry = (id: string, pkg: string, version: string, capability: string, rationale: string, family: string, implementation: string, packages: unknown[] = []) => ({
  id,
  kind: "third-party-library",
  ecosystem: "python",
  package: pkg,
  version,
  ...(packages.length ? { packages } : {}),
  capabilities: [capability],
  comparisonProfiles: ["floating-point-v1"],
  license: "BSD-3-Clause",
  testOnly: true,
  homepage: pkg === "numpy" ? "https://numpy.org" : "https://lxml.de",
  rationale,
  engine: { family, implementation, version },
  productionReachable: false,
  networkDuringExecution: false,
});
const additions = [
  entry(
    "bim-1-numpy-clashes",
    "numpy",
    "2.5.0",
    "bim-1-infer",
    "numpy decides the 3D clashes of the committed frame without sharing a line with the subject: the volume of every mesh by the divergence theorem proves it is a box, the clash of two boxes is analytic (the overlap per axis, the penetration -min(overlap), the contact point at the centre of the overlap box, a Euclidean gap for the soft clash), and a brute-force edge/triangle test written with numpy decides that boxes overlap with a positive volume exactly when their surfaces cross or one lies inside the other.",
    "numpy",
    "numpy ndarray linear algebra over the CPython interpreter of the clash oracle",
  ),
  entry(
    "bim-1-lxml-bcf",
    "lxml",
    "6.1.3",
    "bim-1-export-bcf",
    "zipfile opens the committed BCF container and verifies the CRC of every entry, lxml parses every part and the rules of BCF-XML 2.1 are checked (topic child order, required elements, xs:dateTime, UUID guids, orthonormal camera, 22 character IFC GlobalIds); the table of the topics read out of the container is compared with the table the subject reports from the issues of the model. ifcopenshell reads the GlobalIds of the committed IFC export when one is given.",
    "lxml",
    "lxml (libxml2) parsing over the CPython zipfile reader",
    [
      { package: "numpy", version: "2.5.0", license: "BSD-3-Clause", role: "orthonormality of the camera vectors", homepage: "https://numpy.org" },
      { package: "ifcopenshell", version: "0.8.4.post1", license: "LGPL-3.0-or-later", role: "the GlobalIds of the IFC export the selected components must equal", homepage: "https://ifcopenshell.org" },
    ],
  ),
];
for (const added of additions) if (!manifest.oracles.some((row: { id: string }) => row.id === added.id)) manifest.oracles.push(added);
writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + "\n");

const plugin = resolve(subset, "../../../../../..");
const hostPath = join(plugin, readdirSync(plugin).find((name) => name.endsWith("oracles"))!, "🔣️.json");
const host = JSON.parse(readFileSync(hostPath, "utf8"));
if (!host.oracleHostPackages.some((row: { package: string }) => row.package === "numpy")) {
  host.oracleHostPackages.push({ implementation: "python", package: "numpy", version: "2.5.0", module: "numpy" });
  writeFileSync(hostPath, JSON.stringify(host, null, 2) + "\n");
}
console.log(`wrote ${cases.length} cases and ${additions.length} oracle rows`);
