"""🧩️ S15: the plugin module index names every app dialect artifact kind of a package (`dialectArtifactKinds`), so a shell
that never built a plugin can find the hub package that opens a kind. Schema, Rust, TS twin, fixture generator, laws."""
import sys
ROOT = "/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog"
OS = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os"
def edit(path, pairs):
    s = open(path, encoding="utf-8").read()
    for old, new in pairs:
        n = s.count(old)
        if n != 1: sys.exit(f"{path}: anchor count {n}: {old[:140]!r}")
        s = s.replace(old, new)
    open(path, "w", encoding="utf-8").write(s)

edit(f"{ROOT}/🧬️schema/🔣️.json", [
    ('"required": ["pluginId", "packageId", "version", "componentSha256", "descriptorByteSha256", "dependencies", "bundleSha256", "bundleByteLength", "entry"],',
     '"required": ["pluginId", "packageId", "version", "componentSha256", "descriptorByteSha256", "dependencies", "dialectArtifactKinds", "bundleSha256", "bundleByteLength", "entry"],'),
    ('''        "dependencies": { "type": "array", "maxItems": 256, "uniqueItems": true, "items": { "$ref": "#/$defs/identity" } },
        "bundleSha256": { "$ref": "#/$defs/digest" },''', '''        "dependencies": { "type": "array", "maxItems": 256, "uniqueItems": true, "items": { "$ref": "#/$defs/identity" } },
        "dialectArtifactKinds": { "type": "array", "maxItems": 1024, "uniqueItems": true, "items": { "$ref": "#/$defs/identity" } },
        "bundleSha256": { "$ref": "#/$defs/digest" },'''),
])
edit(f"{ROOT}/🧬️schema/🦀️.rs", [
    ('''    pub dependencies: Vec<String>,
    pub bundle_sha256: String,''', '''    pub dependencies: Vec<String>,
    /// 🎭️ Every app dialect artifact kind the package's surfaces open, in ascending byte order: how a shell that never
    /// built the plugin finds the package that opens a kind.
    pub dialect_artifact_kinds: Vec<String>,
    pub bundle_sha256: String,'''),
])
edit(f"{ROOT}/🧩️plugin-module/🦀️.rs", [
    ('''/// 🧯️ Most segments of one plugin-module-relative path.''', '''/// 🧯️ Most app dialect artifact kinds one index entry may name.
pub const TRUSTED_PLUGIN_MODULE_INDEX_MAX_DIALECT_KINDS: usize = 1024;
/// 🧯️ Most segments of one plugin-module-relative path.'''),
    ('''            || !entry.dependencies.iter().all(|dependency| valid_identity(dependency) && dependencies.insert(dependency.as_str()))
        {''', '''            || !entry.dependencies.iter().all(|dependency| valid_identity(dependency) && dependencies.insert(dependency.as_str()))
            || entry.dialect_artifact_kinds.len() > TRUSTED_PLUGIN_MODULE_INDEX_MAX_DIALECT_KINDS
            || !entry.dialect_artifact_kinds.iter().all(|kind| valid_identity(kind))
            || entry.dialect_artifact_kinds.windows(2).any(|pair| pair[0].as_bytes() >= pair[1].as_bytes())
        {'''),
])
edit(f"{ROOT}/🦀️.rs", [
    ('''                dependencies: package.dependencies.clone(),
                bundle_sha256: package.plugin_module.bundle_sha256.clone(),''', '''                dependencies: package.dependencies.clone(),
                dialect_artifact_kinds: package.descriptor.manifest.apps.iter().map(|app| app.dialect.artifact_kind.clone()).collect::<BTreeSet<_>>().into_iter().collect(),
                bundle_sha256: package.plugin_module.bundle_sha256.clone(),'''),
])
edit(f"{ROOT}/🧪️tests/🔬️unit/🦀️.rs", [
    ('''    assert_eq!(index.modules[1].dependencies, vec!["fixture.base".to_owned()]);''', '''    assert_eq!(index.modules[1].dependencies, vec!["fixture.base".to_owned()]);
    assert_eq!(index.modules[1].dialect_artifact_kinds, vec!["s.fixture.document".to_owned()]);
    assert!(index.modules[0].dialect_artifact_kinds.is_empty(), "a package without apps opens no dialect");'''),
])
edit(f"{OS}/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧬️schema/🟦️.ts", [
    ('''  descriptorByteSha256: string;
  dependencies: readonly string[];
  bundleSha256: string;''', '''  descriptorByteSha256: string;
  dependencies: readonly string[];
  dialectArtifactKinds: readonly string[];
  bundleSha256: string;'''),
    ('''const DEPENDENCIES_MAX: number = indexEntryProperties.dependencies.maxItems;''', '''const DEPENDENCIES_MAX: number = indexEntryProperties.dependencies.maxItems;
const DIALECT_KINDS_MAX: number = indexEntryProperties.dialectArtifactKinds.maxItems;'''),
    ('''    const dependencies = entry.dependencies;
    if (''', '''    const dependencies = entry.dependencies;
    const dialects = entry.dialectArtifactKinds;
    if ('''),
    ('''      new Set(dependencies).size !== dependencies.length
    )''', '''      new Set(dependencies).size !== dependencies.length ||
      !Array.isArray(dialects) ||
      dialects.length > DIALECT_KINDS_MAX ||
      !dialects.every(validIdentity) ||
      dialects.some((kind, index) => index > 0 && utf8OrderV1(dialects[index - 1] as string, kind as string) >= 0)
    )'''),
    ('''      dependencies: Object.freeze([...(dependencies as string[])]),
      bundleSha256: entry.bundleSha256,''', '''      dependencies: Object.freeze([...(dependencies as string[])]),
      dialectArtifactKinds: Object.freeze([...(dialects as string[])]),
      bundleSha256: entry.bundleSha256,'''),
])
edit("/Users/ueli/Documents/semio/.tmp-ticket/wp-s15/s15-plugin-module-fixture.ts", [
    ('''descriptorByteSha256: record.descriptorByteSha256, dependencies: ["stdio"], bundleSha256''', '''descriptorByteSha256: record.descriptorByteSha256, dependencies: ["stdio"], dialectArtifactKinds: ["s.note.note"], bundleSha256'''),
    ('''modules: [{ ...entry, pluginId: "draw", packageId: "semio:draw", entry: "🖍️draw/🌉️bridge.js", dependencies: [] }, entry] };''', '''modules: [{ ...entry, pluginId: "draw", packageId: "semio:draw", entry: "🖍️draw/🌉️bridge.js", dependencies: [], dialectArtifactKinds: ["s.draw.drawing", "s.draw.symbol"] }, entry] };'''),
    ('''  indexEdit("duplicate-dependency", false, false, (i) => { i.modules[1].dependencies = ["stdio", "stdio"]; }),''', '''  indexEdit("duplicate-dependency", false, false, (i) => { i.modules[1].dependencies = ["stdio", "stdio"]; }),
  indexEdit("missing-dialects", false, false, (i) => { delete i.modules[1].dialectArtifactKinds; }),
  indexEdit("duplicate-dialect", false, false, (i) => { i.modules[0].dialectArtifactKinds = ["s.draw.drawing", "s.draw.drawing"]; }),
  indexEdit("unsorted-dialects", true, false, (i) => { i.modules[0].dialectArtifactKinds = ["s.draw.symbol", "s.draw.drawing"]; }),'''),
])
print("dialect kinds wired")
