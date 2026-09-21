import { afterEach, expect, it } from "vitest";
import React from "react";
import { cleanup, render } from "@testing-library/react";

afterEach(cleanup);

const control = (kind: "select" | "input") =>
  kind === "select"
    ? React.createElement("select", { key: "polymorphic-control", "data-testid": "control", defaultValue: "system" },
        React.createElement("option", { value: "system" }, "System"),
        React.createElement("option", { value: "dark" }, "Dark"))
    : React.createElement("input", { key: "polymorphic-control", "data-testid": "control", defaultValue: "" });

it("retains focus for a same-key same-kind successor and drops it when the control kind changes", () => {
  const view = render(control("select"));
  const initial = view.getByTestId("control");
  initial.focus();
  expect(document.activeElement).toBe(initial);

  view.rerender(control("select"));
  const retained = view.getByTestId("control");
  expect(retained).toBe(initial);
  expect(document.activeElement).toBe(retained);

  view.rerender(control("input"));
  const replacement = view.getByTestId("control");
  expect(replacement).not.toBe(initial);
  expect(document.activeElement).toBe(document.body);
});
