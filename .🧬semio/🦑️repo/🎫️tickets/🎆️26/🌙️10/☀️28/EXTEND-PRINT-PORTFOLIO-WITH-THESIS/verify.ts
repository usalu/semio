import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, createIssuedResourceControlV1, findWorkspaceRoot } from "@semio-tech/process/routing";
import { checkResourceLeaseControl, type ResourceLeaseControl } from "@semio-tech/process/leases";
import { readNativeCallerAuthorityV1 } from "@semio-tech/repo-lib/artifacts/native-orchestration/invocation";
import { contrastRatio } from "@semio-tech/ui-styling";
import { colord, extend } from "colord";
import a11y from "colord/plugins/a11y";
import { withPrintExecutionV1, withPrintScratchV1 } from "../../../../../../../🧰️framework/🛍️products/📓️print/📦️artifacts/🎛️operation/🟦️.ts";
import { withPublishedPrintDocumentV1, printTemplatePdfNames } from "../../../../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";
import { readPrintMaterialFileV1 } from "../../../../../../../🧰️framework/🛍️products/📓️print/📦️artifacts/📥️material/📤️producer/🟦️.ts";
import { inspectPrintPdfV1, rasterPrintPdfV1 } from "../../../../../../../🧰️framework/🛍️products/📓️print/📦️artifacts/📖️pdf/🟦️.ts";
import { loadPrintDesignTokens } from "../../../../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts";
import { thesisPdfPaintFactsV1, thesisFilledHeadingNumberPaintV1 } from "../../../../../../../🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/📤️publication/🎨️paint/🟦️.ts";
import thesis from "../../../../../../../🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/🔣️.json";
import contrast from "../../../../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🧪️tests/🔣️.json";
extend([a11y]);
const checkout = findWorkspaceRoot(import.meta.dirname), output = new URL("./🗑️generated/canonical-inspection/", import.meta.url);
const normal = (text: string): string => text.normalize("NFC").replace(/[‘’]/g, "'").replace(/\s+/g, "");
const digest = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const save = async (file: string, bytes: string | Uint8Array, control: ResourceLeaseControl): Promise<void> => {
  const target = new URL(file, output);
  assert.ok(fileURLToPath(target).length <= 256);
  checkResourceLeaseControl(control);
  await writeFile(target, bytes, { signal: control.signal });
  checkResourceLeaseControl(control);
};
class Verify extends BundleScript {
  async run(): Promise<void> {
    assert.equal(this.repoRoot, checkout);
    assert.equal(readNativeCallerAuthorityV1(this.invocation.capabilities).repositoryRoot, checkout);
    const phaseControl = createIssuedResourceControlV1(this.invocation, "print:canonical:thesis:verification", 0), files: unknown[] = [];
    await mkdir(output, { recursive: true });
    const proof = (complete: boolean) => JSON.stringify({ kind: "canonical-print-publication", canonicalWorkspaceProduct: true, repositoryRoot: checkout, complete, files }, null, 2) + "\n";
    await save("publication-proof.json", proof(false), phaseControl);
    await withPrintExecutionV1({ workspace: checkout, invocation: this.invocation, phaseControl, kind: "document" }, async execution => {
      for (const row of thesis.publications) await withPrintScratchV1(execution, "canonical-thesis-verification", async scratch => {
        await withPublishedPrintDocumentV1(scratch.execution, "print", row.id, "@semio-tech/print:build-" + row.id, async artifact => {
          const names = Object.values(printTemplatePdfNames(row.id + ".tex"));
          assert.deepEqual([...artifact.files].sort(), [...names].sort());
          for (const name of names) {
            const bytes = await readPrintMaterialFileV1(join(artifact.path, name), 67108864, artifact.control);
            assert.equal(new TextDecoder().decode(bytes.subarray(0, 5)), "%PDF-");
            await save(name, bytes, artifact.control);
            const actual = { operation: scratch.execution.operation, phaseControl: artifact.control }, pdf = await inspectPrintPdfV1(actual, scratch.destination, bytes);
            const pages = pdf.pages.map((page, index) => ({ page: index + 1, text: page.items.flatMap(item => "str" in item ? [item.str] : []).join(" ") }));
            const text = pages.map(page => page.text).join(" "), normalized = normal(text);
            const textProof = JSON.stringify({ canonicalWorkspaceProduct: true, pages });
            assert.ok(Buffer.byteLength(textProof) <= thesis.publicationProof.maximumTextBytes);
            await save(name.replace(/\.pdf$/, "-text.json"), textProof + "\n", artifact.control);
            assert.doesNotMatch(text, /[\uFFFD\uFFFF]|\?{2}|\[\s*\?\s*\]/u);
            for (const expected of row.expected) assert.ok(normalized.includes(normal(expected)), name + ": " + expected);
            for (const expected of row.bibliography) assert.ok(normalized.toLowerCase().includes(normal(expected).toLowerCase()), name + " bibliography: " + expected);
            for (const key of row.citationKeys) assert.ok(!normalized.includes(key), name + " resolved citation: " + key);
            const body = pdf.pages.findIndex((page, index) => normal(pages[index]!.text).includes(normal(row.firstBodyMarker)) && page.items.some(item => "str" in item && item.str.trim() === "1" && item.transform[5] < page.height / 4));
            assert.ok(body > 0, name + " first Arabic body");
            assert.ok(pdf.pages.some(page => page.items.some(item => "str" in item && /^[ivxlcdm]+$/.test(item.str.trim()) && item.transform[5] < page.height / 4)), name + " Roman frontmatter");
            const theme = name.includes("-dark") ? "dark" : "light", expected = contrast.actualThemes.find(row => row.theme === theme);
            assert.ok(expected);
            const bodyFacts = thesisPdfPaintFactsV1(pdf.pages[body]!, artifact.control), coverFacts = thesisPdfPaintFactsV1(pdf.pages[0]!, artifact.control), bodyRuns = bodyFacts.runs, coverRuns = coverFacts.runs;
            const paintProof = JSON.stringify({ body: bodyFacts, cover: coverFacts });
            assert.ok(Buffer.byteLength(paintProof) <= thesis.publicationProof.maximumTextBytes);
            await save(name.replace(/\.pdf$/, "-paint.json"), paintProof + "\n", artifact.control);
            const grouped = (runs: typeof bodyRuns, color: string) => normal(runs.filter(run => run.color === "#" + color.toLowerCase()).map(run => run.text).join(""));
            assert.ok(grouped(bodyRuns, expected.normal).includes(normal(row.firstBodyMarker)), name + " actual normal body paint");
            assert.ok(grouped(coverRuns, expected.normal).includes(normal(row.expected[1]!)), name + " actual cover title paint");
            const heading = row.id === "thesis" ? "Research Question" : "Scope";
            assert.ok(grouped(bodyRuns, expected.filled).includes(normal(heading)), name + " actual filled section foreground");
            for (const filled of row.filledHeadings) thesisFilledHeadingNumberPaintV1(bodyRuns, filled, "#" + expected.filled);
            const flattened = bodyRuns.map(run => normal(run.text)).join(""), start = flattened.indexOf(normal(row.firstBodyMarker)), end = flattened.indexOf(normal(heading), start);
            assert.ok(start >= 0 && end > start);
            let offset = 0, citation = false;
            for (const run of bodyRuns) { const value = normal(run.text); if (offset >= start && offset < end && /\d/.test(value) && run.color === "#" + expected.link.toLowerCase()) citation = true; offset += value.length; }
            assert.ok(citation, name + " actual inline citation foreground");
            const palette = loadPrintDesignTokens().colors, themeSurface = palette[theme === "light" ? "light" : "dark"], secondary = palette.secondary;
            assert.ok(themeSurface && secondary);
            assert.ok(bodyFacts.fills.includes(themeSurface) && coverFacts.fills.includes(themeSurface), name + " actual page surface");
            assert.ok(bodyFacts.fills.includes(secondary), name + " actual filled section surface");
            const pairs = [{ role: "body", foreground: "#" + expected.normal, background: themeSurface }, { role: "filled-section", foreground: "#" + expected.filled, background: secondary }, { role: "citation", foreground: "#" + expected.link, background: themeSurface }];
            const colors = pairs.map(pair => ({ ...pair, ratio: contrastRatio(pair.foreground, pair.background), oracleRatio: colord(pair.foreground).contrast(pair.background) }));
            for (const pair of colors) { assert.ok(pair.ratio >= contrast.minimum, name + " contrast " + pair.role); assert.ok(Math.abs(pair.ratio - pair.oracleRatio) < 0.011); }
            const images = await rasterPrintPdfV1(actual, scratch.destination, bytes, [1, body + 1], thesis.publicationProof.previewScale), previews = [];
            for (const image of images) { const file = name.replace(/\.pdf$/, "") + (image.page === 1 ? "-cover.png" : "-body.png"); await save(file, image.png, artifact.control); previews.push({ file, page: image.page, width: image.width, height: image.height, sha256: image.sha256 }); }
            files.push({ file: name, producer: "@semio-tech/print:build-" + row.id, pages: pdf.pages.length, firstArabicBodyPage: body + 1, sha256: digest(bytes), colors, previews });
            await save("publication-proof.json", proof(false), artifact.control);
            console.log("[print] Canonical owned PDF verified " + name);
          }
          await artifact.recheck();
        });
      });
    });
    await save("publication-proof.json", proof(true), phaseControl);
  }
}
const router = new ScriptRouter(import.meta.dirname).register("verify", Verify);
await receiveScriptProcessInvocation(process.env, invocation => router.run(["verify"], invocation));
