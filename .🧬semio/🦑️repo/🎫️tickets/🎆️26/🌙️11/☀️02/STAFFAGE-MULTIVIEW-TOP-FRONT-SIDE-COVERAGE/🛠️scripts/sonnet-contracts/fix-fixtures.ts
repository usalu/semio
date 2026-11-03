import { readFileSync, writeFileSync } from "node:fs";
for (const file of process.argv.slice(2)) {
  const text = readFileSync(file, "utf8");
  const match = /(\n(\s*)"geometry": \{\n\s*"elevation": \[\n)((?:.|\n)*?)(\n\s*\]\n\s*\},\n\s*"dimensionalNote")/u.exec(text.slice(text.indexOf('"symbols"')));
  if (match === null) throw new Error(`no symbol geometry in ${file}`);
  const offset = text.indexOf('"symbols"');
  const whole = match[0];
  const elevation = match[3]!;
  const indent = match[2]!;
  const rebuilt = `${match[1]}${elevation}\n${indent}  ],\n${indent}  "front": [\n${elevation}\n${indent}  ],\n${indent}  "plan": [\n${elevation}\n${indent}  ]\n${indent}},\n${indent.slice(0, -2)}  "dimensionalNote"`;
  void rebuilt;
  const at = offset + match.index;
  const replaced = `${match[1]}${elevation}\n${indent}  ],\n${indent}  "front": [\n${elevation}\n${indent}  ],\n${indent}  "plan": [\n${elevation}\n${indent}  ]\n${indent}},\n${indent}"dimensionalNote"`;
  writeFileSync(file, text.slice(0, at) + replaced + text.slice(at + whole.length));
  JSON.parse(readFileSync(file, "utf8"));
}
