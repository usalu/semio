//! 📜️ History verbs: checkpoints, interactions, drafts and definitions.

use super::flag_or_arg;
use semio_framework_repo_cli_command_framework::{CliError, Invocation};
use semio_framework_repo_cli_tree_source_and_search::StreamOptions;
use semio_framework_repo_identity::{emoji_text, entity as identity_entity, flat};
use semio_framework_repo_model::{entity, DraftCreateInput};
use serde_json::json;
use std::collections::BTreeSet;

pub fn checkpoint_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let options = StreamOptions::of(invocation);
    let limit = if invocation.int("limit") > 0 { invocation.int("limit") } else { 100 };
    let results = invocation
        .context()
        .checkpoints(Some(limit))
        .map_err(|error| CliError::Message(error.message))?
        .into_iter()
        .filter(|checkpoint| options.matches_filter(&format!("{} {}", checkpoint.sha, checkpoint.title)) && options.matches_query(&format!("{} {}", checkpoint.sha, checkpoint.title)))
        .map(|checkpoint| json!({ "checkpoint": checkpoint }))
        .collect();
    invocation.stream("checkpoint list", results, Vec::new())
}

pub fn interaction_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let mut interactions = invocation.context().interactions().map_err(|error| CliError::Message(error.message))?;
    if invocation.flag("sorted") {
        interactions.sort_by(|left, right| left.interaction.date.cmp(&right.interaction.date));
    }
    let results = interactions.into_iter().map(|interaction| json!({ "interaction": interaction })).collect();
    invocation.stream("interaction list", results, Vec::new())
}

pub fn interaction_tree(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let context = invocation.context();
    let mut nodes: Vec<(String, String, String)> = Vec::new();
    for goal in context.goals().unwrap_or_default() {
        nodes.push((goal.id.clone(), format!("{} - {} - {}", entity::goal_artifact_id(&goal.id), goal.title, goal.status.as_str()), goal.parent.clone()));
    }
    for ticket in context.tickets(None, None, None, None).unwrap_or_default() {
        let id = emoji_text(identity_entity("ticket")) + &flat(&ticket.slug);
        for interaction in &ticket.interactions {
            nodes.push((format!("{id}/{}/{}", interaction.kind, interaction.date), format!("🔄️ {} [{}] {} @{}", interaction.kind, interaction.date, interaction.client, interaction.author), id.clone()));
        }
        nodes.push((id.clone(), format!("{id} - {} - {}", ticket.title, ticket.status.as_str()), ticket.goal.clone()));
    }
    let known: BTreeSet<String> = nodes.iter().map(|(id, _, _)| id.clone()).collect();
    fn render(nodes: &[(String, String, String)], known: &BTreeSet<String>, parent: Option<&str>, prefix: &str, lines: &mut Vec<String>) {
        let mut level: Vec<&(String, String, String)> = nodes
            .iter()
            .filter(|(id, _, owner)| match parent {
                None => owner.is_empty() || owner == id || !known.contains(owner),
                Some(parent) => owner == parent && owner != id,
            })
            .collect();
        level.sort_by(|left, right| left.0.cmp(&right.0));
        let last = level.len().saturating_sub(1);
        for (index, (id, label, _)) in level.into_iter().enumerate() {
            let (connector, child_prefix) = if index == last { ("└️─️─️ ", format!("{prefix}    ")) } else { ("├️─️─️ ", format!("{prefix}│️   ")) };
            lines.push(format!("{prefix}{connector}{label}"));
            render(nodes, known, Some(id), &child_prefix, lines);
        }
    }
    let mut lines = Vec::new();
    render(&nodes, &known, None, "", &mut lines);
    invocation.stream("interaction tree", Vec::new(), lines)
}

pub fn draft_create(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    if invocation.args().is_empty() {
        return Err("missing title".into());
    }
    let draft = invocation.context().draft_create(DraftCreateInput { title: invocation.arg(0), files: invocation.texts("files") }).map_err(|error| CliError::Message(error.message))?;
    invocation.stream("draft create", vec![json!({ "draft": draft })], Vec::new())
}

pub fn draft_delete(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    if invocation.args().is_empty() {
        return Err("missing slug".into());
    }
    invocation.context().draft_delete(&invocation.arg(0)).map_err(|error| CliError::Message(error.message))?;
    invocation.stream("draft delete", Vec::new(), Vec::new())
}

pub fn draft_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    invocation.graphql("query Drafts { drafts { id uri } }", json!({}))
}

pub fn definition_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let path = flag_or_arg(invocation, "file", 0);
    if path.is_empty() {
        return Err("missing file".into());
    }
    let options = StreamOptions::of(invocation);
    let content = std::fs::read_to_string(invocation.root().join(&path)).map_err(|error| CliError::Message(format!("{path}: {error}")))?;
    let results = semio_framework_repo_codebase::file_definitions(&content, &path)
        .into_iter()
        .filter(|definition| options.includes_definition_kind(definition.kind.as_str()) && options.matches_filter(&definition.name) && options.matches_query(&definition.name))
        .map(|definition| json!({ "definition": definition }))
        .collect();
    invocation.stream("definition list", results, Vec::new())
}
