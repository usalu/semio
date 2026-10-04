#!/usr/bin/env python3
"""🎯️ S4-TEXT (session 4, AUDIT-TOOLS F9): the raw per-plugin tool-flow codes in writer, vcs and jack become the framework's
localized `app.command.tool-mismatch`; jack's bounded capacity refusal is split into its own named `trinity.jack.retained-capacity`
(localized in `jack_fault_notices`) and vcs' raw `vcs-command-payload-too-large` becomes the named `vcs.command.payload-too-large`
(localized in a new vcs `fault_notices`). Every replacement asserts its exact count; staged then written (`--check` = dry run)."""
import sys
from pathlib import Path

P = Path("✏️s/🔌️plugins")
EDITOR = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
WRITER = P / "✒️writer/🗿️artifacts/✒️writer" / EDITOR
VCS = P / "🌿️vcs/🗿️artifacts/🌿️vcs" / EDITOR
JACK = P / "🔱️trinity/🗿️artifacts/🔌️jack" / EDITOR
JACK_CONTENT = P / "🔱️trinity/🗿️artifacts/🔌️jack/🪆️content/🦀️.rs"
FP = "semio_framework_plugin"


def mismatch(app):
    return f'Fault::new({FP}::FaultOrigin::App, {FP}::FaultCode::new("app.command.tool-mismatch"), "{app} command does not match its exact registered tool")'


JACK_CAPACITY = 'Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("trinity.jack.retained-capacity"), "the jack command exceeds the capacity of one bounded edit")'
VCS_TOO_LARGE = f'Fault::new({FP}::FaultOrigin::App, {FP}::FaultCode::new("vcs.command.payload-too-large"), "the vcs command payload exceeds its bounded capacity")'
EDITS = {
    WRITER: [('return Err(Fault::from("writer-command-tool-mismatch"));', f"return Err({mismatch('Writer')});", 1)],
    VCS: [
        ('return Err(Fault::from("vcs-command-tool-mismatch"));', f"return Err({mismatch('VCS')});", 1),
        ('return Err(Fault::from("vcs-command-payload-too-large"));', f"return Err({VCS_TOO_LARGE});", 4),
        ("""    type Command = VcsCommand;

    const DIALECT: Dialect = crate::VCS_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = VCS_DOCUMENT_SCHEMA;
""", """    type Command = VcsCommand;

    const DIALECT: Dialect = crate::VCS_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = VCS_DOCUMENT_SCHEMA;

    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 1]> = std::sync::LazyLock::new(|| [("vcs.command.payload-too-large", LocalizedLabel::native("This edit is too large to apply in one step.", "Diese Änderung ist zu groß, um sie in einem Schritt anzuwenden."))]);
        NOTICES.as_slice()
    }
""", 1),
    ],
    JACK: [
        ('return Err(Fault::from("jack-retained-transient-tool-mismatch"));', f"return Err({mismatch('Jack')});", 1),
        ("""            if tool_id != request.tool_id || jack_retained_document_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
                return Err(Fault::from("jack-retained-document-tool-mismatch-or-capacity"));
            }""", f"""            if tool_id != request.tool_id {{
                return Err({mismatch('Jack')});
            }}
            if jack_retained_document_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {{
                return Err({JACK_CAPACITY});
            }}""", 1),
        ("""            if tool_id != request.tool_id || jack_retained_window_config_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
                return Err(Fault::from("jack-retained-config-tool-mismatch-or-capacity"));
            }""", f"""            if tool_id != request.tool_id {{
                return Err({mismatch('Jack')});
            }}
            if jack_retained_window_config_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {{
                return Err({JACK_CAPACITY});
            }}""", 1),
    ],
    JACK_CONTENT: [
        ("[(&str, semio_framework_ui_locale::LocalizedLabel); 4]", "[(&str, semio_framework_ui_locale::LocalizedLabel); 5]", 1),
        ("""            ("trinity.jack.layout-run.start", semio_framework_ui_locale::LocalizedLabel::native("The reorganize run could not start.", "Der Neuanordnungslauf konnte nicht starten.")),
""", """            ("trinity.jack.layout-run.start", semio_framework_ui_locale::LocalizedLabel::native("The reorganize run could not start.", "Der Neuanordnungslauf konnte nicht starten.")),
            ("trinity.jack.retained-capacity", semio_framework_ui_locale::LocalizedLabel::native("This edit is too large to apply in one step.", "Diese Änderung ist zu groß, um sie in einem Schritt anzuwenden.")),
""", 1),
    ],
}
staged = {}
for path, pairs in EDITS.items():
    text = path.read_text()
    for old, new, count in pairs:
        if text.count(old) != count:
            sys.exit(f"{path}: expected {count} of {old[:90]!r}, found {text.count(old)}")
        text = text.replace(old, new)
    staged[path] = text
if "--check" not in sys.argv:
    for path, text in staged.items():
        path.write_text(text)
print(f"F9 codes: {len(staged)} files {'checked' if '--check' in sys.argv else 'written'}")
