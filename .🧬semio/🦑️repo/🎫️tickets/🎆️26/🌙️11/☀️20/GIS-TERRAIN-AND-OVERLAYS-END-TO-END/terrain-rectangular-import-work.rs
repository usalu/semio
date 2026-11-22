//! 📥️ Reads original ASCII Grid rows into one cancellable Terrain mutation.

use super::{ErrorNotice, MAX_FILE_BYTES, ascii_grid};
use crate::editor::gis3d::{Gis3dCommand, Gis3dPlayApp};
use crate::mutations::{GisTerrainMutation, ReplaceElevationSource};
use crate::schema::snapshot::{ElevationSource, GisTerrainSnapshot};
use semio_framework_job::{InteractiveJobCloseStep, OriginalTurnFailure, StepContext, close_original_effect_step};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::{RetainedCloneProgress, RetirementDemand, ValueError};
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_plugin::{EditorApp, Emit};

type App = EditorApp<Gis3dPlayApp>;
type Step = ArtifactCommandWorkStep<App>;
const STAGE: &str = "terrain-elevation-import";
const PREVIEW: &[u8] = r#"{"en":"Reading elevation rows","de":"Höhenrasterzeilen werden gelesen"}"#.as_bytes();

pub struct Work {
    reader: Option<ascii_grid::GridReader>,
    rows: ControlledRetirement<Vec<Vec<i32>>>,
}

impl Default for Work {
    fn default() -> Self {
        Self { reader: None, rows: ControlledRetirement::retain(Vec::new()) }
    }
}

impl ArtifactCommandWork<App> for Work {
    fn tool_id(&self) -> &'static str { super::FILE_ACTION }

    fn extent(&self, command: &Gis3dCommand, _: &GisTerrainSnapshot, _: &protocol::InteractionState, _: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<App>>) -> Option<usize> {
        matches!(command, Gis3dCommand::ImportElevationGridFile(file) if !file.payload.is_empty() && file.payload.len() <= MAX_FILE_BYTES as usize).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, App>, cx: &mut StepContext<'_>) -> Result<Step, OriginalTurnFailure> {
        let pending = Step::Pending { stage: STAGE, preview: PREVIEW };
        let progress = Step::Progress { stage: STAGE, preview: PREVIEW };
        if cx.is_cancelled() || cx.should_yield() { return Ok(pending); }
        let Gis3dCommand::ImportElevationGridFile(file) = input.command else {
            return Err(ValueError::invariant("ASCII Grid import received another command").into());
        };
        let rows = self.rows.original_mut().ok_or_else(|| ValueError::invariant("ASCII Grid import is closing"))?;
        let Some(reader) = self.reader.as_mut() else {
            let copy_bytes = size_of::<Option<ascii_grid::GridReader>>();
            if !cx.retained_work_can_enter(RetirementDemand { copy_bytes, depth: 1, ..Default::default() }) {
                return Ok(pending);
            }
            let reader = ascii_grid::GridReader::new(&file.payload, MAX_FILE_BYTES as usize).map_err(ErrorNotice::fault)?;
            let fuel = reader.offset() as u64;
            self.reader = Some(reader);
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() })?;
            cx.consume_fuel(fuel.max(1));
            return Ok(progress);
        };
        let [columns, height] = reader.dimensions;
        if rows.len() < height {
            let copy_bytes = columns * size_of::<i32>() + size_of::<Vec<i32>>();
            let outer_capacity = if rows.capacity() == 0 { height * size_of::<Vec<i32>>() } else { 0 };
            let capacity_bytes = columns * size_of::<i32>() + outer_capacity;
            if !cx.retained_work_can_enter(RetirementDemand { copy_bytes, capacity_bytes, depth: 2, ..Default::default() }) {
                return Ok(pending);
            }
            let before = reader.offset();
            let row = reader.next_row(&file.payload).map_err(ErrorNotice::fault)?
                .ok_or_else(|| ValueError::invariant("ASCII Grid reader ended before its last row"))?;
            let mut retained_capacity_bytes = row.capacity() * size_of::<i32>();
            if outer_capacity != 0 {
                rows.reserve_exact(height);
                retained_capacity_bytes += rows.capacity() * size_of::<Vec<i32>>();
            }
            rows.push(row);
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, retained_capacity_bytes, ..Default::default() })?;
            cx.consume_fuel((reader.offset() - before + columns * 4) as u64);
            return Ok(progress);
        }
        let bounds = reader.bounds;
        if input.snapshot.elevation_source.as_ref().is_some_and(|source| {
            source.bounds.map(f64::to_bits) == bounds.map(f64::to_bits) && source.rows_millimetres.iter().eq(rows.iter().rev())
        }) {
            return Ok(Step::Complete(Emit::default()));
        }
        let copy_bytes = size_of::<Step>() + height * size_of::<Vec<i32>>() * 2;
        let capacity_bytes = size_of::<GisTerrainMutation>();
        if !cx.retained_work_can_enter(RetirementDemand { copy_bytes, capacity_bytes, depth: 2, ..Default::default() }) {
            return Ok(pending);
        }
        rows.reverse();
        let source = ElevationSource { rows_millimetres: std::mem::take(rows), bounds };
        let mutation = GisTerrainMutation::ReplaceElevationSource(ReplaceElevationSource { new_elevation_source: Some(source) });
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, retained_capacity_bytes: capacity_bytes, ..Default::default() })?;
        cx.consume_fuel(height as u64);
        Ok(Step::Complete(Emit::mutations(vec![mutation])))
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        let body = cx.retained_grant().maximum_copy_bytes;
        let demand = self.rows.next_demand(body)?;
        close_original_effect_step(cx, demand, |effect| self.rows.step(body, effect))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.rows.next_demand(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, size: usize) -> Result<usize, ValueError> { Ok(self.rows.next_demand(size)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.rows.next_demand(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.rows.next_demand(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.rows.terminal_is_empty() }
    fn terminal_frame_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
}
