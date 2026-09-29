/** 🔬️ FH1: per production source, textual `FaultCode::new("`/`app_fault("` occurrences vs the census raises — finds sources whose
 * production code the census over-blanks (nested `#[cfg(test)]`). */
const overlay = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults";
const { faultFactsOfText } = await import(`${overlay}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts`);
const { readFileSync } = await import("node:fs");
const { spawnSync } = await import("node:child_process");
const files = spawnSync("git", ["-c", "core.quotePath=false", "grep", "-l", "--untracked", "-e", "FaultCode::new(\"", "-e", "app_fault(\"", "--", "*.rs", ":!.🧬semio"], { cwd: overlay, encoding: "utf8", maxBuffer: 1 << 30 }).stdout.split("\n").filter((path: string) => path && !/(^|\/)(🧪️tests|tests|🧫️fixtures|benches|examples)\//u.test(path));
for (const path of files) {
  const text = readFileSync(`${overlay}/${path}`, "utf8");
  const textual = (text.match(/FaultCode::new\("|app_fault\("/gu) ?? []).length;
  const raised = faultFactsOfText(path, text).raises.length;
  if (raised < textual) console.log(`${textual - raised}\t${textual}\t${path}`);
}
