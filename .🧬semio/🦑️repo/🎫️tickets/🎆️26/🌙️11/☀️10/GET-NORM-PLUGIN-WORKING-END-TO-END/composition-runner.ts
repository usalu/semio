const root = process.cwd();
const which = process.argv[2];
if (which === "doc") {
  const { testNormDocumentContractOracle } = await import(`${root}/✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/🧪️testing/🪪️document-contract/🟦️.ts`);
  await testNormDocumentContractOracle();
} else {
  const { testNormResultsWindowOwnershipOracle } = await import(`${root}/✏️s/🔌️plugins/📕️norm/🪟️results/🎚️config/🧪️tests/🔬️window-ownership/🟦️.ts`);
  testNormResultsWindowOwnershipOracle();
}
console.log("oracle ok", which);
