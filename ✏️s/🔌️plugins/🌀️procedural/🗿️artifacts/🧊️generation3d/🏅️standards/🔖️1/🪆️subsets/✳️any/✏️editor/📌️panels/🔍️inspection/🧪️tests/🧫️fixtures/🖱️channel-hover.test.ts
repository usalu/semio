import { expect, test } from "bun:test";
import { JSDOM } from "jsdom";
import fixture from "./🖱️channel-hover.json";

test("inspector channel rows publish graph targets distinct from row identities", () => {
    const dom = new JSDOM("<div role=tree></div>");
    try {
        const { document, MouseEvent } = dom.window;
        const tree = document.querySelector("[role=tree]")!;
        for (const row of fixture.cases) {
            const element = document.createElement("div");
            element.id = row.key;
            element.setAttribute("role", row.kind === "literal" ? "group" : "treeitem");
            tree.append(element);
            let hover: { granularity: string; id: string } | undefined;
            element.addEventListener("pointerenter", () => {
                const direction = row.kind !== "output" ? "!i" : (!row.port || row.port.startsWith("!")) ? "!o" : "";
                hover = { granularity: "handle", id: `h:${row.widget}@${direction}${row.port}` };
            });
            element.addEventListener("pointerleave", () => { hover = undefined; });
            element.dispatchEvent(new MouseEvent("pointerenter"));
            expect(hover).toEqual(row.target);
            expect(hover?.id).not.toBe(element.id);
            element.dispatchEvent(new MouseEvent("pointerleave"));
            expect(hover).toBeUndefined();
        }
    } finally {
        dom.window.close();
    }
});
