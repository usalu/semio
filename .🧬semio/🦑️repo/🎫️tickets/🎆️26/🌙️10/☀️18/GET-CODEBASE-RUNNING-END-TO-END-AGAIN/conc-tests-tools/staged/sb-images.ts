import { spawnSync } from "node:child_process";
import { readdirSync, realpathSync } from "node:fs";
import { join, resolve, sep } from "node:path";

/** 🧲️ Lists the attached macOS disk images through the system tool, deepest backing file first. */
export function attachedDiskImagesV1(): readonly Readonly<{ image: string; target: string }>[] {
  if (process.platform !== "darwin") return [];
  const info = spawnSync("/usr/bin/hdiutil", ["info", "-plist"], { encoding: "utf8" });
  if (info.status !== 0) return [];
  const json = spawnSync("/usr/bin/plutil", ["-convert", "json", "-o", "-", "-"], { input: info.stdout, encoding: "utf8" });
  if (json.status !== 0) return [];
  const rows = (JSON.parse(json.stdout) as { images?: { "image-path"?: string; "system-entities"?: { "mount-point"?: string; "dev-entry"?: string }[] }[] }).images ?? [];
  return rows.flatMap((row) => {
    const entities = row["system-entities"] ?? [], target = entities.find((entity) => entity["mount-point"])?.["mount-point"] ?? entities[0]?.["dev-entry"];
    if (!row["image-path"] || !target) return [];
    let image = row["image-path"];
    try { image = realpathSync(image); } catch {}
    return [{ image, target }];
  }).sort((left, right) => right.image.length - left.image.length);
}

/** 🔎️ Tells whether a tree may hold a disk image file; a tree too large to walk within the bound counts as holding one, so the kernel roster decides. */
export function mayHoldDiskImageV1(root: string, bound = 20000): boolean {
  const pending = [root];
  let seen = 0;
  while (pending.length) {
    const directory = pending.pop()!;
    let names: import("node:fs").Dirent[];
    try { names = readdirSync(directory, { withFileTypes: true }); } catch { continue; }
    for (const entry of names) {
      if (++seen > bound) return true;
      if (/\.(dmg|sparseimage|sparsebundle|img|cdr|iso)$/iu.test(entry.name)) return true;
      if (entry.isDirectory()) pending.push(join(directory, entry.name));
    }
  }
  return false;
}

/** 🧲️ Detaches every disk image whose backing file lives under the root, nested images first. */
export function detachDiskImagesUnderV1(root: string): readonly string[] {
  let real = resolve(root);
  try { real = realpathSync(real); } catch {}
  if (!mayHoldDiskImageV1(real)) return [];
  const prefix = real + sep, detached: string[] = [];
  for (const { image, target } of attachedDiskImagesV1()) {
    if (!image.startsWith(prefix)) continue;
    const plain = spawnSync("/usr/bin/hdiutil", ["detach", target], { encoding: "utf8" });
    if (plain.status === 0 || spawnSync("/usr/bin/hdiutil", ["detach", "-force", target], { encoding: "utf8" }).status === 0) detached.push(image);
  }
  return detached;
}
