from pathlib import Path
import re

eng, plugin, app3d, app2d = [Path(p) for p in Path("/tmp/procedural-paths.txt").read_text().splitlines()]

public_fn = """
/// 🔗 Registers in-process flow extension operators so eval + tessellate share one brep kernel.
/// Safe to call repeatedly; installers are registered once and the host registry is rebuilt.
pub fn ensure_linked_flow_extensions() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        flow::register_linked_flow_extension_installer("brep", semio_s_plugin_flow_extension_brep::register);
        flow::register_linked_flow_extension_installer("math", semio_s_plugin_flow_extension_math::register);
        flow::register_linked_flow_extension_installer("primitive", semio_s_plugin_flow_extension_primitive::register);
        flow::register_linked_flow_extension_installer("logic", semio_s_plugin_flow_extension_logic::register);
        flow::register_linked_flow_extension_installer("dictionary", semio_s_plugin_flow_extension_dictionary::register);
        flow::register_linked_flow_extension_installer("list", semio_s_plugin_flow_extension_list::register);
        flow::register_linked_flow_extension_installer("text", semio_s_plugin_flow_extension_text::register);
        flow::sync_host_flow_extension_contributions("[]");
    });
}

"""

et = eng.read_text()
if "pub fn ensure_linked_flow_extensions()" not in et:
    marker = "//#region 🧪️TestSupport"
    et = et.replace(marker, public_fn + marker, 1)
    start = et.find("    fn ensure_linked_flow_extensions()")
    if start >= 0:
        i = et.find("{", start)
        depth = 0
        j = i
        while j < len(et):
            if et[j] == "{":
                depth += 1
            elif et[j] == "}":
                depth -= 1
                if depth == 0:
                    j += 1
                    break
            j += 1
        while j < len(et) and et[j] == "\n":
            j += 1
        et = et[:start] + et[j:]
        print("removed private ensure")
    eng.write_text(et)
    print("engine updated")
else:
    print("engine already public")

pt = plugin.read_text()
pt2 = pt.replace(
    "install_linked_flow_extensions();",
    "crate::artifacts::procedural3d::engine::ensure_linked_flow_extensions();",
)
m = re.search(
    r"\n/// 🔗 Registers in-process flow extension operators.*?\nfn install_linked_flow_extensions\(\) \{.*?\n\}\n",
    pt2,
    flags=re.S,
)
if m:
    pt2 = pt2[: m.start()] + "\n" + pt2[m.end() :]
    print("removed install_linked fn")
else:
    print("WARN: install_linked fn not removed")
plugin.write_text(pt2)
print("plugin written, still has install?", "fn install_linked_flow_extensions" in pt2)

for app, kind in [(app3d, "3d"), (app2d, "2d")]:
    t = app.read_text()
    if kind == "3d":
        pairs = [
            (
                "    pub fn app() -> Procedural3dApp {\n        new_app::<Procedural3dPlayApp>()\n    }",
                "    pub fn app() -> Procedural3dApp {\n        crate::artifacts::procedural3d::engine::ensure_linked_flow_extensions();\n        new_app::<Procedural3dPlayApp>()\n    }",
            ),
            (
                "    pub fn app_with_registry() -> Procedural3dApp {\n        new_app_with_registry::<Procedural3dPlayApp>(create_procedural3d_app)\n    }",
                "    pub fn app_with_registry() -> Procedural3dApp {\n        crate::artifacts::procedural3d::engine::ensure_linked_flow_extensions();\n        new_app_with_registry::<Procedural3dPlayApp>(create_procedural3d_app)\n    }",
            ),
        ]
    else:
        pairs = [
            (
                "    pub fn app() -> Procedural2dApp {\n        new_app::<Procedural2dPlayApp>()\n    }",
                "    pub fn app() -> Procedural2dApp {\n        crate::artifacts::procedural3d::engine::ensure_linked_flow_extensions();\n        new_app::<Procedural2dPlayApp>()\n    }",
            ),
            (
                "    pub fn app_with_registry() -> Procedural2dApp {\n        new_app_with_registry::<Procedural2dPlayApp>(create_procedural2d_app)\n    }",
                "    pub fn app_with_registry() -> Procedural2dApp {\n        crate::artifacts::procedural3d::engine::ensure_linked_flow_extensions();\n        new_app_with_registry::<Procedural2dPlayApp>(create_procedural2d_app)\n    }",
            ),
        ]
    t2 = t
    for a, b in pairs:
        if a not in t2:
            print(kind, "MISSING snippet")
        else:
            t2 = t2.replace(a, b)
            print(kind, "replaced one")
    app.write_text(t2)
    print(kind, "done")
