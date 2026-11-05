const fs = require("node:fs");
const path = require("node:path");
const { createCanvas, loadImage } = require("./🗑️generated/t/node_modules/@napi-rs/canvas");
async function main() {
  const ids = ["person-elevation", "person-front", "person-plan", "car-plan", "thermal-person", "thermal-heat-pump"];
  const canvas = createCanvas(1080, 720), ctx = canvas.getContext("2d");
  ctx.fillStyle = "#f8f5ef"; ctx.fillRect(0, 0, 1080, 720);
  ctx.fillStyle = "#202020"; ctx.font = "24px sans-serif"; ctx.fillText("Canonical Staffage — Three Views and Thermal Casts", 24, 38);
  for (let i = 0; i < ids.length; i++) {
    const id = ids[i], x = (i % 3) * 360, y = 64 + Math.floor(i / 3) * 320;
    const svg = fs.readFileSync(path.join(__dirname, "🗑️generated/proof", id, "0000.svg"), "utf8");
    const box = svg.match(/viewBox="([^"]+)"/)[1].split(/\s+/).map(Number);
    const sized = svg.replace(/<svg[^>]+>/, tag => tag.replace(/width="[^"]+"/, `width="1200"`).replace(/height="[^"]+"/, `height="${1200 * box[3] / box[2]}"`));
    const image = await loadImage(Buffer.from(sized));
    const scale = Math.min(312 / image.width, 252 / image.height);
    ctx.drawImage(image, x + (360 - image.width * scale) / 2, y + (260 - image.height * scale) / 2, image.width * scale, image.height * scale);
    ctx.fillStyle = "#202020"; ctx.font = "18px sans-serif"; ctx.fillText(id, x + 24, y + 288);
  }
  fs.writeFileSync(path.join(__dirname, "🗑️generated/proof-casts.png"), canvas.toBuffer("image/png"));
}
main().catch(error => { console.error(error); process.exitCode = 1; });
