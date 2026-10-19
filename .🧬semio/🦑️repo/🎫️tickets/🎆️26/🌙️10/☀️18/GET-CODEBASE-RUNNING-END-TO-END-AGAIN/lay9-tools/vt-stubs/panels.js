import { createElement } from "react";
const Box = (props) => createElement("div", { "data-stub": "panel", ...Object.fromEntries(Object.entries(props).filter(([k]) => ["id", "className", "style"].includes(k))) }, props.children);
export const Group = Box, Panel = Box, Separator = Box, PanelGroup = Box, PanelResizeHandle = Box;
export const useGroupRef = () => ({ current: null }), usePanelRef = () => ({ current: null });
