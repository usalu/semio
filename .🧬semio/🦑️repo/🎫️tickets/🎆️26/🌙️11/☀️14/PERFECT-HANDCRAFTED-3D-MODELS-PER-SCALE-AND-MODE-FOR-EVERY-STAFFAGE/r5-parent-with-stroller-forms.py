"""💇 Round-5 hand-stated form changes of `humans/parent-with-stroller` written into the asset's own solid scripts (`r3-stroller-s50-figure.ts`, `r3-stroller-coarse.ts`, `r3-stroller-s50-structure.ts`): every replaced row list is stated here in full, old text -> new text; the files are re-read right before writing and a missing old text is reported, not guessed.

Usage: python3 r5-parent-with-stroller-forms.py
"""
import os

T = os.path.dirname(os.path.abspath(__file__))


EDITS = {
    "r3-stroller-s50-figure.ts": [
        ('  out.push(tag(P("hair-back", "head", "hair", HAIR, [[0.175, 0.098, 0, 0.07, 0.022, 2.4], [0.12, 0.094, 0, 0.086, 0.028, 2.4], [0.06, 0.086, 0, 0.094, 0.026, 2.4], [0.01, 0.086, 0, 0.096, 0.02, 2.4], [-0.03, 0.09, 0, 0.088, 0.013, 2.4], [-0.045, 0.092, 0, 0.07, 0.007, 2.4]]), "hair"));\n  both(P("hair-side-l", "head", "hair", HAIR, [[0.17, 0.04, 0.078, 0.012, 0.03, 2.2], [0.12, 0.05, 0.082, 0.014, 0.034, 2.2], [0.06, 0.052, 0.084, 0.015, 0.034, 2.2], [0.01, 0.05, 0.09, 0.014, 0.03, 2.2], [-0.025, 0.046, 0.094, 0.01, 0.018, 2.2]]), "hair");\n',
         '  out.push(tag(L("hair-fall", "head", "hair", HAIR, dense([[0.19, 0.088, 0.07, 0.03, 0, 2.2], [0.15, 0.09, 0.068, 0.038, 0, 2.3], [0.11, 0.088, 0.062, 0.044, 0, 2.3], [0.07, 0.084, 0.054, 0.05, 0, 2.3], [0.03, 0.08, 0.046, 0.056, 0, 2.3], [-0.005, 0.077, 0.038, 0.064, 0, 2.2], [-0.03, 0.07, 0.03, 0.07, 0, 2.2], [-0.042, 0.056, 0.022, 0.072, 0, 2.1], [-0.05, 0.03, 0.012, 0.073, 0, 2]], 1)), "hair"));\n'),
        ('[[0.168, -0.082, 0, 0.007, 0.008], [0.148, -0.09, 0, 0.009, 0.012], [0.128, -0.098, 0, 0.013, 0.017], [0.114, -0.098, 0, 0.016, 0.016], [0.104, -0.09, 0, 0.014, 0.008]]',
         '[[0.158, -0.084, 0, 0.0075, 0.007], [0.143, -0.091, 0, 0.0085, 0.011], [0.127, -0.098, 0, 0.0105, 0.015], [0.114, -0.1, 0, 0.0155, 0.014], [0.104, -0.092, 0, 0.0145, 0.008]]'),
        ('const skirt: Row[] = [[-0.075, 0.186, 0.142, 0.004, 0, 2.5], [-0.055, 0.188, 0.144, 0.004, 0, 2.5], [-0.035, 0.19, 0.146, 0.004, 0, 2.5], [0, 0.188, 0.142, 0.004, 0, 2.5], [0.04, 0.17, 0.12, 0, 0, 2.5], [0.125, 0.152, 0.1, -0.004, 0, 2.5]];\n  out.push(tag(L("jacket-skirt", "pelvis", "cotton-clothing", JACKET, quilt(skirt, [-0.05], 0.006)), "torso-clothing"));',
         'const skirt: Row[] = [[-0.075, 0.186, 0.142, 0.004, 0, 2.5], [-0.055, 0.188, 0.144, 0.004, 0, 2.5], [-0.035, 0.19, 0.146, 0.004, 0, 2.5], [0, 0.188, 0.144, 0.002, 0, 2.5], [0.04, 0.184, 0.141, 0, 0, 2.5], [0.08, 0.18, 0.137, -0.003, 0, 2.5], [0.11, 0.177, 0.134, -0.004, 0, 2.5], [0.14, 0.172, 0.124, -0.008, 0, 2.5], [0.175, 0.16, 0.106, -0.014, 0, 2.5]];\n  out.push(tag(L("jacket-skirt", "pelvis", "cotton-clothing", JACKET, quilt(skirt, [-0.05, 0.03], 0.006)), "torso-clothing"));'),
        ('[0.34, 0.064, 0.069, 0.004, 0, 2.2], [0.4, 0.06, 0.066, 0.005, 0, 2.2], [0.4276, 0.059, 0.066, 0.005, 0, 2.2]];',
         '[0.34, 0.064, 0.069, 0.004, 0, 2.2], [0.4, 0.06, 0.066, 0.004, 0, 2.2], [0.4276, 0.059, 0.065, 0.003, 0, 2.2]];'),
        ('const sh: Row[] = [[0, 0.052, 0.058, 0.004, 0, 2.2], [0.05, 0.058, 0.064, 0, 0, 2.2],',
         'const sh: Row[] = [[0, 0.058, 0.064, 0.003, 0, 2.2], [0.05, 0.059, 0.065, 0, 0, 2.2],'),
    ],
    "r3-stroller-coarse.ts": [
        ('  o.push(tag(P("hair-back", "head", "hair", HAIR, [[0.17, 0.09, 0, 0.078, 0.03, 2.6], [0.09, 0.08, 0, 0.098, 0.036, 2.8], [0.005, 0.078, 0, 0.102, 0.03, 2.8], [-0.042, 0.086, 0, 0.086, 0.014, 2.6]]), "hair"));',
         '  o.push(tag(L("hair-fall", "head", "hair", HAIR, [[0.19, 0.088, 0.07, 0.03, 0, 2.2], [0.13, 0.09, 0.066, 0.04, 0, 2.3], [0.06, 0.084, 0.052, 0.052, 0, 2.3], [-0.005, 0.077, 0.038, 0.064, 0, 2.2], [-0.04, 0.062, 0.025, 0.071, 0, 2.1], [-0.05, 0.032, 0.014, 0.073, 0, 2]]), "hair"));'),
        ('[[-0.075, 0.186, 0.142, 0.004, 0, 2.5], [-0.04, 0.19, 0.146, 0.004, 0, 2.5], [0.02, 0.186, 0.14, 0.003, 0, 2.5], [0.045, 0.17, 0.12, 0, 0, 2.5], [0.125, 0.152, 0.1, -0.004, 0, 2.5]]',
         '[[-0.075, 0.186, 0.142, 0.004, 0, 2.5], [-0.04, 0.19, 0.146, 0.004, 0, 2.5], [0.02, 0.187, 0.143, 0.002, 0, 2.5], [0.08, 0.18, 0.137, -0.003, 0, 2.5], [0.14, 0.172, 0.124, -0.008, 0, 2.5], [0.175, 0.16, 0.106, -0.014, 0, 2.5]]'),
        ('capped([[0, 0.052, 0.058, 0.004, 0, 2.2], [0.13, 0.061, 0.068, -0.006, 0, 2.2], [0.3, 0.057, 0.06, -0.005, 0, 2.1]',
         'capped([[0, 0.058, 0.064, 0.003, 0, 2.2], [0.13, 0.061, 0.068, -0.006, 0, 2.2], [0.3, 0.057, 0.06, -0.005, 0, 2.1]'),
        ('  o.push(tag(P("hair-back", "head", "hair", HAIR, [[0.16, 0.085, 0, 0.082, 0.034, 2.8], [0.04, 0.076, 0, 0.1, 0.034, 2.8], [-0.04, 0.084, 0, 0.088, 0.018, 2.6]]), "hair"));',
         '  o.push(tag(L("hair-fall", "head", "hair", HAIR, [[0.19, 0.089, 0.072, 0.03, 0, 2.2], [0.1, 0.087, 0.06, 0.046, 0, 2.3], [0.0, 0.078, 0.04, 0.062, 0, 2.2], [-0.045, 0.05, 0.022, 0.072, 0, 2]]), "hair"));'),
        ('[[0, 0.051, 0.057, 0.004, 0, 2.2], [0.14, 0.061, 0.067, -0.006, 0, 2.2], [0.447, 0.057, 0.06, 0, 0, 2.1]], 0.045, 0, 1)',
         '[[0, 0.058, 0.064, 0.003, 0, 2.2], [0.14, 0.061, 0.067, -0.006, 0, 2.2], [0.447, 0.057, 0.06, 0, 0, 2.1]], 0.045, 0, 1)'),
        ('[[-0.075, 0.186, 0.142, 0.004, 0, 2.5], [-0.03, 0.19, 0.146, 0.004, 0, 2.5], [0.04, 0.17, 0.12, 0, 0, 2.5], [0.12, 0.152, 0.1, -0.004, 0, 2.5]]',
         '[[-0.075, 0.186, 0.142, 0.004, 0, 2.5], [-0.03, 0.19, 0.146, 0.004, 0, 2.5], [0.05, 0.184, 0.14, -0.001, 0, 2.5], [0.175, 0.16, 0.106, -0.014, 0, 2.5]]'),
        ('[[-0.035, 0.049, 0.055, 0.004, 0, 2.2], [0.13, 0.061, 0.067, -0.006, 0, 2.2], [0.447, 0.057, 0.06, 0, 0, 2.1]]',
         '[[-0.035, 0.057, 0.063, 0.003, 0, 2.2], [0.13, 0.061, 0.067, -0.006, 0, 2.2], [0.447, 0.057, 0.06, 0, 0, 2.1]]'),
    ],
}

for name, pairs in EDITS.items():
    path = os.path.join(T, name)
    text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if new in text:
            continue
        n = text.count(old)
        if n != 1:
            print("[DEBUG] %s: old text found %d times: %s" % (name, n, old[:80]))
            continue
        text = text.replace(old, new)
    open(path, "w", encoding="utf-8").write(text)
    print("written", name)

STRUCT = [
    ('const ribs: number[][] = fine ? [[0.14, 0.085, 0.125], [0.185, 0.095, 0.135], [0.23, 0.1, 0.14], [0.275, 0.1, 0.14], [0.32, 0.092, 0.132], [0.362, 0.078, 0.115]] : [[0.16, 0.09, 0.13], [0.24, 0.1, 0.14], [0.32, 0.09, 0.13]];\n  ribs.forEach(([s, d, w], i) => one(ring(`rib-${i + 1}`, "torso", s!, -0.002 + i * 0.002, d!, w!, fine ? 0.006 : 0.009, -14), "torso"));\n  one(B("sternum", "torso", "skin", BONE, [0.31, -0.094, 0], [0.15, fine ? 0.012 : 0.016, 0.036], 0.004), "torso");',
     'const ribs: number[][] = fine ? [[0.15, 0.085, 0.125, -0.002], [0.195, 0.095, 0.137, 0], [0.24, 0.1, 0.14, 0.002], [0.285, 0.1, 0.138, 0.004], [0.33, 0.096, 0.128, 0.006], [0.372, 0.086, 0.112, 0.008], [0.405, 0.07, 0.086, 0.01]] : [[0.17, 0.092, 0.132, 0], [0.26, 0.1, 0.14, 0.003], [0.35, 0.09, 0.12, 0.007]];\n  ribs.forEach(([s, d, w, cv], i) => one(ring(`rib-${i + 1}`, "torso", s!, cv!, d!, w!, fine ? 0.006 : 0.009, -24), "torso"));\n  one(P("sternum", "torso", "skin", BONE, [[0.39, -0.064, 0, 0.022, fine ? 0.006 : 0.008], [0.34, -0.08, 0, 0.017, fine ? 0.006 : 0.008], [0.29, -0.092, 0, 0.016, fine ? 0.006 : 0.008], [0.24, -0.097, 0, 0.012, fine ? 0.006 : 0.008], [0.218, -0.096, 0, 0.008, fine ? 0.005 : 0.008]]), "torso");'),
    ('  both(e("scapula-l", "torso", [0.34, 0.078, 0.1], [0.075, 0.009, 0.055]), "torso");',
     '  both(L("scapula-l", "torso", "skin", BONE, [[0.245, 0.012, 0.008, 0, -0.025, 2], [0.29, 0.03, 0.008, 0, -0.012, 2.2], [0.34, 0.045, 0.008, 0, 0, 2.4], [0.385, 0.052, 0.008, 0, 0.008, 2.4], [0.405, 0.04, 0.008, 0, 0.005, 2.2]], { extra: { transform: { translation: [0, 0.093, 0.1], rotation: qaxis([1, 0, 0], 30).map(r5) } } }), "torso");\n  both(b("scapular-spine-l", "torso", [0.385, 0.1, 0.07], [0.428, 0.035, 0.172], fine ? 0.0065 : 0.008), "torso");'),
    ('  both(E("ilium-l", "pelvis", "skin", BONE, [0.06, 0.012, 0.108], [0.072, 0.062, 0.022], { rot: qaxis([1, 0, 0], 20) }), "pelvis");',
         '  both(L("ilium-l", "pelvis", "skin", BONE, [[-0.005, 0.009, 0.022, -0.008, -0.022, 2.2], [0.03, 0.008, 0.036, -0.004, -0.012, 2.2], [0.07, 0.008, 0.054, 0, 0.002, 2.4], [0.105, 0.008, 0.066, 0.006, 0.014, 2.6], [0.13, 0.008, 0.06, 0.008, 0.02, 2.6], [0.145, 0.008, 0.034, 0.008, 0.018, 2.2]], { extra: { transform: { translation: [0, 0.012, 0.11], rotation: qaxis([1, 0, 0], -35).map(r5) } } }), "pelvis");'),
    ('import { C, E, B, P, T, mirror, shiftW, st, qaxis, r5, type Json, type V3 } from "./r3-stroller-kit.ts";',
     'import { C, E, B, L, P, T, mirror, shiftW, st, qaxis, r5, type Json, type V3 } from "./r3-stroller-kit.ts";'),
]
path = os.path.join(T, "r3-stroller-s50-structure.ts")
text = open(path, encoding="utf-8").read()
for old, new in STRUCT:
    if new in text:
        continue
    if text.count(old) != 1:
        print("[DEBUG] structure: old text found %d times: %s" % (text.count(old), old[:80]))
        continue
    text = text.replace(old, new)
open(path, "w", encoding="utf-8").write(text)
print("written r3-stroller-s50-structure.ts")

ROUND2 = {
    "r3-stroller-s50-figure.ts": [
        ('out.push(tag(L("hair-cap", "head", "hair", HAIR, [[0.118, 0.076, 0.03, 0.074, 0, 2.2], [0.14, 0.081, 0.04, 0.067, 0, 2.2], [0.163, 0.084, 0.046, 0.064, 0, 2.2]', 'out.push(tag(L("hair-cap", "head", "hair", HAIR, [[0.163, 0.082, 0.04, 0.062, 0, 2.2]'),
    ],
    "r3-stroller-coarse.ts": [
        ('o.push(tag(L("hair-cap", "head", "hair", HAIR, [[0.118, 0.077, 0.03, 0.074, 0, 2.2], [0.162, 0.085, 0.046, 0.064, 0, 2.2]', 'o.push(tag(L("hair-cap", "head", "hair", HAIR, [[0.162, 0.083, 0.04, 0.062, 0, 2.2]'),
        ('o.push(tag(L("hair-cap", "head", "hair", HAIR, [[0.118, 0.08, 0.032, 0.074, 0, 2.2], [0.17,', 'o.push(tag(L("hair-cap", "head", "hair", HAIR, [[0.17,'),
    ],
    "r3-stroller-s50-structure.ts": [
        ('  one(e("sacrum", "pelvis", [0.05, 0.05, 0], [0.058, 0.02, 0.048]), "pelvis");',
         '  one(L("sacrum", "pelvis", "skin", BONE, [[-0.02, 0.008, 0.008, 0.066, 0, 2], [0.0, 0.016, 0.01, 0.064, 0, 2], [0.03, 0.03, 0.012, 0.06, 0, 2.2], [0.065, 0.042, 0.014, 0.054, 0, 2.3], [0.1, 0.05, 0.016, 0.046, 0, 2.3]]), "pelvis");'),
    ],
}
for name, pairs in ROUND2.items():
    path = os.path.join(T, name)
    text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if new in text:
            continue
        if text.count(old) != 1:
            print("[DEBUG] %s: old text found %d times: %s" % (name, text.count(old), old[:80]))
            continue
        text = text.replace(old, new)
    open(path, "w", encoding="utf-8").write(text)

ROUND3 = {
    "r3-stroller-coarse.ts": [
        ('const wedge = [[-0.195, 0.03], [-0.195, 0.2], [-0.12, 0.285], [0.12, 0.285], [0.195, 0.2], [0.195, 0.03]];\n  pram1(s, fan("canopy-rear", "stroller", HINGE, 65, wedge), "stroller-fabric");\n  pram1(s, fan("canopy-front", "stroller-hood", [0, 0], 25, wedge), "stroller-fabric");',
         'const wedge = [[-0.2, 0.03], [-0.2, 0.2], [-0.125, 0.285], [0.125, 0.285], [0.2, 0.2], [0.2, 0.03]];\n  pram1(s, fan("canopy", "stroller", HINGE, 25, wedge, 80), "stroller-fabric");'),
    ],
    "r3-stroller-draw2d.ts": [
        ('  if (id === "canopy-rear") return { id: "hood-rear" };', '  if (id === "canopy-rear" || id === "canopy") return { id: "hood-rear" };'),
    ],
    "r3-stroller-decisions.ts": [
        ("long hair as cap, back fall and two side strands tucked behind the ears", "long hair as a crown cap and one shoulder-length fall that wraps the back of the skull and the nape, with the ears showing at its front edge"),
        ("langes Haar als Kappe, Rückenfall und zwei hinter die Ohren gelegte Seitensträhnen", "langes Haar als Scheitelkappe und ein schulterlanger Fall, der Hinterkopf und Nacken umschließt und die Ohren an seiner Vorderkante zeigt"),
        ("six rib hoops with sternum, clavicles, scapulae, iliac wings, sacrum", "seven rib hoops falling 24 degrees to the front onto a curved sternum, clavicles, triangular scapulae with their spine on the back of the rib cage, fanned iliac wings, a tapering sacrum"),
        ("sechs Rippenreifen mit Brustbein, Schlüsselbeine, Schulterblätter, Darmbeinschaufeln, Kreuzbein", "sieben nach vorn um 24 Grad abfallende Rippenreifen an einem gebogenen Brustbein, Schlüsselbeine, dreieckige Schulterblätter mit ihrer Gräte auf dem Rücken des Brustkorbs, gefächerte Darmbeinschaufeln, ein sich verjüngendes Kreuzbein"),
        ("the skeleton with three rib hoops, sternum", "the skeleton with three rib hoops falling to the front, sternum"),
        ("Das Skelett mit drei Rippenreifen", "Das Skelett mit drei nach vorn abfallenden Rippenreifen"),
        ("two canopy wedges and four disc wheels", "one canopy shell over both bows (at this scale it does not fold) and four disc wheels"),
        ("zwei Verdeckkeile und vier Scheibenräder", "eine Verdeckschale über beide Bügel (in diesem Maßstab klappt sie nicht) und vier Scheibenräder"),
    ],
}
for name, pairs in ROUND3.items():
    path = os.path.join(T, name)
    text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if new in text:
            continue
        if text.count(old) != 1:
            print("[DEBUG] %s: old text found %d times: %s" % (name, text.count(old), old[:80]))
            continue
        text = text.replace(old, new)
    open(path, "w", encoding="utf-8").write(text)
