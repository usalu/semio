impl JsonGrammarCursor<DslValue> {
    /// 🧭️ Detects a requested direct object path at the opening bracket of its array.
    pub fn selected_number_array_start(&self, source: &[u8], path: &[&str]) -> Option<usize> {
        if self.validated_position != source.len() || self.lexeme.is_some() || self.pending.is_some()
            || self.result.is_some() || self.frames.len() != path.len() || path.is_empty()
            || source.get(self.position) != Some(&b'[')
        {
            return None;
        }
        for (index, member) in path.iter().enumerate() {
            let frame = self.frames.get(index)?;
            if !frame.object || frame.state != 4 || frame.key.as_deref() != Some(*member) {
                return None;
            }
        }
        Some(self.position)
    }

    /// 📩️ Advances past one validated numeric child and leaves its ordered object slot in place.
    pub fn accept_number_array_marker(
        &mut self, source: &[u8], path: &[&str], span: &crate::JsonNumberArraySpan<'_>,
        effect: &mut semio_framework_value::RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        let range = span.range(source).ok_or_else(|| ValueError::invariant("JSON number-array marker changed the enclosing source"))?;
        if self.selected_number_array_start(source, path) != Some(range.start) {
            return Err(ValueError::invariant("JSON number-array marker changed the selected object path"));
        }
        let demand = Self::number_array_marker_demand();
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(effect.grant(), demand)
            || effect.remaining_work_units()? == 0 || !effect.performed(1)?
        {
            return Ok(Default::default());
        }
        self.position = range.end;
        self.pending = Some(DslValue::Array(Vec::new()));
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
    }

    pub fn number_array_marker_demand() -> RetirementDemand {
        RetirementDemand {
            copy_bytes: size_of::<usize>() + size_of::<Option<DslValue>>(),
            depth: 4,
            ..Default::default()
        }
    }
}
