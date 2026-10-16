/** 📊 Reduce a bun junit report and a vitest json report to sorted `status<TAB>file<TAB>name` rows. */
import { readFileSync, writeFileSync } from "node:fs";

const [junit, vitest, out] = process.argv.slice(2) as [string, string, string];
const rows: string[] = [];
const decode = (s: string) => s.replace(/&quot;/g, '"').replace(/&apos;/g, "'").replace(/&lt;/g, "<").replace(/&gt;/g, ">").replace(/&amp;/g, "&");
try {
  const xml = readFileSync(junit, "utf8");
  const suiteRe = /<testsuite name="([^"]*)"[^>]*?(?:\/>|>([\s\S]*?)<\/testsuite>)/g;
  let m: RegExpExecArray | null;
  while ((m = suiteRe.exec(xml))) {
    const file = decode(m[1]!);
    const body = m[2] ?? "";
    const caseRe = /<testcase name="([^"]*)"[^>]*?(?:\/>|>([\s\S]*?)<\/testcase>)/g;
    let c: RegExpExecArray | null;
    while ((c = caseRe.exec(body))) {
      const inner = c[2] ?? "";
      const status = /<failure|<error/.test(inner) ? "failed" : /<skipped/.test(inner) ? "skipped" : "passed";
      rows.push(`${status}\tbun:${file.split("/").slice(-3).join("/")}\t${decode(c[1]!)}`);
    }
  }
} catch (error) {
  rows.push(`MISSING\tbun\t${(error as Error).message}`);
}
try {
  const report = JSON.parse(readFileSync(vitest, "utf8"));
  for (const file of report.testResults) {
    const short = file.name.split("/").slice(-3).join("/");
    if (file.assertionResults.length === 0) rows.push(`${file.status}\tvitest:${short}\t<file-level>${file.message ? " " + file.message.split("\n")[0].slice(0, 100) : ""}`);
    for (const t of file.assertionResults) rows.push(`${t.status}\tvitest:${short}\t${t.fullName}`);
  }
} catch (error) {
  rows.push(`MISSING\tvitest\t${(error as Error).message}`);
}
rows.sort();
writeFileSync(out, rows.join("\n") + "\n");
const count = (s: string) => rows.filter(r => r.startsWith(s)).length;
console.log(`rows=${rows.length} passed=${count("passed")} failed=${count("failed")} skipped=${count("skipped")} other=${rows.length - count("passed") - count("failed") - count("skipped")}`);
