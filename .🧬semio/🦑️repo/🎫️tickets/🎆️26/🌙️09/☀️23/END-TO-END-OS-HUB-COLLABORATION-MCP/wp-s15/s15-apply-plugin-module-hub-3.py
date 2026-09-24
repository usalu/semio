"""🧩️ S15: the atomic-case law restores plugin modules with its descriptors, and the check-in process fixture and
publication CLI relocate a generation's plugin modules (compiled-dependencies `publicationFiles` + `publicationDirectories`)."""
import sys
HUB = "/Users/ueli/Documents/semio/🌎️hub"

def edit(path, pairs):
    s = open(path, encoding="utf-8").read()
    for old, new in pairs:
        n = s.count(old)
        if n != 1:
            sys.exit(f"{path}: anchor count {n}: {old[:160]!r}")
        s = s.replace(old, new)
    open(path, "w", encoding="utf-8").write(s)

edit(f"{HUB}/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs", [
    ("""            for (path, bytes) in &originals {
                std::fs::write(path, bytes).expect("restore exact descriptor bytes");
            }""", """            for (index, (path, bytes)) in originals.iter().enumerate() {
                std::fs::write(path, bytes).expect("restore exact descriptor bytes");
                attach_fixture_plugin_module(&fixture.root, &mut fixture.bundle["packages"][index], bytes);
            }"""),
])

FILES = '["component.wasm", "descriptor.semio", "closed-actor.mjs", "plugin-module-gis.json", "stdio-component.wasm", "stdio-descriptor.semio", "plugin-module-stdio.json", "trusted-catalog.json"]'
OLD_FILES = '["component.wasm", "descriptor.semio", "closed-actor.mjs", "stdio-component.wasm", "stdio-descriptor.semio", "trusted-catalog.json"]'

edit(f"{HUB}/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🔗️compiled-dependencies/🔣️.json", [
    (f'"publicationFiles": {OLD_FILES.replace(", ", ", ")},', f'"publicationFiles": {FILES},\n  "publicationDirectories": ["plugin-modules"],'),
])

edit(f"{HUB}/🧪️tests/🔬️bin-unit/🦀️.rs", [
    (f"""    for name in {OLD_FILES} {{
        std::fs::copy(source.join(name), generation.join(name)).unwrap_or_else(|error| panic!("copy verified {{name}}: {{error}}"));
    }}""", f"""    for name in {FILES} {{
        std::fs::copy(source.join(name), generation.join(name)).unwrap_or_else(|error| panic!("copy verified {{name}}: {{error}}"));
    }}
    for directory in ["plugin-modules"] {{
        std::fs::create_dir_all(generation.join(directory)).expect("create verified plugin module store");
        for entry in std::fs::read_dir(source.join(directory)).expect("verified plugin module store") {{
            let entry = entry.expect("verified plugin module file");
            std::fs::copy(entry.path(), generation.join(directory).join(entry.file_name())).expect("copy verified plugin module file");
        }}
    }}"""),
    ("""    let expected_files: std::collections::BTreeSet<_> = dependency_fixture["publicationFiles"].as_array().unwrap().iter().map(|name| name.as_str().unwrap().to_owned()).collect();""",
     """    let expected_files: std::collections::BTreeSet<_> = ["publicationFiles", "publicationDirectories"].iter().flat_map(|key| dependency_fixture[*key].as_array().unwrap().iter()).map(|name| name.as_str().unwrap().to_owned()).collect();"""),
])

edit(f"{HUB}/📦️packages/🦀️rust/📜️script.ts", [
    ('''  assert.deepEqual(Object.keys(fixture), ["schema", "publicationFiles", "descriptorPreviewCases", "selectedClosure", "cases", "consumerCatalogCases", "atomicCases", "nativeCases", "rawCases", "encodingCases", "ordering"]);''',
     '''  assert.deepEqual(Object.keys(fixture), ["schema", "publicationFiles", "publicationDirectories", "descriptorPreviewCases", "selectedClosure", "cases", "consumerCatalogCases", "atomicCases", "nativeCases", "rawCases", "encodingCases", "ordering"]);'''),
    (f'''  assert.deepEqual(fixture.publicationFiles, {OLD_FILES});''', f'''  assert.deepEqual(fixture.publicationFiles, {FILES});
  assert.deepEqual(fixture.publicationDirectories, ["plugin-modules"]);'''),
    ('''  assert.deepEqual(JSON.parse(cliCopy.match(/for \\(const file of (\\[[^\\]\\n]+\\])/u)?.[1] ?? "null"), fixture.publicationFiles, "publication CLI must retain every dependency file");''',
     '''  assert.deepEqual(JSON.parse(cliCopy.match(/for \\(const file of (\\[[^\\]\\n]+\\])/u)?.[1] ?? "null"), fixture.publicationFiles, "publication CLI must retain every dependency file");
  assert.deepEqual(JSON.parse(fixtureCopy.match(/for directory in (\\[[^\\]\\n]+\\])/u)?.[1] ?? "null"), fixture.publicationDirectories, "native process fixture must retain every plugin module file");
  assert.deepEqual(JSON.parse(cliCopy.match(/for \\(const directory of (\\[[^\\]\\n]+\\])/u)?.[1] ?? "null"), fixture.publicationDirectories, "publication CLI must retain every plugin module file");'''),
    (f'''  for (const file of {OLD_FILES}) {{
    const bytes = trustedBootstrapReadRegular(join(fixtureData, "trusted-catalog/generations", fixtureCurrent.pointer.generationId, file), file.endsWith(".wasm") ? DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES : 4 * 1024 * 1024, `publication CLI fixture ${{file}}`, () => {{}});
    try {{ trustedBootstrapWriteNew(join(generationRoot, file), bytes, () => {{}}); }} finally {{ bytes.fill(0); }}
  }}''', f'''  for (const file of {FILES}) {{
    const bytes = trustedBootstrapReadRegular(join(fixtureData, "trusted-catalog/generations", fixtureCurrent.pointer.generationId, file), file.endsWith(".wasm") ? DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES : 4 * 1024 * 1024, `publication CLI fixture ${{file}}`, () => {{}});
    try {{ trustedBootstrapWriteNew(join(generationRoot, file), bytes, () => {{}}); }} finally {{ bytes.fill(0); }}
  }}
  for (const directory of ["plugin-modules"]) {{
    mkdirSync(join(generationRoot, directory), {{ recursive: true }});
    for (const name of readdirSync(join(fixtureData, "trusted-catalog/generations", fixtureCurrent.pointer.generationId, directory))) {{
      const bytes = trustedBootstrapReadRegular(join(fixtureData, "trusted-catalog/generations", fixtureCurrent.pointer.generationId, directory, name), TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES, `publication CLI fixture ${{directory}}/${{name}}`, () => {{}});
      try {{ trustedBootstrapWriteNew(join(generationRoot, directory, name), bytes, () => {{}}); }} finally {{ bytes.fill(0); }}
    }}
  }}'''),
])
print("hub step 3 wired")
