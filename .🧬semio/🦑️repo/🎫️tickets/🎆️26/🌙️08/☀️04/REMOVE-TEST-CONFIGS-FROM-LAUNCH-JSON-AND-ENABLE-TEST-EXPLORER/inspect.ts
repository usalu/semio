import { readFileSync } from "fs";

const content = readFileSync(".vscode/launch.json", "utf8");

// Remove comments from JSONC string properly
function stripJsonComments(jsonc: string): string {
  let inString = false;
  let isEscaped = false;
  let result = "";
  
  for (let i = 0; i < jsonc.length; i++) {
    const char = jsonc[i];
    const nextChar = jsonc[i + 1];
    
    if (inString) {
      result += char;
      if (isEscaped) {
        isEscaped = false;
      } else if (char === "\\") {
        isEscaped = true;
      } else if (char === '"') {
        inString = false;
      }
    } else {
      if (char === '"') {
        inString = true;
        result += char;
      } else if (char === "/" && nextChar === "/") {
        // Skip till newline
        while (i < jsonc.length && jsonc[i] !== "\n" && jsonc[i] !== "\r") {
          i++;
        }
        result += "\n";
      } else if (char === "/" && nextChar === "*") {
        // Skip block comment
        i += 2;
        while (i < jsonc.length && !(jsonc[i] === "*" && jsonc[i + 1] === "/")) {
          i++;
        }
        i++; // skip closing slash
      } else {
        result += char;
      }
    }
  }
  return result;
}

const clean = stripJsonComments(content).replace(/,\s*([\}\]])/g, "$1");
try {
  const data = JSON.parse(clean);
  console.log(`Total configurations in launch.json: ${data.configurations.length}`);
  
  const testConfigs = data.configurations.filter((c: any) => {
    const name = c.name || "";
    const command = c.command || "";
    const group = c.presentation?.group || "";
    return name.includes("🧪") || name.includes("test") || command.includes("test") || group.includes("test");
  });

  console.log(`\nFound ${testConfigs.length} test configurations:`);
  testConfigs.forEach((c: any, index: number) => {
    console.log(`${index + 1}. [${c.presentation?.group || "no-group"}] "${c.name}" -> cmd: "${c.command}"`);
  });
} catch (e: any) {
  console.error("Parse error:", e);
}
