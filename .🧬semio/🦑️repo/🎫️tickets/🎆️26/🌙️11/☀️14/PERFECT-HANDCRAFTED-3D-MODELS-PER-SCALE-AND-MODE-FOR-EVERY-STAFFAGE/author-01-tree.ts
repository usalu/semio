import { draftBody } from "./volumes-draft.ts";
import { simplifyRing } from "./volumes-base.ts";
import { Author, type V3, type Json } from "./author-01-kit.ts";

export type TreeCfg = {
  slug: string;
  woody: (b: Json) => boolean;
  foliage: (b: Json) => boolean;
  pick: Record<string, (s: Json) => boolean>;
  memberR: (b: Json, scale: number) => [number, number];
  emitterD: number;
  hidden?: (a: Author) => void;
  notes: Record<number, [string, string][]>;
  tweak?: (a: Author) => void;
  regionBones?: Record<string, RegExp>;
  kind?: string;
  kindDe?: string;
  fallback?: (b: Json, id: string) => { material: string; tone: number };
  foliageExp?: number;
  trunkLimit?: number;
};

const boneWorldDir = (a: Author, bone: string): V3 => a.fr[bone]!.u;

export function treeBody(a: Author, cfg: TreeCfg) {
  a.into("body");
  if (cfg.tweak && (cfg as Json).pre?.(a)) return;
  const f = a.feature;
  for (const b of a.asset.rig.bones) {
    const E = a.shapesOf("elevation", b.id), F = a.shapesOf("front", b.id);
    if (E.length === 0) continue;
    const base = (() => { try { return a.meta(E[0]!); } catch { const q = cfg.fallback!(b, E[0]!); return a.mat(q.material, q.tone); } })();
    if (cfg.foliage(b)) {
      const n = a.scale <= 50 ? 9 : a.scale <= 100 ? 7 : a.scale <= 200 ? 5 : 4;
      a.loftFromViews(`${b.id}-mass`, b.id, { elev: E, front: F.length ? F : undefined, n, exp: cfg.foliageExp ?? 2.4, pinch: 0.25, m: base });
    } else if (cfg.woody(b)) {
      const eb = a.box2("elevation", E);
      const w = eb.maxX - eb.minX, h = eb.maxY - eb.minY;
      const u = boneWorldDir(a, b.id);
      const vertical = Math.abs(u[1]) > 0.8;
      if (vertical && F.length) a.loftFromViews(b.id, b.id, { elev: E, front: F, n: Math.max(3, Math.round(h / 1.5)), exp: 2.1, pinch: 1, m: base, range: [eb.minY, eb.maxY] });
      else {
        const o = a.origin(b.id), t = a.tip(b.id);
        const area = a.area("elevation", E);
        const r = Math.max(area / Math.max(Math.hypot(t[0] - o[0], t[1] - o[1]), 1e-3) / 2, f * 0.6);
        a.limbs([{ bone: b.id, r0: r * 1.2, r1: r * 0.8, meta: base }]);
      }
      void w;
    }
  }
}

export function treeFinish(a: Author, cfg: TreeCfg) {
  const sc = a.scale, f = a.feature;
  for (const [region, pick] of Object.entries(cfg.pick)) {
    const reg = a.asset.thermal.regions.find((r: Json) => r.id === region);
    if (reg?.hidden) continue;
    if (a.lists.body.some(pick)) a.thermalRegion(region, pick, "ref");
    else { const any = a.lists.body.filter((s) => s.material === reg.material); if (any.length) a.thermalRegion(region, (s) => s.material === reg.material, "ref"); else {
      const re = cfg.regionBones?.[region];
      if (!re) throw new Error(`no body solids for ${region} at ${sc}`);
      a.into("thermal", { region });
      for (const b of a.asset.rig.bones.filter((x: Json) => re.test(x.id))) { const o = a.origin(b.id), t = a.tip(b.id); const m = a.mat(reg.material, 0.4); if (reg.kind === "surface") a.ell(`th-${region}-${b.id}`, b.id, [(o[0] + t[0]) / 2, (o[1] + t[1]) / 2, (o[2] + t[2]) / 2], [f * 3, f * 3, f * 3], m); else a.cap(`th-${region}-${b.id}`, b.id, o, t, Math.max(f * 0.6, 0.02), Math.max(f * 0.6, 0.015), m); }
    } }
  }
  cfg.hidden?.(a);
  for (const em of a.asset.thermal.emitters) a.emitter(em.id, { maxD: cfg.emitterD * (1 + Math.log2(sc / 50) * 0.2), region: em.region });
  const sec = (b: Json): [number, number] => (b.section?.areaM2 > 0 ? [Math.sqrt(b.section.areaM2 / Math.PI), Math.sqrt(b.section.areaM2 / Math.PI) * 0.85] : cfg.memberR(b, sc));
  a.autoMembers(sec, { t0: 0.04, t1: 0.96 });
  if (sc === 50) a.autoJoints((b) => Math.max(0.5 * (a.bone(b.parent!)?.section ? Math.sqrt(a.bone(b.parent!).section.areaM2 / Math.PI) : 0.05), f), { only: (b) => !/^fol/.test(b.id) });
  for (const id of a.asset.structure.supports) a.support(id, "block", { size: [Math.max(a.asset.anchors.find((x: Json) => x.id === id).footprintLengthM ?? 0.3, f), Math.max(f * 1.2, 0.08), Math.max(a.asset.anchors.find((x: Json) => x.id === id).footprintWidthM ?? 0.3, f)] });
  a.fitMembers(24);
  if (!(cfg.notes[sc]?.length)) {
    const nb = a.lists.body.length, nt = a.lists.thermal.length, ns = a.lists.structure.length;
    const lofts = a.lists.body.filter((x) => x.kind === "loft").length, ext = a.lists.body.filter((x) => x.kind === "extrude").length;
    a.note(`Body of 1:${sc} (${nb} solids: ${lofts} lofts, ${ext} polygon extrusions, the rest capsules and ellipsoids) is built against this scale's own elevation, front and plan drawings: ${cfg.kind ?? "woody parts are lofts or tapered capsules along their bones, foliage masses are superelliptic lofts read slice by slice from the outlines"}; parts below the ${(a.feature * 1000).toFixed(1)} mm feature are merged into their neighbours.`, `Körper von 1:${sc} (${nb} Körper: ${lofts} Lofts, ${ext} Polygon-Extrusionen, der Rest Kapseln und Ellipsoide) ist gegen die eigenen Aufriss-, Front- und Grundrisszeichnungen dieses Maßstabs gebaut: ${cfg.kindDe ?? "Holzteile sind Lofts oder verjüngte Kapseln entlang ihrer Knochen, Laubmassen sind superelliptische Lofts, Schnitt für Schnitt aus den Konturen gelesen"}; Teile unter dem Merkmal von ${(a.feature * 1000).toFixed(1)} mm sind mit ihren Nachbarn verschmolzen.`);
    a.note(`Thermal (${nt} solids): every region references the exposed body solids of its material (bark, leaf sun and shade, fruit, twig); where a coarser scale has no solid of a region a thin own solid on the carrying bone stands in; emitters are flat discs of their own diameter inside the crown; areaM2 of regions whose authored value differs from the modelled exposed surface was corrected at the root (ruling C1).`, `Thermisch (${nt} Körper): jede Region verweist auf die freien Körper ihres Materials (Rinde, Sonnen- und Schattenlaub, Frucht, Zweig); wo ein gröberer Maßstab keinen Körper einer Region hat, steht ein dünner eigener Körper am tragenden Knochen; Emitter sind flache Scheiben ihres eigenen Durchmessers in der Krone; areaM2 von Regionen, deren angegebener Wert von der modellierten freien Fläche abweicht, wurde an der Wurzel korrigiert (Regel C1).`);
    a.note(`Structure (${ns} solids): a member capsule along every bone of the structural view with the bone's section (else an estimated radius), placed inside the body silhouette, joint balls at the joints of 1:50, and a support block at the support anchor.`, `Tragwerk (${ns} Körper): eine Stabkapsel entlang jedem Knochen der Tragwerksansicht mit dem Querschnitt des Knochens (sonst geschätzter Radius), innerhalb der Körpersilhouette, Gelenkkugeln an den Gelenken von 1:50 und ein Auflagerblock am Auflagerankerpunkt.`);
  }
  for (const [en, de] of cfg.notes[sc] ?? []) a.note(en, de);
  void f;
}

export function draftBase(a: Author, simplifyFrom = 50) {
  const { solids } = draftBody(a.asset, a.scale);
  for (const sol of solids) {
    const c = JSON.parse(JSON.stringify(sol));
    if (c.kind === "extrude" && a.scale > simplifyFrom) {
      const ring = simplifyRing(c.profile.map(([x, y]: number[]) => ({ x, y })), a.feature * 0.8);
      if (ring.length >= 3) c.profile = ring.map((p) => [Math.round(p.x * 1e5) / 1e5, Math.round(p.y * 1e5) / 1e5]);
    }
    if (c.kind === "extrude" && c.from !== undefined) { const h = Math.max(Math.abs(c.from), Math.abs(c.to), a.feature * 0.6); c.from = -h; c.to = h; }
    a.lists.body.push(c);
  }
}
