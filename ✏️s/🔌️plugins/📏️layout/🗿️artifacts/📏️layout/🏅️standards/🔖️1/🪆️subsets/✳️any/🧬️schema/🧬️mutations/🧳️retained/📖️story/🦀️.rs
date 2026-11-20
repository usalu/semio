//! ✂️ Original retained story splice preparation.
pub use semio_s_artifact_layout_layout_mutations_decisions::story_plan as plan;
pub use semio_s_artifact_layout_layout_mutations_decisions::story_text as text;
pub use semio_s_artifact_layout_layout_mutations_decisions::story_source as source;
pub use semio_s_artifact_layout_layout_mutations_fields::story_output as output;

pub mod inverse {
    pub type StoryInverse = semio_s_artifact_layout_layout_mutations_fields::story_inverse::StoryInverse<crate::LayoutMutation>;
}
