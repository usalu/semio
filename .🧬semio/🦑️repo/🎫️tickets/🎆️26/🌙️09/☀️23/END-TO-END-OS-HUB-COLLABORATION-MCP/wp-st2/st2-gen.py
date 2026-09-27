#!/usr/bin/env python3
"""🧬️ ST1 generator: derives the per-family stdio component packages (Option A) from the CURRENT stdio plugin assembly.

Reads (never writes) the repo tree:
  - `✏️s/🔌️plugins/🗄️stdio/🔌️plugin/🦀️.rs` — the full-app-catalog `StdioApps` enum + `register_apps` (type paths,
    create fns, mutation-roster calls), so every family registers exactly what the library fleet registered;
  - each editor's `fn examples()` + its subset `📚️examples/<emoji><id>/🦀️.rs` — the pane example id and label;
  - `🧫️fixtures/✏️editor-catalog/🔣️.json` — the 88 editor app ids (validation of the hand table below).
Writes `payload/` (new files, repo-relative tree) and `plan.json` (data the apply step splices into edited files).

usage: python3 st1-gen.py [--repo /Users/ueli/Documents/semio]
"""
import argparse
import json
import os
import re
import shutil
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
STDIO = "✏️s/🔌️plugins/🗄️stdio"
EXT = f"{STDIO}/🧩️extensions"
CEILING = 40
PORT_REACT0, PORT_WGPU0 = 6219, 6319
PORT_SKIP = {6222, 6274, 6277}

# 🗂️ family id, dir emoji, dir slug, label, play group (id, label), description, artifact crates (format keys), enum
FAMILIES = [
    ("stdio-image", "🖼️", "image", "Stdio Image", ("images", "Images"), "the png/jpg/bmp/tiff/gif/svg raster and vector image apps", ["png", "jpg", "bmp", "tiff", "gif", "svg"], "StdioImageApps"),
    ("stdio-media", "🎵️", "media", "Stdio Media", ("audio-video", "Audio & Video"), "the mp4/mp3/wav/avi audio and video apps", ["mp4", "mp3", "wav", "avi"], "StdioMediaApps"),
    ("stdio-cad", "🛠️", "cad", "Stdio CAD", ("cad-exchange", "CAD Exchange"), "the STEP AP214, DXF and DWG CAD exchange apps", ["step", "dxf", "dwg"], "StdioCadApps"),
    ("stdio-bim", "🏠️", "bim", "Stdio BIM", ("bim", "BIM"), "the IFC and BCF building-information apps", ["ifc", "bcf"], "StdioBimApps"),
    ("stdio-mesh", "🔺️", "mesh", "Stdio Mesh", ("meshes", "Meshes & Point Clouds"), "the glTF/OBJ/STL/PLY mesh and LAS point-cloud apps", ["gltf", "obj", "stl", "ply", "las"], "StdioMeshApps"),
    ("stdio-pdf", "📘️", "pdf", "Stdio PDF", ("pdf", "PDF"), "the PDF 1.4 and 1.7 document apps with their ISO profiles", ["pdf"], "StdioPdfApps"),
    ("stdio-office", "💼️", "office", "Stdio Office", ("office", "Office"), "the Office Open XML docx/pptx/xlsx apps", ["docx", "pptx", "xlsx"], "StdioOfficeApps"),
    ("stdio-semio", "🧿️", "semio", "Stdio Semio", ("semio-files", "Semio Files"), "the nineteen semio v1 file apps", ["semio"], "StdioSemioApps"),
    ("stdio-binary", "🔢️", "binary", "Stdio Binary", ("binary", "Binary & Archives"), "the raw binary, deflate, zip and EnergyPlus weather apps", ["binary", "deflate", "zip", "epw"], "StdioBinaryApps"),
]
BASE_FORMATS = ["html", "md", "csv", "tsv", "txt", "json", "xml"]

# ✏️ editor type → (app id, variant, alias, pane label, tagline, description, icon). One row per shipped editor.
SUBSETS = {
    "PngEditor": ("s.stdio.png@1.2/*#editor", "stdio-png", "stdio png", "PNG", "Edit PNG images", "Open PNG 1.2 images and edit every decoded chunk and pixel field.", "image"),
    "JpgAnyEditor": ("s.stdio.jpg@jfif-1.01/*#editor", "stdio-jpg", "stdio jpeg", "JPEG", "Edit JPEG photos", "Open JFIF 1.01 JPEG photos and edit their decoded segments field by field.", "camera"),
    "JpgBaselineEditor": ("s.stdio.jpg@jfif-1.01/baseline#editor", "stdio-jpg-baseline", "stdio baseline jpeg", "Baseline JPEG", "Edit baseline JPEG photos", "Edit JPEG photos restricted to the baseline sequential process.", "camera"),
    "BmpEditor": ("s.stdio.bmp@v3/*#editor", "stdio-bmp", "stdio bitmap", "BMP", "Edit bitmap images", "Open Windows BMP v3 bitmaps and edit their header and pixel data.", "file-image"),
    "TiffAnyEditor": ("s.stdio.tiff@6.0/*#editor", "stdio-tiff", "stdio tiff", "TIFF", "Edit TIFF images", "Open TIFF 6.0 images and edit their image file directories and tags.", "file-image"),
    "TiffBaselineEditor": ("s.stdio.tiff@6.0/baseline#editor", "stdio-tiff-baseline", "stdio baseline tiff", "Baseline TIFF", "Edit baseline TIFF images", "Edit TIFF images restricted to the baseline tag set.", "file-image"),
    "Gif87aEditor": ("s.stdio.gif@87a/*#editor", "stdio-gif-87a", "stdio gif 87a", "GIF 87a", "Edit GIF 87a images", "Open GIF 87a images and edit their frames and color tables.", "image"),
    "Gif89aEditor": ("s.stdio.gif@89a/*#editor", "stdio-gif-89a", "stdio gif 89a", "GIF 89a", "Edit GIF 89a images", "Open GIF 89a images and edit frames, color tables and extensions.", "image"),
    "SvgAnyEditor": ("s.stdio.svg@1.1/*#editor", "stdio-svg", "stdio svg", "SVG", "Edit vector graphics", "Open SVG 1.1 vector graphics and edit their elements and attributes.", "pen-tool"),
    "SvgBasicEditor": ("s.stdio.svg@1.1/basic#editor", "stdio-svg-basic", "stdio svg basic", "SVG Basic", "Edit SVG Basic graphics", "Edit SVG 1.1 graphics restricted to the SVG Basic profile.", "pen-tool"),
    "SvgTinyEditor": ("s.stdio.svg@1.1/tiny#editor", "stdio-svg-tiny", "stdio svg tiny", "SVG Tiny", "Edit SVG Tiny graphics", "Edit SVG 1.1 graphics restricted to the SVG Tiny profile.", "pen-tool"),
    "Mp4Editor": ("s.stdio.mp4@isobmff/*#editor", "stdio-mp4", "stdio mp4", "MP4", "Edit MP4 videos", "Open ISO base media MP4 files and edit their boxes and tracks.", "file-video"),
    "Mp3Editor": ("s.stdio.mp3@mpeg1-layer3/*#editor", "stdio-mp3", "stdio mp3", "MP3", "Edit MP3 audio", "Open MPEG-1 Layer III audio and edit its frames and tags.", "play"),
    "WavEditor": ("s.stdio.wav@riff-pcm/*#editor", "stdio-wav", "stdio wave", "WAV", "Edit WAV audio", "Open RIFF PCM wave audio and edit its format and sample chunks.", "play"),
    "AviEditor": ("s.stdio.avi@1.0/*#editor", "stdio-avi", "stdio avi", "AVI", "Edit AVI videos", "Open AVI 1.0 videos and edit their RIFF chunks and stream headers.", "file-video"),
    "StepAnyEditor": ("s.stdio.step@ap214/*#editor", "stdio-step", "stdio step", "STEP", "Edit STEP models", "Open ISO 10303 AP214 STEP exchange files and edit their entities.", "cad-shape"),
    "StepCc1Editor": ("s.stdio.step@ap214/cc1#editor", "stdio-step-cc1", "stdio step cc1", "STEP CC1", "Edit STEP CC1 models", "Edit STEP AP214 files restricted to conformance class 1.", "cad-shape"),
    "StepCc2Editor": ("s.stdio.step@ap214/cc2#editor", "stdio-step-cc2", "stdio step cc2", "STEP CC2", "Edit STEP CC2 models", "Edit STEP AP214 files restricted to conformance class 2.", "cad-shape"),
    "StepCc3Editor": ("s.stdio.step@ap214/cc3#editor", "stdio-step-cc3", "stdio step cc3", "STEP CC3", "Edit STEP CC3 models", "Edit STEP AP214 files restricted to conformance class 3.", "cad-shape"),
    "StepCc4Editor": ("s.stdio.step@ap214/cc4#editor", "stdio-step-cc4", "stdio step cc4", "STEP CC4", "Edit STEP CC4 models", "Edit STEP AP214 files restricted to conformance class 4.", "cad-shape"),
    "StepCc5Editor": ("s.stdio.step@ap214/cc5#editor", "stdio-step-cc5", "stdio step cc5", "STEP CC5", "Edit STEP CC5 models", "Edit STEP AP214 files restricted to conformance class 5.", "cad-shape"),
    "StepCc6Editor": ("s.stdio.step@ap214/cc6#editor", "stdio-step-cc6", "stdio step cc6", "STEP CC6", "Edit STEP CC6 models", "Edit STEP AP214 files restricted to conformance class 6.", "cad-shape"),
    "DxfAnyEditor": ("s.stdio.dxf@r12/*#editor", "stdio-dxf", "stdio dxf", "DXF", "Edit DXF drawings", "Open R12 DXF drawings and edit their sections, tables and entities.", "draw"),
    "DwgAc1018Editor": ("s.stdio.dwg@ac1018/*#editor", "stdio-dwg-ac1018", "stdio dwg ac1018", "DWG AC1018", "Edit DWG AC1018 drawings", "Open DWG drawings of format version AC1018 and edit their objects.", "draw"),
    "DwgAc1024Editor": ("s.stdio.dwg@ac1024/*#editor", "stdio-dwg-ac1024", "stdio dwg ac1024", "DWG AC1024", "Edit DWG AC1024 drawings", "Open DWG drawings of format version AC1024 and edit their objects.", "draw"),
    "Ifc2x3AnyEditor": ("s.stdio.ifc@2x3/*#editor", "stdio-ifc-2x3", "stdio ifc 2x3", "IFC 2x3", "Edit IFC 2x3 models", "Open IFC 2x3 building models and edit their entities.", "building"),
    "Ifc2x3CobieEditor": ("s.stdio.ifc@2x3/cobie#editor", "stdio-ifc-2x3-cobie", "stdio ifc cobie", "IFC COBie", "Edit COBie handover models", "Edit IFC 2x3 models restricted to the COBie handover view.", "building"),
    "Ifc2x3Cv20Editor": ("s.stdio.ifc@2x3/cv20#editor", "stdio-ifc-2x3-cv20", "stdio ifc coordination view", "IFC Coordination View", "Edit coordination models", "Edit IFC 2x3 models restricted to Coordination View 2.0.", "building"),
    "Ifc2x3SavEditor": ("s.stdio.ifc@2x3/sav#editor", "stdio-ifc-2x3-sav", "stdio ifc structural analysis view", "IFC Structural Analysis View", "Edit structural analysis models", "Edit IFC 2x3 models restricted to the Structural Analysis View.", "building"),
    "Ifc4AnyEditor": ("s.stdio.ifc@4/*#editor", "stdio-ifc-4", "stdio ifc 4", "IFC 4", "Edit IFC 4 models", "Open IFC 4 building models and edit their entities.", "building"),
    "BcfAnyEditor": ("s.stdio.bcf@2.1/*#editor", "stdio-bcf", "stdio bcf", "BCF", "Edit BIM issues", "Open BIM Collaboration Format 2.1 archives and edit topics, comments and viewpoints.", "message-square"),
    "GltfAnyEditor": ("s.stdio.gltf@2.0/*#editor", "stdio-gltf", "stdio gltf", "glTF", "Edit glTF scenes", "Open glTF 2.0 scenes and edit nodes, meshes, materials and animations.", "scene-3d"),
    "ObjAnyEditor": ("s.stdio.obj@3.0/*#editor", "stdio-obj", "stdio obj", "OBJ", "Edit OBJ meshes", "Open Wavefront OBJ meshes and edit their vertices, faces and groups.", "box"),
    "StlAnyEditor": ("s.stdio.stl@ascii/*#editor", "stdio-stl", "stdio stl", "STL", "Edit STL meshes", "Open ASCII STL meshes and edit their facets.", "triangle"),
    "PlyAnyEditor": ("s.stdio.ply@1.0/*#editor", "stdio-ply", "stdio ply", "PLY", "Edit PLY meshes", "Open PLY 1.0 polygon files and edit their elements and properties.", "hexagon"),
    "LasAnyEditor": ("s.stdio.las@1.0/*#editor", "stdio-las", "stdio las", "LAS", "Edit point clouds", "Open LAS 1.0 point clouds and edit their header, records and points.", "terrain-3d"),
    "Pdf14Editor": ("s.stdio.pdf@1.4/*#editor", "stdio-pdf-1-4", "stdio pdf 1.4", "PDF 1.4", "Edit PDF 1.4 documents", "Open PDF 1.4 documents and edit their objects and pages.", "document-report"),
    "Pdf14AEditor": ("s.stdio.pdf@1.4/a#editor", "stdio-pdf-1-4-a", "stdio pdf 1.4 archival", "PDF/A (1.4)", "Edit archival PDFs", "Edit PDF 1.4 documents restricted to the PDF/A archival profile.", "document-report"),
    "Pdf14XEditor": ("s.stdio.pdf@1.4/x#editor", "stdio-pdf-1-4-x", "stdio pdf 1.4 print", "PDF/X (1.4)", "Edit print-exchange PDFs", "Edit PDF 1.4 documents restricted to the PDF/X print exchange profile.", "document-report"),
    "Pdf17Editor": ("s.stdio.pdf@1.7/*#editor", "stdio-pdf-1-7", "stdio pdf 1.7", "PDF 1.7", "Edit PDF 1.7 documents", "Open PDF 1.7 documents and edit their objects and pages.", "document-report"),
    "Pdf17AEditor": ("s.stdio.pdf@1.7/a#editor", "stdio-pdf-1-7-a", "stdio pdf 1.7 archival", "PDF/A (1.7)", "Edit archival PDFs", "Edit PDF 1.7 documents restricted to the PDF/A archival profile.", "document-report"),
    "Pdf17EEditor": ("s.stdio.pdf@1.7/e#editor", "stdio-pdf-1-7-e", "stdio pdf engineering", "PDF/E", "Edit engineering PDFs", "Edit PDF 1.7 documents restricted to the PDF/E engineering profile.", "document-report"),
    "Pdf17HEditor": ("s.stdio.pdf@1.7/h#editor", "stdio-pdf-1-7-h", "stdio pdf healthcare", "PDF/H", "Edit healthcare PDFs", "Edit PDF 1.7 documents restricted to the PDF/H healthcare profile.", "document-report"),
    "Pdf17UaEditor": ("s.stdio.pdf@1.7/ua#editor", "stdio-pdf-1-7-ua", "stdio pdf accessible", "PDF/UA", "Edit accessible PDFs", "Edit PDF 1.7 documents restricted to the PDF/UA accessibility profile.", "document-report"),
    "Pdf17VtEditor": ("s.stdio.pdf@1.7/vt#editor", "stdio-pdf-1-7-vt", "stdio pdf variable print", "PDF/VT", "Edit variable-print PDFs", "Edit PDF 1.7 documents restricted to the PDF/VT variable printing profile.", "document-report"),
    "Pdf17XEditor": ("s.stdio.pdf@1.7/x#editor", "stdio-pdf-1-7-x", "stdio pdf 1.7 print", "PDF/X (1.7)", "Edit print-exchange PDFs", "Edit PDF 1.7 documents restricted to the PDF/X print exchange profile.", "document-report"),
    "DocxEditor": ("s.stdio.docx@ecma-376/*#editor", "stdio-docx", "stdio docx", "Word Document", "Edit word-processing documents", "Open ECMA-376 docx documents and edit their parts and paragraphs.", "file-text"),
    "DocxStrictEditor": ("s.stdio.docx@ecma-376/strict#editor", "stdio-docx-strict", "stdio docx strict", "Word Document (Strict)", "Edit strict docx documents", "Edit docx documents restricted to the ECMA-376 strict conformance class.", "file-text"),
    "DocxTransitionalEditor": ("s.stdio.docx@ecma-376/transitional#editor", "stdio-docx-transitional", "stdio docx transitional", "Word Document (Transitional)", "Edit transitional docx documents", "Edit docx documents under the ECMA-376 transitional conformance class.", "file-text"),
    "PptxEditor": ("s.stdio.pptx@ecma-376/*#editor", "stdio-pptx", "stdio pptx", "Presentation", "Edit presentations", "Open ECMA-376 pptx presentations and edit their parts and slides.", "monitor"),
    "PptxStrictEditor": ("s.stdio.pptx@ecma-376/strict#editor", "stdio-pptx-strict", "stdio pptx strict", "Presentation (Strict)", "Edit strict pptx presentations", "Edit pptx presentations restricted to the ECMA-376 strict conformance class.", "monitor"),
    "PptxTransitionalEditor": ("s.stdio.pptx@ecma-376/transitional#editor", "stdio-pptx-transitional", "stdio pptx transitional", "Presentation (Transitional)", "Edit transitional pptx presentations", "Edit pptx presentations under the ECMA-376 transitional conformance class.", "monitor"),
    "XlsxEditor": ("s.stdio.xlsx@ecma-376/*#editor", "stdio-xlsx", "stdio xlsx", "Spreadsheet", "Edit spreadsheets", "Open ECMA-376 xlsx workbooks and edit their parts and cells.", "file-spreadsheet"),
    "XlsxStrictEditor": ("s.stdio.xlsx@ecma-376/strict#editor", "stdio-xlsx-strict", "stdio xlsx strict", "Spreadsheet (Strict)", "Edit strict xlsx workbooks", "Edit xlsx workbooks restricted to the ECMA-376 strict conformance class.", "file-spreadsheet"),
    "XlsxTransitionalEditor": ("s.stdio.xlsx@ecma-376/transitional#editor", "stdio-xlsx-transitional", "stdio xlsx transitional", "Spreadsheet (Transitional)", "Edit transitional xlsx workbooks", "Edit xlsx workbooks under the ECMA-376 transitional conformance class.", "file-spreadsheet"),
    "SemioAnyEditor": ("s.stdio.semio@v1/*#editor", "stdio-semio", "stdio semio", "Semio File", "Edit semio files", "Open semio v1 files of any content and edit their documents.", "box"),
    "SemioAnimationEditor": ("s.stdio.semio@v1/animation#editor", "stdio-semio-animation", "stdio semio animation", "Semio Animation", "Edit semio animations", "Edit semio v1 files that carry an animation document.", "animate"),
    "SemioAudioEditor": ("s.stdio.semio@v1/audio#editor", "stdio-semio-audio", "stdio semio audio", "Semio Audio", "Edit semio audio", "Edit semio v1 files that carry an audio document.", "play"),
    "SemioBrepEditor": ("s.stdio.semio@v1/brep#editor", "stdio-semio-brep", "stdio semio brep", "Semio B-Rep", "Edit semio solids", "Edit semio v1 files that carry a boundary-representation solid.", "cad-shape"),
    "SemioCadEditor": ("s.stdio.semio@v1/cad#editor", "stdio-semio-cad", "stdio semio cad", "Semio CAD", "Edit semio CAD models", "Edit semio v1 files that carry a CAD model.", "cad-shape"),
    "SemioDocumentEditor": ("s.stdio.semio@v1/document#editor", "stdio-semio-document", "stdio semio document", "Semio Document", "Edit semio documents", "Edit semio v1 files that carry a text document.", "file-text"),
    "SemioDrawingEditor": ("s.stdio.semio@v1/drawing#editor", "stdio-semio-drawing", "stdio semio drawing", "Semio Drawing", "Edit semio drawings", "Edit semio v1 files that carry a drawing.", "draw"),
    "SemioFlowEditor": ("s.stdio.semio@v1/flow#editor", "stdio-semio-flow", "stdio semio flow", "Semio Flow", "Edit semio flows", "Edit semio v1 files that carry a flow graph.", "flow"),
    "SemioGraphEditor": ("s.stdio.semio@v1/graph#editor", "stdio-semio-graph", "stdio semio graph", "Semio Graph", "Edit semio graphs", "Edit semio v1 files that carry a graph.", "network"),
    "SemioImageEditor": ("s.stdio.semio@v1/image#editor", "stdio-semio-image", "stdio semio image", "Semio Image", "Edit semio images", "Edit semio v1 files that carry an image.", "image"),
    "SemioKitEditor": ("s.stdio.semio@v1/kit#editor", "stdio-semio-kit", "stdio semio kit", "Semio Kit", "Edit semio kits", "Edit semio v1 files that carry a kit of types and designs.", "component"),
    "SemioMeshEditor": ("s.stdio.semio@v1/mesh#editor", "stdio-semio-mesh", "stdio semio mesh", "Semio Mesh", "Edit semio meshes", "Edit semio v1 files that carry a mesh.", "triangle"),
    "SemioModelEditor": ("s.stdio.semio@v1/model#editor", "stdio-semio-model", "stdio semio model", "Semio Model", "Edit semio models", "Edit semio v1 files that carry a model.", "building"),
    "SemioObjectEditor": ("s.stdio.semio@v1/object#editor", "stdio-semio-object", "stdio semio object", "Semio Object", "Edit semio objects", "Edit semio v1 files that carry an object.", "box"),
    "SemioPresentationEditor": ("s.stdio.semio@v1/presentation#editor", "stdio-semio-presentation", "stdio semio presentation", "Semio Presentation", "Edit semio presentations", "Edit semio v1 files that carry a presentation.", "monitor"),
    "SemioTableEditor": ("s.stdio.semio@v1/table#editor", "stdio-semio-table", "stdio semio table", "Semio Table", "Edit semio tables", "Edit semio v1 files that carry a table.", "table-2"),
    "SemioTextEditor": ("s.stdio.semio@v1/text#editor", "stdio-semio-text", "stdio semio text", "Semio Text", "Edit semio text", "Edit semio v1 files that carry plain text.", "type"),
    "SemioValueEditor": ("s.stdio.semio@v1/value#editor", "stdio-semio-value", "stdio semio value", "Semio Value", "Edit semio values", "Edit semio v1 files that carry a structured value.", "hash"),
    "SemioVideoEditor": ("s.stdio.semio@v1/video#editor", "stdio-semio-video", "stdio semio video", "Semio Video", "Edit semio videos", "Edit semio v1 files that carry a video.", "file-video"),
    "BinaryEditor": ("s.stdio.binary@raw/*#editor", "stdio-binary", "stdio binary", "Binary", "Edit raw bytes", "Open any file as raw bytes and edit them.", "hard-drive"),
    "DeflateEditor": ("s.stdio.deflate@rfc1950/*#editor", "stdio-deflate", "stdio deflate", "zlib", "Edit zlib streams", "Open RFC 1950 zlib streams and edit their decompressed payload.", "file-archive"),
    "ZipAnyEditor": ("s.stdio.zip@2.0/*#editor", "stdio-zip", "stdio zip", "ZIP", "Edit ZIP archives", "Open ZIP 2.0 archives and edit their entries.", "file-archive"),
    "ZipIso21320Editor": ("s.stdio.zip@2.0/iso21320#editor", "stdio-zip-iso21320", "stdio iso zip", "ZIP (ISO/IEC 21320-1)", "Edit document container archives", "Edit ZIP archives restricted to the ISO/IEC 21320-1 document container profile.", "file-archive"),
    "EpwEditor": ("s.stdio.epw@energyplus/*#editor", "stdio-epw", "stdio weather", "EPW Weather", "Edit weather files", "Open EnergyPlus EPW weather files and edit their header and hourly records.", "sun"),
}


def die(message):
    print(f"st1-gen: {message}", file=sys.stderr)
    sys.exit(1)


def read(repo, rel):
    with open(os.path.join(repo, rel), encoding="utf-8") as handle:
        return handle.read()


def full_catalog_blocks(source):
    """📜️ The full-app-catalog enum body and register_apps body, exactly as the live file spells them."""
    enum_start = source.find('#[cfg(feature = "full-app-catalog")]\ndyn_enum_close! {\n    pub enum StdioApps: PluginApp {\n')
    if enum_start < 0:
        die("full-app-catalog StdioApps enum anchor not found")
    body_start = source.index("{\n", source.index("pub enum StdioApps", enum_start)) + 2
    body_end = source.index("\n    }\n}\n", body_start)
    enum_body = source[body_start:body_end]
    reg_start = source.find('#[cfg(feature = "full-app-catalog")]\nfn register_apps(')
    if reg_start < 0:
        die("full-app-catalog register_apps anchor not found")
    reg_body_start = source.index("{\n", reg_start) + 2
    reg_body_end = source.index("\n    builder\n}\n", reg_body_start)
    return enum_body, source[reg_body_start:reg_body_end]


def parse_enum(enum_body):
    """🗃️ Variant name → (role, type path)."""
    rows = {}
    for match in re.finditer(r"^\s*(\w+)\(VcsArtifactApp<(EditorApp|ViewerApp)<([\w:]+)>>\),\s*$", enum_body, re.M):
        rows[match.group(3)] = (match.group(1), "editor" if match.group(2) == "EditorApp" else "viewer")
    return rows


def parse_registrations(register_body):
    """🧾️ Ordered builder statements: (kind, type path, create path) with kind in editor/viewer/editor_roster/viewer_roster."""
    text = re.sub(r"//[^\n]*", "", register_body)
    statements = [" ".join(chunk.split()) for chunk in text.split(";") if chunk.strip()]
    rows = []
    for statement in statements:
        match = re.fullmatch(r"builder = builder\.(editor|viewer)::<([\w:]+)>\( ?([\w:]+)\(\),? ?\)", statement)
        if match:
            rows.append((match.group(1), match.group(2), match.group(3)))
            continue
        match = re.fullmatch(r"builder = builder\.(editor|viewer)_mutation_roster::<([\w:]+)>\(\)", statement)
        if match:
            rows.append((match.group(1) + "_roster", match.group(2), None))
            continue
        die(f"unparsed registration statement: {statement}")
    return rows


def crate_of(type_path):
    match = re.match(r"semio_s_artifact_stdio_(\w+?)::", type_path)
    if not match:
        die(f"type path outside a stdio artifact crate: {type_path}")
    return match.group(1)


IMPL_INDEX = {}


def find_impl_file(repo, fmt, type_name):
    """🔎️ The one file holding `impl ArtifactEditor for <type_name>` inside the format's artifact folder (indexed once per folder)."""
    artifacts = os.path.join(repo, STDIO, "🗿️artifacts")
    folder = next((name for name in os.listdir(artifacts) if re.sub(r"^[^a-z0-9]+", "", name) == fmt), None)
    if folder is None:
        die(f"no artifact folder for {fmt}")
    if folder not in IMPL_INDEX:
        index = {}
        for dirpath, dirnames, filenames in os.walk(os.path.join(artifacts, folder)):
            dirnames[:] = [name for name in dirnames if name not in ("🧪️tests", "🧫️fixtures", "📦️packages", "node_modules", "dist", "📚️examples", "🔮️oracles", "🖼️assets")]
            if "🦀️.rs" in filenames:
                path = os.path.join(dirpath, "🦀️.rs")
                with open(path, encoding="utf-8") as handle:
                    for name in re.findall(r"impl ArtifactEditor for (\w+)\b", handle.read()):
                        index.setdefault(name, []).append(path)
        IMPL_INDEX[folder] = index
    hits = IMPL_INDEX[folder].get(type_name, [])
    if len(hits) != 1:
        die(f"{type_name}: expected one ArtifactEditor impl in {folder}, found {len(hits)}")
    return hits[0]


def editor_example(repo, fmt, type_name):
    """📚️ The first example the editor's `examples()` names, resolved through its artifact crate's `examples` module
    (`pub mod <name> { #[path = "<artifact-relative leaf>"] mod component; }`): (id, label.native.en) or None."""
    path = find_impl_file(repo, fmt, type_name)
    with open(path, encoding="utf-8") as handle:
        source = handle.read()
    impl = source[source.index(f"impl ArtifactEditor for {type_name}"):]
    match = re.search(r"fn examples\(\)[^{]*\{(.*?)\n    \}", impl, re.S)
    if not match:
        return None, path
    modules = re.findall(r"examples::(\w+)::source\(\)", match.group(1))
    if not modules:
        return None, path
    relative = os.path.relpath(path, os.path.join(repo, STDIO, "🗿️artifacts"))
    artifact_dir = os.path.join(repo, STDIO, "🗿️artifacts", relative.split(os.sep)[0])
    with open(os.path.join(artifact_dir, "🦀️.rs"), encoding="utf-8") as handle:
        root = handle.read()
    block = root[root.index("pub mod examples {"):]
    leaf = re.search(r"pub mod " + re.escape(modules[0]) + r" \{\s*#\[path = \"([^\"]+)\"\]\s*mod component;", block) or re.search(r"#\[path = \"([^\"]+)\"\]\s*pub mod " + re.escape(modules[0]) + r";", block)
    if not leaf:
        die(f"{type_name}: example module {modules[0]} not wired in {artifact_dir}/🦀️.rs")
    with open(os.path.join(artifact_dir, leaf.group(1)), encoding="utf-8") as handle:
        example = handle.read()
    example_id = re.search(r'pub const ID: &str = "([^"]+)";', example)
    label = re.search(r'LocalizedLabel::native\("([^"]+)",', example)
    if not example_id or not label:
        die(f"{type_name}: example {leaf.group(1)} has no ID/label")
    return (example_id.group(1), label.group(1)), path


def rust_family_source(family, subsets):
    fid, emoji, slug, label, _group, summary, formats, enum = family
    variants = []
    builder = []
    for row in subsets:
        variants.append(f"        {row['editorVariant']}(VcsArtifactApp<EditorApp<{row['editorType']}>>),")
        variants.append(f"        {row['viewerVariant']}(VcsArtifactApp<ViewerApp<{row['viewerType']}>>),")
        builder.append(f"        .editor::<{row['editorType']}>({row['editorCreate']}())")
        if row["editorRoster"]:
            builder.append(f"        .editor_mutation_roster::<{row['editorType']}>()")
        builder.append(f"        .viewer::<{row['viewerType']}>({row['viewerCreate']}())")
        if row["viewerRoster"]:
            builder.append(f"        .viewer_mutation_roster::<{row['viewerType']}>()")
    activations = [f"        .activation(ActivationEvent::OnArtifactKind {{ kind: semio_s_artifact_stdio_{fmt}::artifact_kind().id }})" for fmt in formats]
    kinds = "/".join(formats)
    return f"""//! {emoji} `{fid}` — {summary} as their own wasm component, over the artifact kinds and codecs the
//! `stdio` package owns.
//!
//! Every registered app monomorphises the whole app runtime and is live code inside its component, so one stdio
//! component cannot assemble all 176 stdio apps; each family ships its bounded fleet as its own package and depends on
//! `stdio` for the kinds it opens (`🗄️stdio/🧪️tests/🚢️shipped-fleet`: every stdio app is shipped by exactly one
//! package, every stdio kind is opened by exactly one package).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_value_derive as value_derive;

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{{ActivationEvent, CapabilityId, CapabilityRequest}};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{{ExecutionMode, Plugin, PluginApp}};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {{
    /// 🗃️ Closed runtime app fleet of the {kinds} editors and viewers — one editor and one viewer per subset.
    pub enum {enum}: PluginApp {{
{chr(10).join(variants)}
    }}
}}
//#endregion 🗃️Apps

/// 🔌️ Builds the `{fid}` bundle: every {kinds} subset's editor and viewer (with the owner-mutation roster where
/// the subset's mutation enum derives one), one activation per artifact kind it opens read live from that kind's own
/// `artifact_kind().id`, and the exact-pin runtime dependency on `stdio`, which owns those kinds and their codecs.
pub fn plugin() -> Result<Plugin<{enum}>, PluginAssemblyError> {{
    Plugin::<{enum}>::builder("{fid}")
        .label("{label}")
        .version(env!("CARGO_PKG_VERSION"))
        .package_id("semio:{fid}")
        .depends_on("stdio", semio_framework::tree_pin!())
{chr(10).join(builder)}
{chr(10).join(activations)}
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {{
            id: CapabilityId("artifacts.write".into()),
            scope: "plugin".into(),
            reason: "persist {kinds} editor edits back to the open stdio document".into(),
            optional: false,
        }})
        .try_build()
}}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, {enum});
"""


def cargo_family_source(family, subsets, ports):
    fid, emoji, slug, label, _group, summary, formats, _enum = family
    rows = []
    for row, (react, wgpu) in zip(subsets, ports):
        rows.append(f"""[[package.metadata.semio.playground]]
variant = "{row['variant']}"
app = "{row['appId']}"
aliases = ["{row['alias']}"]
ports = {{ react = {react}, wgpu = {wgpu} }}
""")
    deps = "\n".join(f'semio-s-artifact-stdio-{fmt} = {{ workspace = true, features = ["component-app-assembly"] }}' for fmt in formats)
    return f"""[package]
name = "semio-s-plugin-{fid}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Stdio family component: {summary}"

[lints]
workspace = true

[package.metadata.component]
package = "semio:{fid}"

# 🔗️ `stdio` owns every kind this family opens (definitions, codecs, hub-native codec closure); the family ships only the
# apps, so the host loads `stdio` beside it (`.depends_on("stdio", tree_pin!())` mirrors this row).
[package.metadata.semio]
role = "plugin"
depends-on = ["stdio"]

# 🎮️ ONE row per shipped editor, each pinning its own `app`: `semio-tech play`'s coverage laws reach every editor a
# descriptor declares and every viewer through its pane's editor⇄viewer switch exactly when every shipped app owns a
# row, and `🗄️stdio/🧪️tests/🚢️shipped-fleet` asserts these rows ARE the shipped editors. Ports continue stdio's block.
{chr(10).join(rows)}
[lib]
crate-type = ["cdylib", "rlib"]
path = "../../🦀️.rs"

[features]
# 🔌️ The WASM component entry point (`plugin_exports!`). Default-on so this family builds as its own component, and
# switched off by any crate that links it as a library (the stdio census law links every family at once).
default = ["plugin-root"]
plugin-root = ["semio-framework-plugin/component-guest"]

[dependencies]
{deps}
semio-framework = {{ workspace = true }}
semio-framework-dispatch-macros = {{ workspace = true }}
semio-framework-plugin = {{ workspace = true }}
semio-framework-value-derive = {{ workspace = true }}
"""


def project_json(family):
    fid, emoji, slug = family[0], family[1], family[2]
    cwd = f"{EXT}/{emoji}{slug}/📦️packages/🦀️rust"
    targets = {}
    for name, command in [("test", "bun ./📜️script.ts test"), ("test-quick", "bun ./📜️script.ts test quick"), ("test-long", "bun ./📜️script.ts test long"), ("test-exhaustive", "bun ./📜️script.ts test exhaustive")]:
        targets[name] = {"executor": "nx:run-commands", "options": {"cwd": cwd, "command": command, "forwardAllArgs": True}}
    document = {
        "name": f"@semio-tech/{fid}-plugin",
        "$schema": "../../../../../../../node_modules/nx/schemas/project-schema.json",
        "namedInputs": {"default": [f"{{workspaceRoot}}/{EXT}/{emoji}{slug}/**/*.rs", "{projectRoot}/**/*"]},
        "targets": targets,
    }
    return json.dumps(document, ensure_ascii=False, indent=2) + "\n"


def script_ts(family):
    fid, emoji = family[0], family[1]
    return f"""#!/usr/bin/env bun
/** {emoji} `@semio-tech/{fid}-plugin` router: `bun ./📜️script.ts test [quick|long|exhaustive]`. */
import {{ BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted }} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {{
  async run(segments: string[]): Promise<void> {{
    const {{ rest }} = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-s-plugin-{fid}"], this.repoRoot, rest);
  }}
}}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, {{ defaultCommand: "test" }});
"""


def used_playground_ports(repo):
    """🔌️ Every react/wgpu/user port any tracked Cargo playground row already claims."""
    listing = subprocess.run(["git", "ls-files", "-z", "*Cargo.toml", ":!:.🧬semio/**"], cwd=repo, capture_output=True, check=True).stdout.decode().split("\0")
    used = set()
    for rel in filter(None, listing):
        with open(os.path.join(repo, rel), encoding="utf-8") as handle:
            for block in re.findall(r"^(?:user_)?ports = \{[^}]*\}", handle.read(), re.M):
                used.update(int(port) for port in re.findall(r"\d{4}", block))
    return used


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", default="/Users/ueli/Documents/semio")
    args = parser.parse_args()
    repo = args.repo
    source = read(repo, f"{STDIO}/🔌️plugin/🦀️.rs")
    enum_body, register_body = full_catalog_blocks(source)
    variants = parse_enum(enum_body)
    registrations = parse_registrations(register_body)
    fixture = json.loads(read(repo, f"{STDIO}/🧫️fixtures/✏️editor-catalog/🔣️.json"))

    editors = [row for row in registrations if row[0] == "editor"]
    viewers = [row for row in registrations if row[0] == "viewer"]
    rosters = {(row[0], row[1]) for row in registrations if row[0].endswith("_roster")}
    if len(editors) != 88 or len(viewers) != 88 or len(variants) != 176:
        die(f"library fleet is {len(editors)} editors / {len(viewers)} viewers / {len(variants)} enum variants, expected 88/88/176")
    viewer_by_prefix = {}
    for kind, type_path, create in viewers:
        viewer_by_prefix.setdefault((crate_of(type_path), type_path.rsplit("::", 1)[0].replace("::viewer::", "::")), []).append((type_path, create))

    known_formats = set(BASE_FORMATS) | {fmt for family in FAMILIES for fmt in family[6]}
    rows_by_format = {}
    for kind, editor_type, editor_create in editors:
        fmt = crate_of(editor_type)
        if fmt not in known_formats:
            die(f"format {fmt} belongs to no package")
        editor_name = editor_type.rsplit("::", 1)[1]
        module = editor_type.rsplit("::", 1)[0]
        viewer_module = module.replace("::editor::", "::viewer::", 1)
        viewer_candidates = [(type_path, create) for _kind, type_path, create in viewers if type_path.rsplit("::", 1)[0] == viewer_module]
        if len(viewer_candidates) != 1:
            die(f"{editor_type}: expected one viewer in {viewer_module}, found {viewer_candidates}")
        viewer_type, viewer_create = viewer_candidates[0]
        row = {
            "format": fmt,
            "editorType": editor_type,
            "editorCreate": editor_create,
            "editorVariant": variants[editor_type][0],
            "editorRoster": ("editor_roster", editor_type) in rosters,
            "viewerType": viewer_type,
            "viewerCreate": viewer_create,
            "viewerVariant": variants[viewer_type][0],
            "viewerRoster": ("viewer_roster", viewer_type) in rosters,
        }
        if fmt not in BASE_FORMATS:
            if editor_name not in SUBSETS:
                die(f"{editor_name} has no subset row")
            app_id, variant, alias, pane_label, tagline, description, icon = SUBSETS[editor_name]
            example, impl_path = editor_example(repo, fmt, editor_name)
            row.update({"appId": app_id, "variant": variant, "alias": alias, "pane": {"variant": variant, "label": pane_label, "tagline": tagline, "description": description, "icon": icon, **({"example": example[0], "exampleLabel": example[1]} if example else {})}, "editorSource": os.path.relpath(impl_path, repo)})
        rows_by_format.setdefault(fmt, []).append(row)

    table_apps = {value[0] for value in SUBSETS.values()}
    fixture_apps = set(fixture["editorApps"])
    base_apps = {app for app in fixture_apps if re.match(r"s\.stdio\.(" + "|".join(BASE_FORMATS) + r")@", app)}
    if table_apps != fixture_apps - base_apps or len(table_apps) != len(SUBSETS):
        die(f"hand table ≠ fixture: missing {sorted(fixture_apps - base_apps - table_apps)} extra {sorted(table_apps - fixture_apps)}")
    variants_seen = [value[1] for value in SUBSETS.values()]
    aliases_seen = [value[2] for value in SUBSETS.values()]
    if len(set(variants_seen)) != len(variants_seen) or len(set(aliases_seen)) != len(aliases_seen):
        die("variant or alias repeated")

    used = used_playground_ports(repo) | PORT_SKIP
    ports = [(react, react + 100) for react in range(PORT_REACT0, 6400) if react not in used and react + 100 not in used]
    payload = os.path.join(HERE, "payload")
    if os.path.isdir(payload):
        shutil.rmtree(payload)
    plan = {"ceiling": CEILING, "families": [], "baseFormats": BASE_FORMATS}
    port_cursor = 0
    for family in FAMILIES:
        fid, emoji, slug, label, group, summary, formats, enum = family
        subsets = [row for fmt in formats for row in rows_by_format[fmt]]
        family_ports = ports[port_cursor:port_cursor + len(subsets)]
        port_cursor += len(subsets)
        if len(family_ports) != len(subsets):
            die("ran out of free playground ports")
        if 2 * len(subsets) > CEILING:
            die(f"{fid} ships {2 * len(subsets)} apps over the ceiling {CEILING}")
        owner = f"{EXT}/{emoji}{slug}"
        files = {
            f"{owner}/🦀️.rs": rust_family_source(family, subsets),
            f"{owner}/📦️packages/🦀️rust/Cargo.toml": cargo_family_source(family, subsets, family_ports),
            f"{owner}/📦️packages/🦀️rust/📋️project.json": project_json(family),
            f"{owner}/📦️packages/🦀️rust/📜️script.ts": script_ts(family),
        }
        for rel, content in files.items():
            path = os.path.join(payload, rel)
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(content)
        plan["families"].append({
            "id": fid, "emoji": emoji, "slug": slug, "dir": f"{emoji}{slug}", "owner": owner, "label": label, "enum": enum,
            "crate": f"semio-s-plugin-{fid}", "lib": f"semio_s_plugin_{fid.replace('-', '_')}", "directoryName": f"{emoji}{fid}",
            "group": {"id": group[0], "label": group[1]}, "formats": formats, "apps": 2 * len(subsets),
            "subsets": [{k: v for k, v in row.items() if k in ("appId", "variant", "alias", "pane", "editorType", "viewerType", "format")} for row in subsets],
            "ports": family_ports,
        })
    with open(os.path.join(HERE, "plan.json"), "w", encoding="utf-8") as handle:
        json.dump(plan, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    total = sum(family["apps"] for family in plan["families"])
    print(f"st1-gen: {len(plan['families'])} families, {total} family apps (+18 base = {total + 18}), ports {ports[0]}..{ports[port_cursor - 1]}, {sum(1 for f in plan['families'] for s in f['subsets'] if 'example' in s['pane'])} panes with a curated example")
    for family in plan["families"]:
        print(f"  {family['directoryName']:28} {family['apps']:3} apps  {','.join(family['formats'])}")


if __name__ == "__main__":
    main()
