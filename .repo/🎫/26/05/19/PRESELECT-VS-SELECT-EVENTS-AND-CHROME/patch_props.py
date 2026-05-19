from pathlib import Path

# index.tsx node props
tsx = Path("elements/client/lib/board/index.tsx")
t = tsx.read_text(encoding="utf-8")
for block in ("BoardNodeCircleProps", "BoardNodeRectangleProps"):
    needle = f"export type {block} = {{\n  children?: ReactNode;\n  contextMenu?: ContextMenuItem[];\n  draggable?: boolean;\n"
    if "highlighted?: boolean;" not in t.split(needle, 1)[1].split("selected?:", 1)[0]:
        t = t.replace(
            f"  root?: boolean;\n  selected?: boolean;",
            f"  root?: boolean;\n  highlighted?: boolean;\n  selected?: boolean;",
            2,
        )
# handle edge wire in tsx if any
tsx.write_text(t, encoding="utf-8")

ts = Path("elements/client/lib/board/index.ts")
t = ts.read_text(encoding="utf-8")
for iface in ("BoardHandleProps", "BoardEdgeProps", "BoardWireProps"):
    t = t.replace(
        f"export interface {iface} {{\n",
        f"export interface {iface} {{\n",
    )
repls = [
    ("export interface BoardHandleProps {\n\tangle:", "export interface BoardHandleProps {\n\tangle:"),
]
for marker in ("BoardHandleProps", "BoardEdgeProps", "BoardWireProps"):
    old = f"\tid: string;\n\tselected?: boolean;"
    if marker == "BoardEdgeProps":
        old = f"\tid: string;\n\tselected?: boolean;"
    # insert highlighted before selected in each interface
    idx = t.find(f"export interface {marker}")
    if idx < 0:
        continue
    chunk = t[idx : idx + 400]
    if "highlighted?: boolean;" in chunk:
        continue
    t = t[:idx] + t[idx:].replace("\tselected?: boolean;", "\thighlighted?: boolean;\n\tselected?: boolean;", 1)

# newBoardNodeFromProps
t = t.replace(
    "\t\t\tselected: props.selected,\n\t\t\tshape: \"rectangle\",",
    "\t\t\thighlighted: props.highlighted,\n\t\t\tselected: props.selected,\n\t\t\tshape: \"rectangle\",",
)
t = t.replace(
    "\t\tselected: props.selected,\n\t\tshape: \"circle\",",
    "\t\thighlighted: props.highlighted,\n\t\tselected: props.selected,\n\t\tshape: \"circle\",",
)
# props equal
t = t.replace(
    "\t\ta.selected === b.selected &&\n\t\ta.style === b.style &&",
    "\t\ta.highlighted === b.highlighted &&\n\t\ta.selected === b.selected &&\n\t\ta.style === b.style &&",
    3,
)
# instance sync key for node
t = t.replace(
    "\t\ta.selected !== b.selected ||\n\t\ta.style !== b.style ||",
    "\t\ta.highlighted !== b.highlighted ||\n\t\ta.selected !== b.selected ||\n\t\ta.style !== b.style ||",
    1,
)
ts.write_text(t, encoding="utf-8")
print("patched props")
