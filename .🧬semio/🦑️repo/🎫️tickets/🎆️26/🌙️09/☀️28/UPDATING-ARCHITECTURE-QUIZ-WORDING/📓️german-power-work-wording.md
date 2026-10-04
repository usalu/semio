# German Power-or-Work Quiz Wording

Changed the German physics wording to use **Arbeit** instead of **Energie(bedarf)** for the classification choice: the quiz description, question title, prompt, category label, and the two annual-quantity explanations now consistently say **„Leistung oder Arbeit?“** / **„Arbeit“**. English text and schema identifiers remain unchanged; scientific references that define power as energy per time remain accurate.

Catalog assertions cover the German question title, prompt, and category label. `bun nx run @teaching/architecture-quiz:test` passed all 5 files and 235 tests before concurrent catalog schema edits began. A rerun during those edits failed on schema/core issues in the quiz files (including newly required `quantity.additive` fields); a direct JSON/content assertion passed for the updated German title, category, and annual explanations.
