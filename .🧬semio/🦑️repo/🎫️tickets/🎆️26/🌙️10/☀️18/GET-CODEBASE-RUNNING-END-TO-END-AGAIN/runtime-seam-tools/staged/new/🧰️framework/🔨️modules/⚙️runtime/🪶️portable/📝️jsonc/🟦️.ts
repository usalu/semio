/** 📝️ Parses JSON with `//` and block comments and trailing commas; the portable counterpart of a runtime's native JSONC parser, with `jsonc-parser` as test oracle. */
export function parseJsoncV1(text: string): any {
  let output = "";
  let index = 0;
  const length = text.length;
  while (index < length) {
    const character = text[index]!;
    if (character === '"') {
      let end = index + 1;
      while (end < length && text[end] !== '"') { if (text[end] === "\\") end++; end++; }
      output += text.slice(index, end + 1);
      index = end + 1;
      continue;
    }
    if (character === "/" && text[index + 1] === "/") {
      while (index < length && text[index] !== "\n") index++;
      continue;
    }
    if (character === "/" && text[index + 1] === "*") {
      const end = text.indexOf("*/", index + 2);
      if (end < 0) throw new SyntaxError("Unterminated JSONC comment");
      index = end + 2;
      output += " ";
      continue;
    }
    if (character === ",") {
      let probe = index + 1;
      for (;;) {
        while (probe < length && /\s/u.test(text[probe]!)) probe++;
        if (text[probe] === "/" && text[probe + 1] === "/") { while (probe < length && text[probe] !== "\n") probe++; continue; }
        if (text[probe] === "/" && text[probe + 1] === "*") { const end = text.indexOf("*/", probe + 2); if (end < 0) throw new SyntaxError("Unterminated JSONC comment"); probe = end + 2; continue; }
        break;
      }
      if (text[probe] === "}" || text[probe] === "]") { index++; continue; }
    }
    output += character;
    index++;
  }
  return JSON.parse(output.charCodeAt(0) === 0xfeff ? output.slice(1) : output);
}
