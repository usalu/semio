import { renderToStaticMarkup } from "react-dom/server";
import { Tree, TreeStateProvider } from "../../../../../../ui/react/index.tsx";

const markup = renderToStaticMarkup(
  <TreeStateProvider>
    <Tree
      showLines
      sections={[
        {
          id: "objects",
          label: "Objects",
          defaultOpen: true,
          items: [
            {
              id: "object-left",
              label: "Hexagonal Cut Concrete Forest Left",
              description: "seed-left-001",
              defaultOpen: true,
              items: [
                { id: "v0", label: "b-l", description: "seed-left-001:v0" },
                { id: "v1", label: "b-l-m", description: "seed-left-001:v1" },
                { id: "v2", label: "b-l", description: "seed-left-001:v2" },
              ],
            },
          ],
        },
        {
          id: "references",
          label: "References",
          defaultOpen: false,
          items: [],
        },
      ]}
    />
  </TreeStateProvider>,
);

const guideLineCount = markup.match(/data-tree-guide-line/g)?.length ?? 0;
const guideSlots = markup.match(/data-slot="tree-guide"/g)?.length ?? 0;
const itemContents = markup.split('data-slot="tree-item-content"').length - 1;
const sectionContents = markup.split('data-slot="tree-section-content"').length - 1;

console.log("[DEBUG] guide lines total:", guideLineCount);
console.log("[DEBUG] tree-guide slots:", guideSlots);
console.log("[DEBUG] tree-item-content branches:", itemContents);
console.log("[DEBUG] tree-section-content branches:", sectionContents);

for (const [index, chunk] of markup.split('data-slot="tree-item-content"').slice(1).entries()) {
  const head = chunk.slice(0, 800);
  const lines = (head.match(/data-tree-guide-line/g) ?? []).length;
  const lefts = [...head.matchAll(/style="left:([0-9.]+)px"/g)].map((m) => m[1]);
  console.log(`[DEBUG] item-content branch ${index + 1}: guide lines=${lines}, lefts=${lefts.join(",")}`);
}

for (const [index, chunk] of markup.split('data-slot="tree-section-content"').slice(1).entries()) {
  const head = chunk.slice(0, 800);
  const lines = (head.match(/data-tree-guide-line/g) ?? []).length;
  const lefts = [...head.matchAll(/style="left:([0-9.]+)px"/g)].map((m) => m[1]);
  console.log(`[DEBUG] section-content branch ${index + 1}: guide lines=${lines}, lefts=${lefts.join(",")}`);
}
