//! 🥦️ Close ladder: the numerical child terminal check, close rung and interactive close step. See [`super`].

use super::*;

//#region 📦️PageVisualJob
impl Fem3dNumericalChild {
    pub fn terminal_is_empty(&self) -> bool {
        self.close_lane > 21
            && self.model.is_none()
            && self.pending_support.is_none()
            && self.pending_element.is_none()
            && self.pending_tet.is_none()
            && self.pending_node_id.is_none()
            && self.mesh.is_none()
            && self.assembly_build.is_none()
            && self.assembly.is_none()
            && self.csr_build.is_none()
            && self.pcg_build.is_none()
            && self.rejected_pcg_matrix.is_none()
            && self.rejected_pcg_rhs.is_none()
            && self.pcg.is_none()
            && self.modal_build.is_none()
            && self.ldlt.is_none()
            && self.subspace.is_none()
            && self.modal_mass.is_none()
            && self.solid_domain.is_none()
            && self.analysis_node_ids.terminal_is_empty()
            && self.meshed_solids.terminal_is_empty()
            && self.solid_points.terminal_is_empty()
            && self.solid_tris.terminal_is_empty()
            && self.solid_node_ids.terminal_is_empty()
            && self.solid_node_analysis_indices.terminal_is_empty()
            && self.rhs.terminal_is_empty()
            && self.modal_free_mass.terminal_is_empty()
    }

    fn close_rung((terminal, copied_items, released_bytes): (bool, usize, usize)) -> (bool, RetainedCloneProgress) {
        (terminal, RetainedCloneProgress { copied_items, released_bytes, ..Default::default() })
    }

    pub fn close_step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<(bool, RetainedCloneProgress), semio_framework_job::OriginalTurnFailure> {
        let maximum_bytes = grant.maximum_release_bytes;
        let child = RetainedCloneGrant { maximum_items: 1, ..grant };
        if self.terminal_is_empty() { return Ok(Self::close_rung((true, 0, 0))); }
        if maximum_bytes == 0 { return Ok(Self::close_rung((false, 0, 0))); }
        if let Some(matrix) = self.rejected_pcg_matrix.as_mut() {
            let step = matrix.close_step(maximum_bytes);
            if step.0 {
                self.rejected_pcg_matrix = None;
            }
            return Ok(Self::close_rung((false, step.1, step.2)));
        }
        if let Some(rhs) = self.rejected_pcg_rhs.as_mut() {
            if rhs.close_step() {
                self.rejected_pcg_rhs = None;
            }
            return Ok(Self::close_rung((false, 1, 0)));
        }
        let step = match self.close_lane {
            0 => {
                let Some(job) = self.mesh.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if !job.terminal_is_empty() {
                    let step = cx.with_retained_loan(child, |same| InteractiveJob::close_step(job, same))?;
                    return Ok((false, step.progress()));
                }
                self.mesh = None;
                let step = (true, 1, 0);
                step
            }
            1 => {
                let Some(job) = self.assembly_build.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                let step = job.close_step(maximum_bytes);
                if step.0 {
                    self.assembly_build = None;
                }
                step
            }
            2 => {
                let Some(job) = self.assembly.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                let step = job.close_retained_step(maximum_bytes);
                if step.0 {
                    self.assembly = None;
                }
                step
            }
            3 => {
                let Some(job) = self.csr_build.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                let step = job.close_step(maximum_bytes);
                if step.0 {
                    self.csr_build = None;
                }
                step
            }
            4 => {
                let Some(job) = self.pcg_build.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if !job.terminal_is_empty() {
                    let step = cx.with_retained_loan(child, |same| job.close_step(same))?;
                    return Ok((false, step.progress()));
                }
                self.pcg_build = None;
                let step = (true, 1, 0);
                step
            }
            5 => {
                let Some(job) = self.pcg.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if !job.terminal_is_empty() {
                    let step = cx.with_retained_loan(child, |same| InteractiveJob::close_step(job, same))?;
                    return Ok((false, step.progress()));
                }
                self.pcg = None;
                let step = (true, 1, 0);
                step
            }
            6 => {
                let Some(job) = self.modal_build.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                let step = job.close_step(maximum_bytes);
                if step.0 {
                    self.modal_build = None;
                }
                step
            }
            7 => {
                let Some(job) = self.ldlt.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if self.close_started_jobs & 1 == 0 {
                    job.begin_close();
                    self.close_started_jobs |= 1;
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                if !job.terminal_is_empty() {
                    let step = cx.with_retained_loan(child, |same| InteractiveJob::close_step(job, same))?;
                    return Ok((false, step.progress()));
                }
                self.ldlt = None;
                let step = (true, 1, 0);
                step
            }
            8 => {
                let Some(job) = self.subspace.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if self.close_started_jobs & 2 == 0 {
                    job.begin_close();
                    self.close_started_jobs |= 2;
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                if !job.terminal_is_empty() {
                    let step = cx.with_retained_loan(child, |same| InteractiveJob::close_step(job, same))?;
                    return Ok((false, step.progress()));
                }
                self.subspace = None;
                let step = (true, 1, 0);
                step
            }
            9 => {
                let Some(mass) = self.modal_mass.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                let step = mass.close_step(maximum_bytes);
                if step.0 {
                    self.modal_mass = None;
                }
                step
            }
            10 => {
                let Some(domain) = self.solid_domain.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if domain.close_step() {
                    self.solid_domain = None;
                    (true, 1, 0)
                } else {
                    (false, 1, 0)
                }
            }
            11 => {
                if let Some(solid) = self.meshed_solids.len().checked_sub(1).and_then(|index| self.meshed_solids.get_mut(index)) {
                    if let Some(value) = solid.node_ids.len().checked_sub(1).and_then(|index| solid.node_ids.get_mut(index)) {
                        let bytes = value.capacity();
                        if bytes != 0 {
                            if bytes > maximum_bytes {
                                return Ok(Self::close_rung((false, 0, 0)));
                            }
                            *value = String::new();
                            return Ok(Self::close_rung((false, 1, bytes)));
                        }
                        solid.node_ids.pop();
                        return Ok(Self::close_rung((false, 1, 0)));
                    }
                    if !solid.node_ids.close_admission_one() {
                        return Ok(Self::close_rung((false, 1, 0)));
                    }
                    if solid.tris.pop().is_some() || !solid.tris.close_admission_one() {
                        return Ok(Self::close_rung((false, 1, 0)));
                    }
                    if solid.points.pop().is_some() || !solid.points.close_admission_one() {
                        return Ok(Self::close_rung((false, 1, 0)));
                    }
                    self.meshed_solids.pop();
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                (self.meshed_solids.close_admission_one(), 1, 0)
            }
            12 => {
                if let Some(value) = self.pending_node_id.as_mut() {
                    let bytes = value.capacity();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return Ok(Self::close_rung((false, 0, 0)));
                        }
                        *value = String::new();
                        return Ok(Self::close_rung((false, 1, bytes)));
                    }
                    self.pending_node_id = None;
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                (true, 0, 0)
            }
            13 => {
                if let Some(value) = self.solid_node_ids.len().checked_sub(1).and_then(|index| self.solid_node_ids.get_mut(index)) {
                    let bytes = value.capacity();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return Ok(Self::close_rung((false, 0, 0)));
                        }
                        *value = String::new();
                        return Ok(Self::close_rung((false, 1, bytes)));
                    }
                    self.solid_node_ids.pop();
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                if !self.solid_node_ids.close_admission_one() {
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                if self.solid_node_analysis_indices.pop().is_some() || !self.solid_node_analysis_indices.close_admission_one() {
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                (true, 0, 0)
            }
            14 => {
                let Some(support) = self.pending_support.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                let step = support.close_step(maximum_bytes);
                if step.0 {
                    self.pending_support = None;
                }
                step
            }
            15 => {
                if let Some(value) = self.analysis_node_ids.len().checked_sub(1).and_then(|index| self.analysis_node_ids.get_mut(index)) {
                    let bytes = value.capacity();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return Ok(Self::close_rung((false, 0, 0)));
                        }
                        *value = String::new();
                        return Ok(Self::close_rung((false, 1, bytes)));
                    }
                    self.analysis_node_ids.pop();
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                (self.analysis_node_ids.close_admission_one(), 1, 0)
            }
            16 => {
                if self.solid_tris.pop().is_some() || !self.solid_tris.close_admission_one() {
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                if self.solid_points.pop().is_some() || !self.solid_points.close_admission_one() {
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                (true, 0, 0)
            }
            17 => (self.rhs.close_step(), 1, 0),
            18 => (self.modal_free_mass.close_step(), 1, 0),
            19 => {
                let Some(element) = self.pending_element.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if let Some(bytes) = element.mounted_next_string_bytes() {
                    if bytes > maximum_bytes {
                        return Ok(Self::close_rung((false, 0, 0)));
                    }
                    let released = element.close_mounted_string_step().unwrap_or(0);
                    return Ok(Self::close_rung((false, 1, released)));
                }
                self.pending_element = None;
                (true, 1, 0)
            }
            20 => {
                let Some(element) = self.pending_tet.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if let Some(bytes) = element.mounted_next_string_bytes() {
                    if bytes > maximum_bytes {
                        return Ok(Self::close_rung((false, 0, 0)));
                    }
                    let released = element.close_mounted_string_step().unwrap_or(0);
                    return Ok(Self::close_rung((false, 1, released)));
                }
                self.pending_tet = None;
                (true, 1, 0)
            }
            21 => {
                let Some(model) = self.model.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                let step = model.close_step(maximum_bytes);
                if step.0 {
                    self.model = None;
                }
                step
            }
            _ => return Ok(Self::close_rung((self.terminal_is_empty(), 0, 0))),
        };
        if step.0 {
            self.close_lane += 1;
            return Ok(Self::close_rung((false, step.1, step.2)));
        }
        Ok(Self::close_rung(step))
    }
}
//#endregion 📦️PageVisualJob
