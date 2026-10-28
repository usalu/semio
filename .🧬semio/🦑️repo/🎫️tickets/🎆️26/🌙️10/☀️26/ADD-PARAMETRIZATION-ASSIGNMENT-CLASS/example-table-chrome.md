# Native Print Table for the Worked Example

The worked example is now wrapped in the existing AssignmentTable component in both language templates. Its localized title is Beispiel / Example. The component supplies the connected native table outline, title tab, hierarchy number, outline registration and normal Print spacing, just as it does for the two exercise matrices.

The change is limited to the table wrapper. It does not change the worked-example shapes, parameters, values or graph. No published PDF was overwritten in this side conversation; the parent task can include the wrapper in its next export.

## Verification

The language-neutral silhouette fixture now expects three native tables: example, A and B. AJV validates that contract. The focused test first failed because only the exercise tables were wrapped, then passed after integration: 18 tests, 73 assertions, zero failures.

## Changed Files

- 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-en.content.tex
- 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-de.content.tex
- 🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🔣️.json
- 🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🟦️.ts
- 🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🥒️.feature
- This report and the retained example-table-chrome.mjs authoring input.

The parent ticket was already open. It remains open for the separate ongoing work. Only this side conversation's temporary subdirectory was removed; no parent outputs or sources were removed.
