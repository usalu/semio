from pathlib import Path
root = Path("C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter")
def update(relative, edits):
    path = root / relative
    value = path.read_text(encoding="utf-8")
    for before, after, count in edits:
        actual = value.count(before)
        if actual != count:
            raise RuntimeError(f"{relative}: expected {count}, found {actual}: {before!r}")
        value = value.replace(before, after)
    path.write_text(value, encoding="utf-8", newline="\n")
update("✍️input-views/🟦️.tsx", [
    ('import { interpLabel, type UiInterpreterContext }', 'import { interpLabel }', 1),
    ('import { dispatchTrigger, resolveControlIconNode, toUiValue, useContinuousTriggerLane }', 'import { resolveControlIconNode, toUiValue, useContinuousTriggerLane }', 1),
    ('import { nodeDomId } from "../🪪️stable-dom-ids/🟦️.ts";', 'import { controlDomId } from "../🪪️stable-dom-ids/🟦️.ts";\nimport { dispatchControlTrigger as dispatchTrigger, type UiControlCompletion, type UiControlContext } from "../🎛️control-port/🟦️.ts";', 1),
    ('ReturnType<UiInterpreterContext["onIntent"]>', 'UiControlCompletion', 1),
    ('readonly context: UiInterpreterContext', 'readonly context: UiControlContext', 7),
    ('context.store.getNodeSnapshot(', 'context.controls.read(', 2),
    ('context.store.subscribeRevision(', 'context.controls.subscribe(', 1),
    ('!current || current.disabled ||', '!current || context.controls.disabled(current) ||', 1),
    ('nodeDomId(context.store, record, context.domScope)', 'controlDomId(context, record)', 13),
    ('disabled={record.disabled}', 'disabled={context.controls.disabled(record)}', 8),
    ('orbs={[{ disabled: record.disabled,', 'orbs={[{ disabled: context.controls.disabled(record),', 1),
    ('const storedNumber = Number(component.value);', 'const value = context.controls.value(record) ?? component.value;\n  const storedNumber = Number(value);', 1),
    ('const published = factor != null && component.value.trim() !== "" && Number.isFinite(storedNumber) ? shownText(storedNumber) : component.value;', 'const published = factor != null && value.trim() !== "" && Number.isFinite(storedNumber) ? shownText(storedNumber) : value;', 1),
    ('value={commitOnBlur ? draft : component.value}', 'value={commitOnBlur ? draft : published}', 1),
    ('value={component.value}', 'value={context.controls.value(record) ?? component.value}', 2),
    ('value={component.value || undefined}', 'value={(context.controls.value(record) ?? component.value) || undefined}', 1),
    ('className="w-full min-w-0"\n      max={component.max}', 'className="w-full min-w-0"\n      disabled={context.controls.disabled(record)}\n      max={component.max}', 1),
])
update("🔤️display-views/🟦️.tsx", [
    ('import { dispatchTrigger, resolveControlIconNode }', 'import { resolveControlIconNode }', 1),
    ('import { nodeDomId } from "../🪪️stable-dom-ids/🟦️.ts";', 'import { controlDomId } from "../🪪️stable-dom-ids/🟦️.ts";\nimport { dispatchControlTrigger as dispatchTrigger, type UiControlContext } from "../🎛️control-port/🟦️.ts";', 1),
    ('readonly context: UiInterpreterContext', 'readonly context: UiControlContext', 1),
    ('nodeDomId(context.store, record, context.domScope)', 'controlDomId(context, record)', 1),
    ('store: context.store', 'source: context.controls.identity', 2),
    ('pending.current.store !== context.store', 'pending.current.source !== context.controls.identity', 1),
    ('context.store.getNodeSnapshot(record.id)', 'context.controls.read(record.id)', 1),
    ('!current || current.key !== record.key || current.component.type !== "button" || current.disabled', '!current || current !== record || current.component.type !== "button" || context.controls.disabled(current)', 1),
    ('disabled={record.disabled || state.busy}', 'disabled={context.controls.disabled(record) || state.busy}', 1),
    ('import { type UiInterpreterContext } from "../🧷️interpreter-context/🟦️.ts";\n', '', 1),
])

