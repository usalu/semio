#!/usr/bin/env python3
"""🪪️ C13 prepared set P3 (T6 queue, guest-linked): a surface opens the document its subset's codec names.

Measured 2026-09-29 on hub 7800 p33 (C12 collab STEP 14): no hub puzzle 3d document ever opens —
`loadDocumentArchive(s.puzzle.puzzle3d@1/*#editor)` fails `document archive parent Pack and SPR hydration was rejected:
Identity`. Root cause: the hub creates (genesis) every puzzle 3d document with the subset's codec schema `puzzle.3d`
(`PUZZLE_3D_SCHEMA`, also the viewer's `DOCUMENT_SCHEMA`), while the editor declares `DOCUMENT_SCHEMA = "puzzle.3d.fixture"`
(the fixture CONTENT tag); the parent hydration compares the history's schema with the opening app's and refuses. 2d
(`puzzle.2d.fixture` both) and 5d (`puzzle.5d` both) agree; a static scan of all 449 surface `DOCUMENT_SCHEMA`s against their
subset codecs finds puzzle 3d's editor the only drift.

Part A (puzzle 3d): the editor and its retained tool-job factory open `crate::PUZZLE_3D_SCHEMA`; everything derived from the
document schema follows (tool proofs' `artifact_schema`, the retained payload schema `<schema>.tool-command.v1`, the retained-jobs
fixture, the test catalog tuple, the legacy codec capability row `9:puzzle.3d:puzzle3d-play`, a doc line). The fixture content
tag `PUZZLE3D_FIXTURE_SCHEMA` stays what it is (the JSON fixture's own `schema` field).
Part B (SDK): `preflight_artifact_declarations` refuses a subset whose editor or viewer `document_schema` differs from its codec's
schema (`plugin-assembly.declaration-document-schema`), so this drift fails every describe instead of every hub open.

usage: p3-puzzle3d-document-schema.py [--root <repo or overlay>] [--parts A|B|A,B] (--dry-run | --write | --revert)
  --parts   A = puzzle 3d, B = the SDK guard (default A,B); B alone only after A (A removes the one known drift)
  --write   byte-backs up every touched file to `.🧬semio/🌐hub/s14-c13-w3-backup/c13-p3/before/` (+ `after/`) first
  --revert  restores `before/` for every file whose live bytes still equal `after/`; a file edited since is listed and kept"""
import json
import pathlib
import sys

args = sys.argv[1:]
root = pathlib.Path(args[args.index("--root") + 1]) if "--root" in args else pathlib.Path("/Users/ueli/Documents/semio")
modes = [mode for mode in ("--dry-run", "--write", "--revert") if mode in args]
if len(modes) != 1:
    sys.exit("usage: p3-puzzle3d-document-schema.py [--root <dir>] (--dry-run | --write | --revert)")
mode = modes[0]
parts = set((args[args.index("--parts") + 1] if "--parts" in args else "A,B").split(","))
backup = pathlib.Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c13-w3-backup/c13-p3")

P3D = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d"
EDITOR = f"{P3D}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
CRATE = f"{P3D}/🦀️.rs"
STANDARD = f"{P3D}/🏅️standards/🔖️1/🦀️.rs"
RETAINED = f"{P3D}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"

HUNKS: dict[str, list[tuple[str, str, int]]] = {
    EDITOR: [
        ("    const DOCUMENT_SCHEMA: &'static str = PUZZLE3D_FIXTURE_SCHEMA;\n", "    const DOCUMENT_SCHEMA: &'static str = crate::PUZZLE_3D_SCHEMA;\n", 2),
        ('        artifact_schema: "puzzle.3d.fixture",\n', '        artifact_schema: "puzzle.3d",\n', 2),
        ('const PUZZLE3D_RETAINED_PAYLOAD_SCHEMA: &str = "puzzle.3d.fixture.tool-command.v1";', 'const PUZZLE3D_RETAINED_PAYLOAD_SCHEMA: &str = "puzzle.3d.tool-command.v1";', 1),
    ],
    CRATE: [
        ('    ("puzzle3d", "puzzle.3d.fixture", editor::puzzle3d::PUZZLE3D_RETAINED_TOOL_IDS,', '    ("puzzle3d", "puzzle.3d", editor::puzzle3d::PUZZLE3D_RETAINED_TOOL_IDS,', 1),
        (
            '("s.puzzle.puzzle3d.codec.document-1", "codec", "puzzle.3d.fixture:puzzle3d-play", &[("codec", "puzzle.3d.fixture"), ("codec-extension", "17:puzzle.3d.fixture:puzzle3d-play")], None),',
            '("s.puzzle.puzzle3d.codec.document-1", "codec", "puzzle.3d:puzzle3d-play", &[("codec", "puzzle.3d"), ("codec-extension", "9:puzzle.3d:puzzle3d-play")], None),',
            1,
        ),
    ],
    STANDARD: [
        ("//! (`puzzle.3d.fixture:puzzle3d-play`) and an extension", "//! (`puzzle.3d:puzzle3d-play`) and an extension", 1),
    ],
    RETAINED: [
        ('  "documentSchema": "puzzle.3d.fixture",\n  "payloadSchema": "puzzle.3d.fixture.tool-command.v1",', '  "documentSchema": "puzzle.3d",\n  "payloadSchema": "puzzle.3d.tool-command.v1",', 1),
    ],
    SDK: [
        (
            """                        check_surface_id(subset, &subset.editor, AppRole::Editor)?;
                        check_surface_id(subset, &subset.viewer, AppRole::Viewer)?;
""",
            """                        check_surface_id(subset, &subset.editor, AppRole::Editor)?;
                        check_surface_id(subset, &subset.viewer, AppRole::Viewer)?;
                        check_surface_document_schema(subset, &subset.editor)?;
                        check_surface_document_schema(subset, &subset.viewer)?;
""",
            1,
        ),
        (
            """        fn check_surface_id<PA: PluginApp>(subset: &SubsetDeclaration<PA>, surface: &SurfaceDeclaration<PA>, role: AppRole) -> Result<(), PluginAssemblyError> {""",
            """        /// 🪪️ A surface opens the document its subset's codec names: the hub creates every document of the subset with
        /// `io.native.codec.schema` in its genesis history, and a surface whose `DOCUMENT_SCHEMA` differs refuses that history at
        /// hydration (`Identity`) — measured on hub 7800 p33, where the puzzle 3d editor named `puzzle.3d.fixture` against its
        /// codec's (and viewer's) `puzzle.3d` and no hub puzzle 3d document ever opened. Refused here, the drift fails `describe`.
        fn check_surface_document_schema<PA: PluginApp>(subset: &SubsetDeclaration<PA>, surface: &SurfaceDeclaration<PA>) -> Result<(), PluginAssemblyError> {
            if surface.document_schema != subset.io.native.codec.schema {
                return Err(PluginAssemblyError::new(
                    "plugin-assembly.declaration-document-schema",
                    format!("surface {:?} opens document schema {:?} but its subset's codec names {:?}", surface.definition.id, surface.document_schema, subset.io.native.codec.schema),
                ));
            }
            Ok(())
        }

        fn check_surface_id<PA: PluginApp>(subset: &SubsetDeclaration<PA>, surface: &SurfaceDeclaration<PA>, role: AppRole) -> Result<(), PluginAssemblyError> {""",
            1,
        ),
    ],
}

if mode == "--revert":
    manifest = json.loads((backup / "manifest.json").read_text(encoding="utf-8"))
    kept = 0
    for relative in manifest["files"]:
        live = root / relative
        if live.read_bytes() != (backup / "after" / relative).read_bytes():
            kept += 1
            print(f"{relative}: KEPT (edited since the write)")
            continue
        live.write_bytes((backup / "before" / relative).read_bytes())
        print(f"{relative}: reverted")
    print(f"REVERTED: {kept} file(s) kept")
    sys.exit(1 if kept else 0)

problems = 0
writes: dict[pathlib.Path, str] = {}
for relative, hunks in HUNKS.items():
    if ("B" if relative == SDK else "A") not in parts:
        continue
    path = root / relative
    original = path.read_text(encoding="utf-8")
    text = original
    for old, new, count in hunks:
        if old in new:
            state = "already applied" if text.count(new) == count else "applies" if text.count(old) == count else f"MISSING (old {text.count(old)}, new {text.count(new)}, want {count})"
        else:
            state = "applies" if text.count(old) == count else "already applied" if text.count(new) == count and text.count(old) == 0 else f"MISSING (old {text.count(old)}, new {text.count(new)}, want {count})"
        if state == "applies":
            text = text.replace(old, new)
        if state.startswith("MISSING"):
            problems += 1
        print(f"{relative}: {state}: {old.strip().splitlines()[0][:90]}")
    if text != original:
        writes[path] = text
write = mode == "--write" and problems == 0
if write:
    files = [str(path.relative_to(root)) for path in writes]
    for path, text in writes.items():
        relative = path.relative_to(root)
        for side, data in (("before", path.read_bytes()), ("after", text.encode("utf-8"))):
            target = backup / side / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
    (backup / "manifest.json").write_text(json.dumps({"root": str(root), "files": files}, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    for path, text in writes.items():
        path.write_text(text, encoding="utf-8")
print(f"{'WRITTEN' if write else 'DRY RUN'}: {problems} problem(s), {len(writes)} file(s)")
sys.exit(1 if problems else 0)
