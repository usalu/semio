const path = process.argv[2];
let text = await Bun.file(path).text();
if (/import \{[\s\S]*?\bIcon\b[\s\S]*?\} from "@semio-tech\/ui-react"/.test(text)) {
  console.log("already");
  process.exit(0);
}
const needle = "  elementSkeleton,\n  initUiLocaleSync,";
const repl = "  elementSkeleton,\n  Icon,\n  initUiLocaleSync,";
if (!text.includes(needle)) {
  console.error("needle missing");
  process.exit(1);
}
await Bun.write(path, text.replace(needle, repl));
console.log("ok");
