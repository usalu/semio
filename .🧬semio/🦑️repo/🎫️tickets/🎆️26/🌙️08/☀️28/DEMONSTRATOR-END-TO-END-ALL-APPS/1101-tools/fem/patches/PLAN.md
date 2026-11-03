# fem 3d patches to apply after the activation freeze (from fem-27.txt)
3d `editor-panels` (10 errors, snapshot port: nodes/materials are SealedShared rows now):
- `🗿️artifact/🦀️.rs:238,245,294`: compare via `node.id.as_str() == id`, `material.id.as_str() == material_id`, `section.id.as_str() == ...`, `material.id.as_str() == solid.material_id.as_str()`.
- `🗿️artifact/🦀️.rs:275`: closure param type is lost once `nodes` is the persistent list; annotate `|node: &FemNode|`.
- `🔍️inspection/🦀️.rs:198,206`: `(node.id.as_str().to_owned(), ...)`, `(material.id.as_str().to_owned(), material.name.as_str().to_owned())`.
- `🔍️inspection/🦀️.rs:481,493`: `node.id.as_str() == id`, `material.id.as_str() == id`.
- `🔍️inspection/🦀️.rs:482,494`: `node_rows`/`material_rows` take `&FemNodeValue`/`&FemMaterialValue`; change them to take the persistent `&FemNode`/`&FemMaterial` (read fields via `as_str()`), or convert via the existing `to_reference` row conversion.
2d `editor` (1 error): `🎛️play-app-editor/🦀️.rs:69` `FemGumballTransientMutation` lacks `ArtifactCanonicalJsonTree` (needed by `bounded_transient_preparation_factory`); derive/implement it as the 3d equivalent does.
