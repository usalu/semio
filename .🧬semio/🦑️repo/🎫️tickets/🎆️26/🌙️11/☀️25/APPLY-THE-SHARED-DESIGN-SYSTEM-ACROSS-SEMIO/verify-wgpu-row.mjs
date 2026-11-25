import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createCanvas } from "@napi-rs/canvas";

const ticket = dirname(fileURLToPath(import.meta.url));
const root = resolve(ticket, "../../../../../../..");
const chrome = resolve(root, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖥️chrome/🦀️.rs");
const production = readFileSync(chrome, "utf8").match(/^pub fn push_chrome_row\([\s\S]*?^\}/m)?.[0];
assert(production, "production row painter");
const output = resolve(ticket, "🗑️generated/wgpu-row");
mkdirSync(output, { recursive: true });
const program = resolve(output, "row.rs");
const binary = resolve(output, process.platform === "win32" ? "row.exe" : "row");
writeFileSync(program, `
pub type Rgba = [u8; 4];
pub struct Rect { x: f32, y: f32, w: f32, h: f32 }
pub struct Theme { stroke_hairline: f32, border_normal: Rgba }
pub struct DrawList;
impl DrawList {
    pub fn push_solid(&mut self, rect: [f32; 4], color: Rgba) {
        println!("{rect:?} {color:?}");
    }
}
${production}
fn main() {
    let mut draw = DrawList;
    let theme = Theme { stroke_hairline: 1.0, border_normal: [0, 0, 0, 255] };
    push_chrome_row(&mut draw, Rect { x: 0., y: 0., w: 3., h: 3. }, None, false, &theme);
    push_chrome_row(&mut draw, Rect { x: 0., y: 3., w: 3., h: 3. }, Some([255, 0, 0, 255]), true, &theme);
    push_chrome_row(&mut draw, Rect { x: 0., y: 6., w: 3., h: 3. }, Some([0, 255, 0, 255]), true, &theme);
}
`);
execFileSync("rustc", ["--edition=2021", program, "-o", binary], { cwd: root, stdio: "inherit" });
const draws = execFileSync(binary, { encoding: "utf8" }).trim().split("\n").map(line => JSON.parse(`[${line.replace("] [", "], [")}]`));
const canvas = createCanvas(3, 9);
const context = canvas.getContext("2d");
for (const [box, [r, g, b, a]] of draws) {
  context.fillStyle = `rgba(${r}, ${g}, ${b}, ${a / 255})`;
  context.fillRect(...box);
}
const expected = [
  [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
  [0, 0, 0, 255], [255, 0, 0, 255], [255, 0, 0, 255],
  [0, 0, 0, 255], [0, 255, 0, 255], [0, 255, 0, 255],
];
assert.equal(draws.length, 4);
assert.deepEqual([...context.getImageData(0, 0, 3, 9).data], expected.flatMap(pixel => [...pixel, ...pixel, ...pixel]));
console.log("Rust row draw order matches the raster for resting, selected, and hovered rows");
