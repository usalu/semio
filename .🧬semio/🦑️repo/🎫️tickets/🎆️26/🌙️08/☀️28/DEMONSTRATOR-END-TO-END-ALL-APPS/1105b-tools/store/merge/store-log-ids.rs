    /// 🔢️ How many edit rows this replica's log ledger holds (committed or not): the extent a paged id census walks.
    pub fn event_log_edit_count(&self) -> usize {
        self.envelope.vcs.edits.iter().count()
    }

    /// 🔢️ How many history transitions this replica's log holds.
    pub fn event_log_transition_count(&self) -> usize {
        self.envelope.transitions.iter().count()
    }

    /// 🧾️ The operation ids of the committed edit rows `from..from + limit` in ledger order — [`Self::event_log`]'s ids without building any envelope.
    pub fn event_log_edit_ids_page(&self, from: usize, limit: usize) -> Vec<MutationId> {
        self.envelope.vcs.edits.iter().skip(from).take(limit).filter(|edit| self.envelope.holds_committed_edit(&edit.id)).flat_map(|edit| semio_os_spr::mutation_ids_for_edit::<P, Mutation>(edit)).collect()
    }

    /// 🧾️ The ids of the history transitions `from..from + limit` in ledger order.
    pub fn event_log_transition_ids_page(&self, from: usize, limit: usize) -> Vec<MutationId> {
        self.envelope.transitions.iter().skip(from).take(limit).map(|transition| transition.mutation_id.clone()).collect()
    }

