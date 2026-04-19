const fs = require("fs");
const filePath = "c:\\git\\semio\\semio\\rs\\lib.rs";
let content = fs.readFileSync(filePath, "utf-8");

const modStartMarker = "mod apply_diff {";
const modEndMarker = "} // ✈️ApplyDiff";

const modStart = content.indexOf(modStartMarker);
const modEnd = content.indexOf(modEndMarker);

if (modStart === -1 || modEnd === -1) {
  console.error("Could not find module boundaries");
  process.exit(1);
}

const endIdx = modEnd + modEndMarker.length;
const before = content.substring(0, modStart);
const after = content.substring(endIdx);

const newModule = `mod apply_diff {
    // 🎲ApplyDiff
    // ApplyDiff MUST provide the applydiff functionality.

    /// <summary>🔖apply_collection_diff holds the data fields for a apply_collection_diff record.</summary>
    /// <remarks>
    /// </remarks>
    use super::*;
    use std::collections::{HashMap, HashSet};
    use std::sync::{Arc, Weak};

    impl Kit {
        pub fn ports_arc_map(&self) -> HashMap<String, Arc<Port>> {
            self.ports
                .as_ref()
                .map(|ps| {
                    ps.iter()
                        .map(|p| (p.guid.clone(), Arc::new(p.clone())))
                        .collect()
                })
                .unwrap_or_default()
        }

        pub fn types_map_index(&self) -> HashMap<String, Arc<Type>> {
            self.types
                .as_ref()
                .map(|ts| ts.iter().map(|t| (t.guid.clone(), t.clone())).collect())
                .unwrap_or_default()
        }

        pub fn designs_map_index(&self) -> HashMap<String, Arc<Design>> {
            self.designs
                .as_ref()
                .map(|ds| ds.iter().map(|d| (d.guid.clone(), d.clone())).collect())
                .unwrap_or_default()
        }
    }

    impl Design {
        /// Maps [\`Piece::type_ref\`] values present in this design for [\`Design::apply_diff\`] without a full [\`Kit\`].
        pub fn types_from_pieces(&self) -> HashMap<String, Arc<Type>> {
            self
                .pieces
                .as_ref()
                .into_iter()
                .flatten()
                .filter_map(|p| p.type_ref.as_ref().map(|t| (t.guid.clone(), t.clone())))
                .collect()
        }
    }

    fn piece_arc_from_dto(
        pw: &PieceDto,
        types: &HashMap<String, Arc<Type>>,
        design_weak: Option<Weak<Design>>,
    ) -> Arc<Piece> {
        Arc::new(Piece {
            guid: pw.guid.clone(),
            name: pw.name.clone(),
            type_ref: pw
                .type_ref
                .as_ref()
                .and_then(|tid| types.get(&tid.guid).cloned()),
            design: design_weak,
            plane: pw.plane.as_ref().map(Plane::from),
            center: pw.center.as_ref().map(Coord::from),
            scale: pw.scale,
            mirror_plane: pw.mirror_plane.as_ref().map(Plane::from),
            is_hidden: pw.is_hidden,
            is_locked: pw.is_locked,
            color: pw.color.clone(),
            description: pw.description.clone(),
            props: pw
                .props
                .as_ref()
                .map(|v| v.iter().map(Prop::from).collect()),
            attributes: pw
                .attributes
                .as_ref()
                .map(|v| v.iter().map(Attribute::from).collect()),
        })
    }

    fn connector_for_piece_side(side: &Side, cid: &ConnectorId) -> Option<Arc<Connector>> {
        let mut cur = side.piece.type_ref.clone();
        while let Some(t) = cur {
            if let Some(c) = t
                .connectors
                .as_ref()
                .and_then(|cs| cs.iter().find(|c| c.guid == cid.guid).cloned())
            {
                return Some(c);
            }
            cur = t.parent.clone();
        }
        None
    }

    fn apply_kit_types_patch(
        kit: &mut Kit,
        diff: Option<&CollectionDiff<TypeDto, TypeDiff>>,
        ports_map: &HashMap<String, Arc<Port>>,
    ) {
        let Some(diff) = diff else {
            return;
        };
        let mut types = kit.types.take().unwrap_or_default();
        let mut type_map: HashMap<String, Arc<Type>> =
            types.iter().map(|t| (t.guid.clone(), t.clone())).collect();

        if let Some(removed) = &diff.removed {
            let rs: HashSet<String> = removed.iter().map(|r| r.guid.clone()).collect();
            types.retain(|t| !rs.contains(&t.guid));
            for r in removed {
                type_map.remove(&r.guid);
            }
        }

        if let Some(added) = &diff.added {
            types = merge_type_dtos(types, added, ports_map)
                .unwrap_or_else(|e| panic!("apply_kit_types_patch: {}", e));
            type_map = types.iter().map(|t| (t.guid.clone(), t.clone())).collect();
        }

        if let Some(updated) = &diff.updated {
            for u in updated {
                if let Some(idx) = types.iter().position(|t| t.guid == u.guid) {
                    {
                        let t = Arc::make_mut(&mut types[idx]);
                        t.apply_diff(&u.diff, &type_map, ports_map);
                    }
                    type_map.insert(u.guid.clone(), types[idx].clone());
                }
            }
        }

        kit.types = if types.is_empty() { None } else { Some(types) };
    }

    fn apply_kit_designs_patch(
        kit: &mut Kit,
        diff: Option<&CollectionDiff<DesignDto, DesignDiff>>,
        types_map: &HashMap<String, Arc<Type>>,
    ) {
        let Some(diff) = diff else {
            return;
        };
        let mut designs = kit.designs.take().unwrap_or_default();
        let mut design_map: HashMap<String, Arc<Design>> = designs
            .iter()
            .map(|d| (d.guid.clone(), d.clone()))
            .collect();

        if let Some(removed) = &diff.removed {
            let rs: HashSet<String> = removed.iter().map(|r| r.guid.clone()).collect();
            designs.retain(|d| !rs.contains(&d.guid));
            for r in removed {
                design_map.remove(&r.guid);
            }
        }

        if let Some(added) = &diff.added {
            for dw in added {
                let d = resolve_design_from_dto(dw, types_map)
                    .unwrap_or_else(|e| panic!("apply_kit_designs_patch: {}", e));
                design_map.insert(d.guid.clone(), d.clone());
                designs.push(d);
            }
        }

        if let Some(updated) = &diff.updated {
            for u in updated {
                if let Some(idx) = designs.iter().position(|d| d.guid == u.guid) {
                    {
                        let d_mut = Arc::make_mut(&mut designs[idx]);
                        d_mut.apply_diff(&u.diff, types_map, &design_map);
                    }
                    design_map.insert(u.guid.clone(), designs[idx].clone());
                }
            }
        }

        kit.designs = if designs.is_empty() {
            None
        } else {
            Some(designs)
        };
    }

    pub fn apply_collection_diff<T, W, D>(
        collection: &mut Option<Vec<T>>,
        diff: &Option<CollectionDiff<W, D>>,
        apply_item_diff: impl Fn(&mut T, &D),
        from_added: impl Fn(&W) -> T,
    ) where
        T: HasGuid + Clone,
        W: HasGuid + Clone,
        D: DiffHasGuid,
    {
        if let Some(diff) = diff {
            let mut new_items = collection.clone().unwrap_or_default();

            if let Some(removed_items) = &diff.removed {
                let removed_set: HashSet<_> =
                    removed_items.iter().map(|s| s.guid.clone()).collect();
                new_items.retain(|item| !removed_set.contains(&item.guid().to_string()));
            }

            if let Some(updated_diffs) = &diff.updated {
                let diff_map: HashMap<_, _> = updated_diffs
                    .iter()
                    .map(|d| (d.guid().to_string(), d))
                    .collect();
                for item in &mut new_items {
                    if let Some(update) = diff_map.get(item.guid()) {
                        apply_item_diff(item, &update.diff);
                    }
                }
            }

            if let Some(added_items) = &diff.added {
                new_items.extend(added_items.iter().map(|w| from_added(w)));
            }

            *collection = if new_items.is_empty() {
                None
            } else {
                Some(new_items)
            };
        }
    }

    impl Attribute {
        /// 🔖Applies an AttributeDiff to this Attribute in-place.
        pub fn apply_diff(&mut self, diff: &AttributeDiff) {
            if let Some(value) = &diff.key {
                self.key = value.clone();
            }
            if let Some(value) = &diff.value {
                self.value = value.clone();
            }
            if let Some(value) = &diff.definition {
                self.definition = value.clone();
            }
        }
    }

    impl Prop {
        /// 🔖Applies a PropDiff to this Prop in-place.
        pub fn apply_diff(&mut self, diff: &PropDiff) {
            if let Some(value) = &diff.quality {
                self.quality = value.clone();
            }
            if let Some(value) = &diff.value {
                self.value = value.clone();
            }
            if let Some(value) = &diff.unit {
                self.unit = value.clone();
            }
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }

    impl Connector {
        /// 🔖Applies a ConnectorDiff to this Connector in-place.
        pub fn apply_diff(
            &mut self,
            diff: &ConnectorDiff,
            ports: &HashMap<String, Arc<Port>>,
        ) {
            if let Some(value) = &diff.point {
                self.point.x += value.x;
                self.point.y += value.y;
                self.point.z += value.z;
            }
            if let Some(value) = &diff.direction {
                self.direction.x += value.x;
                self.direction.y += value.y;
                self.direction.z += value.z;
            }
            if let Some(value) = &diff.t {
                self.t = *value;
            }
            if let Some(v) = &diff.name {
                self.name = v.clone();
            }
            if let Some(v) = &diff.description {
                self.description = v.clone();
            }
            if let Some(v) = &diff.mandatory {
                self.mandatory = v.clone();
            }
            if let Some(v) = &diff.max_children {
                self.max_children = v.clone();
            }
            if let Some(outer) = &diff.port {
                self.port = outer.as_ref().and_then(|pid| ports.get(&pid.guid).cloned());
            }
            apply_collection_diff(&mut self.props, &diff.props, |item, diff| item.apply_diff(diff), |p| {
                Prop::from(p)
            });
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }

    impl Model {
        /// 🔖Applies a ModelDiff to this Model in-place.
        pub fn apply_diff(&mut self, diff: &ModelDiff) {
            if let Some(value) = &diff.file {
                self.file = value.clone();
            }
            if let Some(value) = &diff.name {
                self.name = value.clone();
            }
            if let Some(value) = &diff.description {
                self.description = value.clone();
            }
            if let Some(value) = &diff.tags {
                self.tags = value.clone();
            }
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }

    impl Type {
        /// 🔖Applies a TypeDiff to this Type in-place.
        pub fn apply_diff(
            &mut self,
            diff: &TypeDiff,
            types: &HashMap<String, Arc<Type>>,
            ports: &HashMap<String, Arc<Port>>,
        ) {
            if let Some(value) = &diff.name {
                self.name = value.clone();
            }
            if let Some(v) = &diff.parent {
                self.parent = v.as_ref().and_then(|tid| types.get(&tid.guid).cloned());
            }
            if let Some(v) = &diff.families {
                self.families = v.as_ref().map(|ids| {
                    ids.iter()
                        .filter_map(|tid| types.get(&tid.guid).cloned())
                        .collect()
                });
            }
            if let Some(v) = &diff.description {
                self.description = v.clone();
            }
            if let Some(v) = &diff.icon {
                self.icon = v.clone();
            }
            if let Some(v) = &diff.image {
                self.image = v.clone();
            }
            if let Some(v) = &diff.folder {
                self.folder = v.clone();
            }
            if let Some(v) = &diff.unit {
                self.unit = v.clone();
            }
            if let Some(v) = &diff.stock {
                self.stock = v.clone();
            }
            if let Some(v) = &diff.is_abstract {
                self.is_abstract = v.clone();
            }
            if let Some(v) = &diff.virtual_type {
                self.virtual_type = v.clone();
            }
            if let Some(v) = &diff.location {
                self.location = v.clone();
            }
            if let Some(v) = &diff.concepts {
                self.concepts = v.clone();
            }
            if let Some(v) = &diff.authors {
                self.authors = v.clone();
            }
            apply_collection_diff(&mut self.props, &diff.props, |item, diff| item.apply_diff(diff), |p| {
                Prop::from(p)
            });
            apply_collection_diff(&mut self.models, &diff.models, |item, diff| item.apply_diff(diff), |m| {
                Model::from(m)
            });

            if let Some(cd) = &diff.connectors {
                let mut conns = self.connectors.clone().unwrap_or_default();
                if let Some(removed) = &cd.removed {
                    let rs: HashSet<String> = removed.iter().map(|r| r.guid.clone()).collect();
                    conns.retain(|c| !rs.contains(&c.guid));
                }
                if let Some(updated) = &cd.updated {
                    let um: HashMap<String, &ConnectorDiff> =
                        updated.iter().map(|u| (u.guid.clone(), &u.diff)).collect();
                    for c in &mut conns {
                        if let Some(d) = um.get(&c.guid) {
                            Arc::make_mut(c).apply_diff(d, ports);
                        }
                    }
                }
                if let Some(added) = &cd.added {
                    for cw in added {
                        conns.push(Arc::new(Connector::from_dto(cw, ports)));
                    }
                }
                self.connectors = if conns.is_empty() { None } else { Some(conns) };
            }

            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }

    impl Layer {
        /// 🔖Applies a LayerDiff to this Layer in-place.
        pub fn apply_diff(&mut self, diff: &LayerDiff) {
            if let Some(value) = &diff.path {
                self.path = value.clone();
            }
            if let Some(value) = &diff.is_hidden {
                self.is_hidden = *value;
            }
            if let Some(value) = &diff.is_locked {
                self.is_locked = *value;
            }
            if let Some(value) = &diff.color {
                self.color = value.clone();
            }
            if let Some(value) = &diff.description {
                self.description = value.clone();
            }
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }

    impl Group {
        /// 🔖Applies a GroupDiff to this Group in-place.
        pub fn apply_diff(&mut self, diff: &GroupDiff) {
            if let Some(value) = &diff.name {
                self.name = value.clone();
            }
            if let Some(value) = &diff.color {
                self.color = value.clone();
            }
            if let Some(value) = &diff.description {
                self.description = value.clone();
            }
            if let Some(value) = &diff.pieces {
                self.pieces = value.clone();
            }
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }

    impl Stat {
        /// 🔖Applies a StatDiff to this Stat in-place.
        pub fn apply_diff(&mut self, diff: &StatDiff) {
            if let Some(value) = &diff.quality {
                self.quality = value.clone();
            }
            if let Some(value) = &diff.min {
                self.min = *value;
            }
            if let Some(value) = &diff.min_excluded {
                self.min_excluded = *value;
            }
            if let Some(value) = &diff.max {
                self.max = *value;
            }
            if let Some(value) = &diff.max_excluded {
                self.max_excluded = *value;
            }
            if let Some(value) = &diff.unit {
                self.unit = value.clone();
            }
        }
    }

    impl Piece {
        /// 🔖Applies a PieceDiff to this Piece in-place.
        pub fn apply_diff(
            &mut self,
            diff: &PieceDiff,
            types: &HashMap<String, Arc<Type>>,
            designs: &HashMap<String, Arc<Design>>,
        ) {
            if let Some(v) = &diff.name {
                self.name = v.clone();
            }
            if let Some(v) = &diff.type_ref {
                self.type_ref = v.as_ref().and_then(|tid| types.get(&tid.guid).cloned());
            }
            if let Some(v) = &diff.design {
                self.design = v
                    .as_ref()
                    .map(|did| designs.get(&did.guid).map(|d| Arc::downgrade(d)))
                    .flatten();
            }
            if let Some(v) = &diff.plane {
                self.plane = v.as_ref().map(Plane::from);
            }
            if let Some(v) = &diff.center {
                self.center = v.as_ref().map(Coord::from);
            }
            if let Some(v) = &diff.scale {
                self.scale = v.clone();
            }
            if let Some(v) = &diff.mirror_plane {
                self.mirror_plane = v.as_ref().map(Plane::from);
            }
            if let Some(v) = &diff.is_hidden {
                self.is_hidden = v.clone();
            }
            if let Some(v) = &diff.is_locked {
                self.is_locked = v.clone();
            }
            if let Some(v) = &diff.color {
                self.color = v.clone();
            }
            if let Some(v) = &diff.description {
                self.description = v.clone();
            }
            apply_collection_diff(&mut self.props, &diff.props, |item, diff| item.apply_diff(diff), |p| {
                Prop::from(p)
            });
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }

    impl Connection {
        /// 🔖Applies a ConnectionDiff to this Connection in-place.
        pub fn apply_diff(
            &mut self,
            diff: &ConnectionDiff,
            pieces: &HashMap<String, Arc<Piece>>,
        ) {
            let prev_connected_connector_guid =
                self.connected.connector.as_ref().map(|c| c.guid.clone());
            let prev_connecting_connector_guid =
                self.connecting.connector.as_ref().map(|c| c.guid.clone());
            if let Some(sd) = &diff.connected {
                if let Some(pid) = &sd.piece {
                    if let Some(p) = pieces.get(&pid.guid) {
                        self.connected.piece = p.clone();
                    }
                }
                if let Some(o) = &sd.design_piece {
                    self.connected.design_piece =
                        o.as_ref().and_then(|pid| pieces.get(&pid.guid).cloned());
                }
                match &sd.connector {
                    None => {
                        if let Some(ref g) = prev_connected_connector_guid {
                            self.connected.connector = connector_for_piece_side(
                                &self.connected,
                                &ConnectorId { guid: g.clone() },
                            );
                        }
                    }
                    Some(inner) => {
                        self.connected.connector = inner
                            .as_ref()
                            .and_then(|cid| connector_for_piece_side(&self.connected, cid));
                    }
                }
            }
            if let Some(sd) = &diff.connecting {
                if let Some(pid) = &sd.piece {
                    if let Some(p) = pieces.get(&pid.guid) {
                        self.connecting.piece = p.clone();
                    }
                }
                if let Some(o) = &sd.design_piece {
                    self.connecting.design_piece =
                        o.as_ref().and_then(|pid| pieces.get(&pid.guid).cloned());
                }
                match &sd.connector {
                    None => {
                        if let Some(ref g) = prev_connecting_connector_guid {
                            self.connecting.connector = connector_for_piece_side(
                                &self.connecting,
                                &ConnectorId { guid: g.clone() },
                            );
                        }
                    }
                    Some(inner) => {
                        self.connecting.connector = inner
                            .as_ref()
                            .and_then(|cid| connector_for_piece_side(&self.connecting, cid));
                    }
                }
            }
            if let Some(value) = &diff.gap {
                self.gap += value;
            }
            if let Some(value) = &diff.shift {
                self.shift += value;
            }
            if let Some(value) = &diff.rise {
                self.rise += value;
            }
            if let Some(value) = &diff.rotation {
                self.rotation += value;
            }
            if let Some(value) = &diff.turn {
                self.turn += value;
            }
            if let Some(value) = &diff.tilt {
                self.tilt += value;
            }
            if let Some(value) = &diff.u {
                match value {
                    Some(delta) => {
                        self.u = Some(self.u.unwrap_or(0.0) + delta);
                    }
                    None => {
                        self.u = None;
                    }
                }
            }
            if let Some(value) = &diff.v {
                match value {
                    Some(delta) => {
                        self.v = Some(self.v.unwrap_or(0.0) + delta);
                    }
                    None => {
                        self.v = None;
                    }
                }
            }
            if let Some(value) = &diff.description {
                self.description = value.clone();
            }
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }

    impl Design {
        /// 🔖Applies a DesignDiff to this Design in-place.
        pub fn apply_diff(
            &mut self,
            diff: &DesignDiff,
            types: &HashMap<String, Arc<Type>>,
            designs: &HashMap<String, Arc<Design>>,
        ) {
            if let Some(value) = &diff.name {
                self.name = value.clone();
            }
            if let Some(v) = &diff.parent {
                self.parent = v.clone();
            }
            if let Some(v) = &diff.families {
                self.families = v.clone();
            }
            if let Some(v) = &diff.description {
                self.description = v.clone();
            }
            if let Some(v) = &diff.icon {
                self.icon = v.clone();
            }
            if let Some(v) = &diff.image {
                self.image = v.clone();
            }
            if let Some(v) = &diff.folder {
                self.folder = v.clone();
            }
            if let Some(v) = &diff.unit {
                self.unit = v.clone();
            }
            if let Some(v) = &diff.is_abstract {
                self.is_abstract = v.clone();
            }
            if let Some(v) = &diff.can_scale {
                self.can_scale = v.clone();
            }
            if let Some(v) = &diff.can_mirror {
                self.can_mirror = v.clone();
            }
            if let Some(v) = &diff.concepts {
                self.concepts = v.clone();
            }
            if let Some(v) = &diff.authors {
                self.authors = v.clone();
            }
            if let Some(v) = &diff.active_layer {
                self.active_layer = v.clone();
            }
            apply_collection_diff(&mut self.props, &diff.props, |item, diff| item.apply_diff(diff), |p| {
                Prop::from(p)
            });

            let self_weak = designs.get(&self.guid).map(|d| Arc::downgrade(d));

            if let Some(pd) = &diff.pieces {
                let mut pieces_vec = self.pieces.clone().unwrap_or_default();
                if let Some(removed) = &pd.removed {
                    let rs: HashSet<String> = removed.iter().map(|r| r.guid.clone()).collect();
                    pieces_vec.retain(|p| !rs.contains(&p.guid));
                }
                if let Some(added) = &pd.added {
                    for pw in added {
                        pieces_vec.push(piece_arc_from_dto(pw, types, self_weak.clone()));
                    }
                }
                if let Some(updated) = &pd.updated {
                    let um: HashMap<String, &PieceDiff> =
                        updated.iter().map(|u| (u.guid.clone(), &u.diff)).collect();
                    for p in &mut pieces_vec {
                        if let Some(d) = um.get(&p.guid) {
                            Arc::make_mut(p).apply_diff(d, types, designs);
                        }
                    }
                }
                self.pieces = if pieces_vec.is_empty() {
                    None
                } else {
                    Some(pieces_vec)
                };
            }

            let piece_map: HashMap<String, Arc<Piece>> = self
                .pieces
                .as_ref()
                .map(|pv| pv.iter().map(|p| (p.guid.clone(), p.clone())).collect())
                .unwrap_or_default();

            if let Some(cd) = &diff.connections {
                let mut conns = self.connections.clone().unwrap_or_default();
                if let Some(removed) = &cd.removed {
                    let rs: HashSet<String> = removed.iter().map(|r| r.guid.clone()).collect();
                    conns.retain(|c| !rs.contains(&c.guid));
                }
                if let Some(added) = &cd.added {
                    for cw in added {
                        conns.push(connection_from_dto(cw, &piece_map).unwrap_or_else(|e| {
                            panic!("Design::apply_diff connection {}: {:?}", cw.guid, e)
                        }));
                    }
                }
                if let Some(updated) = &cd.updated {
                    let um: HashMap<String, &ConnectionDiff> =
                        updated.iter().map(|u| (u.guid.clone(), &u.diff)).collect();
                    for c in &mut conns {
                        if let Some(d) = um.get(&c.guid) {
                            c.apply_diff(d, &piece_map);
                        }
                    }
                }
                self.connections = if conns.is_empty() { None } else { Some(conns) };
            }

            apply_collection_diff(&mut self.layers, &diff.layers, |item, diff| item.apply_diff(diff), |d| {
                Layer::from(d)
            });
            apply_collection_diff(&mut self.groups, &diff.groups, |item, diff| item.apply_diff(diff), |d| {
                Group::from(d)
            });
            apply_collection_diff(&mut self.stats, &diff.stats, |item, diff| item.apply_diff(diff), |d| {
                Stat::from(d)
            });
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
            self.invalidate_hash();
        }

        /// 📌Creates a mixed design keeping old entities with diff status annotations.
        /// annotate each with a semio.diffStatus attribute (unchanged/modified/removed/added).
        /// Updated entities are applied (new positions/values) and marked as modified.
        /// Removed entities are kept in place marked as removed.
        /// Added entities are appended marked as added.
        pub fn with_diff(&self, diff: &DesignDiff) -> Design {
            let status_attr = |status: &str| Attribute {
                guid: format!("semio.diffStatus.{}", status),
                key: "semio.diffStatus".to_string(),
                value: Some(status.to_string()),
                definition: None,
            };

            let types_map = self.types_from_pieces();
            let designs_map: HashMap<String, Arc<Design>> = HashMap::new();

            let removed_piece_guids: std::collections::HashSet<&str> = diff
                .pieces
                .as_ref()
                .map(|pd| {
                    pd.removed
                        .as_ref()
                        .map(|r| r.iter().map(|id| id.guid.as_str()).collect())
                        .unwrap_or_default()
                })
                .unwrap_or_default();
            let updated_piece_map: std::collections::HashMap<&str, &PieceDiff> = diff
                .pieces
                .as_ref()
                .map(|pd| {
                    pd.updated
                        .as_ref()
                        .map(|u| u.iter().map(|upd| (upd.guid.as_str(), &upd.diff)).collect())
                        .unwrap_or_default()
                })
                .unwrap_or_default();

            let removed_conn_guids: std::collections::HashSet<&str> = diff
                .connections
                .as_ref()
                .map(|cd| {
                    cd.removed
                        .as_ref()
                        .map(|r| r.iter().map(|id| id.guid.as_str()).collect())
                        .unwrap_or_default()
                })
                .unwrap_or_default();
            let updated_conn_map: std::collections::HashMap<&str, &ConnectionDiff> = diff
                .connections
                .as_ref()
                .map(|cd| {
                    cd.updated
                        .as_ref()
                        .map(|u| u.iter().map(|upd| (upd.guid.as_str(), &upd.diff)).collect())
                        .unwrap_or_default()
                })
                .unwrap_or_default();

            let mut result_pieces: Vec<Arc<Piece>> = Vec::new();
            if let Some(ref pieces) = self.pieces {
                for p in pieces {
                    let mut pc = p.as_ref().clone();
                    if removed_piece_guids.contains(pc.guid.as_str()) {
                        let mut attrs = pc.attributes.clone().unwrap_or_default();
                        attrs.push(status_attr("removed"));
                        pc.attributes = Some(attrs);
                    } else if let Some(piece_diff) = updated_piece_map.get(pc.guid.as_str()) {
                        pc.apply_diff(piece_diff, &types_map, &designs_map);
                        let mut attrs = pc.attributes.clone().unwrap_or_default();
                        attrs.push(status_attr("modified"));
                        pc.attributes = Some(attrs);
                    } else {
                        let mut attrs = pc.attributes.clone().unwrap_or_default();
                        attrs.push(status_attr("unchanged"));
                        pc.attributes = Some(attrs);
                    }
                    result_pieces.push(Arc::new(pc));
                }
            }
            if let Some(ref pd) = diff.pieces {
                if let Some(ref added) = pd.added {
                    for pw in added {
                        let arc = piece_arc_from_dto(pw, &types_map, None);
                        let mut inner = arc.as_ref().clone();
                        let mut attrs = inner.attributes.clone().unwrap_or_default();
                        attrs.push(status_attr("added"));
                        inner.attributes = Some(attrs);
                        result_pieces.push(Arc::new(inner));
                    }
                }
            }

            let piece_map: HashMap<String, Arc<Piece>> = result_pieces
                .iter()
                .map(|p| (p.guid.clone(), p.clone()))
                .collect();

            let mut result_conns: Vec<Connection> = Vec::new();
            if let Some(ref conns) = self.connections {
                for c in conns {
                    let mut cc = c.clone();
                    if removed_conn_guids.contains(cc.guid.as_str()) {
                        let mut attrs = cc.attributes.clone().unwrap_or_default();
                        attrs.push(status_attr("removed"));
                        cc.attributes = Some(attrs);
                    } else if let Some(conn_diff) = updated_conn_map.get(cc.guid.as_str()) {
                        cc.apply_diff(conn_diff, &piece_map);
                        let mut attrs = cc.attributes.clone().unwrap_or_default();
                        attrs.push(status_attr("modified"));
                        cc.attributes = Some(attrs);
                    } else {
                        let mut attrs = cc.attributes.clone().unwrap_or_default();
                        attrs.push(status_attr("unchanged"));
                        cc.attributes = Some(attrs);
                    }
                    result_conns.push(cc);
                }
            }
            if let Some(ref cd) = diff.connections {
                if let Some(ref added) = cd.added {
                    for cw in added {
                        let mut ac = connection_from_dto(cw, &piece_map).unwrap_or_else(|e| {
                            panic!("Design::with_diff connection {}: {:?}", cw.guid, e)
                        });
                        let mut attrs = ac.attributes.clone().unwrap_or_default();
                        attrs.push(status_attr("added"));
                        ac.attributes = Some(attrs);
                        result_conns.push(ac);
                    }
                }
            }

            let mut result = self.clone();
            result.pieces = Some(result_pieces);
            result.connections = Some(result_conns);
            result
        }
    }

    impl Tag {
        /// 🔖Applies a TagDiff to this Tag in-place.
        pub fn apply_diff(&mut self, diff: &TagDiff) {
            if let Some(value) = &diff.name {
                self.name = value.clone();
            }
            if let Some(value) = &diff.description {
                self.description = value.clone();
            }
            if let Some(value) = &diff.icon {
                self.icon = value.clone();
            }
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }

    impl Concept {
        /// 🔖Applies a ConceptDiff to this Concept in-place.
        pub fn apply_diff(&mut self, diff: &ConceptDiff) {
            if let Some(value) = &diff.name {
                self.name = value.clone();
            }
            if let Some(value) = &diff.description {
                self.description = value.clone();
            }
            if let Some(value) = &diff.icon {
                self.icon = value.clone();
            }
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }

    impl Port {
        /// 🔖Applies a PortDiff to this Port in-place.
        pub fn apply_diff(&mut self, diff: &PortDiff) {
            if let Some(value) = &diff.name {
                self.name = value.clone();
            }
            if let Some(value) = &diff.description {
                self.description = value.clone();
            }
            if let Some(value) = &diff.icon {
                self.icon = value.clone();
            }
            if let Some(value) = &diff.compatible_interfaces {
                self.compatible_interfaces = value.clone();
            }
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }

    impl Quality {
        /// 🔖Applies a QualityDiff to this Quality in-place.
        pub fn apply_diff(&mut self, diff: &QualityDiff) {
            if let Some(value) = &diff.key {
                self.key = value.clone();
            }
            if let Some(value) = &diff.name {
                self.name = value.clone();
            }
            if let Some(value) = &diff.kind {
                self.kind = value.clone();
            }
            if let Some(value) = &diff.default_value {
                self.default_value = *value;
            }
            if let Some(value) = &diff.formula {
                self.formula = value.clone();
            }
            if let Some(value) = &diff.default_si_unit {
                self.default_si_unit = value.clone();
            }
            if let Some(value) = &diff.default_imperial_unit {
                self.default_imperial_unit = value.clone();
            }
            if let Some(value) = &diff.min {
                self.min = *value;
            }
            if let Some(value) = &diff.is_min_excluded {
                self.is_min_excluded = *value;
            }
            if let Some(value) = &diff.max {
                self.max = *value;
            }
            if let Some(value) = &diff.is_max_excluded {
                self.is_max_excluded = *value;
            }
            if let Some(value) = &diff.can_scale {
                self.can_scale = *value;
            }
            if let Some(value) = &diff.uri {
                self.uri = value.clone();
            }
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }

    impl File {
        /// 🔖Applies a FileDiff to this File in-place.
        pub fn apply_diff(&mut self, diff: &FileDiff) {
            if let Some(value) = &diff.name {
                self.name = value.clone();
            }
            if let Some(value) = &diff.remote {
                self.remote = value.clone();
            }
            if let Some(value) = &diff.folder {
                self.folder = value.clone();
            }
            if let Some(value) = &diff.size {
                self.size = *value;
            }
            if let Some(value) = &diff.hash {
                self.hash = value.clone();
            }
        }
    }

    impl Folder {
        /// 🔖Applies a FolderDiff to this Folder in-place.
        pub fn apply_diff(&mut self, diff: &FolderDiff) {
            if let Some(value) = &diff.name {
                self.name = value.clone();
            }
            if let Some(value) = &diff.parent {
                self.parent = value.clone();
            }
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }

    impl Author {
        /// 🔖Applies an AuthorDiff to this Author in-place.
        pub fn apply_diff(&mut self, diff: &AuthorDiff) {
            if let Some(value) = &diff.name {
                self.name = value.clone();
            }
            if let Some(value) = &diff.email {
                self.email = value.clone();
            }
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }

    impl Kit {
        /// 🔖Applies a KitDiff to this Kit in-place.
        pub fn apply_diff(&mut self, diff: &KitDiff) {
            if let Some(value) = &diff.name {
                self.name = value.clone();
            }
            if let Some(value) = &diff.version {
                self.version = value.clone();
            }
            if let Some(value) = &diff.description {
                self.description = value.clone();
            }
            if let Some(value) = &diff.icon {
                self.icon = value.clone();
            }
            if let Some(value) = &diff.image {
                self.image = value.clone();
            }
            if let Some(value) = &diff.preview {
                self.preview = value.clone();
            }
            if let Some(value) = &diff.remote {
                self.remote = value.clone();
            }
            if let Some(value) = &diff.homepage {
                self.homepage = value.clone();
            }
            if let Some(value) = &diff.license {
                self.license = value.clone();
            }
            apply_collection_diff(
                &mut self.concepts,
                &diff.concepts,
                |item, diff| item.apply_diff(diff),
                |c| Concept::from(c),
            );
            apply_collection_diff(&mut self.tags, &diff.tags, |item, diff| item.apply_diff(diff), |t| Tag::from(t));
            apply_collection_diff(&mut self.ports, &diff.ports, |item, diff| item.apply_diff(diff), |p| {
                Port::from(p)
            });

            let ports_map = self.ports_arc_map();
            apply_kit_types_patch(self, diff.types.as_ref(), &ports_map);

            let types_map = self.types_map_index();
            apply_kit_designs_patch(self, diff.designs.as_ref(), &types_map);

            apply_collection_diff(
                &mut self.qualities,
                &diff.qualities,
                |item, diff| item.apply_diff(diff),
                |q| Quality::from(q),
            );
            apply_collection_diff(&mut self.files, &diff.files, |item, diff| item.apply_diff(diff), |x| x.clone());
            apply_collection_diff(&mut self.folders, &diff.folders, |item, diff| item.apply_diff(diff), |f| {
                Folder::from(f)
            });
            apply_collection_diff(&mut self.authors, &diff.authors, |item, diff| item.apply_diff(diff), |a| {
                Author::from(a)
            });
            apply_collection_diff(
                &mut self.attributes,
                &diff.attributes,
                |item, diff| item.apply_diff(diff),
                |a| Attribute::from(a),
            );
        }
    }
} // ✈️ApplyDiff`;

content = before + newModule + after;
fs.writeFileSync(filePath, content, "utf-8");
console.log("Module replaced successfully");
console.log("New module length:", newModule.length);
