    //#region 🧪️MutationReport
    /// 🧪️ The report every language-neutral mutation case compares against its committed specification vector. The
    /// base and expected-after snapshots and the mutation are decoded through the production JSON codec, the mutation is
    /// applied through `Mutation::diff(..).apply_to` (so an apply refusal surfaces as the fatal message production
    /// dispatch records), and the mutation's own computed inverse steps are replayed in order onto the applied snapshot.
    /// The forward half is `base`, `expectedSnapshot`, `snapshot`, `diff`, `messages`; the inverse half is
    /// `inverseSteps`, `inverseSnapshot`, `inverseMessages`. `expectedSnapshot` is decoded through the same path as
    /// `base`, so a caller compares like with like.
    ///
    /// @see ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs — the laws case adapters assert over this report.
    pub fn mutation_report_json<S, M>(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String>
    where
        S: Clone + ToValue + FromValue,
        M: Mutation<S>,
    {
        let decode_snapshot = |text: &str| -> Result<S, String> { crate::os_pack::json::from_json_str(text).map_err(|error| error.to_string()) };
        let base = decode_snapshot(base_json)?;
        let expected = decode_snapshot(after_json)?;
        let mutation: M = crate::os_pack::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
        let mut applied = base.clone();
        let forward = mutation.diff(&base).apply_to(&mut applied);
        let inverse = mutation.inverse(&base);
        let mut undone = applied.clone();
        let mut inverse_messages = Vec::new();
        for step in &inverse {
            inverse_messages.extend(step.diff(&undone).apply_to(&mut undone).messages().iter().cloned());
        }
        let json = |value: DslValue| crate::os_pack::json::from_dsl_value(&value);
        let report = crate::os_pack::json::object([
            ("base".to_string(), json(base.to_value())),
            ("expectedSnapshot".to_string(), json(expected.to_value())),
            ("snapshot".to_string(), json(applied.to_value())),
            ("diff".to_string(), json(forward.diff().to_value())),
            ("messages".to_string(), json(forward.messages().to_vec().to_value())),
            ("inverseSteps".to_string(), json(inverse.to_value())),
            ("inverseSnapshot".to_string(), json(undone.to_value())),
            ("inverseMessages".to_string(), json(inverse_messages.to_value())),
        ]);
        Ok(crate::os_pack::json::to_string(&report))
    }
    //#endregion 🧪️MutationReport
