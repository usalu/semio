import { writeFileSync } from "node:fs";
import { createCanvas, loadImage } from "@napi-rs/canvas";

const ticket = ".repo/🎫/26/08/04/FIX-WINDOW-SHORT-TABLE-GEOMETRY";
const paths = {
  pk1: "C:/Users/Kinosh/.cursor/projects/e-semio/assets/c__Users_Kinosh_AppData_Roaming_Cursor_User_workspaceStorage_empty-window_images_image-6704b037-fb89-4f08-a960-6765dccf3f18.png",
  bbma: "C:/Users/Kinosh/.cursor/projects/e-semio/assets/c__Users_Kinosh_AppData_Roaming_Cursor_User_workspaceStorage_empty-window_images_image-03be6b14-2a67-429d-85ac-2fef38c699f2.png",
};

function lum(d, W, x, y) {
  const i = (y * W + x) * 4;
  return 0.2126 * d[i] + 0.7152 * d[i + 1] + 0.0722 * d[i + 2];
}

async function analyze(key, path) {
  const img = await loadImage(path);
  const c = createCanvas(img.width, img.height);
  const ctx = c.getContext("2d");
  ctx.drawImage(img, 0, 0);
  const { width: W, height: H } = c;
  const d = ctx.getImageData(0, 0, W, H).data;

  // Sample bg (corners)
  let bg = 0,
    n = 0;
  for (const [x, y] of [
    [2, 2],
    [W - 3, 2],
    [2, H - 3],
  ]) {
    bg += lum(d, W, x, y);
    n++;
  }
  bg /= n;
  // borders are brighter than dark bg
  const isInk = (x, y) => lum(d, W, x, y) > bg + 35;

  // For each y, find leftmost ink x in left third
  const leftEdge = [];
  for (let y = 0; y < H; y++) {
    let x = -1;
    for (let xi = 0; xi < Math.floor(W * 0.45); xi++) {
      if (isInk(xi, y)) {
        x = xi;
        break;
      }
    }
    leftEdge.push(x);
  }

  // Collapse into runs with stable x
  const runs = [];
  for (let y = 0; y < H; y++) {
    const x = leftEdge[y];
    if (x < 0) continue;
    const last = runs[runs.length - 1];
    if (last && y === last.y1 + 1 && Math.abs(x - last.xMed) <= 2) {
      last.y1 = y;
      last.xs.push(x);
      last.xMed = last.xs[Math.floor(last.xs.length / 2)];
    } else {
      runs.push({ y0: y, y1: y, xs: [x], xMed: x });
    }
  }
  const substantial = runs.filter((r) => r.y1 - r.y0 >= 8);
  console.log(
    key,
    JSON.stringify({
      W,
      H,
      bg: +bg.toFixed(1),
      runs: substantial.map((r) => ({
        y0: r.y0,
        y1: r.y1,
        h: r.y1 - r.y0 + 1,
        xMed: r.xMed,
        xMin: Math.min(...r.xs),
        xMax: Math.max(...r.xs),
      })),
    }),
  );

  // Annotate
  const out = createCanvas(W, H);
  const o = out.getContext("2d");
  o.drawImage(c, 0, 0);
  for (const r of substantial) {
    o.strokeStyle = "rgba(255,0,0,0.9)";
    o.beginPath();
    o.moveTo(r.xMed + 0.5, r.y0);
    o.lineTo(r.xMed + 0.5, r.y1);
    o.stroke();
  }
  writeFileSync(`${ticket}/user-${key}-ann.png`, out.toBuffer("image/png"));

  // Zoom left strip
  const z = 4;
  const crop = createCanvas(80 * z, H * z);
  const cz = crop.getContext("2d");
  cz.imageSmoothingEnabled = false;
  cz.drawImage(c, 0, 0, 80, H, 0, 0, 80 * z, H * z);
  writeFileSync(`${ticket}/user-${key}-zoomL.png`, crop.toBuffer("image/png"));
}

await analyze("pk1", paths.pk1);
await analyze("bbma", paths.bbma);
