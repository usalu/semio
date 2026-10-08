#!/usr/bin/env bun
/**
 * 🔬️ Compiles one cover into the ticket's generated folder without touching published artifacts.
 * `report <id>` truncates a Mit-Bestand report after its cover page; `template <id>` compiles a whole print template.
 */
import { cpSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { publishPrintArtifact } from "../../../../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";

const ticket = import.meta.dir, repo = join(ticket, "../../../../../../.."), [kind, id] = process.argv.slice(2);
if (!id || !["report", "template"].includes(kind ?? "")) throw new Error("usage: cover-probe.ts <report|template> <id>");
const output = join(ticket, "🗑️generated", `${kind}-${id}`);
process.env.SEMIO_TICKET_DIR = ticket;
if (kind === "report") {
  const catalog = JSON.parse(readFileSync(join(repo, "♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🔣️.json"), "utf8")) as { documents: { id: string; texPath: string }[] };
  const texPath = catalog.documents.find(document => document.id === id)!.texPath, folder = dirname(texPath), report = join(repo, "♻️mit-bestand/📋️bericht", folder);
  const root = join(ticket, "🗑️generated", `source-${id}`), document = join(root, folder);
  rmSync(root, { recursive: true, force: true });
  mkdirSync(join(document, "🖼️asset"), { recursive: true });
  for (const asset of ["🪧️logo", "🧺️demonstrator"]) if (existsSync(join(report, "🖼️asset", asset))) cpSync(join(report, "🖼️asset", asset), join(document, "🖼️asset", asset), { recursive: true });
  cpSync(join(report, "📚️references.bib"), join(document, "📚️references.bib"));
  const source = readFileSync(join(repo, "♻️mit-bestand/📋️bericht", texPath), "utf8");
  writeFileSync(join(document, basename(texPath)), `${source.slice(0, source.indexOf("\\makecoverpages"))}\\makecoverpages\n\\end{document}\n`);
  await publishPrintArtifact({ id: `cover-${id}`, sourceRoot: root, texPath, sources: [folder], output, owner: "ticket:cover-probe", dark: false });
} else {
  const product = join(repo, "🧰️framework/🛍️products/📓️print");
  const catalog = JSON.parse(readFileSync(join(product, "🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json"), "utf8")) as { documents: { id: string; texPath: string; sources: string[] }[] };
  const template = catalog.documents.find(document => document.id === id)!;
  await publishPrintArtifact({ id: `template-${id}`, sourceRoot: product, texPath: template.texPath, sources: template.sources, output, owner: "ticket:cover-probe", dark: false });
}
