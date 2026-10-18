//! 🧵️ process3d retained cursors: the incremental capability, machine, pose, child, measure, step, string and solid cursors that decode one record per grant.

use super::*;

impl Process3dRetainedCapabilityCursor {
    fn take_recipe(&mut self) -> Result<MeasureRecipe, String> {
        let first = self.recipe_fields[0].take().unwrap_or_default();
        let second = self.recipe_fields[1].take().unwrap_or_default();
        let third = self.recipe_fields[2].take().unwrap_or_default();
        match self.recipe_tag {
            0 => Ok(MeasureRecipe::DiscCut { diameter: first, kerf: second }),
            1 => Ok(MeasureRecipe::BladeCut { kerf: first, length: second, depth: third }),
            2 => Ok(MeasureRecipe::PocketCut { diameter: first, depth: second }),
            3 => Ok(MeasureRecipe::BoreDrill { radius: first, depth: second }),
            4 => Ok(MeasureRecipe::CylinderAttach { radius: first, length: second }),
            5 => Ok(MeasureRecipe::BoxAttach { width: first, depth: second, height: third }),
            _ => Err("process3d retained capability recipe tag is invalid".into()),
        }
    }

    fn finish(&mut self) -> Capability {
        Capability {
            id: self.id.take().unwrap_or_default(),
            label: self.label.take().unwrap_or_default(),
            icon_id: self.icon_id.take().unwrap_or_default(),
            recipe: self.recipe.take().unwrap_or(MeasureRecipe::DiscCut { diameter: String::new(), kerf: String::new() }),
            parameters: std::mem::take(&mut self.parameters),
            rules: std::mem::take(&mut self.rules),
        }
    }

    pub(super) fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<Capability>, String> {
        match self.phase {
            0 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.id = Some(value);
            }
            1 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.label = Some(value);
            }
            2 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.icon_id = Some(value);
            }
            3 => {
                self.recipe_tag = reader.read_u8().map_err(|error| error.to_string())?;
                if self.recipe_tag > 5 {
                    return Err("process3d retained capability recipe tag is invalid".into());
                }
            }
            4..=6 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.recipe_fields[(self.phase - 4) as usize] = Some(value);
            }
            7 => {
                self.recipe = Some(self.take_recipe()?);
                self.expected = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
                if self.expected > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                    return Err("process3d retained capability parameter count exceeds capacity".into());
                }
                self.parameters.try_reserve_exact(self.expected).map_err(|_| "process3d retained capability parameter admission failed".to_string())?;
                self.index = 0;
                self.phase = if self.expected == 0 { 11 } else { 8 };
                return Ok(None);
            }
            8 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.parameter_id = Some(value);
            }
            9 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.parameter_label = Some(value);
            }
            10 => {
                let value = reader.read_f64_le().map_err(|error| error.to_string())?;
                self.parameters.push(CapabilityParameter { id: self.parameter_id.take().unwrap_or_default(), label: self.parameter_label.take().unwrap_or_default(), value });
                self.index += 1;
                self.phase = if self.index < self.expected { 8 } else { 11 };
                return Ok(None);
            }
            11 => {
                self.expected = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
                if self.expected > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                    return Err("process3d retained capability rule count exceeds capacity".into());
                }
                self.rules.try_reserve_exact(self.expected).map_err(|_| "process3d retained capability rule admission failed".to_string())?;
                self.index = 0;
                if self.expected == 0 {
                    return Ok(Some(self.finish()));
                }
                self.phase = 12;
                return Ok(None);
            }
            12 => {
                self.rule_tag = reader.read_u8().map_err(|error| error.to_string())?;
                if self.rule_tag > 1 {
                    return Err("process3d retained capability rule tag is invalid".into());
                }
            }
            13 => {
                self.rule_quantity = Some(match reader.read_u8().map_err(|error| error.to_string())? {
                    0 => StockQuantity::Width,
                    1 => StockQuantity::Depth,
                    2 => StockQuantity::Height,
                    3 => StockQuantity::MaxDimension,
                    4 => StockQuantity::MinDimension,
                    _ => return Err("process3d retained stock quantity tag is invalid".into()),
                });
            }
            14 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.rule_parameter = Some(value);
            }
            15 => {
                let margin = reader.read_f64_le().map_err(|error| error.to_string())?;
                let quantity = self.rule_quantity.take().unwrap_or(StockQuantity::Width);
                let parameter = self.rule_parameter.take().unwrap_or_default();
                self.rules.push(if self.rule_tag == 0 { CapabilityRule::Min { quantity, parameter, margin } } else { CapabilityRule::Max { quantity, parameter, margin } });
                self.index += 1;
                if self.index == self.expected {
                    return Ok(Some(self.finish()));
                }
                self.phase = 12;
                return Ok(None);
            }
            _ => unreachable!("retained capability phase"),
        }
        self.phase += 1;
        Ok(None)
    }

    pub(super) fn take_partial(&mut self) -> Capability {
        if self.string.has_partial() {
            let partial = self.string.take_partial();
            match self.phase {
                0 => self.id = Some(partial),
                1 => self.label = Some(partial),
                2 => self.icon_id = Some(partial),
                4..=6 => self.recipe_fields[(self.phase - 4) as usize] = Some(partial),
                8 => self.parameter_id = Some(partial),
                9 => self.parameter_label = Some(partial),
                14 => self.rule_parameter = Some(partial),
                _ => drop(partial),
            }
            self.string = Default::default();
        }
        if self.parameter_id.is_some() || self.parameter_label.is_some() {
            self.parameters.push(CapabilityParameter { id: self.parameter_id.take().unwrap_or_default(), label: self.parameter_label.take().unwrap_or_default(), value: 0.0 });
        }
        if let Some(parameter) = self.rule_parameter.take() {
            let quantity = self.rule_quantity.take().unwrap_or_default();
            self.rules.push(if self.rule_tag == 1 { CapabilityRule::Max { quantity, parameter, margin: 0.0 } } else { CapabilityRule::Min { quantity, parameter, margin: 0.0 } });
        }
        if self.recipe.is_none() {
            self.recipe = self.take_recipe().ok();
        }
        self.finish()
    }
}

impl Process3dRetainedMachineCursor {
    fn finish(&mut self) -> WorkshopMachine {
        WorkshopMachine {
            id: self.id.take().unwrap_or_default(),
            label: self.label.take().unwrap_or_default(),
            icon_id: self.icon_id.take().unwrap_or_default(),
            catalog_id: self.catalog_id.take(),
            capabilities: std::mem::take(&mut self.capabilities),
        }
    }

    pub(super) fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<WorkshopMachine>, String> {
        match self.phase {
            0 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.id = Some(value);
            }
            1 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.label = Some(value);
            }
            2 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.icon_id = Some(value);
            }
            3 => match reader.read_u8().map_err(|error| error.to_string())? {
                0 => {
                    self.phase = 5;
                    return Ok(None);
                }
                1 => {}
                _ => return Err("process3d retained machine catalog tag is invalid".into()),
            },
            4 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.catalog_id = Some(value);
            }
            5 => {
                self.expected = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
                if self.expected > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                    return Err("process3d retained machine capability count exceeds capacity".into());
                }
                self.capabilities.try_reserve_exact(self.expected).map_err(|_| "process3d retained machine capability admission failed".to_string())?;
                if self.expected == 0 {
                    return Ok(Some(self.finish()));
                }
                self.phase = 6;
                return Ok(None);
            }
            6 => {
                let cursor = self.capability.get_or_insert_with(Process3dRetainedCapabilityCursor::default);
                if let Some(capability) = cursor.step(reader)? {
                    self.capabilities.push(capability);
                    self.capability = None;
                    self.index += 1;
                    if self.index == self.expected {
                        return Ok(Some(self.finish()));
                    }
                }
                return Ok(None);
            }
            _ => unreachable!("retained machine phase"),
        }
        self.phase += 1;
        Ok(None)
    }

    pub(super) fn take_partial(&mut self) -> WorkshopMachine {
        if self.string.has_partial() {
            let partial = self.string.take_partial();
            match self.phase {
                0 => self.id = Some(partial),
                1 => self.label = Some(partial),
                2 => self.icon_id = Some(partial),
                4 => self.catalog_id = Some(partial),
                _ => drop(partial),
            }
            self.string = Default::default();
        }
        if let Some(mut cursor) = self.capability.take() {
            self.capabilities.push(cursor.take_partial());
        }
        self.finish()
    }
}

#[derive(Default)]
pub(super) struct Process3dRetainedCapabilityCursor {
    pub(super) phase: u8,
    pub(super) string: Process3dRetainedStringCursor,
    pub(super) id: Option<String>,
    pub(super) label: Option<String>,
    pub(super) icon_id: Option<String>,
    recipe_tag: u8,
    recipe_fields: [Option<String>; 3],
    pub(super) recipe: Option<MeasureRecipe>,
    pub(super) expected: usize,
    pub(super) index: usize,
    parameter_id: Option<String>,
    parameter_label: Option<String>,
    pub(super) parameters: Vec<CapabilityParameter>,
    rule_tag: u8,
    rule_quantity: Option<StockQuantity>,
    rule_parameter: Option<String>,
    pub(super) rules: Vec<CapabilityRule>,
}

#[derive(Default)]
pub(super) struct Process3dRetainedMachineCursor {
    pub(super) phase: u8,
    pub(super) string: Process3dRetainedStringCursor,
    pub(super) id: Option<String>,
    pub(super) label: Option<String>,
    pub(super) icon_id: Option<String>,
    pub(super) catalog_id: Option<String>,
    pub(super) expected: usize,
    pub(super) index: usize,
    pub(super) capabilities: Vec<Capability>,
    pub(super) capability: Option<Process3dRetainedCapabilityCursor>,
}

#[derive(Default)]
pub(super) struct Process3dRetainedPoseCursor {
    pub(super) index: usize,
    values: [f64; 7],
}

impl Process3dRetainedPoseCursor {
    pub(super) fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<Pose>, String> {
        self.values[self.index] = reader.read_f64_le().map_err(|error| error.to_string())?;
        self.index += 1;
        Ok((self.index == self.values.len()).then(|| Pose { position: [self.values[0], self.values[1], self.values[2]], axis: [self.values[3], self.values[4], self.values[5]], angle: self.values[6] }))
    }

    pub(super) fn take_partial(&mut self) -> Pose {
        self.index = 0;
        Pose { position: [self.values[0], self.values[1], self.values[2]], axis: [self.values[3], self.values[4], self.values[5]], angle: self.values[6] }
    }
}

#[derive(Default)]
pub(super) struct Process3dRetainedChildCursor {
    pub(super) phase: u8,
    pub(super) child_id: Option<String>,
    pub(super) target: Option<semio_framework_artifact_reference::ArtifactRef>,
    pub(super) string: Process3dRetainedStringCursor,
}

impl Process3dRetainedChildCursor {
    pub(super) fn step<S>(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<store::os_store::artifact_child::ArtifactChild<S>>, String> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        if self.phase == 0 {
            let Some(child_id) = self.string.step(reader)? else { return Ok(None) };
            self.string = Process3dRetainedStringCursor::default();
            self.child_id = Some(child_id);
            self.phase = 1;
            return Ok(None);
        }
        let Some(uri) = self.string.step(reader)? else { return Ok(None) };
        self.string = Process3dRetainedStringCursor::default();
        let target = semio_framework_artifact_reference::ArtifactRef::parse_uri(&uri)?;
        self.target = Some(target);
        Ok(Some(store::os_store::artifact_child::ArtifactChild::new(self.child_id.take().unwrap_or_default(), self.target.take().expect("Process3d retained child target exists"))))
    }

    pub(super) fn take_partial<S>(&mut self) -> store::os_store::artifact_child::ArtifactChild<S> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        if self.phase == 0 && self.child_id.is_none() {
            self.child_id = Some(self.string.take_partial());
        } else if self.phase == 1 && self.target.is_none() {
            self.target = semio_framework_artifact_reference::ArtifactRef::parse_uri(&self.string.take_partial()).ok();
        }
        store::os_store::artifact_child::ArtifactChild::new(
            self.child_id.take().unwrap_or_default(),
            self.target.take().unwrap_or(semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } }),
        )
    }
}

pub(super) struct Process3dRetainedMeasureCursor {
    pub(super) phase: Process3dRetainedMeasurePhase,
    pub(super) tag: u8,
    pub(super) radius: f64,
    pub(super) depth: f64,
    pub(super) solid: Option<WorkingSolid>,
    solid_cursor: Process3dRetainedSolidCursor,
    pose_cursor: Process3dRetainedPoseCursor,
}

impl Default for Process3dRetainedMeasureCursor {
    fn default() -> Self {
        Self { phase: Process3dRetainedMeasurePhase::Tag, tag: 0, radius: 0.0, depth: 0.0, solid: None, solid_cursor: Process3dRetainedSolidCursor::default(), pose_cursor: Process3dRetainedPoseCursor::default() }
    }
}

impl Process3dRetainedMeasureCursor {
    pub(super) fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<ProcessMeasure>, String> {
        match self.phase {
            Process3dRetainedMeasurePhase::Tag => {
                self.tag = reader.read_u8().map_err(|error| error.to_string())?;
                self.phase = match self.tag {
                    0 | 2 => Process3dRetainedMeasurePhase::Solid,
                    1 => Process3dRetainedMeasurePhase::Radius,
                    _ => return Err("process3d retained measure tag is invalid".into()),
                };
            }
            Process3dRetainedMeasurePhase::Solid => {
                if let Some(solid) = self.solid_cursor.step(reader)? {
                    self.solid = Some(solid);
                    self.phase = Process3dRetainedMeasurePhase::Pose;
                }
            }
            Process3dRetainedMeasurePhase::Radius => {
                self.radius = reader.read_f64_le().map_err(|error| error.to_string())?;
                self.phase = Process3dRetainedMeasurePhase::Depth;
            }
            Process3dRetainedMeasurePhase::Depth => {
                self.depth = reader.read_f64_le().map_err(|error| error.to_string())?;
                self.phase = Process3dRetainedMeasurePhase::Pose;
            }
            Process3dRetainedMeasurePhase::Pose => {
                if let Some(pose) = self.pose_cursor.step(reader)? {
                    let solid = self.solid.take();
                    return Ok(Some(match self.tag {
                        0 => ProcessMeasure::Cut { tool: solid.unwrap_or(WorkingSolid::Sphere { radius: 0.0 }), pose },
                        1 => ProcessMeasure::Drill { radius: self.radius, depth: self.depth, pose },
                        2 => ProcessMeasure::Attach { component: solid.unwrap_or(WorkingSolid::Sphere { radius: 0.0 }), pose },
                        _ => unreachable!("validated retained measure tag"),
                    }));
                }
            }
        }
        Ok(None)
    }

    pub(super) fn take_partial(&mut self) -> ProcessMeasure {
        let pose = self.pose_cursor.take_partial();
        let solid = self.solid.take().unwrap_or_else(|| self.solid_cursor.take_partial());
        match self.tag {
            1 => ProcessMeasure::Drill { radius: self.radius, depth: self.depth, pose },
            2 => ProcessMeasure::Attach { component: solid, pose },
            _ => ProcessMeasure::Cut { tool: solid, pose },
        }
    }
}

pub(super) struct Process3dRetainedStepCursor {
    pub(super) phase: Process3dRetainedStepPhase,
    pub(super) id: Option<String>,
    pub(super) label: Option<String>,
    pub(super) enabled: bool,
    origin_machine: Option<String>,
    origin_capability: Option<String>,
    pub(super) string: Process3dRetainedStringCursor,
    measure_cursor: Process3dRetainedMeasureCursor,
}

impl Default for Process3dRetainedStepCursor {
    fn default() -> Self {
        Self { phase: Process3dRetainedStepPhase::Id, id: None, label: None, enabled: false, origin_machine: None, origin_capability: None, string: Process3dRetainedStringCursor::default(), measure_cursor: Process3dRetainedMeasureCursor::default() }
    }
}

impl Process3dRetainedStepCursor {
    pub(super) fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<ProcessStep>, String> {
        match self.phase {
            Process3dRetainedStepPhase::Id => {
                let Some(id) = self.string.step(reader)? else { return Ok(None) };
                self.string = Process3dRetainedStringCursor::default();
                self.id = Some(id);
                self.phase = Process3dRetainedStepPhase::Label;
            }
            Process3dRetainedStepPhase::Label => {
                let Some(label) = self.string.step(reader)? else { return Ok(None) };
                self.string = Process3dRetainedStringCursor::default();
                self.label = Some(label);
                self.phase = Process3dRetainedStepPhase::Enabled;
            }
            Process3dRetainedStepPhase::Enabled => {
                self.enabled = reader.read_u8().map_err(|error| error.to_string())? != 0;
                self.phase = Process3dRetainedStepPhase::OriginTag;
            }
            Process3dRetainedStepPhase::OriginTag => {
                self.phase = match reader.read_u8().map_err(|error| error.to_string())? {
                    0 => Process3dRetainedStepPhase::Measure,
                    1 => Process3dRetainedStepPhase::OriginMachine,
                    _ => return Err("process3d retained origin tag is invalid".into()),
                };
            }
            Process3dRetainedStepPhase::OriginMachine => {
                let Some(machine) = self.string.step(reader)? else { return Ok(None) };
                self.string = Process3dRetainedStringCursor::default();
                self.origin_machine = Some(machine);
                self.phase = Process3dRetainedStepPhase::OriginCapability;
            }
            Process3dRetainedStepPhase::OriginCapability => {
                let Some(capability) = self.string.step(reader)? else { return Ok(None) };
                self.string = Process3dRetainedStringCursor::default();
                self.origin_capability = Some(capability);
                self.phase = Process3dRetainedStepPhase::Measure;
            }
            Process3dRetainedStepPhase::Measure => {
                if let Some(measure) = self.measure_cursor.step(reader)? {
                    return Ok(Some(ProcessStep {
                        id: self.id.take().unwrap_or_default(),
                        label: self.label.take().unwrap_or_default(),
                        enabled: self.enabled,
                        origin: self.origin_machine.take().map(|machine_id| StepOrigin { machine_id, capability_id: self.origin_capability.take().unwrap_or_default() }),
                        measure,
                    }));
                }
            }
        }
        Ok(None)
    }

    pub(super) fn take_partial(&mut self) -> ProcessStep {
        let partial = self.string.take_partial();
        match self.phase {
            Process3dRetainedStepPhase::Id if self.id.is_none() => self.id = Some(partial),
            Process3dRetainedStepPhase::Label if self.label.is_none() => self.label = Some(partial),
            Process3dRetainedStepPhase::OriginMachine if self.origin_machine.is_none() => self.origin_machine = Some(partial),
            Process3dRetainedStepPhase::OriginCapability if self.origin_capability.is_none() => self.origin_capability = Some(partial),
            _ => drop(partial),
        }
        ProcessStep {
            id: self.id.take().unwrap_or_default(),
            label: self.label.take().unwrap_or_default(),
            enabled: self.enabled,
            origin: self.origin_machine.take().map(|machine_id| StepOrigin { machine_id, capability_id: self.origin_capability.take().unwrap_or_default() }),
            measure: self.measure_cursor.take_partial(),
        }
    }
}

pub(super) struct Process3dRetainedStringCursor {
    pub(super) length: usize,
    shift: u32,
    remaining: Option<usize>,
    pub(super) bytes: Vec<u8>,
    pub(super) maximum_bytes: usize,
}

impl Default for Process3dRetainedStringCursor {
    fn default() -> Self {
        Self { length: 0, shift: 0, remaining: None, bytes: Vec::new(), maximum_bytes: store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES }
    }
}

impl Process3dRetainedStringCursor {
    #[cfg(test)]
    pub(super) fn with_maximum_bytes(maximum_bytes: usize) -> Self {
        let mut cursor = Self::default();
        cursor.maximum_bytes = maximum_bytes;
        cursor
    }

    fn finish(&mut self) -> Result<String, String> {
        self.length = 0;
        self.shift = 0;
        self.remaining = None;
        String::from_utf8(std::mem::take(&mut self.bytes)).map_err(|error| error.to_string())
    }

    pub(super) fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
        if let Some(remaining) = self.remaining {
            let byte = reader.read_u8().map_err(|error| error.to_string())?;
            self.bytes.push(byte);
            let remaining = remaining - 1;
            self.remaining = Some(remaining);
            return if remaining == 0 { Ok(Some(self.finish()?)) } else { Ok(None) };
        }
        let byte = reader.read_u8().map_err(|error| error.to_string())?;
        let payload = usize::from(byte & 0x7f);
        self.length = self.length.checked_add(payload.checked_shl(self.shift).ok_or_else(|| "process3d retained string length overflow".to_string())?).ok_or_else(|| "process3d retained string length overflow".to_string())?;
        if byte & 0x80 != 0 {
            self.shift = self.shift.checked_add(7).filter(|shift| *shift < usize::BITS).ok_or_else(|| "process3d retained string length overflow".to_string())?;
            return Ok(None);
        }
        if self.length > self.maximum_bytes {
            return Err("process3d retained string exceeded its fixed byte credit".into());
        }
        self.bytes.try_reserve_exact(self.length).map_err(|_| "process3d retained string admission failed".to_string())?;
        self.remaining = Some(self.length);
        if self.length == 0 {
            Ok(Some(self.finish()?))
        } else {
            Ok(None)
        }
    }

    pub(super) fn take_partial(&mut self) -> String {
        self.finish().unwrap_or_default()
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.length == 0 && self.shift == 0 && self.remaining.is_none() && self.bytes.is_empty()
    }

    pub(super) fn has_partial(&self) -> bool {
        self.length != 0 || self.shift != 0 || self.remaining.is_some() || !self.bytes.is_empty()
    }
}

impl Drop for Process3dRetainedStringCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d retained string cursor reached Drop before exact handback");
    }
}

#[derive(Default)]
struct Process3dRetainedSolidCursor {
    pub(super) tag: Option<u8>,
    pub(super) index: usize,
    values: [f64; 3],
    text: Option<String>,
    pub(super) string: Process3dRetainedStringCursor,
}

impl Process3dRetainedSolidCursor {
    pub(super) fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<WorkingSolid>, String> {
        let Some(tag) = self.tag else {
            let tag = reader.read_u8().map_err(|error| error.to_string())?;
            if tag > 5 {
                return Err("process3d retained solid tag is invalid".into());
            }
            self.tag = Some(tag);
            return Ok(None);
        };
        let scalar_count = match tag {
            0 => 3,
            1 => 2,
            2 => 1,
            _ => 0,
        };
        if self.index < scalar_count {
            self.values[self.index] = reader.read_f64_le().map_err(|error| error.to_string())?;
            self.index += 1;
            if self.index < scalar_count {
                return Ok(None);
            }
        } else if tag >= 3 {
            let Some(text) = self.string.step(reader)? else { return Ok(None) };
            self.string = Process3dRetainedStringCursor::default();
            self.text = Some(text);
        }
        Ok(Some(self.take_partial()))
    }

    pub(super) fn take_partial(&mut self) -> WorkingSolid {
        let tag = self.tag.take().unwrap_or(0);
        self.index = 0;
        if tag >= 3 && self.text.is_none() {
            self.text = Some(self.string.take_partial());
        }
        match tag {
            0 => WorkingSolid::Box { width: self.values[0], depth: self.values[1], height: self.values[2] },
            1 => WorkingSolid::Cylinder { radius: self.values[0], height: self.values[1] },
            2 => WorkingSolid::Sphere { radius: self.values[0] },
            3 => WorkingSolid::ImportedMesh { mesh_url: self.text.take().unwrap_or_default() },
            4 => WorkingSolid::ImportedSolid { solid_handle: self.text.take().unwrap_or_default() },
            5 => WorkingSolid::Reference { reference_id: self.text.take().unwrap_or_default() },
            _ => unreachable!("validated retained solid tag"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dRetainedMeasurePhase {
    Tag,
    Solid,
    Radius,
    Depth,
    Pose,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dRetainedStepPhase {
    Id,
    Label,
    Enabled,
    OriginTag,
    OriginMachine,
    OriginCapability,
    Measure,
}
