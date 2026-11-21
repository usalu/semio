
impl JsonGrammarCursor<DslValue> {
    /// 🏷️ Selects a numeric array and borrows the leading discriminator values already parsed.
    pub fn selected_number_array_tags<const N: usize>(
        &self, source: &[u8], path: &[&str], tags: &[(usize, &str); N],
    ) -> Option<(usize, [Option<&DslValue>; N])> {
        let start = self.selected_number_array_start(source, path)?;
        let discriminators = std::array::from_fn(|index| {
            let (layer, key) = tags[index];
            let (name, value) = self.frames.get(layer)?.entries.get(0)?;
            (name == key).then_some(value)
        });
        Some((start, discriminators))
    }
}
