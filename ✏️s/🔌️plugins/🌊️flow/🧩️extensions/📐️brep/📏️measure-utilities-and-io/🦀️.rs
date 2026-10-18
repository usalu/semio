//! 📏️ Measure, utilities and io: the measure and utility operators and the step, stl, obj and dwg export and import operators. See [`super`].

use super::*;

// #region 🔖️Measure
num_operation!(Volume, "volume", |k, i| k.volume(&read_geometry(i, "geometry")?));

num_operation!(Area, "area", |k, i| k.area(&read_geometry(i, "geometry")?));

num_operation!(Length, "length", |k, i| k.length(&read_geometry(i, "geometry")?));

point_operation!(CenterOfMass, "center", |k, i| k.center_of_mass(&read_geometry(i, "geometry")?));

geo_operation!(BoundingBox, "box", |k, i| k.bounding_box(&read_geometry(i, "geometry")?));

num_operation!(Distance, "distance", |k, i| k.distance(&read_geometry(i, "a")?, &read_geometry(i, "b")?));

pub(super) struct ClosestPoint(pub(super) SessionCapture);

impl Operator for ClosestPoint {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let result = kernel.closest_point(&read_geometry(input, "geometry")?, read_xyz(input, "point")?).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("pointOut", point_dictionary(result.point)))
        })
    }
}

pub(super) struct ClassifyPoint(pub(super) SessionCapture);

impl Operator for ClassifyPoint {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let classification = kernel.classify_point(&read_geometry(input, "solid")?, read_xyz(input, "point")?).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("classification", number_dictionary(classify_number(classification))))
        })
    }
}

text_operation!(Validate, "report", |k, i| k.validate(&read_geometry(i, "geometry")?));
// #endregion 🔖️Measure

// #region 🔖️Utilities
geo_operation!(Vertex, "vertex", |k, i| k.vertex(read_xyz(i, "point")?));

geo_operation!(FaceFromWire, "face", |k, i| k.face_from_wire(&read_geometry(i, "wire")?));

pub(super) struct SewFaces(pub(super) SessionCapture);

impl Operator for SewFaces {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let faces = read_geometry_list(input, "faces")?;
            let tolerance = read_channel_number(input, "tolerance")?;
            let handle = kernel.sew_faces(&faces, tolerance).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(HealSolid, "solid", |k, i| k.heal_solid(&read_geometry(i, "geometry")?, read_channel_number(i, "tolerance")?));

geo_operation!(ConvertToNurbs, "geometryOut", |k, i| k.convert_to_nurbs(&read_geometry(i, "geometry")?));
// #endregion 🔖️Utilities

// #region 🔖️IO
pub(super) struct ExportStep(pub(super) SessionCapture);

impl Operator for ExportStep {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let value = semio_s_artifact_stdio_step_geometry::geometry::export_step(kernel, &[geometry]).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("step", text_dictionary(value)))
        })
    }
}

pub(super) struct ExportStl(pub(super) SessionCapture);

impl Operator for ExportStl {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let deflection = read_channel_number(input, "deflection")?;
            let data = kernel.export_stl(&[geometry], deflection).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("stl", text_dictionary(encode_base64(&data))))
        })
    }
}

pub(super) struct ExportObj(pub(super) SessionCapture);

impl Operator for ExportObj {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let deflection = read_channel_number(input, "deflection")?;
            let value = kernel.export_obj(&[geometry], deflection).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("obj", text_dictionary(value)))
        })
    }
}

pub(super) struct ImportStep(pub(super) SessionCapture);

impl Operator for ImportStep {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let data = read_text(input, "data")?;
            let shapes = semio_s_artifact_stdio_step_geometry::geometry::import_step(kernel, &data).map_err(|error| map_kernel_error(&error))?;
            let handle = shapes.into_iter().next().ok_or_else(|| EvalError::InvalidInput("step import produced no solids".into()))?;
            Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct ImportStl(pub(super) SessionCapture);

impl Operator for ImportStl {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let data = decode_base64(&read_text(input, "data")?)?;
            let tolerance = read_channel_number(input, "tolerance")?;
            let handle = kernel.import_stl(&data, tolerance).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct ImportObj(pub(super) SessionCapture);

impl Operator for ImportObj {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let data = read_text(input, "data")?;
            let tolerance = read_channel_number(input, "tolerance")?;
            let handle = kernel.import_obj(&data, tolerance).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct ExportDwg(pub(super) SessionCapture);

impl Operator for ExportDwg {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let deflection = read_channel_number(input, "deflection")?;
            let data = semio_s_artifact_stdio_semio_brep_export::dwg::export(kernel, &[geometry], deflection).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("dwg", text_dictionary(encode_base64(&data))))
        })
    }
}

pub(super) struct ImportDwg(pub(super) SessionCapture);

impl Operator for ImportDwg {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let data = decode_base64(&read_text(input, "data")?)?;
            let tolerance = read_channel_number(input, "tolerance")?;
            let handle = semio_s_artifact_stdio_semio_brep_import::dwg::import(kernel, &data, tolerance).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
        })
    }
}
// #endregion 🔖️IO
