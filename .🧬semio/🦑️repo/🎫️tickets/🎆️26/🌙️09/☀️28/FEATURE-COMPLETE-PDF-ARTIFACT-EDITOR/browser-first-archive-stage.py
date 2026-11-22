"""🧩 Generate the journal and trusted provider overlay from the current checkout."""
from difflib import unified_diff
import os
from pathlib import Path

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").is_file())
storage = next(root.glob("🧰*/🛍*/💻*/🔨*/🧑*/🔌*/🗄*backbone-storage"))
journal = storage / "📜️event-log/🟦️.ts"
provider = storage / "🟦️.ts"
plugin = storage.parent / "🧱️backbone-vite-plugin/🟦️.ts"
dev = storage.parent.parent
serve = dev / "♻️activation/🌐️serve/🟦️.ts"
native = next(root.glob("🧰*/🛍*/💻*/🔨*/🏪*/🔄*/🖥*/📦*/🦀*/Cargo.toml"))
output = ticket / "🗑️generated/browser-first-archive"
output.mkdir(parents=True, exist_ok=True)

def source(name):
    return (ticket / name).read_text(encoding="utf-8")

changes = {}

def edit(path, substitutions):
    text = path.read_text(encoding="utf-8")
    for old, new in substitutions:
        assert text.count(old) == 1, (path, old)
        text = text.replace(old, new)
    changes[path] = text

edit(provider, [
    ('import { dirname, join } from "node:path";',
     'import { mkdir, realpath } from "node:fs/promises";\nimport { dirname, isAbsolute, join } from "node:path";'),
    ('import { appendArchive, eventLogPath, readLatestArchive }',
     'import { appendArchive, eventLogPath, readLatestArchive, withFolderEventWriter }'),
    ('import { backboneDatabaseCtorLazy,',
     'import type { FolderFirstArchiveCapture, NativeFolderJournal, NativeFolderJournalScope } from "./📞️native-journal.ts";\nimport { backboneDatabaseCtorLazy,'),
    ('    const bytes = readLatestArchive(eventLogPath(folder), id);',
     '    const bytes = await readLatestArchive(eventLogPath(folder), id);'),
    ('schema: string | null, payload: Uint8Array): Promise<void> {',
     'schema: string | null, payload: Uint8Array, journal?: NativeFolderJournal): Promise<void> {'),
    ('    appendArchive(eventLogPath(folder), id, schema ?? "", payload);',
     '''    const path = eventLogPath(folder);
    if (journal) await withFolderEventWriter(path, () => journal.put(folder, id, schema ?? "", payload));
    else await appendArchive(path, id, schema ?? "", payload);'''),
    ('/** 👁️ One debounced', source("browser-first-archive-provider.ts") + '\n/** 👁️ One debounced'),
])
edit(plugin, [
    ('import { devStreamMuxServer,', '''import { nativeFolderJournal } from "../🗄️backbone-storage/📞️native-journal.ts";
import { firstArchiveFolderRoute } from "./🧾️first-archive.ts";
import { devStreamMuxServer,'''),
    ('write: (chunk: string) => void; end:',
     'write: (chunk: string) => void; on?: (event: "close", handler: () => void) => void; off?: (event: "close", handler: () => void) => void; end:'),
    ('export function semioBackboneVitePlugin() {',
     '''export function semioBackboneVitePlugin() {
  const binary = process.env.SEMIO_FOLDER_JOURNAL_BIN;
  const journal = binary ? nativeFolderJournal(binary) : undefined;
  const dataRoot = process.env.S_DATA_DIR;
  const firstArchive = journal && dataRoot ? firstArchiveFolderRoute(
    `${BACKBONE_ENDPOINT_PATH}/first-archive`, dataRoot, journal, collectBackboneRequestBody,
  ) : undefined;'''),
    ('        const uri = requestUrl.searchParams.get("uri");',
     '''        if (firstArchive?.(req, res, requestUrl)) return;
        const uri = requestUrl.searchParams.get("uri");'''),
    ('writeBackbonePayload(uri, documentId, schema, body)',
     'writeBackbonePayload(uri, documentId, schema, body, journal)'),
])
edit(native, [
    ('[features]\n', '''[[bin]]
name = "semio-folder-journal"
path = "../../⌨️folder-journal/🦀️.rs"
required-features = ["folder-journal"]

[features]
folder-journal = []
'''),
])
edit(serve, [
    ('import {frameworkOsEngineFacetV1}', '''import { ensureNativeFolderJournalV1 } from "./📞️native-journal.ts";
import {frameworkOsEngineFacetV1}'''),
    ('    const phase=scriptCompilerAdmissionControlV1(this.invocation,"vite:activation",0);',
     '''    const folderJournal = await ensureNativeFolderJournalV1(this.repoRoot, this.invocation);
    const phase=scriptCompilerAdmissionControlV1(this.invocation,"vite:activation",0);'''),
    ('env: { SEMIO_PLUGIN: variant,', 'env: { SEMIO_FOLDER_JOURNAL_BIN: folderJournal, SEMIO_PLUGIN: variant,'),
])
process = next(root.glob("🧰*/🔨*/🏃*process/📦*/🟦*typescript"))
provision = source("browser-folder-stdio-provision.ts")
for marker, path in {
    "PROCESS_CARGO_BUILD": process / "🧪️testing/🦀️cargo/🟦️.ts",
    "PROCESS_CARGO_RECEIPT": process / "📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts",
    "PROCESS_SCRIPT_ROUTING": process / "🧭️routing/🟦️.ts",
}.items():
    assert path.is_file(), path
    provision = provision.replace(marker, Path(os.path.relpath(path, serve.parent)).as_posix())
provision = provision.replace("NATIVE_STORE_MANIFEST", native.relative_to(root).as_posix())
changes[serve.with_name("📞️native-journal.ts")] = provision
changes[native.parent.parent.parent / "⌨️folder-journal/🦀️.rs"] = source("browser-folder-stdio.rs")
changes[storage / "📞️native-journal.ts"] = source("browser-folder-stdio.ts")
changes[plugin.with_name("🧾️first-archive.ts")] = source("browser-first-archive-route.ts")
journal_source = source("browser-first-archive-journal.ts").split("async function validateFirstArchive", 1)[0]
journal_source = journal_source.replace('import { createHash } from "node:crypto";\n', "")
journal_source = journal_source.replace("FileHandle, mkdir, mkdtemp, open, realpath, rm", "FileHandle, mkdir, open, realpath")
journal_source = journal_source[:journal_source.index("type Writer")] + journal_source[journal_source.index("const writers"):]
journal_source = journal_source[:journal_source.index("async function hashRange")] + journal_source[journal_source.index("async function scan"):]
changes[journal] = journal_source.rstrip() + "\n"

patch = []
for path, revised in changes.items():
    assert len(str(path).encode("utf-16-le")) // 2 <= 256
    relative = path.relative_to(root).as_posix()
    original = path.read_text(encoding="utf-8").splitlines(True) if path.exists() else []
    patch.extend(unified_diff(original, revised.splitlines(True), f"a/{relative}" if path.exists() else "/dev/null", f"b/{relative}"))
(output / "browser-first-archive.patch").write_text("".join(patch), encoding="utf-8")

proof_route = source("browser-first-archive-route.ts").replace("../🗄️backbone-storage/🟦️.ts", "./proof-bound.ts").replace(
    "../🗄️backbone-storage/📞️native-journal.ts", "../../browser-folder-stdio.ts").replace(
    'import type { BackboneServerRequest, BackboneServerResponse } from "./🟦️.ts";',
    'import type { IncomingMessage as BackboneServerRequest, ServerResponse as BackboneServerResponse } from "node:http";')
(output / "proof-route.ts").write_text(proof_route, encoding="utf-8")
proof_bound = '''import { existsSync } from "node:fs";
import { mkdir, realpath } from "node:fs/promises";
import { isAbsolute, join } from "node:path";
import type { FolderFirstArchiveCapture, NativeFolderJournal, NativeFolderJournalScope } from "../../browser-folder-stdio.ts";
function canonicalBootstrapMirrorError(code: string) { return new Error(code); }
async function backboneDbHandleFor(_: string): Promise<{query(_: string): {get(_: string): unknown}}> {
  throw Error("the archive route fixture requires a fresh folder without any canonical mirror");
}
'''
(output / "proof-bound.ts").write_text(proof_bound + source("browser-first-archive-provider.ts"), encoding="utf-8")
