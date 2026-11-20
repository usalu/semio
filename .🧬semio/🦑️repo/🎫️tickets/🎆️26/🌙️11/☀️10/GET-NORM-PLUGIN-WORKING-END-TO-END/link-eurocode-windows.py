"""🧪️ Links actual authored windows from one cached WASI Cargo cohort."""

import argparse
import subprocess
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("family", choices=[*(f"en199{i}" for i in range(10)), "din4108", "din16798", "din18599", "iso16757", "vdi3805"])
    parser.add_argument("--lane", required=True)
    parser.add_argument("--rustc", type=Path, required=True)
    parser.add_argument("--visual-chrome", type=Path)
    parser.add_argument("--check-pages", action="store_true")
    parser.add_argument("--gallery-only", action="store_true")
    parser.add_argument("--viewport-pages", action="store_true")
    parser.add_argument("--supplemental-dir", type=Path, action="append", default=[])
    arguments = parser.parse_args()
    ticket = Path(__file__).resolve().parent
    workspace = ticket.parents[6]
    lane = workspace / ".cache/b/r/m/u/l" / arguments.lane
    build = lane / "wasm32-wasip2/prototype/build"
    fixture = "gallery" if arguments.gallery_only else "windows"
    source = "audit-eurocode-gallery.rs" if arguments.gallery_only else "audit-eurocode-windows.rs"
    if arguments.viewport_pages:
        fixture, source = "viewport-pages", "audit-eurocode-viewport-pages.rs"
    output = ticket / "🗑️generated" / f"{fixture}-{arguments.family}.wasm"
    flags = [
        "--edition=2021", "--crate-name=authored_windows", "--target=wasm32-wasip2",
        "-Cpanic=abort", "-Copt-level=1", "-Cstrip=debuginfo",
        f"-o{output}", str(ticket / source),
    ]
    directories = set(build.glob("*/*/out")) | set((lane / "prototype/build").glob("*/*/out"))
    directories.update(directory.resolve() for directory in arguments.supplemental_dir)
    flags.extend(f"-Ldependency={directory}" for directory in sorted(directories))
    externs = {
        "serde_json": "serde_json",
        "pack": "semio-framework-pack",
        "semio_framework_value": "semio-framework-value",
        "semio_plugin": "semio-plugin",
        "semio_ui_contract": "semio-ui-contract",
        "semio_ui_contract_values": "semio-ui-contract-values",
        "semio_ui_locale": "semio-ui-locale",
        "semio_s_plugin_norm_app_surface": "semio-s-plugin-norm-app-surface",
        "semio_s_plugin_norm_compliance": "semio-s-plugin-norm-compliance",
        "audited_editor": f"semio-s-artifact-norm-{arguments.family}-editor",
    }
    if arguments.gallery_only:
        for alias in ("semio_s_plugin_norm_app_surface", "audited_editor"):
            del externs[alias]
        externs["audited_model"] = f"semio-s-artifact-norm-{arguments.family}-model"
        externs["audited_report"] = f"semio-s-artifact-norm-{arguments.family}-inferences"
        if arguments.family == "en1994":
            externs["semio_s_plugin_norm_app_surface"] = "semio-s-plugin-norm-app-surface"
            externs["semio_s_plugin_norm_structural_scene"] = "semio-s-plugin-norm-structural-scene"
            externs["semio_ui_scene"] = "semio-ui-scene"
            externs["semio_framework_deflate"] = "semio-framework-deflate"
        flags.extend(("--cfg", f'feature="{arguments.family}"'))
    if arguments.viewport_pages:
        del externs["audited_editor"]
        externs["semio_framework_pack_json"] = "semio-framework-pack-json"
        externs["semio_ui_runtime"] = "semio-ui-runtime"
        for alias, suffix in (("audited_model", "model"), ("audited_scene", "scene"), ("audited_report", "inferences")):
            externs[alias] = f"semio-s-artifact-norm-{arguments.family}-{suffix}"
        flags.extend(("--cfg", f'feature="{arguments.family}"'))
    if arguments.visual_chrome or arguments.check_pages or arguments.gallery_only:
        externs["audited_scene"] = f"semio-s-artifact-norm-{arguments.family}-scene"
        if arguments.family == "en1996":
            externs["audited_scene"] += "-diagrams"
        flags.extend(("--cfg", 'feature="updated-diagrams"'))
    if arguments.visual_chrome:
        for file in [arguments.visual_chrome, arguments.visual_chrome.with_suffix(".rmeta")]:
            flags.extend(("--extern", f"audited_diagrams={file.resolve()}"))
    elif arguments.check_pages or arguments.gallery_only:
        externs["audited_diagrams"] = "semio-s-plugin-norm-app-surface-visual-chrome"
    for alias, package in externs.items():
        filename = f"lib{package.replace('-', '_')}.rlib"
        libraries = [
            directory.resolve() / filename for directory in arguments.supplemental_dir
            if (directory / filename).is_file()
        ] or list((build / package).glob("*/out/*.rlib"))
        if not libraries:
            raise FileNotFoundError(f"No cached RLIB for {package}")
        rlib = max(libraries, key=lambda file: file.stat().st_mtime)
        for path in [rlib, rlib.with_suffix(".rmeta")]:
            if not path.is_file():
                raise FileNotFoundError(path)
            flags.extend(("--extern", f"{alias}={path}"))
    response = output.with_suffix(".args")
    response.write_text("\n".join(flags) + "\n")
    subprocess.run([str(arguments.rustc.resolve()), f"@{response}"], cwd=workspace, check=True)
    print(output)


if __name__ == "__main__":
    main()
