"""🔬 Checks the exact reviewed PDF PNG progress methods against an admitted WASI graph."""

import argparse
import json
from pathlib import Path
import runpy
import subprocess

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated" / "image-progress-release"
SCHEMA = "semio_s_artifact_stdio_pdf_schema_5"
PAGE = "semio_s_artifact_stdio_pdf_page_editor"
DOCUMENT = "semio_s_artifact_stdio_pdf_editor_document"


def artifact_stem(path):
    return path.stem.removeprefix("lib")


def sidecar(path):
    return json.loads(Path(str(path) + ".semio-deps").read_text(encoding="utf-8"))


def check(root, cargo_stream):
    stage = runpy.run_path(str(TICKET / "image-progress-release.py"))
    _, _, receipt, additions = stage["candidate"](root)
    scope = additions[0].removesuffix("    fn render_with_request_context(\n")
    if scope == additions[0] or len(additions) != 2:
        raise ValueError("The exact reviewed method bodies are missing")
    rows = [
        row for line in cargo_stream.read_text(encoding="utf-8").splitlines()
        if line.startswith("{") and (row := json.loads(line)).get("reason") == "compiler-artifact"
    ]
    wanted = {SCHEMA, PAGE, DOCUMENT}
    input_artifacts = {}
    for row in rows:
        name = row["target"]["name"]
        if name not in wanted:
            continue
        for label in row["filenames"]:
            path = Path(label)
            if path.suffix == ".rmeta" and "wasm32-wasip2" in path.parts:
                input_artifacts[name] = path
    if input_artifacts.keys() != wanted:
        raise ValueError(f"The selected Cargo run did not emit all three WASI inputs: {input_artifacts.keys()}")
    schema = input_artifacts[SCHEMA]
    deps = dict(sidecar(schema)["dependencies"])
    for name in (PAGE, DOCUMENT):
        dep = input_artifacts[name]
        if deps.get(artifact_stem(dep)) != sidecar(dep)["svh"]:
            raise ValueError(f"{name} does not match the schema-5 artifact receipt")

    exact = {}
    for row in rows:
        for label in row["filenames"]:
            path = Path(label)
            stem = artifact_stem(path)
            if stem in deps and path.is_file() and path.suffix in (".rmeta", ".dylib", ".so", ".dll", ".rlib"):
                if stem not in exact or path.suffix == ".rmeta":
                    exact[stem] = path
    if not exact:
        raise ValueError("The current Cargo stream has no materialized dependency library")

    externs = {"captured_schema": schema}
    for alias in ("semio_plugin", "semio_s_artifact_stdio_contract"):
        names = [path for stem, path in exact.items() if stem.rsplit("-", 1)[0] == alias]
        if len(names) != 1 or (names[0].suffix == ".rmeta" and sidecar(names[0])["svh"] != deps[artifact_stem(names[0])]):
            raise ValueError(f"Cannot identify the exact built library for {alias}")
        externs[alias] = names[0]

    wrapper = """#![allow(dead_code)]
pub use captured_schema::editor;
use captured_schema::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use semio_plugin::{ArtifactEditor, ArtifactView, ComponentTree, ViewModel, built_to_component_tree};
""" + scope + """
fn check_trait_scope<Editor: ArtifactEditor>() -> semio_plugin::kernel::UiDirtyScope {
    let _: fn() -> semio_plugin::kernel::UiDirtyScope = Editor::operation_progress_scope;
    operation_progress_scope()
}

pub fn render(
    doc: &ArtifactView<'_, PdfSnapshot>,
    selected: Option<&str>,
    view_state: &ViewModel,
) -> semio_plugin::UiAssemblyResult<ComponentTree> {
""" + additions[1] + "}\n"
    OUTPUT.mkdir(parents=True, exist_ok=True)
    arguments = [
        "--crate-name", "pdf_png_progress_check", "--crate-type=lib", "--edition=2021",
        "--target", "wasm32-wasip2", "--emit=metadata", "-W", "future-incompatible", "-o", str(OUTPUT / "check.rmeta"),
    ]
    for folder in sorted({path.parent for path in exact.values()}):
        arguments.extend(["-L", f"dependency={folder}"])
    for alias, path in externs.items():
        arguments.extend(["--extern", f"{alias}={path}"])
    arguments.append("-")
    response = OUTPUT / "rustc-args.txt"
    response.write_text("\n".join(arguments) + "\n", encoding="utf-8")
    subprocess.run([
        "repo", "tools", "exec", "--light", "rustup", "--",
        "run", "nightly-2026-07-20", "rustc", "@" + str(response),
    ], input=wrapper, text=True, check=True)
    print(f"[DEBUG] Exact PDF progress controller {receipt['postimage']} type-checked against {cargo_stream.parent.parent.name}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path)
    parser.add_argument("cargo_stream", type=Path)
    args = parser.parse_args()
    check(args.root, args.cargo_stream)
