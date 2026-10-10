//! 🦴️ Solver-neutral JSON projection; geometry, resolved loads and links come exclusively from the shared inference.
use crate::{ModelInference, ModelSnapshot};
use semio_framework_value::{DslValue, ToValue};
/// 📤️ A solver-input document in SI units with no analysis results or invented stiffness.
pub fn solver_json(model: &ModelSnapshot, inference: &ModelInference) -> String { semio_framework_pack_json::to_json_string(&DslValue::object([("schema".into(),DslValue::String("s.bim.model.structural-solver@1".into())),("units".into(),DslValue::String("m,N,Nm; distributed loads per m or m2".into())),("analysis".into(),inference.structural_analysis.to_value()),("load_cases".into(),model.load_cases.to_value())])) }
/// 🔮️ Reads the mounted shared graph before projecting solver input.
pub fn model_to_solver_json(model: &ModelSnapshot) -> Result<String, semio_framework::io_schema::IoError> { crate::standards::v1::subsets::any::io::with_inferred("solver JSON",model,|inference|solver_json(model,inference)) }
