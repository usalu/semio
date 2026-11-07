//! 🍂️ v2 snippet (UNCOMPILED, not wired by apply.py yet): `follow_derivable_children` as a resumable per-child step.
//! Replaces the one-shot body at `children/📥️staging/🚪️ingress/🦀️.rs`; the one-shot name stays as a loop over the step for non-poll callers (tests).

/// 🧭️ One follow pass in flight: the parent generation it plans for and the three worklists it drains one child per unit.
pub struct ChildFollowPlan {
    pub generation: u64,
    pub deferred: bool,
    pub declared_at: usize,
    pub genesis: Vec<(String, String, ArtifactRef, Vec<u8>)>,
    pub retire: Vec<(String, String)>,
    pub stage: u8,
}

/// 📊️ What one step did; `work` is charged with `cx.consume_fuel(work)` by the host.
pub struct ChildFollowProgress { pub done: bool, pub work: u64 }

pub const CHILD_FOLLOW_PRICING: ChildFollowPricing = ChildFollowPricing { project_per_child: 1, genesis_per_child: 16, retire_per_child: 8, open_per_child: 64 };

impl<M: SpaceMember + MemberFactory + 'static> ChildComposition<M> {
    /// Stage 0 scans the projection one declared child per unit (cursor `declared_at`), stage 1 reclaims retirements once, stage 2 retires one child per unit,
    /// stage 3 opens one genesis child per unit (`open_child` is the indivisible unit). Takes at most half of `budget` per call, always at least one unit.
    pub async fn follow_derivable_children_step<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>, identity: &mut EntityIdentityAuthority<'_>, budget: u64) -> Result<ChildFollowProgress, Fault> {
        let mut work = 0u64;
        let allowance = (budget / 2).max(1);
        if self.child_follow_plan.is_none() {
            let generation = stores.store.generation_now();
            if generation == self.followed_parent_generation || self.children.has_admission_in_flight() { return Ok(ChildFollowProgress { done: true, work }) }
            self.child_follow_plan = Some(ChildFollowPlan { generation, deferred: false, declared_at: 0, genesis: Vec::new(), retire: Vec::new(), stage: 0 });
        }
        loop {
            let Some(plan) = self.child_follow_plan.as_mut() else { break };
            let unit = match plan.stage { 0 => CHILD_FOLLOW_PRICING.project_per_child, 1 => CHILD_FOLLOW_PRICING.retire_per_child, 2 => CHILD_FOLLOW_PRICING.retire_per_child, _ => CHILD_FOLLOW_PRICING.open_per_child };
            if work != 0 && work + unit > allowance { return Ok(ChildFollowProgress { done: false, work }) }
            match plan.stage {
                0 => {
                    let at = plan.declared_at;
                    let snapshot = stores.store.snapshot_ref();
                    let projection = ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| plugin_sdk_fault(format!("derivable child projection failed: {error}")))?;
                    match projection.get(at) {
                        Some((slot, fields)) => {
                            let key = (slot.to_string(), fields.child_id.to_string());
                            if self.children.get(&key).is_none() {
                                if semio_plugin_child::member_is_retiring(&self.child_member_retirements, fields.artifact_id) { plan.deferred = true }
                                else if let Some(pack) = A::genesis_child_pack(snapshot, slot, fields.child_id).map_err(|error| plugin_sdk_fault(error.into_message()))? {
                                    let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
                                    plan.genesis.push((key.0, key.1, ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect }, pack));
                                }
                            }
                            plan.declared_at += 1;
                            work += unit + CHILD_FOLLOW_PRICING.genesis_per_child;
                        }
                        None => {
                            for (member, _) in self.children.keyed_entries().filter(|(member, _)| member.owner.is_empty()) {
                                let declared = (0..projection.len()).filter_map(|index| projection.get(index)).any(|(slot, fields)| *slot == *member.slot && *fields.child_id == *member.child_id);
                                if !declared && plan.genesis.iter().any(|(slot, _, _, _)| slot.as_str() == &*member.slot) { plan.retire.push((member.slot.to_string(), member.child_id.to_string())) }
                            }
                            plan.stage = 1;
                        }
                    }
                }
                1 => {
                    let publications = plan.retire.len() + plan.genesis.len();
                    if !self.reclaim_child_retirements(publications, publications)? { self.child_follow_awaits_retirement = true; self.child_follow_plan = None; return Ok(ChildFollowProgress { done: true, work }) }
                    if let Some(plan) = self.child_follow_plan.as_mut() { plan.stage = 2 }
                    work += unit;
                }
                2 => match plan.retire.pop() {
                    Some((slot, child_id)) => { self.retire_followed_child(&slot, &child_id).await?; work += unit }
                    None => plan.stage = 3,
                },
                _ => match plan.genesis.pop() {
                    Some((slot, child_id, expected, initial_pack)) => {
                        let dialect = expected.dialect.clone();
                        let schema = genesis_member_schema::<M>(&dialect)?;
                        let parent = ArtifactRef { artifact_id: stores.store.envelope().id.clone(), dialect: A::DIALECT.into() };
                        let owner = store::OwnerRef { parent, slot: slot.clone(), child_id: child_id.clone() };
                        let envelope_pack = store::os_store::text_format::genesis_member_envelope_pack(schema, &expected, &owner, &initial_pack).await.map_err(|error| plugin_sdk_fault(format!("followed child {slot}/{child_id} envelope: {error}")))?;
                        self.open_child(stores, slot, child_id, dialect, &envelope_pack, identity).await?;
                        work += unit;
                    }
                    None => {
                        let plan = self.child_follow_plan.take().expect("plan present");
                        if !plan.deferred { self.followed_parent_generation = plan.generation }
                        self.child_follow_awaits_retirement = plan.deferred;
                        return Ok(ChildFollowProgress { done: true, work });
                    }
                },
            }
        }
        Ok(ChildFollowProgress { done: true, work })
    }
}
