from pathlib import Path

fw = next(Path('/Users/ueli/Documents/semio').glob('*framework'))

plugin = fw / '🛍️products' / '💻️os' / '🔨️modules' / '🔌️plugin' / '🦀️component.rs'
text = plugin.read_text()
text = text.replace(
    'type Presence: Clone + Default + PartialEq + Serialize + DeserializeOwned + Send + store::DocumentDsl + DocumentPack = NoPresence;',
    'type Presence: Clone + Default + PartialEq + Serialize + DeserializeOwned + Send + store::DocumentDsl + DocumentPack;',
)
text = text.replace(
    'type PresenceMutation: ::protocol::Mutation<Self::Presence> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary = NoPresenceMutation;',
    'type PresenceMutation: ::protocol::Mutation<Self::Presence> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;',
)
text = text.replace(
    '        /// Defaulted so A3 can land the associated types before A4/A5 override per app; empty presence needs no boilerplate.\n',
    '',
)
plugin.write_text(text)
print('plugin defaults reverted; still has = NoPresence?', 'DocumentPack = NoPresence' in text)

cargo = fw / '🔨️modules' / '🧬️schema' / '📦️packages' / '🦀️rust' / 'Cargo.toml'
ct = cargo.read_text()
if 'catalog-integration' in ct:
    print('cargo already gated')
else:
    lines = ct.splitlines(keepends=True)
    header, other_dev, plugin_lines, after = [], [], [], []
    stage = 'header'
    for line in lines:
        if line.strip() == '[dev-dependencies]':
            stage = 'dev'
            continue
        if stage == 'header':
            header.append(line)
        elif stage == 'dev':
            if line.startswith('['):
                stage = 'after'
                after.append(line)
            elif line.startswith('semio-s-plugin-'):
                plugin_lines.append(line)
            else:
                other_dev.append(line)
        else:
            after.append(line)
    feature_deps = []
    new_plugin_lines = []
    for pl in plugin_lines:
        name = pl.split('=')[0].strip()
        feature_deps.append('  "dep:%s",' % name)
        if 'optional' in pl:
            new_plugin_lines.append(pl)
        elif pl.rstrip().endswith('}'):
            new_plugin_lines.append(pl.rstrip()[:-1] + ', optional = true }\n')
        else:
            new_plugin_lines.append(pl)
    features_block = '[features]\ndefault = []\ncatalog-integration = [\n' + '\n'.join(feature_deps) + '\n]\n\n'
    new_header = []
    inserted = False
    for line in header:
        if line.strip() == '[dependencies]' and not inserted:
            new_header.append(features_block)
            inserted = True
        new_header.append(line)
    if not inserted:
        new_header.append('\n' + features_block)
    new_ct = ''.join(new_header) + '[dev-dependencies]\n' + ''.join(other_dev) + ''.join(new_plugin_lines) + ''.join(after)
    cargo.write_text(new_ct)
    print('cargo.toml updated,', len(plugin_lines), 'plugins optional')

schema = fw / '🔨️modules' / '🧬️schema' / '🦀️component.rs'
st = schema.read_text()
start = st.find('    //#region 🔖️CatalogIntegration')
end = st.find('    //#endregion 🔖️CatalogIntegration')
if start < 0 or end < 0:
    raise SystemExit('catalog region missing')
region = st[start:end]
if '#[cfg(feature = "catalog-integration")]' in region:
    print('schema tests already cfg-gated')
else:
    region2 = region.replace('\n    #[test]\n', '\n    #[cfg(feature = "catalog-integration")]\n    #[test]\n')
    region2 = region2.replace(
        '    fn register_all_plugin_artifact_schema_descriptors() {',
        '    #[cfg(feature = "catalog-integration")]\n    fn register_all_plugin_artifact_schema_descriptors() {',
        1,
    )
    for helper in (
        '    fn validate_registered_artifact_descriptor(',
        '    fn register_and_assert_catalog_leaf_coverage(',
    ):
        gated = '    #[cfg(feature = "catalog-integration")]\n' + helper
        if helper in region2 and gated not in region2:
            region2 = region2.replace(helper, gated, 1)
    st = st[:start] + region2 + st[end:]
    schema.write_text(st)
    print('schema CatalogIntegration feature-gated')

print('DONE')
