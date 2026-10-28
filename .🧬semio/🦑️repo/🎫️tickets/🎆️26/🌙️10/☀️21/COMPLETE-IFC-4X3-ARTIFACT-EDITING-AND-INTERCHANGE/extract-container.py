from pathlib import Path
root = Path("C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter")
path = root / "🧱️node-tree/🟦️.tsx"
value = path.read_text(encoding="utf-8")
start = value.index("function ContainerView(")
end = value.index("\n//#region Tree", start)
replacement = '''function ContainerView({ store, record, context }: { readonly store: UiDocumentStore; readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const presence = usePresenceOverlayEntry(record.key);
  const childIds = record.children ?? [];
  const keys = uiChildReactKeys(store.getState(), childIds);
  const children = childIds.map((id, index) => <UiNodeView key={keys[index]} store={store} id={id} context={context} />);
  const activate = (record.bindings ?? []).some(binding => binding.trigger === "activate");
  return <UiContainerFrame record={record} id={nodeDomId(store, record, context.domScope)} selected={presence.selected} hovered={presence.hovered} onActivate={activate ? event => { event.stopPropagation(); dispatchTrigger(context, record, "activate"); } : undefined}>{children}</UiContainerFrame>;
}
'''
value = value[:start] + replacement + value[end:]
anchor = 'import { documentControlContext } from "../🎛️control-port/🟦️.ts";'
if value.count(anchor) != 1:
    raise RuntimeError("container import changed")
value = value.replace(anchor, anchor + '\nimport { UiContainerFrame } from "../🎛️control-port/container.tsx";')
path.write_text(value, encoding="utf-8", newline="\n")

