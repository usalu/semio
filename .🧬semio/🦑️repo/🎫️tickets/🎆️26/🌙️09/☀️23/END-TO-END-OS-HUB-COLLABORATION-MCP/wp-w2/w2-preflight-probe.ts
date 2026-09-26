/** 🛫️ W2: runs the trusted bootstrap's descriptor preflight over `--packages all` on the committed descriptors and times it. */
const { trustedBootstrapPreflightDescriptorsV1, trustedBootstrapSelectPackages } = await import("/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts");
const started = performance.now();
try {
  trustedBootstrapPreflightDescriptorsV1("/Users/ueli/Documents/semio", trustedBootstrapSelectPackages(process.argv[2] ?? "all"));
  console.log(`PASS ${Math.round(performance.now() - started)} ms`);
} catch (error) {
  console.log(`REFUSED ${Math.round(performance.now() - started)} ms: ${error instanceof Error ? error.message : String(error)}`);
  process.exitCode = 1;
}
