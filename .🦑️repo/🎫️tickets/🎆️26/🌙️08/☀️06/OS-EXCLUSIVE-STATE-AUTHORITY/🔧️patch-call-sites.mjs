import fs from "fs";

function patch(path, replacements) {
  let text = fs.readFileSync(path, "utf8");
  for (const [from, to] of replacements) {
    const count = text.split(from).length - 1;
    if (count === 0) {
      console.error("MISSING in " + path + ": " + JSON.stringify(from).slice(0, 180));
      process.exit(1);
    }
    text = text.replaceAll(from, to);
    console.log(path + ": " + count + " replacement(s)");
  }
  fs.writeFileSync(path, text);
}

const SYNC = "टे्ট্রো";
