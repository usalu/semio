from pathlib import Path
import re

glue_path = Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs')
g = glue_path.read_text()

if 'extern crate semio_framework_schema as schema;' not in g:
    g = g.replace(
        'extern crate semio_framework_os_kernel as protocol;',
        'extern crate semio_framework_os_kernel as protocol;\nextern crate semio_framework_schema as schema;',
    )

def patch_artifact_block(g, folder):
    old = f'''        #[path = "../../🗿️artifacts/{folder}/🎒️pack/🦀️component.rs"]
        pub mod pack;'''
    new = f'''        #[path = "../../🗿️artifacts/{folder}/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {{
            #[path = "../../🗿️artifacts/{folder}/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/{folder}/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }}

        #[path = "."]
        pub mod snapshot {{
            #[path = "../../🗿️artifacts/{folder}/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/{folder}/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }}'''
    old_diff = f'''        #[path = "../../🗿️artifacts/{folder}/🔺️diff/🦀️component.rs"]
        pub mod diff;'''
    if old_diff in g:
        g = g.replace(old_diff, '')
    if old in g:
        g = g.replace(old, new)
        print('replaced pack for', folder)
    else:
        print('WARN no pack for', folder)
    return g

g = patch_artifact_block(g, '◻2d')
g = patch_artifact_block(g, '🧊️3d')
g = g.replace('📄set-document', '📄set-snapshot')
glue_path.write_text(g)
print('glue written')

cargo = Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml')
c = cargo.read_text()
if 'semio-framework-schema' not in c:
    needle = 'semio-framework-plugin = { path = "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust", features = ["component-guest"], package = "semio-framework-plugin" }'
    insert = needle + '\nsemio-framework-schema = { path = "../../../../../� minkframework/🔨️modules/🧬️schema/📦️packages/🦀️rust", package = "semio-framework-schema" }'
    insert = needle + '\nsemio-framework-schema = { path = "../../../../../� minkframework/🔨️modules/🧬️schema/📦️packages/🦀️rust", package = "semio-framework-schema" }'.replace('� minkframework','🧰️framework')
    insert = needle + '\nsemio-framework-schema = { path = "../../../../../� minkframework/🔨️modules/🧬️schema/📦️packages/🦀️rust", package = "semio-framework-schema" }'
