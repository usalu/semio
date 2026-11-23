from pathlib import Path
root = Path("/Users/ueli/Documents/semio")
framework = next(p for p in root.iterdir() if p.is_dir() and p.name.endswith("framework"))
modules = next(p for p in framework.iterdir() if p.is_dir() and p.name.endswith("modules"))
process = next(p for p in modules.iterdir() if p.is_dir() and p.name.endswith("process"))
admit = None
for path in process.rglob("*.ts"):
    if "export async function admitCargoSourceBoundariesV1" in path.read_text(errors="replace"):
        admit = path
        break
text = admit.read_text(errors="replace")
start = text.index("export async function admitCargoSourceBoundariesV1")
chunk = text[start:start+6000]
# pretty-print by inserting breaks after semicolons for reading
pretty = chunk.replace(";", ";\n")
print(pretty[:5000])
print("\n---S DIRS---")
for path in root.iterdir():
    if path.is_dir() and path.name.endswith("s"):
        print(path.name.encode("unicode_escape").decode(), [child.name.encode("unicode_escape").decode() for child in path.iterdir() if child.is_dir()][:8])
