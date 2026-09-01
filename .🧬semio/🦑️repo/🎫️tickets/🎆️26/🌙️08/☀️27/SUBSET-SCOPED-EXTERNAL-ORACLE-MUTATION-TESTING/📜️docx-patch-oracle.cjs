const fs = require("fs");

const ORACLE_PATH = process.argv[2];
const FIXTURE_MANIFESTS_PATH = process.argv[3];

const oracle = JSON.parse(fs.readFileSync(ORACLE_PATH, "utf8"));
const fixtureManifests = JSON.parse(fs.readFileSync(FIXTURE_MANIFESTS_PATH, "utf8"));

// --- 1. new oracle entry (javascript ecosystem, mirrors BCF's jszip-fast-xml-parser entry) ---
oracle.oracles.push({
  id: "jszip-fast-xml-parser-docx-ecma-376-mutate",
  kind: "third-party-library",
  ecosystem: "javascript",
  package: "jszip",
  version: "3.10.1",
  packages: [
    {
      package: "fast-xml-parser",
      version: "5.11.1",
      license: "MIT",
      homepage: "https://github.com/NaturalIntelligence/fast-xml-parser",
      role: "independent XML reader/writer for every part inside the archive",
    },
  ],
  engine: {
    family: "zip-xml",
    implementation: "jszip + fast-xml-parser (read-only projection; comparison done by the pipeline, never by this composition)",
    version: "jszip@3.10.1 + fast-xml-parser@5.11.1",
  },
  capabilities: ["docx-ecma-376-mutate"],
  comparisonProfiles: ["semantic-docx-ecma-376-jszip-v1"],
  license: "MIT",
  testOnly: true,
  productionReachable: false,
  networkDuringExecution: false,
  platforms: ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64", "win32-x64"],
  homepage: "https://github.com/Stuk/jszip ; https://github.com/NaturalIntelligence/fast-xml-parser",
  rationale:
    "Ticket 26/08/27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING, mirroring the BCF 2.1/✳️any pilot's own mid-session redirect: production Rust (`semio-s-plugin-stdio`) does not compile right now (a peer's in-flight migration, confirmed independently, not this ticket's bug), so the pre-existing `zip-quick-xml-docx-ecma-376-mutate` Rust oracle entry is left untouched and this JS-ecosystem composition is registered as a SECOND, independent oracle for the same `docx-ecma-376-mutate` capability. This composition performs NO mutation semantics and predicts nothing: `../🔬️probes/📜️script.ts`'s `docx-import`/`docx-project`/`docx-compare` only open the OPC container with `jszip` and parse every XML part with `fast-xml-parser`'s `XMLParser` (`preserveOrder: true`, the only mode that keeps a `w:p` and a following `w:tbl` in the order they appear rather than folding same-tag siblings into an array and losing cross-tag order), projecting to the typed body/styles view `semantic-docx-ecma-376-mutate-v1` already documents. The comparison (`docx-compare`'s own order-sensitive structural deep-equal for `body`/`styles`, unordered path-keyed digest map for every other real OPC part) is a structural fact about two already-existing byte blobs, never a computed expectation. Verified both ways this session with real fixtures: comparing the `no-mutation-no-op` before/after pair (byte-identical) reports `equal:true, diffCount:0`; comparing that same before against a deliberately corrupted copy (one style's name replaced) reports `equal:false, diffCount:1`, naming `$.styles[0].name`. `../🏭️generator/📜️script.ts` builds every fixture the same way — typed recipe objects written directly to real OPC bytes via `jszip`+`fast-xml-parser`'s `XMLBuilder`, never by executing this repository's own mutation dispatch.",
  hostPath: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🔬️probes",
});

// --- 2. new comparisonProfile + comparisonPipeline (mirrors BCF's semantic-bcf-jszip-v1 / bcf-2-1-jszip-compare-v1) ---
oracle.comparisonProfiles.push({
  id: "semantic-docx-ecma-376-jszip-v1",
  description: "Bundle-style comparison for the jszip+fast-xml-parser oracle: delegates to the docx-ecma-376-jszip-compare-v1 pipeline rather than a single-projection diff.",
  pipeline: "docx-ecma-376-jszip-compare-v1",
});

oracle.comparisonPipelines = [
  ...(oracle.comparisonPipelines ?? []),
  {
    id: "docx-ecma-376-jszip-compare-v1",
    description: "Reads the subject's produced docx and the fixture's own expected docx with an independent jszip+fast-xml-parser composition, then compares their typed body/styles projections plus the unordered other-parts digest map. GATING.",
    stages: [
      {
        probe: "docx-import",
        description: "An independent reader accepts both files.",
        inputs: ["expected-docx", "actual-docx"],
        assertions: { bothImport: true },
      },
      {
        probe: "docx-compare",
        description: "Order-sensitive body/styles structural equality plus unordered other-parts digest equality — the operative equality.",
        inputs: ["expected-docx", "actual-docx"],
        assertions: { equal: true },
      },
    ],
  },
];

// --- 3. probes[] (docx-import / docx-project / docx-compare) ---
const PROBE_COMMAND = ["bun", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🔬️probes/📜️script.ts"];
const PROBE_ENGINE = { family: "zip-xml", implementation: "jszip + fast-xml-parser", version: "jszip@3.10.1 + fast-xml-parser@5.11.1" };
const PROBE_COMMON = {
  kind: "external-process",
  ecosystem: "javascript",
  package: "jszip",
  version: "3.10.1",
  engine: PROBE_ENGINE,
  outputSchema: "semio.repository-test.probe-report/v2",
  deterministic: true,
  license: "MIT",
  testOnly: true,
  productionReachable: false,
  networkDuringExecution: false,
};

oracle.probes = [
  ...(oracle.probes ?? []),
  {
    ...PROBE_COMMON,
    id: "docx-import",
    capabilities: ["docx.zip.import"],
    command: [...PROBE_COMMAND, "docx-import"],
    rationale: "An INDEPENDENT reader accepts both files at all. Nothing downstream means anything if one of them does not parse.",
    qualification: {
      status: "qualified",
      evidence: "Run against every generated fixture pair this session (25 recipes, 43 files) — bothImport true in every case.",
      checkedAt: "2026-08-28",
      criteria: [
        { id: "reads-a-real-docx", met: true, detail: "parses [Content_Types].xml + word/document.xml + word/styles.xml + every other OPC part" },
        { id: "offline", met: true, detail: "jszip and fast-xml-parser are both pure-JS, already vendored in node_modules" },
      ],
    },
  },
  {
    ...PROBE_COMMON,
    id: "docx-project",
    capabilities: ["docx.zip.project"],
    command: [...PROBE_COMMAND, "docx-project"],
    rationale: "The typed projection every comparison is measured against — the ordered body block tree (paragraphs with style ref + ordered runs, tables with ordered rows/cells recursively), the ordered styles list (id/name/basedOn), and every other real OPC part as a path-keyed content-type+digest map.",
    qualification: {
      status: "qualified",
      evidence: "Run against insert-block-appends-a-pricing-table/after.docx: reports blockCount 3 with the appended 2×2 table's rows/cells/paragraphs recovered correctly, and against set-part-adds-core-properties/after.docx: reports otherPartCount 1 for docProps/core.xml with the correct resolved content type.",
      checkedAt: "2026-08-28",
      criteria: [
        { id: "recovers-ordered-body", met: true, detail: "paragraph and table blocks recovered in document order, xml:space=preserve text kept intact" },
        { id: "recovers-ordered-styles", met: true, detail: "id/name/basedOn recovered in word/styles.xml document order" },
      ],
    },
  },
  {
    ...PROBE_COMMON,
    id: "docx-compare",
    capabilities: ["docx.zip.compare"],
    command: [...PROBE_COMMAND, "docx-compare"],
    rationale: "Order-sensitive structural deep-equal over body/styles plus unordered digest equality over every other OPC part — the GATING comparison. Computes no mutation semantics, only structural equality of two already-existing byte blobs.",
    qualification: {
      status: "qualified",
      evidence: "Validated BOTH ways this session with real measured numbers: no-mutation-no-op's identical before/after pair -> {equal:true, diffCount:0}; that same before against a deliberately corrupted copy (word/styles.xml's Heading1 name replaced with \"CORRUPTED HEADING\") -> {equal:false, diffCount:1, diffs:[\"$.styles[0].name: \\\"heading 1\\\" \\u2260 \\\"CORRUPTED HEADING\\\"\"]}. Also exercised on a genuine applied mutation: bolds-the-tower-run-of-the-opening-paragraph's before/after -> {equal:false, diffCount:1, diffs:[\"$.body[0].runs[1].bold: false \\u2260 true\"]}.",
      checkedAt: "2026-08-28",
      criteria: [
        { id: "accepts-a-known-good-pair", met: true, detail: "equal:true, diffCount:0" },
        { id: "rejects-a-known-bad-pair", met: true, detail: "equal:false, diffCount:1, exact field named" },
      ],
    },
  },
];

// --- 4. mutationManifests (13 kinds, oracleRequirements naming the new oracle) ---
const OUTCOMES_BY_KIND = {
  "no-mutation": ["no-op"],
  "set-snapshot": ["applied", "no-op"],
  "insert-block": ["applied", "rejected"],
  "remove-block": ["applied", "rejected"],
  "set-block-content": ["applied", "no-op"],
  "set-run-text": ["applied", "no-op"],
  "set-run-formatting": ["applied", "no-op"],
  "insert-style": ["applied", "rejected"],
  "remove-style": ["applied", "rejected"],
  "set-style-name": ["applied", "rejected"],
  "set-style-based-on": ["applied", "rejected"],
  "set-part": ["applied", "no-op"],
  "remove-part": ["applied", "rejected"],
};
const VARIANT_BY_KIND = {
  "no-mutation": "NoMutation",
  "set-snapshot": "SetSnapshot",
  "insert-block": "InsertBlock",
  "remove-block": "RemoveBlock",
  "set-block-content": "SetBlockContent",
  "set-run-text": "SetRunText",
  "set-run-formatting": "SetRunFormatting",
  "insert-style": "InsertStyle",
  "remove-style": "RemoveStyle",
  "set-style-name": "SetStyleName",
  "set-style-based-on": "SetStyleBasedOn",
  "set-part": "SetPart",
  "remove-part": "RemovePart",
};

oracle.mutationManifests = [
  ...(oracle.mutationManifests ?? []),
  {
    schema: "semio.repository-test.mutation-manifest/v2",
    artifact: "s.stdio.docx",
    standard: "ecma-376",
    subset: "any",
    standardDirectoryName: "🔖️ecma-376",
    subsetDirectoryName: "✳️any",
    mutations: Object.keys(OUTCOMES_BY_KIND).map((id) => ({
      id,
      capability: "docx-ecma-376-mutate",
      outcomes: OUTCOMES_BY_KIND[id],
      productionDispatch: { operation: id, bridgeVersion: 1, variant: VARIANT_BY_KIND[id] },
      oracleRequirements: [{ capability: "docx-ecma-376-mutate", qualifyingKind: "third-party-library", oracle: "jszip-fast-xml-parser-docx-ecma-376-mutate" }],
    })),
  },
];

// --- 5. fixtureManifests (25 recipes) ---
oracle.fixtureManifests = [...(oracle.fixtureManifests ?? []), ...fixtureManifests];

fs.writeFileSync(ORACLE_PATH, `${JSON.stringify(oracle, null, 2)}\n`);
console.log("patched", ORACLE_PATH);
