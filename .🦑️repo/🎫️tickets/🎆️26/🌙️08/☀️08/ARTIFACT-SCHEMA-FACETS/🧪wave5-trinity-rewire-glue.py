from pathlib import Path

glue = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs")
t = glue.read_text()

for folder, mod in [("🔌️jack", "jack"), ("♻️rewrite", "rewrite")]:
    old = (
        f'        pub use component::*;\n\n'
        f'        #[path = "."]\n'
        f'        pub mod diff {{\n'
        f'            #[path = "../../🗿️artifacts/{folder}/🔺️diff/🦀️component.rs"]\n'
        f'            mod component;\n'
        f'            pub use component::*;\n'
        f'        }}'
    )
    new = (
        f'        pub use component::*;\n\n'
        f'        #[path = "../../🗿️artifacts/{folder}/🧬️schema/🦀️component.rs"]\n'
        f'        pub mod schema;\n\n'
        f'        #[path = "."]\n'
        f'        pub mod diff {{\n'
        f'            #[path = "../../🗿️artifacts/{folder}/🔺️diff/🦀️component.rs"]\n'
        f'            mod component;\n'
        f'            pub use component::*;\n\n'
        f'            #[path = "../../🗿️artifacts/{folder}/🔺️diff/🧬️schema/🦀️component.rs"]\n'
        f'            pub mod schema;\n'
        f'            pub use schema::*;\n'
        f'        }}'
    )
    if old not in t:
        raise SystemExit(f"old diff block missing {mod}")
    t = t.replace(old, new, 1)

    old_pack = (
        f'        #[path = "."]\n'
        f'        pub mod pack {{\n'
        f'            #[path = "../../🗿️artifacts/{folder}/🎒️pack/🦀️component.rs"]\n'
        f'            mod component;\n'
        f'            pub use component::*;\n'
        f'        }}'
    )
    new_pack = (
        f'        #[path = "."]\n'
        f'        pub mod snapshot {{\n'
        f'            #[path = "../../🗿️artifacts/{folder}/📸️snapshot/🧬️schema/🦀️component.rs"]\n'
        f'            pub mod schema;\n'
        f'            #[path = "../../🗿️artifacts/{folder}/📸️snapshot/🎒️pack/🦀️component.rs"]\n'
        f'            pub mod pack;\n'
        f'        }}'
    )
    if old_pack not in t:
        raise SystemExit(f"old pack missing {mod}")
    t = t.replace(old_pack, new_pack, 1)
    print("done", mod)

glue.write_text(t)
print("schema count", t.count("🧬️schema/🦀️component.rs"))
print("snapshot pack count", t.count("📸️snapshot/🎒️pack"))
