//! ♻️ Retirement and retained clone of the GIS map owned values: the snapshot and mutation retirement factories and the bounded snapshot clone authority. See [`super`].

use super::*;

pub const GIS_MAP_OWNED_FIELD_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct GisMapSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<GisMapSnapshot> for GisMapSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &GisMapSnapshot) -> usize { semio_framework_value::retirement::owned_retirement_birth_bytes::<GisMapSnapshot>() }

    fn retire_owned(&self, value: GisMapSnapshot, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (semio_framework_value::ValueError, GisMapSnapshot)> {
        semio_framework_value::retirement::admit_owned_retirement(value, grant)
    }
}

impl store::SnapshotRetirementFactory<GisMapSnapshot> for GisMapSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<GisMapSnapshot>) -> usize { semio_framework_value::shared_retirement_birth_bytes::<GisMapSnapshot>() }

    fn retire(&self, snapshot: std::sync::Arc<GisMapSnapshot>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<GisMapSnapshot>)> {
        semio_framework_value::admit_shared_retirement(snapshot, grant, true)
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct GisMapMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<GisMapMutation> for GisMapMutationRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &GisMapMutation) -> usize { semio_framework_value::retirement::owned_retirement_birth_bytes::<GisMapMutation>() }

    fn retire_owned(&self, value: GisMapMutation, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (semio_framework_value::ValueError, GisMapMutation)> {
        semio_framework_value::retirement::admit_owned_retirement(value, grant)
    }
}

pub(super) struct GisMapSnapshotCloneAuthority {
    pub(super) value: std::mem::ManuallyDrop<Option<GisMapSnapshot>>,
    pub(super) retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub(super) phase: u8,
    pub(super) index: usize,
    pub(super) terminal: bool,
}

impl GisMapSnapshotCloneAuthority {
    fn empty_child<S>() -> store::os_store::artifact_child::ArtifactChild<S> {
        store::os_store::artifact_child::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } })
    }

    pub(super) fn new() -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(Some(GisMapSnapshot { positions: Vec::new(), routes: Vec::new(), regions: Vec::new(), drawing: Self::empty_child(), image: None, value: Self::empty_child() })),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: 0,
            index: 0,
            terminal: false,
        }
    }

    fn clone_string(source: &str) -> Result<String, &'static str> {
        if source.len() > GIS_MAP_OWNED_FIELD_BYTES {
            return Err("gis-map-store.initializer-field-too-large");
        }
        let mut value = String::new();
        value.try_reserve_exact(source.len()).map_err(|_| "gis-map-store.initializer-string-admission")?;
        value.push_str(source);
        Ok(value)
    }

    fn clone_feature(source: &MapFeature) -> Result<MapFeature, &'static str> {
        let encoded = semio_framework_pack_json::to_json_string(source).into_bytes();
        if encoded.len() > GIS_MAP_OWNED_FIELD_BYTES {
            return Err("gis-map-store.initializer-feature-too-large");
        }
        Ok(source.clone())
    }

    fn copy_child_field<'a, S>(target: &mut store::os_store::artifact_child::ArtifactChild<S>, source: &'a store::os_store::artifact_child::ArtifactChild<S>, phase: u8, base: u8) -> Result<&'a [u8], &'static str> {
        let observed = match phase - base {
            0 => {
                target.child_id = Self::clone_string(&source.child_id)?;
                source.child_id.as_bytes()
            }
            1 => {
                target.target.artifact_id = Self::clone_string(&source.target.artifact_id)?;
                source.target.artifact_id.as_bytes()
            }
            2 => {
                target.target.dialect.artifact_kind = Self::clone_string(&source.target.dialect.artifact_kind)?;
                source.target.dialect.artifact_kind.as_bytes()
            }
            3 => {
                target.target.dialect.standard = Self::clone_string(&source.target.dialect.standard)?;
                source.target.dialect.standard.as_bytes()
            }
            4 => {
                target.target.dialect.subset = Self::clone_string(&source.target.dialect.subset)?;
                source.target.dialect.subset.as_bytes()
            }
            _ => return Err("gis-map-store.initializer-child-phase"),
        };
        Ok(observed)
    }

    pub(super) fn step(&mut self, source: &GisMapSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let target = self.value.as_mut().ok_or("gis-map-store.initializer-clone-target")?;
        let observed = match self.phase {
            0..=2 => {
                let (source, target, admission) = match self.phase {
                    0 => (&source.positions, &mut target.positions, "gis-map-store.initializer-position-admission"),
                    1 => (&source.routes, &mut target.routes, "gis-map-store.initializer-route-admission"),
                    _ => (&source.regions, &mut target.regions, "gis-map-store.initializer-region-admission"),
                };
                if self.index == 0 && target.capacity() == 0 {
                    target.try_reserve_exact(source.len()).map_err(|_| admission)?;
                }
                if let Some(feature) = source.get(self.index) {
                    let encoded = semio_framework_pack_json::to_json_string(feature).into_bytes();
                    if encoded.len() > GIS_MAP_OWNED_FIELD_BYTES {
                        return Err("gis-map-store.initializer-feature-too-large");
                    }
                    target.push(Self::clone_feature(feature)?);
                    self.index += 1;
                    cx.consume_fuel(encoded.len().max(1) as u64);
                    return Ok(false);
                }
                self.index = 0;
                &[]
            }
            3..=7 => Self::copy_child_field(&mut target.drawing, &source.drawing, self.phase, 3)?,
            8 => {
                if source.image.is_some() {
                    target.image = Some(Self::empty_child());
                }
                &[]
            }
            9..=13 => {
                let source = source.image.as_ref().ok_or("gis-map-store.initializer-image-source")?;
                let target = target.image.as_mut().ok_or("gis-map-store.initializer-image-target")?;
                Self::copy_child_field(target, source, self.phase, 9)?
            }
            14..=18 => Self::copy_child_field(&mut target.value, &source.value, self.phase, 14)?,
            _ => {
                self.terminal = true;
                return Ok(true);
            }
        };
        self.phase = match self.phase {
            8 if source.image.is_none() => 14,
            value => value + 1,
        };
        cx.consume_fuel(observed.len().max(1) as u64);
        Ok(false)
    }

    pub(super) fn take_value(&mut self) -> Option<GisMapSnapshot> {
        if !self.terminal {
            return None;
        }
        self.value.take()
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, semio_framework_value::ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "GIS clone retirement depth overflow"))?;
            Ok(demand)
        };
        if let Some(active) = self.retirement.as_ref() {
            return nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?);
        }
        if self.value.is_some() {
            return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&*self.value)?);
        }
        Ok(Default::default())
    }

    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        if self.retirement.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| {
                self.terminal = self.value.is_none() && self.retirement.is_none();
                RetainedCloneStep::Progress(step.progress())
            });
        }
        if self.value.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant);
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for GisMapSnapshotCloneAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "GIS snapshot clone reached Drop before exact handoff or cursor retirement");
    }
}
