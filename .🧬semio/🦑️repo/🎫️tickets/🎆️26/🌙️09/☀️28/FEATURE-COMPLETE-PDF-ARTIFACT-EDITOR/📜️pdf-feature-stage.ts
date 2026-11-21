/** 🔗️ Combines the reviewable PDF editor stages against one live source generation. */
import { createHash } from "node:crypto";
import { resolve, join } from "node:path";

type Row = readonly [path: string, before: string | null, body: string];
type Receipt = { before: string | null; after: string; stage?: string; staged?: string };
const output = join(import.meta.dir, "🗑️generated");
const digest = (body: string) => createHash("sha256").update(body).digest("hex");
const editor = process.argv[2];
if (!editor || !editor.endsWith("/✏️editor/🦀️.rs")) throw new Error("Pass the PDF 1.7 Base editor source");
const sources = new Map<string, Row>();

async function checked(row: Row) {
    const [path, before] = row;
    const live = Bun.file(path);
    const present = await live.exists();
    const observed = present ? createHash("sha256").update(await live.bytes()).digest("hex") : null;
    if (observed !== before) throw new Error(`Source preimage moved: ${path}`);
    if (resolve(path).normalize("NFD").length > 256) throw new Error(`Source path exceeds 256 UTF-16 units: ${path}`);
}

function replace(body: string, anchor: string, candidate: string): string {
    if (body.split(anchor).length !== 2) throw new Error(`Unexpected PDF editor anchor: ${anchor}`);
    return body.replace(anchor, candidate);
}

function combineEditor(save: string, archive: string): string {
    const owners = archive.match(/    fn document_store_owners_source_demands[^\n]*\n[^\n]*\n    }\n/)?.[0];
    const factory = archive.match(/        document_store_owners: [^\n]*\n/)?.[0];
    if (!owners || !factory) throw new Error("Archive admission anchors are missing");
    const decode = "    const SINGLE_TURN_NATURAL_DECODE: bool = true;";
    const prep = '        preparation: "stdio-pdf-1-7-base-snapshot-edit",\n';
    let body = replace(replace(save, decode, owners + decode), prep, prep + factory);
    const start = "    /// ✏️ Replaces the addressed page";
    const direct = `                let artifact_mutations = page_text_edit_mutations(doc.snapshot, *page, *item, revision, text)?;
                Ok(Emit { artifact_mutations, ..Default::default() })`;
    const bounded = direct.replace("doc.snapshot", "snapshot");
    const commands = `fn emit_page_command(command: &Pdf17EditorCommand, snapshot: &PdfSnapshot) -> Option<Result<Emit<PdfMutation>, Fault>> {
    match command {
        Pdf17EditorCommand::SetPage { page, item, revision, text } => Some(page_text_edit_mutations(snapshot, *page, *item, revision, text)
            .map(|artifact_mutations| Emit { artifact_mutations, ..Default::default() })),
        Pdf17EditorCommand::PageEdit { action, payload } => Some(crate::editor::page::emit_page_edit(snapshot, action, payload)),
        _ => None,
    }
}

`;
    const impl = "impl ArtifactEditor for Pdf17Editor {\n";
    const nativeTag = "semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native";
    const directMatch = `            ${nativeTag}(Pdf17EditorCommand::SetPage { page, item, revision, text }) => {
${direct}
            }
                        ${nativeTag}(Pdf17EditorCommand::PageEdit { action, payload }) => {
                crate::editor::page::emit_page_edit(doc.snapshot, action, payload)
            }
            ${nativeTag}(Pdf17EditorCommand::PickImage { .. } | Pdf17EditorCommand::ImportImage { .. }) =>
                Err(Fault::from("PNG import requires its retained interactive job")),`;
    const boundedMatch = `            ${nativeTag}(Pdf17EditorCommand::SetPage { page, item, revision, text }) => {
${bounded}
            }
            ${nativeTag}(Pdf17EditorCommand::PageEdit { action, payload }) => {
                crate::editor::page::emit_page_edit(snapshot, action, payload)
            }
            _ => Err(Fault::from("stdio-pdf-native-edit-command-mismatch")),`;
    body = replace(body, impl, commands + impl);
    body = replace(body, directMatch, `            ${nativeTag}(command) => emit_page_command(command, doc.snapshot)
                .unwrap_or_else(|| Err(Fault::from("PNG import requires its retained interactive job"))),`);
    body = replace(body, boundedMatch, `            ${nativeTag}(command) => emit_page_command(command, snapshot)
                .unwrap_or_else(|| Err(Fault::from("stdio-pdf-native-edit-command-mismatch"))),
            _ => Err(Fault::from("stdio-pdf-native-edit-command-mismatch")),`);
    if (!body.includes(start)) throw new Error("Unexpected editor trait owner");
    return body;
}

async function include(row: Row, section: string) {
    await checked(row);
    const [path, before, body] = row;
    const previous = sources.get(path);
    if (!previous || previous[2] === body) {
        sources.set(path, row);
        return;
    }
    if (path === editor && section === "archive" && before === previous[1]) {
        sources.set(path, [path, before, combineEditor(previous[2], body)]);
        return;
    }
    throw new Error(`Distinct ${section} candidates share ${path}`);
}

async function includeReceipt(rows: Record<string, Receipt>, directory: string, section: string) {
    for (const [path, receipt] of Object.entries(rows)) {
        const { before, after, stage, staged } = receipt;
        if (!stage && !staged) throw new Error(`Missing candidate for ${path}`);
        const body = await Bun.file(staged ?? join(directory, stage!)).text();
        if (digest(body) !== after) throw new Error(`Reviewed candidate moved: ${path}`);
        await include([path, before, body], section);
    }
}

for (const row of await Bun.file(join(output, "classic-graph", "registered-save-overlay.json")).json() as Row[]) {
    await include(row, "save");
}
const archive = join(output, "archive-install");
await includeReceipt((await Bun.file(join(archive, "receipt.json")).json()).paths, archive, "archive");
const details = join(output, "snapshot-read-only");
await includeReceipt(await Bun.file(join(details, "stage.json")).json(), details, "details");

const activeLines = (body: string) => body.split("\n").filter(line => line.trim() && !line.trimStart().startsWith("//")).length;
const original = (await Bun.file(join(output, "classic-graph", "registered-save-overlay.json")).json() as Row[]).find(row => row[0] === editor);
const combined = sources.get(editor);
if (!original || !combined || activeLines(combined[2]) > activeLines(original[2]) + 2) {
    throw new Error(`Merged controller exceeds its reviewed source allowance: ${combined && activeLines(combined[2])}`);
}
const ordered = [...sources.values()].sort(([first], [second]) => first.localeCompare(second, "en"));
await Bun.write(join(output, "pdf-registered-feature-overlay.json"), JSON.stringify(ordered, null, 2) + "\n");
console.log(`[DEBUG] Combined and rechecked ${ordered.length} source candidates. Controller active lines ${activeLines(combined[2])} from ${activeLines(original[2])}.`);
