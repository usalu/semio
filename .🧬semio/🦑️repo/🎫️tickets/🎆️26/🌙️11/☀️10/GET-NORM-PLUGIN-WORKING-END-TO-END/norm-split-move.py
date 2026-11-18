"""🚚️ Moves norm families from one split plugin into a new extension plugin: hub source, manifests, playground rows, descriptor, workspace and taxonomy."""
import json, re, sys
from pathlib import Path

ROOT = Path(sys.argv[1])
NORM = ROOT / "🌎️hub/🧩️compositions/📕️norm"
FW7 = "../../../../../../../🧰️framework"
SOURCES = {"norm": NORM, "norm-eurocode-materials": NORM / "🧩️extensions/📙️eurocode-materials"}
MOVES = [
    dict(source="norm", owner="norm-energy", families=["din18599"], dir="📒️energy", enum="NormEnergyApps", label="Norm Energy", emoji="📒️",
         summary="building energy performance assessment (DIN V 18599), composed with semio members", extra=["semio-s-artifact-stdio-semio"]),
    dict(source="norm-eurocode-materials", owner="norm-eurocode-structural", families=["en1992", "en1993", "en1994"], dir="📓️eurocode-structural", enum="NormEurocodeStructuralApps",
         label="Norm Eurocode Structural", emoji="📓️", summary="Eurocode concrete, steel and composite structures (EN 1992, EN 1993, EN 1994)", extra=[]),
]
MATERIALS_SUMMARY = ("Eurocode material design: concrete, steel, composite, timber, masonry and aluminium (EN 1992–1996, EN 1999)", "Eurocode timber, masonry and aluminium structures (EN 1995, EN 1996, EN 1999)")


def mentions(line, families):
    return any(re.search(rf"\b{f}\b|norm_{f}(_|::)|norm-{f}(-| =)", line) for f in families)


for move in MOVES:
    src = SOURCES[move["source"]]
    fam = move["families"]
    crate = f"semio-hub-{move['owner']}"
    lines = (src / "🦀️.rs").read_text().splitlines(keepends=True)
    variants = "".join(l for l in lines if re.match(r"\s+[A-Z]\w+(Editor|Viewer)\(VcsArtifactApp<", l) and mentions(l, fam))
    lets = "".join(l for l in lines if l.lstrip().startswith("let ") and mentions(l, fam))
    chain = "".join(l for l in lines if re.match(r"\s+\.(artifact|editor|editor_mutation_roster|viewer|viewer_mutation_roster|activation)\b", l) and mentions(l, fam))
    assert variants and lets and chain, move["owner"]
    (src / "🦀️.rs").write_text("".join(l for l in lines if not mentions(l, fam)))

    hub_toml_path = src / "📦️packages/🦀️rust/Cargo.toml"
    hub_toml = hub_toml_path.read_text()
    head, dev = hub_toml.split("[dev-dependencies]", 1) if "[dev-dependencies]" in hub_toml else (hub_toml, None)
    head_lines = head.splitlines(keepends=True)
    moved_deps = [l for l in head_lines if mentions(l, fam) or any(l.startswith(e + " =") for e in move["extra"])]
    head = "".join(l for l in head_lines if l not in moved_deps)
    hub_toml_path.write_text(head + ("[dev-dependencies]" + dev if dev is not None else ""))

    comp_path = src / "🧩️component/📦️packages/🦀️rust/Cargo.toml"
    comp = comp_path.read_text()
    first, *rows = comp.split("[[package.metadata.semio.playground]]")
    last, lib = rows[-1].split("\n[lib]", 1)
    rows[-1] = last
    keep = [r for r in rows if re.search(r'variant = "(\w+)"', r).group(1) not in fam]
    moved_rows = [r for r in rows if re.search(r'variant = "(\w+)"', r).group(1) in fam]
    comp_path.write_text(first + "".join("[[package.metadata.semio.playground]]" + r for r in keep).rstrip("\n") + "\n\n[lib]" + lib)

    desc_path = src / "🧩️component/🔣️.json"
    whole = json.loads(desc_path.read_text())

    def family(value):
        found = re.search(r"norm\.(\w+)", json.dumps(value, ensure_ascii=False))
        return found.group(1) if found else None

    def project(d, families, owner, label):
        d = json.loads(json.dumps(d))
        d["packageId"] = f"semio:{owner}"
        m = d["manifest"]; m["pluginId"] = owner; m["label"] = label
        m["apps"] = [a for a in m["apps"] if family(a["id"]) in families]
        m["examples"] = [e for e in m["examples"] if family(e["dialect"]) in families]
        d["activationEvents"] = [e for e in d["activationEvents"] if family(e) in families]
        d["contributions"] = {k: [x for x in v if family(x) is None or family(x) in families] for k, v in d["contributions"].items()}
        return d

    remaining = sorted({family(a["id"]) for a in whole["manifest"]["apps"]} - set(fam))
    desc_path.write_text(json.dumps(project(whole, remaining, whole["manifest"]["pluginId"], whole["manifest"]["label"]), ensure_ascii=False, indent=2) + "\n")

    base = NORM / "🧩️extensions" / move["dir"]
    (base / "📦️packages/🦀️rust").mkdir(parents=True, exist_ok=True)
    (base / "🧩️component/📦️packages/🦀️rust").mkdir(parents=True, exist_ok=True)
    (base / "🦀️.rs").write_text(f"""//! {move['emoji']} {move['label']} plugin — {move['summary']}.

#![allow(async_fn_in_trait)]

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::kernel::{{ActivationEvent, CapabilityId, CapabilityRequest}};
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{{ExecutionMode, Plugin, PluginApp, PluginAssemblyError}};
use semio_plugin::PluginBuilder;

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {{
    /// 🗃️ Closed runtime app fleet of the {move['label']} editor/viewer pairs.
    pub enum {move['enum']}: PluginApp {{
{variants}    }}
}}
//#endregion 🗃️Apps

/// 🔌️ Builds the {move['label']} plugin: one artifact declaration, editor/viewer pair and activation per family,
/// split from `norm` so each component stays below the wasm function-count limit.
pub fn plugin() -> Result<Plugin<{move['enum']}>, PluginAssemblyError> {{
{lets}    PluginBuilder::<_, {move['enum']}>::new("{move['owner']}")
        .label("{move['label']}")
        .version("0.1.0")
        .package_id("semio:{move['owner']}")
        .shell_hooks(std::sync::Arc::new(semio_s_plugin_norm_app_surface::NormShellHooks))
{chain}        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {{ id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist norm family edits to the open document".into(), optional: false }})
        .try_build()
}}
""")
    (base / "📦️packages/🦀️rust/Cargo.toml").write_text(f"""[package]
workspace = "../../../../../.."
name = "{crate}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "{move['label']} plugin — {move['summary']}"

[lints]
workspace = true

[package.metadata.semio]
role = "hub"

[lib]
path = "../../🦀️.rs"

[dependencies]
{''.join(moved_deps)}semio-framework-pack = {{ path = "{FW7}/🔨️modules/🎒️pack/📦️packages/🦀️rust" }}
semio-ui-locale = {{ path = "{FW7}/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust" }}
semio-framework = {{ workspace = true }}
semio-framework-value = {{ workspace = true }}
semio-framework-dispatch-macros = {{ workspace = true }}
semio-s-plugin-norm-app-surface = {{ workspace = true }}
semio-framework-job = {{ workspace = true }}
semio-os-kernel = {{ workspace = true }}
semio-plugin = {{ workspace = true }}
""")
    rel = f"🌎️hub/🧩️compositions/📕️norm/🧩️extensions/{move['dir']}"
    (base / "📦️packages/🦀️rust/📋️project.json").write_text(json.dumps({
        "name": f"@semio-tech/{move['owner']}-plugin",
        "$schema": f"{FW7}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json",
        "namedInputs": {"default": [f"{{workspaceRoot}}/{rel}/**/*.rs", "{projectRoot}/**/*"]},
        "targets": {"test": {"executor": "repo:script", "options": {"cwd": f"{rel}/📦️packages/🦀️rust", "command": "repo ./📜️script.ts test", "forwardAllArgs": True}, "outputs": [], "cache": True}},
    }, ensure_ascii=False, indent=2) + "\n")
    (base / "📦️packages/🦀️rust/📜️script.ts").write_text(f"""#!/usr/bin/env repo
import {{ receiveScriptProcessInvocation }} from "{FW7}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {{ resolveTestLevel }} from "{FW7}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** {move['emoji']} `@semio-tech/{move['owner']}-plugin` router: `repo ./📜️script.ts test [quick|long|exhaustive]`. */
import {{ runRepositoryCargoTests }} from "{FW7}/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import {{ BundleScript, ScriptRouter }} from "{FW7}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {{ runScriptMain }} from "{FW7}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

class TestScript extends BundleScript {{
  async run(segments: string[]): Promise<void> {{
    const {{ rest }} = resolveTestLevel(segments);
    await runRepositoryCargoTests(["{crate}"], this.repoRoot, this.invocation, rest);
  }}
}}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, {{ invocation: original, defaultCommand: "test" }}));
""")
    lib_name = crate.replace("-", "_")
    (base / "🧩️component/🦀️.rs").write_text(f"""//! {move['emoji']} The `{move['owner']}` plugin component: the one cdylib of `{crate}`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, {lib_name}::plugin, {lib_name}::{move['enum']});
""")
    (base / "🧩️component/📦️packages/🦀️rust/Cargo.toml").write_text(f"""[package]
workspace = "../../../../../../.."
name = "{crate}-component"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "The `{move['owner']}` plugin component: the one cdylib of `{crate}`, holding only its entry point"

[lints]
workspace = true

[package.metadata.component]
package = "semio:{move['owner']}"

[package.metadata.semio]
role = "hub"
deployment-directory = "{move['emoji']}{move['owner']}"
component-kind = "plugin"

{''.join("[[package.metadata.semio.playground]]" + r for r in moved_rows).rstrip(chr(10))}

[lib]
crate-type = ["cdylib"]
path = "../../🦀️.rs"

[dependencies]
semio-framework-pack = {{ path = "../{FW7}/🔨️modules/🎒️pack/📦️packages/🦀️rust" }}
{crate} = {{ path = "../../../📦️packages/🦀️rust" }}
semio-plugin = {{ workspace = true }}
""")
    (base / "🧩️component/🔣️.json").write_text(json.dumps(project(whole, fam, move["owner"], move["label"]), ensure_ascii=False, indent=2) + "\n")

    ws_path = ROOT / "🌎️hub/Cargo.toml"
    ws = ws_path.read_text()
    member = f'    "🧩️compositions/📕️norm/🧩️extensions/{move["dir"]}/📦️packages/🦀️rust",\n    "🧩️compositions/📕️norm/🧩️extensions/{move["dir"]}/🧩️component/📦️packages/🦀️rust",\n'
    anchor = '    "🧩️compositions/📕️norm/🧩️component/📦️packages/🦀️rust",\n'
    if move["dir"] not in ws: ws_path.write_text(ws.replace(anchor, anchor + member, 1))
    tax_path = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
    tax = tax_path.read_text()
    deploy = f'{move["emoji"]}{move["owner"]}'
    if f'"{deploy}"' not in tax: tax_path.write_text(tax.replace('        "📕️norm",\n', f'        "📕️norm",\n        "{deploy}",\n', 1))
    print(move["owner"], "apps", variants.count("VcsArtifactApp"), "rows", len(moved_rows), "deps", len(moved_deps))

materials = NORM / "🧩️extensions/📙️eurocode-materials"
for path in [materials / "🦀️.rs", materials / "📦️packages/🦀️rust/Cargo.toml"]:
    path.write_text(path.read_text().replace(*MATERIALS_SUMMARY))
