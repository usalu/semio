//! Incremental flatten spanning tree over a design. Geometry caches live on [`FlattenPieceState`], not on the shared graph object.

use super::*;
use nalgebra::Matrix4;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::rc::{Rc, Weak};

/// Affine helpers used by [`Connection::child_plane_matrix`].
pub struct FlattenAffine;

impl FlattenAffine {
    pub fn quat_to_matrix4(q: &nalgebra::UnitQuaternion<f64>) -> Matrix4<f64> {
        let rot = q.to_rotation_matrix();
        let m = rot.matrix();
        Matrix4::new(
            m[(0, 0)],
            m[(0, 1)],
            m[(0, 2)],
            0.0,
            m[(1, 0)],
            m[(1, 1)],
            m[(1, 2)],
            0.0,
            m[(2, 0)],
            m[(2, 1)],
            m[(2, 2)],
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix4<f64> {
        Matrix4::new(
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn apply_mat_vec3(
        m: &Matrix4<f64>,
        v: &nalgebra::Vector3<f64>,
    ) -> nalgebra::Vector3<f64> {
        nalgebra::Vector3::new(
            m[(0, 0)] * v.x + m[(0, 1)] * v.y + m[(0, 2)] * v.z,
            m[(1, 0)] * v.x + m[(1, 1)] * v.y + m[(1, 2)] * v.z,
            m[(2, 0)] * v.x + m[(2, 1)] * v.y + m[(2, 2)] * v.z,
        )
    }
}

pub struct PieceFlattenParent<'a> {
    pub parent_piece_guid: &'a str,
    pub conn: &'a Connection,
    pub from_connected: bool,
}

/// Shared graph structure (incremental BFS); **does not** store centralized matrix/center caches.
pub struct FlattenGraphInner<'a> {
    kit: &'a Kit,
    design: &'a Design,
    pieces_map: HashMap<&'a str, &'a Piece>,
    adjacency: HashMap<String, Vec<(String, &'a Connection, bool)>>,
    bfs_queue: RefCell<VecDeque<String>>,
    bfs_visited: RefCell<HashSet<String>>,
    next_seed_index: RefCell<usize>,
    parent_of: RefCell<HashMap<String, PieceFlattenParent<'a>>>,
    children_of: RefCell<HashMap<String, Vec<String>>>,
    piece_paths: RefCell<HashMap<String, String>>,
    nodes: RefCell<HashMap<String, Rc<FlattenPieceState<'a>>>>,
}

/// Per-piece flatten state and **local** memoization for derived geometry.
pub struct FlattenPieceState<'a> {
    graph: Weak<FlattenGraphInner<'a>>,
    guid: String,
    matrix_memo: RefCell<Option<Matrix4<f64>>>,
    center_memo: RefCell<Option<Coord>>,
}

impl<'a> FlattenPieceState<'a> {
    pub fn guid(&self) -> &str {
        &self.guid
    }

    pub fn clear_geometry_memo(&self) {
        *self.matrix_memo.borrow_mut() = None;
        *self.center_memo.borrow_mut() = None;
    }

    pub fn world_matrix(&self) -> Matrix4<f64> {
        if let Some(m) = *self.matrix_memo.borrow() {
            return m;
        }
        let graph_rc = match self.graph.upgrade() {
            Some(g) => g,
            None => return Matrix4::identity(),
        };
        graph_rc.ensure_piece_reachable(&self.guid);
        let piece = match graph_rc.pieces_map.get(self.guid.as_str()) {
            Some(p) => *p,
            None => return Matrix4::identity(),
        };
        let m = if let Some(edge) = graph_rc.parent_of.borrow().get(&self.guid) {
            let parent = FlattenGraphInner::piece_state(&graph_rc, edge.parent_piece_guid);
            let conn_m = graph_rc
                .kit
                .connection_matrix_fast(&graph_rc.pieces_map, edge.conn, edge.from_connected)
                .unwrap_or_else(Matrix4::identity);
            parent.world_matrix() * conn_m
        } else {
            piece
                .plane
                .as_ref()
                .map(|p| p.to_matrix())
                .unwrap_or_else(Matrix4::identity)
        };
        *self.matrix_memo.borrow_mut() = Some(m);
        m
    }

    pub fn flat_plane(&self) -> Plane {
        Plane::from_matrix(&self.world_matrix()).round()
    }

    pub fn flat_center(&self) -> Coord {
        if let Some(c) = self.center_memo.borrow().as_ref() {
            return c.clone();
        }
        let graph_rc = match self.graph.upgrade() {
            Some(g) => g,
            None => return Coord { u: 0.0, v: 0.0 },
        };
        graph_rc.ensure_piece_reachable(&self.guid);
        let piece = match graph_rc.pieces_map.get(self.guid.as_str()) {
            Some(p) => *p,
            None => return Coord { u: 0.0, v: 0.0 },
        };
        const RADIUS: f64 = 2.697;
        const VERTICAL_V_EXTRA: f64 = 1.0;
        const HORIZONTAL_SCALE: f64 = 3.0633;
        let c = if let Some(edge) = graph_rc.parent_of.borrow().get(&self.guid) {
            let parent = FlattenGraphInner::piece_state(&graph_rc, edge.parent_piece_guid);
            let parent_center = parent.flat_center();
            let (parent_side, _child_side) = if edge.from_connected {
                (&edge.conn.connected, &edge.conn.connecting)
            } else {
                (&edge.conn.connecting, &edge.conn.connected)
            };
            let parent_connector = graph_rc
                .kit
                .connector_for_side_fast(&graph_rc.pieces_map, parent_side)
                .expect("parent connector validated when tree was built");
            let conn_u = edge.conn.u.unwrap_or(0.0);
            let conn_v = edge.conn.v.unwrap_or(0.0);
            let (child_u, child_v) = if parent_center.u.abs() < 0.0001 && parent_center.v.abs() < 0.0001
            {
                let angle = 2.0 * PI * parent_connector.t;
                (RADIUS * angle.sin(), RADIUS * angle.cos())
            } else {
                let is_vertical = parent_connector.direction.z.abs() > 0.5;
                if is_vertical {
                    (
                        parent_center.u + conn_u,
                        parent_center.v + conn_v + VERTICAL_V_EXTRA,
                    )
                } else {
                    (
                        parent_center.u + conn_u * HORIZONTAL_SCALE,
                        parent_center.v + conn_v * HORIZONTAL_SCALE,
                    )
                }
            };
            Coord {
                u: (child_u * 1_000_000.0).round() / 1_000_000.0,
                v: (child_v * 1_000_000.0).round() / 1_000_000.0,
            }
        } else {
            piece.center.clone().unwrap_or(Coord { u: 0.0, v: 0.0 })
        };
        *self.center_memo.borrow_mut() = Some(c.clone());
        c
    }
}

impl<'a> FlattenGraphInner<'a> {
    fn piece_state(graph: &Rc<FlattenGraphInner<'a>>, guid: &str) -> Rc<FlattenPieceState<'a>> {
        graph
            .nodes
            .borrow_mut()
            .entry(guid.to_string())
            .or_insert_with(|| {
                Rc::new(FlattenPieceState {
                    graph: Rc::downgrade(graph),
                    guid: guid.to_string(),
                    matrix_memo: RefCell::new(None),
                    center_memo: RefCell::new(None),
                })
            })
            .clone()
    }

    pub fn ensure_piece_reachable(&self, piece_guid: &str) {
        loop {
            if self.parent_of.borrow().contains_key(piece_guid) {
                return;
            }
            if self.bfs_visited.borrow().contains(piece_guid) {
                return;
            }
            if !self.expand_one_bfs_step() {
                return;
            }
        }
    }

    fn expand_one_bfs_step(&self) -> bool {
        let pieces = self.design.pieces.as_ref().map(|p| p.as_slice()).unwrap_or(&[]);

        let mut queue = self.bfs_queue.borrow_mut();
        if queue.is_empty() {
            let mut idx = self.next_seed_index.borrow_mut();
            while *idx < pieces.len() {
                let p = &pieces[*idx];
                *idx += 1;
                if !self.bfs_visited.borrow().contains(&p.guid) {
                    self.bfs_visited.borrow_mut().insert(p.guid.clone());
                    self.piece_paths
                        .borrow_mut()
                        .insert(p.guid.clone(), p.guid.clone());
                    queue.push_back(p.guid.clone());
                    return true;
                }
            }
            return false;
        }

        let current_guid = queue.pop_front().expect("queue non-empty");
        drop(queue);

        let neighbors = self.adjacency.get(&current_guid).cloned().unwrap_or_default();
        for (neighbor_guid, conn, is_connected) in neighbors {
            if self.bfs_visited.borrow().contains(&neighbor_guid) {
                continue;
            }
            let (parent_side, _child_side) = if is_connected {
                (&conn.connected, &conn.connecting)
            } else {
                (&conn.connecting, &conn.connected)
            };
            if self
                .kit
                .connector_for_side_fast(&self.pieces_map, parent_side)
                .is_none()
            {
                continue;
            }
            if self
                .kit
                .connection_matrix_fast(&self.pieces_map, conn, is_connected)
                .is_none()
            {
                continue;
            }
            let parent_piece_guid = self
                .pieces_map
                .get(current_guid.as_str())
                .expect("current vertex is a design piece")
                .guid
                .as_str();
            self.parent_of.borrow_mut().insert(
                neighbor_guid.clone(),
                PieceFlattenParent {
                    parent_piece_guid,
                    conn,
                    from_connected: is_connected,
                },
            );
            self.children_of
                .borrow_mut()
                .entry(current_guid.clone())
                .or_default()
                .push(neighbor_guid.clone());

            let parent_path = self
                .piece_paths
                .borrow()
                .get(&current_guid)
                .cloned()
                .unwrap_or_default();
            self.piece_paths.borrow_mut().insert(
                neighbor_guid.clone(),
                format!("{},{}", parent_path, neighbor_guid),
            );
            self.bfs_visited.borrow_mut().insert(neighbor_guid.clone());
            self.bfs_queue.borrow_mut().push_back(neighbor_guid);
        }
        true
    }

    pub fn parent_piece_guid(&self, piece_guid: &str) -> Option<&'a str> {
        self.ensure_piece_reachable(piece_guid);
        self.parent_of
            .borrow()
            .get(piece_guid)
            .map(|e| e.parent_piece_guid)
    }

    pub fn semio_path_string(&self, piece_guid: &str) -> Option<String> {
        self.ensure_piece_reachable(piece_guid);
        self.piece_paths.borrow().get(piece_guid).cloned()
    }
}

/// Resolve flattened geometry for one design; coordinates lazy tree expansion and delegates caching to [`FlattenPieceState`].
pub struct FlattenDesign<'a> {
    inner: Rc<FlattenGraphInner<'a>>,
}

impl<'a> FlattenDesign<'a> {
    pub fn try_new(kit: &'a Kit, design_guid: &str) -> Option<Self> {
        let design = kit.design_by_guid(design_guid)?;
        let pieces = design.pieces.as_ref().map(|p| p.as_slice()).unwrap_or(&[]);
        let connections = design.connections.as_ref().map(|c| c.as_slice()).unwrap_or(&[]);
        let pieces_map: HashMap<&str, &Piece> =
            pieces.iter().map(|p| (p.guid.as_str(), p)).collect();

        let mut adjacency: HashMap<String, Vec<(String, &'a Connection, bool)>> = HashMap::new();
        for conn in connections {
            let src = conn.connected.piece.guid.as_str();
            let tgt = conn.connecting.piece.guid.as_str();
            if pieces_map.contains_key(src) && pieces_map.contains_key(tgt) {
                adjacency
                    .entry(src.to_string())
                    .or_default()
                    .push((tgt.to_string(), conn, true));
                adjacency
                    .entry(tgt.to_string())
                    .or_default()
                    .push((src.to_string(), conn, false));
            }
        }

        Some(Self {
            inner: Rc::new(FlattenGraphInner {
                kit,
                design,
                pieces_map,
                adjacency,
                bfs_queue: RefCell::new(VecDeque::new()),
                bfs_visited: RefCell::new(HashSet::new()),
                next_seed_index: RefCell::new(0),
                parent_of: RefCell::new(HashMap::new()),
                children_of: RefCell::new(HashMap::new()),
                piece_paths: RefCell::new(HashMap::new()),
                nodes: RefCell::new(HashMap::new()),
            }),
        })
    }

    pub fn design(&self) -> &'a Design {
        self.inner.design
    }

    pub fn piece(&self, piece_guid: &str) -> FlattenPiece<'a> {
        self.inner.ensure_piece_reachable(piece_guid);
        FlattenPiece {
            state: FlattenGraphInner::piece_state(&self.inner, piece_guid),
        }
    }

    pub fn expand_entire_design(&self) {
        while self.inner.expand_one_bfs_step() {}
    }

    pub fn invalidate_flat_geometry_below(&self, piece_guid: &str) {
        let mut stack = vec![piece_guid.to_string()];
        while let Some(g) = stack.pop() {
            if let Some(n) = self.inner.nodes.borrow().get(&g) {
                n.clear_geometry_memo();
            }
            if let Some(chs) = self.inner.children_of.borrow().get(&g) {
                for c in chs.clone() {
                    stack.push(c.clone());
                }
            }
        }
    }

    pub fn to_design_change(&self) -> DesignChange {
        self.expand_entire_design();
        let design_guid = self.inner.design.guid.clone();
        let before_design = self.inner.design.clone();
        let pieces = self
            .inner
            .design
            .pieces
            .as_ref()
            .map(|p| p.as_slice())
            .unwrap_or(&[]);
        let mut updated_pieces: Vec<DiffUpdate<PieceDiff>> = Vec::new();

        for piece in pieces {
            let fp = FlattenPiece {
                state: FlattenGraphInner::piece_state(&self.inner, piece.guid.as_str()),
            };
            let new_plane = fp.flat_plane();
            let center = fp.flat_center();

            let plane_needs_update = match &piece.plane {
                Some(existing) => !existing.approx_eq(&new_plane),
                None => true,
            };

            let center_needs_update = match &piece.center {
                Some(existing) => {
                    (existing.u - center.u).abs() > 0.0001
                        || (existing.v - center.v).abs() > 0.0001
                }
                None => true,
            };

            if plane_needs_update || center_needs_update {
                let path_attr = self.inner.semio_path_string(piece.guid.as_str()).map(|path| {
                    CollectionDiff {
                        added: Some(vec![Attribute {
                            guid: guid(),
                            key: "semio.path".to_string(),
                            value: Some(path),
                            definition: None,
                        }]),
                        removed: None,
                        updated: None,
                    }
                });
                updated_pieces.push(DiffUpdate {
                    key: "piece".to_string(),
                    guid: piece.guid.clone(),
                    diff: PieceDiff {
                        guid: piece.guid.clone(),
                        plane: if plane_needs_update {
                            Some(Some(new_plane))
                        } else {
                            None
                        },
                        center: if center_needs_update {
                            Some(Some(center.clone()))
                        } else {
                            None
                        },
                        attributes: path_attr,
                        ..Default::default()
                    },
                });
            }
        }

        let mut forward = DesignDiff {
            guid: design_guid.clone(),
            ..Default::default()
        };

        if !updated_pieces.is_empty() {
            forward.pieces = Some(CollectionDiff {
                added: None,
                removed: None,
                updated: Some(updated_pieces),
            });
        }

        let mut after_design = before_design.clone();
        apply_design_diff(&mut after_design, &forward);
        let backward = after_design.diff_from(&before_design);

        DesignChange {
            forward,
            backward,
            author: None,
            time: None,
            before: Some(before_design),
            after: Some(after_design),
        }
    }

    pub fn merkle_hashes(&self) -> HashMap<String, FlatMerkleHashes> {
        let pieces = self
            .inner
            .design
            .pieces
            .as_ref()
            .map(|p| p.as_slice())
            .unwrap_or(&[]);
        if pieces.is_empty() {
            return HashMap::new();
        }
        let connections = self
            .inner
            .design
            .connections
            .as_ref()
            .map(|c| c.as_slice())
            .unwrap_or(&[]);
        let pieces_map: HashMap<&str, &Piece> =
            pieces.iter().map(|p| (p.guid.as_str(), p)).collect();

        let mut adjacency: HashMap<&str, Vec<(&str, &Connection, bool)>> = HashMap::new();
        for conn in connections {
            let src = conn.connected.piece.guid.as_str();
            let tgt = conn.connecting.piece.guid.as_str();
            if pieces_map.contains_key(src) && pieces_map.contains_key(tgt) {
                adjacency.entry(src).or_default().push((tgt, conn, true));
                adjacency.entry(tgt).or_default().push((src, conn, false));
            }
        }

        let mut components: Vec<Vec<&str>> = Vec::new();
        {
            let mut component_visited: HashSet<&str> = HashSet::new();
            for piece in pieces {
                let guid = piece.guid.as_str();
                if component_visited.contains(guid) {
                    continue;
                }
                let mut component = Vec::new();
                let mut queue: VecDeque<&str> = VecDeque::new();
                queue.push_back(guid);
                component_visited.insert(guid);
                while let Some(cur) = queue.pop_front() {
                    component.push(cur);
                    if let Some(neighbors) = adjacency.get(cur) {
                        for &(neigh, _, _) in neighbors {
                            if !component_visited.contains(neigh) {
                                component_visited.insert(neigh);
                                queue.push_back(neigh);
                            }
                        }
                    }
                }
                components.push(component);
            }
        }

        let mut plane_hashes: HashMap<String, String> = HashMap::new();
        let mut center_hashes: HashMap<String, String> = HashMap::new();

        for component in &components {
            let component_set: HashSet<&str> = component.iter().copied().collect();

            let mut root: Option<&str> = None;
            for piece in pieces {
                let g = piece.guid.as_str();
                if component_set.contains(g) && piece.plane.is_some() && piece.center.is_some() {
                    root = Some(g);
                    break;
                }
            }
            if root.is_none() {
                let mut sorted: Vec<&str> = component.iter().copied().collect();
                sorted.sort();
                root = sorted.first().copied();
            }
            let root_guid = match root {
                Some(g) => g,
                None => continue,
            };
            let root_piece = match pieces_map.get(root_guid) {
                Some(p) => *p,
                None => continue,
            };
            plane_hashes.insert(
                root_guid.to_string(),
                Plane::flatten_merkle_root_hash(root_guid, root_piece.plane.as_ref()),
            );
            center_hashes.insert(
                root_guid.to_string(),
                Coord::flatten_merkle_root_hash(root_guid, root_piece.center.as_ref()),
            );

            let mut bfs_visited: HashSet<&str> = HashSet::new();
            bfs_visited.insert(root_guid);
            let mut queue: VecDeque<&str> = VecDeque::new();
            queue.push_back(root_guid);
            while let Some(cur) = queue.pop_front() {
                let parent_plane_hash = match plane_hashes.get(cur).cloned() {
                    Some(h) => h,
                    None => continue,
                };
                let parent_center_hash = match center_hashes.get(cur).cloned() {
                    Some(h) => h,
                    None => continue,
                };
                if let Some(neighbors) = adjacency.get(cur) {
                    for &(neigh, conn, is_connected) in neighbors {
                        if bfs_visited.contains(neigh) {
                            continue;
                        }
                        let (parent_side, child_side) = if is_connected {
                            (&conn.connected, &conn.connecting)
                        } else {
                            (&conn.connecting, &conn.connected)
                        };
                        let parent_connector =
                            match self.inner.kit.connector_for_side_fast(&pieces_map, parent_side)
                            {
                                Some(c) => c,
                                None => continue,
                            };
                        let child_connector = match self
                            .inner
                            .kit
                            .connector_for_side_fast(&pieces_map, child_side)
                        {
                            Some(c) => c,
                            None => continue,
                        };
                        plane_hashes.insert(
                            neigh.to_string(),
                            conn.flatten_merkle_plane_chain_hash(
                                &parent_plane_hash,
                                &parent_connector,
                                &child_connector,
                            ),
                        );
                        center_hashes.insert(
                            neigh.to_string(),
                            conn.flatten_merkle_center_chain_hash(
                                &parent_center_hash,
                                &parent_connector,
                            ),
                        );
                        bfs_visited.insert(neigh);
                        queue.push_back(neigh);
                    }
                }
            }
        }

        let mut result: HashMap<String, FlatMerkleHashes> = HashMap::new();
        for (guid, plane_hash) in plane_hashes {
            if let Some(center_hash) = center_hashes.get(&guid).cloned() {
                result.insert(
                    guid,
                    FlatMerkleHashes {
                        plane_hash,
                        center_hash,
                    },
                );
            }
        }
        result
    }
}

pub struct FlattenPiece<'a> {
    pub(crate) state: Rc<FlattenPieceState<'a>>,
}

impl<'a> Clone for FlattenPiece<'a> {
    fn clone(&self) -> Self {
        FlattenPiece {
            state: Rc::clone(&self.state),
        }
    }
}

impl<'a> FlattenPiece<'a> {
    pub fn guid(&self) -> &str {
        self.state.guid()
    }

    pub fn world_matrix(&self) -> Matrix4<f64> {
        self.state.world_matrix()
    }

    pub fn flat_plane(&self) -> Plane {
        self.state.flat_plane()
    }

    pub fn flat_center(&self) -> Coord {
        self.state.flat_center()
    }

    pub fn parent(&self) -> Option<FlattenPiece<'a>> {
        let g = self.state.graph.upgrade()?;
        g.ensure_piece_reachable(&self.state.guid);
        g.parent_of.borrow().get(&self.state.guid).map(|e| FlattenPiece {
            state: FlattenGraphInner::piece_state(&g, e.parent_piece_guid),
        })
    }

    pub fn children(&self) -> Vec<FlattenPiece<'a>> {
        let g = match self.state.graph.upgrade() {
            Some(inner) => inner,
            None => return vec![],
        };
        g.ensure_piece_reachable(&self.state.guid);
        g.children_of
            .borrow()
            .get(&self.state.guid)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|guid| FlattenPiece {
                state: FlattenGraphInner::piece_state(&g, &guid),
            })
            .collect()
    }
}
