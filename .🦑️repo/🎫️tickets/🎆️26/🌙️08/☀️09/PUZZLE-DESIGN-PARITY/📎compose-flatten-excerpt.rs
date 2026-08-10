    //#region 🌤️FlattenDesign
    /// @emoji 🌤️ Computes absolute piece planes and centers from relative connections.
    pub mod flatten {
        use std::collections::{HashMap, HashSet, VecDeque};
        use std::sync::Arc;

        use crate::geom::{CoordinateInput, PlaneInput, PointInput, PositionInput, VectorInput};
        use crate::id::Id;
        use crate::kit::design::connection::Connection;
        use crate::kit::design::piece::Piece;
        use crate::kit::design::Design;
        use crate::kit::r#type::{Connector, Type};
        use crate::kit::Kit;

        const TOLERANCE: f64 = 0.01;
        const DIAGRAM_RADIUS: f64 = 2.697;
        const DIAGRAM_VERTICAL_V_EXTRA: f64 = 1.0;
        const DIAGRAM_HORIZONTAL_SCALE: f64 = 3.0633;

        fn normalize(v: &mut [f64; 3]) {
            let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            if len > 0.0 {
                v[0] /= len;
                v[1] /= len;
                v[2] /= len;
            }
        }

        fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
            [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
        }

        fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
            a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
        }

        fn deg_to_rad(deg: f64) -> f64 {
            deg * std::f64::consts::PI / 180.0
        }

        fn round_f(v: f64) -> f64 {
            (v * 1_000_000.0).round() / 1_000_000.0
        }

        fn plane_input_to_matrix(p: PlaneInput) -> [f64; 16] {
            let x = [p.x_axis.x, p.x_axis.y, p.x_axis.z];
            let y = [p.y_axis.x, p.y_axis.y, p.y_axis.z];
            let z = cross(x, y);
            [x[0], y[0], z[0], p.origin.x, x[1], y[1], z[1], p.origin.y, x[2], y[2], z[2], p.origin.z, 0.0, 0.0, 0.0, 1.0]
        }

        fn matrix_to_plane(m: [f64; 16]) -> PlaneInput {
            PlaneInput { origin: PointInput { x: m[3], y: m[7], z: m[11] }, x_axis: VectorInput { x: m[0], y: m[4], z: m[8] }, y_axis: VectorInput { x: m[1], y: m[5], z: m[9] } }
        }

        fn mul_mat(a: [f64; 16], b: [f64; 16]) -> [f64; 16] {
            let mut out = [0.0; 16];
            for col in 0..4 {
                for row in 0..4 {
                    out[col * 4 + row] = a[row] * b[col * 4] + a[4 + row] * b[col * 4 + 1] + a[8 + row] * b[col * 4 + 2] + a[12 + row] * b[col * 4 + 3];
                }
            }
            out
        }

        fn translation(x: f64, y: f64, z: f64) -> [f64; 16] {
            [1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0]
        }

        fn rotation_axis(axis: [f64; 3], angle: f64) -> [f64; 16] {
            let (x, y, z) = (axis[0], axis[1], axis[2]);
            let c = angle.cos();
            let s = angle.sin();
            let t = 1.0 - c;
            [t * x * x + c, t * x * y + s * z, t * x * z - s * y, 0.0, t * x * y - s * z, t * y * y + c, t * y * z + s * x, 0.0, t * x * z + s * y, t * y * z - s * x, t * z * z + c, 0.0, 0.0, 0.0, 0.0, 1.0]
        }

        fn apply_mat_vec3(m: [f64; 16], v: [f64; 3]) -> [f64; 3] {
            [m[0] * v[0] + m[4] * v[1] + m[8] * v[2], m[1] * v[0] + m[5] * v[1] + m[9] * v[2], m[2] * v[0] + m[6] * v[1] + m[10] * v[2]]
        }

        fn quaternion_from_unit_vectors(from: [f64; 3], to: [f64; 3]) -> [f64; 4] {
            let r = dot(from, to) + 1.0;
            let quat = if r < 0.000_001 {
                if from[0].abs() > from[2].abs() {
                    [-from[1], from[0], 0.0, 0.0]
                } else {
                    [0.0, -from[2], from[1], 0.0]
                }
            } else {
                let c = cross(from, to);
                [c[0], c[1], c[2], r]
            };
            let len = (quat[0] * quat[0] + quat[1] * quat[1] + quat[2] * quat[2] + quat[3] * quat[3]).sqrt();
            [quat[0] / len, quat[1] / len, quat[2] / len, quat[3] / len]
        }

        fn quaternion_to_matrix(q: [f64; 4]) -> [f64; 16] {
            let (x, y, z, w) = (q[0], q[1], q[2], q[3]);
            let (x2, y2, z2) = (x + x, y + y, z + z);
            let (xx, xy, xz) = (x * x2, x * y2, x * z2);
            let (yy, yz, zz) = (y * y2, y * z2, z * z2);
            let (wx, wy, wz) = (w * x2, w * y2, w * z2);
            [1.0 - (yy + zz), xy + wz, xz - wy, 0.0, xy - wz, 1.0 - (xx + zz), yz + wx, 0.0, xz + wy, yz - wx, 1.0 - (xx + yy), 0.0, 0.0, 0.0, 0.0, 1.0]
        }

        async fn connector_geom(c: &Arc<Connector>) -> (PointInput, VectorInput, f64) {
            let point = *c.point.read().await;
            let mut direction = *c.direction.read().await;
            let mut dir = [direction.x, direction.y, direction.z];
            normalize(&mut dir);
            direction = VectorInput { x: dir[0], y: dir[1], z: dir[2] };
            let t = c.t_param.read().await.unwrap_or(0.0);
            (point, direction, t)
        }

        async fn compute_child_plane(parent_plane: PlaneInput, parent_connector: &Arc<Connector>, child_connector: &Arc<Connector>, connection: &Arc<Connection>) -> PlaneInput {
            let parent_matrix = plane_input_to_matrix(parent_plane);
            let (parent_point, parent_direction, _) = connector_geom(parent_connector).await;
            let (child_point, child_direction, _) = connector_geom(child_connector).await;
            let mut parent_dir = [parent_direction.x, parent_direction.y, parent_direction.z];
            let mut child_dir = [child_direction.x, child_direction.y, child_direction.z];
            normalize(&mut parent_dir);
            normalize(&mut child_dir);
            let gap = connection.gap.read().await.unwrap_or(0.0);
            let shift = connection.shift.read().await.unwrap_or(0.0);
            let rise = connection.rise.read().await.unwrap_or(0.0);
            let rotation_rad = deg_to_rad(connection.rotation.read().await.unwrap_or(0.0));
            let turn_rad = deg_to_rad(connection.turn.read().await.unwrap_or(0.0));
            let tilt_rad = deg_to_rad(connection.tilt.read().await.unwrap_or(0.0));
            let reverse_child = [-child_dir[0], -child_dir[1], -child_dir[2]];
            let cross_vec = cross(parent_dir, reverse_child);
            let cross_len = (cross_vec[0] * cross_vec[0] + cross_vec[1] * cross_vec[1] + cross_vec[2] * cross_vec[2]).sqrt();
            let align_quat = if cross_len < TOLERANCE {
                if parent_dir[2].abs() < TOLERANCE {
                    quaternion_from_unit_vectors([0.0, 1.0, 0.0], [0.0, 0.0, -1.0])
                } else {
                    let mut axis = cross([0.0, 0.0, 1.0], parent_dir);
                    normalize(&mut axis);
                    let half = std::f64::consts::PI / 2.0;
                    [axis[0] * half.sin(), axis[1] * half.sin(), axis[2] * half.sin(), half.cos()]
                }
            } else {
                quaternion_from_unit_vectors(reverse_child, parent_dir)
            };
            let direction_t = quaternion_to_matrix(align_quat);
            let y_axis = [0.0, 1.0, 0.0];
            let parent_rotation_t = quaternion_to_matrix(quaternion_from_unit_vectors(y_axis, parent_dir));
            let gap_direction = apply_mat_vec3(parent_rotation_t, [0.0, 1.0, 0.0]);
            let shift_direction = apply_mat_vec3(parent_rotation_t, [1.0, 0.0, 0.0]);
            let raise_direction = apply_mat_vec3(parent_rotation_t, [0.0, 0.0, 1.0]);
            let mut turn_axis = apply_mat_vec3(parent_rotation_t, [0.0, 0.0, 1.0]);
            let mut tilt_axis = apply_mat_vec3(parent_rotation_t, [1.0, 0.0, 0.0]);
            let mut orientation_t = direction_t;
            let rotate_t = rotation_axis(parent_dir, -rotation_rad);
            orientation_t = mul_mat(rotate_t, orientation_t);
            turn_axis = apply_mat_vec3(rotate_t, turn_axis);
            tilt_axis = apply_mat_vec3(rotate_t, tilt_axis);
            orientation_t = mul_mat(rotation_axis(turn_axis, turn_rad), orientation_t);
            orientation_t = mul_mat(rotation_axis(tilt_axis, tilt_rad), orientation_t);
            let center_child_t = translation(-child_point.x, -child_point.y, -child_point.z);
            let mut transform = mul_mat(orientation_t, center_child_t);
            let gap_transform = translation(gap_direction[0] * gap, gap_direction[1] * gap, gap_direction[2] * gap);
            let shift_transform = translation(shift_direction[0] * shift, shift_direction[1] * shift, shift_direction[2] * shift);
            let raise_transform = translation(raise_direction[0] * rise, raise_direction[1] * rise, raise_direction[2] * rise);
            transform = mul_mat(mul_mat(raise_transform, mul_mat(shift_transform, gap_transform)), transform);
            transform = mul_mat(translation(parent_point.x, parent_point.y, parent_point.z), transform);
            matrix_to_plane(mul_mat(parent_matrix, transform))
        }

        async fn resolve_connector(ty: Option<&Arc<Type>>, connector_id: Option<&Id>, kit: &Arc<Kit>) -> Option<Arc<Connector>> {
            if let Some(id) = connector_id {
                if let Some(c) = kit.find_connector(id).await {
                    return Some(c);
                }
                if let Some(t) = ty {
                    for c in t.has_connectors().await {
                        if &c.id == id {
                            return Some(c);
                        }
                    }
                }
            }
            if let Some(t) = ty {
                return t.has_connectors().await.into_iter().next();
            }
            None
        }

        async fn piece_stored_position(piece: &Arc<Piece>) -> Option<PositionInput> {
            if let Some(n) = piece.position.read().await.as_ref() {
                return Some(n.snapshot_input().await);
            }
            None
        }

        async fn piece_is_fixed(piece: &Arc<Piece>) -> bool {
            matches!(*piece.connection_kind.read().await, Some(crate::kit::design::piece::PieceConnectionKind::Fixed))
        }

        /// @emoji 🌤️ Absolute positions for every piece in a design.
        pub async fn flatten_design_positions(kit: &Arc<Kit>, design: &Arc<Design>) -> HashMap<Id, PositionInput> {
            let pieces = design.has_pieces().await;
            if pieces.is_empty() {
                return HashMap::new();
            }
            let mut piece_map: HashMap<String, Arc<Piece>> = HashMap::new();
            for p in &pieces {
                piece_map.insert(p.id.as_str().to_string(), p.clone());
            }
            let connections = design.has_connections().await;
            let mut adjacency: HashMap<String, Vec<(String, Arc<Connection>)>> = HashMap::new();
            for conn in &connections {
                let parent_id = conn.parent.read().await.references_piece().await.id.as_str().to_string();
                let child_id = conn.child.read().await.references_piece().await.id.as_str().to_string();
                if piece_map.contains_key(&parent_id) && piece_map.contains_key(&child_id) {
                    adjacency.entry(parent_id.clone()).or_default().push((child_id.clone(), conn.clone()));
                    adjacency.entry(child_id.clone()).or_default().push((parent_id.clone(), conn.clone()));
                }
            }
            let mut original_centers: HashMap<String, CoordinateInput> = HashMap::new();
            for p in &pieces {
                if let Some(pos) = piece_stored_position(p).await {
                    original_centers.insert(p.id.as_str().to_string(), pos.center);
                }
            }
            let mut piece_planes: HashMap<String, PlaneInput> = HashMap::new();
            let mut piece_centers: HashMap<String, CoordinateInput> = HashMap::new();
            let mut visited: HashSet<String> = HashSet::new();

            async fn bfs_root(
                root_id: &str,
                piece_map: &HashMap<String, Arc<Piece>>,
                adjacency: &HashMap<String, Vec<(String, Arc<Connection>)>>,
                kit: &Arc<Kit>,
                visited: &mut HashSet<String>,
                piece_planes: &mut HashMap<String, PlaneInput>,
                piece_centers: &mut HashMap<String, CoordinateInput>,
                _original_centers: &HashMap<String, CoordinateInput>,
            ) {
                let mut queue: VecDeque<String> = VecDeque::new();
                queue.push_back(root_id.to_string());
                visited.insert(root_id.to_string());
                let root_piece = piece_map.get(root_id).expect("root_id is drawn from the same `pieces` list piece_map was populated from");
                if let Some(pos) = piece_stored_position(root_piece).await {
                    if piece_is_fixed(root_piece).await {
                        piece_planes.insert(root_id.to_string(), pos.plane);
                        piece_centers.insert(root_id.to_string(), pos.center);
                    } else {
                        piece_planes.insert(root_id.to_string(), PlaneInput::default());
                        piece_centers.insert(root_id.to_string(), pos.center);
                    }
                } else {
                    piece_planes.insert(root_id.to_string(), PlaneInput::default());
                    piece_centers.insert(root_id.to_string(), CoordinateInput::default());
                }
                while let Some(current_id) = queue.pop_front() {
                    let current_plane = *piece_planes.get(&current_id).unwrap_or(&PlaneInput::default());
                    let current_piece = piece_map.get(&current_id).expect("queue only ever holds ids sourced from piece_map/adjacency").clone();
                    let parent_center = piece_centers.get(&current_id).copied().unwrap_or_default();
                    for (neighbor_id, conn) in adjacency.get(&current_id).into_iter().flatten() {
                        if visited.contains(neighbor_id) {
                            continue;
                        }
                        visited.insert(neighbor_id.clone());
                        let neighbor_piece = piece_map.get(neighbor_id).expect("adjacency only links ids already verified present in piece_map").clone();
                        let parent_side = conn.parent.read().await.clone();
                        let child_side = conn.child.read().await.clone();
                        let (parent_piece_id, _child_piece_id) = (parent_side.references_piece().await.id.as_str().to_string(), child_side.references_piece().await.id.as_str().to_string());
                        let (parent_side_ref, child_side_ref) = if parent_piece_id == current_id { (&parent_side, &child_side) } else { (&child_side, &parent_side) };
                        let parent_ty = current_piece.is_type().await;
                        let child_ty = neighbor_piece.is_type().await;
                        let parent_connector = resolve_connector(parent_ty.as_ref(), parent_side_ref.references_connector().await.as_ref().map(|c| &c.id), kit).await;
                        let child_connector = resolve_connector(child_ty.as_ref(), child_side_ref.references_connector().await.as_ref().map(|c| &c.id), kit).await;
                        let (Some(parent_connector), Some(child_connector)) = (parent_connector, child_connector) else {
                            piece_planes.insert(neighbor_id.clone(), PlaneInput::default());
                            piece_centers.insert(neighbor_id.clone(), CoordinateInput::default());
                            queue.push_back(neighbor_id.clone());
                            continue;
                        };
                        let child_plane = compute_child_plane(current_plane, &parent_connector, &child_connector, conn).await;
                        piece_planes.insert(neighbor_id.clone(), child_plane);
                        let (_, parent_direction, parent_t) = connector_geom(&parent_connector).await;
                        let connection_u = conn.u.read().await.unwrap_or(0.0);
                        let connection_v = conn.v.read().await.unwrap_or(0.0);
                        let (child_u, child_v) = if parent_center.u == 0.0 && parent_center.v == 0.0 {
                            let angle = 2.0 * std::f64::consts::PI * parent_t;
                            (DIAGRAM_RADIUS * angle.sin(), DIAGRAM_RADIUS * angle.cos())
                        } else if parent_direction.z.abs() > 0.5 {
                            (parent_center.u + connection_u, parent_center.v + connection_v + DIAGRAM_VERTICAL_V_EXTRA)
                        } else {
                            (parent_center.u + connection_u * DIAGRAM_HORIZONTAL_SCALE, parent_center.v + connection_v * DIAGRAM_HORIZONTAL_SCALE)
                        };
                        piece_centers.insert(neighbor_id.clone(), CoordinateInput { u: round_f(child_u), v: round_f(child_v) });
                        queue.push_back(neighbor_id.clone());
                    }
                }
            }

            for p in &pieces {
                let pid = p.id.as_str().to_string();
                if !visited.contains(&pid) {
                    bfs_root(&pid, &piece_map, &adjacency, kit, &mut visited, &mut piece_planes, &mut piece_centers, &original_centers).await;
                }
            }
            let mut out = HashMap::new();
            for p in &pieces {
                let pid = p.id.clone();
                let plane = piece_planes.get(p.id.as_str()).copied().unwrap_or_default();
                let center = piece_centers.get(p.id.as_str()).copied().or_else(|| original_centers.get(p.id.as_str()).copied()).unwrap_or_default();
                out.insert(pid, PositionInput { center, plane });
            }
            out
        }
    }
