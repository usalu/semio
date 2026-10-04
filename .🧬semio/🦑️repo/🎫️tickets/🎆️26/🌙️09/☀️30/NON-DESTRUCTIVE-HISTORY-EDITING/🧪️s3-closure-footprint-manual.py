"""🧺️ S3-CLOSURE step 4: the hand-shaped footprint sites the mechanical sweep cannot rewrite (design §20.5)."""
import re, sys
ROOT = '/Users/ueli/Documents/semio/'
S = '✏️s/🔌️plugins/'
ANY = '/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs'
EDITS = {}
def edit(path, old, new, count=1):
    EDITS.setdefault(path, []).append((old, new, count))
def regex(path, pattern, new, count=1):
    EDITS.setdefault(path, []).append((re.compile(pattern, re.S), new, count))

FEM2D = S + '🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs'
edit(FEM2D, '''    /// 🧺️ One forward row plus its inverse rows: one per record a `move-selection` restores — the upper bound its
    /// payload proves without the base — and exactly one for every other kind.
    fn preflight(&self, mutation: &Fem2dMutation''', '''    /// 🧺️ One forward row plus the inverse rows the leaf's payload schema declares (`x-semio-inverse-rows`: one per node
    /// and region a `move-selection` restores, one for every other kind).
    fn preflight(&self, mutation: &Fem2dMutation''')
edit(FEM2D, '''        Ok(match mutation {
            Fem2dMutation::MoveSelection(leaf) => store::ArtifactStoreOneItemFootprint::for_one_item(leaf.node_ids.len() + leaf.region_ids.len(), store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES),
            _ => store::ArtifactStoreOneItemFootprint::for_one_invertible_item(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES),
        })''', '''        Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))''')
FEM3D = S + '🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs'
edit(FEM3D, '''    /// 🧺️ One forward row plus its inverse rows: one per record a `move-selection` restores — the upper bound its
    /// payload proves without the base — and exactly one for every other kind.
    fn preflight(&self, mutation: &Fem3dMutation''', '''    /// 🧺️ One forward row plus the inverse rows the leaf's payload schema declares (`x-semio-inverse-rows`: one per node
    /// and solid a `move-selection` restores, one for every other kind).
    fn preflight(&self, mutation: &Fem3dMutation''')
edit(FEM3D, '''        let footprint = admit_fem3d_artifact_mutation(mutation)?;
        Ok(match mutation {
            Fem3dMutation::MoveSelection(leaf) => store::ArtifactStoreOneItemFootprint::for_one_item(leaf.node_ids.len() + leaf.solid_ids.len(), footprint.retained_bytes),
            _ => footprint,
        })''', '''        admit_fem3d_artifact_mutation(mutation)''')

P5 = S + '🧩️puzzle/🗿️artifacts/🖐️5d' + ANY
edit(P5, '''/// 🪢️ Inverse rows a removal may restore: the record itself plus the fasteners it severs, bounded like puzzle 2d's
/// edges per node.
const PUZZLE5D_REMOVAL_INVERSE_ROWS: usize = 1 + 64;

''', '')
regex(P5, r'''    /// 🧾️ `work_items` counts staged edit ROWS — the forward row plus every row the inverse yields: a selection leaf
    /// restores up to one setter per changed pose field of each target \(a part's board position, world origin,
    /// orientation, scale\), a removal restores the record and the fasteners it severed, every other kind is
    /// point-invertible\.
(    fn preflight\(&self, mutation: &Puzzle5dMutation.*?)        let retained = store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
        Ok\(match mutation \{.*?        \}\)
''', r'''    /// 🧾️ The forward row plus the inverse rows the leaf's payload schema declares (`x-semio-inverse-rows`): a selection
    /// leaf one setter per changed pose field of each target, a removal the record and the fasteners it severs.
\1        Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
''')
P3 = S + '🧩️puzzle/🗿️artifacts/🧊️3d' + ANY
edit(P3, '''/// 🧾️ Inverse rows a selection leaf may restore: every object its attraction re-solve carries along and every
/// attraction it re-derives is one absolute setter per changed field, a count only the base knows. `preflight`
/// never sees the base, so it declares as many rows as the one-item byte budget can retain.
const PUZZLE3D_SELECTION_INVERSE_ROWS: usize = store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES / std::mem::size_of::<Puzzle3dMutation>();

/// 🪢️ Inverse rows a removal may restore: the record itself plus the attractions it severs, bounded like puzzle 2d's
/// edges per node.
const PUZZLE3D_REMOVAL_INVERSE_ROWS: usize = 1 + 64;

''', '')
regex(P3, r'''    /// 🧾️ `work_items` counts staged edit ROWS — the forward row plus every row the inverse yields: a selection
    /// leaf restores up to one setter per changed pose field of every record it moves \(scaling: one per target, no
    /// re-solve\), a removal restores the record and the attractions it severed, every other kind is point-invertible\.
(    fn preflight\(&self, mutation: &Puzzle3dMutation.*?)        let retained = store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
        Ok\(match mutation \{.*?        \}\)
''', r'''    /// 🧾️ The forward row plus the inverse rows the leaf's payload schema declares (`x-semio-inverse-rows`): a selection
    /// leaf one setter per changed pose field of every record its attraction re-solve carries along, a scaling one per
    /// target, a removal the record and the attractions it severs.
\1        Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
''')
P2 = S + '🧩️puzzle/🗿️artifacts/◻️2d' + ANY
edit(P2, '''/// 🧾️ Inverse rows one `delete-node` may yield: the re-created node plus one `connect-handles` per edge
/// on its handles — a node carries at most one edge per handle, and Nakagin's densest node has 12 handles.
const PUZZLE2D_DELETE_NODE_INVERSE_ROWS: usize = 1 + 64;
/// 🧾️ Inverse rows one `rotate-selection` target may yield: the restoring `move-node` plus one
/// `replace-node-handle` per turned handle, under the same per-node handle ceiling as a delete.
/// `drag-selection` restores one position per target and `scale-selection` a region's corner AND extent.
const PUZZLE2D_ROTATE_SELECTION_INVERSE_ROWS: usize = 1 + 64;
''', '')
regex(P2, r'''    /// 🧾️ `work_items` counts staged edit ROWS: the forward row plus every row the inverse yields\.
    /// `delete-node`'s inverse re-creates.*?bounded by the edges one node can carry\.
(    fn preflight\(&self, mutation: &Puzzle2dMutation.*?)        let work_items = match mutation \{.*?        \};
        Ok\(store::ArtifactStoreOneItemFootprint \{ work_items, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES \}\)
''', r'''    /// 🧾️ The forward row plus the inverse rows the leaf's payload schema declares (`x-semio-inverse-rows`): `delete-node`
    /// re-creates the node and re-connects every edge on its handles, a selection transform restores each target.
\1        Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
''')

CAD = S + '📐️cad/🗿️artifacts/📐️cad' + ANY
edit(CAD, '''/// 🧺️ `work_items` counts staged edit ROWS (forward + inverse), never mutations: every
/// `CadConfigMutation` inverts to exactly one `Snapshot` row, so a config gesture is ONE invertible
/// item (`ArtifactStoreOneItemFootprint::for_one_invertible_item`). Declaring `work_items: 1` here
/// fail-closed every config gesture (`setReferenceSelection`, `setCamera`, …) inside
/// `ArtifactStore::fold_batch_item` with `batched item candidate failed its exact fixed fold contract`
/// (ticket 26/09/15/DEV-CAD-REACT-E2E).
fn admit_cad_config(config: &CadConfig) -> Result<store::ArtifactStoreOneItemFootprint, String> {''', '''/// 🧺️ Admits a CAD config against its fixed retained item and byte envelope, answering its retained bytes.
fn admit_cad_config(config: &CadConfig) -> Result<usize, String> {''')
edit(CAD, '''        return Err("CAD config exceeds its fixed retained byte envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint::for_one_invertible_item(retained_bytes))
}''', '''        return Err("CAD config exceeds its fixed retained byte envelope".into());
    }
    Ok(retained_bytes)
}''')
edit(CAD, '''    match mutation {
        CadConfigMutation::Snapshot { config } => admit_cad_config(config),
        CadConfigMutation::SetContributions { json } if json.len() <= CAD_CONFIG_STORE_MAXIMUM_BYTES => Ok(store::ArtifactStoreOneItemFootprint::for_one_invertible_item(json.len())),
        CadConfigMutation::SetContributions { .. } => Err("CAD config mutation exceeds its fixed retained byte envelope".into()),
    }''', '''    let retained_bytes = match mutation {
        CadConfigMutation::Snapshot { config } => admit_cad_config(config)?,
        CadConfigMutation::SetContributions { json } if json.len() <= CAD_CONFIG_STORE_MAXIMUM_BYTES => json.len(),
        CadConfigMutation::SetContributions { .. } => return Err("CAD config mutation exceeds its fixed retained byte envelope".into()),
    };
    Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))''')
edit(CAD, '''/// 🧺️ Same fold contract as [`admit_cad_config`]: every `CadMutation` inverts to at most one row
/// (`🧬️mutations/*/↩️inverse`), so an artifact gesture is one invertible item — forward row plus
/// inverse row.
fn admit_cad_snapshot(snapshot: &CadSnapshot) -> Result<store::ArtifactStoreOneItemFootprint, String> {''', '''/// 🧺️ Admits a CAD Artifact against its fixed retained item and byte envelope, answering its retained bytes.
fn admit_cad_snapshot(snapshot: &CadSnapshot) -> Result<usize, String> {''')
edit(CAD, '''        return Err("CAD Artifact exceeds its fixed retained preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint::for_one_invertible_item(retained_bytes))
}''', '''        return Err("CAD Artifact exceeds its fixed retained preparation envelope".into());
    }
    Ok(retained_bytes)
}''')
CADT = S + '📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs'
edit(CADT, '''    // 🧺️ `work_items` counts staged ROWS: the forward plus every inverse row — declaring fewer fail-closes
    // the gesture in `ArtifactStore::fold_batch_item` (`batched item candidate failed its exact fixed fold contract`).
    assert_eq!(footprint.work_items, 1 + inverse.len(), "config footprint must cover forward + inverse rows");
    assert_eq!(footprint, store::ArtifactStoreOneItemFootprint::for_one_invertible_item(footprint.retained_bytes));''', '''    assert_eq!(footprint.work_items, 1 + inverse.len(), "config footprint must cover forward + inverse rows");
    assert_eq!(footprint, store::ArtifactStoreOneItemFootprint::for_leaf(&mutation, footprint.retained_bytes));''')
edit(CADT, '''    assert_eq!(footprint.work_items, 1 + inverse.len(), "artifact footprint must cover forward + inverse rows");
    assert_eq!(footprint, store::ArtifactStoreOneItemFootprint::for_one_invertible_item(footprint.retained_bytes));''', '''    assert_eq!(footprint.work_items, 1 + inverse.len(), "artifact footprint must cover forward + inverse rows");
    assert_eq!(footprint, store::ArtifactStoreOneItemFootprint::for_leaf(&mutation, footprint.retained_bytes));''')

PROC = S + '🏭️process/🗿️artifacts/🧊️process3d' + ANY
edit(PROC, '''/// 📏️ One semantic mutation's own retained footprint, shaped like what it actually addresses: a
/// single step, machine, stock field or cursor is one work item carrying that target's own text,
/// while a created machine or a replaced capability set is one item per capability leaf it carries.
fn process3d_mutation_footprint(mutation: &Process3dMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let (work_items, retained_bytes) = match mutation {''', '''/// 📏️ One semantic mutation's own retained bytes, shaped like what it actually addresses: a single step, machine,
/// stock field or cursor is one item carrying that target's own text, while a created machine or a replaced capability
/// set is one item per capability leaf it carries — the items bounded by `PROCESS3D_DOCUMENT_MAXIMUM_ITEMS`.
fn process3d_mutation_retained_bytes(mutation: &Process3dMutation) -> Result<usize, String> {
    let (items, retained_bytes) = match mutation {''')
edit(PROC, '''    if work_items > PROCESS3D_DOCUMENT_MAXIMUM_ITEMS || retained_bytes > PROCESS3D_DOCUMENT_MAXIMUM_BYTES {
        return Err("Process3d document mutation exceeds its fixed one-item preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items, retained_bytes })
}''', '''    if items > PROCESS3D_DOCUMENT_MAXIMUM_ITEMS || retained_bytes > PROCESS3D_DOCUMENT_MAXIMUM_BYTES {
        return Err("Process3d document mutation exceeds its fixed one-item preparation envelope".into());
    }
    Ok(retained_bytes)
}''')
edit(PROC, '    process3d_mutation_footprint(&mutation)?;', '    process3d_mutation_retained_bytes(&mutation)?;')
edit(PROC, '        process3d_mutation_footprint(mutation)?;', '        process3d_mutation_retained_bytes(mutation)?;')
edit(PROC, '            let bytes = process3d_mutation_footprint(mutation)?.retained_bytes;', '            let bytes = process3d_mutation_retained_bytes(mutation)?;')
edit(PROC, '''    // 🧺️ `work_items` counts staged edit ROWS, not mutations: `prepare_process3d_config` always yields
    // exactly one inverse row beside the forward one, so every config gesture folds TWO rows. Declaring
    // 1 fail-closes each of them with `batched item candidate failed its exact fixed fold contract`.
''', '')
PROCT = S + '🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs'
edit(PROCT, '''    let footprint = process3d_mutation_footprint(&Process3dMutation::DeleteStep(DeleteStep { id: "step-1".into() })).expect("footprint");
    assert_eq!(footprint.work_items, 1);
    assert!(footprint.is_admissible());
    let oversized = process3d_mutation_footprint(&Process3dMutation::DeleteStep(DeleteStep { id: "x".repeat(PROCESS3D_DOCUMENT_TEXT_BYTES + 1) }));''', '''    let delete = Process3dMutation::DeleteStep(DeleteStep { id: "step-1".into() });
    let footprint = store::ArtifactStoreOneItemFootprint::for_leaf(&delete, process3d_mutation_retained_bytes(&delete).expect("retained bytes"));
    assert_eq!(footprint.work_items, store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS);
    assert!(footprint.is_admissible());
    let oversized = process3d_mutation_retained_bytes(&Process3dMutation::DeleteStep(DeleteStep { id: "x".repeat(PROCESS3D_DOCUMENT_TEXT_BYTES + 1) }));''')

SRC = S + '🪵️sourcing/🗿️artifacts/🗂️curation' + ANY
edit(SRC, '''/// 🧺️ The shared tail every admitted config footprint pays — the mutation owner plus one `String`
/// owner per work item, checked against the one-item preparation envelope.
fn sourcing_curation_config_footprint(work_items: usize, retained_bytes: usize) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = retained_bytes.saturating_add(size_of::<SourcingCurationConfigMutation>()).saturating_add(work_items.saturating_mul(size_of::<String>()));
    if work_items > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS || retained_bytes > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Sourcing Config mutation exceeds its fixed one-item preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items, retained_bytes })
}''', '''/// 🧺️ The shared tail every admitted config mutation pays — the mutation owner plus one `String` owner per item it
/// carries, checked against the one-item preparation envelope; answers the retained bytes.
fn sourcing_curation_config_retained_bytes(items: usize, retained_bytes: usize) -> Result<usize, String> {
    let retained_bytes = retained_bytes.saturating_add(size_of::<SourcingCurationConfigMutation>()).saturating_add(items.saturating_mul(size_of::<String>()));
    if items > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS || retained_bytes > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Sourcing Config mutation exceeds its fixed one-item preparation envelope".into());
    }
    Ok(retained_bytes)
}''')
edit(SRC, 'pub(crate) fn sourcing_curation_config_mutation_footprint(mutation: &SourcingCurationConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {', 'pub(crate) fn sourcing_curation_config_mutation_retained_bytes(mutation: &SourcingCurationConfigMutation) -> Result<usize, String> {')
edit(SRC, '        return sourcing_curation_config_footprint(1, json.len());', '        return sourcing_curation_config_retained_bytes(1, json.len());')
edit(SRC, '    let (work_items, retained_bytes) = match mutation {\n        SourcingCurationConfigMutation::Snapshot', '    let (items, retained_bytes) = match mutation {\n        SourcingCurationConfigMutation::Snapshot')
edit(SRC, '    sourcing_curation_config_footprint(work_items, retained_bytes)\n}', '    sourcing_curation_config_retained_bytes(items, retained_bytes)\n}')
edit(SRC, '    sourcing_curation_config_mutation_footprint(&mutation)?;', '    sourcing_curation_config_mutation_retained_bytes(&mutation)?;')
edit(SRC, '        sourcing_curation_config_mutation_footprint(mutation)?;', '        sourcing_curation_config_mutation_retained_bytes(mutation)?;')
edit(SRC, '            let bytes = sourcing_curation_config_mutation_footprint(mutation)?.retained_bytes;', '            let bytes = sourcing_curation_config_mutation_retained_bytes(mutation)?;')
edit(SRC, 'fn sourcing_curation_mutation_footprint(mutation: &SourcingMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {', 'fn sourcing_curation_mutation_retained_bytes(mutation: &SourcingMutation) -> Result<usize, String> {')
edit(SRC, '''    if retained_bytes > SOURCING_CURATION_DOCUMENT_MAXIMUM_BYTES { return Err("Sourcing Curation mutation exceeds its fixed one-item preparation envelope".into()); }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })''', '''    if retained_bytes > SOURCING_CURATION_DOCUMENT_MAXIMUM_BYTES { return Err("Sourcing Curation mutation exceeds its fixed one-item preparation envelope".into()); }
    Ok(retained_bytes)''')
edit(SRC, '    sourcing_curation_mutation_footprint(&mutation)?;', '    sourcing_curation_mutation_retained_bytes(&mutation)?;')
edit(SRC, '        sourcing_curation_mutation_footprint(mutation)?;', '        sourcing_curation_mutation_retained_bytes(mutation)?;')
edit(SRC, '            let bytes = sourcing_curation_mutation_footprint(mutation)?.retained_bytes;', '            let bytes = sourcing_curation_mutation_retained_bytes(mutation)?;')
for test in ('🏊️pool', None):
    pass
SRCT = S + '🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs'
edit(SRCT, 'sourcing_curation_config_mutation_footprint(', 'sourcing_curation_config_mutation_retained_bytes(', 7)

FLOW = S + '🌊️flow/🗿️artifacts/🌊️flow' + ANY
edit(FLOW, '''#[cfg(test)]
fn admit_flow_artifact_mutation(mutation: &FlowMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let work_items = flow_artifact_mutation_items(mutation);
    if work_items == 0 || work_items > FLOW_STORE_MAX_MUTATION_ITEMS {
        return Err("Flow artifact mutation exceeds its fixed semantic-item cap".into());
    }
    let retained_bytes = flow_bounded_serialized_bytes(mutation, FLOW_STORE_MAX_TEXT_BYTES)?;
    Ok(store::ArtifactStoreOneItemFootprint { work_items, retained_bytes })
}''', '''#[cfg(test)]
fn admit_flow_artifact_mutation(mutation: &FlowMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let items = flow_artifact_mutation_items(mutation);
    if items == 0 || items > FLOW_STORE_MAX_MUTATION_ITEMS {
        return Err("Flow artifact mutation exceeds its fixed semantic-item cap".into());
    }
    let retained_bytes = flow_bounded_serialized_bytes(mutation, FLOW_STORE_MAX_TEXT_BYTES)?;
    Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}''')
FLOWP = S + '🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/📬️preparation/🦀️.rs'
edit(FLOWP, '''        let inverse_rows = match mutation {
            FlowMutation::DeleteWidget(_) => FLOW_STORE_MAX_MUTATION_ITEMS.saturating_add(2),
            _ => 1,
        };
        Ok(store::ArtifactStoreOneItemFootprint::for_one_item(inverse_rows, FLOW_STORE_MAX_TEXT_BYTES))''', '''        Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, FLOW_STORE_MAX_TEXT_BYTES))''')

LAY = S + '📏️layout/🗿️artifacts/📏️layout' + ANY
regex(LAY, r'''//#region 🧺️ArtifactPreparation
/// 🧺️ The Artifact lane's one-item publication authority:.*?//#endregion 🧺️ArtifactPreparation

''', '')
edit(LAY, '''    /// page with its layers; `ChangeDataFields` a `fields:in` dictionary); its fold footprint is the leaf's own
    /// inverse-row bound ([`LayoutArtifactStorePreparationFactory`]).''', '''    /// page with its layers; `ChangeDataFields` a `fields:in` dictionary); its fold footprint is the leaf's own
    /// schema-declared inverse rows (`ArtifactStoreOneItemFootprint::for_leaf`).''')
edit(LAY, '''        Some(std::sync::Arc::new(LayoutArtifactStorePreparationFactory(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("layout-artifact-retained", LAYOUT_ARTIFACT_MUTATION_MAXIMUM_BYTES))))''', '''        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("layout-artifact-retained", LAYOUT_ARTIFACT_MUTATION_MAXIMUM_BYTES))''')
LAYM = S + '📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs'
edit(LAYM, '''/// 🧺️ The most rows `Mutation::inverse` yields for `mutation` on ANY base — the bound its one-item fold footprint declares,
/// since `preflight` never sees the base: a frame-selection leaf restores one absolute row per changed bound of every
/// target (`drag-frames` the origin; `rotate-frames` origin and rotation; `scale-frames` origin and extent), every other
/// leaf is point-invertible.
pub fn layout_mutation_inverse_rows(mutation: &LayoutMutation) -> usize {
    match mutation {
        LayoutMutation::DragFrames(leaf) => leaf.targets.len(),
        LayoutMutation::RotateFrames(leaf) => 2 * leaf.targets.len(),
        LayoutMutation::ScaleFrames(leaf) => 2 * leaf.targets.len(),
        _ => 1,
    }
}

''', '')
LAYT = S + '📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs'
edit(LAYT, '''/// 🧺️ Every committed quintet's inverse fits the fold footprint its leaf declares (`layout_mutation_inverse_rows`) — the''', '''/// 🧺️ Every committed quintet's inverse fits the fold footprint its leaf schema declares (`x-semio-inverse-rows`) — the''')
edit(LAYT, '            let (rows, declared) = (mutation.inverse(&base).len(), layout_mutation_inverse_rows(&mutation));', '            let (rows, declared) = (mutation.inverse(&base).len(), mutation.inverse_rows());')
edit(LAYT, '        assert_eq!(mutation.inverse(&document).len(), layout_mutation_inverse_rows(&mutation), "{mutation:?} declares exactly the rows it restores");', '        assert_eq!(mutation.inverse(&document).len(), mutation.inverse_rows(), "{mutation:?} declares exactly the rows it restores");')

DRAW = S + '🖍️draw/🗿️artifacts/🖍️drawing' + ANY
edit(DRAW, '        Ok(store::ArtifactStoreOneItemFootprint::for_one_item(crate::mutations::drawing_inverse_rows(mutation), store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))', '        Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))')
DRAWM = S + '🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs'
regex(DRAWM, r'''/// 🧺️ The most inverse rows `mutation` yields on any base — the retained store's fold declaration, proven from the
/// mutation alone:.*?pub fn drawing_inverse_rows\(mutation: &DrawingMutation\) -> usize \{.*?\n    \.max\(1\)\n\}\n''', '')

WFC = S + '🀄️wfc/🗿️artifacts/🖼️bitmap' + ANY
regex(WFC, r'''/// 🧺️ Inverse rows `resize-output` may declare:.*?const BITMAP_PIN_CASCADE_ROWS: usize = 1_024;

/// 🧺️ Staged inverse rows one bitmap mutation may fold\..*?fn bitmap_inverse_rows\(mutation: &BitmapMutation\) -> usize \{.*?\n\}

''', '')
edit(WFC, '    Ok(store::ArtifactStoreOneItemFootprint::for_one_item(bitmap_inverse_rows(mutation), retained_bytes))', '    Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))')

EQ = S + '➗️mathematical/🗿️artifacts/➗️equation' + ANY
regex(EQ, r'''        // 🧺️ `work_items` counts staged edit ROWS, never mutations:.*?declares; the number is never written at the call site\.
''', '')
EQT = S + '➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs'
edit(EQT, '''/// 🧺️ `ArtifactStoreOneItemFootprint::work_items` counts staged edit ROWS, so a point-invertible
/// item costs 2 — the hand-written `1` fail-closed every durable equation gesture with
/// `batched item candidate failed its exact fixed fold contract`.
#[semio_framework_async_macros::async_test]
async fn the_store_preparation_declares_a_point_invertible_footprint() {
    assert_eq!(
        store::ArtifactStoreOneItemFootprint::for_one_invertible_item(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).work_items,
        store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS
    );
    assert_eq!(store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS, 2);
}''', '''/// 🧺️ The Store preparation derives its fold footprint from the leaf's schema-declared inverse rows: a point-invertible
/// `disconnect-nodes` folds its forward row plus one inverse row, `delete-node` the re-created node plus every edge it
/// reconnects (`x-semio-inverse-rows` bounded by the editor's edge cap).
#[semio_framework_async_macros::async_test]
async fn the_store_preparation_derives_its_footprint_from_the_leaf() {
    let factory = EquationStorePreparationFactory::<EquationSnapshot, EquationMutation>::default();
    let rows = |mutation: EquationMutation| store::ArtifactStoreOneItemPreparationFactory::preflight(&factory, &mutation, None, store::HistoryLane::Document).expect("admitted").work_items;
    assert_eq!(rows(EquationMutation::DisconnectNodes(DisconnectNodes { id: "edge".into() })), store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS);
    assert_eq!(rows(EquationMutation::DeleteNode(DeleteNode { id: "node".into() })), 1 + 1 + EQUATION_MAX_EDGES);
}''')

for path, label in ((S + '🧱️block/🗿️artifacts/◻️2d' + ANY, 'Block2d'), (S + '🧱️block/🗿️artifacts/🖐️5d' + ANY, 'Block5d')):
    edit(path, f'''        // 🧾️ Every `{label}Mutation` is point-invertible (its `inverse` yields at most one row), and the
        // fold counts staged edit ROWS: one forward plus one inverse. Declaring one row fail-closed
        // every durable gesture with `batched item candidate failed its exact fixed fold contract`.
''', '')
RAS = S + '🖨️raster/🗿️artifacts/🖨️raster' + ANY
edit(RAS, '''        // 🧮 `work_items` counts the forward AND its inverse (every raster inverse is exactly one
        // operation: `create-layer` ↔ `delete-layer` of the whole subtree, `reorder` ↔ `reorder`, …) —
        // the batch fold sizes its inverse capacity as `Σ work_items − admitted_items`, so declaring
        // `1` exhausted it on the very first fold ("batched fold exceeded its admitted fixed inverse
        // capacity") and the two-layer demo never landed (process3d declares the same `2`).
''', '')
edit(RAS, '''        // 🧮 Forward + its one inverse (a whole-record config swap), for the same fold-capacity reason
        // the document lane's `preflight` records — at `1` every `setCompositeViewport`/`setCamera`
        // was refused "batched item candidate failed its exact fixed fold contract" (react boot
        // `raster-boot-5`, 2026-09-16).
''', '')
LOW = S + '💠️lowpoly/🗿️artifacts/💠️lowpoly' + ANY
edit(LOW, '''fn admit_lowpoly_artifact_mutation(mutation: &LowpolyMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {''', '''/// 🧺️ One forward row plus the leaf's derived inverse rows. A relative leaf's inverse carries what the base held — a
/// stroke's overwritten pixels, a selection motion's prior mesh content — which preflight never sees, so it declares the
/// lane's whole one-item byte envelope; every other inverse is bounded by its forward twin.
fn admit_lowpoly_artifact_mutation(mutation: &LowpolyMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {''')
edit(LOW, '''    // ↩️ One forward row plus its point inverse. A relative leaf's inverse carries what the base held — a stroke's
    // overwritten pixels, a selection motion's prior mesh content — which preflight never sees, so it declares the
    // lane's whole one-item envelope; every other inverse is bounded by its forward twin.
''', '')
edit(LOW, '''fn admit_lowpoly_config_mutation(mutation: &LowpolyConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {''', '''/// 🧺️ One forward row plus its point inverse, whose bytes (for `create-mesh` the prior content too) the forward twin bounds.
fn admit_lowpoly_config_mutation(mutation: &LowpolyConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {''')
edit(LOW, '''    // ↩️ One forward row plus its point inverse (which, for `create-mesh`, carries the prior content too).
''', '')

FORMS = S + '📋️forms/🗿️artifacts/📋️forms' + ANY
edit(FORMS, 'fn admit_forms_store_mutation<M: protocol::OpBinary>(mutation: &M) -> Result<store::ArtifactStoreOneItemFootprint, String> {', 'fn admit_forms_store_mutation<P, M: protocol::OpBinary + protocol::Mutation<P>>(mutation: &M) -> Result<store::ArtifactStoreOneItemFootprint, String> {')
edit(FORMS, '    Ok(store::ArtifactStoreOneItemFootprint::for_one_invertible_item(retained_bytes))\n}\n\nfn prepare_forms_store_mutation', '    Ok(store::ArtifactStoreOneItemFootprint::for_leaf::<P, M>(mutation, retained_bytes))\n}\n\nfn prepare_forms_store_mutation')
edit(FORMS, '    admit_forms_store_mutation(&mutation)?;', '    admit_forms_store_mutation::<P, M>(&mutation)?;')
edit(FORMS, '    if inverse.iter().any(|step| admit_forms_store_mutation(step).is_err()) {', '    if inverse.iter().any(|step| admit_forms_store_mutation::<P, M>(step).is_err()) {')
edit(FORMS, '        admit_forms_store_mutation(mutation)\n', '        admit_forms_store_mutation::<P, M>(mutation)\n')
SPACE = S + '🪐️space/🫀️core/🦀️.rs'
edit(SPACE, 'fn admit_space_retained_mutation<M: ::protocol::OpBinary>(mutation: &M, maximum_bytes: usize) -> Result<store::ArtifactStoreOneItemFootprint, String> {', 'fn admit_space_retained_mutation<P, M: ::protocol::OpBinary + ::protocol::Mutation<P>>(mutation: &M, maximum_bytes: usize) -> Result<store::ArtifactStoreOneItemFootprint, String> {')
edit(SPACE, '    Ok(store::ArtifactStoreOneItemFootprint::for_one_invertible_item(retained_bytes))\n}', '    Ok(store::ArtifactStoreOneItemFootprint::for_leaf::<P, M>(mutation, retained_bytes))\n}')
edit(SPACE, '    admit_space_retained_mutation(&mutation, maximum_bytes)?;', '    admit_space_retained_mutation::<P, M>(&mutation, maximum_bytes)?;')
edit(SPACE, '        admit_space_retained_mutation(mutation, self.maximum_bytes)\n', '        admit_space_retained_mutation::<P, M>(mutation, self.maximum_bytes)\n')

ZIP = S + '🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/📬️preparation/🦀️.rs'
edit(ZIP, '            .then(|| app_store::ArtifactStoreOneItemFootprint::for_one_item(MAXIMUM_STRUCTURAL_ITEMS, app_store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))', '            .then(|| app_store::ArtifactStoreOneItemFootprint::for_leaf(mutation, app_store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))')
SEMIO = S + '🗄️stdio/🗿️artifacts/🧿️semio/✏️editor/📬️preparation/🦀️.rs'
edit(SEMIO, '''    S: Send + Sync + 'static,
    M: app_store::ArtifactCanonicalJson + Send + Sync + 'static,
{
    fn preflight(''', '''    S: Send + Sync + 'static,
    M: app_store::ArtifactCanonicalJson + protocol::Mutation<S> + Send + Sync + 'static,
{
    fn preflight(''')
edit(SEMIO, '        Ok(app_store::ArtifactStoreOneItemFootprint::for_one_item(1, retained_bytes.max(1)))', '        Ok(app_store::ArtifactStoreOneItemFootprint::for_leaf::<S, M>(mutation, retained_bytes.max(1)))')
REG = S + '🗄️stdio/📇️registry/🧬️contract/✏️editing/🧪️tests/🔬️unit/🦀️.rs'
edit(REG, '            Ok(fixture_store::ArtifactStoreOneItemFootprint::for_one_invertible_item(self.retained_bytes))', '            Ok(fixture_store::ArtifactStoreOneItemFootprint { work_items: fixture_store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS, retained_bytes: self.retained_bytes })')

write = '--write' in sys.argv
failures = []
for path, items in EDITS.items():
    text = open(ROOT + path, encoding='utf-8').read()
    for old, new, count in items:
        if isinstance(old, re.Pattern):
            found = len(old.findall(text))
            if found != count: failures.append(f'{path.split("/")[2]} {path.split("/")[-2]} regex {old.pattern[:70]!r}: {found}')
            text = old.sub(new, text)
        else:
            found = text.count(old)
            if found != count: failures.append(f'{path.split("/")[2]} {path.split("/")[-2]} {old[:80]!r}: {found}')
            text = text.replace(old, new)
    if write and not failures:
        open(ROOT + path, 'w', encoding='utf-8').write(text)
print(f'files={len(EDITS)} failures={len(failures)} written={write and not failures}')
print('\n'.join(failures))
