import { readFileSync } from "node:fs";
import { basename, join } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { publishPrintArtifact } from "../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";

type ReportDocument = { id: string; texPath: string; sources: string[]; actorNetwork: boolean };
const catalog: { version: number; documents: ReportDocument[] } = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
const product = "♻️mit-bestand/📋️bericht";

/** 📋️ Resolves a declared report without allowing arbitrary source or output paths. */
export function reportDocument(id: string): ReportDocument {
  const document = catalog.documents.find(document => document.id === id);
  if (catalog.version !== 1 || !document || !/^[a-z]+$/.test(id)) throw new Error(`Unknown report document: ${id}`);
  for (const path of [document.texPath, ...document.sources]) if (!path || path.includes("\\") || path.startsWith("/") || path.split("/").some(part => ["..", ".", ""].includes(part))) throw new Error(`Invalid report source: ${path}`);
  return document;
}

/** 📦️ Assigns one fixed deliverable directory to each report. */
export function reportOutputDirectory(id: string, workspace = getWorkspaceRoot()): string {
  return join(workspace, product, "📦️packages/🟦️typescript/dist/documents", reportDocument(id).id);
}

class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length !== 1) throw new Error("Report compilation requires exactly one document id");
    const document = reportDocument(args[0]!), controller = new AbortController(), abort = (): void => controller.abort(new Error("Report compilation cancelled"));
    process.once("SIGINT", abort); process.once("SIGTERM", abort);
    try { await publishPrintArtifact({ ...document, sourceRoot: join(this.repoRoot, product), output: reportOutputDirectory(document.id, this.repoRoot), owner: `@semio-tech/mit-bestand-bericht:build-${document.id}`, dark: false }, controller.signal); }
    finally { process.removeListener("SIGINT", abort); process.removeListener("SIGTERM", abort); }
  }
}

class CompleteScript extends BundleScript {
  run(args: string[]): void {
    if (args.length) throw new Error("Report collection completion accepts no arguments");
    for (const document of catalog.documents) {
      const path = join(reportOutputDirectory(document.id, this.repoRoot), `${basename(document.texPath, ".tex")}.pdf`);
      if (readFileSync(path).subarray(0, 5).toString() !== "%PDF-") throw new Error(`Invalid report PDF: ${path}`);
    }
    console.log(`[print] ${catalog.documents.length} report PDFs ready`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("complete", CompleteScript);
if (import.meta.main) await router.run(process.argv.slice(2));
