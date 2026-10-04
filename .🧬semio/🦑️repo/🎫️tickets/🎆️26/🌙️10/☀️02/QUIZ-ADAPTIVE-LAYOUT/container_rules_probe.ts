/** 🔬️ Prints how lightningcss reads the containers of the quiz stylesheet: `bun container_rules_probe.ts` lists every
 * rule that declares a container with its selector and the parsed declaration, and per `@container` rule its name, its
 * parsed condition and how many rules it holds. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { transform } from "lightningcss";

const css = readFileSync(resolve(import.meta.dir, "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🎨️.css"));
transform({
  filename: "🎨️.css",
  code: css,
  visitor: {
    Rule: {
      style(rule) {
        const declarations = rule.value.declarations.declarations.filter((declaration) => declaration.property.startsWith("container") || declaration.property === "unparsed");
        if (declarations.some((declaration) => JSON.stringify(declaration).includes("container"))) console.log(JSON.stringify({ selectors: rule.value.selectors, declarations }));
      },
      container(rule) {
        const value = rule.value as unknown as { readonly name?: unknown; readonly condition?: unknown; readonly rules: readonly unknown[] };
        console.log(JSON.stringify({ name: value.name, condition: value.condition, rules: value.rules.length }));
      },
    },
  },
});
