const truth = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts");
const { renderRepoErrorV1, isRepoErrorV1 } = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts");
const tests = { jsLockParity: () => 0, truth: () => 0 };
for (const args of [["locks", "--format", "xml"], ["parity", "js", "--format", "xml"], ["parity", "go"], ["list", "nonsense"]]) {
  try { truth.runDependencyVerification("/Users/ueli/Documents/semio", args, tests); console.log("no error", args.join(" ")); }
  catch (error) { console.log(args.join(" "), "=>", isRepoErrorV1(error) ? renderRepoErrorV1(error as any, "en") + " | " + renderRepoErrorV1(error as any, "de") : String(error)); }
}
