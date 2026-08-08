use crate::artifacts::vcs::VcsDemoProjection;
pub fn apply(projection: &mut VcsDemoProjection, title: &str) {
    projection.title = title.to_string();
}
