#!/usr/bin/env python3
"""⚙️ Replaces the artifact root's `#[path]` mounts of the engine and the epw extension with crate-root aliases of the
new engine/epw crates and drops the flat engine re-export surface (lane P1, slice S1c).

Every `crate::<engine module>::Item` path in the artifact keeps naming its owner through `extern crate … as <module>;`.
Also mounts the re-homed BESTEST catalogue laws under `examples` and adds the engine crates to the artifact manifest and
workspace. Anchors asserted unique; refuses to run twice.
"""
import os, re, sys
PLUGIN = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy"
ART = PLUGIN + "/🗿️artifacts/🔋️model"
ENGINE = PLUGIN + "/🔨️modules/⚡️simulation/⚙️engine"
root_p = ART + "/🦀️.rs"; s = open(root_p, encoding="utf-8").read()
if "extern crate semio_s_plugin_energy_engine_" in s: sys.exit("already applied")
mods = sorted(re.sub(r"^[^a-z_]+", "", d) for d in os.listdir(ENGINE) if os.path.isdir(os.path.join(ENGINE, d)))
start = '#[path = "../../🧩️extensions/🌦️epw/🦀️.rs"]\npub mod epw;\n'
end = "//#endregion 🔖️FlatReExports\n"
assert s.count(start) == 1 and s.count(end) == 1
i, j = s.index(start), s.index(end) + len(end)
block = ("extern crate semio_s_plugin_energy_epw as epw;\n\n//#region ⚡️SimulationEngine\n"
         + "".join(f"extern crate semio_s_plugin_energy_engine_{m} as {m};\n" for m in mods) + "//#endregion ⚡️SimulationEngine\n")
s = s[:i] + block + s[j:]
s = s.replace("//! 🎪 Energy model artifact — headless BEM document surface over `crate::Model`.", "//! 🎪 Energy model artifact — headless BEM document surface over `crate::model::Model`.")
a = '''        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-950/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
}'''
assert s.count(a) == 1
s = s.replace(a, a[:-2] + '''    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🔬️bestest-catalog/🦀️.rs"]
    mod bestest_catalog_tests;
}''')
open(root_p, "w", encoding="utf-8").write(s)
man_p = ART + "/📦️packages/🦀️rust/Cargo.toml"; m = open(man_p, encoding="utf-8").read()
a = "semio-s-artifact-stdio-zip = { workspace = true }\n"; assert m.count(a) == 1
m = m.replace(a, a + "semio-s-plugin-energy-epw = { workspace = true }\n" + "".join(f"semio-s-plugin-energy-engine-{x.replace('_', '-')} = {{ workspace = true }}\n" for x in mods))
open(man_p, "w", encoding="utf-8").write(m)
ws_p = ART + "/Cargo.toml"; w = open(ws_p, encoding="utf-8").read()
a = "[workspace.dependencies.serde]\n"; assert w.count(a) == 1
rows = "[workspace.dependencies.semio-s-plugin-energy-epw]\npath = \"../../🧩️extensions/🌦️epw/📦️packages/🦀️rust\"\n\n"
for d in sorted(os.listdir(ENGINE)):
    if os.path.isdir(os.path.join(ENGINE, d)):
        x = re.sub(r"^[^a-z_]+", "", d).replace("_", "-")
        rows += f"[workspace.dependencies.semio-s-plugin-energy-engine-{x}]\npath = \"../../🔨️modules/⚡️simulation/⚙️engine/{d}/📦️packages/🦀️rust\"\n\n"
w = w.replace(a, rows + a)
open(ws_p, "w", encoding="utf-8").write(w)
print("ok", len(mods))
