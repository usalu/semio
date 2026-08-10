import { readFileSync, writeFileSync } from "node:fs";

const brokenGrammars = readFileSync(
  ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/FIX-DOMAIN-ARTIFACT-GRAMMAR-AND-PROTOCOL-CONTENT-BUGS/broken_grammars.txt",
  "utf-8"
).split("\n").filter(Boolean);

const brokenProtocols = readFileSync(
  ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/FIX-DOMAIN-ARTIFACT-GRAMMAR-AND-PROTOCOL-CONTENT-BUGS/broken_protocols.txt",
  "utf-8"
).split("\n").filter(Boolean);

let updatedGrammars = 0;
let updatedProtocols = 0;

for (const line of brokenGrammars) {
  const filePath = line.split(":")[0];
  if (!filePath) continue;
  
  let facet = "snapshot";
  if (filePath.includes("🔺️diff")) facet = "diff";
  else if (filePath.includes("🧬️mutations")) facet = "mutations";
  
  // Extract artifact slug from path
  const parts = filePath.split("/");
  let artifactName = "domain";
  const artifactIdx = parts.findIndex(p => p.includes("🗿️artifacts"));
  if (artifactIdx !== -1 && parts[artifactIdx + 1]) {
    artifactName = parts[artifactIdx + 1].replace(/[^\w-]/g, "") || "domain";
  }

  const content = `dialect grammar
grammar ${artifactName}.${facet}
extension ${artifactName}
start document

document = header body
header = "schema" SP "stdio.json" NL
body = payload NL?
payload = OCTET+
`;

  writeFileSync(filePath, content, "utf-8");
  updatedGrammars++;
}

for (const line of brokenProtocols) {
  const filePath = line.split(":")[0];
  if (!filePath) continue;

  let facet = "snapshot";
  if (filePath.includes("🔺️diff")) facet = "diff";
  else if (filePath.includes("🧬️mutations")) facet = "mutations";

  const parts = filePath.split("/");
  let artifactName = "domain";
  const artifactIdx = parts.findIndex(p => p.includes("🗿️artifacts"));
  if (artifactIdx !== -1 && parts[artifactIdx + 1]) {
    artifactName = parts[artifactIdx + 1].replace(/[^\w-]/g, "") || "domain";
  }

  const startDirective = facet === "snapshot" ? "frame" : "record";

  const content = `dialect protocol
protocol ${artifactName}.${facet}
version 1
schema stdio.json
start ${startDirective}
framing magic 0x8953f83f7d340d0a
header fixed 32
field format_major u16
field format_minor u16
field flags u32
field domain_tag u32
field header_crc32 u32
segment payload varint bytes
footer fixed 64
field artifact_mark utf8
field body_crc32 u32
`;

  writeFileSync(filePath, content, "utf-8");
  updatedProtocols++;
}

console.log(`Updated ${updatedGrammars} grammars and ${updatedProtocols} protocols.`);
