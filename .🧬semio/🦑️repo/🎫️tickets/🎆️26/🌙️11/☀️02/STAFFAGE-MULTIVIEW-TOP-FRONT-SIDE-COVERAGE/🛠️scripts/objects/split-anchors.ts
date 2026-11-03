import { fileOf, patchAnchors, patchArray } from "./patch.ts";

type Foot = { id: string; x?: number; y?: number; z: number; title?: { en: string; de: string }; fpL?: number; fpW?: number };
const f = (id: string, z: number, extra: Partial<Foot> = {}): Foot => ({ id, z, ...extra });
const T = (en: string, de: string) => ({ en, de });

const plan: [string, string, Record<string, Foot[]>][] = [
  ["🪑️objects", "charcoal-grill", {
    "leg-left": [f("leg-rear-left", 0.145, { x: -0.275, title: T("Rear left foot", "Hinterer linker Fuß") }), f("leg-rear-right", -0.145, { x: -0.275, title: T("Rear right foot", "Hinterer rechter Fuß") })],
    "leg-right": [f("wheel-left", 0.14, { x: 0.27, fpL: 0.03, fpW: 0.02, title: T("Left wheel", "Linkes Rad") }), f("wheel-right", -0.14, { x: 0.27, fpL: 0.03, fpW: 0.02, title: T("Right wheel", "Rechtes Rad") })],
  }],
  ["🪑️objects", "playground-swing", {
    "pin-left": [f("pin-left-l", 0.55, { fpL: 0.18, fpW: 0.1, title: T("Left A-frame foot, pin end, left side", "Fuß des A-Rahmens, Festlager, linke Seite") }), f("pin-left-r", -0.55, { fpL: 0.18, fpW: 0.1, title: T("Left A-frame foot, pin end, right side", "Fuß des A-Rahmens, Festlager, rechte Seite") })],
    "roller-right": [f("roller-right-l", 0.55, { fpL: 0.18, fpW: 0.1, title: T("A-frame foot, roller end, left side", "Fuß des A-Rahmens, Rollenlager, linke Seite") }), f("roller-right-r", -0.55, { fpL: 0.18, fpW: 0.1, title: T("A-frame foot, roller end, right side", "Fuß des A-Rahmens, Rollenlager, rechte Seite") })],
  }],
  ["🪑️objects", "shopping-trolley", {
    "wheel-rear": [f("wheel-rear-left", 0.22, { x: -0.295, title: T("Rear left wheel", "Hinteres linkes Rad") }), f("wheel-rear-right", -0.22, { x: -0.295, title: T("Rear right wheel", "Hinteres rechtes Rad") })],
    "wheel-front": [f("wheel-front-left", 0.22, { x: 0.33, title: T("Front left wheel", "Vorderes linkes Rad") }), f("wheel-front-right", -0.22, { x: 0.33, title: T("Front right wheel", "Vorderes rechtes Rad") })],
  }],
  ["🪑️objects", "stepladder", {
    "foot-front": [f("foot-front-left", 0.2, { x: -0.365, fpL: 0.08, fpW: 0.06, title: T("Front left foot", "Vorderer linker Fuß") }), f("foot-front-right", -0.2, { x: -0.365, fpL: 0.08, fpW: 0.06, title: T("Front right foot", "Vorderer rechter Fuß") })],
    "foot-rear": [f("foot-rear-left", 0.175, { fpL: 0.09, fpW: 0.05, title: T("Rear left foot", "Hinterer linker Fuß") }), f("foot-rear-right", -0.175, { fpL: 0.09, fpW: 0.05, title: T("Rear right foot", "Hinterer rechter Fuß") })],
  }],
  ["🪑️objects", "waste-bin", {
    "foot-left": [f("foot-rear-left", 0.175, { x: -0.175, fpL: 0.05, fpW: 0.05, title: T("Rear left pedestal foot", "Hinterer linker Sockelfuß") }), f("foot-rear-right", -0.175, { x: -0.175, fpL: 0.05, fpW: 0.05, title: T("Rear right pedestal foot", "Hinterer rechter Sockelfuß") })],
    "foot-right": [f("foot-front-left", 0.175, { x: 0.175, fpL: 0.05, fpW: 0.05, title: T("Front left pedestal foot", "Vorderer linker Sockelfuß") }), f("foot-front-right", -0.175, { x: 0.175, fpL: 0.05, fpW: 0.05, title: T("Front right pedestal foot", "Vorderer rechter Sockelfuß") })],
  }],
  ["🛋️furniture", "bar-stool", {
    "foot-left": [f("foot-xn", 0, { title: T("Foot on the rear axis", "Fuß auf der hinteren Achse") })],
    "foot-right": [f("foot-xp", 0, { title: T("Foot on the front axis", "Fuß auf der vorderen Achse") })],
    "foot-mid": [f("foot-zp", 0.21, { x: 0, fpW: 0.03, title: T("Foot on the left axis", "Fuß auf der linken Achse") }), f("foot-zn", -0.21, { x: 0, fpW: 0.03, title: T("Foot on the right axis", "Fuß auf der rechten Achse") })],
  }],
  ["🛋️furniture", "cafe-chair", {
    "contact-rear": [f("contact-rear-left", 0.23, { title: T("Rear left leg", "Hinteres linkes Bein") }), f("contact-rear-right", -0.23, { title: T("Rear right leg", "Hinteres rechtes Bein") })],
    "contact-front": [f("contact-front-left", 0.245, { title: T("Front left leg", "Vorderes linkes Bein") }), f("contact-front-right", -0.245, { title: T("Front right leg", "Vorderes rechtes Bein") })],
  }],
  ["🛋️furniture", "dining-table", {
    "leg-left": [f("leg-left-l", 0.375, { fpL: 0.045, fpW: 0.05, title: T("Left end, left leg", "Linkes Ende, linkes Bein") }), f("leg-left-r", -0.375, { fpL: 0.045, fpW: 0.05, title: T("Left end, right leg", "Linkes Ende, rechtes Bein") })],
    "leg-right": [f("leg-right-l", 0.375, { fpL: 0.045, fpW: 0.05, title: T("Right end, left leg", "Rechtes Ende, linkes Bein") }), f("leg-right-r", -0.375, { fpL: 0.045, fpW: 0.05, title: T("Right end, right leg", "Rechtes Ende, rechtes Bein") })],
  }],
  ["🛋️furniture", "folding-deck-chair", {
    "foot-rear": [f("foot-rear-left", 0.27, { fpL: 0.05, fpW: 0.045, title: T("Rear left foot", "Hinterer linker Fuß") }), f("foot-rear-right", -0.27, { fpL: 0.05, fpW: 0.045, title: T("Rear right foot", "Hinterer rechter Fuß") })],
    "foot-front": [f("foot-front-left", 0.235, { fpL: 0.05, fpW: 0.045, title: T("Front left foot", "Vorderer linker Fuß") }), f("foot-front-right", -0.235, { fpL: 0.05, fpW: 0.045, title: T("Front right foot", "Vorderer rechter Fuß") })],
  }],
  ["🛋️furniture", "operating-table", {
    "castors-table": [f("castor-table-rear-left", 0.17, { x: -0.32, fpL: 0.04, fpW: 0.03, title: T("Table castor, rear left", "Tischrolle hinten links") }), f("castor-table-rear-right", -0.17, { x: -0.32, fpL: 0.04, fpW: 0.03, title: T("Table castor, rear right", "Tischrolle hinten rechts") }), f("castor-table-front-left", 0.17, { x: 0.32, fpL: 0.04, fpW: 0.03, title: T("Table castor, front left", "Tischrolle vorn links") }), f("castor-table-front-right", -0.17, { x: 0.32, fpL: 0.04, fpW: 0.03, title: T("Table castor, front right", "Tischrolle vorn rechts") })],
    "castors-stand": [f("castor-stand-rear-left", 0.195, { x: -0.22, fpL: 0.04, fpW: 0.03, title: T("Stand castor, rear left", "Ständerrolle hinten links") }), f("castor-stand-rear-right", -0.195, { x: -0.22, fpL: 0.04, fpW: 0.03, title: T("Stand castor, rear right", "Ständerrolle hinten rechts") }), f("castor-stand-front-left", 0.195, { x: 0.22, fpL: 0.04, fpW: 0.03, title: T("Stand castor, front left", "Ständerrolle vorn links") }), f("castor-stand-front-right", -0.195, { x: 0.22, fpL: 0.04, fpW: 0.03, title: T("Stand castor, front right", "Ständerrolle vorn rechts") })],
  }],
  ["🛋️furniture", "picnic-table", {
    "foot-left": [f("foot-left-l", 0.8, { fpL: 0.09, fpW: 0.05, title: T("Left trestle, left foot", "Linker Bock, linker Fuß") }), f("foot-left-r", -0.8, { fpL: 0.09, fpW: 0.05, title: T("Left trestle, right foot", "Linker Bock, rechter Fuß") })],
    "foot-right": [f("foot-right-l", 0.8, { fpL: 0.09, fpW: 0.05, title: T("Right trestle, left foot", "Rechter Bock, linker Fuß") }), f("foot-right-r", -0.8, { fpL: 0.09, fpW: 0.05, title: T("Right trestle, right foot", "Rechter Bock, rechter Fuß") })],
  }],
  ["🛋️furniture", "single-bed", {
    "foot-foot": [f("foot-foot-l", 0.42, { fpL: 0.06, fpW: 0.06, title: T("Foot post, left", "Fußpfosten links") }), f("foot-foot-r", -0.42, { fpL: 0.06, fpW: 0.06, title: T("Foot post, right", "Fußpfosten rechts") })],
  }],
  ["🛋️furniture", "three-seat-sofa", {
    "foot-left": [f("foot-left-l", 0.36, { x: 0.12, fpL: 0.07, fpW: 0.07, title: T("Left end, left foot", "Linkes Ende, linker Fuß") }), f("foot-left-r", -0.36, { x: 0.12, fpL: 0.07, fpW: 0.07, title: T("Left end, right foot", "Linkes Ende, rechter Fuß") })],
    "foot-right": [f("foot-right-l", 0.36, { x: 1.88, fpL: 0.07, fpW: 0.07, title: T("Right end, left foot", "Rechtes Ende, linker Fuß") }), f("foot-right-r", -0.36, { x: 1.88, fpL: 0.07, fpW: 0.07, title: T("Right end, right foot", "Rechtes Ende, rechter Fuß") })],
  }],
];

for (const [cat, slug, map] of plan) {
  const file = fileOf(cat, slug);
  const expand = (id: string): string[] => (map[id] ? map[id]!.map((x) => x.id) : [id]);
  patchAnchors(file, (list) =>
    list.flatMap((a) => {
      const feet = map[a.id];
      if (!feet) return [a];
      return feet.map((ft) => {
        const out: any = { ...a, id: ft.id, z: ft.z };
        if (ft.x !== undefined) out.x = ft.x;
        if (ft.y !== undefined) out.y = ft.y;
        if (ft.title) out.title = ft.title;
        if (ft.fpL !== undefined) out.footprintLengthM = ft.fpL;
        if (ft.fpW !== undefined) out.footprintWidthM = ft.fpW;
        return out;
      });
    }),
  );
  patchArray(file, '"poses"', /\n( *)"poses": \[/, (poses) => poses.map((p: any) => ({ ...p, contacts: [...new Set((p.contacts ?? []).flatMap(expand))] })));
  patchArray(file, '"clips"', /\n( *)"clips": \[/, (clips) =>
    clips.map((c: any) => (c.contacts ? { ...c, contacts: c.contacts.flatMap((k: any) => expand(k.anchor).map((id) => ({ ...k, anchor: id }))) } : c)),
  );
  patchArray(file, '"structure"', /\n( *)"supports": \[/, (s) => [...new Set((s as string[]).flatMap(expand))]);
}
