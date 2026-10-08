#!/usr/bin/env bun
/** 🔬️ Publishes a ticket probe body under the live Zwischenbericht preamble and print styles into the ticket's generated folder. */
import { cpSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { publishPrintArtifact } from "../../../../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";

const ticket = import.meta.dir, repo = join(ticket, "../../../../../../.."), report = join(repo, "♻️mit-bestand/📋️bericht/📋️zwischenbericht");
const name = process.argv[2] ?? "probe", root = join(ticket, "🗑️generated", `${name}-source`), document = join(root, "📋️zwischenbericht");
rmSync(root, { recursive: true, force: true });
mkdirSync(join(document, "🖼️asset"), { recursive: true });
for (const asset of ["🪧️logo", "🧺️demonstrator"]) cpSync(join(report, "🖼️asset", asset), join(document, "🖼️asset", asset), { recursive: true });
cpSync(join(report, "📚️references.bib"), join(document, "📚️references.bib"));
const source = readFileSync(join(report, "📋️zwischenbericht.tex"), "utf8");
writeFileSync(join(document, "📋️zwischenbericht.tex"), `${source.slice(0, source.indexOf("\\begin{document}"))}${readFileSync(join(ticket, `${name}.tex`), "utf8")}`);
process.env.SEMIO_TICKET_DIR = ticket;
await publishPrintArtifact({ id: name, sourceRoot: root, texPath: "📋️zwischenbericht/📋️zwischenbericht.tex", sources: ["📋️zwischenbericht"], output: join(ticket, "🗑️generated", name), owner: `ticket:${name}`, dark: false });
