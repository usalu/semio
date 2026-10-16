"""🏗️ Points the plugin app builder at the reserved-contribution registration point and the onboarding module."""
import pathlib, re
p = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧩️app/🦀️.rs")
s = p.read_text()

def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (s.count(old), old[:90])
    s = s.replace(old, new)

line_start = s.index("use semio_framework::{clipboard_action_definitions, element_id_segment, history_action_definitions,")
line_end = s.index("\n", line_start)
line = s[line_start:line_end]
onboarding = ["record_tutorial_action_definition", "start_introduction_action_definition", "start_tutorial_action_definition", "IntroductionDefinition", "IntroductionInteractionKind", "TutorialDefinition", "RECORD_TUTORIAL_ACTION_ID", "START_INTRODUCTION_ACTION_ID", "START_TUTORIAL_ACTION_ID"]
for name in onboarding:
    assert line.count(f" {name},") == 1, name
    line = line.replace(f" {name},", "")
line += "\nuse semio_framework::onboarding::{record_tutorial_action_definition, start_introduction_action_definition, start_tutorial_action_definition, IntroductionDefinition, IntroductionInteractionKind, TutorialDefinition, RECORD_TUTORIAL_ACTION_ID, START_INTRODUCTION_ACTION_ID, START_TUTORIAL_ACTION_ID};"
s = s[:line_start] + line + s[line_end:]

rep("use crate::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY;\n", '''use crate::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY;

#[path = "🖐️gumball-audience/🦀️.rs"]
mod gumball_audience;

/// 🧩️ The reserved contributions every built app hosts: the framework's features, then the gumball audience rule.
fn builder_reserved_contributions() -> Vec<semio_framework::ReservedContribution> {
    semio_framework::FRAMEWORK_RESERVED_CONTRIBUTIONS.iter().copied().chain([gumball_audience::RESERVED]).collect()
}
''')
rep("fn introduction_gesture_points(gesture: &semio_framework::IntroductionGesture) -> Vec<&semio_framework::IntroductionPoint> {\n    use semio_framework::IntroductionGesture;",
    "fn introduction_gesture_points(gesture: &semio_framework::onboarding::IntroductionGesture) -> Vec<&semio_framework::onboarding::IntroductionPoint> {\n    use semio_framework::onboarding::IntroductionGesture;")
rep('''        for action in self.actions.iter().chain(self.window_kinds.iter().flat_map(|window| window.actions.iter())) {
            if let Some(expected) = semio_framework::framework_fixed_audience_violation(action) {
                return Err(PluginAssemblyError::new(
                    "app-definition.invalid",
                    format!("app {} declares the framework gumball verb {} with audience {:?}; the one rule for every gumball user is {:?}", self.id, action.id, semio_framework::resolve_audience(action), expected),''',
'''        let reserved = builder_reserved_contributions();
        for action in self.actions.iter().chain(self.window_kinds.iter().flat_map(|window| window.actions.iter())) {
            if let Some(expected) = semio_framework::reserved_audience_violation(&reserved, action) {
                return Err(PluginAssemblyError::new(
                    "app-definition.invalid",
                    format!("app {} declares the reserved verb {} with audience {:?}; its reserved contribution fixes {:?}", self.id, action.id, semio_framework::resolve_audience(action), expected),''')
rep("for framework_action in history_action_definitions().into_iter().chain(clipboard_action_definitions()).filter(",
    "for framework_action in history_action_definitions().into_iter().chain(semio_framework::reserved_app_actions(&reserved)).chain(clipboard_action_definitions()).filter(")
for old in ["semio_framework::validate_tutorial(", "semio_framework::TutorialUiChange", "semio_framework::TutorialUiSample", "semio_framework::TutorialEventKind", "semio_framework::IntroductionPoint::Element"]:
    n = s.count(old)
    assert n >= 1, old
    s = s.replace(old, old.replace("semio_framework::", "semio_framework::onboarding::"))
rep('''        if role != AppRole::Viewer && !self.dialogs.iter().any(|dialog| dialog.id == semio_framework::HISTORY_EDIT_FINALIZE_DIALOG_ID) {
            self.dialogs.push(semio_framework::history_edit_finalize_dialog());
        }''', '''        if role != AppRole::Viewer {
            for dialog in semio_framework::reserved_editor_dialogs(&reserved) {
                if !self.dialogs.iter().any(|declared| declared.id == dialog.id) {
                    self.dialogs.push(dialog);
                }
            }
        }''')
rep(".chain(semio_framework::tool_run_action_definitions(&definition))", ".chain(semio_framework::reserved_window_actions(&reserved, &definition))")
p.write_text(s)
left = re.findall(r"semio_framework::(?:Introduction|Tutorial|HISTORY_EDIT|history_edit|tool_run_action|framework_fixed|validate_tutorial)\w*", s)
print("leftover:", left)
