import { writeFileSync } from "node:fs";

/** 🔬️ Differential scan: reads a repository with the TypeScript law and writes the findings as JSON so they can be compared with the Python checker. usage: bun diff-scan.ts <law module path> <repository root> <output json> */
const [modulePath, root, output] = process.argv.slice(2) as [string, string, string];
const { createDirectToolLawV1, listDirectToolFilesV1, scanDirectToolDeclarationsV1 } = await import(modulePath);
const scan = scanDirectToolDeclarationsV1(root, listDirectToolFilesV1(root), createDirectToolLawV1());
writeFileSync(output, JSON.stringify({ scanned: scan.scanned, findings: scan.findings.filter((finding: { allowed: string | null }) => finding.allowed === null) }));
console.log(scan.scanned, scan.findings.length);
