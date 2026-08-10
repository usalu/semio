from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
fw = next(p for p in root.iterdir() if p.is_dir() and "framework" in p.name)
plugin_dir = None
for p in (fw / "🛍️products" / "💻️os" / "🔨️modules").iterdir():
    if "plugin" in p.name and p.is_dir() and "plugin-modules" not in p.name:
        plugin_dir = p
        break
assert plugin_dir is not None
plugin = plugin_dir / "🦀️component.rs"
print("plugin", plugin.exists(), plugin)

text = plugin.read_text(encoding="utf-8")
for i, line in enumerate(text.splitlines()[:220], 1):
    keys = ("mod ", "builder", "#region", "#endregion", "pub mod", "#[path")
    if any(k in line for k in keys):
        print(f"{i}:{line}")

print("--- ArtifactIo ---")
for i, line in enumerate(text.splitlines()[300:340], 301):
    print(f"{i}:{line}")

print("--- PluginManifest lines ---")
for i, line in enumerate(text.splitlines(), 1):
    if "PluginManifest" in line:
        print(f"{i}:{line[:180]}")

print("--- reexports ---")
for i, line in enumerate(text.splitlines()[9105:9130], 9106):
    print(f"{i}:{line}")

builder = next((plugin_dir / "🏗️builder").glob("*.rs"))
print("builder", builder)

modules = fw / "🛍️products" / "💻️os" / "🔨️modules"
dsl = next(p for p in modules.iterdir() if "dsl" in p.name)
diag = next(p for p in dsl.iterdir() if "diagnostic" in p.name)
print("diag dir", diag, list(diag.glob("*.rs")))

mesh = next(p for p in (fw / "🔨️modules").iterdir() if "mesh" in p.name) / "🦀️component.rs"
print("mesh", mesh.exists(), mesh)

man = next(p for p in (fw / "🔨️modules").iterdir() if "manifest" in p.name) / "🦀️component.rs"
print("manifest", man.exists(), man)

glue = plugin_dir / "📦️packages" / "🦀️rust" / "📦️glue.rs"
for p in (glue, plugin):
    t = p.read_text(encoding="utf-8")
    for i, line in enumerate(t.splitlines(), 1):
        if "mod builder" in line or ("builder" in line and ("mod " in line or "#[path" in line)):
            print(f"{p.name}:{i}:{line}")

kernel_glue = fw / "🛍️products" / "💻️os" / "📦️packages" / "🦀️rust" / "📦️glue.rs"
print("kernel glue", kernel_glue.exists())
if kernel_glue.exists():
    for i, line in enumerate(kernel_glue.read_text(encoding="utf-8").splitlines(), 1):
        if i < 220 and any(k in line for k in ("TextError", "PackError", "Diagnostic", "pub use")):
            print(f"kg{i}:{line[:160]}")

diag_rs = next(diag.glob("*.rs"))
print("diag_rs", diag_rs)
for i, line in enumerate(diag_rs.read_text(encoding="utf-8").splitlines()[:160], 1):
    if any(k in line for k in ("struct Diagnostic", "enum Severity", "pub struct", "pub enum", "#region")):
        print(f"D{i}:{line}")
