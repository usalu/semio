from pathlib import Path
import re

fem = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem")
arts = list((fem / "🗿️artifacts").iterdir())
folder2 = next(a.name for a in arts if "2" in a.name)
folder3 = next(a.name for a in arts if "3" in a.name)

glue_path = fem / "📦️packages" / "🦀️rust" / "📦️glue.rs"
g = glue_path.read_text()


def patch(g, folder):
    old_diff = (
        f'        #[path = "../../🗿️artifacts/{folder}/🔺️diff/🦀️component.rs"]\n'
        f"        pub mod diff;"
    )
    old_pack = (
        f'        #[path = "../../🗿️artifacts/{folder}/🎒️pack/🦀️component.rs"]\n'
        f"        pub mod pack;"
    )
    replacement = "\n".join([
        f'        #[path = "../../🗿️artifacts/{folder}/🧬️schema/🦀️component.rs"]',
        "        pub mod schema;",
        "",
        '        #[path = "."]',
        "        pub mod diff {",
        f'            #[path = "../../🗿️artifacts/{folder}/🔺️diff/🦀️component.rs"]',
        "            mod component;",
        "            pub use component::*;",
        f'            #[path = "../../🗿️artifacts/{folder}/🔺️diff/🧬️schema/🦀️component.rs"]',
        "            pub mod schema;",
        "            pub use schema::*;",
        "        }",
        "",
        '        #[path = "."]',
        "        pub mod snapshot {",
        f'            #[path = "../../🗿️artifacts/{folder}/📸️snapshot/🧬️schema/🦀️component.rs"]',
        "            pub mod schema;",
        f'            #[path = "../../🗿️artifacts/{folder}/📸️snapshot/🎒️pack/🦀️component.rs"]',
        "            pub mod pack;",
        "        }",
    ])
    if old_diff in g:
        g = g.replace(old_diff, "", 1)
        print("removed diff", folder)
    else:
        print("NO diff", folder)
    if old_pack in g:
        g = g.replace(old_pack, replacement, 1)
        print("replaced pack", folder)
    else:
        print("NO pack", folder)
    return g

g = patch(g, folder2)
g = patch(g, folder3)
g = g.replace("📄set-document", "📄set-snapshot")
if "extern crate semio_framework_schema as schema;" not in g:
    g = g.replace(
        "extern crate semio_framework_os_kernel as protocol;",
        "extern crate semio_framework_os_kernel as protocol;\nextern crate semio_framework_schema as schema;",
    )
glue_path.write_text(g)
print("snapshot", g.count("pub mod snapshot"), "rootpack", g.count('/🎒️pack/🦀️component.rs"]'), "setdoc", "set-document" in g)

cargo = fem / "📦️packages" / "🦀️rust" / "Cargo.toml"
c = cargo.read_text()
if "semio-framework-schema" not in c:
    fw = None
    for line in c.splitlines():
        if "path =" in line and "framework" in line:
            m = re.search(r"(?:\.\./)+([^/]*framework)", line)
            if m:
                fw = m.group(1)
                break
    print("fw", fw)
    out = []
    for line in c.splitlines():
        out.append(line)
        if line.startswith("semio-framework-plugin"):
            out.append(f'semio-framework-schema = {{ path = "../../../../../{fw}/🔨️modules/🧬️schema/📦️packages/🦀️rust", package = "semio-framework-schema" }}')
    cargo.write_text("\n".join(out) + "\n")
    print("cargo updated")
else:
    print("cargo ok")
