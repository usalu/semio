//! 🏁️ Model build close: the completion take and the close stepping of the mounted model build. See [`super`].

use super::*;

//#region 🔖️Contract
impl MountedModelBuild {
    pub(super) fn take_complete(&mut self) -> Option<AnalysisModel> {
        (self.stage == ModelBuildStage::Complete).then(|| std::mem::replace(&mut self.model, AnalysisModel { nodes: Vec::new(), elements: Vec::new(), supports: Vec::new() }))
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
            if support.node_id.capacity() != 0 {
                let bytes = support.node_id.capacity();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                support.node_id = String::new();
                return (false, 1, bytes);
            }
            if support.fixed.pop().is_some() {
                return (false, 1, 0);
            }
            let bytes = support.fixed.capacity() * size_of::<semio_s_fem_model::Dof>();
            if bytes != 0 {
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                support.fixed = Vec::new();
                return (false, 1, bytes);
            }
            self.pending_support = None;
            return (false, 1, 0);
        }
        loop {
            let released_bytes = match self.close_lane {
                0 => {
                    let Some(node) = self.model.nodes.last_mut() else {
                        self.close_lane += 1;
                        continue;
                    };
                    if node.id.capacity() != 0 {
                        let bytes = node.id.capacity();
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        node.id = String::new();
                        return (false, 1, bytes);
                    }
                    self.model.nodes.pop();
                    return (false, 1, 0);
                }
                1 => {
                    let bytes = self.model.nodes.capacity() * size_of::<semio_s_fem_model::Node>();
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    self.model.nodes = Vec::new();
                    self.close_lane += 1;
                    bytes
                }
                2 => {
                    let Some(element) = self.model.elements.last_mut() else {
                        self.close_lane += 1;
                        continue;
                    };
                    if let Some(next_bytes) = element.mounted_next_string_bytes() {
                        if next_bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        let bytes = element.close_mounted_string_step().expect("mounted element next-owner witness changed without mutation");
                        return (false, 1, bytes);
                    }
                    if !element.mounted_strings_terminal_is_empty() {
                        return (false, 0, 0);
                    }
                    self.model.elements.pop();
                    return (false, 1, 0);
                }
                3 => {
                    let bytes = self.model.elements.capacity() * size_of::<semio_s_fem_static::Elements>();
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    self.model.elements = Vec::new();
                    self.close_lane += 1;
                    bytes
                }
                4 => {
                    let Some(support) = self.model.supports.last_mut() else {
                        self.close_lane += 1;
                        continue;
                    };
                    if support.node_id.capacity() != 0 {
                        let bytes = support.node_id.capacity();
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        support.node_id = String::new();
                        return (false, 1, bytes);
                    }
                    if support.fixed.pop().is_some() {
                        return (false, 1, 0);
                    }
                    let bytes = support.fixed.capacity() * size_of::<semio_s_fem_model::Dof>();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        support.fixed = Vec::new();
                        return (false, 1, bytes);
                    }
                    self.model.supports.pop();
                    return (false, 1, 0);
                }
                5 => {
                    let bytes = self.model.supports.capacity() * size_of::<semio_s_fem_model::Support>();
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    self.model.supports = Vec::new();
                    self.close_lane += 1;
                    bytes
                }
                6 => {
                    self.close_lane += 1;
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
