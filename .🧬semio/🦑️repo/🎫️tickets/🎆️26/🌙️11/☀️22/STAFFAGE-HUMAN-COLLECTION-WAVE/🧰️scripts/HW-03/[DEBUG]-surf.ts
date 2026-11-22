import { loftSurface } from "./geo.ts";
const a = (await Bun.file("../../🗑️generated/HW-03/doctor.json").json()).asset;
for (const s of ["500","200","100","50"]) {
  const coat = a.volumes[s].body.find((x: any) => x.id === "coat");
  const row = [];
  for (const u of [0.10, 0.15, 0.2, 0.25, 0.28, 0.3, 0.31]) row.push(`${u}: f${loftSurface(coat, u, -0.08, -1)?.toFixed(3)} b${loftSurface(coat, u, -0.08, 1)?.toFixed(3)} f0 ${loftSurface(coat, u, 0, -1)?.toFixed(3)}`);
  console.log(s, row.join(" | "));
}
