pub struct PlaygroundEngine { projection: crate::artifacts::playground::mutations::PlaygroundDocument }
impl protocol::ArtifactEngine for PlaygroundEngine {
    type Projection = crate::artifacts::playground::mutations::PlaygroundDocument;
    type Mutation = crate::artifacts::playground::mutations::PlaygroundMutation;
    type Diff = crate::artifacts::playground::mutations::PlaygroundMutation;
    fn projection(&self) -> &Self::Projection { &self.projection }
    fn apply(&mut self, m: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        crate::artifacts::playground::mutations::apply_playground_mutation(&mut self.projection, m);
        Ok(m.diff(&self.projection))
    }
    fn inverse(&self, m: &Self::Mutation) -> Vec<Self::Mutation> { m.inverse(&self.projection) }
}
