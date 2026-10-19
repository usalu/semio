//! 🏁️ Model build close: the completion take and the close stepping of the mounted model build. See [`super`].

use super::*;

//#region 🔖️Contract
impl MountedModelBuild {
    pub(super) fn take_complete(&mut self) -> Option<MountedAnalysisModel> {
        (self.stage == ModelBuildStage::Complete).then(|| std::mem::take(&mut self.model))
    }

    pub(super) fn attach_region(&mut self, region_cursor: usize, mesh: TriMesh2) {
        self.region_cursor = region_cursor;
        self.region_point_cursor = 0;
        self.region_triangle_cursor = 0;
        self.lookup_cursor = 0;
        self.mesh = Some(mesh);
        self.stage = ModelBuildStage::ReserveNodes;
        self.close_lane = 0;
    }

    pub(super) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        for owner in [&mut self.pending_node_id, &mut self.pending_element_id, &mut self.pending_element_start, &mut self.pending_element_end, &mut self.pending_region_id, &mut self.pending_region_element_id] {
            if let Some(value) = owner.as_mut() {
                let bytes = value.capacity();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                *value = String::new();
                *owner = None;
                return (false, 1, bytes);
            }
        }
        for owner in &mut self.pending_region_element_nodes {
            if let Some(value) = owner.as_mut() {
                let bytes = value.capacity();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                *value = String::new();
                *owner = None;
                return (false, 1, bytes);
            }
        }
        if let Some(support) = self.pending_support.as_mut() {
            let progress = support.close_step(maximum_bytes);
            if !progress.0 { return progress; }
            self.pending_support = None;
            return (false, 1, 0);
        }
        loop {
            let released_bytes = match self.close_lane {
                0..=6 => {
                    let progress = self.model.close_step(maximum_bytes);
                    if !progress.0 { return progress; }
                    self.close_lane = 7;
                    continue;
                }
                7 => {
                    let Some(owner) = self.region_node_ids.last_mut() else {
                        self.close_lane += 1;
                        continue;
                    };
                    if owner.capacity() != 0 {
                        let bytes = owner.capacity();
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        *owner = String::new();
                        return (false, 1, bytes);
                    }
                    self.region_node_ids.pop();
                    return (false, 1, 0);
                }
                8 => {
                    let bytes = self.region_node_ids.capacity() * size_of::<String>();
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    self.region_node_ids = Vec::new();
                    self.close_lane += 1;
                    bytes
                }
                9 => {
                    let Some(mesh) = self.mesh.as_mut() else {
                        self.close_lane += 2;
                        continue;
                    };
                    if mesh.points.pop().is_some() {
                        return (false, 1, 0);
                    }
                    let bytes = mesh.points.capacity() * size_of::<[f64; 2]>();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        mesh.points = Vec::new();
                        return (false, 1, bytes);
                    }
                    self.close_lane += 1;
                    continue;
                }
                10 => {
                    let Some(mesh) = self.mesh.as_mut() else {
                        self.close_lane += 1;
                        continue;
                    };
                    if mesh.tris.pop().is_some() {
                        return (false, 1, 0);
                    }
                    let bytes = mesh.tris.capacity() * size_of::<[u32; 3]>();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        mesh.tris = Vec::new();
                        return (false, 1, bytes);
                    }
                    self.mesh = None;
                    self.close_lane += 1;
                    0
                }
                _ => return (true, 0, 0),
            };
            return (false, 1, released_bytes);
        }
    }
}
//#endregion 🔖️Contract
