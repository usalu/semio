/** 🧷️ The reproducible multiview repair of an asset of batch MC, the last step of its chain: applies the repair ops `ops/<id>.json` (the ops of the silhouette rounds plus `anchor-add`, an anchor inserted after another or replaced when its id exists, and `note`, a sentence appended to both languages of a decision text of a band unless it already holds it, and `contact-split`, a lumped contact anchor replaced by one anchor per physical contact (D45) in the anchors, the contacts of poses and clips, the structural supports and the support pads of the volumes, where a pad repeated once per contact becomes one pad per contact, or one box pad per contact at the forward position and length that `pads` gives) to the catalog document and writes it back in its own indentation. Every op is idempotent; the file is read right before it is written.
 *  bun apply.ts <id>… [--in <file>] [--out <file>]
 * @see ../../🧰️silhouette/R2-O/ops.ts — the ops */
import { parseArgs } from "node:util";
import { applyOps, type Op } from "../../🧰️silhouette/R2-O/ops.ts";
import { catalogFile, opsFile } from "./files.ts";

type AnchorAdd = { readonly op: "anchor-add"; readonly after?: string; readonly anchor: { readonly id: string } };
type ContactSplit = { readonly op: "contact-split"; readonly anchor: string; readonly parts: readonly { readonly id: string }[]; readonly pads?: readonly { readonly x: number; readonly lengthM: number }[] };
type Note = { readonly op: "note"; readonly scale: string; readonly decision: string; readonly en: string; readonly de: string };

/** 🦶️ Replaces a lumped contact anchor by its parts wherever the asset names it; a second run finds the lumped anchor gone and only refreshes the parts. */
function splitContact(asset: any, o: ContactSplit): void {
  const ids = o.parts.map((part) => part.id);
  const anchors = asset.anchors as { id: string }[];
  const at = anchors.findIndex((x) => x.id === o.anchor);
  if (at >= 0) anchors.splice(at, 1, ...o.parts);
  else for (const part of o.parts) anchors[anchors.findIndex((x) => x.id === part.id)] = part;
  const spread = (list: string[]): string[] => list.flatMap((id) => (id === o.anchor ? ids : [id]));
  for (const pose of asset.poses) if (pose.contacts !== undefined) pose.contacts = spread(pose.contacts);
  for (const clip of asset.clips) if (clip.contacts !== undefined) clip.contacts = clip.contacts.flatMap((c: { anchor: string }) => (c.anchor === o.anchor ? ids.map((id) => ({ ...c, anchor: id })) : [c]));
  asset.structure.supports = spread(asset.structure.supports);
  for (const volume of Object.values(asset.volumes) as any[]) {
    if (volume.structure === undefined) continue;
    volume.structure = volume.structure.flatMap((solid: any) => {
      if (solid.anchor !== o.anchor) return [solid];
      const { repeat, ...pad } = solid;
      const base = pad.transform?.translation ?? [0, 0, 0];
      if (o.pads !== undefined) return ids.map((id, i) => ({ ...pad, id: `${id}-pad`, size: [o.pads![i]!.lengthM, pad.size[1], pad.size[2]], transform: { ...pad.transform, translation: [o.pads![i]!.x, base[1], base[2]] }, anchor: id }));
      if (repeat?.count !== ids.length) throw new Error(`pad "${solid.id}" is not repeated once per contact`);
      return ids.map((id, i) => ({ ...pad, id: `${id}-pad`, transform: { ...pad.transform, translation: base.map((v: number, k: number) => Math.round((v + i * repeat.translation[k]) * 1e6) / 1e6) }, anchor: id }));
    });
  }
}

/** ⚓️ Applies the ops of a list in order, `anchor-add` here and every other through the silhouette ops. */
export function applyAll(asset: any, ops: readonly (Op | AnchorAdd | Note | ContactSplit)[]): string[] {
  const skipped: string[] = [];
  for (const o of ops) {
    if (o.op === "note") {
      const note = o as Note;
      const text = asset.representations[note.scale].decisions[note.decision] as Record<"en" | "de", string>;
      for (const language of ["en", "de"] as const) if (!text[language].includes(note[language])) text[language] = `${text[language]} ${note[language]}`;
      continue;
    }
    if (o.op === "contact-split") {
      splitContact(asset, o as ContactSplit);
      continue;
    }
    if (o.op !== "anchor-add") { skipped.push(...applyOps(asset, [o as Op])); continue; }
    const { anchor, after } = o as AnchorAdd;
    const list = asset.anchors as { id: string }[];
    const at = list.findIndex((x) => x.id === anchor.id);
    if (at >= 0) list[at] = anchor;
    else list.splice(after === undefined ? list.length : list.findIndex((x) => x.id === after) + 1, 0, anchor);
  }
  return skipped;
}

if (import.meta.main) {
  const { values, positionals } = parseArgs({ allowPositionals: true, options: { in: { type: "string" }, out: { type: "string" } } });
  for (const slug of positionals) {
    const ops = JSON.parse(await Bun.file(opsFile(slug)).text());
    const text = await Bun.file(values.in ?? catalogFile(slug)).text();
    const second = text.split("\n", 2)[1] ?? "";
    const indent = second.length - second.trimStart().length;
    const doc = JSON.parse(text);
    const skipped = applyAll(doc.asset, ops);
    const next = JSON.stringify(doc, null, indent) + "\n";
    if (next !== text || values.out !== undefined) await Bun.write(values.out ?? catalogFile(slug), next);
    console.log(`${slug}: ${ops.length} ops applied${skipped.length > 0 ? `, ${skipped.length} already in place` : ""}${next === text ? " (file unchanged)" : ""}`);
  }
}
