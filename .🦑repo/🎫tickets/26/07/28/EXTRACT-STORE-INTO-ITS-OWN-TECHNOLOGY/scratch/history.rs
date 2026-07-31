//#region 🔖History
/// @emoji 📜 One row of a checkpoint history/ancestor graph.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryColumn {
    pub checkpoint_id: String,
    pub timestamp: String,
    pub labels: Vec<String>,
    pub authors: Vec<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_checkpoint_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub lane: usize,
    pub alternative_ids: Vec<String>,
}

fn checkpoint_alternatives<'a, P, Operation>(
    envelope: &'a DocumentEnvelope<P, Operation>,
    checkpoint_id: &str,
) -> Vec<&'a Alternative> {
    envelope
        .vcs
        .alternatives
        .iter()
        .filter(|alternative| alternative.checkpoint_ids.iter().any(|id| id == checkpoint_id))
        .collect()
}

fn is_checkpoint_main_only<P, Operation>(envelope: &DocumentEnvelope<P, Operation>, checkpoint_id: &str) -> bool {
    checkpoint_alternatives(envelope, checkpoint_id).is_empty()
}

fn has_main_only_descendant<P, Operation>(
    envelope: &DocumentEnvelope<P, Operation>,
    children_of: &HashMap<String, Vec<String>>,
    checkpoint_id: &str,
    seen: &mut HashSet<String>,
) -> bool {
    if !seen.insert(checkpoint_id.to_string()) {
        return false;
    }
    for child_id in children_of.get(checkpoint_id).into_iter().flatten() {
        if is_checkpoint_main_only(envelope, child_id) || has_main_only_descendant(envelope, children_of, child_id, seen) {
            return true;
        }
    }
    false
}

/// @emoji 🛤️ Assigns each checkpoint a swimlane: alternatives get lanes `1..n` in array order, lane
/// `0` is the main trunk. A checkpoint sits on lane 0 if it belongs to no alternative or has any
/// main-only descendant (cycle-guarded DFS); otherwise it takes its single alternative's lane, or
/// the minimum lane among several. Mirrors premigration `assignHistoryCheckpointLanes`.
fn assign_history_checkpoint_lanes<P, Operation>(envelope: &DocumentEnvelope<P, Operation>) -> HashMap<String, usize> {
    let mut lane_by_alternative: HashMap<String, usize> = HashMap::new();
    for (index, alternative) in envelope.vcs.alternatives.iter().enumerate() {
        lane_by_alternative.insert(alternative.id.clone(), index + 1);
    }
    let mut children_of: HashMap<String, Vec<String>> = HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if let Some(parent_id) = &checkpoint.parent_id {
            children_of.entry(parent_id.clone()).or_default().push(checkpoint.id.clone());
        }
    }
    let mut lane_by_checkpoint_id: HashMap<String, usize> = HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if checkpoint.parent_id.is_none() {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let mut seen = HashSet::new();
        if is_checkpoint_main_only(envelope, &checkpoint.id)
            || has_main_only_descendant(envelope, &children_of, &checkpoint.id, &mut seen)
        {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let alternatives = checkpoint_alternatives(envelope, &checkpoint.id);
        let lanes: Vec<usize> = alternatives
            .iter()
            .map(|alternative| *lane_by_alternative.get(&alternative.id).unwrap_or(&0))
            .collect();
        let lane = if lanes.len() == 1 {
            lanes[0]
        } else {
            lanes.into_iter().min().unwrap_or(0)
        };
        lane_by_checkpoint_id.insert(checkpoint.id.clone(), lane);
    }
    lane_by_checkpoint_id
}

/// @emoji 📜 Builds the ancestor-graph rows for a checkpoint history view: newest checkpoint first,
/// each carrying its swimlane, labels (alternative names, `"main"` fallback on the newest unlabeled
/// row), and authors. Mirrors premigration `buildHistoryColumns`.
pub fn build_history_columns<P, Operation>(envelope: &DocumentEnvelope<P, Operation>) -> Vec<HistoryColumn> {
    let lane_by_checkpoint_id = assign_history_checkpoint_lanes(envelope);
    envelope
        .vcs
        .checkpoints
        .iter()
        .rev()
        .enumerate()
        .map(|(index, checkpoint)| {
            let alternatives = checkpoint_alternatives(envelope, &checkpoint.id);
            let alternative_ids: Vec<String> = alternatives.iter().map(|alternative| alternative.id.clone()).collect();
            let mut labels: Vec<String> = alternatives.iter().map(|alternative| alternative.name.clone()).collect();
            if labels.is_empty() && index == 0 {
                labels.push("main".into());
            }
            HistoryColumn {
                checkpoint_id: checkpoint.id.clone(),
                timestamp: checkpoint.timestamp.clone(),
                labels,
                authors: checkpoint.authors.clone(),
                parent_checkpoint_id: checkpoint.parent_id.clone(),
                description: checkpoint.message.clone(),
                lane: *lane_by_checkpoint_id.get(&checkpoint.id).unwrap_or(&0),
                alternative_ids,
            }
        })
        .collect()
}
//#endregion 🔖History
