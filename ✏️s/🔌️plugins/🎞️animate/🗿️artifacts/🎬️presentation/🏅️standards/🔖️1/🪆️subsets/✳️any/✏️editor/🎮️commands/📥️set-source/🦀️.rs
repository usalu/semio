//! 🖼️ 🖼️ Animate presentation app commands command — `set-source`.

#![allow(clippy::result_large_err)]

use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{interaction_select_effect, tile_roster_mutations, PresentationDispatchCtx};
use crate::mutations::replace_source::ReplaceSource;
use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
use crate::{FigureTileSource, PresentationSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "set-source")]
pub struct SetSource {
    #[dsl(block)]
    pub source: FigureTileSource,
}

pub fn handle(payload: &SetSource, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let (deck_source, _) = crate::presentation_working_scene(deck);
    let replaced = payload.source.src != deck_source.src;
    let mut operations = vec![PresentationMutation::ReplaceSource(ReplaceSource { new_source: payload.source.clone() })];
    let mut emit_effects = Vec::new();
    if replaced {
        operations.extend(tile_roster_mutations(deck, Vec::new()));
        emit_effects.push(interaction_select_effect(&[], "replace"));
    }
    Ok(Emit { artifact_mutations: operations, effects: emit_effects, ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
