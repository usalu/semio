import { cleanup, fireEvent, render, within } from "@testing-library/react";
import { afterEach, describe, expect, test } from "vitest";
import type { ActionDescriptor, WindowMeasure } from "@semio-tech/framework";
import { WindowMeasureSelect } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx";
import fixture from "../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🎛️generate-mode-interactions.json";

afterEach(cleanup);

const detail = fixture.previewDetail;
describe("procedural preview detail measure", () => {
  for (const language of ["en", "de"] as const) {
    test(language, async () => {
      const measure: Extract<WindowMeasure, { kind: "select" }> = {
        kind: "select",
        id: detail.id,
        label: detail[language],
        value: detail.default,
        items: detail.presets.map(preset => ({ id: detail.id + "-" + preset.value, value: preset.value, label: preset[language] })),
        onChange: { action: detail.action } as ActionDescriptor,
      };
      const actions: ActionDescriptor[] = [];
      const onAction = (action: ActionDescriptor) => { actions.push(action); };
      const mounted = render(<WindowMeasureSelect measure={measure} onAction={onAction} />);
      const screen = within(document.body);
      const trigger = screen.getByRole("combobox", { name: detail[language] });
      expect(trigger.getAttribute("data-published-value")).toBe(detail.default);
      fireEvent.keyDown(trigger, { key: "Enter" });
      const fine = detail.presets.find(preset => preset.value === "fine")!;
      const option = await screen.findByRole("option", { name: fine[language] });
      fireEvent.click(option);
      expect(actions).toEqual([{ action: detail.action, args: { value: fine.value } }]);
      mounted.rerender(<WindowMeasureSelect measure={{ ...measure, value: fine.value }} onAction={onAction} />);
      expect(trigger.getAttribute("data-published-value")).toBe(fine.value);
    });
  }
});
