# Relocation Declaration Audit

[
  {
    "index": 46,
    "source": "🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️shared/🔒️sealed/🦀️.rs",
    "targets": [
      "🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🔒️sealed/🦀️.rs"
    ],
    "names": [
      [
        "close_step"
      ],
      [
        "next_demand",
        "close_step"
      ]
    ],
    "balance": [
      0,
      0
    ],
    "old": " fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete;}match self.step(grant){Ok(progress)=>RetirementStep::Progress(progress),Err(error)=>RetirementStep::Failure(error)}}\n",
    "new": " fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}\n fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete;}self.step(grant).unwrap_or_else(RetirementStep::Failure)}\n"
  },
  {
    "index": 47,
    "source": "🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️shared/🦀️.rs",
    "targets": [
      "🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🦀️.rs"
    ],
    "names": [
      [
        "next_copy_byte_demand"
      ],
      [
        "next_demand",
        "next_copy_byte_demand"
      ]
    ],
    "balance": [
      0,
      0
    ],
    "old": "    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.owned.as_ref().map_or(Ok(0),ControlledRetirement::next_copy_byte_demand)}\n",
    "new": "    /// 🎟️ One original shared lease or one original controlled payload supplies the complete action quote.\n    pub fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{\n        if let Some(source)=self.source.as_ref(){return Ok(crate::RetirementDemand{release_bytes:if (self.lease_only||Arc::strong_count(source)==1)&&Arc::weak_count(source)==0{arc_bytes::<T>()}else{0},depth:1,..Default::default()});}\n        self.owned.as_ref().map_or(Ok(Default::default()),|owner|owner.n"
  },
  {
    "index": 91,
    "source": "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🦀️.rs",
    "targets": [
      "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔎️lookup-cursor/🦀️.rs"
    ],
    "names": [
      [
        "next_close_copy_byte_demand",
        "next_close_capacity_byte_demand",
        "next_close_release_byte_demand",
        "next_close_depth_demand"
      ],
      [
        "close_demands",
        "next_close_copy_byte_demand",
        "next_close_capacity_byte_demand",
        "next_close_release_byte_demand",
        "next_close_depth_demand"
      ]
    ],
    "balance": [
      0,
      0
    ],
    "old": "    pub fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){c.next_close_copy_byte_demand()}else{RetainedCloneBinding::copy_demand(if self.source.is_some(){&self.source}else{&self.target})}}\n    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){c.next_close_capacity_byte_demand(body)}else{RetainedCloneBinding::capacity_demand(if self.source.is_some(){&self.so",
    "new": "    /// 🎟️ Retains the original comparator or projected binding through one complete next-action quote.\n    pub(super) fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{\n        if let Some(cursor)=self.comparison.as_ref(){if cursor.terminal_is_empty(){return Ok(crate::RetirementDemand{copy_bytes:size_of::<K::Cursor>(),depth:1,..Default::default()});}let mut d=cursor.close_demands(body)?;d.depth=d.depth.checked_add(1).ok_or_else(||crate::ValueError::literal(cr"
  },
  {
    "index": 96,
    "source": "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🦀️.rs",
    "targets": [
      "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs"
    ],
    "names": [
      [
        "next_close_copy_byte_demand",
        "next_close_capacity_byte_demand"
      ],
      [
        "close_demands"
      ]
    ],
    "balance": [
      1,
      1
    ],
    "old": "    pub fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> { let state=&*self.state; if !state.lookup.terminal_is_empty(){state.lookup.next_close_copy_byte_demand()}else if !state.close.is_empty(){state.close.next_copy_byte_demand()}else if state.key.is_some()||state.value.is_some()||state.old_directory.is_some()||state.new_directory.is_some()||state.map.is_some()||state.output.is_some(){Ok(0)}else{state.lookup_lease.next_close_copy_byte_demand()} }\n    pub fn next_close_c",
    "new": "    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError> {\n        let state=&*self.state;\n        if !state.lookup.terminal_is_empty(){return state.lookup.close_demands(body);}\n        if !state.close.is_empty(){return state.close.next_owner_demands::<K>(false,&None,body);}\n        if state.key.is_some(){return state.close.next_owner_demands::<K>(true,&None,body);}\n        if state.value.is_some(){return state.close.next_owner_demands::<V>(true,&None,body);}\n   "
  },
  {
    "index": 97,
    "source": "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🦀️.rs",
    "targets": [
      "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/➕️insert-cursor/🦀️.rs"
    ],
    "names": [
      [
        "next_close_release_byte_demand",
        "next_close_depth_demand",
        "close_step"
      ],
      [
        "next_close_copy_byte_demand",
        "next_close_capacity_byte_demand",
        "next_close_release_byte_demand",
        "next_close_depth_demand",
        "close_step"
      ]
    ],
    "balance": [
      1,
      1
    ],
    "old": "    pub fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> { let state=&*self.state; if !state.lookup.terminal_is_empty(){state.lookup.next_close_release_byte_demand()}else if !state.close.is_empty(){state.close.next_release_byte_demand()}else if state.key.is_some()||state.value.is_some()||state.old_directory.is_some()||state.new_directory.is_some()||state.map.is_some()||state.output.is_some(){Ok(0)}else{state.lookup_lease.next_close_release_byte_demand()} }\n    pub fn ",
    "new": "    pub fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.copy_bytes)}\n    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}\n    pub fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.release_bytes)}\n    pub fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.depth)}\n    pub f"
  },
  {
    "index": 108,
    "source": "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🦀️.rs",
    "targets": [
      "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧮️array-cursor/🦀️.rs"
    ],
    "names": [
      [
        "advance"
      ],
      [
        "advance_demands"
      ]
    ],
    "balance": [
      2,
      2
    ],
    "old": "    fn advance(&mut self, source: RetainedCloneRef<'_, [T; N]>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {\n        if self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, \"retained array clone cursor is closing\")); }\n        if self.spent { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, \"retained array clone cursor is spent\")); }\n        if grant.maximum_items == 0 && grant.maximum_copy_bytes",
    "new": "    fn advance_demands(&self, source: RetainedCloneRef<'_, [T; N]>, body: usize) -> Result<crate::RetirementDemand, crate::ValueError> {\n        if self.closing || self.spent { return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,\"array clone cannot quote a closing or spent owner\")); }\n        if !source.check_binding(&self.source)? { return Ok(crate::RetirementDemand { copy_bytes: source.binding_copy_bytes(), depth: 1, ..Default::default() }); }\n        if self.outpu"
  },
  {
    "index": 119,
    "source": "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🦀️.rs",
    "targets": [
      "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧮️array-cursor/🦀️.rs"
    ],
    "names": [
      [
        "next_close_depth_demand",
        "next_close_copy_byte_demand",
        "next_close_capacity_byte_demand",
        "next_close_release_byte_demand",
        "terminal_is_empty"
      ],
      [
        "close_demands",
        "next_close_depth_demand",
        "next_close_copy_byte_demand",
        "next_close_capacity_byte_demand",
        "next_close_release_byte_demand",
        "terminal_is_empty",
        "drop"
      ]
    ],
    "balance": [
      0,
      0
    ],
    "old": "    fn next_close_depth_demand(&self) -> Result<usize, crate::ValueError> {\n        if !self.closing { return Ok(0); }\n        if let Some(child) = self.child.as_ref() { return child.next_close_depth_demand(); }\n        self.close.next_owner_depth_with_binding(self.child_value.is_some() || self.output.is_some() || self.slots.iter().any(Option::is_some), &self.source)\n    }\n    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {\n        if !self.closing { return Ok(0); }\n ",
    "new": "    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if !self.closing{return Ok(Default::default());}\n        if let Some(child)=self.child.as_ref(){if !self.child_closing{return Ok(crate::RetirementDemand{depth:1,..Default::default()});}return if child.terminal_is_empty(){Ok(crate::RetirementDemand{copy_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}else{nested_retained_clone_close_demands::<T>(child,body)};}\n        if !self.close.is_empty(){re"
  },
  {
    "index": 207,
    "source": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts",
    "targets": [
      "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🎨️styling-resolve/🟦️.ts"
    ],
    "names": [
      [],
      []
    ],
    "balance": [
      0,
      5
    ],
    "old": "",
    "new": "    );\n  });\n\n  it(\"tints the hovered card with the veil so the page behind it stays the crisp surface\", () => {\n    expect(uiCss).toMatch(\n      /\\[data-layered-card\\]\\[data-tinted\\] > \\*::after \\{[\\s\\S]*?background-color: color-mix\\(in srgb, var\\(--surface-bg\\) calc\\(var\\(--veil-alpha\\) \\* 100%\\), transparent\\);/,\n    );\n    expect(uiCss).toMatch(\n      /@media \\(prefers-reduced-transparency: reduce\\) \\{[\\s\\S]*?\\[data-layered-card\\]\\[data-tinted\\] > \\*::after \\{\\s*background-color: var\\(--surf"
  },
  {
    "index": 347,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🪣️fill/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🪣️fill/📥️fill-job-and-snapshot-capture/🦀️.rs"
    ],
    "names": [
      [
        "step"
      ],
      [
        "next_step_demands",
        "page"
      ]
    ],
    "balance": [
      2,
      2
    ],
    "old": "    pub fn step(&mut self, scene: &BoardSceneState, style: &BoardStyleState, brush: &BoardBrushState) -> BoardFillCaptureStep {\n        if self.closing {\n            return BoardFillCaptureStep::Fault(BoardFillCaptureFault::GenerationExhausted);\n",
    "new": "    pub fn next_step_demands(&self, style: &BoardStyleState) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {\n        fn page<T, const N: usize>(pages: &BoardFillFixedPages<T, N>) -> usize {\n            if pages.len < BoardFillFixedPages::<T, N>::item_capacity() && pages.pages.get(pages.len / BOARD_FILL_PAGE_ITEMS).is_some_and(Option::is_none) { BOARD_FILL_PAGE_ITEMS * size_of::<T>() } else { 0 }\n"
  },
  {
    "index": 352,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🪣️fill/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🪣️fill/🧵️job-operation-and-state/🦀️.rs"
    ],
    "names": [
      [
        "with_operation"
      ],
      [
        "from_original_state"
      ]
    ],
    "balance": [
      1,
      1
    ],
    "old": "    pub fn with_operation(snapshot: BoardFillSnapshot, max_count: u32, operation: semio_framework_job::Operation) -> Self {\n",
    "new": "    fn from_original_state(snapshot: BoardFillSnapshot, max_count: u32, operation: semio_framework_job::Operation, original_source: semio_framework_action_bus::OriginalToolSourceBinding) -> Self {\n"
  },
  {
    "index": 368,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🎛️retained-vcs-state-and-begin/🦀️.rs"
    ],
    "names": [
      [
        "host_snapshot_mut"
      ],
      [
        "install_applied"
      ]
    ],
    "balance": [
      1,
      1
    ],
    "old": "    fn host_snapshot_mut(&mut self) -> &mut FlowHostSnapshot {\n        self.versions.get_mut(self.active).expect(\"active Flow VCS document version\")\n",
    "new": "    /// 🎯️ Installs the successor `protocol::apply_diff_cold` produced as the active version and hands the displaced version back to be retired cold.\n    fn install_applied(&mut self, mut next: FlowHostSnapshot) -> FlowHostSnapshot {\n        std::mem::swap(self.versions.get_mut(self.active).expect(\"active Flow VCS document version\"), &mut next);\n        next\n"
  },
  {
    "index": 374,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧮️cursor-steps/🦀️.rs"
    ],
    "names": [
      [
        "flow_vcs_step_shift"
      ],
      [
        "flow_vcs_apply_inverted",
        "flow_vcs_apply",
        "flow_vcs_wire_index",
        "flow_vcs_inverse_widget",
        "flow_vcs_inverse_synapse",
        "flow_vcs_inverse_changed_widget",
        "flow_vcs_inverse_changed_synapse",
        "flow_vcs_inverse_layout_entry",
        "flow_vcs_move_adjacent",
        "flow_vcs_step_shift"
      ]
    ],
    "balance": [
      1,
      1
    ],
    "old": "fn flow_vcs_step_shift(host_snapshot: &mut FlowHostSnapshot, operation: &mut FlowVcsOperation) -> Result<(), FlowVcsFault> {\n",
    "new": "/// 🎯️ The one write lane of the retained stepper. The concrete mutation's diff is built from reads of the active version and turned into a\n/// successor ONLY by [`protocol::apply_diff_cold`]; the successor replaces the displaced active version, which retires cold. Returns the mutation's\n/// own concrete inverse rows (built from the pre-state), which the caller keeps as its history inverse or retires.\nfn flow_vcs_apply_inverted(document: &mut FlowVcsDocument, mutation: FlowMutation) -> Result<Ve"
  },
  {
    "index": 569,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🧬️hydration-types-constructor-and-drive/🦀️.rs"
    ],
    "names": [
      [
        "RetainedConfigStoreHydration"
      ],
      [
        "ReferenceConfigStoreHydration"
      ]
    ],
    "balance": [
      0,
      0
    ],
    "old": "pub struct RetainedConfigStoreHydration<P, M>\n",
    "new": "pub struct ReferenceConfigStoreHydration<P, M>\n"
  },
  {
    "index": 657,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎲️space-members-macro/🦀️.rs"
    ],
    "names": [
      [
        "undo"
      ],
      [
        "undo"
      ]
    ],
    "balance": [
      1,
      3
    ],
    "old": "            async fn undo(&mut self, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {\n                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::undo(m.as_mut(), identity).await),+ }\n",
    "new": "            async fn undo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> where M:ReferenceSpaceMember {\n        let mut candidates = Vec::new();\n        for (document_id, member) in self.members.iter() {\n                if let Some(timestamp) =member.last_local_edit_timestamp().await {\n                candidates.push((timestamp.cmp_key(),  document_id.clone()));\n"
  },
  {
    "index": 660,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎲️space-members-macro/🦀️.rs"
    ],
    "names": [
      [
        "event_log_payload"
      ],
      [
        "event_log_payload",
        "announce_tail_edit_payload"
      ]
    ],
    "balance": [
      1,
      2
    ],
    "old": "            async fn event_log_payload(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {\n                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::event_log_payload(m.as_ref()).await),+ }\n",
    "new": "            async fn event_log_payload(&self) -> Result<Vec<u8>, VcsError> {\n        Ok(crate::os_spr::encode_envelopes(& self .event_log()?)) }\n\n            async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError> {\n        let Some(edit) = self.envelope.vcs.edits.last().filter(|edit| self.envelope.holds_committed_edit(&edit.id)) else { return Ok(Vec::new()) };\n        let document_id = ArtifactId(self.envelope.id.clone());\n        let schema = SchemaId(self.envelope.schema.c"
  },
  {
    "index": 661,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎲️space-members-macro/🦀️.rs"
    ],
    "names": [
      [
        "announce_tail_edit_payload",
        "ingest_remote_payload"
      ],
      [
        "ingest_remote_payload"
      ]
    ],
    "balance": [
      0,
      -1
    ],
    "old": "            async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, $crate::os_store::VcsError> {\n                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::announce_tail_edit_payload(m.as_mut()).await),+ }\n            }\n            async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<(), $crate::os_store::VcsError> {\n                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::ingest_remote_payload(m.as_mut(), envelopes).a",
    "new": "        Ok(crate::os_spr::encode_envelopes(&op_envelopes))\n    }\n            async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<Vec<crate::os_spr::MergeReport>, VcsError>  where P: Clone, Mutation: ColdMutation<P> {\n                let envelopes =crate::os_spr::decode_envelopes( envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;\n        let mut reports = Vec::with_capacity(envelopes.len());\n        for envelope in envelopes {\n            reports.push(self.i"
  },
  {
    "index": 673,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏪️store/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏪️store/🏭️member-factory-visitors-and-no-members/🦀️.rs"
    ],
    "names": [
      [
        "announce_tail_edit_payload"
      ],
      [
        "ingest_remote_payload"
      ]
    ],
    "balance": [
      0,
      1
    ],
    "old": "    async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError> {\n        match *self {}\n    }\n",
    "new": "    async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<Vec<semio_framework_os_spr::MergeReport>, VcsError>  where P: Clone, Mutation: ColdMutation<P> {\n        let envelopes = semio_framework_os_spr::decode_envelopes(envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;\n        let mut reports = Vec::with_capacity(envelopes.len());\n        for envelope in envelopes {\n            reports.push(self .ingest_remote(envelope).await?);}\n"
  },
  {
    "index": 674,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏪️store/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏪️store/🏭️member-factory-visitors-and-no-members/🦀️.rs"
    ],
    "names": [
      [
        "ingest_remote_payload"
      ],
      []
    ],
    "balance": [
      1,
      0
    ],
    "old": "    async fn ingest_remote_payload(&mut self, _envelopes: &[u8]) -> Result<(), VcsError> {\n        match *self {}\n",
    "new": "        Ok(reports)\n"
  },
  {
    "index": 702,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️wire/📥️lifted/🗄️durable-group/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️wire/📥️lifted/🗄️durable-group/📦️outcome-codecs-json-budget-and-map-handles/🦀️.rs"
    ],
    "names": [
      [
        "valid_edit_identities"
      ],
      [
        "valid_edit_identity_header",
        "valid_edit_identity_metadata"
      ]
    ],
    "balance": [
      0,
      0
    ],
    "old": "\npub fn valid_edit_identities<Mutation>(edit: &Edit<Mutation>) -> bool {\n    valid_identity(&edit.id)\n        && edit.actor.as_deref().is_none_or(valid_identity)\n        && edit.mutation_meta.iter().all(|meta| {\n            meta.mutation_id.as_ref().is_none_or(|id| valid_identity(&id.0))\n                && meta.dependencies.iter().all(|id| valid_identity(&id.0))\n                && meta.author_id.as_ref().is_none_or(|id| valid_identity(&id.0))\n                && meta.semantic_kind.as_ref().is_non",
    "new": "pub fn valid_edit_identity_header<Mutation>(edit:&Edit<Mutation>)->bool{valid_identity(&edit.id)&&edit.actor.as_deref().is_none_or(valid_identity)&&edit.line.as_deref().is_none_or(valid_identity)}\nfn valid_edit_identity_metadata(meta:&crate::os_spr::MutationMeta)->bool{meta.mutation_id.as_ref().is_none_or(|id|valid_identity(&id.0))&&meta.author_id.as_ref().is_none_or(|id|valid_identity(&id.0))&&meta.semantic_kind.as_ref().is_none_or(|id|valid_identity(&id.0))&&meta.group_id.as_deref().is_none_or"
  },
  {
    "index": 807,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌈️bin-03/🔬️unit/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌈️bin-03/🔬️unit/🧳️retained-clone-preparation-fixtures/🦀️.rs"
    ],
    "names": [
      [
        "advance"
      ],
      [
        "advance_demands",
        "advance"
      ]
    ],
    "balance": [
      1,
      1
    ],
    "old": "    fn advance(&mut self, base: RetainedCloneRef<'_, DemoSnapshot>, post: &mut DemoSnapshot, mutation: RetainedCloneRef<'_, DemoMutation>, grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError> {\n",
    "new": "    fn advance_demands(&self, _: RetainedCloneRef<'_, DemoSnapshot>, _: &DemoSnapshot, _: RetainedCloneRef<'_, DemoMutation>, _: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Ok(semio_framework_value::RetirementDemand { copy_bytes: if self.phase >= 2 { std::mem::size_of::<DemoSnapshot>() } else { 0 }, capacity_bytes: if self.phase == 1 { std::mem::size_of::<DemoMutation>() } else { 0 }, depth: 1, ..Default::default() }) }\n    fn advance(&mut self, base: RetainedCloneRef"
  },
  {
    "index": 812,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌈️bin-03/🔬️unit/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌈️bin-03/🔬️unit/🧳️retained-clone-preparation-fixtures/🦀️.rs"
    ],
    "names": [
      [
        "advance"
      ],
      [
        "advance_demands",
        "advance"
      ]
    ],
    "balance": [
      1,
      1
    ],
    "old": "    fn advance(&mut self, _base: RetainedCloneRef<'_, RetainedTextSnapshot>, _post: &mut RetainedTextSnapshot, _mutation: RetainedCloneRef<'_, RetainedTextMutation>, _grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError> {\n",
    "new": "    fn advance_demands(&self, _: RetainedCloneRef<'_, RetainedTextSnapshot>, _: &RetainedTextSnapshot, _: RetainedCloneRef<'_, RetainedTextMutation>, _: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, \"retained text edit must not quote after contiguous-capacity refusal\")) }\n    fn advance(&mut self, _base: RetainedCloneRef<'_, RetainedTextSnapshot>, _post: &mut RetainedTextSnapshot, _mutat"
  },
  {
    "index": 852,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌌️bin-04/🔬️unit/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌌️bin-04/🔬️unit/🛤️history-lane-interaction-store-and-immutable-root-laws/🦀️.rs"
    ],
    "names": [
      [
        "interaction_store_apply_updates_hover_and_bumps_generation",
        "interaction_store_reset_discards_hover_and_bumps_generation",
        "interaction_store_default_seeds_from_hover_default"
      ],
      [
        "transient_store_default_seeds_local_state_and_generation"
      ]
    ],
    "balance": [
      1,
      1
    ],
    "old": "async fn interaction_store_apply_updates_hover_and_bumps_generation() {\n    let mut store = InteractionStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });\n    assert_eq!(store.generation().await, 0);\n    assert_eq!(store.hover().await.n, Some(0));\n\n    store.apply(&[DemoMutation::SetN(SetN { n: 7 })]).await.expect(\"valid interaction mutation\");\n    assert_eq!(store.hover().await.n, Some(7), \"apply routes through Mutation::diff/Diff::apply like PresenceStore\");\n    assert_eq!(s",
    "new": "async fn transient_store_default_seeds_local_state_and_generation() {\n    let store = TransientStore::<DemoSnapshot, DemoMutation>::default();\n    assert_eq!(store.current().await.n, Some(0));\n"
  },
  {
    "index": 884,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🌲️trunk-history-shape-and-command-decode-laws/🦀️.rs"
    ],
    "names": [
      [
        "diff"
      ],
      [
        "retire_cold",
        "diff"
      ]
    ],
    "balance": [
      1,
      1
    ],
    "old": "    fn diff(&self, base: &DemoSnapshot) -> semio_framework_os_spr::MutationOutcome<Self::Diff> {\n",
    "new": "\n\n\n\n    fn retire_cold(mut self) {\n        self.1 = false;\n        WITNESS_RETIRED.with(|retired| retired.set(retired.get() + 1));\n    }\n}\n\nimpl semio_framework_os_spr::ColdMutation<DemoSnapshot> for WitnessOp {\n    type ReferenceDiff = <DemoMutation as Mutation<DemoSnapshot>>::Diff;\n    fn diff(&self, base: &DemoSnapshot) -> semio_framework_os_spr::MutationOutcome<Self::ReferenceDiff> {\n"
  },
  {
    "index": 911,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🎯️inferred-field-and-config/🦀️.rs"
    ],
    "names": [
      [
        "compute_step"
      ],
      [
        "OriginalInferredField",
        "validate_source",
        "source_identity",
        "plan_source_extent",
        "plan_source_node",
        "plan_source_key",
        "plan_source_link",
        "clone_original_parent"
      ]
    ],
    "balance": [
      1,
      1
    ],
    "old": "    /// 🪜 Resumable, fallible variant of [`compute`](Self::compute): consumes at most `fuel` units and either finishes or parks its progress in `pending`\n    /// for the next call. The default finishes in one slice at the cost of one unit, so `compute` stays the single semantic source of every field that does not override this.\n    fn compute_step(_snapshot:&P,_key:&Self::Key,_parents:&[Self::Value],_pending:&mut Option<Box<dyn InferencePending>>,_cx:&mut semio_framework_job::StepContext<'_>)->R",
    "new": "\n}\n/// 🎟️ Native field work borrows its defining retained source through every original driver phase.\npub trait OriginalInferredField<P>:InferredField<P>{\n type Source:?Sized;\n fn validate_source(snapshot:&P,source:&Self::Source,cx:&semio_framework_job::StepContext<'_>)->Result<(),InferenceWorkFailure>;\n fn source_identity(snapshot:&P,_source:&Self::Source)->usize{snapshot as*const P as usize}\n fn plan_source_extent(_snapshot:&P,_source:&Self::Source)->Option<(usize,usize)>{None}\n fn plan_source"
  },
  {
    "index": 912,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🎯️inferred-field-and-config/🦀️.rs"
    ],
    "names": [
      [
        "compute_cyclic_step"
      ],
      [
        "compute_step",
        "compute_cyclic_step"
      ]
    ],
    "balance": [
      0,
      0
    ],
    "old": "    /// 🔄️ Produces the field's real cycle result for a row in the admitted plan's cyclic tail.\n    fn compute_cyclic_step(_snapshot:&P,_key:&Self::Key,_pending:&mut Option<Box<dyn InferencePending>>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<ComputeStep<Self::Value>,InferenceWorkFailure>{\n        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,\"original inferred field has no funded cyclic result producer\").into())\n    }\n",
    "new": " fn compute_step(_snapshot:&P,_source:&Self::Source,_key:&Self::Key,_parents:&[Self::Value],_pending:&mut Option<Box<dyn InferencePending>>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<ComputeStep<Self::Value>,InferenceWorkFailure>;\n fn compute_cyclic_step(_snapshot:&P,_source:&Self::Source,_key:&Self::Key,_pending:&mut Option<Box<dyn InferencePending>>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<ComputeStep<Self::Value>,InferenceWorkFailure>;\n"
  },
  {
    "index": 937,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🏁️driver/🦀️.rs"
    ],
    "names": [
      [
        "step_field_update"
      ],
      [
        "update_field_cold"
      ]
    ],
    "balance": [
      1,
      1
    ],
    "old": "pub fn step_field_update<P, F: InferredField<P>>(snapshot: &P, cache: &mut InferenceCache, update:&mut FieldUpdate<F::Key,F::Value>,cx:&mut semio_framework_job::StepContext<'_>)->Result<InferenceStepReport,InferenceWorkFailure> {\n",
    "new": "pub fn update_field_cold<P, F: InferredField<P>>(snapshot: &P, cache: &mut InferenceCache, update:&mut FieldUpdate<F::Key,F::Value>)->Result<InferenceStepReport,InferenceWorkFailure> {\n"
  },
  {
    "index": 1061,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/📁️fs/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/📁️fs/💽️fs-storage/🛤️paths-and-lease-files/🦀️.rs"
    ],
    "names": [
      [
        "write_lease_file"
      ],
      [
        "FsLeaseTransportPolicy",
        "checkpoint",
        "progress",
        "write_lease_file"
      ]
    ],
    "balance": [
      1,
      1
    ],
    "old": "    fn write_lease_file(path: &Path, fence: EpochFence, expires_at_ms: u64, holder: &str) -> Result<(), DbError> {\n",
    "new": "    struct FsLeaseTransportPolicy { handle: super::DbIoTaskHandle }\n\n    impl pack::PackTransportPolicy for FsLeaseTransportPolicy {\n        fn checkpoint(&self, _: pack::PackTransportProgress) -> Result<(), semio_framework_value::ValueError> {\n            if super::db_io_cancel_requested(self.handle) { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled, \"filesystem lease operation canceled\")); }\n            Ok(())\n        }\n        fn progres"
  },
  {
    "index": 1154,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🗝️hash-projection-and-recovery-fixtures/🦀️.rs"
    ],
    "names": [
      [
        "apply"
      ],
      [
        "inverse"
      ]
    ],
    "balance": [
      2,
      2
    ],
    "old": "impl protocol::MutationDiff<HashProjection> for HashDiff {\n    fn apply(&self, base: &HashProjection, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<HashProjection> {\n        Ok(match self.hash {\n            Some(hash) => HashProjection { latest_hash: hash },\n            None => base.clone(),\n        })\n",
    "new": "impl protocol::ColdDiffAlgebra<HashProjection> for HashDiff {\n    fn inverse(&self, base: &HashProjection) -> Self {\n        Self { hash: self.hash.map(|_| base.latest_hash) }\n"
  },
  {
    "index": 1170,
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧬️contract/🦀️.rs",
    "targets": [
      "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧬️contract/🗺️pathmap-diff-and-bridge/🦀️.rs"
    ],
    "names": [
      [
        "envelope_from_operation"
      ],
      [
        "envelope_from_operation_cold"
      ]
    ],
    "balance": [
      0,
      0
    ],
    "old": "pub async fn envelope_from_operation<P, Op>(\n",
    "new": "pub async fn envelope_from_operation_cold<P, Op>(\n"
  }
]
## Corrections

Rejected concrete Store implementations aligned against current macro delegation and NoMembers impossible-variant methods. Restored both current source owners, carrying the two independent factory-bound changes while retaining the current qualified schema owner. Concrete Store undo/redo exists at 📜️space-history/🏠️space/📥️lifted/🏪️store; event-log/publication implementations exist at ⚙️engine/🧱️store-components/📥️lifted/🏪️store/📒️store-log-operations and ⚙️engine/📥️lifted/🏪️store/↩️supersede-undo-redo-and-backbone-pump. Restored all three current InteractionStore tests alongside the incoming TransientStore default test and funding edits. Other flagged declaration changes add demand methods or authored cold/source-bound operations inside the same defining owner.
