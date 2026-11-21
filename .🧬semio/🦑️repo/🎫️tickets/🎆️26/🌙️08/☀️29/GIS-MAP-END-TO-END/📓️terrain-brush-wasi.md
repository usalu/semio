# Terrain Brush WASI Compilation

The managed build log traced the relocated terrain crate's repeated derive errors to three sources. The leaf had no sibling mutation descriptor, the sparse brush delta lacked `RetireOwned`, and that compilation used a Surface terrain crate without the brush action exports. The live Surface source exports both action types today.

The relocated crate owns production IO, terrain window configuration and viewer modules re-exported by the terrain facade and editor. Its declarations therefore stay on the production graph.

The leaf now has the required descriptor and sends a field-targeted rejection for brush settings outside the schema bounds. The sparse diff's brush payload derives bounded retirement. The shared brush settings schema keeps its existing identifier under the configuration schema facet. Both brush documents are discoverable through the DSL schema index, so the leaf carries the referenced settings alongside the mutation payload schema.

The Terrain window contract passed with Ajv for six accepted and nine rejected configurations. The same fixture covers the brush payload schema independently, including omitted and null settings and invalid limits. A focused projection check confirmed the source owner and both index entries. The Rust gesture test now checks that the leaf publishes its brush schema and that an invalid mutation refuses admission with an empty diff. Rust syntax and whitespace checks passed. The active managed WASI build owns full Rust verification.
