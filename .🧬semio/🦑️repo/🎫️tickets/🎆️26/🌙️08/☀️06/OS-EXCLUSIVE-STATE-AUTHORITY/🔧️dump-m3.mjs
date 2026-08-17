import fs from "fs";

function dump(file, ranges) {
  const lines = fs.readFileSync(file, "utf8").split("\n");
  for (const [start, end] of ranges) {
    console.log("\n===== " + file + ":" + start + "-" + end + " =====");
    for (let i = start; i <= end && i <= lines.length; i++) {
      console.log(String(i).padStart(5) + "|" + lines[i - 1]);
    }
  }
}

const plugin = "🧰️framework/🛍️products/💻️os/🔥️modules/🔌️plugin/🦀component.rs";
const host = "🧰️framework/🛍️products/💻️os/🔥️modules/🔌️plugin/🖥️host/🦀component.rs";
const channel = "🧰️framework/🛍️products/💻️os/🔥️modules/📣spr/🔸️channel/🦀component.rs";
const wit = "🧰️framework/🛍️products/💻️os/🔥️modules/🔌️plugin/📦packages/🦀rust/📄wit/📄world.wit";

console.log("plugin", fs.existsSync(plugin));
console.log("host", fs.existsSync(host));
console.log("channel", fs.existsSync(channel));
console.log("wit", fs.existsSync(wit));

dump(plugin, [[2520,2700],[2820,3000],[3240,3600],[3980,4600],[4860,5100],[5500,5700],[7880,7941]]);
dump(host, [[1,462]]);
dump(channel, [[1,120],[200,450],[500,900]]);
dump(wit, [[1,120]]);
