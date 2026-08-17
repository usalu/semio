#!/usr/bin/env bun
import { readFileSync, writeFileSync, mkdirSync, existsSync, readdirSync } from "fs";
import { join } from "path";

const PLUGINS_ROOT = "✏️s/🔌️plugins";
const PLUGIN_DIR = "🔌️plugin";
const CHILDREN = ["🛂️manifest", "🎟️capabilities", "🔧️setup", "🎛️apps"];
const COMPONENT = "🦀️component.rs";

function extractSemioPlugin(text) {
  const m = text.match(/semio_framework_plugin::semio_plugin!\s*\{/);
  if (!m) return null;
  const start = m.index;
  const brace = text.indexOf("{", start);
  let depth = 0;
  for (let i = brace; i < text.length; i++) {
    if (text[i] === "{") depth++;
    else if (text[i] === "}") {
      depth--;
      if (depth === 0) return { start, end: i + 1, body: text.slice(brace + 1, i) };
    }
  }
  return null;
}

function parseMacroBody(body) {
  const id = body.match(/\bid:\s*"([^"]+)"/)?.[1];
  const label = body.match(/\blabel:\s*"([^"]+)"/)?.[1];
  const version = body.match(/\bversion:\s*"([^"]+)"/)?.[1];
  const setup = body.match(/\bsetup:\s*([A-Za-z0-9_:]+)/)?.[1];
  const appsMatch = body.match(/\bapps:\s*\[([\s\S]*?)\]/);
  const apps = [];
  if (appsMatch) {
    for (const part of appsMatch[1].split(",")) {
      const mm = part.match(/([A-Za-z0-9_:]+)\s*=>\s*([A-Za-z0-9_:]+)/);
      if (mm) apps.push({ create: mm[1].trim(), ty: mm[2].trim() });
    }
  }
  if (!id || !label || !version || !setup || apps.length === 0) return null;
  return { id, label, version, setup, apps };
}

function cratePath(pathExpr) {
  return pathExpr.startsWith("crate::") ? pathExpr : `crate::${pathExpr}`;
}

function stubChild(kind, pluginName) {
  const docs = {
    "🛂️manifest": `//! 🛂️ Manifest facet for \`${pluginName}\` — identity surfaces live on \`Plugin::builder\` in the parent.`,
    "🎟️capabilities": `//! 🎟️ Capabilities facet for \`${pluginName}\` — declare rights via \`PluginBuilder::capability\` / \`.local_backbone_storage()\`.`,
    "🔧️setup": `//! 🔧️ Setup facet for \`${pluginName}\` — codec/language/importer registration hooked via \`.setup(...)\`.`,
    "🎛️apps": `//! 🎛️ Apps facet for \`${pluginName}\` — document app factories registered via \`.register_document_app\`.\n`,
  };
  return `${docs[kind] || `//! ${kind}\n`}\n`;
}

function buildPluginRs(meta) {
  const apps = meta.apps
    .map(
      (a) =>
        `        .register_document_app::<${cratePath(a.ty)}>(${cratePath(a.create)}())`,
    )
    .join("\n");
  return `//! 🔌️ Plugin root contract — typestate \`Plugin::builder\` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder(${JSON.stringify(meta.id)})
        .label(${JSON.stringify(meta.label)})
        .version(${JSON.stringify(meta.version)})
        .setup(${cratePath(meta.setup)})
${apps}
        .build()
}
`;
}

function ensureChildren(pluginRoot, pluginName) {
  for (const child of CHILDREN) {
    const dir = join(pluginRoot, child);
    mkdirSync(dir, { recursive: true });
    const leaf = join(dir, COMPONENT);
    if (!existsSync(leaf)) writeFileSync(leaf, stubChild(child, pluginName));
  }
}

const report = [];
for (const entry of readdirSync(PLUGINS_ROOT, { withFileTypes: true })) {
  if (!entry.isDirectory()) continue;
  const pluginName = entry.name;
  const owner = join(PLUGINS_ROOT, pluginName);
  const glue = join(owner, "📦️packages/🦀️rust/📦️glue.rs");
  if (!existsSync(glue)) {
    report.push({ plugin: pluginName, status: "skip-no-glue" });
    continue;
  }
  const pluginRoot = join(owner, PLUGIN_DIR);
  mkdirSync(pluginRoot, { recursive: true });
  ensureChildren(pluginRoot, pluginName);
  const leaf = join(pluginRoot, COMPONENT);
  let text = readFileSync(glue, "utf8");
  const extracted = extractSemioPlugin(text);

  if (pluginName.includes("space") || pluginName === "🪐️space") {
    // Move fn bundle into plugin root
    const m = text.match(/fn bundle\(\)[\s\S]*?\n\}/);
    if (m && !existsSync(leaf)) {
      const body = m[0]
        .replace(/^fn bundle/, "pub fn plugin")
        .replace(/Plugin::new\(/, "Plugin::builder(")
        .replace(
          /Plugin::builder\(([^)]+)\)\s*\n\s*\.local_backbone_storage\(\)/,
          (full, idEtc) => {
            // Plugin::new("s", "S Studio", "0.1.0") style already replaced badly
            return full;
          },
        );
      // Parse Plugin::new("s", "S Studio", "0.1.0")
      const nm = m[0].match(/Plugin::new\(([^)]+)\)/);
      const args = nm ? nm[1].split(",").map((s) => s.trim()) : null;
      let pluginFn;
      if (args && args.length >= 3) {
        const rest = m[0].slice(m[0].indexOf(")") + 1, m[0].lastIndexOf("}") + 1);
        // rest starts with chain ending with }
        const chain = rest.replace(/^\s*/, "").replace(/\}\s*$/, "").trim();
        pluginFn = `//! 🔌️ Plugin root contract for space (multi-app host).

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the S Studio plugin (home + space apps).
pub fn plugin() -> Plugin {
    crate::register_s_exports();
    Plugin::builder(${args[0]})
        .label(${args[1]})
        .version(${args[2]})
        ${chain.replace(/^\./, ".")}
}
`;
        // chain already has .local_backbone... ending without extra }
        // Fix: chain from original after Plugin::new(...) 
        const chainOnly = m[0].split(/Plugin::new\([^)]*\)/)[1].replace(/\}\s*$/, "").trim();
        pluginFn = `//! 🔌️ Plugin root contract for space (multi-app host).

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the S Studio plugin (home + space apps).
pub fn plugin() -> Plugin {
    crate::register_s_exports();
    Plugin::builder(${args[0]})
        .label(${args[1]})
        .version(${args[2]})
${chainOnly.split("\n").map((l) => "        " + l.trim()).filter((l) => l.trim()).join("\n")}
}
`;
      } else {
        pluginFn = `//! 🔌️ Plugin root contract for space.\nuse semio_framework_plugin::Plugin;\n${m[0].replace("fn bundle", "pub fn plugin")}\n`;
      }
      writeFileSync(leaf, pluginFn);
      text = text.replace(m[0], "");
      text = text.replace(
        /semio_framework_plugin::plugin_exports!\(bundle\);/,
        `#[path = "../../${PLUGIN_DIR}/${COMPONENT}"]\nmod plugin;\nsemio_framework_plugin::plugin_exports!(plugin::plugin);`,
      );
      // cleanup empty region
      writeFileSync(glue, text);
      report.push({ plugin: pluginName, status: "space-migrated" });
      continue;
    }
  }

  if (pluginName === "🎪️demonstrator" || pluginName.includes("demonstrator")) {
    if (!existsSync(leaf)) {
      writeFileSync(
        leaf,
        `//! 🔌️ Plugin root contract for the demonstrator multi-pane bundle.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the demonstrator plugin via pane registration.
pub fn plugin() -> Plugin {
    crate::panes::bundle()
}
`,
      );
    }
    if (text.includes("plugin_exports!(panes::bundle)")) {
      text = text.replace(
        /semio_framework_plugin::plugin_exports!\(panes::bundle\);/,
        `#[path = "../../${PLUGIN_DIR}/${COMPONENT}"]\nmod plugin;\nsemio_framework_plugin::plugin_exports!(plugin::plugin);`,
      );
      writeFileSync(glue, text);
    }
    report.push({ plugin: pluginName, status: "demonstrator-migrated" });
    continue;
  }

  if (pluginName === "🔋️energy" || pluginName.includes("energy")) {
    if (!existsSync(leaf)) {
      writeFileSync(
        leaf,
        `//! 🔌️ Plugin root contract for the headless energy library.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the energy library plugin (no document apps).
pub fn plugin() -> Plugin {
    Plugin::builder("energy")
        .label("Energy")
        .version("0.1.0")
        .library()
}
`,
      );
    }
    if (!text.includes("plugin_exports!(plugin::plugin)") && !text.includes(`${PLUGIN_DIR}/${COMPONENT}`)) {
      text += `\n\n//#region 🔖️Plugin\n#[path = "../../${PLUGIN_DIR}/${COMPONENT}"]\nmod plugin;\nsemio_framework_plugin::plugin_exports!(plugin::plugin);\n//#endregion 🔖️Plugin\n`;
      writeFileSync(glue, text);
    }
    report.push({ plugin: pluginName, status: "energy-migrated" });
    continue;
  }

  if (extracted) {
    const meta = parseMacroBody(extracted.body);
    if (!meta) {
      report.push({ plugin: pluginName, status: "parse-failed", body: extracted.body.slice(0, 200) });
      continue;
    }
    writeFileSync(leaf, buildPluginRs(meta));
    const replacement = `//#region 🔖️Plugin
#[path = "../../${PLUGIN_DIR}/${COMPONENT}"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);
//#endregion 🔖️Plugin`;
    // Replace from region start if present
    if (text.includes("//#region 🔖️Plugin")) {
      text = text.replace(/\/\/#region 🔖️Plugin[\s\S]*?\/\/#endregion 🔖️Plugin/, replacement);
    } else {
      text = text.slice(0, extracted.start) + replacement + text.slice(extracted.end);
    }
    writeFileSync(glue, text);
    report.push({ plugin: pluginName, status: "macro-migrated", id: meta.id, apps: meta.apps.length });
  } else {
    // ensure leaf exists somehow
    if (!existsSync(leaf)) {
      writeFileSync(
        leaf,
        `//! 🔌️ Plugin root contract for \`${pluginName}\`.\n\nuse semio_framework_plugin::Plugin;\n\n/// 🔌️ Builds the plugin surface.\npub fn plugin() -> Plugin {\n    Plugin::builder("TODO").label("TODO").version("0.1.0").build()\n}\n`,
      );
      report.push({ plugin: pluginName, status: "stub-only" });
    } else {
      report.push({ plugin: pluginName, status: "already-had-leaf" });
    }
  }
}

console.log(JSON.stringify(report, null, 2));
writeFileSync(process.argv[2] || "wave3-report.json", JSON.stringify(report, null, 2));
