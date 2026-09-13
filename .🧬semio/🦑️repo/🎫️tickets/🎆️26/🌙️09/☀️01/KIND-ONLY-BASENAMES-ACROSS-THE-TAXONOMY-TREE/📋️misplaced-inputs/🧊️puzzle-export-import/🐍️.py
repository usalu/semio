09/☀️02/PUZZLE-3D-END-TO-END/🗑️generated"
)

editor_rs = editor / "🦀️.rs"
unit = editor / "�END/🗑️generated"
)

editor_rs = editor / "🦀️.rs"
unit = editor / "🧪️tests" / "🔬️unit" / "🦀️.rs"
plugin = Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs")

print("editor.rs", editor_rs.exists(), editor_rs.stat().st_size if editor_rs.exists() else 0)
print("unit.rs", unit.exists(), unit.stat().st_size if unit.exists() else 0)
print("plugin.rs", plugin.exists(), plugin.stat().st_size if plugin.exists() else 0)

text = editor_rs.read_text()
needles = [
    '"exportFixture"',
    '"importFixture"',
    '"openImportFixture"',
    "Puzzle3dWindowCommandWork",
    "BoundedFirstStepCommandWork",
    "puzzle3d_shell_only_emit",
    "DownloadMediaExport",
    "RequestFileOpen",
]
for needle in needles:
    idx = 0
    n = 0
    while n < 12:
        i = text.find(needle, idx)
        if i < 0:
            break
        line = text[:i].count("\n") + 1
        start = max(0, i - 80)
        end = min(len(text), i + 160)
        snippet = text[start:end].replace("\n", " / ")
        print(f"EDITOR {needle} @{line}: {snippet}")
        idx = i + len(needle)
        n += 1

ut = unit.read_text()
for needle in [
    "export_fixture_downloads",
    "import_fixture_reproduces",
    "file_menu_import_row",
    "open_import_fixture_requests",
    "import_fixture_of_a_distinct",
    "exported_fixture_bytes_reimport",
    "Puzzle3dWindowCommandWork",
    "fn dispatch(",
]:
    i = ut.find(needle)
    if i < 0:
        print("UNIT MISS", needle)
        continue
    line = ut[:i].count("\n") + 1
    print(f"UNIT {needle} @{line}")

# dump factory match region
i = text.find("Puzzle3dWindowCommandWork")
# find the match arm block around importFixture
i = text.find('"importFixture" => Puzzle3dWindowCommandWork')
if i < 0:
    i = text.find("importFixture")
    print("no exact factory arm, first importFixture", text[:i].count("\n")+1 if i>=0 else None)
else:
    line = text[:i].count("\n") + 1
    (out / "b16-factory-region.txt").write_text(f"line {line}\n" + text[i-800:i+600])
    print("wrote factory region around", line)

# dump unit laws region
i = ut.find("export_fixture_downloads")
if i >= 0:
    line = ut[:i].count("\n") + 1
    (out / "b16-unit-export-import.txt").write_text(f"line {line}\n" + ut[i-200:i+8000])
    print("wrote unit export-import around", line)

# leftover HostOnly in plugin
pt = plugin.read_text()
i = 0
hits = []
while True:
    j = pt.find("leftover", i)
    if j < 0:
        break
    line = pt[:j].count("\n") + 1
    if 19000 <= line <= 22000:
        hits.append((line, pt[max(0,j-100):j+200].replace("\n"," / ")))
    i = j + 8
print("plugin leftover hits in 19k-22k:", len(hits))
for line, snip in hits[:20]:
    print(f"PLUGIN leftover @{line}: {snip[:220]}")
