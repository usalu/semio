from pathlib import Path
root = Path("/Users/ueli/Documents/semio")
fw = next(p for p in root.iterdir() if p.is_dir() and "framework" in p.name)
modules = next(p for p in (fw / "🛍️products").iterdir() if "os" in p.name) 
modules = next(p for p in modules.iterdir() if p.name.endswith("modules") or "modules" in p.name)
# actually list
products = fw / "🛍️products"
os_dir = next(p for p in products.iterdir() if p.is_dir() and "os" in p.name)
modules = next(p for p in os_dir.iterdir() if p.is_dir() and "modules" in p.name)
plugin_dir = next(p for p in modules.iterdir() if p.is_dir() and p.name.endswith("plugin"))
plugin = next(plugin_dir.glob("*component.rs"))
print("plugin", plugin)
text = plugin.read_text(encoding="utf-8")
for i, line in enumerate(text.splitlines()[:220], 1):
    if any(k in line for k in ("mod ", "builder", "#region", "#endregion", "pub mod", "#[path")):
        print("%d:%s" % (i, line))
print("--- ArtifactIo ---")
for i, line in enumerate(text.splitlines()[300:340], 301):
    print("%d:%s" % (i, line))
print("--- PluginManifest ---")
for i, line in enumerate(text.splitlines(), 1):
    if "PluginManifest" in line:
        print("%d:%s" % (i, line[:180]))
print("--- reexports ---")
for i, line in enumerate(text.splitlines()[9105:9130], 9106):
    print("%d:%s" % (i, line))
builder_dir = next(p for p in plugin_dir.iterdir() if "builder" in p.name)
builder = next(builder_dir.glob("*component.rs"))
print("builder", builder)
dsl = next(p for p in modules.iterdir() if "dsl" in p.name)
diag = next(p for p in dsl.iterdir() if "diagnostic" in p.name)
print("diag", list(diag.glob("*.rs")))
fw_modules = next(p for p in fw.iterdir() if p.is_dir() and "modules" in p.name and "products" not in p.name)
mesh = next(p for p in fw_modules.iterdir() if "mesh" in p.name)
man = next(p for p in fw_modules.iterdir() if "manifest" in p.name)
print("mesh", next(mesh.glob("*component.rs")))
print("manifest", next(man.glob("*component.rs")))
glue = next((plugin_dir / "📦️packages" / "🦀️rust").glob("*glue.rs"))
for p in (glue, plugin):
    t = p.read_text(encoding="utf-8")
    for i, line in enumerate(t.splitlines(), 1):
        if "mod builder" in line or ("builder" in line and ("mod " in line or "#[path" in line)):
            print("%s:%d:%s" % (p.name, i, line))
kg = next((os_dir / "📦️packages" / "🦀️rust").glob("*glue.rs"))
print("kernel_glue", kg)
for i, line in enumerate(kg.read_text(encoding="utf-8").splitlines(), 1):
    if i < 220 and any(k in line for k in ("TextError", "PackError", "Diagnostic")):
        print("kg%d:%s" % (i, line[:160]))
diag_rs = next(diag.glob("*component.rs"))
print("diag_rs", diag_rs)
for i, line in enumerate(diag_rs.read_text(encoding="utf-8").splitlines()[:200], 1):
    if any(k in line for k in ("struct Diagnostic", "enum Severity", "pub struct", "pub enum", "#region")):
        print("D%d:%s" % (i, line))
