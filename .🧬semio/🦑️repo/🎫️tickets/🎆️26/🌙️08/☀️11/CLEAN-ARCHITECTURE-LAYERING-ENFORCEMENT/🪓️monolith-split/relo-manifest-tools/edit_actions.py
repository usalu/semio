"""🕹️ Moves ActionKind and InteractiveJobClassification into the actions child and adds the reserved-contribution registration point."""
import pathlib, sys
p = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🕹️actions/🦀️.rs")
s = p.read_text()

def rep(old, new):
    global s
    assert s.count(old) == 1, (s.count(old), old[:80])
    s = s.replace(old, new)

action_kind = '''//#region 🔖️ActionKind
/// 🗂️ Classifies a declared action by how it interacts with VCS history.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ActionKind {
    /// Mutates the document — dispatched as VCS mutations with a true inverse, recorded in history.
    Mutation,
    /// Ephemeral window or session state — recorded in the session command log, never as a VCS edit.
    View,
    /// Moves the applied stack, the head of the artifact or a reserved history session — injected by the framework or
    /// a [`ReservedContribution`], never app-declared.
    History,
    /// Copies, cuts or pastes through the shell clipboard — framework-injected, never app-declared.
    Clipboard,
    /// Shell-only effect without a document mutation — recorded in the session command log via dispatch or the
    /// `noteShellCommand` mechanism.
    Shell,
    /// Raw pointer interaction of a declared interaction domain — framework-injected, never app-declared.
    Interaction,
}

semio_framework_value::artifact_retire_leaf!(ActionKind);
//#endregion 🔖️ActionKind

'''
rep("// #region 🛂️Manifest\n//#region 🔖️Manifest\n//#region 🔖️ActionSemantics\n", "// #region 🛂️Manifest\n//#region 🔖️Manifest\n" + action_kind + "//#region 🔖️ActionSemantics\n")

classification = '''/// 🧵️ How a declared action or command may be dispatched as an interactive job. The default is deliberately not
/// executable: a declaration decoded without an explicit disposition stays visible to audit tooling but is rejected by
/// [`validate_interactive_job_classification`] before a release catalog can be activated.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum InteractiveJobClassification {
    #[default]
    Unclassified,
    Migrated,
    BatchOnlyPendingRewrite,
    ForbiddenFromUi,
    Deleted,
}

'''
rep("/// ⚙️ Preview/undo/idempotency/cancellation shape of one capability invocation.\n", classification + "/// ⚙️ Preview/undo/idempotency/cancellation shape of one capability invocation.\n")

rep('''/// 🕹️ The framework-owned History actions, auto-injected into every `AppDefinition`: the seven history-lane verbs
/// followed by the [`history_edit_action_definitions`] of non-destructive history editing.
pub fn history_action_definitions() -> Vec<ActionDefinition> {
    let mut actions = history_lane_action_definitions();
    actions.extend(history_edit_action_definitions());
    actions
}

''', "")

rep('''/// 🕹️ The seven history-lane verbs: they move the applied stack or the head of the artifact.
fn history_lane_action_definitions() -> Vec<ActionDefinition> {
''', '''/// 🕹️ The seven framework-owned History verbs, auto-injected into every `AppDefinition`: they move the applied stack or
/// the head of the artifact. A feature that reserves further history verbs contributes them through a
/// [`ReservedContribution`].
pub fn history_action_definitions() -> Vec<ActionDefinition> {
''')

registration = '''
//#region 🧩️ReservedContribution
/// 🧩️ What one feature reserves on the apps that host it: actions joined to the app roster, actions joined to every
/// window kind of a built app, dialogs joined to an app that may edit, and the audience it fixes for its own verbs. A
/// host (the app builder, the capability catalog) passes the contributions it hosts to [`reserved_app_actions`],
/// [`reserved_window_actions`], [`reserved_editor_dialogs`] and [`reserved_audience_violation`], so a feature reaches
/// every app without the manifest naming it.
#[derive(Clone, Copy)]
pub struct ReservedContribution {
    pub id: &'static str,
    pub app_actions: fn() -> Vec<ActionDefinition>,
    pub window_actions: fn(&AppDefinition) -> Vec<ActionDefinition>,
    pub editor_dialogs: fn() -> Vec<DialogDefinition>,
    pub fixed_audience: fn(&str) -> Option<CapabilityAudience>,
}

impl ReservedContribution {
    /// 🌱️ A contribution named `id` that reserves nothing yet.
    pub const fn new(id: &'static str) -> Self {
        Self { id, app_actions: no_reserved_actions, window_actions: no_reserved_window_actions, editor_dialogs: no_reserved_dialogs, fixed_audience: no_fixed_audience }
    }

    /// 🕹️ Reserves `app_actions` on the app roster of every hosting app.
    pub const fn with_app_actions(self, app_actions: fn() -> Vec<ActionDefinition>) -> Self {
        Self { app_actions, ..self }
    }

    /// 🪟️ Reserves `window_actions` on every window kind of a built app.
    pub const fn with_window_actions(self, window_actions: fn(&AppDefinition) -> Vec<ActionDefinition>) -> Self {
        Self { window_actions, ..self }
    }

    /// 🗳️ Reserves `editor_dialogs` on every hosting app that may edit.
    pub const fn with_editor_dialogs(self, editor_dialogs: fn() -> Vec<DialogDefinition>) -> Self {
        Self { editor_dialogs, ..self }
    }

    /// 🧭️ Fixes the audience of this contribution's own verbs.
    pub const fn with_fixed_audience(self, fixed_audience: fn(&str) -> Option<CapabilityAudience>) -> Self {
        Self { fixed_audience, ..self }
    }
}

fn no_reserved_actions() -> Vec<ActionDefinition> {
    Vec::new()
}

fn no_reserved_window_actions(_app: &AppDefinition) -> Vec<ActionDefinition> {
    Vec::new()
}

fn no_reserved_dialogs() -> Vec<DialogDefinition> {
    Vec::new()
}

fn no_fixed_audience(_action_id: &str) -> Option<CapabilityAudience> {
    None
}

/// 🕹️ The app-roster actions of every contribution, in contribution order.
pub fn reserved_app_actions(contributions: &[ReservedContribution]) -> Vec<ActionDefinition> {
    contributions.iter().flat_map(|contribution| (contribution.app_actions)()).collect()
}

/// 🪟️ The window-kind actions every contribution reserves on the built `app`, in contribution order.
pub fn reserved_window_actions(contributions: &[ReservedContribution], app: &AppDefinition) -> Vec<ActionDefinition> {
    contributions.iter().flat_map(|contribution| (contribution.window_actions)(app)).collect()
}

/// 🗳️ The dialogs every contribution reserves on an app that may edit, in contribution order.
pub fn reserved_editor_dialogs(contributions: &[ReservedContribution]) -> Vec<DialogDefinition> {
    contributions.iter().flat_map(|contribution| (contribution.editor_dialogs)()).collect()
}

/// 🧭️ The audience the first contribution that owns `action_id` fixes for it.
pub fn reserved_fixed_audience(contributions: &[ReservedContribution], action_id: &str) -> Option<CapabilityAudience> {
    contributions.iter().find_map(|contribution| (contribution.fixed_audience)(action_id))
}

/// ⚖️ The audience a contribution fixes for `action` when `action` resolves to a different one.
pub fn reserved_audience_violation(contributions: &[ReservedContribution], action: &ActionDefinition) -> Option<CapabilityAudience> {
    reserved_fixed_audience(contributions, &action.id).filter(|expected| *expected != resolve_audience(action))
}

#[cfg(test)]
#[path = "../🧪️tests/🧩️reserved-contributions/🦀️.rs"]
mod reserved_contribution_tests;
//#endregion 🧩️ReservedContribution
'''
rep('''/// 🎚️ The framework-owned action id apps dispatch to change the history panel's mutations
/// filter — auto-injected unconditionally (mirrors `RECORD_TUTORIAL_ACTION_ID`).
''', registration.lstrip("\n") + '''
/// 🎚️ The framework-owned action id apps dispatch to change the history panel's mutations
/// filter — auto-injected unconditionally.
''')
p.write_text(s)
print("ok")
