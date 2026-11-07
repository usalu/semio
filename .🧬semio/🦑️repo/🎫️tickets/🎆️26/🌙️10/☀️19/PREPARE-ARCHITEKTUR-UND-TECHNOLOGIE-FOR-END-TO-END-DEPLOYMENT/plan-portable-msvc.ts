import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const directory = join(process.env.SEMIO_TICKET_DIR!, "🗑️generated");
const bytes = readFileSync(join(directory, "bazel-msvc-manifest.json"));
if (createHash("sha256").update(bytes).digest("hex") !== "a8e854fa9f3fbaeac27d7304e297f1660d7137401389deb57cf7983f33c16fef") throw Error("Untrusted manifest");
const manifest = JSON.parse(bytes.toString("utf8"));
const get = (id: string) => {
  const found = manifest.packages.filter((entry: { id: string; language?: string }) => entry.id.toLowerCase() === id.toLowerCase() && (!entry.language || entry.language === "en-US"));
  if (found.length !== 1) throw Error(`Ambiguous package ${id}: ${found.length}`);
  return found[0];
};
const ids = ["Microsoft.VC.14.50.18.0.Tools.HostX64.TargetX64.base", "Microsoft.VC.14.50.18.0.Tools.HostX64.TargetX64.Res.base", "Microsoft.VC.14.50.18.0.CRT.Headers.base", "Microsoft.VC.14.50.18.0.CRT.x64.Desktop.base"];
const sdk = get("Win11SDK_10.0.26100");
const names = ["Windows SDK for Windows Store Apps Tools-x86_en-us.msi", "Windows SDK for Windows Store Apps Headers-x86_en-us.msi", "Windows SDK for Windows Store Apps Headers OnecoreUap-x86_en-us.msi", "Windows SDK for Windows Store Apps Libs-x86_en-us.msi", "Universal CRT Headers Libraries and Sources-x86_en-us.msi", "Windows SDK Desktop Headers x64-x86_en-us.msi", "Windows SDK OnecoreUap Headers x64-x86_en-us.msi", "Windows SDK Desktop Libs x64-x86_en-us.msi"];
const payloads = names.map(name => {
  const matches = sdk.payloads.filter((payload: { fileName: string }) => payload.fileName.replaceAll("\\", "/") === `Installers/${name}`);
  if (matches.length !== 1) throw Error(`SDK payload missing or ambiguous: ${name}`);
  return matches[0];
});
const plan = { source: "Bazel independently pinned Microsoft catalog 18.4.2", compiler: ids.map(get), sdk: { id: sdk.id, version: sdk.version, payloads } };
writeFileSync(join(directory, "portable-msvc-plan.json"), JSON.stringify(plan, null, 2));
console.log(JSON.stringify({ packages: plan.compiler.map(({ id, version, payloads }: any) => ({ id, version, payloads })), sdk: plan.sdk }));
