import { render } from "@testing-library/react";
import { expect, test } from "vitest";
import { InterpretedUiNode } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx";
import { UiDocumentStore } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx";
import fixture from "../../../../../../../✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🧪️tests/🧫️fixtures/form-image.json";

const surface = "test.image-body";
const label = fixture.en.alt;
const record = (id, key, component, children = []) => ({
  id, key, component, children, layout: { kind: "leaf", width: "hug", height: "hug" }, style: {},
});

test("renders only a complete image source body and hides all transport rows", () => {
  const store = new UiDocumentStore(surface);
  const source = fixture.en.steps[0].blocks[0].src;
  const alternate = source.replace("image/png", "image/gif");
  const install = (revision, key, body, finish) => {
    const parts = [body.slice(0, 512), body.slice(512, 1024), body.slice(1024)].slice(0, finish ? 3 : 1);
    store.loadSnapshot({ surface, revision, root: 1, layoutEpoch: 0n, nodes: [
      record(1, "visible", { type: "image", src: key, alt: label, sourceBodyBytes: fixture.en.sourceBytes }, [2]),
      record(2, key, { type: "container", role: "plain" }, parts.map((_, i) => 3 + i)),
      ...parts.map((value, i) => record(3 + i, "part" + i, { type: "text", value })),
    ] });
  };
  const context = { dispatchIntent: () => {} };
  install(1, "first-body", source, false);
  const mounted = render(<InterpretedUiNode store={store} nodeId={1} context={context} />);
  expect(mounted.getByRole("img", { name: label }).getAttribute("src")).toBeNull();
  install(2, "first-body", source, true);
  mounted.rerender(<InterpretedUiNode store={store} nodeId={1} context={context} />);
  expect(mounted.getByRole("img", { name: label }).getAttribute("src")).toBe(source);
  expect(mounted.container.textContent).toBe("");
  install(3, "second-body", alternate, false);
  mounted.rerender(<InterpretedUiNode store={store} nodeId={1} context={context} />);
  expect(mounted.getByRole("img", { name: label }).getAttribute("src")).toBeNull();
  install(4, "second-body", alternate, true);
  mounted.rerender(<InterpretedUiNode store={store} nodeId={1} context={context} />);
  expect(mounted.getByRole("img", { name: label }).getAttribute("src")).toBe(alternate);
});
