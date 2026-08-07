import fs from "fs";
const lines = fs.readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs", "utf8").split("\n");
for (let i = 0; i < 95; i++) console.log(`${i + 1}|${lines[i]}`);
