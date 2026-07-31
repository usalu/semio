import React from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem, NavbarExampleSelect } from "../../../../../../ui/js/react/index.tsx";

const selectMarkup = renderToStaticMarkup(
  React.createElement(
    Select,
    { id: "t", defaultValue: "a" },
    React.createElement(SelectTrigger, null, React.createElement(SelectValue, { placeholder: "Select" })),
    React.createElement(SelectContent, null, React.createElement(SelectItem, { value: "a" }, "A")),
  ),
);

console.log("--- SELECT ---");
console.log(selectMarkup.includes("chevron-down") ? "HAS chevron-down" : "MISSING chevron-down");
console.log(selectMarkup.includes("data-icon") ? "HAS data-icon" : "MISSING data-icon");
console.log(selectMarkup);

const navMarkup = renderToStaticMarkup(
  React.createElement(NavbarExampleSelect, {
    id: "navbar.example",
    value: "",
    options: [{ id: "nakagin", label: "Nakagin" }],
    onValueChange: () => undefined,
  }),
);

console.log("--- NAVBAR ---");
console.log(navMarkup.includes("chevron-down") ? "HAS chevron-down" : "MISSING chevron-down");
console.log(navMarkup.includes('data-icon="chevron-down"') ? "HAS data-icon=chevron-down" : "no data-icon attr");
console.log(navMarkup);
