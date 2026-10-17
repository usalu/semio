//! 🧪️ schema tests (🔬️board-host): compiled against the `testkit` surface of the board crates.



/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.
fn read_static(path_from_manifest: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path_from_manifest);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("test input {}: {error}", path.display())).into_boxed_str())
}

#[cfg(test)]
#[allow(non_snake_case)]
fn BOARD_HOST_SOURCE() -> &'static str {
    read_static("../../../🔌️ports/➡️directed/➕️normal/🪣️fill/🦀️.rs")
}

#[cfg(test)]
    fn fill_full_commit_candidate_contract(source: &str) -> bool {
        let Some(start) = source.find("pub struct BoardFillResult") else { return false };
        let Some(end) = source[start..].find("/// 🧵️ Persistent worker-owned fill search") else { return false };
        let codec = &source[start..start + end];
        let Some(job_start) = source.find("fn complete(&mut self, context: &mut semio_framework_job::StepContext") else { return false };
        let Some(job_end) = source[job_start..].find("fn fault_outcome") else { return false };
        let job = &source[job_start..job_start + job_end];
        codec.contains("pub struct BoardFillCommitPlacement")
            && codec.contains("pub edge_kind: BoardFillText")
            && codec.contains("pub handles: [Option<BoardFillCommitHandle>; BOARD_FILL_KIND_HANDLE_CAPACITY]")
            && codec.contains("pub placement: Option<BoardFillCommitPlacement>")
            && codec.contains("struct BoardFillCommitPayload")
            && codec.contains("BoardFillCommitPayload::new(&candidate.output)")
            && codec.contains("payload.page_count() == BOARD_FILL_COMMIT_PAGE_COUNT")
            && codec.contains("payload.page(page_index)")
            && codec.contains("text_byte: usize")
            && codec.contains("fn step_text")
            && codec.contains("self.bytes[offset + 2 + self.text_byte] = text.bytes[self.text_byte]")
            && !codec.contains("String")
            && !codec.contains("Vec<")
            && !codec.contains("BTreeMap")
            && !codec.contains("single_page()")
            && !codec.contains("const BOARD_FILL_COMMIT_BYTES: usize = 13")
            && !codec.contains("result.encode")
            && job.contains("start.saturating_add(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)")
            && job.contains("if end < BOARD_FILL_COMMIT_BYTES")
            && job.contains("encoder.output_cursor = end")
    }

#[cfg(test)]
    #[test]
    fn fill_commit_candidate_granularity_mutations_are_rejected() {
        let source = BOARD_HOST_SOURCE();
        assert!(fill_full_commit_candidate_contract(source));
        let text = source.replacen("self.bytes[offset + 2 + self.text_byte] = text.bytes[self.text_byte];", "self.bytes[offset + 2..offset + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES].copy_from_slice(&text.bytes);", 1);
        assert!(!fill_full_commit_candidate_contract(&text));
    }

#[cfg(test)]
    fn fill_ingress_granularity_contract(source: &str) -> bool {
        let Some(start) = source.find("impl BoardFillSnapshotIngress {") else { return false };
        let Some(end) = source[start..].find("impl Drop for BoardFillSnapshotIngress") else { return false };
        let ingress = &source[start..start + end];
        [
            "pub fn begin_node(&mut self)",
            "pub fn push_node_id_byte(&mut self, byte: u8)",
            "pub fn set_node_bound(&mut self, index: usize, value: f64)",
            "pub fn begin_handle(&mut self)",
            "pub fn push_handle_text_byte(&mut self, field: BoardFillIngressHandleText, byte: u8)",
            "pub fn begin_kind(&mut self)",
            "pub fn push_kind_text_byte(&mut self, field: BoardFillIngressKindText, byte: u8)",
            "pub fn push_kind_handle_text_byte(&mut self, field: BoardFillIngressTemplateText, byte: u8)",
            "pub fn begin_rule(&mut self)",
            "pub fn push_rule_text_byte(&mut self, field: BoardFillIngressRuleText, byte: u8)",
        ]
        .iter()
        .all(|marker| ingress.contains(marker))
            && ingress.matches("try_push_byte(byte)").count() == 13
            && !ingress.contains("pub fn push_node(")
            && !ingress.contains("pub fn push_handle(")
            && !ingress.contains("pub fn push_kind(")
            && !ingress.contains("pub fn push_rule(")
            && !ingress.contains("for byte")
    }

#[cfg(test)]
    #[test]
    fn fill_ingress_granularity_mutations_are_rejected() {
        let source = BOARD_HOST_SOURCE();
        assert!(fill_ingress_granularity_contract(source));
        let fields = source.replacen("pub fn set_node_bound(&mut self, index: usize, value: f64)", "pub fn push_node(&mut self, bounds: [f64; 4])", 1);
        assert!(!fill_ingress_granularity_contract(&fields));
        let text = source.replacen("pub fn push_node_id_byte(&mut self, byte: u8)", "pub fn push_node_id(&mut self, bytes: &[u8])", 1);
        assert!(!fill_ingress_granularity_contract(&text));
    }

#[cfg(test)]
    fn fill_owned_page_handback_contract(source: &str) -> bool {
        let Some(start) = source.find("impl BoardFillSnapshotIngress {") else { return false };
        let Some(end) = source[start..].find("impl Drop for BoardFillJob") else { return false };
        let retained = &source[start..start + end];
        let mut insertions = 0usize;
        for line in retained.lines().filter(|line| line.contains("try_push_owned")) {
            insertions += 1;
            if !line.contains("if let Err(") {
                return false;
            }
        }
        insertions == 11 && !retained.contains("try_push_owned(candidate).is_err()")
    }

#[cfg(test)]
    #[test]
    fn fill_owned_page_handback_mutation_is_rejected() {
        let source = BOARD_HOST_SOURCE();
        assert!(fill_owned_page_handback_contract(source));
        let marker = "if let Err(candidate) = state.candidates.try_push_owned(candidate) {";
        let production_end = source.find("impl Drop for BoardFillJob").expect("live fill job drop");
        let index = source[..production_end].rfind(marker).expect("live scan-compatibility handback");
        let mut mutant = String::with_capacity(source.len());
        mutant.push_str(&source[..index]);
        mutant.push_str("if state.candidates.try_push_owned(candidate).is_err() {");
        mutant.push_str(&source[index + marker.len()..]);
        assert!(!fill_owned_page_handback_contract(&mutant));
    }
