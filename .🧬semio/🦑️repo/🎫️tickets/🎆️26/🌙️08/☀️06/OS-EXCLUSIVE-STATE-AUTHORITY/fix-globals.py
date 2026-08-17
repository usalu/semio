
from pathlib import Path
import re

def read(p):
    return Path(p).read_text()
def write(p,s):
    Path(p).write_text(s)

# animate
f = "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎬️core/🦀️component.rs"
s = read(f)
s = s.replace("static TYPST_FONTS: OnceLock<Vec<Font>> = OnceLock::new();\n", "")
s = s.replace("static TYPST_BOOK: OnceLock<LazyHash<FontBook>> = OnceLock::new();\n", "")
s = s.replace("TYPST_FONTS.get_or_init(|| ", "(")
s = s.replace("TYPST_BOOK.get_or_init(|| ", "(")
write(f,s)
print("animate OnceLock left", "OnceLock" in s, "TYPST_FONTS" in s)

# process workpiece
for p in Path("✏️s/🔌️plugins").rglob("*workpiece*component.rs"):
    s = read(p)
    if "PROCESS3D_PREVIEW_CACHE" not in s: continue
    s = s.replace("static PROCESS3D_PREVIEW_CACHE: OnceLock<Mutex<Option<Process3dPreviewCache>>> = OnceLock::new();\n", "")
    s = s.replace("PROCESS3D_PREVIEW_CACHE.get_or_init(|| Mutex::new(None))", "&Mutex::new(None)")
    write(p,s)
    print("process", p)

# trinity
for p in Path("✏️s/🔌️plugins").rglob("*/core/component.rs"):
    pass
for p in Path("✏️s").rglob("*trinity*/**/*component.rs"):
    s = read(p)
    if "TRINITY_JACK_MANIFEST" in s:
        s = s.replace("static TRINITY_JACK_MANIFEST: OnceLock<GraphManifest> = OnceLock::new();\n", "")
        s = s.replace("TRINITY_JACK_MANIFEST.get_or_init(|| ", "(")
        write(p,s)
        print("trinity", p)

# space thread_local - remove entirely if test-only and wrap usages
f = "✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🦀️component.rs"
s = read(f)
# remove thread_local block
s2 = re.sub(r"#\[cfg\(test\)\]\nthread_local! \{\nstatic STUDIO_TEST_APP: RefCell<SpaceApp> = RefCell::new\(SpaceApp::default\(\)\);\n\}", "", s)
s2 = re.sub(r"thread_local! \{\nstatic STUDIO_TEST_APP: RefCell<SpaceApp> = RefCell::new\(SpaceApp::default\(\)\);\n\}", "", s2)
write(f,s2)
print("space tls removed", "STUDIO_TEST_APP" in s2)

# compiler fonts
for p in Path("테्트framework").rglob("**/compiler/**/component.rs") if False else Path(".").glob("**/compiler/**/🦀️component.rs"):
    pass
for p in Path(".").rglob("*compiler*/**/🦀️component.rs"):
    if "framework" not in str(p): continue
    s = read(p)
    if "static FONTS: OnceLock" in s:
        s = s.replace("static FONTS: OnceLock<Fonts> = OnceLock::new();\n", "")
        s = s.replace("FONTS.get_or_init(|| ", "(")
        write(p,s)
        print("compiler", p)

# procedural test serial - delete line
f = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/✨️procedural3d/⚙️engine/🦀️component.rs"
# find
for p in Path("✏️s").rglob("*procedural3d*/**/🦀️component.rs"):
    s = read(p)
    if "TEST_SERIAL" in s:
        s = re.sub(r"#\[cfg\(test\)\]\nstatic TEST_SERIAL: Mutex<\(\)> = Mutex::new\(\(\)\);\n", "", s)
        s = re.sub(r"static TEST_SERIAL: Mutex<\(\)> = Mutex::new\(\(\)\);\n", "", s)
        write(p,s)
        print("procedural", p)

# cad interaction leftover OnceLock
f = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/⚙️engine/🕹️interaction/🦀️component.rs"
s = read(f)
print("cad once", "OnceLock" in s, "PARSED_SPECS" in s, "static CATALOG" in s)
