//! 🏭️ Factory and inferred fields of the WFC 3D inference: the job factory, the descriptor, the solve entry points and the inferred solve, contradiction and entropy fields. See [`super`].

use super::*;

pub struct Wfc3dInferenceJobFactory {
    keys: [semio_framework::ToolFactoryKey; 1],
}

impl Default for Wfc3dInferenceJobFactory {
    fn default() -> Self {
        Self { keys: [semio_framework::ToolFactoryKey::new(WFC3D_INFERENCE_JOB_KIND, WFC3D_INFERENCE_TOOL_ID)] }
    }
}

impl semio_framework::ToolJobFactory for Wfc3dInferenceJobFactory {
    type Payload = Wfc3dInferenceRequest;
    type Job = Wfc3dInferenceJob;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        WFC3D_INFERENCE_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(16 << 20, MAX_WFC3D_TILES + MAX_WFC3D_SLOTS + MAX_WFC3D_RULES + MAX_WFC3D_EDGES, 4_096, MAX_WFC3D_OUTPUT_BYTES, 7_500, 1, 1)
    }

    fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Wfc3dInferenceJob::new(operation, payload).map_err(semio_framework::ToolJobFactoryError::new)
    }

    fn create_job_from_wire(&mut self, operation: semio_framework_job::Operation, payload: &[u8], checkpoint: Option<Vec<u8>>) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        let payload_text = std::str::from_utf8(payload).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("wfc3d-inference-wire-decode:{error}")))?;
        let mut request: Wfc3dInferenceRequest = crate::standards::v1::subsets::any::io::text::inferences::decode_inference_value(payload_text).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("wfc3d-inference-wire-decode:{error}")))?;
        if checkpoint.is_some() {
            request.checkpoint = checkpoint;
        }
        Wfc3dInferenceJob::new(operation, request).map_err(semio_framework::ToolJobFactoryError::new)
    }
}

/// 💡️ Descriptor for the `s.wfc.wfc3d.solve` inference — five handcrafted facet leaves.
pub fn wfc3d_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.wfc.wfc3d.solve",
        inference: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"),
            typescript: include_str!("../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts"),
            graphql: include_str!("../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔗️.graphql"),
            json_schema: include_str!("../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️.json"),
            proto: include_str!("../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🛰️.proto"),
        },
    }
}

pub fn register_wfc3d_inference_factory(bus: &semio_framework::ActionBus) -> Result<(), semio_framework::ToolRegistrationError> {
    bus.register_once(Wfc3dInferenceJobFactory::default())
}

/// 🏁 Explicit headless adapter over the same complete parent job used by the public factory.
pub(crate) fn solve_with_job(snapshot: &Wfc3dSnapshot) -> Result<Wfc3dInferenceCommit, String> {
    solve_with_clock(snapshot, semio_framework_job::default_now_us)
}

/// 🧮️ The same headless adapter driven by an injected clock, so correctness laws run on
/// [`semio_framework_job::logical_now_us`] and never on a descheduled thread's wall clock.
pub(crate) fn solve_with_clock(snapshot: &Wfc3dSnapshot, now_us: fn() -> Option<u64>) -> Result<Wfc3dInferenceCommit, String> {
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), snapshot.seed);
    let job = Wfc3dInferenceJob::new(operation, Wfc3dInferenceRequest { snapshot: Some(snapshot.clone()), document: None, checkpoint: None })?;
    engine::job::run_headless(
        job,
        operation,
        now_us,
        engine::job::HeadlessSite { site: "wfc3d.inference.headless", label: "wfc3d-inference-headless", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: HEADLESS_FUEL_PER_STEP, step_budget_us: HEADLESS_STEP_BUDGET_US },
        |_job, output| {
            std::str::from_utf8(output)
                .map_err(|error| format!("wfc3d-invalid-commit:{error}"))
                .and_then(|text| crate::standards::v1::subsets::any::io::text::inferences::decode_inference_value::<Wfc3dInferenceCommit>(text).map_err(|error| format!("wfc3d-invalid-commit:{error}")))
        },
    )
}

impl store::InferredField<Wfc3dSnapshot> for Wfc3dSolve {
    type Dependency = Vec<u8>;
    type Key = String;
    type Value = Wfc3dSolveResult;

    const FIELD_ID: &'static str = "s.wfc.wfc3d.inference.solve";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "slots", "edges", "tiles", "rules"]
    }
    fn plan(_snapshot: &Wfc3dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "wfc3d".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &Wfc3dSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        crate::standards::v1::subsets::any::io::text::inferences::encode_inference_value(snapshot).into_bytes()
    }
    fn compute(snapshot: &Wfc3dSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        match solve_with_job(snapshot) {
            Ok(solution) => Wfc3dSolveResult::Solved { assignments: solution.assignments },
            _ => Wfc3dSolveResult::Unsolved,
        }
    }
}

impl store::InferredField<Wfc3dSnapshot> for Wfc3dContradiction {
    type Dependency = Vec<u8>;
    type Key = String;
    type Value = bool;

    const FIELD_ID: &'static str = "s.wfc.wfc3d.inference.contradiction";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "slots", "edges", "tiles", "rules"]
    }
    fn plan(_snapshot: &Wfc3dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "wfc3d".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &Wfc3dSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        crate::standards::v1::subsets::any::io::text::inferences::encode_inference_value(snapshot).into_bytes()
    }
    fn compute(snapshot: &Wfc3dSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        solve_with_job(snapshot).is_ok()
    }
}

impl store::InferredField<Wfc3dSnapshot> for Wfc3dEntropy {
    type Dependency = Vec<u8>;
    type Key = String;
    type Value = f64;

    const FIELD_ID: &'static str = "s.wfc.wfc3d.inference.entropy";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["slots", "tiles"]
    }
    fn plan(snapshot: &Wfc3dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        snapshot.slots.iter().map(|slot| store::InferenceStep { key: slot.id.clone(), parents: Vec::new() }).collect()
    }
    fn dep_input(snapshot: &Wfc3dSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        let mut bytes = Vec::new();
        let pinned = snapshot.slots.iter().find(|slot| &slot.id == key).and_then(|slot| slot.pinned_tile_id.clone());
        bytes.extend_from_slice(pinned.unwrap_or_default().as_bytes());
        bytes.push(0);
        for tile in &snapshot.tiles {
            bytes.extend_from_slice(tile.id.as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(&tile.weight.to_le_bytes());
        }
        bytes
    }
    fn compute(snapshot: &Wfc3dSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let pinned = snapshot.slots.iter().find(|slot| &slot.id == key).and_then(|slot| slot.pinned_tile_id.as_ref());
        if pinned.is_some() {
            return 0.0;
        }
        shannon_entropy_over_tiles(snapshot)
    }
}
