const at = (...starts) => starts.map((n) => [n, n]);
const cfg = {
  modules: [
    { name: "scan", folder: "🧰️repository-scan", emoji: "🧰️", title: "Repository scan for the test platform laws", header: true, doc: ["The repository-wide contribution scan, oracle registry, discovered cases, oracle-purity hits and contract breaches the test platform laws read, each taken ONCE per process, with the taxonomy vocabulary helpers they share."], names: ["relativeToRepo", "repoRoot", "repoContributions", "repoRegistry", "repoCases", "repoOracleHits", "repoWideBreaches", "HOST_PROTOCOL_PARITY_CASE_DIR", "contributed", "taxonomyAreas", "exemptAreas", "implementationAreas"] },
    { name: "discovery", folder: "🔍️discovery-contract", emoji: "🔍️", title: "Test platform: discovery, layout and openness laws", doc: ["The frozen vocabulary, discovery, clean safety, subject routing and selection, contribution ownership, projected vector storage and open/closed laws of the test platform."], lines: at(71, 272, 304, 411, 1193, 1270, 1507, 1531) },
    { name: "comparison", folder: "⚖️comparison-profiles", emoji: "⚖️", title: "Test platform: comparison laws", doc: ["The feature profile, comparison profiles, digests, results, pipelines, artifact profiles and non-aggregate metric laws of the test platform."], lines: at(108, 183, 236, 245, 443, 909, 1171) },
    { name: "oracles", folder: "🔮️oracle-governance", emoji: "🔮️", title: "Test platform: oracle governance laws", doc: ["The oracle registry, dependency ratchet, evidence rules, native second implementation, purity, cross-language hosts, production debt, serializer stub and re-implementation laws of the test platform."], lines: at(252, 350, 490, 546, 895, 942, 1047, 1075, 1094) },
    { name: "mutations", folder: "🦠️mutation-catalogs", emoji: "🦠️", title: "Test platform: mutation catalog laws", doc: ["The case-above-subset, binary protocol, mutation fixture, completeness, physical vector registry and real-world artifact fixture laws of the test platform."], lines: at(724, 773, 779, 1345, 1392, 1462) },
  ],
};
export default cfg;
