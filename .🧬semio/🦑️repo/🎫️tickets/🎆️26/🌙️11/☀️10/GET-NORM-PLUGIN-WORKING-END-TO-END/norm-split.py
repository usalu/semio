"""🪓️ Splits the norm hub into `norm`, `norm-eurocode` and `norm-eurocode-materials` so no component exceeds the wasm function limit."""
import json, re, sys
from pathlib import Path

ROOT = Path(sys.argv[1])
HUB = ROOT / "🌎️hub"
NORM = HUB / "🧩️compositions/📕️norm"
OWNERS = {
    "norm": dict(families=["din4108", "din16798", "din18599", "iso16757", "vdi3805"]),
    "norm-eurocode": dict(families=["en1990", "en1991", "en1997", "en1998"], dir="📗️eurocode", deploy="📗️norm-eurocode", enum="NormEurocodeApps", label="Norm Eurocode", crate="semio-hub-norm-eurocode", emoji="📗️",
                          summary="Eurocode basis of design, actions, geotechnical and seismic design (EN 1990, EN 1991, EN 1997, EN 1998)"),
    "norm-eurocode-materials": dict(families=["en1992", "en1993", "en1994", "en1995", "en1996", "en1999"], dir="📙️eurocode-materials", deploy="📙️norm-eurocode-materials", enum="NormEurocodeMaterialsApps", label="Norm Eurocode Materials", crate="semio-hub-norm-eurocode-materials", emoji="📙️",
                                    summary="Eurocode material design: concrete, steel, composite, timber, masonry and aluminium (EN 1992–1996, EN 1999)"),
}
MOVED = [f for o in ("norm-eurocode", "norm-eurocode-materials") for f in OWNERS[o]["families"]]


def mentions(line: str, families: list[str]) -> bool:
    return any(re.search(rf"\b{f}\b|norm_{f}(_|::)|norm-{f}(-| =)", line) for f in families)


source = (NORM / "🦀️.rs").read_text()
lines = source.splitlines(keepends=True)


def body(families: list[str], marker: str) -> str:
    return "".join(l for l in lines if marker in l and mentions(l, families))


def extension_source(owner: str) -> str:
    o = OWNERS[owner]
    fam = o["families"]
    variants = "".join(l for l in lines if re.match(r"\s+[A-Z]\w+(Editor|Viewer)\(VcsArtifactApp<", l) and mentions(l, fam))
    lets = "".join(l for l in lines if l.lstrip().startswith("let ") and mentions(l, fam))
    chain = "".join(l for l in lines if re.match(r"\s+\.(artifact|editor|editor_mutation_roster|viewer|viewer_mutation_roster|activation)\b", l) and mentions(l, fam))
    return f"""//! {o['emoji']} {o['label']} plugin — {o['summary']}.

#![allow(async_fn_in_trait)]

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::kernel::{{ActivationEvent, CapabilityId, CapabilityRequest}};
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{{ExecutionMode, Plugin, PluginApp, PluginAssemblyError}};
use semio_plugin::PluginBuilder;

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {{
    /// 🗃️ Closed runtime app fleet of the {o['label']} editor/viewer pairs.
    pub enum {o['enum']}: PluginApp {{
{variants}    }}
}}
//#endregion 🗃️Apps

/// 🔌️ Builds the {o['label']} plugin: one artifact declaration, editor/viewer pair and activation per family,
/// split from `norm` so each component stays below the wasm function-count limit.
pub fn plugin() -> Result<Plugin<{o['enum']}>, PluginAssemblyError> {{
{lets}    PluginBuilder::<_, {o['enum']}>::new("{owner}")
        .label("{o['label']}")
        .version("0.1.0")
        .package_id("semio:{owner}")
        .shell_hooks(std::sync::Arc::new(semio_s_plugin_norm_app_surface::NormShellHooks))
{chain}        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {{ id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist norm family edits to the open document".into(), optional: false }})
        .try_build()
}}
"""


main = "".join(l for l in lines if not mentions(l, MOVED))
main = main.replace("all fifteen norm-family editor/viewer pairs", "the DIN, ISO and VDI norm-family editor/viewer pairs").replace("covering all fifteen editors", "covering all five editors").replace("with fifteen data declarations", "with five data declarations")
(NORM / "🦀️.rs").write_text(main)

hub_toml = (NORM / "📦️packages/🦀️rust/Cargo.toml").read_text()
deps_head, dev = hub_toml.split("[dev-dependencies]", 1)
family_deps = {f: [l for l in deps_head.splitlines(keepends=True) if mentions(l, [f])] for f in MOVED}
deps_head = "".join(l for l in deps_head.splitlines(keepends=True) if not mentions(l, MOVED))
(NORM / "📦️packages/🦀️rust/Cargo.toml").write_text(deps_head + "[dev-dependencies]" + dev)

comp_toml = (NORM / "🧩️component/📦️packages/🦀️rust/Cargo.toml").read_text()
head, *rows = comp_toml.split("[[package.metadata.semio.playground]]")
tail_split = rows[-1].split("\n[lib]", 1)
rows[-1], lib = tail_split[0], "\n[lib]" + tail_split[1]
row_of = {re.search(r'variant = "(\w+)"', r).group(1): r for r in rows}
(NORM / "🧩️component/📦️packages/🦀️rust/Cargo.toml").write_text(head + "".join("[[package.metadata.semio.playground]]" + row_of[v] for v in row_of if v not in MOVED).rstrip("\n") + "\n" + lib)

for owner in ("norm-eurocode", "norm-eurocode-materials"):
    o = OWNERS[owner]
    base = NORM / "🧩️extensions" / o["dir"]
    (base / "📦️packages/🦀️rust").mkdir(parents=True, exist_ok=True)
    (base / "🧩️component/📦️packages/🦀️rust").mkdir(parents=True, exist_ok=True)
    (base / "🦀️.rs").write_text(extension_source(owner))
    fw = "../../../../../../../🧰️framework"
    deps = "".join(l for f in o["families"] for l in family_deps[f])
    (base / "📦️packages/🦀️rust/Cargo.toml").write_text(f"""[package]
workspace = "../../../../../.."
name = "{o['crate']}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "{o['label']} plugin — {o['summary']}"

[lints]
workspace = true

[package.metadata.semio]
role = "hub"

[lib]
path = "../../🦀️.rs"

[dependencies]
{deps}semio-framework-pack = {{ path = "{fw}/🔨️modules/🎒️pack/📦️packages/🦀️rust" }}
semio-ui-locale = {{ path = "{fw}/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust" }}
semio-framework = {{ workspace = true }}
semio-framework-value = {{ workspace = true }}
semio-framework-dispatch-macros = {{ workspace = true }}
semio-s-plugin-norm-app-surface = {{ workspace = true }}
semio-framework-job = {{ workspace = true }}
semio-os-kernel = {{ workspace = true }}
semio-plugin = {{ workspace = true }}
""")
    rel = f"🌎️hub/🧩️compositions/📕️norm/🧩️extensions/{o['dir']}"
    (base / "📦️packages/🦀️rust/📋️project.json").write_text(json.dumps({
        "name": f"@semio-tech/{owner}-plugin",
        "$schema": f"{fw}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json",
        "namedInputs": {"default": [f"{{workspaceRoot}}/{rel}/**/*.rs", "{projectRoot}/**/*"]},
        "targets": {"test": {"executor": "repo:script", "options": {"cwd": f"{rel}/📦️packages/🦀️rust", "command": "repo ./📜️script.ts test", "forwardAllArgs": True}, "outputs": [], "cache": True}},
    }, ensure_ascii=False, indent=2) + "\n")
    (base / "📦️packages/🦀️rust/📜️script.ts").write_text(f"""#!/usr/bin/env repo
import {{ receiveScriptProcessInvocation }} from "{fw}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {{ resolveTestLevel }} from "{fw}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** {o['emoji']} `@semio-tech/{owner}-plugin` router: `repo ./📜️script.ts test [quick|long|exhaustive]`. */
import {{ runRepositoryCargoTests }} from "{fw}/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import {{ BundleScript, ScriptRouter }} from "{fw}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {{ runScriptMain }} from "{fw}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

class TestScript extends BundleScript {{
  async run(segments: string[]): Promise<void> {{
    const {{ rest }} = resolveTestLevel(segments);
    await runRepositoryCargoTests(["{o['crate']}"], this.repoRoot, this.invocation, rest);
  }}
}}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, {{ invocation: original, defaultCommand: "test" }}));
""")
    lib_name = o["crate"].replace("-", "_")
    (base / "🧩️component/🦀️.rs").write_text(f"""//! {o['emoji']} The `{owner}` plugin component: the one cdylib of `{o['crate']}`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, {lib_name}::plugin, {lib_name}::{o['enum']});
""")
    playground = "".join("[[package.metadata.semio.playground]]" + row_of[f] for f in o["families"]).rstrip("\n") + "\n"
    (base / "🧩️component/📦️packages/🦀️rust/Cargo.toml").write_text(f"""[package]
workspace = "../../../../../../.."
name = "{o['crate']}-component"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "The `{owner}` plugin component: the one cdylib of `{o['crate']}`, holding only its entry point"

[lints]
workspace = true

[package.metadata.component]
package = "semio:{owner}"

[package.metadata.semio]
role = "hub"
deployment-directory = "{o['deploy']}"
component-kind = "plugin"

{playground}
[lib]
crate-type = ["cdylib"]
path = "../../🦀️.rs"

[dependencies]
semio-framework-pack = {{ path = "../{fw}/🔨️modules/🎒️pack/📦️packages/🦀️rust" }}
{o['crate']} = {{ path = "../../../📦️packages/🦀️rust" }}
semio-plugin = {{ workspace = true }}
""")

ws = (HUB / "Cargo.toml").read_text()
anchor = '    "🧩️compositions/📕️norm/🧩️component/📦️packages/🦀️rust",\n'
extra = "".join(f'    "🧩️compositions/📕️norm/🧩️extensions/{OWNERS[o]["dir"]}/📦️packages/🦀️rust",\n    "🧩️compositions/📕️norm/🧩️extensions/{OWNERS[o]["dir"]}/🧩️component/📦️packages/🦀️rust",\n' for o in ("norm-eurocode", "norm-eurocode-materials"))
if "📕️norm/🧩️extensions" not in ws:
    (HUB / "Cargo.toml").write_text(ws.replace(anchor, anchor + extra, 1))

tax_path = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
tax_text = tax_path.read_text()
for owner in ("norm-eurocode-materials", "norm-eurocode"):
    d = OWNERS[owner]["deploy"]
    if f'"{d}"' not in tax_text:
        tax_text = tax_text.replace('        "📕️norm",\n', f'        "📕️norm",\n        "{d}",\n', 1)
tax_path.write_text(tax_text)
print("split done")
