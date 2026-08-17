import { readFileSync, writeFileSync } from "fs";

const glue = readFileSync("/tmp/plugin-glue-path.txt", "utf8").trim();
const plugin = readFileSync("/tmp/plugin-comp-path.txt", "utf8").trim();

let g = readFileSync(glue, "utf8");
if (!g.includes("feature(linkage)")) {
  g = `#![cfg_attr(feature = "component-guest", feature(linkage))]\n` + g;
  writeFileSync(glue, g);
  console.log("glue: added linkage feature");
} else {
  console.log("glue: linkage already present");
}

let s = readFileSync(plugin, "utf8");
const old = `    #[cfg(feature = "component-guest")]
    extern "C" {
        fn semio_plugin_bundle_installer_link_shim();
    }

    /// Ensures the embedding plugin crate's bundle installer ran before any WIT export is served.`;
const neu = `    /// 🔗️ Weak default so intermediate \`cdylib\` links (e.g. \`semio-framework-os\` pulled into a
    /// wasip2 plugin build via feature unification of \`component-guest\`) succeed; the embedding
    /// plugin's \`plugin_exports!\` / \`semio_plugin!\` provides the strong installer override.
    #[cfg(feature = "component-guest")]
    #[unsafe(no_mangle)]
    #[linkage = "weak"]
    pub extern "C" fn semio_plugin_bundle_installer_link_shim() {}

    /// Ensures the embedding plugin crate's bundle installer ran before any WIT export is served.`;
if (!s.includes(old)) {
  console.error("extern block not found");
  const i = s.indexOf("semio_plugin_bundle_installer_link_shim");
  console.log(JSON.stringify(s.slice(Math.max(0, i - 200), i + 250)));
  process.exit(1);
}
s = s.replace(old, neu);
writeFileSync(plugin, s);
console.log("plugin: weak shim installed");
