import { createCanvas, loadImage } from "@napi-rs/canvas";
const path =
  "C:/Users/Kinosh/.cursor/projects/e-semio/assets/c__Users_Kinosh_AppData_Roaming_Cursor_User_workspaceStorage_empty-window_images_image-03be6b14-2a67-429d-85ac-2fef38c699f2.png";
const img = await loadImage(path);
const c = createCanvas(img.width, img.height);
c.getContext("2d").drawImage(img, 0, 0);
const { width: W, height: H } = c;
const d = c.getContext("2d").getImageData(0, 0, W, H).data;
const lum = (x, y) => {
  const i = (y * W + x) * 4;
  return 0.2126 * d[i] + 0.7152 * d[i + 1] + 0.0722 * d[i + 2];
};
for (const y of [128, 129, 130, 131, 132, 133, 134, 135, 150, 180, 200, 280, 284]) {
  const row = [];
  for (let x = 0; x < 45; x++) row.push(Math.round(lum(x, y)));
  console.log("y" + y, row.join(" "));
}
