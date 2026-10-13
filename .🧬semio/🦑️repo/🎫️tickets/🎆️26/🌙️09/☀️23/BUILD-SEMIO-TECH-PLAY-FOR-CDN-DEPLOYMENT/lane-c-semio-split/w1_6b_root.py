#!/usr/bin/env python3
"""🔌️ W1 step 6b: `SemioConversion` in the foundation; the monolith root declares and registers the conversions its
features select, through the conversion roots' `CONVERSION`s."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *

root = read(ROOT_RS)
old = "    subsets::video::io::declare(builder).try_build()\n}\n"
assert root.count(old) == 1
root = root.replace(old, "    let builder = subsets::video::io::declare(builder);\n    CONVERSIONS.iter().fold(builder, |builder, conversion| (conversion.declare)(builder)).try_build()\n}\n\n"
    "/// 🔗️ The conversions this build's features select, each subset's import before its export.\nconst CONVERSIONS: &[crate::base_foundation::SemioConversion] = &[\n"
    + "".join('    #[cfg(feature = "conversion-%s")]\n    subsets::%s::io::import::CONVERSION,\n    #[cfg(feature = "conversion-%s")]\n    subsets::%s::io::export::CONVERSION,\n' % (s, s, s, s) for s in CONVERSION_SUBSETS)
    + "];\n")
old = "    subsets::base::io::register();\n}\n"
assert root.count(old) == 1
root = root.replace(old, "    subsets::base::io::register();\n    for conversion in CONVERSIONS {\n        (conversion.register)();\n    }\n}\n")
write(ROOT_RS, root)
f = SUBSETS + "/✉️base/🦀️.rs"
write(f, read(f).rstrip("\n") + '''

/// 🔗️ One direction of one subset's conversions: `declare` adds the composer rows that write a semio dialect to an
/// artifact declaration, `register` publishes all of the direction's rows into the process-wide io registry.
#[derive(Clone, Copy)]
pub struct SemioConversion {
    pub declare: fn(semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>,
    pub register: fn(),
}
''')
