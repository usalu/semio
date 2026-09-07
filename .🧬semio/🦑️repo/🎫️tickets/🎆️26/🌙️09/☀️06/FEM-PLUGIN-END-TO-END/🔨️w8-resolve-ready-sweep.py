#!/usr/bin/env python3
"""🧹 W8 — strip stale `semio_framework_plugin::resolve_ready(...)` wrappers around
now-sync callees in the fem plugin (de-async fallout).

Classification is explicit: every wrapped inner expression must match a SYNC or an
ASYNC prefix below, otherwise the script aborts so no site is swept blindly.
Files under a `🚪️io/` directory are never touched (owned by W4).
"""

import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
FEM = ROOT / "✏️s/🔌️plugins/🏗️fem"
WRAPPER = "semio_framework_plugin::resolve_ready("

SYNC = [
    "vcs::apply_mutation(",
    "<Fem3dViewer as ArtifactViewer>::initial_snapshot(",
    "<Fem2dViewer as ArtifactViewer>::initial_snapshot(",
    "semio_framework_plugin::HistoryView::empty(",
    "ArtifactView::new(",
    "Fem3dPlayApp::export_media(",
    "Fem3dPlayApp::import_media(",
    "Fem3dPlayApp::io(",
    "Fem2dPlayApp::export_media(",
    "Fem2dPlayApp::import_media(",
    "Fem2dPlayApp::io(",
    "semio_framework_plugin::world3d_meshes_json_from_kinds(",
]

SYNC_SUFFIX_CALL = ".snapshot("  # ArtifactStore::snapshot / VcsArtifactApp::snapshot

ASYNC = [
    "SemioMeshToStl::serialize(",
    "SemioMeshToObj::serialize(",
    "store::print_document_spr(",
    "new_app::<",
    "new_app_with_registry::<",
    "semio_framework_plugin::testkit::paired_apps::<",
    "crate::artifacts::fem3d::schema::mutations::Fem3dStore::new(",
    "crate::artifacts::fem2d::schema::mutations::Fem2dStore::new(",
    "crate::artifacts::fem2d::mutations::Fem2dStore::new(",
]

ASYNC_SUFFIX_CALL = (".render(", ".handle_action(")


def matching_paren(text: str, open_index: int) -> int:
    depth = 0
    i = open_index
    in_string = False
    while i < len(text):
        c = text[i]
        if in_string:
            if c == "\\":
                i += 2
                continue
            if c == '"':
                in_string = False
        else:
            if c == '"':
                in_string = True
            elif c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
                if depth == 0:
                    return i
        i += 1
    raise SystemExit(f"unbalanced parens starting at offset {open_index}")


def classify(inner: str) -> str:
    stripped = inner.lstrip()
    for prefix in ASYNC:
        if stripped.startswith(prefix):
            return "async"
    for prefix in SYNC:
        if stripped.startswith(prefix):
            return "sync"
    head = stripped.split(")", 1)[0]
    if SYNC_SUFFIX_CALL in head + ")":
        if stripped.split("(", 1)[0].endswith(".snapshot"):
            return "sync"
    for suffix in ASYNC_SUFFIX_CALL:
        if stripped.split("(", 1)[0].endswith(suffix[:-1]):
            return "async"
    return "unknown"


def main() -> None:
    apply = "--apply" in sys.argv
    report = []
    unknown = []
    for path in sorted(FEM.rglob("🦀️.rs")):
        if any(part.startswith("🚪️io") for part in path.parts):
            in_io = True
        else:
            in_io = False
        text = path.read_text(encoding="utf-8")
        if WRAPPER not in text:
            continue
        out = []
        cursor = 0
        while True:
            idx = text.find(WRAPPER, cursor)
            if idx < 0:
                out.append(text[cursor:])
                break
            open_index = idx + len(WRAPPER) - 1
            close_index = matching_paren(text, open_index)
            inner = text[open_index + 1 : close_index]
            verdict = classify(inner)
            line = text.count("\n", 0, idx) + 1
            rel = str(path.relative_to(ROOT))
            report.append((rel, line, verdict, "io" if in_io else "-", inner.strip()[:110]))
            if verdict == "unknown":
                unknown.append((rel, line, inner.strip()[:160]))
            if verdict == "sync" and not in_io:
                out.append(text[cursor:idx])
                out.append(inner)
                cursor = close_index + 1
            else:
                out.append(text[cursor : close_index + 1])
                cursor = close_index + 1
        new_text = "".join(out)
        if apply and new_text != text:
            path.write_text(new_text, encoding="utf-8")

    for rel, line, verdict, io, inner in report:
        print(f"{verdict:8s} {io:3s} {rel}:{line}  {inner}")
    print(f"\ntotal={len(report)} sync={sum(1 for r in report if r[2]=='sync')} "
          f"async={sum(1 for r in report if r[2]=='async')} unknown={len(unknown)} "
          f"io={sum(1 for r in report if r[3]=='io')}")
    if unknown:
        print("\nUNCLASSIFIED:")
        for rel, line, inner in unknown:
            print(f"  {rel}:{line}  {inner}")
        raise SystemExit(1)


if __name__ == "__main__":
    main()
