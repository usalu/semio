from pathlib import Path
import sys

folder = Path(sys.argv[1]) / "🔀️interchange/🪆️drawing/🌱️birth"
source = folder / "🦀️.rs"
tests = folder / "🧪️tests/🦀️.rs"
source_before = source.read_text()
tests_before = tests.read_text()

def replace_once(text, old, new):
    if text.count(old) != 1:
        raise ValueError(f"Expected one matching source region: {old}")
    return text.replace(old, new, 1)

changes = [
    ("type Output<S> = (LayoutDrawingMember, LayoutDrawingBirth<S>);", "type Output<S> = (LayoutDrawingMember, LayoutDrawingBirth<S>);\ntype ParentField = RetainedFieldPreparation<LayoutDrawingMember>;"),
    ("enum Action { Source, SourceFailure, Clone,", "enum Action { Source, SourceFailure, BirthField, Clone,"),
    ("    field: RetainedFieldPreparation<LayoutDrawingMember>,", "    field: Option<ParentField>,"),
    ("            field: Default::default(),", "            field: None,"),
    ("self.field.terminal_is_empty() && self.source.terminal_is_empty()", "self.field.as_ref().is_none_or(ParentField::terminal_is_empty) && self.source.terminal_is_empty()"),
    ("            if !self.field.terminal_is_empty() {\n                return Ok(if self.field.close_started() { Action::FieldClose } else { Action::StartFieldClose });\n            }", "            if let Some(action) = self.field_close_action() { return Ok(action); }"),
    ("        if self.field.value().is_some() { return Ok(Action::TakeField); }\n        if self.source.demand(OriginalSourceMode::Original, body)?.is_some() { return Ok(Action::Source); }\n        Ok(Action::Clone)", "        if self.field.as_ref().and_then(ParentField::value).is_some() { return Ok(Action::TakeField); }\n        if self.source.demand(OriginalSourceMode::Original, body)?.is_some() { return Ok(Action::Source); }\n        Ok(if self.field.is_some() { Action::Clone } else { Action::BirthField })"),
    ("    fn handback(&self, body: usize)", "    fn field_close_action(&self) -> Option<Action> {\n        self.field.as_ref().filter(|field| !field.terminal_is_empty()).map(|field| {\n            if field.close_started() { Action::FieldClose } else { Action::StartFieldClose }\n        })\n    }\n\n    fn handback(&self, body: usize)"),
    ("            Action::Clone => (self.field.demand(self.source.original()?, body, u64::MAX)?, self.field.advance_work_demand(self.source.original()?, body, u64::MAX)?),", "            Action::BirthField => (fixed(size_of::<ParentField>() + size_of::<Option<ParentField>>()), ParentField::embedded_birth_work()?),\n            Action::Clone => {\n                let field = self.field.as_ref().ok_or_else(fault)?;\n                let source = self.source.original()?;\n                (field.demand(source, body, u64::MAX)?, field.advance_work_demand(source, body, u64::MAX)?)\n            }"),
    ("            Action::TakeField => (self.field.exchange_demand()?, 1),\n            Action::StartFieldClose => (self.field.begin_close_demand(), self.field.begin_close_work_demand()),\n            Action::FieldClose => (self.field.close_demand(body)?, self.field.close_work_demand(body)?),", "            Action::TakeField => (self.field.as_ref().ok_or_else(fault)?.exchange_demand()?, 1),\n            Action::StartFieldClose => {\n                let field = self.field.as_ref().ok_or_else(fault)?;\n                (field.begin_close_demand(), field.begin_close_work_demand())\n            }\n            Action::FieldClose => {\n                let field = self.field.as_ref().ok_or_else(fault)?;\n                (field.close_demand(body)?, field.close_work_demand(body)?)\n            }"),
    ("Action::FieldClose => self.field.requires_original_birth(),", "Action::FieldClose => self.field.as_ref().ok_or_else(fault)?.requires_original_birth(),"),
    ("Action::FieldClose if self.field.requires_original_birth() => self.field.close_original_birth_step(loan, grant),", "Action::FieldClose if self.field.as_ref().is_some_and(ParentField::requires_original_birth) => {\n                self.field.as_mut().ok_or_else(fault)?.close_original_birth_step(loan, grant)\n            }"),
    ("Action::FieldClose => self.field.requires_original_control(),", "Action::FieldClose => self.field.as_ref().ok_or_else(fault)?.requires_original_control(),"),
    ("        match action {\n            Action::SourceFailure => {", "        match action {\n            Action::BirthField => {\n                let (demand, work) = self.quote(action, body)?;\n                if !effect.performed(work)? { return Err(fault()); }\n                self.field = Some(ParentField::default());\n                return Ok(reply.complete(Ok(RetainedCloneStep::Progress(progress(demand)))));\n            }\n            Action::SourceFailure => {"),
    ("self.field.take_into(&mut self.parent, effect, inner)?", "self.field.as_mut().ok_or_else(fault)?.take_into(&mut self.parent, effect, inner)?"),
    ("self.field.begin_close_paid(effect)", "self.field.as_mut().ok_or_else(fault)?.begin_close_paid(effect)"),
    ("        self.field.terminal_is_empty() && self.source.terminal_is_empty()", "        self.field.as_ref().is_none_or(ParentField::terminal_is_empty) && self.source.terminal_is_empty()"),
]

source_after = source_before
for old, new in changes:
    if old.startswith("            if !self.field.terminal_is_empty()"):
        if source_after.count(old) != 2:
            raise ValueError("Drawing Birth field close selection changed")
        source_after = source_after.replace(old, new)
    elif old.startswith("self.field.terminal_is_empty()"):
        source_after = replace_once(source_after, "self.birth.is_some() && " + old, "self.birth.is_some() && " + new)
    else:
        source_after = replace_once(source_after, old, new)
for method in (
    "advance_into", "recover_advance_into", "close_into", "recover_close_into",
    "close_original_into", "recover_close_original_into",
):
    source_after = replace_once(source_after, f"state.field.{method}(", f"state.field.as_mut().ok_or_else(fault)?.{method}(")

tests_after = replace_once(
    tests_before,
    "        let mut owner = DrawingBirthPreparation::default();\n        for (denial, work) in denials(grant, demand)",
    "        let mut owner = DrawingBirthPreparation::default();\n        assert!(owner.terminal_is_empty());\n        for (denial, work) in denials(grant, demand)",
)
if source.read_text() != source_before or tests.read_text() != tests_before:
    raise ValueError("Drawing Birth sources changed while preparing the patch")
source.write_text(source_after)
tests.write_text(tests_after)
