import { parse, prop, setProps, apply } from "./jsonspan.ts";
const t1 = `{
  "targets": {
    "a": {
      "executor": "repo:script",
      "cache": false,
      "options": { "command": "x" }
    },
    "b": { "executor": "repo:script", "options": { "command": "y" } }
  }
}
`;
const ast = parse(t1) as any;
const targets = prop(ast, "targets")!.value;
const a = prop(targets, "a")!.value, b = prop(targets, "b")!.value;
const edits = [
  ...setProps(t1, a, [["cache", true], ["inputs", ["default", "^default", { env: "X" }]], ["outputs", []]], ["executor"]),
  ...setProps(t1, b, [["cache", true], ["inputs", ["default", "^default"]], ["outputs", []]], ["executor"]),
];
const out = apply(t1, edits);
console.log(out); JSON.parse(out);
