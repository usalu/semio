"""🧭 Rebuilds only the Save overlay's edited Rust providers against one native cache."""

from argparse import ArgumentParser
from hashlib import sha256
import json
import os
from pathlib import Path
import re
import runpy
import shlex
import subprocess

TOOLS = runpy.run_path(str(Path(__file__).with_name("pdf-save-native-link.py")))
TICKET = TOOLS["TICKET"]
OUT = TICKET / "🗑️generated" / "save-native-link"
REPO = Path.cwd()
PDF = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf")
PLUGIN = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
GRAPH = (
    "semio_s_artifact_stdio_pdf_lift",
    "semio_s_artifact_stdio_pdf_lower",
    "semio_s_artifact_stdio_pdf_conformance_support",
    "semio_s_artifact_stdio_pdf_schema_4",
)


def args_file(name, args, env=None, cwd=None):
    response = OUT / f"{name}.args"
    response.write_text("\n".join(args) + "\n", encoding="utf8")
    subprocess.run(TOOLS["RUSTC"] + ["@" + str(response)], check=True, env=env, cwd=cwd)


def rust_names(manifest):
    names = {}
    active = False
    for line in manifest.read_text().splitlines():
        if line.startswith("["):
            active = line in ("[dependencies]", "[dev-dependencies]")
        if active:
            match = re.match(r"^([\w_-]+)\s*=\s*(.+)", line)
            if match:
                alias, value = match.groups()
                package = re.search(r'\bpackage\s*=\s*"([^"]+)"', value)
                names[alias.replace("-", "_")] = (package[1] if package else alias).replace("-", "_")
    return names


def copied_sources(metadata, name, edits):
    dep = metadata.with_suffix(".d").with_name(metadata.with_suffix(".d").name.removeprefix("lib"))
    lines = dep.read_text().splitlines()
    roots = shlex.split(lines[0].split(": ", 1)[1])
    basis = (REPO / (PLUGIN if name == "semio_plugin" else PDF)).resolve()
    destination = TICKET / "🗑️generated" / "sl" / ("sdk" if name == "semio_plugin" else "pdf")
    files = {}
    source_env = {}
    for item in roots:
        path = Path(item)
        original = (path if path.is_absolute() else basis / path).resolve()
        if not original.is_relative_to(basis):
            continue
        relative = original.relative_to(REPO).as_posix()
        content = original.read_bytes()
        if relative in edits:
            before, staged = edits[relative]
            assert sha256(content).hexdigest() == before, original
            content = staged.encode()
        files[original] = (content.decode(), relative in edits)
    initial = next(iter(files))
    manifest = initial.with_name("Cargo.toml")
    path_attr = re.compile(r'(#\[path\s*=\s*|(?:include|include_str|include_bytes)!\(\s*)"([^"]+)"')

    def child(path, member):
        return (path.parent / member).resolve()

    redirected = {path for path, (_, staged) in files.items() if staged}
    while True:
        new = {path for path, (text, _) in files.items() if any(
            child(path, match[2]) in redirected for match in path_attr.finditer(text))}
        if new <= redirected:
            break
        redirected.update(new)
    assert initial in redirected, name
    for original in redirected:
        target = destination / original.relative_to(basis)
        assert len(str(target).encode("utf-16-le")) // 2 <= 256, target
        target.parent.mkdir(parents=True, exist_ok=True)

        def mount(match):
            landed = child(original, match[2])
            if landed not in files:
                return match[0]
            anchored = destination / landed.relative_to(basis) if landed in redirected else landed
            return match[1] + json.dumps(anchored.as_posix(), ensure_ascii=False)

        target.write_text(path_attr.sub(mount, files[original][0]), encoding="utf8")
    entry = destination / initial.relative_to(basis)
    for line in lines:
        if line.startswith("# env-dep:") and "=" in line:
            key, value = line.removeprefix("# env-dep:").split("=", 1)
            source_env[key] = value
    source_env.setdefault("CARGO_MANIFEST_DIR", str(manifest.parent))
    return entry, manifest, source_env


def extern_args(selected):
    result = []
    for name, path in selected.items():
        if path.suffix == ".rmeta":
            result += ["--extern", f"{name}={path}"]
            if path.with_suffix(".rlib").is_file():
                result += ["--extern", f"{name}={path.with_suffix('.rlib')}"]
        else:
            result += ["--extern", f"{name}={path}"]
    return result


def native_fixture(inventory, baseline, providers, package, holds, program):
    selected = dict(providers)
    selected["semio_s_artifact_stdio_pdf_native_save"] = package
    dependencies = TOOLS["declarations"](baseline["semio_plugin"])
    for name in ("semio_os_vcs", *TOOLS["CORE"]):
        key = next(k for k in dependencies if k.rsplit("-", 1)[0] == name)
        selected[name] = inventory[key]
    test = OUT / "native-graph-fixture.rs"
    laws = TICKET / "pdf-exact-current-graph-test.rs"
    entry = "pub use semio_s_artifact_stdio_pdf_schema_4::standards;\n"
    if program:
        for name in TOOLS["WITNESS"]:
            key = next(k for k in dependencies if k.rsplit("-", 1)[0] == name)
            selected[name] = inventory[key]
        source, lower = TOOLS["prepare_sources"]()
        TOOLS["prepare_witness"](source, lower, False)
        program_law = TICKET / "pdf-save-original-program-test.rs"
        entry += ('extern crate self as semio_s_artifact_stdio_pdf_native_save;\n'
            '#[path = "save/🦀️.rs"] mod save;\npub use save::*;\n'
            '#[path = "page-witness.rs"] mod output_stage;\n'
            f"#[path = {json.dumps(program_law.as_posix(), ensure_ascii=False)}]\nmod program_law;\n")
        selected.pop("semio_s_artifact_stdio_pdf_native_save")
    test.write_text(entry + f"#[path = {json.dumps(laws.as_posix(), ensure_ascii=False)}]\nmod graph_laws;\n")
    executable = OUT / ("graph-fixture.exe" if os.name == "nt" else "graph-fixture")
    args_file("graph-fixture", ["--edition=2021", "--test", "-Zthreads=1", "-Copt-level=0",
        str(test), "-o", str(executable), *extern_args(selected), *holds])
    sample = TICKET / "🗑️generated/classic-graph/graph.pdf"
    subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(executable), "--nocapture"],
        check=True, env={**os.environ, "SEMIO_PDF_EXACT_GRAPH_FIXTURE": str(sample),
            "SEMIO_PDF_SAVE_PAGE_FIXTURE": str(sample), "SEMIO_PDF_ORIGINAL_SAVE_OUTPUT": str(OUT / "original-program.pdf")})


def compile_staged(cache, only, codegen, fixtures, existing):
    overlay = {path: (h, s) for path, h, s in json.loads(TOOLS["INPUTS"].read_bytes())}
    inventory = {}
    for path in cache.glob("*/*/out/lib*"):
        if path.suffix in (".rmeta", ".dylib", ".so", ".dll"):
            inventory[TOOLS["metadata_key"](path)] = path
    holds = [f"-Ldependency={p}" for p in sorted({path.parent for path in inventory.values()})]
    baseline = {name: next(path for key, path in inventory.items()
        if key.rsplit("-", 1)[0] == name) for name in (*GRAPH, "semio_plugin")}
    native_current = {}
    outdir = OUT / "providers"
    outdir.mkdir(parents=True, exist_ok=True)
    holds.append(f"-Ldependency={outdir}")
    for name in GRAPH:
        prior = baseline[name]
        if existing:
            native_current[name] = outdir / f"lib{name}-pdfsaveproof.rmeta"
            assert native_current[name].with_suffix(".rlib").is_file(), native_current[name]
            continue
        if only and name not in only:
            continue
        entry, manifest, env = copied_sources(prior, name, overlay)
        versions = TOOLS["declarations"](prior)
        selected = {}
        for alias, crate in rust_names(manifest).items():
            if crate in native_current:
                selected[alias] = native_current[crate]
                continue
            present = [inventory[k] for k in versions if k.rsplit("-", 1)[0] == crate and k in inventory]
            if len(present) == 1:
                selected[alias] = present[0]
        mode = "metadata,link" if codegen else "metadata"
        args = ["--edition=2021", "--crate-name", name, "--crate-type=rlib", f"--emit={mode}",
            "--out-dir", str(outdir), "-Cmetadata=pdfsaveproof", "-Cextra-filename=-pdfsaveproof",
            "-Zthreads=1", "-Aunused", "-Aunexpected_cfgs", str(entry), *extern_args(selected), *holds]
        args_file(name, args, env={**os.environ, **env})
        native_current[name] = outdir / f"lib{name}-pdfsaveproof.rmeta"
        print(f"[DEBUG] emitted staged provider {name}", flush=True)
    if not only:
        source, _ = TOOLS["prepare_sources"]()
        root = source / "📦️packages/🦀️rust/🦀️.rs"
        selected = {"semio_plugin": baseline["semio_plugin"],
            "semio_s_artifact_stdio_pdf_schema_4": native_current["semio_s_artifact_stdio_pdf_schema_4"]}
        for name in TOOLS["CORE"]:
            key = next(k for k in TOOLS["declarations"](baseline["semio_plugin"]) if k.rsplit("-", 1)[0] == name)
            selected[name] = inventory[key]
        package = outdir / "libsemio_s_artifact_stdio_pdf_native_save.rmeta"
        if not existing:
            args_file("coherent-package", ["--edition=2021", "--crate-name", "semio_s_artifact_stdio_pdf_native_save",
                "--crate-type=rlib", f"--emit={'metadata,link' if codegen else 'metadata'}",
                "--out-dir", str(outdir), "-Zthreads=1", str(root), *extern_args(selected), *holds])
            print("[DEBUG] exact native Save package compiled against staged providers")
        if fixtures:
            native_fixture(inventory, baseline, selected, package, holds, existing)


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("cache", type=Path)
    parser.add_argument("--provider", action="append")
    parser.add_argument("--link", action="store_true")
    parser.add_argument("--fixtures", action="store_true")
    parser.add_argument("--original-program", action="store_true")
    args = parser.parse_args()
    if args.fixtures and args.provider:
        parser.error("--fixtures requires all staged providers")
    compile_staged(args.cache.resolve(), args.provider, args.link or args.fixtures,
        args.fixtures or args.original_program, args.original_program)
