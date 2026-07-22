#!/usr/bin/env python3
"""Remove InputStepper / channel param overlays — channels are ports only."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve()
while not (ROOT / "flow/core/rs/lib.rs").exists():
    ROOT = ROOT.parent
    if ROOT == ROOT.parent:
        raise SystemExit("repo root not found")


def strip_or_patterns(text: str, patterns: list[str]) -> str:
    for pat in patterns:
        text = re.sub(rf"\s*\|\s*{pat}\s*", " ", text)
        text = re.sub(rf"{pat}\s*\|\s*", "", text)
    return text


def remove_match_arms(text: str, arm_starts: list[str]) -> str:
    """Remove full match arms that begin with one of the given patterns."""
    for start in arm_starts:
        # Single-line arm ending with `,`
        text = re.sub(rf"[ \t]*{re.escape(start)}[^\n]*,\n", "", text)
        # Multi-line arm: start line then until a line that is just `},` or `),` or a sibling arm
        pattern = rf"[ \t]*{re.escape(start)}.*?,\n(?=[ \t]*(?:Widget::|WidgetDescriptor::|DagNodeKind::|Some\(|None|_ =>|}}))"
        text = re.sub(pattern, "", text, flags=re.S)
    return text


def remove_block(text: str, start_marker: str, end_marker: str | None = None) -> str:
    if start_marker not in text:
        return text
    start = text.index(start_marker)
    if end_marker:
        end = text.index(end_marker, start)
    else:
        # find matching brace block from first `{` after marker
        brace = text.index("{", start)
        depth = 0
        i = brace
        while i < len(text):
            if text[i] == "{":
                depth += 1
            elif text[i] == "}":
                depth -= 1
                if depth == 0:
                    # include trailing newline and optional comma
                    end = i + 1
                    if end < len(text) and text[end] == ",":
                        end += 1
                    if end < len(text) and text[end] == "\n":
                        end += 1
                    break
            i += 1
        else:
            return text
    # also eat preceding docstring/comment lines
    while start > 0 and text[start - 1] != "\n":
        start -= 1
    line_start = text.rfind("\n", 0, start) + 1
    # include preceding `///` docs
    while line_start > 0:
        prev_nl = text.rfind("\n", 0, line_start - 1) + 1
        prev_line = text[prev_nl:line_start]
        if prev_line.lstrip().startswith("///") or prev_line.lstrip().startswith("//"):
            line_start = prev_nl
            continue
        break
    return text[:line_start] + text[end:]


def remove_function(text: str, sig: str) -> str:
    idx = text.find(sig)
    if idx < 0:
        return text
    # include preceding docs
    line_start = text.rfind("\n", 0, idx) + 1
    while line_start > 0:
        prev_nl = text.rfind("\n", 0, line_start - 1) + 1
        prev_line = text[prev_nl:line_start]
        if prev_line.lstrip().startswith("///") or prev_line.lstrip().startswith("//") or prev_line.lstrip().startswith("#["):
            line_start = prev_nl
            continue
        break
    brace = text.index("{", idx)
    depth = 0
    i = brace
    while i < len(text):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                end = i + 1
                if end < len(text) and text[end] == "\n":
                    end += 1
                return text[:line_start] + text[end:]
        i += 1
    return text


def clean_flow_core(path: Path) -> None:
    text = path.read_text()
    # imports
    text = text.replace("stepper_widget_height, stepper_widget_width, ", "")
    text = text.replace(", DagStepperField", "")
    text = text.replace("DagStepperField, ", "")

    # remove StepperFieldSpec and helpers
    for sig in [
        "pub struct StepperFieldSpec",
        "fn default_stepper_schema()",
        "fn default_stepper_step()",
        "enum StepperSchemaKind",
        "fn default_stepper_fields_for_schema",
        "fn stepper_output_port",
        "fn effective_stepper_fields",
    ]:
        text = remove_function(text, sig) if "fn " in sig or "struct " in sig or "enum " in sig else text

    # remove NodeChrome::Stepper variant
    text = re.sub(
        r"\n    Stepper \{\n        schema: String,\n        fields: Vec<StepperFieldSpec>,\n        step: f64,\n    },\n",
        "\n",
        text,
    )

    # remove Widget::InputStepper variant
    text = re.sub(
        r"\n    InputStepper \{\n        id: String,\n        #\[serde\(default = \"default_stepper_schema\"\)\]\n        schema: String,\n        #\[serde\(default\)\]\n        fields: Vec<StepperFieldSpec>,\n        #\[serde\(default = \"default_stepper_step\"\)\]\n        step: f64,\n    },\n",
        "\n",
        text,
    )

    # remove WidgetDescriptor::InputStepper variant
    text = re.sub(
        r"\n    InputStepper \{\n        #\[serde\(default\)\]\n        id: Option<String>,\n        #\[serde\(default\)\]\n        schema: Option<String>,\n        #\[serde\(default\)\]\n        step: Option<f64>,\n    },\n",
        "\n",
        text,
    )

    # or-patterns
    text = strip_or_patterns(
        text,
        [
            r"Widget::InputStepper \{ id, \.\. \}",
            r"WidgetDescriptor::InputStepper \{ id, \.\. \}",
            r"WidgetDescriptor::InputStepper \{ id, \.\.\}",
        ],
    )

    # dedicated match arms / blocks
    arm_blocks = [
        (r"Widget::InputStepper \{ schema, fields, step, \.\. \} => NodeChrome::Stepper \{[^}]+\},", ""),
        (r"Widget::InputStepper \{ id, schema, fields, \.\. \} => \{\n(?:.*?\n)*?                Some\(Neuron \{ id: id\.clone\(\), kind: \"core\.stepper\"\.into\(\), params, tree: None \}\)\n            \}", ""),
        (r"Widget::InputStepper \{ schema, \.\. \} => schema\.clone\(\),", ""),
        (r"Widget::InputStepper \{ schema, \.\. \} => \{\n(?:.*?\n)*?        \}", ""),
        (r"Widget::InputStepper \{ schema, \.\. \} => \(vec!\[\], vec!\[stepper_output_port\(schema\)\], false, false\),", ""),
        (r"Widget::InputStepper \{ schema, fields, \.\. \} => \{\n(?:.*?\n)*?            \(stepper_widget_width\(\), stepper_widget_height\(count\)\)\n        \}", ""),
        (r"Widget::InputStepper \{ \.\. \} => Some\(\"core\.stepper\"\.into\(\)\),", ""),
        (r"Widget::InputStepper \{ schema, fields, \.\. \} => \{\n(?:.*?\n)*?        \}", ""),
        (r"Widget::InputStepper \{ schema, fields, step, \.\. \} => \{\n(?:.*?\n)*?            DagNodeSpec \{ id, name, abbreviation, icon, x, y, width, height, operator_kind: None, properties: PropertyBag::new\(\), kind: DagNodeKind::Stepper \{ fields: dag_fields, output \} \}\n        \}", ""),
        (r"WidgetDescriptor::InputStepper \{ schema, step, \.\. \} => \{\n(?:.*?\n)*?            Widget::InputStepper \{ id, fields: vec!\[\], schema, step \}\n        \}", ""),
        (r"Widget::InputStepper \{ id, schema, fields, \.\. \} => \{\n(?:.*?\n)*?                    seeds\.insert\(id\.clone\(\), channel_output\(schema, dict\)\);\n                \}", ""),
        (r"\(Widget::InputStepper \{ schema, fields, step, \.\. \}, DagNodeKind::Stepper \{ fields: dag_fields, \.\. \}\) => \{\n(?:.*?\n)*?                \}", ""),
        (r"WidgetDescriptor::InputStepper \{ \.\. \} => \"stepper\"\.into\(\),", ""),
        (r"Widget::InputStepper \{ id, schema, fields, step, \.\. \} => \{\n(?:.*?\n)*?            \}", ""),
    ]
    for pat, repl in arm_blocks:
        text = re.sub(pat, repl, text, flags=re.S)

    # methods
    text = remove_function(text, "pub fn set_stepper_field_value(&mut self, widget_id: &str, field_key: &str, value: f64)")
    text = remove_function(text, "pub fn stepper_overlay_state_json(&self) -> Result<String, FlowCoreError>")
    text = remove_function(text, "pub fn param_overlay_paint_state_json(&self) -> Result<String, FlowCoreError>")

    # wasm bindings
    text = remove_function(text, "pub fn set_stepper_field_value(&self, widget_id: &str, field_key: &str, value: f64)")
    text = remove_function(text, "pub fn stepper_overlay_state_json(&self) -> Result<String, JsValue>")
    text = remove_function(text, "pub fn param_overlay_paint_state_json(&self) -> Result<String, JsValue>")

    # forms bridge
    text = remove_function(text, "fn patch_stepper_fields(widget: &mut serde_json::Value, value: &serde_json::Value)")
    text = text.replace("        InputStepper,\n", "")
    text = text.replace('                "inputStepper" => Some(Self::InputStepper),\n', "")
    text = text.replace("                Some(WidgetPatchKind::InputStepper) => patch_stepper_fields(widget, value),\n", "")

    # tests import
    text = text.replace("use super::{effective_stepper_fields, FlowFixture, Widget};\n", "use super::{FlowFixture, Widget};\n")

    path.write_text(text)
    print(f"updated {path}")


def clean_dag(path: Path) -> None:
    text = path.read_text()

    text = remove_function(text, "pub fn stepper_widget_width() -> f64")
    text = remove_function(text, "pub fn stepper_widget_height(field_count: usize) -> f64")

    text = re.sub(
        r"/// 🎚️ One named numeric field inside a stepper input widget\.\n#\[derive[^\]]+\]\n#\[serde[^\]]+\]\npub struct DagStepperField \{\n(?:.*?\n)*?\}\n\n",
        "",
        text,
    )

    text = text.replace("computation, slider, stepper, select, or screen", "computation, slider, select, or screen")
    text = re.sub(
        r"\n    Stepper \{\n        fields: Vec<DagStepperField>,\n        output: IoPortSpec,\n    },\n",
        "\n",
        text,
    )
    text = text.replace('        DagNodeKind::Stepper { .. } => "stepper",\n', "")
    text = text.replace(" | DagNodeKind::Stepper { output, .. }", "")
    text = text.replace(" | DagNodeKind::Stepper { .. }", "")
    text = re.sub(
        r"\n        DagNodeKind::Stepper \{ fields, \.\. \} => \{\n            node\.width = stepper_widget_width\(\);\n            node\.height = stepper_widget_height\(fields\.len\(\)\);\n        \}",
        "",
        text,
    )

    # param overlay + stepper overlay methods
    text = remove_function(text, "fn is_editable_input_port(port: &IoPortSpec) -> bool")
    text = remove_function(text, "fn param_overlay_rows_for_node(node: &DagNodeSpec, lod: DagDrawLod) -> Vec<serde_json::Value>")
    text = remove_function(text, "pub fn stepper_overlay_state_json(&self) -> Result<String, DagError>")
    text = remove_function(text, "pub fn param_overlay_paint_state_json(&self) -> Result<String, DagError>")

    # paint arm for Stepper
    text = re.sub(
        r"\n                DagNodeKind::Stepper \{ fields, \.\. \} => \{\n(?:.*?\n)*?                \}",
        "",
        text,
        count=1,
    )

    path.write_text(text)
    print(f"updated {path}")


def main() -> None:
    clean_flow_core(ROOT / "flow/core/rs/lib.rs")
    clean_dag(ROOT / "infinite/board/port/directed/dag/rs/lib.rs")


if __name__ == "__main__":
    main()
