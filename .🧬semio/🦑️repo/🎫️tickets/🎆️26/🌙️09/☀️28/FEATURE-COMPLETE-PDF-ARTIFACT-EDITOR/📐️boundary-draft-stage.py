"""📐 Stages an explicit atomic decimal-quad input policy for both retained renderers."""

from pathlib import Path
import json
import sys

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated" / "boundary-draft"
UI = Path("🧰️framework/🔨️modules/🖱️ui")
RENDERER = Path("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine")
INTERPRETER = RENDERER / "🧱️elements/🗣️Interpreter"
COMPONENT = UI / "🧬️contract/🧩️component"
RUST = "/// 📏 A blur-committed text field carrying four finite decimal coordinates or an empty reset.\npub const UI_DECIMAL_QUAD_COMMIT: &str = \"blurDecimalQuad\";\n\n/// 🫰 Reports whether an authored text input keeps a draft until Enter or blur.\npub fn input_commits_on_blur(commit: Option<&str>) -> bool {\n    matches!(commit, Some(\"blur\" | UI_DECIMAL_QUAD_COMMIT))\n}\n\n/// 🧮 Returns a round-trippable decimal tuple. Signed zeros denote the same coordinate.\npub fn decimal_quad_text(text: &str) -> Option<String> {\n    if text.trim().is_empty() { return Some(String::new()); }\n    let mut tokens = text.split_ascii_whitespace();\n    let mut values = [0.0; 4];\n    for value in &mut values {\n        let number = tokens.next()?.parse::<f64>().ok().filter(|number| number.is_finite())?;\n        *value = if number == 0.0 { 0.0 } else { number };\n    }\n    tokens.next().is_none().then(|| values.map(|value| value.to_string()).join(\" \"))\n}\n"
TS = "/** 📏 A blur-committed text field carrying four finite decimal coordinates or an empty reset. */\nexport const UI_DECIMAL_QUAD_COMMIT = \"blurDecimalQuad\";\n\nexport function uiInputCommitsOnBlur(commit: string | null | undefined): boolean {\n  return commit === \"blur\" || commit === UI_DECIMAL_QUAD_COMMIT;\n}\n\n/** 🧮 Returns a round-trippable decimal tuple. Signed zeros denote the same coordinate. */\nexport function uiDecimalQuad(text: string): string | undefined {\n  if (/^\\p{White_Space}*$/u.test(text)) return \"\";\n  const values = text.replace(/^[\\t\\n\\f\\r ]+|[\\t\\n\\f\\r ]+$/gu, \"\").split(/[\\t\\n\\f\\r ]+/u);\n  if (values.length !== 4 || values.some(value => !/^[+-]?(?:[0-9]+(?:\\.[0-9]*)?|\\.[0-9]+)(?:[eE][+-]?[0-9]+)?$/u.test(value))) return;\n  const numbers = values.map(Number);\n  if (numbers.every(Number.isFinite)) return numbers.join(\" \");\n}\n"


def single(old, new, text):
    assert text.count(old) == 1, old
    return text.replace(old, new)


def browser(text):
    text = single('import { interpLabel }', 'import { UI_DECIMAL_QUAD_COMMIT, uiDecimalQuad, uiInputCommitsOnBlur } from "@semio-tech/ui-contract/component";\nimport { interpLabel }', text)
    text = single('  const commitOnBlur = component.commit === "blur";', '''  const commitOnBlur = uiInputCommitsOnBlur(component.commit);
  const decimalQuad = component.kind === "text" && component.commit === UI_DECIMAL_QUAD_COMMIT;
  const textForPublication = (text: string) => decimalQuad ? uiDecimalQuad(text) ?? text : text;''', text)
    text = single('? shownText(storedNumber) : value;', '? shownText(storedNumber) : textForPublication(value);', text)
    text = single('target: JSON.stringify([draftScope, input.kind, input.displayFactor, input.precision]), value: factor != null && input.value.trim() !== "" && Number.isFinite(Number(input.value)) ? shownText(Number(input.value)) : input.value,', 'target: JSON.stringify([draftScope, input.kind, input.displayFactor, input.precision]), value: factor != null && input.value.trim() !== "" && Number.isFinite(Number(input.value)) ? shownText(Number(input.value)) : textForPublication(input.value),', text)
    text = single('      const input = current.component;\n      return { target:', '      const input = current.component;\n      if (decimalQuad !== (input.kind === "text" && input.commit === UI_DECIMAL_QUAD_COMMIT)) return null;\n      return { target:', text)
    text = single('  const commitValue = (raw: string) => {\n    const number', '''  const commitValue = (raw: string) => {
    if (decimalQuad) {
      const canonical = uiDecimalQuad(raw);
      if (canonical === undefined) { setRefusal({ message: null }); return; }
      setRefusal(null);
      setDraft(canonical);
      commitDraft(canonical);
      return;
    }
    const number''', text)
    return text


def native_events(text):
    text = single('    matches!(node, UiNode::Input(input) if input.commit.as_deref() == Some("blur"))', '    matches!(node, UiNode::Input(input) if ui_contract::input_commits_on_blur(input.commit.as_deref()))', text)
    text = single('        UiNode::Input(input) => Some(match input.display_factor', '''        UiNode::Input(input) if input.input_kind == "text" && input.commit.as_deref() == Some(ui_contract::UI_DECIMAL_QUAD_COMMIT) =>
            ui_contract::decimal_quad_text(&input.value),
        UiNode::Input(input) => Some(match input.display_factor''', text)
    text = single('} else { DslValue::String(text.to_string()) };\n            let trigger', '''} else if input.input_kind == "text" && input.commit.as_deref() == Some(ui_contract::UI_DECIMAL_QUAD_COMMIT) {
                DslValue::String(ui_contract::decimal_quad_text(text)?)
            } else { DslValue::String(text.to_string()) };
            let trigger''', text)
    text = single('        UiNode::Input(input) if input.input_kind == "number" => typed_number(text,', '''        UiNode::Input(input) if input.input_kind == "text" && input.commit.as_deref() == Some(ui_contract::UI_DECIMAL_QUAD_COMMIT) =>
            ui_contract::decimal_quad_text(text).is_none().then(String::new),
        UiNode::Input(input) if input.input_kind == "number" => typed_number(text,''', text)
    return text


def declarative(text):
    text = "import { UI_DECIMAL_QUAD_COMMIT, uiDecimalQuad, uiInputCommitsOnBlur } from \"@semio-tech/ui-contract/component\";\n" + text
    return single('''      const commitOnBlur = control.commit === "blur";
      const commitValue = (raw: string) => dispatchDeclarativeControlAction(onAction, control.onChange, { value: control.inputKind === "number" ? Number(raw) : raw });''', '''      const commitOnBlur = uiInputCommitsOnBlur(control.commit);
      const commitValue = (raw: string) => {
        const value = control.commit === UI_DECIMAL_QUAD_COMMIT ? uiDecimalQuad(raw) : control.inputKind === "number" ? Number(raw) : raw;
        if (value !== undefined) dispatchDeclarativeControlAction(onAction, control.onChange, { value });
      };''', text)


def use_helper(old, new, text):
    text = single(old, new, text)
    return 'import { uiInputCommitsOnBlur } from "@semio-tech/ui-contract/component";\n' + text


def page_inspector(text):
    return single('.commit(inspector_text("blur")?)', '''.commit(inspector_text(if field.action == "set-page-extra" && object_id.starts_with("box.") {
            semio_ui_contract::UI_DECIMAL_QUAD_COMMIT
        } else { "blur" })?)''', text)

DRAFTS = json.loads(bytes.fromhex("7b22706f6c696379223a22626c7572446563696d616c51756164222c227370656c6c696e6773223a5b7b22726177223a22205c742d31302e30202b326531203137302e30303030202b3238302e205c72222c2276616c756573223a5b2d31302c32302c3137302c3238305d7d2c7b22726177223a222d30202b30202e32352035652d333234222c2276616c756573223a5b302c302c302e32352c35652d3332345d7d2c7b22726177223a222b312e3235653231202d32452d393020312e323530303030303030303030303030326532312034652d3837222c2276616c756573223a5b312e3235652b32312c2d32652d39302c312e32353030303030303030303030303033652b32312c34652d38375d7d2c7b22726177223a222d31652d393939203220332034222c2276616c756573223a5b302c322c332c345d7d2c7b22726177223a2220c2a0c285222c2276616c756573223a5b5d7d2c7b22726177223a223120322033222c2276616c756573223a6e756c6c7d2c7b22726177223a22312032203320342035222c2276616c756573223a6e756c6c7d2c7b22726177223a2231204e614e20332034222c2276616c756573223a6e756c6c7d2c7b22726177223a223120322033203165393939222c2276616c756573223a6e756c6c7d2c7b22726177223a2231203220307831302034222c2276616c756573223a6e756c6c7d2c7b22726177223a2231203220332034c2a0222c2276616c756573223a6e756c6c7d2c7b22726177223a2231203220332034e280a8222c2276616c756573223a6e756c6c7d2c7b22726177223a22315c75303030623220332034222c2276616c756573223a6e756c6c7d2c7b22726177223a22312c32203320342035222c2276616c756573223a6e756c6c7d5d2c226d6f7665223a7b226f626a656374223a22626f782e7472696d222c226265666f7265223a2231352034352031343520323535222c22636f6f7264696e61746573223a2220202b312e356532202034352e3132353020202b3137352e31323520203235352e3020222c2276616c7565223a5b3135302c34352e3132352c3137352e3132352c3235355d7d7d").decode("utf-8"))


def corpus(text):
    marker = '    "clearedViewSize": ['
    pos = text.index(marker)
    additions = json.dumps(DRAFTS, ensure_ascii=True, indent=2).splitlines()
    rendered = '    "decimalDrafts": {\n' + "\n".join("    " + line for line in additions[1:]) + ",\n"
    return text[:pos] + rendered + text[pos:]


def registered_test(text):
    marker = '    let malformed = edit_from_action("set-page-extra",'
    assert text.count(marker) == 1
    pos = text.index(marker)
    return text[:pos] + """    let policies = panel.children.iter().filter(|node| matches!(&node.component,
        semio_ui_contract::Component::Input(input) if input.commit.as_ref().is_some_and(|mode| mode.as_str() == semio_ui_contract::UI_DECIMAL_QUAD_COMMIT))).count();
    assert_eq!(policies, 5);
    let movement = &case["decimalDrafts"]["move"];
    assert_eq!(snapshot.pages[0].trim_box.unwrap()[2], 145.0);
    let edit = edit_from_action("set-page-extra", Some(&DslValue::from(box_args(movement)))).unwrap();
    let edits = apply_payload(&snapshot, &edit.action, &edit.payload).unwrap();
    assert_eq!(edits.len(), 1);
    apply_pdf_mutation(&mut snapshot, &edits[0]);
    let moved = &after_edit(&snapshot).pages[0];
    let expected: [f64; 4] = serde_json::from_value(movement["value"].clone()).unwrap();
    assert_eq!(moved.trim_box, Some(expected));
""" + text[pos:]




def browser_smoke(text):
    text = single('id: string, before: string, after: string) {\n  const field = await inspectorField', 'id: string, before: string, after: string, draft = after) {\n  const field = await inspectorField', text)
    text = single("  await field.fill(after);", "  await field.fill(draft);", text)
    return single('  await field("pdf-page-trim-box", spec.page.trim.before.join(" "), spec.page.trim.after.join(" "));',
        """  const trim = spec.page.trim.after.join(" ");
  if (editing) {
    const spelling = "  " + spec.page.trim.after.map(value => Number(value).toExponential()).join("  ") + " ";
    await commitInspectorField(owned, "pdf-page-trim-box", spec.page.trim.before.join(" "), trim, spelling);
    assert.equal(await owned.locator('[data-ui-node-key="pdf-page-trim-box"][aria-invalid="true"]').count(), 0);
  } else await inspectorField(owned, "pdf-page-trim-box", trim);""", text)



def stage(root, pdf):
    edits = {
        COMPONENT / "🎚️slider-geometry-and-key-handling/🦀️.rs": lambda s: s + "\n" + RUST,
        COMPONENT / "🦀️.rs": lambda s: single("    slider_axis_value,", "    UI_DECIMAL_QUAD_COMMIT, decimal_quad_text, input_commits_on_blur, slider_axis_value,", s),
        COMPONENT / "🟦️.ts": lambda s: s + "\n" + TS,
        UI / "🎯️targets/🧊️wgpu/⚡️events/🧮️control-commit-rules/🦀️.rs": native_events,
        UI / "🌳️retained/🌳️tree/🔀️reconcile/🎛️widget-nodes/🦀️.rs":
            lambda s: single('props.commit.as_ref().is_some_and(|value| value.as_str() == "blur")', 'ui_contract::input_commits_on_blur(props.commit.as_ref().map(|value| value.as_str()))', single('/// ⌨️ Which trigger an `Input`\'s value commits through — `InputProps`\' own doc names the pair\n/// (`Trigger::Change`/`Trigger::Commit`), and `commit == "blur"` is what picks between them, exactly\n/// as React\'s `InputView` picks `commitOnBlur ? "commit" : "change"`. Resolving it HERE keeps the\n/// retained node\'s single `on_change` field the one descriptor `events`\' commit authority\n/// dispatches, whichever trigger the guest actually bound.', "/// ⌨️ Resolves the input policy's trigger to the single retained action WGPU dispatches.", s)),
        INTERPRETER / "✍️input-views/🟦️.tsx": browser,
        INTERPRETER / "🎛️declarative-controls/🟦️.tsx": declarative,
        INTERPRETER / "🎛️control-port/body.ts": lambda s: use_helper('component.commit !== "blur"', '!uiInputCommitsOnBlur(component.commit)', s),
        RENDERER / "🧱️elements/🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🛡️intents/owned.ts":
            lambda s: use_helper('record.component.commit !== "blur"', '!uiInputCommitsOnBlur(record.component.commit)', s),
        pdf / "✏️editor/🖼️page/🔎️inspector/🦀️.rs": page_inspector,
        pdf / "🧪️tests/🧫️fixtures/📝️native-page-edit/🔣️.json": corpus,
        pdf / "✏️editor/🖼️page/🧪️tests/🔬️unit/📝️native-page-edit/🦀️.rs": registered_test,
        COMPONENT / "🎨️basic-widget-props/🦀️.rs": lambda s: single("    pub commit: Option<crate::UiText>,",
            '    /// 🎙️ Named commit policy. `blur` commits text exactly, `blurDecimalQuad` commits a finite decimal tuple or an empty reset.\n    pub commit: Option<crate::UiText>,', s),
    }
    OUTPUT.mkdir(exist_ok=True)
    mapping = {}
    for index, (path, edit) in enumerate(edits.items()):
        original = (root / path).read_text()
        target = OUTPUT / f"{index}{path.suffix}"
        target.write_text(edit(original))
        mapping[str(path)] = target.name
    (OUTPUT / "overlay.json").write_text(json.dumps(mapping, ensure_ascii=False, indent=2) + "\n")
    (OUTPUT / "browser-smoke.ts").write_text(browser_smoke((TICKET / "📜️pdf-browser-smoke.ts").read_text()))


if __name__ == "__main__":
    stage(Path.cwd(), Path(sys.argv[1]))
