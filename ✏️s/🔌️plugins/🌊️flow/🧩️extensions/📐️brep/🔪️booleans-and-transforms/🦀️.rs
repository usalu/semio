//! 🔪️ Booleans and transforms: the boolean operation macro with its kernel plan job and the compound cut and transform operators. See [`super`].

use super::*;

// #region 🔖️Booleans
/// ⏱️ BRep set operations retain kernel plans across evaluator turns; mesh modeling jobs use the same domain-neutral scheduler.
macro_rules! boolean_operation {
    ($name:ident, $op:expr) => {
        pub(super) struct $name(pub(super) SessionCapture);
        impl Operator for $name {
            retire_geometry_capture!(0);
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                self.0.with_kernel(|kernel| {
                    let handle = kernel.boolean_sync(&read_geometry(input, "a")?, &read_geometry(input, "b")?, $op).map_err(|error| map_kernel_error(&error))?;
                    Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
                })
            }

            retained_geometry_job!($name);
        }
    };
}

boolean_operation!(Fuse, BooleanOp::Unite);

boolean_operation!(Cut, BooleanOp::Cut);

boolean_operation!(Intersect, BooleanOp::Intersect);

pub(super) struct CompoundCut(pub(super) SessionCapture);

impl Operator for CompoundCut {
    retire_geometry_capture!(0);
    retained_geometry_job!(CompoundCut);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let target = read_geometry(input, "target")?;
            let tools = read_geometry_list(input, "tools")?;
            let handle = kernel.compound_cut(&target, &tools).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}
// #endregion 🔖️Booleans

// #region 🔖️Transforms
geo_operation!(Translate, "geometryOut", |k, i| k.translate(&read_geometry(i, "geometry")?, read_xyz(i, "offset")?), Translate);

geo_operation!(Rotate, "geometryOut", |k, i| k.rotate(&read_geometry(i, "geometry")?, read_xyz(i, "axis")?, read_channel_number(i, "angle")?), Rotate);

geo_operation!(RotateAbout, "geometryOut", |k, i| k.rotate_about(&read_geometry(i, "geometry")?, read_xyz(i, "origin")?, read_xyz(i, "axis")?, read_channel_number(i, "angle")?), RotateAbout);

geo_operation!(Scale, "geometryOut", |k, i| k.scale_axes(&read_geometry(i, "geometry")?, read_xyz(i, "factor")?, read_xyz(i, "center")?), Scale);

geo_operation!(Mirror, "geometryOut", |k, i| k.mirror(&read_geometry(i, "geometry")?, read_xyz(i, "origin")?, read_xyz(i, "normal")?), Mirror);

geo_operation!(CopyShape, "geometryOut", |k, i| k.copy_shape(&read_geometry(i, "geometry")?), Copy);

geo_operation!(LinearPattern, "compound", |k, i| k.linear_pattern(&read_geometry(i, "geometry")?, read_xyz(i, "direction")?, read_channel_number(i, "spacing")?, read_channel_number(i, "count")? as usize,), LinearPattern);

geo_operation!(CircularPattern, "compound", |k, i| k.circular_pattern(&read_geometry(i, "geometry")?, read_xyz(i, "axis")?, read_channel_number(i, "count")? as usize,), CircularPattern);

geo_operation!(GridPattern, "compound", |k, i| k.grid_pattern(
    &read_geometry(i, "geometry")?,
    read_xyz(i, "dirX")?,
    read_xyz(i, "dirY")?,
    read_channel_number(i, "spacingX")?,
    read_channel_number(i, "spacingY")?,
    read_channel_number(i, "countX")? as usize,
    read_channel_number(i, "countY")? as usize,
), GridPattern);
// #endregion 🔖️Transforms
