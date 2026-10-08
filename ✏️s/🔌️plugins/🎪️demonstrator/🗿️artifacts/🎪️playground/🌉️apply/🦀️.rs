//! 🌉️ Central apply entry points: the only code of this artifact that turns a mutation's diff into the next snapshot with `protocol::apply_diff`; the schema and mutation leaves only build diffs and inverses.
#![allow(unused_imports)]

use crate::standards::v1::subsets::any::schema::{diff::PlaygroundDiff, mutations::PlaygroundMutation, snapshot::PlaygroundSnapshot};

pub(crate) fn bridge_step(snapshot: &PlaygroundSnapshot, mutation: &PlaygroundMutation) -> Result<(PlaygroundSnapshot, Vec<String>), String> {
    use protocol::Mutation;
    let outcome = <PlaygroundMutation as Mutation<PlaygroundSnapshot>>::diff(mutation, snapshot);
    let messages = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
    protocol::apply_diff(outcome.diff(), snapshot).map(|next| (next, messages)).map_err(|error| format!("{error:?}"))
}
