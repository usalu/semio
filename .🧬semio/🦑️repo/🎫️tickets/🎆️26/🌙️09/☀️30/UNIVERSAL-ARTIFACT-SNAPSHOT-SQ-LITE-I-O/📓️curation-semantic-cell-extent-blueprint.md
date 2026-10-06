# Curation Complete Semantic Cell Extent Blueprint

Read-only source roster, no gate executed. Owner located by rg --files: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`. All14 actual tables come from sibling `🗄️.sql`.

Current defect: Native encode44–46 and decode47–49 check rows but never compute actual projected SQL cell semantic bytes. Preflight50–53 estimates Native encoding backing (`8192`, repeated24 etc), which is a separate quantity. Decode70+ couples NativeDecodeControl maximum to max_value_bytes and charges guessed map128/256 bytes in table24/singles28/members31; this does not replace semantic SQL extent.

Authoritative formula: integer/real SQL cell8 bytes, null0, text UTF8 bytes, blob byte length. Include every primary key even though wire INTEGER PRIMARY KEY alias may serialize NULL. `framework/.../sqlite-snapshot/🧩️artifact/🦀️.rs:51` defines Cell bytes; Projection::insert_key103–111 adds8 primary key then cell bytes and checks cumulative value bytes. IEEE helper175/190 supplies query/null + raw bits integer + class text. For each IEEE scalar charge `(NaN?0:8)+8+class_utf8_len`; Binary32 raw bits still SQL INTEGER8; classes finite6/nan3/positiveInfinity16/negativeInfinity16.

| Table | Entire semantic extent per row |
|---|---|
| document |8|
| catalog |16 + child_id.len + artifact_id.len + artifact_kind.len + standard.len + subset.len|
| stock_extra |32 + id.len + name.len + module_id.len|
| typology_segment |24 + segment.len|
| geometry |16 + kind.len|
| box |16 + I(width)+I(height)+I(depth)|
| frame |16 + I(width)+I(height)+I(depth)+I(profile)|
| slab |16 + I(width)+I(depth)+I(thickness)|
| mesh |16|
| glb |16 + url.len + I(extent)|
| mesh_position |24 + I(f32 value)|
| mesh_normal |24 + I(f32 value)|
| mesh_index |32|
| curated |32 + object_id.len|

Each stock emits stock_extra+geometry+exactly one recipe plus ordered typology/mesh rows. Root emits document+catalog; each curated emits one row. Use checked_add throughout and checkpoint per traversal. Typed extent should read actual owned fields and borrowed Record extent should mirror these exact rows. Before __dsl_from_record_controlled, use Record catalog0, stock1, curated2; stock id0,name1,module2,typology3,availability4,geometry5 statement; recipe order box[width,height,depth], frame[width,height,depth,profile], slab[width,depth,thickness], mesh[positions,normals,indices], glb[url,extent]. Verify catalog nested record field indexes against actual derived ArtifactChild spec before implementation; do not guess them. Shared existing native_rows20–23 already demonstrates row traversal and geometry statement scope.

Concrete control boundary: Native route receives caller SqliteSnapshotControl; capture control.limits().max_value_bytes before borrowing it into record decode. Typed encoder call control.check_value_bytes(extent) before controlled conversion. Borrowed decode helper compares cumulative exact extent to captured maximum before constructing owner, with native.step checkpoints. Retain allocation ledger authority separately; no guessed workspace multiplier can serve either semantic extent or complete actual backing proof. This fixes tiny semantic limit acceptance independent of backing repair.
