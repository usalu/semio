"""Restores prototype browser support from the published transpile binary and the source vendors."""

import shutil
import stat
import subprocess
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
BUN = ROOT / ".cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"


def only(parent: Path, suffix: str) -> Path:
    matches = [path for path in parent.iterdir() if path.name.endswith(suffix)]
    if len(matches) != 1:
        raise SystemExit(f"{suffix} under {parent}: {[path.name for path in matches]}")
    return matches[0]


def main() -> None:
    framework = only(ROOT, "framework")
    products = only(framework, "products")
    os_dir = only(products, "os")
    modules_dir = only(os_dir, "modules")
    plugin = only(modules_dir, "plugin")
    browser = only(plugin, "browser-bundle")
    material = only(browser, "materialization")
    preview = only(material, "preview2-shim-patches")
    shard_source = only(material, "shard-worker-source")
    contract = only(material, "contract")
    imports = only(browser, "imports")
    tool_source = ROOT / ".cache/b/r/m/published/0531fbfa4a6b583eb6e0/prototype/semio-native-transpile"
    tool_destination = ROOT / ".cache/b/p/m/browser/prototype/tools/native-transpile/darwin/semio-native-transpile"
    tool_destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(tool_source, tool_destination)
    tool_destination.chmod(tool_destination.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
    modules = ROOT / ".cache/b/p/m/browser/prototype/modules"
    modules.mkdir(parents=True, exist_ok=True)
    script = ROOT / ".cache/b/tmp/claude-host-prepare-vendor.ts"
    script.parent.mkdir(parents=True, exist_ok=True)
    script.write_text(
        "\n".join([
            'import { mkdirSync, writeFileSync } from "node:fs";',
            'import { join } from "node:path";',
            f'const {{ ensurePreview2ShimVendorAt }} = await import({str(preview / "package.ts")!r}.replace("package.ts", ""));',
            f'const {{ PREVIEW2_VENDOR_RELATIVE }} = await import({str(imports / "package.ts")!r}.replace("package.ts", ""));',
            f'const {{ shardWorkerSource }} = await import({str(shard_source / "package.ts")!r}.replace("package.ts", ""));',
            f'const {{ SHARD_WORKER_FILE, MODULE_SHARD_DIRECTORY }} = await import({str(contract / "package.ts")!r}.replace("package.ts", ""));',
            f'const modules = {str(modules)!r};',
            'const vendor = join(modules, PREVIEW2_VENDOR_RELATIVE);',
            'ensurePreview2ShimVendorAt(vendor, "/Users/ueli/Documents/semio");',
            'const shard = join(modules, MODULE_SHARD_DIRECTORY ?? "shard");',
            'mkdirSync(shard, { recursive: true });',
            'writeFileSync(join(shard, SHARD_WORKER_FILE), shardWorkerSource());',
            'console.log("[DEBUG] vendored", vendor);',
            'console.log("[DEBUG] shard", shard);',
        ]),
        encoding="utf-8",
    )
    # The package files are named with the blue square emoji, not package.ts.
    preview_ts = only(preview, ".ts")
    imports_ts = only(imports, ".ts")
    shard_ts = only(shard_source, ".ts")
    contract_ts = only(contract, ".ts")
    script.write_text(
        "\n".join([
            'import { mkdirSync, writeFileSync } from "node:fs";',
            'import { join } from "node:path";',
            f'const {{ ensurePreview2ShimVendorAt }} = await import({str(preview_ts)!r});',
            f'const {{ PREVIEW2_VENDOR_RELATIVE }} = await import({str(imports_ts)!r});',
            f'const {{ shardWorkerSource }} = await import({str(shard_ts)!r});',
            f'const {{ SHARD_WORKER_FILE }} = await import({str(contract_ts)!r});',
            f'const modules = {str(modules)!r};',
            'const vendor = join(modules, PREVIEW2_VENDOR_RELATIVE);',
            'ensurePreview2ShimVendorAt(vendor, "/Users/ueli/Documents/semio");',
            'const shard = join(modules, "\\u{1f9f5}\\ufe0fshard");',
            'mkdirSync(shard, { recursive: true });',
            'writeFileSync(join(shard, SHARD_WORKER_FILE), shardWorkerSource());',
            'console.log("[DEBUG] vendored", vendor);',
            'console.log("[DEBUG] shard", join(shard, SHARD_WORKER_FILE));',
        ]),
        encoding="utf-8",
    )
    subprocess.run([str(BUN), str(script)], check=True)
    print("[DEBUG] staged", tool_destination, tool_destination.stat().st_size)


if __name__ == "__main__":
    main()
