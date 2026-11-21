extern crate semio_framework_3d_brep_kernel_contract as contract;
extern crate semio_framework_3d_brep_native_registry as registry;

fn classify(left: Result<(), contract::BrepError>, right: Result<(), contract::BrepError>) -> u32 {
    match (left, right) {
        (Ok(()), Ok(())) => 0,
        (Err(contract::BrepError::InvalidInput(left)), Err(contract::BrepError::InvalidInput(right)))
            if left == right && left.starts_with("fixture field must be ") => 1,
        _ => 2,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn finite_vector(x: f64, y: f64, z: f64) -> u32 {
    let point = [x, y, z];
    classify(contract::require_finite_vector("fixture field", point), registry::require_finite_vector("fixture field", point))
}

#[unsafe(no_mangle)]
pub extern "C" fn finite_scalar(value: f64) -> u32 {
    classify(contract::require_finite_scalar("fixture field", value), registry::require_finite_scalar("fixture field", value))
}

#[unsafe(no_mangle)]
pub extern "C" fn positive_size(value: f64) -> u32 {
    classify(contract::require_positive_size("fixture field", value), registry::require_positive_size("fixture field", value))
}

#[unsafe(no_mangle)]
pub extern "C" fn scale_factor(value: f64) -> u32 {
    classify(contract::require_scale_factor("fixture field", value), registry::require_scale_factor("fixture field", value))
}
