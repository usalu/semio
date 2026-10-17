//! 🥽️ Indexed mesh widgets with explicit polygon data and B-Rep preview conversion.

#[macro_export]
macro_rules! retire_geometry_capture {
    ($field:tt) => {
        fn retirement_is_empty(&self) -> bool { self.$field.terminal_is_empty() }
        fn retire_step(&mut self, grant:semio_framework_value::retained_clone::RetainedCloneGrant, _: &mut neural_engine::ValueRetirement) -> Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError> { self.$field.close_step(grant) }
        fn next_retire_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {self.$field.next_close_copy_byte_demand()}
        fn next_retire_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError> {self.$field.next_close_capacity_byte_demand(copy)}
        fn next_retire_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {self.$field.next_close_release_byte_demand()}
        fn next_retire_depth_demand(&self)->Result<usize,semio_framework_value::ValueError> {self.$field.next_close_depth_demand()}
        fn retire_cold(mut self:Box<Self>) { self.$field.retire_cold(); }
    };
}

#[path = "🧰️helpers/🦀️.rs"]
mod helpers;
pub use helpers::*;
#[path = "🧮️operator-core/🦀️.rs"]
mod operator_core;
pub use operator_core::*;
#[path = "📇️registration/🦀️.rs"]
mod registration;
pub use registration::*;
