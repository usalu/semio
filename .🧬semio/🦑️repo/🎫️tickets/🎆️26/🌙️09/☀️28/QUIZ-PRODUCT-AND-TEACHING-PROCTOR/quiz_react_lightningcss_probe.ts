import { transform } from "lightningcss";
const css = `@media (min-width: 768px) { .quiz-home-grid { grid-template-columns: repeat(2, minmax(0, 1fr)); } .quiz-home-grid > [data-card="board"] { grid-column: 1 / -1; } }`;
transform({
  filename: "probe.css",
  code: Buffer.from(css),
  visitor: {
    Rule: {
      media(rule) {
        console.log(JSON.stringify(rule.value.query, null, 0).slice(0, 800));
        for (const inner of rule.value.rules) console.log(JSON.stringify(inner, null, 0).slice(0, 1200));
        return rule;
      },
    },
  },
});
