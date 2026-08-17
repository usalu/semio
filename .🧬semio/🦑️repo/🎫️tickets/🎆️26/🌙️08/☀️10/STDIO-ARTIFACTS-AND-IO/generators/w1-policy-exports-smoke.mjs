
const m = await import(process.cwd() + "/script.ts");
const names = [
  "policyStdioCatalogBreaches",
  "policyArtifactBuilderBreaches",
  "policyArtifactDecomposerBreaches",
  "policySchemaRepresentationBreaches",
  "policyIoSerializerMatrixBreaches",
  "policyIoTerminalityBreaches",
  "policyCodecFidelityBreaches",
  "policyStdioArtifactsBreaches",
];
for (const n of names) {
  console.log(n, typeof m[n]);
}
const term = m.policyIoTerminalityBreaches(process.cwd());
console.log("terminalityBreaches", term.length);
