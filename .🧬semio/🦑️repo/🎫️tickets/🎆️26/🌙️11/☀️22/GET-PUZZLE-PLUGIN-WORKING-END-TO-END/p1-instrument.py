p="/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs"
s=open(p).read()
a="    pub fn close_step(&mut self, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, semio_framework_value::RetirementPollExit>\n    where P: Send + Sync + 'static, Mutation: Send + 'static,\n    {\n        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress};\n        let grant = effect.grant();\n        self.begin_close();"
assert s.count(a)==1, s.count(a)
b="""    pub fn close_step(&mut self, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, semio_framework_value::RetirementPollExit>
    where P: Send + Sync + 'static, Mutation: Send + 'static,
    {
        let label = if self.original_origins.is_some()||!self.origins_retirement.is_empty(){"origins"}else if self.transaction_close.is_some(){"transaction_close"}else if self.transaction.is_some(){"transaction"}else if self.lane_close.is_some(){"lane_close"}else if self.lane_insert.is_some(){"lane_insert"}else if self.preparation.is_some(){"preparation"}else if self.source.is_some(){"source"}else if self.stage.is_some(){"stage"}else if self.verb.is_some(){"verb"}else if self.receipt.is_some(){"receipt"}else if self.authority_retirement.is_some(){"authority_retirement"}else if self.authority.is_some()||self.refused_authority.is_some(){"authority"}else if self.registry_retirement.is_some()||self.original_registry.is_some(){"registry"}else{"other"};
        let r = self.close_step_unlabeled(effect);
        if let Ok(step) = &r { eprintln!("[DEBUG] pclose {label} items={}", step.progress().copied_items); }
        r
    }

    pub fn close_step_unlabeled(&mut self, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, semio_framework_value::RetirementPollExit>
    where P: Send + Sync + 'static, Mutation: Send + 'static,
    {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress};
        let grant = effect.grant();
        self.begin_close();"""
open(p,"w").write(s.replace(a,b))
