//! 🔁️ Job queues, patterns and initialize: the propagation queue and entropy heap, the visible cell drain, the pattern removal and addition, the initialization and the choice steps. See [`super`].

use super::*;

//#region 🧩️Job
impl<T: Topology + Clone> WfcJob<T> {
    pub(super) fn reset_queue(&mut self) {
        self.state.queue.clear();
        self.state.queue_epoch = self.state.queue_epoch.wrapping_add(1).max(1);
    }

    pub(super) fn push_queue(&mut self, node: NodeId) {
        if self.state.queued_marks[node.index()] != self.state.queue_epoch {
            self.state.queued_marks[node.index()] = self.state.queue_epoch;
            self.state.queue.push_back(node);
        }
    }

    pub(super) fn pop_queue(&mut self) -> Option<NodeId> {
        let node = self.state.queue.pop_front()?;
        self.state.queued_marks[node.index()] = 0;
        Some(node)
    }

    pub(super) fn entropy(&self, node: NodeId) -> f64 {
        let index = node.index();
        let sum = self.state.domain_weight_sums[index];
        if sum <= 0.0 {
            0.0
        } else {
            sum.ln() - self.state.domain_weighted_log_sums[index] / sum
        }
    }

    pub(super) fn push_entropy(&mut self, node: NodeId) {
        if self.state.domain_counts[node.index()] > 1 {
            self.state.entropy_heap.push(EntropyEntry { entropy_bits: self.entropy(node).to_bits(), node, revision: self.state.revisions[node.index()] });
        }
    }

    fn change_count(&mut self, node: NodeId, old: u32, new: u32) {
        self.state.singleton_count = self.state.singleton_count + usize::from(new == 1) - usize::from(old == 1);
        self.state.empty_count = self.state.empty_count + usize::from(new == 0) - usize::from(old == 0);
        self.state.domain_counts[node.index()] = new;
    }

    fn note_visible(&mut self, node: NodeId, pattern: PatternId, discarded: bool) {
        self.visible.push(VisibleCell { node: node.get(), pattern: pattern.get(), discarded });
    }

    /// 👁 Moves the collapses and discards recorded since the last drain into `into`.
    pub fn drain_visible(&mut self, into: &mut Vec<VisibleCell>) {
        if !self.visible.is_empty() {
            into.append(&mut self.visible);
        }
    }

    pub(super) fn remove_pattern(&mut self, node: NodeId, pattern: PatternId, record: bool) {
        if !self.state.domains[node.index()].get(pattern) {
            return;
        }
        let old = self.state.domain_counts[node.index()];
        self.state.domains[node.index()].set(pattern, false);
        self.change_count(node, old, old - 1);
        let weight = self.model.weights().w(pattern);
        self.state.domain_weight_sums[node.index()] -= weight;
        self.state.domain_weighted_log_sums[node.index()] -= self.model.weights().w_ln_w(pattern);
        if record {
            self.state.trail.push(Removal { node, pattern });
        }
        if old == 1 {
            self.note_visible(node, pattern, true);
        } else if old == 2 {
            if let Some(remaining) = self.state.domains[node.index()].first_set() {
                self.note_visible(node, remaining, false);
            }
        }
    }

    pub(super) fn add_pattern(&mut self, node: NodeId, pattern: PatternId) {
        if self.state.domains[node.index()].get(pattern) {
            return;
        }
        let old = self.state.domain_counts[node.index()];
        let previous = if old == 1 { self.state.domains[node.index()].first_set() } else { None };
        self.state.domains[node.index()].set(pattern, true);
        self.change_count(node, old, old + 1);
        let weight = self.model.weights().w(pattern);
        self.state.domain_weight_sums[node.index()] += weight;
        self.state.domain_weighted_log_sums[node.index()] += self.model.weights().w_ln_w(pattern);
        if let Some(previous) = previous {
            self.note_visible(node, previous, true);
        }
        if old == 0 {
            self.note_visible(node, pattern, false);
        }
    }

    pub(super) fn initialize_one(&mut self) {
        if self.state.init_node == self.topology.node_count() {
            self.state.stage = WfcStage::PropagateCompatibilityEdge;
            return;
        }
        let node = NodeId::from_index(self.state.init_node);
        match self.state.init_cursor.phase {
            InitPhase::Acquire => {
                if let Some(initial) = self.initial_domains.as_mut() {
                    let domain = std::mem::replace(&mut initial[self.state.init_node], PatternSet::new_empty(0));
                    assert_eq!(domain.len(), self.model.pattern_count());
                    self.state.init_cursor.domain = Some(domain);
                    self.state.init_cursor.phase = InitPhase::Fixed;
                } else {
                    self.state.init_cursor.domain = Some(PatternSet::with_word_capacity(self.model.pattern_count()));
                    self.state.init_cursor.phase = InitPhase::BuildFull;
                }
            }
            InitPhase::BuildFull => {
                let cursor = &mut self.state.init_cursor;
                let word_count = self.model.pattern_count().div_ceil(64);
                if cursor.word < word_count {
                    let remaining = self.model.pattern_count().saturating_sub(cursor.word * 64);
                    let word = if remaining >= 64 { u64::MAX } else { (1u64 << remaining) - 1 };
                    cursor.domain.as_mut().expect("initial domain").push_word(word);
                    cursor.word += 1;
                } else {
                    cursor.phase = InitPhase::Fixed;
                }
            }
            InitPhase::Fixed => {
                let cursor = &mut self.state.init_cursor;
                if cursor.fixed < self.fixed.len() {
                    let (fixed_node, pattern) = self.fixed[cursor.fixed];
                    cursor.fixed += 1;
                    if fixed_node == node {
                        if cursor.required.is_some_and(|required| required != pattern) {
                            self.state.contradiction = Some(node);
                        }
                        cursor.required = Some(pattern);
                    }
                } else {
                    cursor.phase = InitPhase::Measure;
                }
            }
            InitPhase::Measure => {
                let cursor = &mut self.state.init_cursor;
                if cursor.pattern < self.model.pattern_count() {
                    let pattern = PatternId::from_index(cursor.pattern);
                    cursor.pattern += 1;
                    let domain = cursor.domain.as_mut().expect("initial domain");
                    if domain.get(pattern) && cursor.required.is_some_and(|required| required != pattern) {
                        domain.set(pattern, false);
                    }
                    if domain.get(pattern) {
                        let weight = self.model.weights().w(pattern);
                        cursor.count += 1;
                        cursor.weight_sum += weight;
                        cursor.weighted_log_sum += self.model.weights().w_ln_w(pattern);
                    }
                } else {
                    let cursor = std::mem::take(&mut self.state.init_cursor);
                    self.state.domains.push(cursor.domain.expect("measured domain"));
                    if cursor.count == 1 {
                        if let Some(pattern) = self.state.domains.last().and_then(|domain| domain.first_set()) {
                            self.note_visible(node, pattern, false);
                        }
                    }
                    self.state.domain_counts.push(cursor.count);
                    self.state.domain_weight_sums.push(cursor.weight_sum);
                    self.state.domain_weighted_log_sums.push(cursor.weighted_log_sum);
                    self.state.singleton_count += usize::from(cursor.count == 1);
                    self.state.empty_count += usize::from(cursor.count == 0);
                    self.state.revisions.push(0);
                    self.state.queued_marks.push(0);
                    if cursor.count == 0 {
                        self.state.contradiction = Some(node);
                    }
                    self.push_queue(node);
                    self.push_entropy(node);
                    self.state.init_node += 1;
                }
            }
        }
    }

    pub(super) fn find_slot(&mut self) {
        if let Some(entry) = self.state.entropy_heap.pop() {
            if self.state.revisions[entry.node.index()] == entry.revision && self.state.domain_counts[entry.node.index()] > 1 {
                self.state.active_slot = Some(entry.node);
                self.state.stage = WfcStage::ChooseCandidate;
            }
            return;
        }
        self.state.stage = if self.state.singleton_count == self.state.domains.len() { WfcStage::Complete } else { WfcStage::DetectContradiction };
    }

    pub(super) fn choose_one(&mut self) {
        let node = self.state.active_slot.expect("choose stage has active slot");
        if self.state.choice_cursor.is_none() {
            let cursor = match self.config.sampler {
                WfcSampler::Uniform => ChoiceCursor { phase: ChoicePhase::Select, pattern: 0, total: 0.0, target: 0.0, running: 0.0, ordinal: self.state.rng.range(self.state.domain_counts[node.index()] as u64), candidate: None },
                WfcSampler::WeightedRoulette => ChoiceCursor {
                    phase: ChoicePhase::Weigh,
                    pattern: 0,
                    total: 0.0,
                    target: 0.0,
                    running: 0.0,
                    ordinal: 0,
                    candidate: None,
                },
            };
            self.state.choice_cursor = Some(cursor);
            return;
        }
        let cursor = self.state.choice_cursor.as_mut().expect("choice cursor");
        match cursor.phase {
            ChoicePhase::Weigh => {
                if cursor.pattern < self.model.pattern_count() {
                    let pattern = PatternId::from_index(cursor.pattern);
                    cursor.pattern += 1;
                    if self.state.domains[node.index()].get(pattern) {
                        cursor.total += self.model.weights().w(pattern);
                    }
                } else {
                    cursor.target = self.state.rng.next_f64() * cursor.total;
                    cursor.pattern = 0;
                    cursor.phase = ChoicePhase::Select;
                }
            }
            ChoicePhase::Select => {
                if cursor.pattern < self.model.pattern_count() {
                    let pattern = PatternId::from_index(cursor.pattern);
                    cursor.pattern += 1;
                    if self.state.domains[node.index()].get(pattern) {
                        let selected = match self.config.sampler {
                            WfcSampler::Uniform => {
                                let selected = cursor.ordinal == 0;
                                cursor.ordinal = cursor.ordinal.saturating_sub(1);
                                selected
                            }
                            WfcSampler::WeightedRoulette => {
                                cursor.running += self.model.weights().w(pattern);
                                cursor.running >= cursor.target
                            }
                        };
                        if selected {
                            cursor.candidate = Some(pattern);
                            cursor.pattern = 0;
                            cursor.phase = ChoicePhase::Remove;
                            self.state.decisions.push(Decision { node, candidate: pattern, trail_mark: self.state.trail.len(), rng_state: self.state.rng.state });
                            self.state.tested_tile = Some(pattern);
                            self.state.observations += 1;
                            self.state.observed.push((node, pattern));
                        }
                    }
                } else {
                    let fallback = self.state.domains[node.index()].first_set().expect("unresolved domain");
                    cursor.candidate = Some(fallback);
                    cursor.pattern = 0;
                    cursor.phase = ChoicePhase::Remove;
                    self.state.decisions.push(Decision { node, candidate: fallback, trail_mark: self.state.trail.len(), rng_state: self.state.rng.state });
                    self.state.tested_tile = Some(fallback);
                    self.state.observations += 1;
                    self.state.observed.push((node, fallback));
                }
            }
            ChoicePhase::Remove => {
                let candidate = cursor.candidate.expect("selected candidate");
                if cursor.pattern < self.model.pattern_count() {
                    let pattern = PatternId::from_index(cursor.pattern);
                    cursor.pattern += 1;
                    if pattern != candidate {
                        self.remove_pattern(node, pattern, true);
                    }
                } else {
                    self.state.revisions[node.index()] += 1;
                    self.reset_queue();
                    self.push_queue(node);
                    self.state.choice_cursor = None;
                    self.state.stage = WfcStage::PropagateCompatibilityEdge;
                }
            }
        }
    }

}
//#endregion 🧩️Job
